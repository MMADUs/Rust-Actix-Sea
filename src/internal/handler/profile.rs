use actix_web::{web, HttpResponse, Responder, Error};
use actix_web::error::{ErrorInternalServerError};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult};
use sea_orm::{EntityTrait, Set};
use serde_json::json;
use validator::{Validate};

use crate::internal::model::profile::{ProfileRequest};
use crate::internal::entity::profile::{ActiveModel as ProfileActiveModel, Entity as ProfileEntity, Model as ProfileModel};
use crate::internal::entity::post::{ActiveModel as PostActiveModel, Entity as PostEntity, Model as PostModel};

pub async fn create_profile(
    db: web::Data<DatabaseConnection>,
    profile: web::Json<ProfileRequest>,
) -> impl Responder {
    if let Err(errors) = profile.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let new_profile = ProfileActiveModel{
        name: Set(profile.name.to_owned()),
        bio: Set(profile.bio.to_owned()),
        ..Default::default()
    }
        .insert(db.get_ref())
        .await;

    match new_profile {
        Ok(profile) => Ok(HttpResponse::Created().json(profile)),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub async fn get_profiles(
    db: web::Data<DatabaseConnection>,
) -> impl Responder {
    let profiles_result = ProfileEntity::find()
        .find_with_related(PostEntity)
        .all(db.get_ref())
        .await;

    match profiles_result {
        Ok(result) => {
            let formatted_result: Vec<_> = result
                .into_iter()
                .map(|(profile, posts)| {
                    let profile_json = serde_json::to_value(&profile).unwrap();
                    let posts_json = serde_json::to_value(&posts).unwrap();

                    json!({
                        "profile": profile_json,
                        "posts": posts_json
                    })
                })
                .collect();

            Ok(HttpResponse::Ok().json(formatted_result))
        },
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub async fn get_profile(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> impl Responder {
    let profile_result = ProfileEntity::find_by_id(id.into_inner())
        .one(db.get_ref())
        .await;

    match profile_result {
        Ok(Some(profile)) => HttpResponse::Ok().json(profile),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update_profile(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
    profile: web::Json<ProfileRequest>,
) -> impl Responder {
    let profile_result = ProfileEntity::find_by_id(id.into_inner())
        .one(db.get_ref())
        .await;

    match profile_result {
        Ok(Some(existing_profile)) => {
            let mut new_profile: ProfileActiveModel = existing_profile.into();
            new_profile.name = Set(profile.name.to_owned());
            new_profile.bio = Set(profile.bio.to_owned());

            let updated_profile = new_profile
                .update(db.get_ref())
                .await;

            match updated_profile {
                Ok(profile) => HttpResponse::Ok().json(profile),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        },
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn delete_profile(
    db: web::Data<DatabaseConnection>,
    id: web::Path<i32>,
) -> impl Responder {
    let delete_result = ProfileEntity::delete_by_id(id.into_inner())
        .exec(db.get_ref())
        .await;

    match delete_result {
        Ok(DeleteResult { rows_affected: 1 }) => HttpResponse::Ok().finish(),
        Ok(_) => HttpResponse::NotFound().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}