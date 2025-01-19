use Stress::*;

fn main () {
    let mut server = HTTPServer::new();
    server.listen("127.0.0.1:8080").unwrap();
}