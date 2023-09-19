use crate::entity::user::P;
use actix_web::web;
use mysql_async::prelude::*;
use web::Data;
use std::error::Error;

pub async fn get_all_user(pool: web::Data<mysql_async::Pool>) -> Result<Vec<P>, Box<dyn Error>> {
    // let resu: Vec<(i32, String, i32, String)> = conn
    //  .exec("select * from test.t_user", ())
    //     .unwrap_or_else(|_| vec![(0, "".to_string(), 0, "".to_string())]);
    let mut conn = pool.get_conn().await?;
    let loaded_payments = "select * from test.t_user"
        .with(())
        .map(&mut conn, |(id, name, age, address)| P { id, name, age, address })
        .await?;
    drop(conn);
    Ok(loaded_payments)
}
