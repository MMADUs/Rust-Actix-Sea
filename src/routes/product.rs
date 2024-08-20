use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .route("", web::get().to(get_products))
            // .route("/{id}", web::get().to(get_product))
    );
}

async fn get_products() -> impl Responder {
    HttpResponse::Ok().body("List of products")
}

// async fn get_product(path: web::Path<(u32,)>) -> impl Responder {
//     HttpResponse::Ok().body(format!("Product details for ID: {:?}", path.0))
// }