use axum::Json;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::router::api::v0::ApiError;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    status: &'static str,
    timestamp: u64,
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/api/v0/healthcheck",
    responses(
        (status = 200, description = "Healthcheck Succeded", body = HealthResponse),
        (status = 500, description = "Failed to perform Healthcheck", body = ApiError)
    ),
    tag = "healthcheck"
)]
pub async fn handler() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "healthy",
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };

    Json(response)
}
