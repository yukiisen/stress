use std::error::Error;
use std::sync::RwLockReadGuard;

use crate::request::Request;
use crate::response::Response;
use crate::Routes;

pub fn handle_requests(req: Request, res: Response, routes: &RwLockReadGuard<Routes>) {
    let method = req.method;

    // Apply the global handlers first.
    for route in routes.get("global").unwrap().iter() {
        if route.path == req.path {
            let rslt = route.handler(req, res);
            if rslt {
                return;
            }
        }
    }

    for route in routes.get(method.as_str()).unwrap().iter() {
        if route.path == req.path {
            let rslt = route.handler(req, res);
            if rslt {
                return;
            }
        }
    }
}
