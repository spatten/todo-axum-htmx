use axum::{extract::Query, response::Html, routing::get, Router};
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let serve_dir = ServeDir::new("client").not_found_service(ServeFile::new("assets/index.html"));
    let app = Router::new()
        .route("/search", get(search))
        .fallback_service(serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    search: String,
}

async fn search(Query(params): Query<SearchParams>) -> Html<String> {
    println!("{:?}", params);
    let result = params
        .search
        .split(',')
        .map(|s| format!("<li>{s}</li>"))
        .collect::<Vec<_>>()
        .join("\n");
    Html(result)
}
