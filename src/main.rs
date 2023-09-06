// src/main.rs

mod handler;
mod product_model;
mod schema;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("\n\t✅ Connected to database");
            pool
        }
        Err(err) => {
            println!("Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    };

    println!("\n\t🦀 Server started\n");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
            header::CONTENT_TYPE, 
            header::AUTHORIZATION, 
            header::ACCEPT
        ]).supports_credentials();

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

struct Product {
    pub title: String,
    pub description: String,
    pub sku: String,
    pub quantity: u32,
    pub price: u32,
    pub sale_price: u32,
}
