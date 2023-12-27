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
    User,
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

impl UserForm {
    async fn validate(mut self, pool: &PgPool) -> Result<Self, (StatusCode, String)> {
        // password validations
        let mut password_errors = vec![];
        if self.password.len() < 10 {
            password_errors.push("passwords must be at least 10 characters long".to_string())
        }
        if self.password != self.password_confirmation {
            password_errors.push("password and password confirmation must match".to_string())
        };
        self.password_errors = password_errors.join(", ");

        // email validations
        let mut email_errors = vec![];
        let existing = db::find_by_email(self.email.clone(), pool).await?;
        if existing.is_some() {
            email_errors.push("A user with this email already exists".to_string())
        }
        self.email_errors = email_errors.join(", ");
        Ok(self)
    }

    fn is_valid(&self) -> bool {
        self.password_errors.is_empty() && self.email_errors.is_empty()
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
        println!("user form is not valid!\n{:?}", validated_form);
        return Ok(HtmlTemplate(validated_form).into_response());
    }

    // create
    db::create(validated_form, &pool).await?;

    // redirect
    Ok(Redirect::to("/").into_response())
}
