mod models;
mod db;
mod serial;
mod api;
mod ws;
mod embedded;

use std::sync::{Arc, Mutex};
use axum::{
    routing::{get, post, delete},
    Router,
    http::{header, Uri},
    response::{IntoResponse, Response},
};
use rusqlite::Connection;
use tokio::sync::broadcast;
use tower_http::cors::{CorsLayer, Any};
use tracing_subscriber::EnvFilter;

use serial::{SerialState, SharedState, WsTx};

#[derive(Clone)]
pub struct AppState {
    pub serial: SharedState,
    pub db: Arc<Mutex<Connection>>,
    pub ws_tx: WsTx,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .init();

    let conn = Connection::open("serial_port.db").expect("Failed to open SQLite");
    db::init(&conn).expect("Failed to init DB");
    let db = Arc::new(Mutex::new(conn));

    let serial = Arc::new(Mutex::new(SerialState::new()));
    let (ws_tx, _) = broadcast::channel::<String>(256);

    serial::spawn_reader(serial.clone(), ws_tx.clone());

    let state = AppState { serial, db, ws_tx };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/ports", get(api::ports::get_ports))
        .route("/api/ports/open", post(api::ports::open))
        .route("/api/ports/close", post(api::ports::close))
        .route("/api/ports/status", get(api::ports::status))
        .route("/api/ports/display", post(api::ports::set_display_mode))
        .route("/api/send", post(api::ports::send))
        .route("/api/macros", get(api::macros::list))
        .route("/api/macros", post(api::macros::create))
        .route("/api/macros/:id", delete(api::macros::delete))
        .route("/api/capture/start", post(api::capture::start))
        .route("/api/capture/stop", post(api::capture::stop))
        .route("/api/logs", get(api::capture::get_logs))
        .route("/api/logs/clear", post(api::capture::clear_logs))
        .route("/ws", get(ws::handler))
        .fallback(static_handler)
        .layer(cors)
        .with_state(state);

    #[cfg(feature = "bundle-frontend")]
    tracing::info!("Serving bundled frontend at http://localhost:8080");

    let addr = "0.0.0.0:8080";
    tracing::info!("Listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[allow(unused_imports, unused_variables)]
async fn static_handler(uri: Uri) -> Response {
    #[cfg(feature = "bundle-frontend")]
    {
        use axum::http::StatusCode;
        use embedded::Frontend;

        let path = uri.path().trim_start_matches('/');
        let path = if path.is_empty() { "index.html" } else { path };

        if let Some(content) = Frontend::get(path) {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            return (
                [(header::CONTENT_TYPE, mime.as_ref().to_string())],
                content.data.to_vec(),
            )
                .into_response();
        }

        // SPA fallback — serve index.html for unknown paths
        if let Some(index) = Frontend::get("index.html") {
            return (
                [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
                index.data.to_vec(),
            )
                .into_response();
        }

        return StatusCode::NOT_FOUND.into_response();
    }

    #[cfg(not(feature = "bundle-frontend"))]
    {
        let _ = uri;
        (
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            "<html><body><p>Frontend not bundled. Run <code>./package.sh</code> to build.</p></body></html>",
        )
            .into_response()
    }
}
