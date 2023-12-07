use axum::{extract::Query, response::Html, routing::get, Router};
use listenfd::ListenFd;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
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

    tracing_subscriber::fmt::init();
    // Serve files from the client directory, falling back to client/404.html
    let serve_dir = ServeDir::new("client").not_found_service(ServeFile::new("client/404.html"));

    // Respond to GET /search, otherwise attempt to serve the file from the client directory
    let app = Router::new()
        .route("/search", get(search))
        .fallback_service(serve_dir);

    // Auto-reload if you use `make watch`: https://github.com/tokio-rs/axum/blob/main/examples/auto-reload/src/main.rs
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd
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
    };
    axum::serve(listener, app)
        .await
        .expect("should be able to serve");
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    search: String,
}

async fn search(Query(params): Query<SearchParams>) -> Html<String> {
    println!("/search: {:?}", params);
    let result = params
        .search
        .split(',')
        .map(|s| format!("<li>{s}</li>"))
        .collect::<Vec<_>>()
        .join("\n");
    Html(result)
}
