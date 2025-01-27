use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::io::Error as err;
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;
use thread::JoinHandle;

use crate::controller;
use crate::Request;
use crate::Response;
use crate::Routes;

pub type ErrorHandler = Arc<dyn Fn(err) + 'static + Send + Sync>;

#[allow(unused)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    transmitter: Sender<(TcpStream, Arc<RwLock<Routes>>)>,
    reciever: Arc<Mutex<Receiver<(TcpStream, Arc<RwLock<Routes>>)>>>,
    pub error_handler: ErrorHandler,
}
impl ThreadPool {
    pub fn new(worker_count: usize) -> Self {
        let workers = Vec::with_capacity(worker_count);
        let (transmitter, reciever) = mpsc::channel();
        let recv = Arc::new(Mutex::new(reciever));

        ThreadPool {
            transmitter,
            reciever: recv,
            workers,
            error_handler: Arc::new(|e| eprintln!("{e}")),
        }
    }

    pub fn init(
        &mut self,
        mime_map: Arc<HashMap<&'static str, &'static str>>,
        status_codes: Arc<HashMap<u16, String>>,
    ) {
        for _ in 0..self.workers.capacity() {
            self.workers.push(Worker::new(
                self.reciever.clone(),
                self.error_handler.clone(),
                mime_map.clone(),
                status_codes.clone(),
            ));
        }
    }

    pub fn execute(
        &self,
        stream: TcpStream,
        handlers: Arc<RwLock<Routes>>,
    ) -> Result<(), Box<dyn Error>> {
        self.transmitter.send((stream, handlers))?;

        Ok(())
    }
}

struct Worker {
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(
        recv: Arc<Mutex<Receiver<(TcpStream, Arc<RwLock<Routes>>)>>>,
        error_handler: ErrorHandler,
        mime_map: Arc<HashMap<&'static str, &'static str>>,
        status_codes: Arc<HashMap<u16, String>>,
    ) -> Self {
        let thread = thread::spawn(move || {
            let on_error = error_handler;
            println!("I'm on!!");
            loop {
                let (stream, handlers) = recv.lock().unwrap().recv().unwrap();
                let stream = Rc::new(RefCell::new(stream));
                let routes = handlers.read().unwrap();

                let res = Response::new(
                    200,
                    Arc::downgrade(&status_codes),
                    stream.clone(),
                    Arc::downgrade(&mime_map),
                );

                let req = match Request::build(stream.clone()) {
                    Ok(data) => data,
                    Err(error) => {
                        on_error(error);
                        continue;
                    }
                };

                controller::handle_requests(req, res, routes);
            }
        });

        Worker { thread }
    }
}
