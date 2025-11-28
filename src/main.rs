use diplomind::routes;

use dotenv::dotenv;
use poem::{EndpointExt, Server, listener::TcpListener};
use sqlx::PgPool;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgPool::connect(&db_url).await {
        Ok(pool) => {
            println!("Database connection pool created successfully");
            let routes = routes().data(pool);
            let _ = Server::new(TcpListener::bind("0.0.0.0:3000"))
                .run(routes)
                .await;
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to create database connection pool: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database connection pool creation failed",
            ));
        }
    }
}
