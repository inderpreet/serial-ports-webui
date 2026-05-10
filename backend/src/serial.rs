use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use serialport::SerialPort;
use crate::models::{PortStatus, DisplayMode, WsMessage};

pub struct SerialState {
    pub port: Option<Box<dyn SerialPort>>,
    pub port_name: Option<String>,
    pub baud_rate: Option<u32>,
    pub capturing: bool,
    pub display_mode: DisplayMode,
}

impl SerialState {
    pub fn new() -> Self {
        Self {
            port: None,
            port_name: None,
            baud_rate: None,
            capturing: false,
            display_mode: DisplayMode::Ascii,
        }
    }

    pub fn status(&self) -> PortStatus {
        PortStatus {
            open: self.port.is_some(),
            port_name: self.port_name.clone(),
            baud_rate: self.baud_rate,
            capturing: self.capturing,
        }
    }
}

pub type SharedState = Arc<Mutex<SerialState>>;
pub type WsTx = broadcast::Sender<String>;

pub fn list_ports() -> Vec<serialport::SerialPortInfo> {
    serialport::available_ports().unwrap_or_default()
}

pub fn open_port(state: &mut SerialState, port_name: &str, baud_rate: u32) -> Result<(), String> {
    if state.port.is_some() {
        return Err("Port already open".into());
    }
    let port = serialport::new(port_name, baud_rate)
        .timeout(std::time::Duration::from_millis(10))
        .open()
        .map_err(|e| e.to_string())?;
    state.port = Some(port);
    state.port_name = Some(port_name.to_string());
    state.baud_rate = Some(baud_rate);
    Ok(())
}

pub fn close_port(state: &mut SerialState) {
    state.port = None;
    state.port_name = None;
    state.baud_rate = None;
    state.capturing = false;
}

pub fn send_data(state: &mut SerialState, data: &[u8]) -> Result<usize, String> {
    match &mut state.port {
        Some(port) => port.write(data).map_err(|e| e.to_string()),
        None => Err("Port not open".into()),
    }
}

pub fn format_bytes(bytes: &[u8], mode: &DisplayMode) -> String {
    match mode {
        DisplayMode::Ascii => String::from_utf8_lossy(bytes).to_string(),
        DisplayMode::Hex => bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" "),
    }
}

pub fn parse_hex_string(s: &str) -> Result<Vec<u8>, String> {
    s.split_whitespace()
        .map(|h| u8::from_str_radix(h, 16).map_err(|e| e.to_string()))
        .collect()
}

pub fn spawn_reader(shared: SharedState, tx: WsTx) {
    tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        loop {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            let result = {
                let mut state = shared.lock().unwrap();
                if let Some(port) = &mut state.port {
                    match port.read(&mut buf) {
                        Ok(n) if n > 0 => {
                            let data = buf[..n].to_vec();
                            let display = format_bytes(&data, &state.display_mode);
                            let mode_str = match state.display_mode {
                                DisplayMode::Ascii => "ascii",
                                DisplayMode::Hex => "hex",
                            };
                            Some(WsMessage::Data {
                                raw: data,
                                display,
                                mode: mode_str.to_string(),
                            })
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            };
            if let Some(msg) = result {
                let json = serde_json::to_string(&msg).unwrap_or_default();
                let _ = tx.send(json);
            }
        }
    });
}
