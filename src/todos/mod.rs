mod db;
pub mod routes;
mod templates;

#[derive(Debug, Clone)]
struct Todo {
    id: i32,
    done: bool,
    description: String,
    position: i32,
}
