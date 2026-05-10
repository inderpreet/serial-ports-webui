use axum::{extract::{State, Path}, Json};
use serde_json::{json, Value};
use crate::AppState;
use crate::models::CreateMacroRequest;
use crate::db;

pub async fn list(State(state): State<AppState>) -> Json<Value> {
    let db = state.db.lock().unwrap();
    match db::list_macros(&db) {
        Ok(macros) => Json(json!({ "macros": macros })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

pub async fn create(
    State(state): State<AppState>,
    Json(req): Json<CreateMacroRequest>,
) -> Json<Value> {
    let db = state.db.lock().unwrap();
    match db::create_macro(&db, &req) {
        Ok(m) => Json(json!({ "ok": true, "macro": m })),
        Err(e) => Json(json!({ "ok": false, "error": e.to_string() })),
    }
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<Value> {
    let db = state.db.lock().unwrap();
    match db::delete_macro(&db, &id) {
        Ok(true) => Json(json!({ "ok": true })),
        Ok(false) => Json(json!({ "ok": false, "error": "not found" })),
        Err(e) => Json(json!({ "ok": false, "error": e.to_string() })),
    }
}
