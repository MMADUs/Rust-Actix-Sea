use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::internal::model::category_model::{CreateCategory, Category, UpdateCategory};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/category")
            .route("", web::post().to(create_category))
            .route("", web::get().to(get_categories))
            .route("/{id}", web::get().to(get_category))
            .route("/{id}", web::put().to(update_category))
            .route("/{id}", web::delete().to(delete_category))
    );
}

async fn create_category(
    pool: web::Data<PgPool>,
    category: web::Json<CreateCategory>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Category,
        "INSERT INTO categories (name) VALUES ($1) RETURNING id, name, created_at, updated_at",
        category.name,
    )
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(category) => HttpResponse::Ok().json(category),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_categories(
    pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Category,
        "SELECT id, name, created_at, updated_at FROM categories",
    )
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_category(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Category,
        "SELECT id, name, created_at, updated_at FROM categories WHERE id = $1",
        id.into_inner(),
    )
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(category)) => HttpResponse::Ok().json(category),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_category(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
    category: web::Json<UpdateCategory>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Category,
        r#"
        UPDATE categories
        SET name = COALESCE($1, name), updated_at = NOW()
        WHERE id = $2
        RETURNING id, name, created_at, updated_at
        "#,
        category.name,
        id.into_inner(),
    )
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(category)) => HttpResponse::Ok().json(category),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_category(
    pool: web::Data<PgPool>,
    id: web::Path<i32>
) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM categories WHERE id = $1",
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
