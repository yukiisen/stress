use request::Request;
use response::Response;
use router::RouteResult;
use stress::*;

fn main() {
    let mut server = HTTPServer::new(2);
    server.get("/", Box::new(on_req));

    server.listen("127.0.0.1:8080").unwrap();
}

fn on_req(_req: &mut Request, res: &mut Response) -> RouteResult {
    res.send_file("./index.html")?;
    Ok(true)
}

