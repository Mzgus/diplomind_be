use diplomind::auth;
use poem::{Route, get};

#[poem::handler]
fn test() -> &'static str {
    "hello"
}

pub fn routes() -> Route {
    Route::new()
        .at("/", get(test))
        .at("/generate-refresh-token", get(auth::generate_refresh_token))
}
