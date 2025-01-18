use std::{
    io::{
        prelude::*, BufReader
    }, 
    net::{TcpListener, TcpStream},
    fs
};

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in server.incoming() {
        let client = stream.unwrap();
        connection_handler(client);
    }
}


fn connection_handler (mut client: TcpStream) {
    let reader = BufReader::new(&client);
    let req: Vec<_> = reader
        .lines()
        .map(|result| result.expect("Invalid Data (Are you using HTTPS?)"))
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{req:#?}");

    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("./index.html").unwrap();
    let length = format!("content-length: {}", content.len());

    let res = [status.to_string(), length, String::from(""), content].join("\r\n");

    client.write_all(res.as_bytes()).expect("Failed To Return a response");
}