use stressless::*;

fn main () {
    let mut server = HTTPServer::new();

    server.listen("127.0.0.1:9090").unwrap();
}