mod db;
pub mod responses;
mod templates;

#[derive(Debug, Clone)]
struct Todo {
    id: i64,
    done: bool,
    description: String,
    position: i64,
}
