#[derive(Debug, Clone)]
struct User {
    id: i32,
    email: String,
    hashed_password: String,
    salt: String,
}
