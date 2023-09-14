use crate::dao::user::get_all_user;
use crate::entity::user::P;
use crate::tool::consts::*;
use actix_web::{error, get, middleware, web, App, HttpResponse, HttpServer, Responder};

fn print_type_of<T>(log: &str, _: &T) {
    println!("##{}##:{}", log, std::any::type_name::<T>())
}

#[get("/{name}")]
pub async fn hello(
    pool: web::Data<mysql::Pool>,
    redis_client: web::Data<redis::Client>,
    name: web::Path<String>,
) -> impl Responder {
    let resu = get_all_user(pool);
    let mut resuo: Vec<P> = vec![];
    let resuo2: Vec<P> = resu
        .iter()
        .map(|(id, name, age, adress)| {
            let pp = P {
                id: *id,
                name: name.to_string(),
                age: *age,
                address: adress.to_string(),
            };
            println!("{:?}", pp);
            //resuo.push(pp);
            pp
        })
        .collect();
    print_type_of("resuo2", &resuo2);
    let mapuserJson = serde_json::to_string(&resuo2).unwrap();
    println!("mapuserJson{:?}", resuo2);
    for (id, name, age, adress) in resu {
        let pp = P {
            id: id,
            name: name.to_string(),
            age: age,
            address: adress.to_string(),
        };
        println!("for-earch:{:?}", pp);
        resuo.push(pp);
    }
    let userJson = serde_json::to_string(&resuo).unwrap();
    println!("{}", &userJson);
    println!("{:?}", &resuo);

    let mut con = redis_client.get_connection().unwrap();
    let redis_value: String = redis::cmd("GET").arg(USER_REDIS_KEY).query(&mut con).unwrap();

    format!(
        "Hello{}  <br>  :you info: {} <br> !redisvalue:{} <br> &resuo2{}",
        &userJson, &name, &redis_value, &mapuserJson
    )
}
