// src/handler.rs

use crate::{
    product_model::ProductModel,
    schema::{CreateProductSchema, FilterOptions, UpdateProductSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
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
    let limit = opts.limit.unwrap_or(10);
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
async fn get_product_handler(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
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

            return HttpResponse::Ok().json(product_response);
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
        "INSERT INTO products (title,description,sku,category,quantity,price,sale_price,on_sale) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
        body.title.to_string(),
        body.description.to_string(),
        body.sku as i32,
        body.description.to_string(),
        body.quantity as i32,
        body.price as i32,
        body.sale_price as i32,
        body.on_sale
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(product) => {
            let product_response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "product": product
            })});

            return HttpResponse::Ok().json(product_response);
        }
        Err(err) => {
            if err
                .to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest().json(serde_json::json!({"status": "fail", "message": "That product already exists!"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", err)}));
        }
    }
}

// UPDATE PRODUCT
#[patch("/products/{sku}")]
async fn edit_product_handler(
    path: web::Path<u32>,
    body: web::Json<UpdateProductSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let product_sku = path.into_inner();
    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products WHERE sku = $1",
        product_sku as i32
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("Product with SKU: {}", product_sku);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let product = query_result.unwrap();

    let query_result = sqlx::query_as!(
        ProductModel,
        "UPDATE products SET title = $1, description = $2, category = $4, quantity = $5, price = $6, sale_price = $7, on_sale = $8 WHERE sku = $3 RETURNING *",
        body.title.to_owned().unwrap_or(product.title),
        body.description.to_owned().unwrap_or(product.description),
        product_sku as i32,
        body.category.to_owned().unwrap_or(product.category),
        body.quantity.to_owned().unwrap_or(product.quantity.try_into().unwrap()) as i32,
        body.price.to_owned().unwrap_or(product.price.try_into().unwrap()) as i32,
        body.sale_price.to_owned().unwrap_or(product.sale_price.try_into().unwrap()) as i32,
        body.on_sale.to_owned().unwrap_or(product.on_sale.try_into().unwrap()) as bool,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(product) => {
            let product_response = serde_json::json!({"status": "succcess", "data": serde_json::json!({
                "product": product
            })});

            return HttpResponse::Ok().json(product_response);
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

// DELETE PRODUCT
#[delete("/products/{sku}")]
async fn delete_product_handler(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let product_sku = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM products WHERE sku = $1", product_sku as i32)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Product with SKU: {} not found", product_sku);
        return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}));
    }

    HttpResponse::NoContent().finish()
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(product_list_handler)
        .service(create_product_handler)
        .service(get_product_handler)
        .service(edit_product_handler)
        .service(delete_product_handler);

    conf.service(scope);
}
