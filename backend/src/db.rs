use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::config;

pub async fn connect_db() -> Pool<Postgres> {
    config::db::setup_environment();

    let db_url = config::db::url();

    match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("\n\t✅ Connected to database");
            pool
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}
