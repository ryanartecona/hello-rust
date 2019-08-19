use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer));

    let hello_html = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", hello_html);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}
