use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use tokio_stream::{self as stream, StreamExt};

use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};



#[tokio::test]
async fn tokio_main_test() {
    println!("Hello, world!");

    let sp1 = tokio::spawn(async {
        read1().await;
    });
    let sp2 = tokio::spawn(async {
        read2().await;
    });

    let _ = tokio::join!(sp1, sp2);
}

async fn read1() -> String {
    sleep(Duration::new(4, 0));
    println!("1");
    String::from("1")
}

async fn read2() -> String {
    sleep(Duration::new(2, 0));
    println!("2");
    String::from("2")
}

// #[async_recursion]
async fn say_hello2() ->Option<i32>{
    println!("nihao!2");

    //panic!("E");
     Some(1/0)
    //Some(11)
  //  "".to_string()
}

// #[async_recursion]
async fn say_hello() ->Option<i32>{
    println!("nihao!");

    //panic!("E");
    // Some(1/0)
    let b = say_hello2().await.ok_or(100);
    Some(10+b.unwrap_or(300))
  //  "".to_string()
}
// use async_recursion::async_recursion;

fn print_type_of<T>(_: &T){
    println!("{}", std::any::type_name::<T>());
}
#[tokio::test]
async fn tokio_say_hello_test() ->Result<(),Box<dyn Error>> {

    let r =say_hello();
    println!("main-print-start");
    let b = r.await.ok_or(());
    print_type_of(&b);
    println!("sub-fn-return:{}",b.unwrap_or(300));
    // Ok("".to_string())
    Ok(())
}
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
#[tokio::test]
async fn tokio_file_test_async() {

    let file = File::open("hello.html").await;
    if file.is_err(){
        println!("file is not ex222");
    }else{

    }
    let file=file.unwrap();
    let mut reader = BufReader::new(file).lines();
    while let Some(line) = reader.next_line().await.unwrap() {
        println!("{}", line);
    }
}




#[tokio::test]
async fn tokio_test_stream_ex() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut stream = stream::iter(vec).map(|x| x * 2);

    while let Some(num) = stream.next().await {
        println!("{}", num);
    }
}


#[tokio::test]
async fn tokio_telnet_stream_ex() ->io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    let mut buffer = [0; 1024];
    stream.write_all(b"set a b\r\n").await?;
    let n = stream.read(&mut buffer).await?;
    let s = String::from_utf8_lossy(&buffer);
    println!("@@@The bytes read: {}", s);
    let p = tokio::time::timeout(std::time::Duration::from_secs(5),stream.write_all(b"info\r\n")).await?;
    print_type_of(&p);
     let n = stream.read(&mut buffer).await?;
    let s = String::from_utf8_lossy(&buffer);
    println!("@@@The bytes read: {}", s);
    Ok(())
}