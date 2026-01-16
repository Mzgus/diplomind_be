use diplomind::{MyError, handlers, services};
// use dotenvy::dotenv;
use poem::{EndpointExt, Route, Server, get, post, patch, listener::TcpListener, middleware::CookieJarManager};
use sqlx::PgPool;
use dotenv::dotenv;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    dotenv().unwrap();
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(res) => res,
        Err(_) => return Err(std::io::Error::other(MyError::EnvVarNotSet { entity: "DATABASE_URL" }.to_string()))
    };
    let secret = match std::env::var("JWT_SECRET") {
        Ok(res) => res,
        Err(_) => return Err(std::io::Error::other(MyError::EnvVarNotSet { entity: "JWT_SECRET" }.to_string()))
    };
    let cookie_name = match std::env::var("COOKIE_NAME") {
        Ok(res) => res,
        Err(_) => return Err(std::io::Error::other(MyError::EnvVarNotSet { entity: "COOKIE_NAME" }.to_string()))
    };
    let token_manager = services::auth::TokenManager::new(secret, cookie_name);
    match PgPool::connect(&db_url).await {
        Ok(pool) => {
            println!("Database connection pool created successfully");
            use diplomind::middleware::jwt_auth::JwtAuth;
            
            // Create the main route with all endpoints
            let routes = Route::new()
                // Public routes (no authentication required)
                .at("/", get(test))
                .at("/login", post(handlers::auth::login))
                .at("/refresh_tokens", get(handlers::auth::refresh_tokens))
                // Protected routes (authentication required) - add middleware per route
                .at("/logout", get(handlers::auth::logout).with(JwtAuth::new(token_manager.clone())))
                .at("/verify_token", get(handlers::auth::verify_token).with(JwtAuth::new(token_manager.clone())))
                // users routes (complete user information with JOIN)
                .at("/users", get(handlers::users::get_all_users).with(JwtAuth::new(token_manager.clone())))
                .at("/users/:id", get(handlers::users::get_user_by_id).with(JwtAuth::new(token_manager.clone())))
                .at("/users/email/:email", get(handlers::users::get_user_by_email).with(JwtAuth::new(token_manager.clone())))
                // users_sheets CRUD routes
                .at("/users_sheets", 
                    get(handlers::users_sheets::get_all_user_sheets)
                        .post(handlers::users_sheets::create_user_sheet)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                .at("/users_sheets/:id", 
                    get(handlers::users_sheets::get_user_sheet)
                        .put(handlers::users_sheets::update_user_sheet)
                        .delete(handlers::users_sheets::delete_user_sheet)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                // users_auth CRUD routes
                .at("/users_auth", 
                    post(handlers::users_auth::create_user_auth)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                .at("/users_auth/:id", 
                    get(handlers::users_auth::get_user_auth)
                        .delete(handlers::users_auth::delete_user_auth)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                .at("/users_auth/:id/email", 
                    patch(handlers::users_auth::update_user_auth_email)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                .at("/users_auth/:id/password", 
                    patch(handlers::users_auth::update_user_auth_password)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                // Admin security routes
                .at("/admin/users/:id/deactivate",
                    patch(handlers::admin::deactivate_user)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                .at("/admin/users/:id/activate",
                    patch(handlers::admin::activate_user)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                .at("/admin/security/revoke-all-sessions",
                    post(handlers::admin::revoke_all_sessions)
                        .with(JwtAuth::new(token_manager.clone()))
                )
                .data(pool)
                .data(token_manager)
                .with(CookieJarManager::new());
                
            Server::new(TcpListener::bind("0.0.0.0:3000"))
                .run(routes)
                .await?;
            return Ok(())
        }
        Err(e) => {
            eprintln!("Failed to create database connection pool: {}", e);
            return Err(std::io::Error::other(
                "Database connection pool creation failed",
            ));
        }
    }
}

#[poem::handler]
fn test() -> &'static str {
    "hello"
}