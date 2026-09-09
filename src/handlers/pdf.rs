use axum::{extract::multipart::Multipart, response::Response};

pub async fn pdf_merge(
    mut multipart: Multipart,
) -> Response {
    let mut files = Vec::new();

    while let Some(field) = match multipart.next_field().await {
        Ok(field) => field,
        Ok(None) => break,
        Err(_) => {
            return Response::builder()
                .header("Content-Type", "text/plain")
                .body("multipart error".into())
                .unwrap();
        }
    } {
        if field.name() == Some("file") {
            match field.bytes().await {
                Ok(bytes) => files.push(bytes),
                Err(_) => {
                    return Response::builder()
                        .header("Content-Type", "text/plain")
                        .body("failed to read file".into())
                        .unwrap();
                }
            }
        }
    }

    println!("received {} PDFs", files.len());

    Response::builder()
        .header("Content-Type", "text/plain")
        .body("received PDFs".into())
        .unwrap()
}