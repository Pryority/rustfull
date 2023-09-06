// src/product_model.rs

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ProductModel {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub sku: String,
    pub quantity: i32,
    pub price: i32,
    pub sale_price: i32,
}
