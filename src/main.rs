mod internal;

use actix_web::{web, App, HttpServer, middleware};

use internal::config::db::connect_to_db;
use internal::router::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_conn = connect_to_db().await.expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_conn.clone()))
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Trim))
            .configure(config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}