use axum::{routing::get, Router};
use listenfd::ListenFd;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::{todos, users};

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
        .route("/", get(todos::routes::index))
        .nest("/todos", todos::routes::routes(&pool))
        .nest("/users", users::routes::routes(&pool))
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http())
}

pub async fn listener() -> TcpListener {
    // Auto-reload if you use `make watch`: https://github.com/tokio-rs/axum/blob/main/examples/auto-reload/src/main.rs
    let mut listenfd = ListenFd::from_env();
    match listenfd
        .take_tcp_listener(0)
        .expect("should be able to find an existing listener")
    {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            TcpListener::from_std(listener).expect("should be able to listen to existing listener")
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000")
            .await
            .expect("should be able to bind to 3000"),
    }
}
