mod embeddings;
mod logging;
mod metrics;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = axum::Router::new()
        .merge(logging::router())
        .merge(metrics::router())
        .merge(embeddings::router("model"));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
