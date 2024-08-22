use actix_web::web;

use crate::internal::routes::{post};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(post::config)
            // .configure(post_routes::config)
    );
}