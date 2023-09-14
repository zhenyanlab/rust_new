pub mod action;
pub mod conf;
pub mod dao;
pub mod entity;

use actix_web::{error, get, middleware, web, App, HttpResponse, HttpServer, Responder};
use chrono::prelude::*;
use mysql::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;

use redis::Client;
use redis::RedisResult;
use redis::ToRedisArgs;

#[get("/test")]
async fn index() -> impl Responder {
    "Hello, World!"
}

fn print_type_of<T>(log: &str, _: &T) {
    println!("##{}##:{}", log, std::any::type_name::<T>())
}

fn app_init() -> (mysql::Pool, redis::Client) {
    let pool = mysql::Pool::new("mysql://mysqlroot:12345678@localhost:3306").unwrap();
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let fmt = "%Y年%m月%d日 %H:%M:%S";
    let now = Local::now().format(fmt).to_string();
    print_type_of("current_datetime", &now);

    println!("Current datetime: {}", now);
    let _: () = redis::cmd("SET")
        .arg("start_time")
        .arg(now)
        .query(&mut con)
        .unwrap();
    return (pool, client);
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (pool, client) = app_init();

    // throw away the result, just make sure it does not fail
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client.clone()))
            .wrap(middleware::Logger::default())
            .service(index)
            .configure(conf::conf::config) // <- register resources
    })
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}
