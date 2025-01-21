use std::error::Error;
use std::sync::Arc;
use crate::{
    request::Request, 
    response::Response
};

pub type RouteResult = Result<bool, Box<dyn Error>>;
pub type RouteHandler = Box<dyn Fn(&mut Request, &mut Response) -> RouteResult + 'static + Send>;

pub struct Route {
    pub handler: RouteHandler,
    pub method: &'static str,
    pub path: &'static str
}