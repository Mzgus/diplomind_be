use crate::{errors::MyError, models::accounts::{Account, CreateAccount}};
use sqlx::{PgExecutor, query_as};

pub async fn create_account<'e>(
    executor: impl PgExecutor<'e>,
    data: CreateAccount,
) -> Result<Account, MyError> {
    let account = query_as!(
        Account,
        "INSERT INTO accounts (email) VALUES ($1) RETURNING *",
        data.email
    )
    .fetch_one(executor)
    .await
    .map_err(|_e| MyError::DBErrors {
        entity: "Account creation failed",
    })?;

    Ok(account)
}

pub async fn get_account_by_email<'e>(
    executor: impl PgExecutor<'e>,
    email: &str,
) -> Result<Account, MyError> {
    let account = query_as!(
        Account,
        "SELECT * FROM accounts WHERE email = $1",
        email
    )
    .fetch_optional(executor)
    .await
    .map_err(|_e| MyError::DBErrors {
        entity: "Account fetch failed",
    })?
    .ok_or(MyError::NotFound { entity: "Account" })?;

    Ok(account)
}
