pub mod product;
pub mod category;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(product::config)
            .configure(category::config)
    );
}