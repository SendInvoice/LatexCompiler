use axum::extract::Json;
use tracing::info;

use crate::latex::LatexCompiler;
use crate::router::api::v0::ApiError;
use crate::router::api::v0::compile::{
    CompileError, CompileOptions, CompileRequest, CompileResponse,
};

const MAX_TEXT_LENGTH: usize = 1_000_000; // 1MB limit

async fn compile(text: String, options: CompileOptions) -> Result<CompileResponse, CompileError> {
    let latex_compiler =
        LatexCompiler::new().map_err(|e| CompileError::InternalError(e.to_string()))?;

    info!("Compile function called with {} characters", text.len());
    info!("Compile options: {:?}", options);

    if text.trim().is_empty() {
        return Err(CompileError::InvalidInput(
            "Text cannot be empty".to_string(),
        ));
    }

    if text.len() > MAX_TEXT_LENGTH {
        return Err(CompileError::InvalidInput(
            "Text too large (max 1MB)".to_string(),
        ));
    }

    let output = latex_compiler
        .compile_text(text.as_str(), "main.pdf")
        .map_err(|e| CompileError::CompilationFailed(e.to_string()))?;

    info!(?output, "Compiled file generated");

    let bytes = std::fs::read(output).map_err(|e| CompileError::InternalError(e.to_string()))?;

    info!("Compiled file read into bytes, size: {} bytes", bytes.len());

    Ok(CompileResponse {
        success: true,
        message: "Compilation successful".to_string(),
        output: Some(bytes),
        errors: None,
        warnings: None,
    })
}

/// LaTex Compilation handler
#[utoipa::path(
    post,
    path = "/api/v0/compile",
    request_body = CompileRequest,
    responses(
        (status = 201, description = "Compilation Successful", body = Vec<u8>),
        (status = 400, description = "Failed to Compile LaTex", body = ApiError)
    ),
    tag = "compile"
)]
pub async fn handler(
    Json(payload): Json<CompileRequest>,
) -> Result<Vec<u8>, Json<CompileError>> {
    info!(
        "Received compile request for {} characters",
        payload.text.len()
    );

    let result = compile(payload.text, payload.options).await?;

    Ok(result.output.unwrap_or_default())
}
