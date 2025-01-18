use std::{
    io::{
        prelude::*, BufReader
    }, 
    net::{TcpListener, TcpStream}
};

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in server.incoming() {
        let client = stream.unwrap();
        connection_handler(client);
    }
}


fn connection_handler (client: TcpStream) {
    let reader = BufReader::new(&client);
    let req: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{req:#?}");
}