use axum::http::StatusCode;
use sqlx::PgPool;

use crate::{users::model::User, utils};

use super::{model::salted_hash, templates::UserForm};

pub async fn create(params: UserForm, pool: &PgPool) -> Result<User, (StatusCode, String)> {
    let (password_hash, salt) = salted_hash(params.password.as_str())
        .map_err(|_| utils::internal_error_from_string("error while hashing password"))?;
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (email, password_hash, salt) VALUES ($1,$2,$3) returning id, email, password_hash, salt;",
        params.email,
        password_hash,
        salt
    )
    .fetch_one(pool)
    .await
    .map_err(utils::internal_error)?;

    Ok(user)
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
