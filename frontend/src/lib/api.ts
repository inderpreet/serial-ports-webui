const BASE = "/api";

export type DisplayMode = "ascii" | "hex";

export interface PortInfo {
  name: string;
  port_type: string;
}

export interface PortStatus {
  open: boolean;
  port_name: string | null;
  baud_rate: number | null;
  capturing: boolean;
}

export interface Macro {
  id: string;
  name: string;
  data: string;
  mode: DisplayMode;
}

export interface LogEntry {
  id: number;
  timestamp: string;
  direction: string;
  data: string;
  display_mode: string;
}

async function req<T>(path: string, options?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, options);
  return res.json();
}

export const api = {
  ports: {
    list: () => req<{ ports: PortInfo[] }>("/ports"),
    open: (port_name: string, baud_rate: number) =>
      req<{ ok: boolean; error?: string }>("/ports/open", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ port_name, baud_rate }),
      }),
    close: () => req<{ ok: boolean }>("/ports/close", { method: "POST" }),
    status: () => req<{ status: PortStatus }>("/ports/status"),
    setDisplay: (mode: DisplayMode) =>
      req<{ ok: boolean; mode: string }>("/ports/display", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ mode }),
      }),
  },
  send: (data: string, mode: DisplayMode) =>
    req<{ ok: boolean; error?: string }>("/send", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ data, mode }),
    }),
  macros: {
    list: () => req<{ macros: Macro[] }>("/macros"),
    create: (name: string, data: string, mode: DisplayMode) =>
      req<{ ok: boolean; macro: Macro }>("/macros", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ name, data, mode }),
      }),
    delete: (id: string) =>
      req<{ ok: boolean }>(`/macros/${id}`, { method: "DELETE" }),
  },
  capture: {
    start: () => req<{ ok: boolean }>("/capture/start", { method: "POST" }),
    stop: () => req<{ ok: boolean }>("/capture/stop", { method: "POST" }),
    logs: (limit = 500) => req<{ logs: LogEntry[] }>(`/logs?limit=${limit}`),
    clear: () => req<{ ok: boolean }>("/logs/clear", { method: "POST" }),
  },
};

export const BAUD_RATES = [300, 1200, 2400, 4800, 9600, 19200, 38400, 57600, 115200, 230400, 460800, 921600];
