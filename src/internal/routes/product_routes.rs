use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::internal::model::product_model::{CreateProduct, Product, UpdateProduct};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .route("", web::post().to(create_product))
            .route("", web::get().to(get_products))
            .route("/{id}", web::get().to(get_product))
            .route("/{id}", web::put().to(update_product))
            .route("/{id}", web::delete().to(delete_product))
    );
}

async fn create_product(
    pool: web::Data<PgPool>,
    product: web::Json<CreateProduct>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Product,
        "INSERT INTO products (name, description, price) VALUES ($1, $2, $3) RETURNING id, name, description, price, created_at, updated_at",
        product.name,
        product.description,
        product.price,
    )
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_products(
    pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Product,
        "SELECT id, name, description, price, created_at, updated_at FROM products",
    )
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_product(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Product,
        "SELECT id, name, description, price, created_at, updated_at FROM products WHERE id = $1",
        id.into_inner(),
    )
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_product(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    product: web::Json<UpdateProduct>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Product,
        r#"
        UPDATE products
        SET name = COALESCE($1, name), description = COALESCE($2, description), price = COALESCE($3, price), updated_at = NOW()
        WHERE id = $4
        RETURNING id, name, description, price, created_at, updated_at
        "#,
        product.name,
        product.description,
        product.price,
        id.into_inner(),
    )
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_product(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM products WHERE id = $1",
        id.into_inner(),
    )
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(ref deleted) if deleted.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}