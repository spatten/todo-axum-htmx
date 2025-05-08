use std::env;

use todo_axum_htmx::app;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env var should be set");
    let app = app::app(&database_url).await;
    let listener = app::listener().await;

    // Start serving
    axum::serve(listener, app)
        .await
        .expect("should be able to serve");
}
