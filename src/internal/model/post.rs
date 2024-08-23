use serde::{Deserialize, Serialize};
use validator::{Validate};

#[derive(Serialize, Deserialize, Validate)]
pub struct PostRequest {
    #[validate(length(min = 5, max = 20, message = "Title must be between 5 and 20 characters"))]
    pub title: String,
    #[validate(length(min = 1, message = "Text cannot be empty"))]
    pub text: String,
    #[validate(range(min = 1, message = "Profile ID must be a positive integer"))]
    pub profile_id: i32,
}