// fn main() {
//     let a = 1;
//     let b = 2;
//     let c = a+b;
//     let mut d: i32 =-1;
//     d=c;
//     println!("Hello, world!{}",d);

//     println!("Hello, world!{}",c);
//     println!("Hello, world!{}", c);

// }

// #[derive(Debug)]
use std::any::{self, Any};
use actix_web::{web, App, HttpRequest,HttpResponse, HttpServer, Responder, post};


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let mut s1 = String::from("hello:");
    s1+= &req_body;
    HttpResponse::Ok().body(s1.to_string())
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!@@@", &name)
}

#[actix_web::main]
async fn main2() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .route("/", web::get().to(greet))
            .route("/a/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// 需要注意的是用'+'号去拼接字符串，在相加后first_name便丧失了所有权，后续不能再使用first_name变量了。从使用中也可以看到被加的last_name用的是引用，不会丧失所有权，后面还可以正常使用\
// 生命周期保证了结构体实例中引用数据的有效期不短于实例本身。
fn dangle()-> String{
    let s = String::from("test");
    return s;
}
fn dangle2<'a>(b : &'a String) -> &'a String{
    let s: String= String::from("test");

    // 当函数返回一个引用时，返回类型的生命周期参数必须要与其中
    // 一个参数的生命周期参数相匹配。当返回的引用没有 指向任何参数
    // 时，那么它只可能是指向了一个创建于函数内部的值，由于这个值会
    // 因为函数的结束而离开作用域，所以返回的内容也就变成了悬垂引
    // 用。下面来看一个无法通过编译的longest函数实现：
    // return &s;
    // 即便我们在上面的代码中为返回类型指定了生命周期参数'a，这
    // 个实现也依然无法通过编译，因为返回值的生命周期没有与任何参数
    // 的生命周期产生关联。
    
    return  b;
}
struct  changfangxing {
    chang:u32,
    kuan:u32
}
impl  changfangxing {
    pub fn mianji(&self)->u32{
        self.chang * self.kuan
    }
    pub fn da(self,other: &changfangxing) ->bool {
        self.chang>other.chang && self.kuan > other.kuan
    }
}
pub fn langstr<'a>(x: &'a str,y: &'a  str)->&'a  str {
    if x.len()>y.len(){
        x
    }else{
        y
    }
}

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
    println!("{:?}","a".type_id());
    let aa = dangle();
    println!("{}",aa);
    //let  aaa: &String = dangle2(&aa);
    let s1 = String::from("abcdefghijklmnopqrstuvwxyz");
    let s2 =&s1[0..5];
    let s3 =&s1[5..10];
    println!("{}={}={}",s1,s2,s3);
    let x1: changfangxing = changfangxing{chang:6,kuan:5};
    let x2: changfangxing = changfangxing{chang:5,kuan:4};
    println!("{}",x1.mianji());
    println!("{}",x1.da(&x2));
    let z1 = String::from("test");
    let z3 :&str = "init";
    {
        let z2 = String::from("test1");
        let z3 = langstr(z1.as_str(), z2.as_str());
        let z4 = dangle2(&z1);
        println!("z4:{}",z4);
        // println!("z3:{}",z3);

    }
    println!("z3out:{}",z3);
}