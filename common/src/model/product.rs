use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub sku: i32,
    pub category: String,
    pub quantity: i32,
    pub price: i32,
    pub sale_price: i32,
    pub on_sale: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductData {
    pub product: Product,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductResponse {
    pub status: String,
    pub data: ProductData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductListResponse {
    pub status: String,
    pub results: i32,
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
