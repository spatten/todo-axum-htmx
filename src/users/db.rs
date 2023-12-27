use axum::http::StatusCode;
use sqlx::PgPool;

use crate::{users::User, utils};

use super::{salted_hash, templates::UserForm};

pub async fn create(params: UserForm, pool: &PgPool) -> Result<(), (StatusCode, String)> {
    let (password_hash, salt) = salted_hash(params.password.as_str())
        .map_err(|_| utils::internal_error_from_string("error while hashing password"))?;
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

pub async fn find_by_email(
    email: String,
    pool: &PgPool,
) -> Result<Option<User>, (StatusCode, String)> {
    sqlx::query_as!(User, "select * from users where email = $1", email)
        .fetch_optional(pool)
        .await
        .map_err(utils::internal_error)
}
