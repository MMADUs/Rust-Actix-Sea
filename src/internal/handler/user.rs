use actix_web::{web, HttpResponse, Responder, Error, HttpRequest, HttpMessage};
use actix_web::error::{ErrorInternalServerError};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, QueryFilter};
use sea_orm::{EntityTrait, Set};
use serde_json::json;
use validator::{Validate};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

use crate::internal::model::{user::UserRequest, user::LoginRequest, error::ErrorResponse};
use crate::internal::entity::user::{ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel, Column as UserColumn};

pub async fn register(
    db: web::Data<DatabaseConnection>,
    user: web::Json<UserRequest>,
) -> impl Responder {
    if let Err(errors) = user.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let user_exist = UserEntity::find()
        .filter(UserColumn::Username.contains(&user.username))
        .one(db.get_ref())
        .await;

    match user_exist {
        Ok(Some(_)) => {
            let error_response = ErrorResponse {
                message: "Username has already been taken".to_string(),
            };

            Ok(HttpResponse::BadRequest().json(error_response))
        },
        Ok(None) => {
            let hashed_password = match hash(&user.password, DEFAULT_COST) {
                Ok(hashed) => hashed,
                Err(_) => {
                    let error_response = ErrorResponse {
                        message: "Failed to hash password".to_string(),
                    };

                    return Ok(HttpResponse::BadRequest().json(error_response))
                },
            };

            let new_user = UserActiveModel{
                username: Set(user.username.to_owned()),
                password: Set(hashed_password.to_owned()),
                roles: Set(user.roles.to_owned()),
                ..Default::default()
            }
                .insert(db.get_ref())
                .await;

            match new_user {
                Ok(profile) => Ok(HttpResponse::Created().json(profile)),
                Err(err) => Err(ErrorInternalServerError(err)),
            }
        },
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

pub async fn login (
    db: web::Data<DatabaseConnection>,
    credential: web::Json<LoginRequest>,
) -> impl Responder {
    if let Err(errors) = credential.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let user_exist = UserEntity::find()
        .filter(UserColumn::Username.contains(&credential.username))
        .one(db.get_ref())
        .await;

    match user_exist {
        Ok(Some(user)) => {
            match verify(&credential.password, &user.password) {
                Ok(is_valid) => {
                    if is_valid {
                        let token = generate_jwt(&user);
                        match token {
                            Ok(token) => Ok(HttpResponse::Ok().json(json!({
                                "token": token,
                                "message": "Login successful"
                            }))),
                            Err(err) => Err(ErrorInternalServerError(err)),
                        }
                    } else {
                        Ok(HttpResponse::Unauthorized().json("Invalid credentials"))
                    }
                },
                Err(err) => Err(ErrorInternalServerError(err)),
            }
        },
        Ok(None) => Ok(HttpResponse::Unauthorized().json("Invalid credentials")),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}

// private code

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

fn generate_jwt(user: &UserModel) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.username.clone(),
        exp: exp as usize,
        iat: Utc::now().timestamp() as usize,
    };

    let header = Header::new(Algorithm::HS256);
    encode(&header, &claims, &EncodingKey::from_secret("your_secret_key".as_ref()))
}