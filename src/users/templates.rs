use crate::BaseTemplate;
use askama::Template;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "users_new.html")]
pub struct UsersNewTemplate<'a> {
    _parent: &'a BaseTemplate,
    users_form: UserForm,
}

pub fn render_new<'a>(form: Option<UserForm>) -> UsersNewTemplate<'a> {
    let form = form.unwrap_or_default();
    UsersNewTemplate {
        _parent: &BaseTemplate {},
        users_form: form,
    }
}

#[derive(Template, Deserialize, Default, Debug)]
#[template(path = "users_form.html")]
pub struct UserForm {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub email_errors: String,
    #[serde(default)]
    pub password_errors: String,
    pub password_confirmation: String,
}
