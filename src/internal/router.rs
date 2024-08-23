use actix_web::web;

use crate::internal::routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(routes::post::config)
            .configure(routes::profile::config)
    );
}