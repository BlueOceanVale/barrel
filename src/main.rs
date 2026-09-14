use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;

mod handlers;
mod models;

use handlers::pdf::pdf_merge;
use handlers::app::health;
use handlers::files::file_info;
use handlers::image::{convert_image_handler, resize_image_handler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/files/info", post(file_info))
        .route("/pdf/merge", post(pdf_merge))
        .route("/images/convert", post(convert_image_handler))
        .route("/image/resize", post(resize_image_handler));

    let listener = TcpListener::bind("127.0.0.1:5000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}