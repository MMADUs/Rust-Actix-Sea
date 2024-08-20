mod internal;

use actix_web::{web, App, HttpServer};

use internal::config::db::connect_to_db;
use internal::router::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = connect_to_db().await.expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
