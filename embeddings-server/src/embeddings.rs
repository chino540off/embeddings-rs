use axum::response::IntoResponse;
use embeddings_utils::encoding::Encode;
use utoipa::OpenApi;

use crate::models;
use crate::models::Embedder;

const TAG: &str = "embeddings";

#[derive(serde::Serialize, utoipa::ToSchema, Clone)]
#[serde(untagged)]
pub enum EncodedData {
    Float(Vec<f32>),
    Base64(String),
}

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
    embedding: EncodedData,
}

impl Embedding {
    fn new(embedding: Vec<f32>, encoding_format: &str) -> Self {
        Embedding {
            embedding: match encoding_format {
                "float" => EncodedData::Float(Vec::<f32>::encode(embedding)),
                "base64" => EncodedData::Base64(String::encode(embedding)),
                &_ => todo!("not implemented"),
            },
        }
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
    axum::extract::State(model): axum::extract::State<std::sync::Arc<models::Bert>>,
    axum::Json(request): axum::Json<EmbeddingRequest>,
) -> impl axum::response::IntoResponse {
    match model.embed(request.input) {
        Ok(embeddings) => axum::Json(EmbeddingResponse {
            embeddings: embeddings
                .iter()
                .map(|embedding| Embedding::new(embedding.clone(), &request.encoding_format))
                .collect(),
        })
        .into_response(),
        Err(err) => {
            tracing::error!("computing embeddings failed: {}", err);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub fn router(model: models::Bert) -> axum::Router {
    let model = std::sync::Arc::new(model);
    let (router, api) = utoipa_axum::router::OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(
            utoipa_axum::router::OpenApiRouter::new()
                .routes(utoipa_axum::routes!(compute))
                .with_state(model),
        )
        .split_for_parts();
    router
        .merge(utoipa_swagger_ui::SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
}
