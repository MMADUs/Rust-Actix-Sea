use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Serialize, Deserialize, Validate)]
pub struct ProfileRequest {
    #[validate(length(min = 5, max = 20, message = "Name must be between 5 and 20 characters"))]
    pub name: String,
    #[validate(length(min = 1, message = "Bio cannot be empty"))]
    pub bio: String,
}