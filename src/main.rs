mod comm;
//mod lib;
mod  util;
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
    let s1 = String::from("abcdefghijklmnopqrstuvwxyz");
    let s2 =&s1[0..5];
    let s3 =&s1[5..10];
    println!("{}={}={}",s1,s2,s3);
    let x1: cc::changfangxing = cc::changfangxing{chang:6,kuan:5};
    let x2: cc::changfangxing = cc::changfangxing{chang:5,kuan:4};
    println!("{}",x1.mianji());
    println!("{}",x1.da(&x2));
    let z1 = String::from("test");
    let z3 :&str = "init";
    {
        let z2 = String::from("test1");
        let z3 = cc::langstr(z1.as_str(), z2.as_str());
        let z4 = cc::dangle2(&z1);
        println!("z4:{}",z4);
        // println!("z3:{}",z3);

    }
    println!("z3out:{}",z3);


    comm::cu1();
}