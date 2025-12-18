use diplomind::{MyError, handlers, services};
use dotenv::dotenv;
use poem::{EndpointExt, Route, Server, get, listener::TcpListener, middleware::CookieJarManager, post};
use sqlx::PgPool;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(res) => res,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, MyError::EnvVarNotSet { entity: "DATABASE_URL" }.to_string()))
    };
    let secret = match std::env::var("JWT_SECRET") {
        Ok(res) => res,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, MyError::EnvVarNotSet { entity: "JWT_SECRET" }.to_string()))
    };
    let cookie_name = match std::env::var("COOKIE_NAME") {
        Ok(res) => res,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, MyError::EnvVarNotSet { entity: "COOKIE_NAME" }.to_string()))
    };
    let token_manager = services::auth::TokenManager::new(secret, cookie_name);
    match PgPool::connect(&db_url).await {
        Ok(pool) => {
            println!("Database connection pool created successfully");
            // use handlers::*;
            let routes = Route::new()
                .at("/", get(test))
                .at("/login", get(handlers::auth::login))
                .at("/refresh_tokens", get(handlers::auth::refresh_tokens))
                .data(pool)
                .data(token_manager)
                .with(CookieJarManager::new());
            
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

#[poem::handler]
fn test() -> &'static str {
    "hello"
}