use std::sync::RwLockReadGuard;

use crate::request::Request;
use crate::response::Response;
use crate::Routes;

pub fn handle_requests(mut req: Request, mut res: Response, routes: RwLockReadGuard<Routes>) {
    let method = req.method.clone();

    // Apply the global handlers first.
    for route in routes.get("global").unwrap().iter() {
        if route.path == req.path {
            let handle = &route.handler;
            let rslt = handle(&mut req, &mut res);
            if rslt {
                return;
            }
        }
    }

    for route in routes.get(method.as_str()).unwrap().iter() {
        if route.path == req.path {
            let handle = &route.handler;
            let rslt = handle(&mut req, &mut res);
            if rslt {
                return;
            }
        }
    }

    for route in routes.get("final").unwrap().iter() {
        if route.path == req.path {
            let handle = &route.handler;
            let rslt = handle(&mut req, &mut res);
            if rslt {
                return;
            }
        }
    }
}
