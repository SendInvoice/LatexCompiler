mod api;

use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::router::api::v0::ApiV0Doc;

pub fn make_router() -> axum::Router {
    axum::Router::new()
        .nest("/api", api::make_api_router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiV0Doc::openapi()))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
