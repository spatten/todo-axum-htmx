use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::todos;

pub async fn app() -> Router {
    // Connect to postgres
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://localhost/todo-axum-htmx")
        .await
        .expect("should be able to connect to DB");
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .expect("should be able to make a query");
    assert_eq!(row.0, 150);

    // Setup tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Serve files from the client directory, falling back to client/404.html
    let serve_dir = ServeDir::new("client").not_found_service(ServeFile::new("client/404.html"));

    // Respond to these routes, otherwise attempt to serve the file from the client directory
    // Also, add tracing of requests and add the postgres pool to the state so that our routes can use it
    Router::new()
        .nest("/todos", todos::responses::routes(&pool))
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
}
