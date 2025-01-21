use std::net::TcpStream;
use std::sync::mpsc::{ self, Receiver, Sender };
use std::thread;
use thread::JoinHandle;
use std::sync::Arc;
use std::sync::Mutex;
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Error as err;
use std::sync::RwLock;

use crate::Response;
use crate::Request;
use crate::Routes;

pub type ErrorHandler = Arc<dyn Fn(err) + 'static + Send>;

#[allow(unused)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    transmitter: Sender<(TcpStream, Arc<RwLock<Routes>>)>,
    reciever: Arc<Mutex<Receiver<(TcpStream, Arc<RwLock<Routes>>)>>>,
    pub error_handler: ErrorHandler,
}
impl ThreadPool {
    pub fn new (worker_count: usize) -> Self {
        let workers = Vec::with_capacity(worker_count);
        let (transmitter, reciever) = mpsc::channel();
        let recv = Arc::new(Mutex::new(reciever));

        ThreadPool {
            transmitter,
            reciever: recv,
            workers,
            error_handler: Arc::new(|e| eprintln!("{e}"))
        }
    }

    pub fn init (&mut self) {
        for _ in [0..self.workers.capacity()] {
            self.workers.push(Worker::new(self.reciever.clone(), self.error_handler.clone()));
        }
    }


    pub fn execute (&self, stream: TcpStream, handlers: Arc<RwLock<Routes>>) -> Result<(), Box<dyn Error>> {
        self.transmitter.send((stream, handlers))?;

        Ok(())
    }
}

struct Worker {
    thread: JoinHandle<()>
}

impl Worker {
    pub fn new (recv: Arc<Mutex<Receiver<(TcpStream, Arc<RwLock<Routes>>)>>>, error_handler: ErrorHandler) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let (stream, handlers) = recv.lock().unwrap().recv().unwrap();
                let stream = Rc::new(RefCell::new(stream));

                let mut res = Response::new(
                    200, 
                    Rc::downgrade(&status_codes), 
                    stream.clone(),
                    Rc::downgrade(&mime_map)
                );

                let req = match Request::build(stream.clone()) {
                    Ok(data) => data,
                    Err(error) => {
                        error_handler(error);
                        return;
                    }
                };
            }
        });

        Worker {
            thread
        }
    }
} 