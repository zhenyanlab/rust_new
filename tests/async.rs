#![allow(dead_code)]

fn print_type_of<T>(log : &str,_: &T) {
    println!("##{}##:{}", log,std::any::type_name::<T>())
}
#[tokio::test]
async fn my_test() {
    let res = reqwest::Client::new()
    .post("http://www.baidu.com")
    .form(&[("one", "1")])
    .send()
    .await
    .expect("send");
    println!("Response status {}", res.status());

    let body = res.text().await.unwrap();
    print_type_of("body",&body);
    println!("111:{}", body.len());

    println!("111:{}", &body[0..100]);

    let res = reqwest::Client::new().get("https://www.baidu.com").send().await.unwrap();
    println!("222Status: {}", res.status());
    let text = res.text().await.unwrap();

    print_type_of("text",&text);
    println!("222:{}", &text[0..100]);



    assert!(true);
}

use mini_redis::{client, Result};
#[tokio::test]
async fn my_test_redis()  -> Result<()>{
     // Open a connection to the mini-redis address.
     let mut client = client::connect("127.0.0.1:6379").await?;

     // Set the key "hello" with value "world"
     client.set("hello", "@@@@@@@@@".into()).await?;
 
     // Get key "hello"
     let result = client.get("hello").await?;
 
     println!("got value from the server; result={:?}", result);
 
     Ok(())
}


use std::fs::File;
use std::io::{ErrorKind, Read};
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


#[test]
fn my_test_file_error(){
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

use threadpool::ThreadPool;

#[test]
fn my_test_thread_pool(){
        println!("main-test-start");
    let pool = ThreadPool::new(4);

    for i in 1..10 {

        let t =  pool.execute(move || {
            let str ="thread-print".to_string()+&i.to_string();
            println!("{}",&str);
        });

    }
    pool.join();
    println!("main-test-end");
}

#[test]
fn my_test_map(){
    let maybe_some_string = Some(String::from("Hello, World!"));
    // `Option::map` 按值获取 self，消耗 `maybe_some_string`
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));

    let x: Option<&str> = None;
    print_type_of("xx:",&x);
    let  y  = x.map(|s| s.len());
    print_type_of("yy:",&y);
    println!("{}",y.unwrap_or_default());
    assert_eq!(y, None);
    let kkk=10;
    assert_eq!(None.unwrap_or_else(|| 2 * kkk), 20);

    let x = Some(String::from("foo"));
    let y = x.ok_or(0);
    print_type_of("yyy:",&y);
    assert_eq!(y, Ok(String::from("foo")));

    let x: Option<&str> = None;
    print_type_of("xxx:",&x);
    assert_eq!(x.ok_or(0), Err(0));



}