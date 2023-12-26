use askama::Template;

use crate::BaseTemplate;

#[derive(Template)]
#[template(path = "users_new.html")]
pub struct UsersNewTemplate<'a> {
    _parent: &'a BaseTemplate,
    users_create: UsersCreateTemplate,
}

pub fn render_new<'a>() -> UsersNewTemplate<'a> {
    UsersNewTemplate {
        _parent: &BaseTemplate {},
        users_create: render_user_create(),
    }
}

#[derive(Template)]
#[template(path = "users_create.html")]
pub struct UsersCreateTemplate {}

pub fn render_user_create() -> UsersCreateTemplate {
    UsersCreateTemplate {}
}
