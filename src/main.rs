use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use std::error::Error;

struct Product {
    pub title: String,
    pub description: String,
    pub sku: String,
    pub quantity: u32,
    pub price: u32,
    pub sale_price: u32,
}

async fn create(product: &Product, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query =
        "INSERT INTO product (title, description, sku, quantity, price, sale_price) VALUES ($1, $2, $3, $4, $5, $6)";

    sqlx::query(query)
        .bind(&product.title)
        .bind(&product.description)
        .bind(&product.sku)
        .bind(product.quantity as i32)
        .bind(product.price as i32) // Convert u32 to i32
        .bind(product.sale_price as i32) // Convert u32 to i32
        .execute(pool)
        .await?;

    Ok(())
}

async fn update(product: &Product, sku: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE product SET title = $1, description = $2, sku = $3, quantity = $4, price = $5, sale_price = $6 WHERE sku = $3";

    sqlx::query(query)
        .bind(&product.title)
        .bind(&product.description)
        .bind(&product.sku)
        .bind(product.quantity as i32)
        .bind(product.price as i32)
        .bind(product.sale_price as i32)
        .execute(pool)
        .await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    println!("\n\t🦀 Server started!\n");

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
    // let url = "postgres://root:secret@localhost:2345/postgres-rs?sslmode=disable";
    // let pool = sqlx::postgres::PgPool::connect(url).await?;

    // sqlx::migrate!("./migrations").run(&pool).await?;

    // let product = Product {
    //     title: "A Beautiful Painting".to_string(),
    //     description:
    //         "Owned by the Multi-billionaire Matthias, this painting has been passed down for generations."
    //             .to_string(),
    //     sku: "14003".to_string(),
    //     quantity: 15,
    //     price: 999999,
    //     sale_price: 899999,
    // };

    // create(&product, &pool).await?;
    // update(&product, &product.sku, &pool).await?;

    // Ok(())
}

#[get("/api/health")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "CRUD API with Rust, SQLx, Postgre, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}
