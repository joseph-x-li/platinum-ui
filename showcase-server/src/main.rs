//! Tiny static server for the showcase: serves trunk's `dist/` and falls
//! back to `index.html` (200) for anything else, so the CSR app owns
//! routing. Configuration via env: `SHOWCASE_ADDR` (default 127.0.0.1:3050)
//! and `SHOWCASE_DIST` (default `showcase/dist`, resolved from the repo
//! root; `cargo run -p platinum-showcase-server` from the root Just Works
//! after `trunk build` in `showcase/`).

use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;

fn dist_dir() -> String {
    std::env::var("SHOWCASE_DIST").unwrap_or_else(|_| "showcase/dist".into())
}

async fn static_or_spa(uri: Uri) -> Response {
    let dir = dist_dir();
    let req_path = uri.path().trim_start_matches('/');

    // Tiny path-traversal guard — file lookups stay inside `dir`.
    if req_path.contains("..") {
        return (StatusCode::BAD_REQUEST, "bad path").into_response();
    }

    if !req_path.is_empty() {
        if let Ok(bytes) = tokio::fs::read(format!("{dir}/{req_path}")).await {
            return ([(header::CONTENT_TYPE, content_type(req_path))], bytes).into_response();
        }
    }

    match tokio::fs::read(format!("{dir}/index.html")).await {
        Ok(bytes) => ([(header::CONTENT_TYPE, "text/html; charset=utf-8")], bytes).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("index.html missing in {dir} — run `trunk build` in showcase/ first"),
        )
            .into_response(),
    }
}

fn content_type(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if lower.ends_with(".js") || lower.ends_with(".mjs") {
        "application/javascript"
    } else if lower.ends_with(".wasm") {
        "application/wasm"
    } else if lower.ends_with(".css") {
        "text/css"
    } else if lower.ends_with(".woff2") {
        "font/woff2"
    } else if lower.ends_with(".svg") {
        "image/svg+xml"
    } else if lower.ends_with(".png") {
        "image/png"
    } else {
        "application/octet-stream"
    }
}

#[tokio::main]
async fn main() {
    let addr = std::env::var("SHOWCASE_ADDR").unwrap_or_else(|_| "127.0.0.1:3050".into());
    let app = Router::new().fallback(get(static_or_spa));
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("bind {addr}: {e}"));
    println!("platinum-ui showcase on http://{addr} (dist: {})", dist_dir());
    axum::serve(listener, app).await.expect("serve");
}
