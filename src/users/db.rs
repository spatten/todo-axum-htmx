use axum::http::StatusCode;
use sqlx::PgPool;

use crate::utils;

use super::routes::UserCreateParams;

pub async fn create(params: UserCreateParams, pool: &PgPool) -> Result<(), (StatusCode, String)> {
    let password_hash = params.password;
    let salt = "01234".to_string();
    sqlx::query!(
        "INSERT INTO users (email, password_hash, salt) VALUES ($1,$2,$3);",
        params.email,
        password_hash,
        salt
    )
    .execute(pool)
    .await
    .map_err(utils::internal_error)?;

    Ok(())
}
