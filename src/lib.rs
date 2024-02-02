use askama::Template;

pub mod app;
pub mod sessions;
pub mod todos;
pub mod users;
pub mod utils;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate {}

static SESSION_COOKIE_NAME: &str = "SESSION";
