pub mod root;

use axum::Router;
use axum::routing::get;

pub fn routes() -> Router {
    Router::new().route("/", get(root::handler))
}
