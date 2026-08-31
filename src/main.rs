use axum::{Router, routing:: get};
use axum::Json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health))
        .unwrap();

    
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, app).await?;
}

fn health() -> Json<String> {
    Json("ok".to_string())
}

