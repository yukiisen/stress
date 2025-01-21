use std::collections::HashMap;
use std::io::{ self, BufReader, prelude::*, Error as err };
use std::net::{ IpAddr, TcpStream };
use std::cell::RefCell;
use std::rc::Rc;

use super::extract_option;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub host: String,
    pub http_version: String,
    headers: HashMap<String, String>,
    pub ip: IpAddr,
    pub user_agent: String,
    pub body: Option<()>,
    pub socket: Rc<RefCell<TcpStream>>
}

impl Request {
    /// Builds a `Request` object from a connection stream.
    pub fn build (
        stream: Rc<RefCell<TcpStream>>
    ) -> Result<Self, err> {
        
        let reader = BufReader::new(stream.borrow().try_clone()?);
        let ip = stream.borrow().peer_addr()?.ip();

        let data: Vec<String> = reader
            .lines()
            .map(|result | result.unwrap_or("".to_string()))
            .take_while(|line| !line.is_empty())
            .collect();

        let mut metadata = if let Some(first) = data.first() { first.split(" ") } 
        else { 
            return Err(err::new(io::ErrorKind::InvalidData, "Invalid Request"));
        };

        let method = extract_option(metadata.next())?.to_string();
        let path = extract_option(metadata.next())?.to_string();
        let http_version = extract_option(metadata.next())?.to_string();
        let mut headers = HashMap::new();

        for line in &data {
            if *line == data[0] { continue; }

            let (key, val) = extract_option(line.split_once(":"))?;
            headers.insert(key.to_string(), val.trim().to_string());
        }

        if http_version != String::from("HTTP/1.1") {
            return Err(err::new(io::ErrorKind::InvalidData, format!("Unsupported HTTP version {http_version}")));
        }

        Ok(Request {
            ip,
            method,
            path,
            http_version,
            host: headers.get("Host").unwrap_or(&"".to_string()).clone(),
            user_agent: headers.get("User-Agent").unwrap_or(&"".to_string()).clone(),
            headers,
            body: None,
            socket: stream
        })
    }

    /// Returns the request header.
    pub fn get_header (&self, val: &str) -> Option<&String>{
        self.headers.get(val)
    }
}