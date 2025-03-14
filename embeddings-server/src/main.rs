mod cmd;
mod embeddings;
mod logging;
mod metrics;
mod models;
mod utils;

use clap::Parser;

async fn serve(model: models::Bert, listening_addr: &str) {
    // build our application with a route
    let app = axum::Router::new()
        .merge(embeddings::router(model))
        .merge(metrics::router())
        .layer(logging::layer());

    // run it
    let listener = tokio::net::TcpListener::bind(listening_addr).await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    logging::init();
    let args = cmd::Arguments::parse();

    let model = models::builder(&args.model_id)
        .with_revision(&args.model_rev)
        .build();

    match args.command {
        cmd::Commands::Serve { listening_addr } => serve(model, &listening_addr).await,
        cmd::Commands::Prefetch {} => (),
    };
}
