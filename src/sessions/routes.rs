use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::Form;
use sqlx::{PgPool, Pool, Postgres};
use tower_cookies::{CookieManagerLayer, Cookies};

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
    cookies: Cookies,
    State(pool): State<PgPool>,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (user, validated_form) = form.attempt_login(&pool).await?;
    let Some(user) = user else {
        return Ok(HtmlTemplate(validated_form).into_response());
    };
    user.set_cookie(cookies)?;

    Ok(Redirect::to("/").into_response())
}
