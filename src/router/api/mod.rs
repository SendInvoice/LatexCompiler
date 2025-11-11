use axum::Router;

pub mod v0;

pub fn make_api_router() -> Router {
    Router::new()
        .nest("/v0", v0::routes())
}
