use common::model::product::{ErrorResponse, Product, ProductResponse};
use reqwasm::{
    self,
    http::{Request, Response},
};

pub async fn create_product(product_data: &str) -> Result<Product, String> {
    let response = match Request::post("http://localhost:8000/api/products")
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

// pub async fn get_all() -> Result<Vec<Product>, String> {
//     let request = Request::get("http://localhost:8000/api/products")
//         .header("Content-Type", "application/json")
//         .send();

//     let response: Response = match request.await {
//         Ok(response) => response,
//         Err(_) => return Err("Failed to make request".to_string()),
//     };

//     // let status_code = response.status();
//     // let text: String = match response.text().await {
//     //     Ok(text) => text,
//     //     Err(_) => return Err("Failed to retrieve response text".to_string()),
//     // };

//     if response.status() != 200 {
//         let error_response = response.json::<ErrorResponse>().await;
//         if let Ok(error_response) = error_response {
//             return Err(error_response.message);
//         } else {
//             return Err(format!("API error: {}", response.status()));
//         }
//     }

//     let res_json = response.json::<ProductListData>().await;
//     log::info!("{:?}", res_json);
//     match res_json {
//         Ok(data) => Ok(data.products),
//         Err(_) => Err("Failed to parse response".to_string()),
//     }
// }
