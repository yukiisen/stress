use std::error::Error;
use std::rc::Rc;
use crate::{
    request::Request, 
    response::Response
};

pub type RouteResult = Result<bool, Box<dyn Error>>;
pub type RouteHandler = Rc<dyn Fn(&mut Request, &mut Response) -> RouteResult + 'static>;

pub struct Route {
    pub handler: RouteHandler,
    pub method: &'static str,
    pub path: &'static str
}