use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{http, Router};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Debug)]
pub struct AppState {
    path: PathBuf,
}

pub async fn http_serve(dir: PathBuf, port: u16) -> anyhow::Result<()> {
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving directory {:?} on {}", dir, address);
    let state = AppState { path: dir.clone() };
    let app = Router::new()
        .route("/*path", get(index_handler))
        .nest_service("/tower-file-server", ServeDir::new(dir))
        .with_state(Arc::new(state));
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_handler(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let file = state.path.join(&path);
    info!("Requesting file {:?}", file);

    if !file.exists() {
        return (
            StatusCode::NOT_FOUND,
            headers,
            Body::from(format!("File {:?} not found", file)),
        );
    }

    if file.is_dir() {
        // If the path is a directory, we return a html page with the list of files
        let mut content = String::new();
        content.push_str("<html><body><ul>");
        let mut entries = match tokio::fs::read_dir(&file).await {
            Ok(entries) => entries,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    headers,
                    Body::from(format!("Error reading directory {:?}: {:?}", file, e)),
                );
            }
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            let path = path.strip_prefix(&state.path).unwrap();
            let name = entry.file_name();
            let name = name.to_string_lossy();
            content.push_str(&format!("<li><a href={:?}>{}</a></li>", path, name));
        }
        content.push_str("</ul></body></html>");
        headers.insert(http::header::CONTENT_TYPE, "text/html".parse().unwrap());
        return (StatusCode::OK, headers, Body::from(content));
    }

    headers.insert(http::header::CONTENT_TYPE, "text/plain".parse().unwrap());
    match tokio::fs::read_to_string(&file).await {
        Ok(content) => (StatusCode::OK, headers, Body::from(content)),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            headers,
            Body::from(format!("Error reading file {:?}: {:?}", file, e)),
        ),
    }
}
