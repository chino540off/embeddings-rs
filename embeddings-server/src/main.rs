mod embeddings;
mod logging;
mod metrics;

#[tokio::main]
async fn main() {
    logging::init();

    // build our application with a route
    let app = axum::Router::new()
        .merge(embeddings::router("model"))
        .merge(metrics::router())
        .layer(logging::layer());

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
