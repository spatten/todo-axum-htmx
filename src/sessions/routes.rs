use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use sqlx::{Pool, Postgres};
use tower_cookies::CookieManagerLayer;

use crate::utils::HtmlTemplate;

use super::templates;

// sessions routes, nested under /sessions
pub fn routes(pool: &Pool<Postgres>) -> Router {
    Router::new()
        .route("/login", get(new).post(create))
        // .route("/logout", delete(logout))
        .layer(CookieManagerLayer::new())
        .with_state(pool.clone())
}

async fn new() -> Result<impl IntoResponse, (StatusCode, String)> {
    let template = templates::render_new(None);
    Ok(HtmlTemplate(template))
}

async fn create() -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(Redirect::to("/").into_response())
}
