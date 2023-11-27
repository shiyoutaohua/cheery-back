use std::net::SocketAddr;

use axum::{http::StatusCode, response::IntoResponse, Router};
use tokio::signal;

pub async fn serve(router: Router, addr: SocketAddr) {
    let _ = axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown())
        .await;
}

pub async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler");
    };

    cfg_if::cfg_if! {
        if #[cfg(unix)] {
            use signal::unix::SignalKind;
            let terminate = async {
                signal::unix::signal(SignalKind::terminate())
                    .expect("failed to install signal handler")
                    .recv()
                    .await;
            };
        } else {
            let terminate = std::future::pending::<()>();
        }
    }

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}

/// 404 handler
pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "How ! 404.")
}
