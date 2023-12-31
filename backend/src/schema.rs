// backend/src/schema.rs

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
    pub sku: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProductSchema {
    pub title: String,
    pub description: String,
    pub sku: u32,
    pub category: String,
    pub quantity: u32,
    pub price: u32,
    pub sale_price: u32,
    pub on_sale: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateProductSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub sku: Option<u32>,
    pub category: Option<String>,
    pub quantity: Option<u32>,
    pub price: Option<u32>,
    pub sale_price: Option<u32>,
    pub on_sale: Option<bool>,
}
