mod comm;
//mod lib;
mod  util;
mod tool;
use comm as cc;

fn  main() {
    let a = "a";
    let mut b = String::new();
    b.push_str("b");
    let mut c: String = String::from("1");
    let mut d: String = "2".to_string();
    println!("{}",c+a);
    // println!("{}",c);
    let bb : &str =&b;
    let e =d.push_str(&bb);
    let f = format!("{}-{}-{}",a,b,d);
    println!("{}",f);
    for  fc in f.chars() {
        println!("{}",fc);
    }
    println!("{:?}","a");
    let aa = cc::dangle();
    println!("{}",aa);
    //let  aaa: &String = dangle2(&aa);
    let s1: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let s2: &str =&s1[0..5];
    let s3: &str =&s1[5..10];
    println!("{}={}={}",s1,s2,s3);
    let x1: cc::changfangxing = cc::changfangxing{chang:6,kuan:5};
    let x2: cc::changfangxing = cc::changfangxing{chang:5,kuan:4};
    println!("{}",x1.mianji());
    println!("{}",x1.da(&x2));
    let z1 = String::from("test");
    let z3 :&str = "init";
    {
        let z2 = String::from("test1");
        let z3: &str = cc::langstr(z1.as_str(), z2.as_str());
        let z4 = cc::dangle2(&z1);
        println!("z4:{}",z4);
        // println!("z3:{}",z3);

    }
    println!("z3out:{}",z3);


    comm::cu1();

    println!("{}",tool::ttt::plus());
}

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

