mod logging;
mod metrics;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = axum::Router::new()
        .merge(logging::router())
        .merge(metrics::router())
        .route("/", axum::routing::get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> axum::response::Html<&'static str> {
    axum::response::Html("<h1>Hello, World!</h1>")
}
