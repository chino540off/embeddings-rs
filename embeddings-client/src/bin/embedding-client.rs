use bytes::Bytes;
use http_body_util::Full;
use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use tower::{Service, ServiceBuilder, ServiceExt};

#[tokio::main]
async fn main() {
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();
    let mut client = tower::ServiceBuilder::new()
        // Add tracing and consider server errors and client
        // errors as failures.
        .layer(tower_http::trace::TraceLayer::new(
            tower_http::classify::StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ))
        // Set a `User-Agent` header on all requests.
        .layer(tower_http::set_header::SetRequestHeaderLayer::overriding(
            http::header::USER_AGENT,
            http::HeaderValue::from_static("tower-http demo"),
        ))
        // Decompress response bodies
        .layer(tower_http::decompression::DecompressionLayer::new())
        // Wrap a `Client` in our middleware stack.
        // This is possible because `Client` implements
        // `tower::Service`.
        .service(client);

    // Make a request
    let request = http::Request::builder()
        .uri("http://example.com")
        .body(Full::<Bytes>::default())
        .unwrap();

    let response = client.ready().await.unwrap().call(request).await.unwrap();
}
