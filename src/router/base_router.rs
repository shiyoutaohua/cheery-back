use crate::handler::base_handler;
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base_handler::greet))
        .route("/header", get(base_handler::header))
        .route("/headers", get(base_handler::headers))
        .route("/path/:key", get(base_handler::path))
        .route("/query", get(base_handler::query))
        .route("/download-file", get(base_handler::download_file))
        .route("/upload-file", post(base_handler::upload_file))
        .route("/post-text", post(base_handler::post_text))
        .route("/open-sse", get(base_handler::open_sse))
        .route("/ping-redis", get(base_handler::ping_redis))
        .route("/ping-zk", get(base_handler::ping_zk))
}
