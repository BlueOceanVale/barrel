use axum::{Json, extract::multipart::Multipart, response::Response};
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

pub async fn file_format(
    mut multipart: Multipart,
) -> Response {
    loop {
        let Some(file) = match multipart.next_field() {
            Ok(file) => file,
            Ok(None) => break,
            Err(_) => {
                return Response::builder()
                    .header("Content-Type", "text/plain")
                    .body("multipart error".into())
                    .unwrap()
            }
        }
    }

}