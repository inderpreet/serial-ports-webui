use axum::{extract::State, Json};
use serde_json::{json, Value};
use crate::AppState;
use crate::models::{PortConfig, PortInfo, SendRequest, DisplayMode};
use crate::serial::{list_ports, open_port, close_port, send_data, parse_hex_string, format_bytes};

pub async fn get_ports() -> Json<Value> {
    let ports: Vec<PortInfo> = list_ports()
        .into_iter()
        .map(|p| PortInfo {
            name: p.port_name,
            port_type: format!("{:?}", p.port_type),
        })
        .collect();
    Json(json!({ "ports": ports }))
}

pub async fn open(
    State(state): State<AppState>,
    Json(config): Json<PortConfig>,
) -> Json<Value> {
    let mut serial = state.serial.lock().unwrap();
    match open_port(&mut serial, &config.port_name, config.baud_rate) {
        Ok(_) => {
            let status = serial.status();
            Json(json!({ "ok": true, "status": status }))
        }
        Err(e) => Json(json!({ "ok": false, "error": e })),
    }
}

pub async fn close(State(state): State<AppState>) -> Json<Value> {
    let mut serial = state.serial.lock().unwrap();
    close_port(&mut serial);
    Json(json!({ "ok": true }))
}

pub async fn status(State(state): State<AppState>) -> Json<Value> {
    let serial = state.serial.lock().unwrap();
    Json(json!({ "status": serial.status() }))
}

pub async fn send(
    State(state): State<AppState>,
    Json(req): Json<SendRequest>,
) -> Json<Value> {
    let bytes = match req.mode {
        DisplayMode::Hex => match parse_hex_string(&req.data) {
            Ok(b) => b,
            Err(e) => return Json(json!({ "ok": false, "error": e })),
        },
        DisplayMode::Ascii => req.data.into_bytes(),
    };

    let mut serial = state.serial.lock().unwrap();
    match send_data(&mut serial, &bytes) {
        Ok(n) => {
            if serial.capturing {
                let display = format_bytes(&bytes, &serial.display_mode);
                let mode_str = match serial.display_mode {
                    DisplayMode::Ascii => "ascii",
                    DisplayMode::Hex => "hex",
                };
                let db = state.db.lock().unwrap();
                let _ = crate::db::insert_log(&db, "tx", &display, mode_str);
            }
            Json(json!({ "ok": true, "bytes_sent": n }))
        }
        Err(e) => Json(json!({ "ok": false, "error": e })),
    }
}

pub async fn set_display_mode(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Json<Value> {
    let mode_str = body.get("mode").and_then(|v| v.as_str()).unwrap_or("ascii");
    let mut serial = state.serial.lock().unwrap();
    serial.display_mode = if mode_str == "hex" { DisplayMode::Hex } else { DisplayMode::Ascii };
    Json(json!({ "ok": true, "mode": mode_str }))
}
