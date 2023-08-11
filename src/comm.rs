//mod lib;

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

// mod  ../lib;
// use crate::lib::com;
use rust_new::com;
use rust_new::wowo2;
mod tests {
    use super::*;
    //error
    // mod  lib;

    #[test]
    fn internal2() {
        // assert_eq!(4, com::wowo(2, 2));
        dangle();
        wowo2(2,3);
        com::wowo(2, 3);
        // assert_eq!(4, com::wowo(2, 2));
        // private function
        // assert_eq!(4, com::wohaha(2, 2));
    }
}
// use rust_new::u1;
// use rust_new::util;
use crate::util;
pub fn cu1() -> String{
   return  util::util::u1();
}


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
// 函数参数或方法参数中的生命周期被称为输入生命周期（input
// lifetime），而返回值的生命周期则被称为输出生命周期（output
// lifetime）。
// 在没有显式标注的情况下，编译器目前使用了3种规则来计算引用
// 的生命周期。第一条规则作用于输入生命周期，第二条和第三条规则
// 作用于输出生命周期。当编译器检查完这3条规则后仍有无法计算出生
// 命周期的引用时，编译器就会停止运行并抛出错误。这些规则不但对
// fn定义生效，也对impl代码块生效。
// 第一条规则是，每一个引用参数都会拥有自己的生命周期参数。
// 换句话说，单参数函数拥有一个生命周期参数：fn foo<'a>(x: &'a
// i32)；双参数函数拥有两个不同的生命周期参数：fn foo<'a, 'b>(x:
// &'a i32, y: &'b i32)；以此类推。
// 第二条规则是，当只存在一个输入生命周期参数时，这个生命周
// 期会被赋予给所有输出生命周期参数，例如fn foo<'a>(x: &'a i32)
// -> &'a i32。
// 第三条规则是，当拥有多个输入生命周期参数，而其中一个是
// &self或&mut self时，self的生命周期会被赋予给所有的输出生命周
// 期参数。这条规则使方法更加易于阅读和编写，因为它省略了一些不
// 必要的符号。

pub fn dangle()-> String{
    let s = String::from("test");
    return s;
}
pub  fn dangle2<'a>(b : &'a String) -> &'a String{
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
    //     这里的问题在于result在longest函数结束时就离开了作用域，并
    // 被清理。但我们依然在尝试从函数中返回一个指向result的引用。无
    // 论我们怎么改变生命周期参数，都无法阻止悬垂引用的产生，而Rust
    // 并不允许创建悬垂引用。在本例中，最好的解决办法就是返回一个持
    // 有自身所有权的数据类型而不是引用，这样就可以将清理值的责任转
    // 移给函数调用者了。
    // 从根本上说，生命周期语法就是用来关联一个函数中不同参数及
    // 返回值的生命周期的。一旦它们形成了某种联系，Rust就获得了足够
    // 的信息来支持保障内存安全的操作，并阻止那些可能会导致悬垂指针
    // 或其他违反内存安全的行为
    return  b;
}
pub struct  changfangxing {
    pub chang:u32,
    pub kuan:u32
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