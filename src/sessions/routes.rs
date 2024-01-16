use std::env;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{delete, get},
    Router,
};
use axum_extra::extract::Form;
use data_encoding::HEXUPPER;
use sqlx::{PgPool, Pool, Postgres};
use tower_cookies::{cookie::SameSite, Cookie, CookieManagerLayer, Cookies, Key};

use crate::{
    utils::{self, HtmlTemplate},
    SESSION_COOKIE_NAME,
};

use super::templates::{self, LoginForm};

// sessions routes, nested under /sessions
pub fn routes(pool: &Pool<Postgres>) -> Router {
    Router::new()
        .route("/login", get(new).post(create))
        .route("/logout", delete(logout))
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

async fn logout(cookies: Cookies) -> Result<impl IntoResponse, (StatusCode, String)> {
    let key = env::var("COOKIE_ENCRYPTION_KEY").map_err(utils::internal_error)?;
    let key = HEXUPPER
        .decode(key.as_bytes())
        .map_err(utils::internal_error)?;
    let key = Key::from(&key);
    let private = cookies.private(&key);

    let cookie = Cookie::build((SESSION_COOKIE_NAME, ""))
        .path("/")
        .secure(true)
        .same_site(SameSite::Strict)
        .http_only(true)
        .into();
    private.remove(cookie);

    Ok(Redirect::to("/sessions/new").into_response())
}
