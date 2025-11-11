mod compile;
mod healthcheck;

use axum::Router;

use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema, schema};

use crate::router::api::v0::compile::{CompileError, CompileRequest, CompileResponse};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    /// Error message
    #[schema(example = "An error occurred while processing your request")]
    pub message: String,
    /// Error code for client handling
    #[schema(example = "COMPILATION_ERROR")]
    pub code: Option<ApiErrorCode>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub enum ApiErrorCode {
    CompileError(CompileError),
}

#[derive(OpenApi)]
#[openapi(
    paths(
        compile::root::handler,
        healthcheck::root::handler,
    ),
    components(
        schemas(
            ApiError,
            ApiErrorCode,
            ApiError,
            CompileResponse,
            CompileRequest,
        )
    ),
    tags(
        (name = "compile", description = "Compiles LaTex Code into PDF documents"),
        (name = "healthcheck", description = "Health check endpoints for the API")
    ),
    info(
        title = "LaTex Compiler API",
        version = "0.1.0",
        description = "API for compiling LaTex documents into PDF format",
        contact(
            name = "API Support",
            email = "estebanborai@gmail.com"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
    )
)]
pub struct ApiV0Doc;

pub fn routes() -> Router {
    Router::new()
        .nest("/compile", compile::routes())
        .nest("/healthcheck", healthcheck::routes())
}
