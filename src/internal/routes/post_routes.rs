use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection};
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};
use crate::internal::entity::post::{ActiveModel, Entity as PostEntity, Model as PostModel};
use serde_json::json;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("", web::post().to(create_post))
            .route("", web::get().to(get_posts))
            // .route("/{id}", web::get().to(get_category))
            // .route("/{id}", web::put().to(update_category))
            // .route("/{id}", web::delete().to(delete_category))
    );
}

#[derive(Serialize, Deserialize)]
struct PostRequest {
    title: String,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct PostResponse {
    id: i32,
    title: String,
    text: String,
}

async fn create_post(
    db: web::Data<DatabaseConnection>,
    post: web::Json<PostRequest>,
) -> impl Responder {
    let new_post = ActiveModel {
        title: Set(post.title.clone()),
        text: Set(post.text.clone()),
        ..Default::default()
    };

    let created_post = new_post.save(db.get_ref()).await.unwrap();

    HttpResponse::Created()
}

async fn get_posts(
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let posts = PostEntity::find()
        .all(db.get_ref())
        .await
        .unwrap();

    let posts = posts
        .into_iter()
        .map(|post| {
            json!({
        "id": post.id,
        "title": post.title,
        "text": post.text,
      })
        })
        .collect::<Vec<serde_json::Value>>();

    HttpResponse::Ok().json(posts)
}

// async fn create_category(
//     pool: web::Data<PgPool>,
//     category: web::Json<CreateCategory>,
// ) -> impl Responder {
//     let result = sqlx::query_as!(
//         Category,
//         "INSERT INTO categories (name) VALUES ($1) RETURNING id, name, created_at, updated_at",
//         category.name,
//     )
//         .fetch_one(pool.get_ref())
//         .await;
//
//     match result {
//         Ok(category) => HttpResponse::Ok().json(category),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// async fn get_categories(
//     pool: web::Data<PgPool>,
// ) -> impl Responder {
//     let result = sqlx::query_as!(
//         Category,
//         "SELECT id, name, created_at, updated_at FROM categories",
//     )
//         .fetch_all(pool.get_ref())
//         .await;
//
//     match result {
//         Ok(categories) => HttpResponse::Ok().json(categories),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// async fn get_category(
//     pool: web::Data<PgPool>,
//     id: web::Path<i32>,
// ) -> impl Responder {
//     let result = sqlx::query_as!(
//         Category,
//         "SELECT id, name, created_at, updated_at FROM categories WHERE id = $1",
//         id.into_inner(),
//     )
//         .fetch_optional(pool.get_ref())
//         .await;
//
//     match result {
//         Ok(Some(category)) => HttpResponse::Ok().json(category),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// async fn update_category(
//     pool: web::Data<PgPool>,
//     id: web::Path<i32>,
//     category: web::Json<UpdateCategory>,
// ) -> impl Responder {
//     let result = sqlx::query_as!(
//         Category,
//         r#"
//         UPDATE categories
//         SET name = COALESCE($1, name), updated_at = NOW()
//         WHERE id = $2
//         RETURNING id, name, created_at, updated_at
//         "#,
//         category.name,
//         id.into_inner(),
//     )
//         .fetch_optional(pool.get_ref())
//         .await;
//
//     match result {
//         Ok(Some(category)) => HttpResponse::Ok().json(category),
//         Ok(None) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
//
// async fn delete_category(
//     pool: web::Data<PgPool>,
//     id: web::Path<i32>
// ) -> impl Responder {
//     let result = sqlx::query!(
//         "DELETE FROM categories WHERE id = $1",
//         id.into_inner(),
//     )
//         .execute(pool.get_ref())
//         .await;
//
//     match result {
//         Ok(ref deleted) if deleted.rows_affected() > 0 => HttpResponse::NoContent().finish(),
//         Ok(_) => HttpResponse::NotFound().finish(),
//         Err(_) => HttpResponse::InternalServerError().finish(),
//     }
// }
