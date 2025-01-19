use std::collections::HashMap;
use std::io::{ Error, ErrorKind };
use std::net::{TcpListener, TcpStream};
use std::rc::Rc;
use std::cell::RefCell;

mod pool;
mod request;
mod response;

pub struct HTTPServer {
    addr: Option<&'static str>,
    worker_count: usize,
    error_handler: Box<dyn Fn(Error)>,
    status_codes: Rc<HashMap<u16, String>>
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
            status_codes: Rc::new(import_status_messages("./data/status_codes.json")
                            .expect("Status messages file not found!"))
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
    pub fn listen (&mut self, addr: &'static str) -> Result<(), std::io::Error> {
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

        let mut res =  response::Response::new(200, Rc::downgrade(&self.status_codes), stream.clone());
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

fn import_status_messages (path: &str) -> Result<HashMap<u16, String>, Error> {
    let mut map = HashMap::new();

    let data = std::fs::read_to_string(path)?
        .replace("{", "")
        .replace("}", "")
        .replace("\"", "")
        .replace(",", "");
    
    let lines: Vec<Option<(&str, &str)>> = data.lines()
        .map(|line| line.split_once(":"))
        .collect();

    for line in lines {
        let line = if let Some(text) = line { text } else { continue; };
        map.insert(line.0.trim().parse::<u16>().expect("Key must be a number"),line.1.trim().to_string());
    }
    
    Ok(map)
}