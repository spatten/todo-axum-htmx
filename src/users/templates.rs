use askama::Template;

use crate::BaseTemplate;

#[derive(Template)]
#[template(path = "users_new.html")]
pub struct UsersNewTemplate<'a> {
    _parent: &'a BaseTemplate,
    users_form: UsersFormTemplate,
}

pub fn render_new<'a>() -> UsersNewTemplate<'a> {
    UsersNewTemplate {
        _parent: &BaseTemplate {},
        users_form: render_user_create(),
    }
}

#[derive(Template)]
#[template(path = "users_form.html")]
pub struct UsersFormTemplate {}

pub fn render_user_create() -> UsersFormTemplate {
    UsersFormTemplate {}
}
