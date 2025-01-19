use std::collections::HashMap;
use std::io::{ Error, ErrorKind };
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::cell::RefCell;

mod pool;
pub mod request;
pub mod response;
pub mod status_codes;
pub mod mime_types;

pub struct HTTPServer {
    pub addr: Option<&'static str>,
    worker_count: usize,
    error_handler: Box<dyn Fn(Error)>,
    status_codes: Rc<HashMap<u16, String>>,
    mime_map: Rc<HashMap<&'static str, &'static str>>
}

impl HTTPServer {
    ///
    /// creates a new instance of the `HTTPServer`
    /// 
    /// this doesn't listen for connections until you run the `listen` method.
    ///
    pub fn new () -> Self {
        HTTPServer {
            addr: None,
            worker_count: 1,
            error_handler: Box::new(|_e| ()),
            status_codes: Rc::new(import_status_messages()),
            mime_map: Rc::new(import_mime_map())
        }
    }

    ///
    /// Listen for connections on `addr`
    /// 
    /// This method blocks the current thread so you must configure the server before calling it.
    /// 
    /// ## Returns
    /// a result with either `Ok(())` or some error
    /// 
    pub fn listen (&mut self, addr: &'static str) -> Result<(), Error> {
        let listener = TcpListener::bind(addr)?;
        self.addr = Some(addr);

        for connection in listener.incoming() {
            let stream = connection?;
            self.on_connection(stream);
        }

        Ok(())
    }

    fn on_connection (&mut self, stream: TcpStream) {
        let stream = Rc::new(RefCell::new(stream));

        let mut res =  response::Response::new(
            200, 
            Rc::downgrade(&self.status_codes), 
            stream.clone(),
            Rc::downgrade(&self.mime_map)
        );
        let req = match request::Request::build(stream.clone()) {
            Ok(data) => data,
            Err(error) => {
                (self.error_handler)(error);
                return;
            }
        };

        println!("{req:#?}");
        
        res.set_header("content-type", "text/html").unwrap();
        res.send_file("./index.html").unwrap();
        
        println!("{res:#?}");
    }

    ///
    /// Sets an Error Handler if any error happens during req parsing.
    /// 
    pub fn on_error <F> (&mut self, f: F)
    where
        F: Fn(Error) + 'static
    {
        self.error_handler = Box::new(f);
    }

    ///
    /// How much threads to spawn, default to `1`
    ///
    pub fn set_workers (&mut self, amount: usize) {
        self.worker_count = amount;
    }
}


// Some helpers:
fn extract_option <T> (op: Option<T>) -> Result<T, Error> {
    if let Some(data) = op {
        Ok(data)
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Empty Option"))
    }
}

fn import_status_messages () -> HashMap<u16, String> {
    let mut map = HashMap::new();

    for (key, val) in status_codes::StatusMap::get_map().iter() {
       map.insert(key.parse::<u16>().unwrap(), val.to_string());
    }

    map
}

fn import_mime_map () -> HashMap<&'static str, &'static str> {
    mime_types::MimeTypes::get_map()
}

fn extract_ext <'a> (path: &'a str) -> &'a str {
    path.split(".").collect::<Vec<&str>>().pop().unwrap_or_default()
}


// A very useless test...
#[cfg(test)]
mod HelperTests {
    use super::*;
    #[test]
    fn extract_extention_helper () {
        assert_eq!(extract_ext("./main.rs"), "rs");
        assert_eq!(extract_ext(".script.sh"), "sh");
        assert_eq!(extract_ext("file.md"), "md");
        assert_eq!(extract_ext("poi.nte.d.file.buzz"), "buzz");
    }
}