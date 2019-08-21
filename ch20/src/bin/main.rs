use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let root_get_preamble = b"GET / HTTP/1.1\r\n";
    let sleep_get_preamble = b"GET /sleep HTTP/1.1\r\n";

    // println!("Request: {}", String::from_utf8_lossy(&buffer));

    let (status_line, filename) = if buffer.starts_with(root_get_preamble) {
        (
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n",
            "hello.html",
        )
    } else if buffer.starts_with(sleep_get_preamble) {
        thread::sleep(Duration::from_secs(5));
        (
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n",
            "hello.html",
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\n\r\n",
            "404.html",
        )
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
