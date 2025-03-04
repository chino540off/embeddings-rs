use utoipa::OpenApi;

const TAG: &str = "embeddings";

#[derive(utoipa::OpenApi)]
#[openapi(
        tags(
            (name = TAG, description = "embeddings computing API")
        )
    )]
struct ApiDoc;

#[derive(serde::Deserialize, utoipa::ToSchema)]
struct EmbeddingRequest {
    #[schema(example = json!(["What is a banana?", "Where can I buy fruits?"]))]
    input: Vec<String>,

    #[schema(example = "float")]
    encoding_format: String,
}

#[derive(serde::Serialize, utoipa::ToSchema, Clone)]
struct Embedding {
    embedding: Vec<f64>,
}

#[derive(serde::Serialize, utoipa::ToSchema, Clone)]
struct EmbeddingResponse {
    embeddings: Vec<Embedding>,
}

/// Compute embeddings
#[utoipa::path(
        post,
        path = "/embeddings",
        tag = TAG,
        responses(
            (status = 200, description = "Embeddings computed successfully", body = EmbeddingResponse),
        )
    )]
async fn compute(
    axum::Json(_request): axum::Json<EmbeddingRequest>,
) -> axum::Json<EmbeddingResponse> {
    // FIXME
    axum::Json(EmbeddingResponse {
        embeddings: vec![Embedding { embedding: vec![] }],
    })
}

pub fn router(model: &str) -> axum::Router {
    let (router, api) = utoipa_axum::router::OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(
            format!("/{}", model).as_str(),
            utoipa_axum::router::OpenApiRouter::new().routes(utoipa_axum::routes!(compute)),
        )
        .split_for_parts();
    router
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
