use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::{Pool, Postgres};

use crate::utils::HtmlTemplate;

use super::templates;

// users routes, nested under /users
pub fn routes(pool: &Pool<Postgres>) -> Router {
    Router::new()
        .route("/new", get(users_new))
        .with_state(pool.clone())
}

pub async fn users_new() -> Result<impl IntoResponse, (StatusCode, String)> {
    let template = templates::render_new();
    Ok(HtmlTemplate(template))
}
