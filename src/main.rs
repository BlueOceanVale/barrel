use axum::{Json, Router};
use axum::routing::{get, post};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize)]
struct FileInfo {
    name: String,
    extension: String,
}

#[derive(Deserialize)]
struct FileQuery {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/files/info", post(file_info));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> Json<&'static str> {
    Json("ok")
}

async fn file_info(
    Json(quer): Json<FileQuery>,
) -> Json<FileInfo>{
    let path = Path::new(&quer.name);
    let extension = match path.extension() {
        Some(value) => value.to_string_lossy().to_string(),
        None => String::from("none"),
    };


    Json(FileInfo { name: quer.name, extension })
}

