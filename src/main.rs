use request::Request;
use response::Response;
use router::RouteResult;
use std::path;
use stress::*;

fn main() {
    let mut server = HTTPServer::new(1);
    server.middleware("*", middlewares::static_serve::serve_static("./public"));
    server.last("*", Box::new(not_found));

    server.listen("127.0.0.1:8080").unwrap();
}

fn on_req(_req: &mut Request, res: &mut Response) -> bool {
    res.send_file("./index.html").unwrap();
    true
}

fn not_found(_req: &mut Request, res: &mut Response) -> RouteResult {
    res.set_status(404)?;
    res.send_file("./not-found.html")?;
    Ok(true)
}
