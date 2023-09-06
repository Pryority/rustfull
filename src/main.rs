use sqlx::Row;
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
        .bind(product.price as i32) // Convert u32 to i32
        .bind(product.sale_price as i32) // Convert u32 to i32
        .execute(pool)
        .await?;

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://root:secret@localhost:2345/postgres-rs?sslmode=disable";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let product = Product {
        title: "A Repulsive Painting".to_string(),
        description:
            "Owned by the Late King Matthias, this painting has been passed down for generations."
                .to_string(),
        sku: "14003".to_string(),
        quantity: 15,
        price: 49999,
        sale_price: 44999,
    };

    create(&product, &pool).await?;

    Ok(())
}
