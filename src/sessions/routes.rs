use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::Form;
use sqlx::{PgPool, Pool, Postgres};
use tower_cookies::CookieManagerLayer;

use crate::utils::HtmlTemplate;

use super::templates::{self, LoginForm};

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

async fn create(
    State(pool): State<PgPool>,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (user, validated_form) = form.attempt_login(&pool).await?;
    println!("user after attempt_login: {:?}", user);
    if user.is_none() {
        return Ok(HtmlTemplate(validated_form).into_response());
    }

    Ok(Redirect::to("/").into_response())
}
