use todo_axum_htmx::app;

#[tokio::main]
async fn main() {
    let app = app::app().await;
    let listener = app::listener().await;

    // Start serving
    axum::serve(listener, app)
        .await
        .expect("should be able to serve");
}
