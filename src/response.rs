use std::collections::HashMap;

pub struct Response {
    status: u16,
    status_message: &'static str,
    headers: HashMap<String, String>,
    http_version: String
}

impl Response {
    
}