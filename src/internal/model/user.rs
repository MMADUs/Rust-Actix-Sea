use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Serialize, Deserialize, Validate)]
pub struct UserRequest {
    #[validate(length(min = 5, max = 20, message = "Username must be between 5 and 20 characters"))]
    pub username: String,
    #[validate(length(min = 5, message = "Password must be at least 5 characters"))]
    pub password: String,
    #[validate(range(min = 1, message = "Roles cannot be empty"))]
    pub roles: i32,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}