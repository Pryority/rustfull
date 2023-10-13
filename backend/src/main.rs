// backend/src/main.rs

mod config;
mod db;
mod handler;
mod schema;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let pool = db::connect_db().await;

    println!("\n\t🦀 Server started\n");

    HttpServer::new(move || {
        config::cors::config();

        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(handler::config::config)
            .wrap(config::cors::config())
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
