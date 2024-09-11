use actix_web::{web};

use crate::internal::handler::user as handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/register", web::post().to(handler::register))
            .route("/login", web::post().to(handler::login))
    );
}