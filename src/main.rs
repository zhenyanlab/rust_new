use std::fs::File;
use std::io::{ErrorKind, Read};
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod tool { // 使用 include!
    include!("../src/tool/httpclient.rs");  
}

fn main2() {
    let f: File = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    let mut buf = String::new();
    // f.read_to_string(&mut  buf);
    println!("{}",buf)
}
use std::error::Error;

#[tokio::main]
async fn main() {
    let res = reqwest::Client::new()
    .post("http://www.baidu.com")
    .form(&[("one", "1")])
    .send()
    .await
    .expect("send");
    println!("Response status {}", res.status());

    let body = res.text().await.unwrap();

    // println!("{}", body);

    let res = reqwest::Client::new().get("https://www.baidu.com").send().await.unwrap();
    println!("222Status: {}", res.status());
    let text = res.text().await.unwrap();
    println!("222:{}", text);



}



async fn main4()   {

    let b = tool::httpClient();
    //println!("{}",b);
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    // rt.block_on(async {
    //     println!("hello");
    // });

    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    // }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}


use mini_redis::{client, Result};

#[tokio::main]
async fn main3() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "@@@@@@@@@".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}

