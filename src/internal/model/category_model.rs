// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;
// use chrono::{NaiveDateTime};

// #[derive(Serialize, Deserialize, FromRow)]
// pub struct Category {
//     pub id: i32,
//     pub name: String,
//     pub created_at: Option<NaiveDateTime>,
//     pub updated_at: Option<NaiveDateTime>,
// }
//
// #[derive(Deserialize)]
// pub struct CreateCategory {
//     pub name: String,
// }
//
// #[derive(Deserialize)]
// pub struct UpdateCategory {
//     pub name: String,
// }