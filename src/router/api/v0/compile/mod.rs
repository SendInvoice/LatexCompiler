pub mod root;

use axum::Router;
use axum::routing::post;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// Custom error type
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum CompileError {
    InvalidInput(String),
    CompilationFailed(String),
    Timeout,
    InternalError(String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompileOptions {
    #[serde(default)]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub output_format: Option<String>,
    #[serde(default)]
    pub optimization_level: Option<String>,
}

// Request payload structure
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompileRequest {
    pub text: String,
    #[serde(default)]
    pub options: CompileOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CompileResponse {
    pub success: bool,
    #[schema(example = "Compilation successful")]
    pub message: String,
    /// Output of the compilation, typically a PDF document in binary format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<u8>>,
    /// List of errors encountered during compilation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// List of warnings encountered during compilation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

pub fn routes() -> Router {
    Router::new().route("/", post(root::handler))
}
