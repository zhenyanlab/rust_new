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





use std::{sync::Arc};

use tokio::{sync::Notify, time::sleep};

#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();
    let notify_clone2 = notify.clone();
    let notify_clone3 = notify.clone();

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        notify_clone.notify_one(); //两秒后通知
        println!("notify one")
    });

    tokio::spawn(async move {
        if let Err(_) = tokio::time::timeout(Duration::from_secs(1), notify_clone2.notified()).await
        {
            println!("time out!"); //1秒后未收到通知而超时
        }
    });

    tokio::spawn(async move {
        if let Ok(_) = tokio::time::timeout(Duration::from_secs(3), notify_clone3.notified()).await
        {
            println!("recive notifyed!"); //等待三秒，但是在第二秒的时候收到了通知，会直接返回成功
        }
    })
        .await
        .unwrap();
}


fn main2() {
    println!("main-test-start");
    let pool = ThreadPool::new(4);

    for i in 1..100 {

        let t =  pool.execute(move || {
            let str ="thread-print".to_string()+&i.to_string();
            println!("{}","_________________________________");
            println!("{}",&str);
        });

    }
    pool.join();
    println!("main-test-end");
}

fn main3() {
    let pool = ThreadPool::new(100);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let t =   pool.execute(|| {
            handle_connection(stream);
        });
    }

}

// #[instrument(target = "handle_connection")]
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 2048];
   let reqLen = stream.read(&mut buffer).unwrap();

    //let mut buffer2 = Vec::new();
    //let reqLen2 = stream.read_to_end(&mut buffer2).unwrap();
    //let buf3 = buffer2.as_slice();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    println!("{}","_________________________________");
    println!("{}",reqLen.to_string());

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}






