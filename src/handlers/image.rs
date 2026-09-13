use axum::{extract::multipart::Multipart, response::Response};
use image::ImageReader;
use std::io::Cursor;

// I1. Convert Image
pub async fn convert_image_handler(
    mut multipart: Multipart,
) -> Response {
    let mut image_bytes = None;
    let mut format = None;

    loop {
        let Some(field) = (match multipart.next_field().await {
            Ok(field) => field,
            Err(e) => {
                println!("❌ MULTIPART ERROR: {:?}", e);
                return Response::builder()
                    .header("Content-Type", "text/plain")
                    .body("multipart error".into())
                    .unwrap();
            }
        }) else {
            break;
        };

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
        None => {
            return Response::builder()
                .header("Content-Type", "text/plain")
                .body("failed to get image bytes".into())
                .unwrap();
        }
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
                .unwrap();
        }
    };

    let content_type = match format.as_deref() {
        Some("webp") => "image/webp",
        Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        _ => "text/plain",
    };

    let img = match reader.decode() {
        Ok(img) => img,
        Err(e) => {
            println!("DECODE ERROR: {:?}", e);
            return Response::builder()
                .header("Content-Type", "text/plain")
                .body("failed to decode image".into())
                .unwrap();
        }
    };

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

    Response::builder()
        .header("Content-Type", content_type)
        .body(bytes.into())
        .unwrap()
}

pub async fn resize_image_handler(mut multipart: Multipart) -> Response {
    let mut image_bytes = None;

    loop {
        let Some(field) = (match multipart.next_field().await {
            Ok(field) => field,
            Err(_) => {
                return Response::builder()
                    .header("Content-Type", "text/plain")
                    .body("multipart error".into())
                    .unwrap();
            }
        }) else {
            break;
        };

        match field.name() {
            Some("file") => {
                image_bytes = Some(field.bytes().await.unwrap());
            }
            _ => {}
        }
    }

    match image_bytes {
        Some(_) => Response::builder()
            .header("Content-Type", "text/plain")
            .body("resize not implemented".into())
            .unwrap(),
        None => Response::builder()
            .header("Content-Type", "text/plain")
            .body("failed to get image bytes".into())
            .unwrap(),
    }
}