use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {
    pub port_name: String,
    pub baud_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DisplayMode {
    Ascii,
    Hex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRequest {
    pub data: String,
    pub mode: DisplayMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub id: String,
    pub name: String,
    pub data: String,
    pub mode: DisplayMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMacroRequest {
    pub name: String,
    pub data: String,
    pub mode: DisplayMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: i64,
    pub timestamp: String,
    pub direction: String,
    pub data: String,
    pub display_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub name: String,
    pub port_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortStatus {
    pub open: bool,
    pub port_name: Option<String>,
    pub baud_rate: Option<u32>,
    pub capturing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WsMessage {
    Data { raw: Vec<u8>, display: String, mode: String },
    Status(PortStatus),
    Error { message: String },
}
