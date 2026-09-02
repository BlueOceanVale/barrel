use axum::{Json, Router};
use axum::routing::{get, post};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
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
) -> Json<ConvertResponse> {
    println!("🔥 1: HANDLER STARTED");

    println!("⏳ waiting for field...");
    let field = match multipart.next_field().await {
        Ok(Some(field)) => {
            println!("✅ 2: GOT FIELD");
            field
        }
        Ok(None) => {
            println!("❌ NO FIELD");
            return Json(ConvertResponse {
                output: "no field".to_string(),
            });
        }
        Err(e) => {
            println!("❌ MULTIPART ERROR: {:?}", e);
            return Json(ConvertResponse {
                output: "multipart error".to_string(),
            });
        }
    };

    println!("📦 field name: {:?}", field.name());

    println!("⏳ reading bytes...");
    let bytes = match field.bytes().await {
        Ok(bytes) => {
            println!("✅ 3: GOT {} BYTES", bytes.len());
            bytes
        }
        Err(e) => {
            println!("❌ BYTES ERROR: {:?}", e);
            return Json(ConvertResponse {
                output: "bytes error".to_string(),
            });
        }
    };

    println!("⏳ decoding image...");

    let cursor = Cursor::new(bytes);

    let reader = match ImageReader::new(cursor).with_guessed_format() {
        Ok(reader) => reader,
        Err(e) => {
            println!("❌ FORMAT ERROR: {:?}", e);
            return Json(ConvertResponse {
                output: "failed to detect image format".to_string(),
            });
        }
    };

    let img = match reader.decode() {
        Ok(img) => {
            println!("✅ 4: IMAGE DECODED");
            img
        }
        Err(e) => {
            println!("❌ DECODE ERROR: {:?}", e);
            return Json(ConvertResponse {
                output: "failed to decode image".to_string(),
            });
        }
    };

    println!("🎉 5: EVERYTHING WORKED");

    Json(ConvertResponse {
        output: "received and decoded image".to_string(),
    })
}

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


