// frontend/src/api.rs

use common::{ErrorResponse, Product, ProductListResponse, ProductResponse};
use reqwasm::http;

pub async fn api_create_product(product_data: &str) -> Result<Product, String> {
    let response = match http::Request::post("http://localhost:8000/api/products")
        .header("Content-Type", "application/json")
        .body(product_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    let res_json = response.json::<ProductResponse>().await;
    match res_json {
        Ok(data) => Ok(data.data.product),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}
