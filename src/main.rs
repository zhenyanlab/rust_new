mod comm;
//mod lib;
mod tool;
mod util;
use comm as cc;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{ErrorKind, Read};
use std::net::TcpListener;
use std::net::TcpStream;

use std::time::Duration;

use std::thread;
use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let t =   pool.execute(|| {
            handle_connection(stream);
        });
    }

}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
