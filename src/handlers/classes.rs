use crate::{db, errors::MyError, middleware::{self, jwt_auth::AuthUser}, models::classes::*};
use poem::web::{Data, Json, Path};
use sqlx::{Pool, Postgres};

/// Create a new class (admin only)
#[poem::handler]
pub async fn create_class(
    Data(pool): Data<&Pool<Postgres>>,
    Json(data): Json<CreateClass>,
    auth_user: AuthUser,
) -> Result<Json<Class>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    
    let class = db::classes::create_class(pool, data).await?;
    Ok(Json(class))
}

/// Get a class by ID
#[poem::handler]
pub async fn get_class(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<Class>, MyError> {
    let class = db::classes::get_class_by_id(pool, id).await?;
    Ok(Json(class))
}

/// Get all classes
#[poem::handler]
pub async fn get_all_classes(
    Data(pool): Data<&Pool<Postgres>>,
    _auth_user: AuthUser, // Requires authentication
) -> Result<Json<Vec<Class>>, MyError> {
    let classes = db::classes::get_all_classes(pool).await?;
    Ok(Json(classes))
}

/// Update a class (admin only)
#[poem::handler]
pub async fn update_class(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(data): Json<UpdateClass>,
    auth_user: AuthUser,
) -> Result<Json<Class>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    
    let class = db::classes::update_class(pool, id, data).await?;
    Ok(Json(class))
}

/// Delete a class (admin only)
#[poem::handler]
pub async fn delete_class(
    Data(pool): Data<&Pool<Postgres>>,
    Path(id): Path<i32>,
    auth_user: AuthUser,
) -> Result<Json<Class>, MyError> {
    middleware::rbac::require_admin(&auth_user.0)?;
    
    let class = db::classes::delete_class(pool, id).await?;
    Ok(Json(class))
}
