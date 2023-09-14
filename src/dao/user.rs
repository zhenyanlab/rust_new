use crate::entity::user::P;
use actix_web::web;
use mysql;
use mysql::prelude::Queryable;
use web::Data;
pub fn get_all_user(pool: web::Data<mysql::Pool>) -> Vec<(i32, String, i32, String)> {
    let mut conn = pool.get_conn().unwrap();
    let resu: Vec<(i32, String, i32, String)> = conn
        .exec("select * from test.t_user", ())
        .unwrap_or_else(|_| vec![(0, "".to_string(), 0, "".to_string())]);
    resu
}
