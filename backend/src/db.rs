use rusqlite::{Connection, Result, params};
use crate::models::{Macro, LogEntry, DisplayMode, CreateMacroRequest};
use uuid::Uuid;

pub fn init(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS macros (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            data TEXT NOT NULL,
            mode TEXT NOT NULL DEFAULT 'ascii'
        );

        CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            direction TEXT NOT NULL,
            data TEXT NOT NULL,
            display_mode TEXT NOT NULL DEFAULT 'ascii'
        );
    ")?;
    Ok(())
}

pub fn list_macros(conn: &Connection) -> Result<Vec<Macro>> {
    let mut stmt = conn.prepare("SELECT id, name, data, mode FROM macros ORDER BY name")?;
    let rows = stmt.query_map([], |row| {
        let mode_str: String = row.get(3)?;
        Ok(Macro {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
            mode: if mode_str == "hex" { DisplayMode::Hex } else { DisplayMode::Ascii },
        })
    })?;
    rows.collect()
}

pub fn create_macro(conn: &Connection, req: &CreateMacroRequest) -> Result<Macro> {
    let id = Uuid::new_v4().to_string();
    let mode_str = match req.mode {
        DisplayMode::Hex => "hex",
        DisplayMode::Ascii => "ascii",
    };
    conn.execute(
        "INSERT INTO macros (id, name, data, mode) VALUES (?1, ?2, ?3, ?4)",
        params![id, req.name, req.data, mode_str],
    )?;
    Ok(Macro {
        id,
        name: req.name.clone(),
        data: req.data.clone(),
        mode: req.mode.clone(),
    })
}

pub fn delete_macro(conn: &Connection, id: &str) -> Result<bool> {
    let changed = conn.execute("DELETE FROM macros WHERE id = ?1", params![id])?;
    Ok(changed > 0)
}

pub fn insert_log(conn: &Connection, direction: &str, data: &str, display_mode: &str) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO logs (timestamp, direction, data, display_mode) VALUES (?1, ?2, ?3, ?4)",
        params![now, direction, data, display_mode],
    )?;
    Ok(())
}

pub fn get_logs(conn: &Connection, limit: i64) -> Result<Vec<LogEntry>> {
    let mut stmt = conn.prepare(
        "SELECT id, timestamp, direction, data, display_mode FROM logs ORDER BY id DESC LIMIT ?1"
    )?;
    let rows = stmt.query_map(params![limit], |row| {
        Ok(LogEntry {
            id: row.get(0)?,
            timestamp: row.get(1)?,
            direction: row.get(2)?,
            data: row.get(3)?,
            display_mode: row.get(4)?,
        })
    })?;
    let mut entries: Vec<LogEntry> = rows.collect::<Result<_>>()?;
    entries.reverse();
    Ok(entries)
}

pub fn clear_logs(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM logs", [])?;
    Ok(())
}
