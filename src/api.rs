use axum::{routing::get, Router, extract::Path, Json};
use serde::Serialize;

use crate::core;

#[derive(Serialize)]
struct Response {
    output: String,
}

async fn generate(Path(size): Path<u32>) -> Json<Response> {
    Json(Response {
        output: core::generate(size),
    })
}

pub fn router() -> Router {
    Router::new().route("/generate/:size", get(generate))
}
