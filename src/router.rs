use crate::{request::Request, response::Response};

pub type RouteHandler = Box<dyn Fn(&mut Request, &mut Response) -> bool + 'static + Send + Sync>;

pub struct Route {
    pub handler: RouteHandler,
    pub method: &'static str,
    pub path: &'static str,
}
