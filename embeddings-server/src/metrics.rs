pub fn router() -> axum::Router {
    let (prometheus_layer, metric_handle) = axum_prometheus::PrometheusMetricLayer::pair();

    axum::Router::new()
        .route(
            "/metrics",
            axum::routing::get(|| async move { metric_handle.render() }),
        )
        .layer(prometheus_layer)
}
