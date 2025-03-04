use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub fn layer() -> tower_http::trace::TraceLayer<
    tower_http::classify::SharedClassifier<tower_http::classify::ServerErrorsAsFailures>,
    impl Fn(&axum::http::Request<axum::body::Body>) -> tracing::Span + Clone,
    impl Fn(&axum::http::Request<axum::body::Body>, &tracing::Span) + Clone,
    impl Fn(&axum::http::Response<axum::body::Body>, std::time::Duration, &tracing::Span) + Clone,
    impl Fn(&axum::body::Bytes, std::time::Duration, &tracing::Span) + Clone,
    impl Fn(Option<&axum::http::HeaderMap>, std::time::Duration, &tracing::Span) + Clone,
    impl Fn(tower_http::classify::ServerErrorsFailureClass, std::time::Duration, &tracing::Span) + Clone,
> {
    // `TraceLayer` is provided by tower-http so you have to add that as a dependency.
    // It provides good defaults but is also very customizable.
    //
    // See https://docs.rs/tower-http/0.1.1/tower_http/trace/index.html for more details.
    //
    // If you want to customize the behavior using closures here is how.
    tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<_>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<axum::extract::MatchedPath>()
                .map(axum::extract::MatchedPath::as_str);

            tracing::info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                status_code = tracing::field::Empty,
                latency = tracing::field::Empty,
            )
        })
        .on_request(|_request: &axum::http::Request<_>, _span: &tracing::Span| {
            // You can use `_span.record("some_other_field", value)` in one of these
            // closures to attach a value to the initially empty field in the info_span
            // created above.
            tracing::debug!("Get request");
        })
        .on_response(
            |response: &axum::response::Response,
             latency: std::time::Duration,
             span: &tracing::Span| {
                span.record("status_code", tracing::field::display(response.status()));
                span.record(
                    "latency",
                    tracing::field::display(format!("{}ms", latency.as_millis())),
                );
                tracing::info!("Replying");
            },
        )
        .on_body_chunk(
            |_chunk: &axum::body::Bytes, _latency: std::time::Duration, _span: &tracing::Span| {
                // ...
            },
        )
        .on_eos(
            |_trailers: Option<&axum::http::HeaderMap>,
             _stream_duration: std::time::Duration,
             _span: &tracing::Span| {
                // ...
            },
        )
        .on_failure(
            |_error: tower_http::classify::ServerErrorsFailureClass,
             _latency: std::time::Duration,
             _span: &tracing::Span| {
                // ...
            },
        )
}
