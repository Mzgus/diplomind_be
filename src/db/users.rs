use crate::errors::*;
use crate::handlers::users::UserPublic;

pub async fn get_all_users<'e>(
    executor: impl sqlx::PgExecutor<'e>,
) -> Result<Vec<UserPublic>, MyError> {
    let query = sqlx::query_as(
        r#"
        SELECT us.id, us.last_name, us.first_name, us.type_user, ua.email
        FROM "users_sheets" as us
        JOIN "users_auth" as ua ON us.id = ua.id_user_sheet
        ORDER BY us.last_name, us.first_name
        "#,
    );
    let users: Vec<UserPublic> = match query.fetch_all(executor).await {
        Ok(res) => res,
        Err(_err) => {
            return Err(MyError::DBErrors {
                entity: "Failed to fetch users",
            });
        }
    };
    Ok(users)
}
