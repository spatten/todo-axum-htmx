use askama::Template;

pub mod app;
pub mod todos;
pub mod users;
pub mod utils;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate {}
