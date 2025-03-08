use axum::response::IntoResponse;
use utoipa::OpenApi;

use crate::model;

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
    embedding: Vec<f32>,
}

impl Embedding {
    fn new(embedding: Vec<f32>) -> Self {
        Embedding { embedding }
    }
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
            (status = 500, description = "Embeddings computed failed"),
        )
    )]
async fn compute(
    axum::extract::State(model): axum::extract::State<std::sync::Arc<model::Bert>>,
    axum::Json(request): axum::Json<EmbeddingRequest>,
) -> impl axum::response::IntoResponse {
    match model.embed(request.input) {
        Ok(embeddings) => axum::Json(EmbeddingResponse {
            embeddings: embeddings
                .iter()
                .map(|embedding| Embedding::new(embedding.clone()))
                .collect(),
        })
        .into_response(),
        Err(err) => {
            tracing::error!("computing embeddings failed: {}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub fn router(model: model::Bert) -> axum::Router {
    let model = std::sync::Arc::new(model);
    let (router, api) = utoipa_axum::router::OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(
            "/model",
            utoipa_axum::router::OpenApiRouter::new()
                .routes(utoipa_axum::routes!(compute))
                .with_state(model),
        )
        .split_for_parts();
    router
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
