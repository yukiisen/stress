use std::{
    io::{
        Error, ErrorKind
    },
    net::{TcpListener, TcpStream}
};

mod pool;
mod request;
mod response;

pub struct HTTPServer {
    addr: Option<&'static str>,
    worker_count: usize,
    error_handler: Box<dyn Fn(Error)>
}

impl HTTPServer {
    ///
    /// creates a new instance of the `HTTPServer`
    /// 
    /// this doesn't listen for connections until you run the `listen` method.
    /// 
    pub fn new () -> HTTPServer {
        HTTPServer {
            addr: None,
            worker_count: 1,
            error_handler: Box::new(|_e| ())
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
        let req = match request::Request::build(&stream) {
            Ok(data) => data,
            Err(error) => {
                (self.error_handler)(error);
                return;
            }
        };
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
        Err(Error::new(ErrorKind::InvalidInput, "Empty method"))
    }
}