use axum::Json;

pub async fn health() -> Json<&'static str> {
    Json("ok")
}