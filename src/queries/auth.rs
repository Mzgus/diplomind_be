use crate::models::*;
use sqlx::*;

pub async fn get_user_sheet_and_email<'e>(executor: impl sqlx::PgExecutor<'e>, user_auth_id: i32) -> Result<UserInfos, Error> {
    let mut query = sqlx::query_as(
        r#"
        SELECT (us.id, us.last_name, us.first_name, us.type_user, us.profile_picture, ua.email) from "users_sheets" as us 
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        WHERE ua.id = ($1)
    "#,
    );
    query = query.bind(user_auth_id);
    let row: UserInfos = match query.fetch_one(executor).await {
        Ok(res) => res,
        Err(err) => {
            return Err(err);
        } 
    };
    Ok(row)
}