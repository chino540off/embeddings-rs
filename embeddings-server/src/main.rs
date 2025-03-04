use std::path::PathBuf;

mod embeddings;
mod logging;
mod metrics;

#[tokio::main]
async fn main() {
    logging::init();

    let model = fastembed::TextEmbedding::try_new(
        fastembed::InitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
            .with_cache_dir(PathBuf::from("/tmp"))
            .with_execution_providers(vec![
                ort::execution_providers::cuda::CUDAExecutionProvider::default().build(),
                ort::execution_providers::cpu::CPUExecutionProvider::default().build(),
            ])
            .with_show_download_progress(false),
    )
    .unwrap();

    // build our application with a route
    let app = axum::Router::new()
        .merge(embeddings::router(model))
        .merge(metrics::router())
        .layer(logging::layer());

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
