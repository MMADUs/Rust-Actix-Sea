// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;
// use chrono::{NaiveDateTime};

// #[derive(Serialize, Deserialize, FromRow)]
// pub struct Product {
//     pub id: i32,
//     pub name: String,
//     pub description: Option<String>,
//     pub price: i32,
//     pub created_at: Option<NaiveDateTime>,
//     pub updated_at: Option<NaiveDateTime>,
// }
//
// #[derive(Deserialize)]
// pub struct CreateProduct {
//     pub name: String,
//     pub description: Option<String>,
//     pub price: i32,
// }
//
// #[derive(Deserialize)]
// pub struct UpdateProduct {
//     pub name: Option<String>,
//     pub description: Option<String>,
//     pub price: Option<i32>,
// }