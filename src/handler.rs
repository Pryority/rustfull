// src/handler.rs

use crate::{
    product_model::ProductModel,
    schema::{CreateProductSchema, FilterOptions, UpdateProductSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;

#[get("/health")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "CRUD API with Rust, SQLx, Postgre, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

// GET ALL PRODUCTS
#[get("/products")]
pub async fn product_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap();
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all product items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}));
    }

    let products = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "message": products.len(),
        "products": products,
    });
    HttpResponse::Ok().json(json_response)
}

// GET SINGLE PRODUCT
#[get("/products/{sku}")]
async fn get_product_handler(
    path: web::Path<u32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let product_sku = path.into_inner();
    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products where sku = $1",
        product_sku as i32
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(product) => {
            let product_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({"product": product })
            });

            return HttpResponse::Ok().json(product_response)
        }
        Err(_) => {
            let message = format!("Product with ID: {}", product_sku);
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "fail", 
                "message": message
            }));
        }
    }
}

// CREATE PRODUCT
#[post("/products")]
async fn create_product_handler(
    body: web::Json<CreateProductSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        ProductModel,
        "INSERT INTO products (title,description,sku,quantity,price,sale_price) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        body.title.to_string(),
        body.description.to_string(),
        body.sku as i32,
        body.quantity as i32,
        body.price as i32,
        body.sale_price as i32,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(product) => {
            let product_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "product": product
            })});

            return HttpResponse::Ok().json(product_response)
        }
        Err(err) => {
            if err.to_string().contains("duplicate key value violates unique constraint") {
                return HttpResponse::BadRequest().json(serde_json::json!({"status": "fail", "message": "That product already exists!"}));
            }

            return HttpResponse::InternalServerError().json(serde_json::json!({"status": "error", "message": format!("{:?}", err)}));
        }
    }
}

// TODO: Implement update_product_handler
// UPDATE PRODUCT


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(product_list_handler)
        .service(create_product_handler)
        .service(get_product_handler);

    conf.service(scope);
}
