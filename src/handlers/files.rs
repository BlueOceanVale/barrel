use axum::Json;
use crate::models::models::{FileInfo, FileQuery};
use std::path::Path;

// F1. File Info
pub async fn file_info(
    Json(quer): Json<FileQuery>,
) -> Json<FileInfo>{
    let path = Path::new(&quer.name);
    let extension = match path.extension() {
        Some(value) => value.to_string_lossy().to_string(),
        None => String::from("none"),
    };

    Json(FileInfo { name: quer.name, extension })
}
