use actix_web::web;

use crate::internal::routes::{product_routes, post_routes};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(product_routes::config)
            .configure(post_routes::config)
    );
}