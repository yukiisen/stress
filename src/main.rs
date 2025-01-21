use request::Request;
use response::Response;
use route::RouteResult;
use stress::*;

fn main () {
    let mut server = HTTPServer::new();
    server.set_workers(4);
    server.get("/", std::rc::Rc::new(on_req));

    server.listen("127.0.0.1:8080").unwrap();
}

fn on_req (req: &mut Request, res: &mut Response) -> RouteResult {
    res.send_file("./index.html")?;
    Ok(true)
}