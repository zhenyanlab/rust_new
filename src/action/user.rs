use crate::dao::user::get_all_user;
use crate::entity::user::P;
use crate::tool::consts::*;
use actix_web::{error, get, middleware, web, App, HttpResponse, HttpServer, Responder};

fn print_type_of<T>(log: &str, _: &T) {
    println!("##{}##:{}", log, std::any::type_name::<T>())
}

#[get("/{name}")]
pub async fn hello(
    pool: web::Data<mysql_async::Pool>,
    redis_client: web::Data<redis::Client>,
    name: web::Path<String>,
) -> impl Responder {
    let resu = get_all_user(pool);
    let resu = resu.await.unwrap();
    println!("len:{}",resu.len());
    let userJson = serde_json::to_string(&resu).unwrap();
    println!("{}", &userJson);

    let resuo : Vec<_>  = resu.iter().map(|p|P{
        id:p.id+p.id,
        name:p.name.to_string() + p.name.as_str(),
        age:p.age+p.age,
        address:p.address.to_string() +p.name.as_str(),
    }).collect();
    println!("{:?}", &resuo);
    let mapuserJson = serde_json::to_string(&resuo).unwrap();

    let mut con = redis_client.get_connection().unwrap();
    let redis_value: String = redis::cmd("GET").arg(USER_REDIS_KEY).query(&mut con).unwrap();

    format!(
        "Hello{}  <br>  :you info: {} <br> !redisvalue:{} <br> &resuo2{}",
        &userJson, &name, &redis_value, &mapuserJson
    )
}
