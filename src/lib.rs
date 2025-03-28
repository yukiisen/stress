use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::RwLock;

mod controller;
pub mod mime_types;
mod pool;
pub mod request;
pub mod response;
pub mod router;
pub mod status_codes;

/// Public module that contains built in middlewares for different purposes.
///
/// this will likely will stay almost empty for like forever.
pub mod middlewares;

use pool::{ErrorHandler, ThreadPool};
use request::Request;
use response::Response;
use router::{Route, RouteHandler};

type Routes = HashMap<&'static str, Vec<Route>>;

pub struct HTTPServer {
    pub addr: Option<&'static str>,
    thread_pool: ThreadPool,
    status_codes: Arc<HashMap<u16, String>>,
    mime_map: Arc<HashMap<&'static str, &'static str>>,
    ///
    /// Uses a Hashmap to store the routes based on their method.
    ///
    // consider using a tree data structure for pathnames later.
    routes: Arc<RwLock<Routes>>,
}

impl HTTPServer {
    ///
    /// creates a new instance of the `HTTPServer`
    ///
    /// this doesn't listen for connections until you run the `listen` method.
    ///
    pub fn new(workers: usize) -> Self {
        HTTPServer {
            addr: None,
            thread_pool: ThreadPool::new(workers),
            status_codes: Arc::new(import_status_messages()),
            mime_map: Arc::new(import_mime_map()),
            routes: Arc::new(RwLock::new(HashMap::from([
                ("errors", Vec::new()),
                ("final", Vec::new()),
                ("global", Vec::new()),
                ("GET", Vec::new()),
                ("POST", Vec::new()),
                ("PUT", Vec::new()),
                ("DELETE", Vec::new()),
                ("PATCH", Vec::new()),
            ]))),
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
    pub fn listen(&mut self, addr: &'static str) -> Result<(), Error> {
        let listener = TcpListener::bind(addr)?;
        self.addr = Some(addr);
        self.thread_pool
            .init(self.mime_map.clone(), self.status_codes.clone());

        for connection in listener.incoming() {
            let stream = connection?;

            self.thread_pool
                .execute(stream, self.routes.clone())
                .unwrap();
        }

        Ok(())
    }

    // Route Initializers..

    ///
    /// Defines a route with the specified `method`, `path` and `handler`.
    ///
    /// Not intended for use unless you want some custom functionality.
    ///
    /// instead use `get`, `post`, `put` or `delete`, based on your target method.
    pub fn register(&mut self, method: &'static str, path: &'static str, handler: RouteHandler) {
        self.routes
            .write()
            .unwrap()
            .get_mut(method.to_uppercase().as_str())
            .unwrap()
            .push(Route {
                handler,
                path,
                method,
            });
    }

    /// Creates a global middleware for the specified `path`.
    ///
    /// global middlewares are always excuted before any route handlers.
    pub fn middleware(&mut self, path: &'static str, handler: RouteHandler) {
        self.routes
            .write()
            .unwrap()
            .get_mut("global")
            .unwrap()
            .push(Route {
                handler,
                path,
                method: "*",
            });
    }

    pub fn last(&mut self, path: &'static str, handler: RouteHandler) {
        self.routes
            .write()
            .unwrap()
            .get_mut("final")
            .unwrap()
            .push(Route {
                handler,
                path,
                method: "*",
            });
    }

    pub fn error_ware(&mut self, method: &'static str, path: &'static str, handler: RouteHandler) {
        self.routes
            .write()
            .unwrap()
            .get_mut("errors")
            .unwrap()
            .push(Route {
                handler,
                path,
                method,
            });
    }

    /// A wrapper for `HTTPServer.register()`.
    pub fn get(&mut self, path: &'static str, handler: RouteHandler) {
        self.register("get", path, handler);
    }

    /// A wrapper for `HTTPServer.register()`.
    pub fn post(&mut self, path: &'static str, handler: RouteHandler) {
        self.register("post", path, handler);
    }

    /// A wrapper for `HTTPServer.register()`.
    pub fn put(&mut self, path: &'static str, handler: RouteHandler) {
        self.register("put", path, handler);
    }

    /// A wrapper for `HTTPServer.register()`.
    pub fn delete(&mut self, path: &'static str, handler: RouteHandler) {
        self.register("delete", path, handler);
    }

    /// A wrapper for `HTTPServer.register()`.
    pub fn patch(&mut self, path: &'static str, handler: RouteHandler) {
        self.register("patch", path, handler);
    }

    ///
    /// Sets an Error Handler if any error happens during req parsing.
    ///
    pub fn on_error<F>(&mut self, f: ErrorHandler) {
        self.thread_pool.error_handler = f;
    }
}

// Some helpers:

fn extract_option<T>(op: Option<T>) -> Result<T, Error> {
    if let Some(data) = op {
        Ok(data)
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Empty Option"))
    }
}

fn import_status_messages() -> HashMap<u16, String> {
    let mut map = HashMap::new();

    for (key, val) in status_codes::StatusMap::get_map().iter() {
        map.insert(key.parse::<u16>().unwrap(), val.to_string());
    }

    map
}

fn import_mime_map() -> HashMap<&'static str, &'static str> {
    mime_types::MimeTypes::get_map()
}

fn extract_ext<'a>(path: &'a str) -> &'a str {
    path.split(".")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap_or_default()
}

// A very useless test...
#[cfg(test)]
mod helper_tests {
    use super::*;
    #[test]
    fn extract_extention_helper() {
        assert_eq!(extract_ext("./main.rs"), "rs");
        assert_eq!(extract_ext(".script.sh"), "sh");
        assert_eq!(extract_ext("file.md"), "md");
        assert_eq!(extract_ext("poi.nte.d.file.buzz"), "buzz");
    }
}
