# Web Serial Port

Portable WebUI for serial ports. Single binary — no install required on target machine.

- Select, open, and close serial ports with configurable baud rate
- Display incoming data as ASCII or HEX
- Send data buffers as ASCII or HEX
- Start/stop capture logging (persisted to SQLite)
- Create and run send buffer macros

**Stack:** Rust (axum) · Next.js 14 · SQLite · WebSocket

---

## Quick Start

### Prerequisites

| Tool | Install |
|------|---------|
| Rust | https://rustup.rs |
| Node.js ≥ 18 | https://nodejs.org |

**Linux only:** serial port access requires `dialout` group membership.
```bash
sudo usermod -aG dialout $USER
# log out and back in
```

---

## Build Single Binary

```bash
./package.sh
```

Builds the frontend as static files, embeds them into the Rust binary, and outputs:

```
backend/target/release/serial-port-backend
```

Run it:

```bash
./backend/target/release/serial-port-backend
```

Open **http://localhost:8080** in a browser.

---

## Development Mode

Run backend and frontend separately with hot reload.

**Backend** (port 8080):
```bash
cd backend
cargo run
```

**Frontend** (port 3000, proxies `/api` → `:8080`):
```bash
cd frontend
npm install
npm run dev
```

Open **http://localhost:3000**.

---

## API Reference

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/ports` | List available serial ports |
| POST | `/api/ports/open` | Open port `{ port_name, baud_rate }` |
| POST | `/api/ports/close` | Close current port |
| GET | `/api/ports/status` | Port status |
| POST | `/api/ports/display` | Set display mode `{ mode: "ascii"\|"hex" }` |
| POST | `/api/send` | Send data `{ data, mode: "ascii"\|"hex" }` |
| GET | `/api/macros` | List macros |
| POST | `/api/macros` | Create macro `{ name, data, mode }` |
| DELETE | `/api/macros/:id` | Delete macro |
| POST | `/api/capture/start` | Start logging |
| POST | `/api/capture/stop` | Stop logging |
| GET | `/api/logs?limit=500` | Get log entries |
| POST | `/api/logs/clear` | Clear logs |
| WS | `/ws` | Real-time serial data stream |

---

## Project Structure

```
serial-port/
├── package.sh              # Single-binary build script
├── backend/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # axum router, app state
│       ├── models.rs       # shared types
│       ├── serial.rs       # port management, background reader
│       ├── db.rs           # SQLite: macros + logs
│       ├── embedded.rs     # rust-embed (bundle-frontend feature)
│       ├── ws.rs           # WebSocket handler
│       └── api/
│           ├── ports.rs    # port + send endpoints
│           ├── macros.rs   # macro CRUD
│           └── capture.rs  # logging endpoints
└── frontend/
    └── src/
        ├── lib/api.ts              # typed API client
        ├── app/page.tsx            # main layout
        └── components/
            ├── PortSelector.tsx    # port, baud rate, display mode
            ├── Terminal.tsx        # live RX/TX display + send input
            ├── MacroPanel.tsx      # macro management
            └── CapturePanel.tsx    # capture controls + log viewer
```

---

## SQLite Database

Created automatically at `serial_port.db` in the working directory.

| Table | Contents |
|-------|----------|
| `macros` | Saved send buffer macros |
| `logs` | Capture log entries (timestamp, direction, data) |
