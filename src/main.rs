use axum::response::Response;
use axum::{Json, Router};
use axum::routing::{get, post};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::path::Path;
use axum::extract::Multipart;
use image::ImageReader;

mod handlers;

use handlers::pdf::pdf_merge;

#[derive(Serialize)]
struct FileInfo {
    name: String,
    extension: String,
}

#[derive(Deserialize)]
struct FileQuery {
    name: String,
}


#[derive(Serialize)]
struct ConvertResponse {
    output: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/files/info", post(file_info))
        .route("/pdf/merge", post(pdf_merge))
        .route("/images/convert", post(convert_image_handler));

    let listener = TcpListener::bind("127.0.0.1:5000").await?;
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


async fn convert_image_handler(
    mut multipart: Multipart,
) -> Response {
    let mut image_bytes = None;
    let mut format = None;

    while let Some(field)= match multipart.next_field().await {
        Ok(field) => {
            field
        }
        Ok(None) => {
            println!("❌ NO FIELD");
            return Response::builder()
                .header("Content-Type", "text/plain")
                .body("no field".into())
                .unwrap();
        }
        Err(e) => {
            println!("❌ MULTIPART ERROR: {:?}", e);
            return Response::builder()
                .header("Content-Type", "text/plain")
                .body("multipart error".into())
                .unwrap();
        }
    } {
        match field.name() {
            Some("file") => {
                image_bytes = Some(field.bytes().await.unwrap())
            }

            Some("format") => {
                format = Some(field.text().await.unwrap())
            }

            _ => {}
        }
    }
    let image_bytes = match image_bytes {
        Some(image_bytes) => image_bytes,
        None => return Response::builder()
            .header("Content-Type", "text/plain")
            .body("failed to get image bytes".into())
            .unwrap(),
    };

    let cursor = Cursor::new(image_bytes);

    let reader = match ImageReader::new(cursor).with_guessed_format() {
        Ok(reader) => reader,
        Err(e) => {
            println!("❌ FORMAT ERROR: {:?}", e);
            return Response::builder()
                .header("Content-Type", "text/plain")
                .body("failed to detect image format".into())
                .unwrap();
        }
    };

    let output_format = match format.as_deref() {
        Some("webp") => image::ImageFormat::WebP,
        Some("jpeg") => image::ImageFormat::Jpeg,
        Some("png") => image::ImageFormat::Png,
        _ => {
            return Response::builder()
                .header("Content-Type", "text/plain")
                .body("failed to match format".into())
                .unwrap()
        }
    };

    let content_type = match format.as_deref() {
        Some("webp") => "image/webp",
        Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        _ => "text/plain",
    };

    let img = match reader.decode() {
        Ok(img) => {
        println!("✅ 4: IMAGE DECODED");

        let mut output = Cursor::new(Vec::new());

        match img.write_to(&mut output, output_format) {
            Ok(_) => println!("✅ IMAGE ENCODED"),
            Err(e) => {
                println!("❌ ENCODE ERROR: {}", e);
                return Response::builder()
                    .header("Content-Type", "text/plain")
                    .body("failed to encode image".into())
                    .unwrap();
            }
        }

        let bytes = output.into_inner();
        
        return Response::builder()
            .header("Content-Type", content_type)
            .body(bytes.into())
            .unwrap();
    }
        Err(e) => {
            println!("DECODE ERROR: {:?}", e);
            return Response::builder()
                .header("Content-Type", "text/plain")
                .body("failed to decode image".into())
                .unwrap();
        }
    };

}
