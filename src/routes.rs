use crate::db::auth::*;
use poem::{Route, get, post};

#[poem::handler]
fn test() -> &'static str {
    "hello"
}

pub async fn routes() -> Route {
    Route::new()
        .at("/", get(test))
        // .at("/generate-refresh-token")
}
