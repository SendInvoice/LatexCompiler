mod latex;
mod router;

use std::net::SocketAddr;

use axum::serve;
use tracing::info;

use crate::router::make_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = make_router();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Server listening on {}", addr);
    info!("OpenAPI documentation available at {}/swagger-ui/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    serve(listener, app).await.expect("Failed to start server");
    Ok(())
}
