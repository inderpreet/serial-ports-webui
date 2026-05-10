use axum::{extract::{State, Query}, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::AppState;
use crate::db;

pub async fn start(State(state): State<AppState>) -> Json<Value> {
    let mut serial = state.serial.lock().unwrap();
    serial.capturing = true;
    Json(json!({ "ok": true, "capturing": true }))
}

pub async fn stop(State(state): State<AppState>) -> Json<Value> {
    let mut serial = state.serial.lock().unwrap();
    serial.capturing = false;
    Json(json!({ "ok": true, "capturing": false }))
}

#[derive(Deserialize)]
pub struct LogsQuery {
    pub limit: Option<i64>,
}

pub async fn get_logs(
    State(state): State<AppState>,
    Query(q): Query<LogsQuery>,
) -> Json<Value> {
    let db = state.db.lock().unwrap();
    let limit = q.limit.unwrap_or(500);
    match db::get_logs(&db, limit) {
        Ok(logs) => Json(json!({ "logs": logs })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

pub async fn clear_logs(State(state): State<AppState>) -> Json<Value> {
    let db = state.db.lock().unwrap();
    match db::clear_logs(&db) {
        Ok(_) => Json(json!({ "ok": true })),
        Err(e) => Json(json!({ "ok": false, "error": e.to_string() })),
    }
}
