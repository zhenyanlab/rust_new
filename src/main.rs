use actix_web::{get, web, App, HttpServer, Responder};
use mysql as my;
use mysql::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;
#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = my::Pool::new("mysql://mysqlroot:12345678@localhost:3306").unwrap();
    HttpServer::new(|| App::new().service(index).service(hello))
        .app_data(Data(pool))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


