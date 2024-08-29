use actix_web::web;

use crate::internal::handler::profile as handler;
use crate::internal::middleware::{log1, log2};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/profile")
            .route("", web::post().to(handler::create_profile))
            .route(
                "",
                web::get()
                    .to(handler::get_profiles)
                    .wrap(log2::Middleware2)
                    .wrap(log1::Middleware1::new("Getting profiles".to_string())),
            )
            .route("/{id}", web::get().to(handler::get_profile))
            .route("/{id}", web::put().to(handler::update_profile))
            .route("/{id}", web::delete().to(handler::delete_profile)),
    );
}
