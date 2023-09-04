use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

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

fn print_type_of<T>(_: &T) ->String{
    println!("{}", std::any::type_name::<T>());
    "".to_string()
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

