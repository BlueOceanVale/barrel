use axum::{Json, Router};
use axum::routing::{get, post};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use tokio::task::JoinError;
use std::io::Cursor;
use std::path::Path;
use axum::extract::Multipart;
use image::ImageReader;

#[derive(Serialize)]
struct FileInfo {
    name: String,
    extension: String,
}

#[derive(Deserialize)]
struct FileQuery {
    name: String,
}

#[derive(Deserialize)]
struct ConvertRequest {
    input: String,
    format: String,
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
        .route("/images/convert", post(convert_image_handler));

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

async fn convert_image_handler(
    mut multipart: Multipart,
) -> Json<ConvertResponse> {
    println!("1");
    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Err(_) => {
            return Json(ConvertResponse {
                output: "failed to read multipart".to_string(),
            });
            println!("2");
        }
    } {
        println!("{:?}", field.name());
        match field.bytes().await {
            Ok(bytes) => {
                let cursor = Cursor::new(bytes);
                let reader = ImageReader::new(cursor);
                let img = match reader.decode(){
                    Ok(img) => img,
                    Err(_) => return Json(ConvertResponse { output: "failed to get image".to_string() })
                };
                println!("received bytes");
            }
            Err(_) => println!("failed to read field"),
        };
    }

    println!("3");

    Json(ConvertResponse { output: "received multipart".to_string() })
    //let img = match image::open(req.input) {
    //    Ok(value) => value,
    //    Err(_) => return Json(ConvertResponse { output: "failed to convert image".to_string() })
    //};
    //let format = match req.format.as_str() {
    //    "webp" => image::ImageFormat::WebP,
    //    "png" => image::ImageFormat::Png,
    //    "jpeg" => image::ImageFormat::Jpeg,
    //    _ => return Json(ConvertResponse { output: "unsupported format".to_string() })
    //};
    //let form = format!("output.{}", req.format);
    //let saved_to = format!("image saved to output.{}", req.format);
    //match img.save_with_format(form, format) {
    //    Ok(_) => Json(ConvertResponse { output: saved_to }),
    //    Err(_) => Json(ConvertResponse { output: "failed to save image".to_string() }),
    //}
}

