use actix_web::error;

mod comm {
    // 使用 include!
    include!("../src/comm.rs");
}

//mod lib;
mod util {
    include!("../src/util.rs");
}

mod tool {
    // 使用 include!
    include!("../src/tool/ttt.rs");
}

mod tool2 {
    // 使用 include!
    include!("../src/tool/httpclient.rs");
}

use mini_redis::{client, Result};

use comm as cc;

// #[test]
pub async fn testReidsAsync() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}

#[test]
fn httpclinetTest() {
    tool2::httpClient();
}

// extern  crate rust_new;
//use rust_new;
//error
// mod rust_new;
#[test]
fn testcase1() {
    println!("testcase1");
}

#[test]
fn testcase2() {
    println!("testcase2");
    // panic!("ERROR");
}

#[test]
fn testcase3() {
    println!("testcase3");
    let c = rust_new::wowo2(2, 3);
    println!("{}", rust_new::wowo2(2, 3));
    println!("{}", rust_new::com::wowo(2, 3));
}

#[test]
fn testcaseVec() {
    println!("testcaseVec");
    let vecc = vec![1, 2, 4, 6];
    let mut vecit = vecc.iter();
    //assert_eq!(vecit.next(),Some(&2));
    println!("netxt:{}", vecit.next().unwrap());
    for i in vecit {
        println!("it:{}", i);
    }
    println!("##################################");
}

#[test]
fn mainTest() {
    let a = "a";
    let mut b = String::new();
    b.push_str("b");
    let mut c: String = String::from("1");
    let mut d: String = "2".to_string();
    println!("{}", c + a);
    // println!("{}",c);
    let bb: &str = &b;
    let e = d.push_str(&bb);
    let f = format!("{}-{}-{}", a, b, d);
    println!("{}", f);
    for fc in f.chars() {
        println!("{}", fc);
    }
    println!("{:?}", "a");
    let aa = cc::dangle();
    println!("{}", aa);
    //let  aaa: &String = dangle2(&aa);
    let s1: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let s2: &str = &s1[0..5];
    let s3: &str = &s1[5..10];
    println!("{}={}={}", s1, s2, s3);
    let x1: cc::changfangxing = cc::changfangxing { chang: 6, kuan: 5 };
    let x2: cc::changfangxing = cc::changfangxing { chang: 5, kuan: 4 };
    println!("{}", x1.mianji());
    println!("{}", x1.da(&x2));
    let z1 = String::from("test");
    let z3: &str = "init";
    {
        let z2 = String::from("test1");
        let z3: &str = cc::langstr(z1.as_str(), z2.as_str());
        let z4 = cc::dangle2(&z1);
        println!("z4:{}", z4);
        // println!("z3:{}",z3);
    }
    println!("z3out:{}", z3);

    comm::cu1();

    println!("{}", tool::plus());
}
