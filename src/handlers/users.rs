use crate::{db, errors, middleware::jwt_auth::AuthUser};
use poem::web::Data;
use poem::web::Json;
use sqlx::*;

#[derive(Debug, sqlx::FromRow, serde::Deserialize, serde::Serialize)]
pub struct UserPublic {
    pub id: i32,
    pub last_name: String,
    pub first_name: String,
    pub type_user: String,
    pub email: String,
}

#[poem::handler]
pub async fn get_all_users(
    Data(executor): Data<&Pool<Postgres>>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<Vec<UserPublic>>, errors::MyError> {
    match db::users::get_all_users(executor).await {
        Ok(users) => Ok(Json(users)),
        Err(err) => Err(err),
    }
}
