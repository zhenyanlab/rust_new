use actix_web::{error, middleware, web, App, HttpResponse, HttpServer, Responder,get};
use mysql as my;
use mysql::prelude::Queryable;
use serde::Deserialize;
use serde::Serialize;

use redis::Client;
use redis::RedisResult;
use redis::ToRedisArgs;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(pool: web::Data<mysql::Pool>,redis_client : web::Data<redis::Client>,name: web::Path<String>) -> impl Responder {
        let mut  conn =pool.get_conn().unwrap();
        let resu:Vec<(i32, String, i32, String)>= conn.exec("select * from test.t_user",()).unwrap_or_else(|_|{vec!{(0,"".to_string(),0,"".to_string())}});

         let mut con = redis_client.get_connection().unwrap();
        let redis_value : String  = redis::cmd("GET").arg("my_key").query(&mut con).unwrap();

        let userJson = serde_json::to_string(&resu).unwrap();
        println!("{}",&userJson);
        format!("Hello{}:you info: {}!redisvalue:{}", &userJson,&name,&redis_value)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = my::Pool::new("mysql://mysqlroot:12345678@localhost:3306").unwrap();
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
        let _ : () = redis::cmd("SET").arg("my_key").arg(42).query(&mut con).unwrap();

    // throw away the result, just make sure it does not fail
        HttpServer::new(
        move || App::new()
                    .app_data(web::Data::new(pool.clone()))
                    .app_data(web::Data::new(client.clone()))
                    .wrap(middleware::Logger::default())
                    .service(index).service(hello)
    )
        .bind(("127.0.0.1", 9999))?
        .run()
        .await
}


