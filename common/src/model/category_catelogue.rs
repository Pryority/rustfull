use super::product::Product;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CategoryCatelogueIdentifier {
    pub category: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewCategoryCatelogue {
    pub catelogue_id: String,
    pub title: String,
    pub category: String,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryCatelogue {
    pub title: Option<String>,
    pub category: Option<String>,
    pub products: Vec<Product>,
}
