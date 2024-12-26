use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug)]
pub struct AppState {
    path: PathBuf,
}

pub async fn http_serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving directory {:?} on {}", dir, address);

    let state = AppState { path: dir };
    let app = Router::new()
        .route("/*path", get(index_handler))
        .with_state(Arc::new(state));
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let file = state.path.join(&path);
    info!("Requesting file {:?}", file);

    if !file.exists() || !file.is_file() {
        return (StatusCode::NOT_FOUND, format!("File {:?} not found", file));
    }

    match tokio::fs::read_to_string(file).await {
        Ok(content) => (StatusCode::OK, content),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error reading file: {:?}", e),
        ),
    }
}
