use std::collections::HashMap;
use std::io::{Error as err, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::error::Error;
use std::fs::File;

use super::*;

#[derive(Debug)]
pub struct Response {
    status: u16,
    status_message: String,
    headers: HashMap<String, String>,
    pub http_version: String,
    status_map: Weak<HashMap<u16, String>>,
    mime_map:  Weak<HashMap<&'static str, &'static str>>,
    /// The connection socket.
    socket: Rc<RefCell<TcpStream>>,
    headers_sent: bool,
    body_sent: bool
}

impl Response {
    /// Creates a new Response instance.
    /// 
    /// Not intended for use unless you're overriding some functionality.
    pub fn new (
        status: u16, 
        status_map: Weak<HashMap<u16, String>>, 
        stream: Rc<RefCell<TcpStream>>, 
        mime_map: Weak<HashMap<&'static str, &'static str>>
    ) -> Self {
        Response { 
            status,
            status_message: status_map.upgrade().unwrap().get(&status).unwrap_or(&"".to_string()).clone(),
            headers: HashMap::from([("content-type".to_string(), "text/plain".to_string())]),
            http_version: String::from("HTTP/1.1"),
            status_map, 
            mime_map,
            headers_sent: false,
            body_sent: false,
            socket: stream
        }
    }

    /// Sets A header and return s a `Result`
    /// 
    /// ## Fails:
    /// if the headers Have been sent already.
    pub fn set_header (&mut self, header: &str, value: &str) -> Result<(), err> {
        if self.headers_sent {
            return Err(err::new(ErrorKind::InvalidInput, "Cannot Write Headers After they're sent!"));
        }

        self.headers.insert(header.to_string(), value.to_string());
        Ok(())
    }

    /// ## Returns:
    /// An `Option` with the specified header or `None`
    pub fn get_header (&self, header: &str) -> Option<&String> {
        self.headers.get(header)
    }

    /// Sets status code and status message automatically based on the status map.
    /// 
    /// ## Fails:
    /// if the response headers has been sent already.
    pub fn set_status (&mut self, status: u16) -> Result<(), err> {
        if self.headers_sent {
            return Err(err::new(ErrorKind::InvalidInput, "Cannot Write Headers After they're sent!"));
        }

        self.status = status;
        self.status_message = self.status_map.upgrade().unwrap().get(&status).unwrap_or(&"".to_string()).clone();
        Ok(())
    }

    /// Current response status code.
    pub fn status (&self) -> u16 {
        self.status
    }

    /// Sets the content type for the response
    pub fn set_content_type (&mut self, c_type: &str) -> Result<(), err> {
        self.set_header("content-type", c_type)?;
        Ok(())
    }

    /// Send the file at `path` as the response.
    /// 
    /// This method doesn't guarantee that the file was succesfully sent to the client.
    pub fn send_file (&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        if self.body_sent {
            return Err(Box::new(err::new(ErrorKind::AlreadyExists, "Soem other data is already sent.")));
        }

        let mut file = File::open(path)?;
        let meta = file.metadata()?;

        self.set_header(
            "content-length", 
            meta.len().to_string().as_str()
        )?;
        self.set_header(
            "content-type", 
            self.mime_map.upgrade().unwrap().get(extract_ext(path)).unwrap_or(&"text/plain")
        )?;

        self.check_headers()?;
        self.body_sent = true;

        let mut buf = [0; 2048];
        let mut stream = self.socket.try_borrow_mut()?;
        
        while let Ok(chunk) = file.read(&mut buf[..]) {
            if chunk == 0 {
                break;
            }
            stream.write_all(&buf[..chunk])?;
        }

        
        Ok(())
    }

    /// Pipes a stream to the response, sending all the data.
    /// 
    /// ## Returns:
    /// an error of the socket is busy or the stream fails.
    /// 
    /// This method can't guarantee that the stream was succesfully sent yet.
    pub fn stream <T> (&mut self, stream: &mut T) -> Result<(), Box<dyn Error>>
    where 
        T: Read
    {
        if self.body_sent {
            return Err(Box::new(err::new(ErrorKind::AlreadyExists, "Soem other data is already sent.")));
        }

        self.body_sent = true;
        self.check_headers()?;

        let mut buf = [0; 2048];
        let mut socket = self.socket.try_borrow_mut()?;

        while let Ok(chunk) = stream.read(&mut buf[..]) {
            if chunk == 0 {
                break;
            }
            socket.write_all(&buf[..chunk])?;
        }

        Ok(())
    }

    /// Write `data` to the response.
    /// 
    /// ## Returns:
    /// an error if writing fails or if you're using the stream in a different place (the `Request` for example).
    pub fn send (&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        self.body_sent = true;
        self.check_headers()?;
        self.socket.try_borrow_mut()?.write_all(data.as_bytes())?;
        Ok(())
    }

    /// Ends the response and closes the stream.
    /// 
    /// ## Deprecated:
    /// This is useless as unexpected.
    pub fn end (&mut self) -> Result<(), Box<dyn Error>> {
        self.body_sent = true;
        self.check_headers()?;
        self.socket.try_borrow_mut()?.shutdown(std::net::Shutdown::Both)?;
        Ok(())
    }


    fn check_headers (&mut self) -> Result<(), Box<dyn Error>> {
        if self.headers_sent == false {
            let res = self.create_response();
            self.socket.try_borrow_mut()?.write_all(res.as_bytes())?;
            self.headers_sent = true;
        }

        Ok(())
    }

    fn create_response (&mut self) -> String {
        let first = format!("{} {} {}\r\n", self.http_version, self.status, self.status_message);
        let mut headers = String::new();

        for (key, val) in self.headers.iter() {
            headers.push_str(
                format!("{key}: {val}\r\n").as_str()
            );
        }

        format!("{first}{headers}\r\n")
    }
}