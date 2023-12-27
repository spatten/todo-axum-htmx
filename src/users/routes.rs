use crate::utils::HtmlTemplate;
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

use super::{db, templates, User};

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
pub struct UserForm {
    pub email: String,
    pub password: String,
    pub email_error: Vec<String>,
    pub password_error: Vec<String>,
    password_confirmation: String,
}

impl UserForm {
    async fn validate(mut self, pool: &PgPool) -> Result<Self, (StatusCode, String)> {
        // password validations
        if self.password.len() < 10 {
            self.password_error
                .push("passwords must be at least 10 characters long".to_string())
        }
        if self.password != self.password_confirmation {
            self.password_error
                .push("password and password confirmation must match".to_string())
        };

        // email validations
        let existing = db::find_by_email(self.email.clone(), pool).await?;
        if existing.is_some() {
            self.email_error
                .push("A user with this email already exists".to_string())
        }
        Ok(self)
    }

    fn is_valid(&self) -> bool {
        self.password_error.is_empty() && self.email_error.is_empty()
    }
}

impl TryFrom<UserForm> for User {
    type Error = UserForm;
    fn try_from(form: UserForm) -> Result<User, UserForm> {
        Ok(User {
            email: form.email,
            password_hash: form.password,
            salt: "1234".to_string(),
            id: None,
        })
    }
}

async fn create(
    State(pool): State<PgPool>,
    Form(form): Form<UserForm>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // validations
    let validated_form = form.validate(&pool).await?;
    if !validated_form.is_valid() {
        let template = templates::render_new();
        return Ok(HtmlTemplate(template).into_response());
    }

    // create
    db::create(validated_form, &pool).await?;

    // redirect
    Ok(Redirect::to("/").into_response())
}
