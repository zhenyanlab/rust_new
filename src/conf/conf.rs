use crate::action::user::hello;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}
