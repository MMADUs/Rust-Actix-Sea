use actix_web::{web};

use crate::internal::handler::post as handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("", web::post().to(handler::create_post))
            .route("", web::get().to(handler::get_posts))
            .route("/{id}", web::get().to(handler::get_post))
            .route("/{id}", web::put().to(handler::update_post))
            .route("/{id}", web::delete().to(handler::delete_post))
    );
}