use crate::{users::model::User, BaseTemplate};
use askama::Template;
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginFormTemplate<'a> {
    _parent: &'a BaseTemplate,
    login_form: LoginForm,
}

pub fn render_new<'a>(form: Option<LoginForm>) -> LoginFormTemplate<'a> {
    let form = form.unwrap_or_default();
    LoginFormTemplate {
        _parent: &BaseTemplate {},
        login_form: form,
    }
}

#[derive(Template, Deserialize, Default, Debug)]
#[template(path = "login_form.html")]
pub struct LoginForm {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub email_errors: String,
    #[serde(default)]
    pub password_errors: String,
}

impl LoginForm {
    pub async fn attempt_login(
        mut self,
        pool: &PgPool,
    ) -> Result<(Option<User>, Self), (StatusCode, String)> {
        let user = crate::users::db::find_by_email(self.email.clone(), pool).await?;
        let bad_login_message = "No user found for that email/password combination".to_string();
        let Some(user) = user else {
            self.email_errors = bad_login_message;
            return Ok((None, self));
        };

        if user.authenticate(&self.password) {
            return Ok((Some(user), self));
        }

        self.email_errors = bad_login_message;
        Ok((None, self))
    }
}
