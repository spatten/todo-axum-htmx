use crate::utils::HtmlTemplate;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::Form;
use sqlx::{PgPool, Pool, Postgres};

use super::{
    db,
    templates::{self, UserForm},
};

// users routes, nested under /users
pub fn routes(pool: &Pool<Postgres>) -> Router {
    Router::new()
        .route("/new", get(new).post(create))
        .with_state(pool.clone())
}

async fn new() -> Result<impl IntoResponse, (StatusCode, String)> {
    let template = templates::render_new(None);
    Ok(HtmlTemplate(template))
}

async fn create(
    State(pool): State<PgPool>,
    Form(form): Form<UserForm>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // validations
    let validated_form = form.validate(&pool).await?;
    if !validated_form.is_valid() {
        println!("user form is not valid!\n{:?}", validated_form);
        return Ok(HtmlTemplate(validated_form).into_response());
    }

    // create
    db::create(validated_form, &pool).await?;

    // redirect
    Ok(Redirect::to("/").into_response())
}
