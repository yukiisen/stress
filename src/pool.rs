use std::{net::TcpStream, sync::{mpsc::Receiver, Arc, Mutex}, thread::{self, JoinHandle}};
#[allow(unused)]
use std::thread::Thread;

#[allow(unused)]
pub struct ThreadPool {
    workers: Vec<Worker>
}
impl ThreadPool {
    pub fn new (workers: usize) {
        
    }
}

struct Worker {
    thread: JoinHandle<()>
}

impl Worker {
    pub fn new (recv: Arc<Mutex<Receiver<TcpStream>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = recv.lock().unwrap().recv().unwrap();
            }
        });

        Worker {
            thread
        }
    }
} 