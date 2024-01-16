use crate::BaseTemplate;
use askama::Template;
use serde::Deserialize;

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
