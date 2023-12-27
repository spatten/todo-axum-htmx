use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::Form;
use serde::Deserialize;
use sqlx::{PgPool, Pool, Postgres};

use crate::utils::HtmlTemplate;

use super::{db, templates};

// users routes, nested under /users
pub fn routes(pool: &Pool<Postgres>) -> Router {
    Router::new()
        .route("/new", get(new).post(create))
        .with_state(pool.clone())
}

async fn new() -> Result<impl IntoResponse, (StatusCode, String)> {
    let template = templates::render_new();
    Ok(HtmlTemplate(template))
}

#[derive(Deserialize)]
pub struct UserCreateParams {
    pub email: String,
    pub password: String,
    password_confirmation: String,
}

async fn create(
    State(pool): State<PgPool>,
    Form(params): Form<UserCreateParams>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // validations
    if params.password != params.password_confirmation {
        return Err((
            StatusCode::NOT_ACCEPTABLE,
            "password and password confirmation must match".to_string(),
        ));
    };

    // create
    db::create(params, &pool).await?;

    // redirect
    Ok(Redirect::to("/"))
}
