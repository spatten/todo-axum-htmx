use listenfd::ListenFd;

use todo_axum_htmx::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = app::app().await;

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

    // Start serving
    axum::serve(listener, app)
        .await
        .expect("should be able to serve");
}
