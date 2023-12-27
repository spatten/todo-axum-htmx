mod db;
pub mod routes;
mod templates;

#[derive(Debug, Clone, Default)]
struct User {
    id: Option<i32>,
    email: String,
    password_hash: String,
    salt: String,
}
