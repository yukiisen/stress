use std::error::Error;
use std::sync::RwLockReadGuard;

use crate::request::Request;
use crate::response::Response;
use crate::router::Route;
use crate::Routes;

pub fn handle_requests(mut req: Request, mut res: Response, routes: RwLockReadGuard<Routes>) {
    let method = req.method.clone();

    // Apply the global handlers first.
    for route in routes.get("global").unwrap().iter() {
        if route.path == req.path || route.path == "*" {
            let handle = &route.handler;
            let rslt = handle(&mut req, &mut res);

            if let Err(e) = rslt {
                return handle_errors(req, res, routes, e);
            } else {
                if rslt.unwrap() {
                    return;
                }
            }
        }
    }

    for route in routes.get(method.as_str()).unwrap().iter() {
        if route.path == req.path {
            let handle = &route.handler;
            let rslt = handle(&mut req, &mut res);

            if let Err(e) = rslt {
                return handle_errors(req, res, routes, e);
            } else {
                if rslt.unwrap() {
                    return;
                }
            }
        }
    }

    for route in routes.get("final").unwrap().iter() {
        if route.path == req.path || route.path == "*" {
            let handle = &route.handler;
            let rslt = handle(&mut req, &mut res);

            if let Err(e) = rslt {
                return handle_errors(req, res, routes, e);
            } else {
                if rslt.unwrap() {
                    return;
                }
            }
        }
    }
}

fn handle_errors(
    mut req: Request,
    mut res: Response,
    routes: RwLockReadGuard<Routes>,
    error: Box<dyn Error>,
) {
    let error_routes = routes.get("errors").unwrap();
    let error_routes = error_routes
        .iter()
        .filter(|r| r.path == req.path || r.path == "*")
        .filter(|r| r.method == req.method || r.method == "*")
        .collect::<Vec<&Route>>();

    req.error = Some(error);

    for route in error_routes {
        let handler = &route.handler;
        let rslt = handler(&mut req, &mut res);

        if let Ok(val) = rslt {
            if val {
                return;
            }
        }
    }
}
