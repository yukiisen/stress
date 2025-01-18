use std::{
    collections::HashMap, 
    io::{
        self,
        BufReader,
        prelude::*,
        Error as err
    }, 
    net::{
        IpAddr, 
        TcpStream
    }
};

use super::extract_option;

pub struct Request {
    method: String,
    path: String,
    host: String,
    http_version: String,
    headers: HashMap<&'static str, String>,
    ip: IpAddr,
    user_agent: String
}

impl Request {
    pub fn build (stream: &TcpStream) -> Result<Request, err> {
        let reader = BufReader::new(stream);
        let ip = stream.peer_addr()?.ip();

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

        let mut data_iter= data.iter();

        for i in 0..data.len() {
            if i == 0 {  data_iter.next(); continue; }
            let (key, val) = extract_option(
                extract_option(data_iter.next())?.split_once(":")
            )?;

            headers.insert(key, val.to_string());
        }

        Ok(Request {
            ip,
            method,
            path,
            http_version,
            host: headers.get("Host").unwrap_or(&"".to_string()).clone(),
            user_agent: headers.get("User-Agent").unwrap_or(&"".to_string()).clone(),
            headers,
        })
    }
}