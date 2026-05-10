"use client";
import { useState, useEffect } from "react";
import { api, PortInfo, PortStatus, DisplayMode, BAUD_RATES } from "@/lib/api";

interface Props {
  onStatusChange: (status: PortStatus) => void;
  onDisplayModeChange: (mode: DisplayMode) => void;
  displayMode: DisplayMode;
}

export default function PortSelector({ onStatusChange, onDisplayModeChange, displayMode }: Props) {
  const [ports, setPorts] = useState<PortInfo[]>([]);
  const [selectedPort, setSelectedPort] = useState("");
  const [baudRate, setBaudRate] = useState(115200);
  const [status, setStatus] = useState<PortStatus | null>(null);
  const [error, setError] = useState("");

  const refreshPorts = async () => {
    const res = await api.ports.list();
    setPorts(res.ports);
  };

  const refreshStatus = async () => {
    const res = await api.ports.status();
    setStatus(res.status);
    onStatusChange(res.status);
  };

  useEffect(() => {
    refreshPorts();
    refreshStatus();
  }, []);

  const handleOpen = async () => {
    setError("");
    const res = await api.ports.open(selectedPort, baudRate);
    if (!res.ok) {
      setError(res.error ?? "Failed to open port");
    } else {
      await refreshStatus();
    }
  };

  const handleClose = async () => {
    await api.ports.close();
    await refreshStatus();
  };

  const handleDisplayMode = async (mode: DisplayMode) => {
    await api.ports.setDisplay(mode);
    onDisplayModeChange(mode);
  };

  const isOpen = status?.open ?? false;

  return (
    <div className="bg-gray-900 border border-gray-700 rounded p-4 space-y-3">
      <div className="flex items-center gap-2">
        <span className={`w-2 h-2 rounded-full ${isOpen ? "bg-green-400" : "bg-red-500"}`} />
        <span className="text-sm text-gray-300 font-mono">
          {isOpen ? `${status?.port_name} @ ${status?.baud_rate}` : "Disconnected"}
        </span>
      </div>

      <div className="flex gap-2 flex-wrap">
        <select
          value={selectedPort}
          onChange={(e) => setSelectedPort(e.target.value)}
          disabled={isOpen}
          className="bg-gray-800 text-gray-200 border border-gray-600 rounded px-2 py-1 text-sm font-mono"
        >
          <option value="">-- Select Port --</option>
          {ports.map((p) => (
            <option key={p.name} value={p.name}>{p.name}</option>
          ))}
        </select>

        <select
          value={baudRate}
          onChange={(e) => setBaudRate(Number(e.target.value))}
          disabled={isOpen}
          className="bg-gray-800 text-gray-200 border border-gray-600 rounded px-2 py-1 text-sm font-mono"
        >
          {BAUD_RATES.map((r) => (
            <option key={r} value={r}>{r}</option>
          ))}
        </select>

        <button onClick={refreshPorts} disabled={isOpen} className="btn-secondary">Refresh</button>

        {!isOpen ? (
          <button onClick={handleOpen} disabled={!selectedPort} className="btn-primary">Open</button>
        ) : (
          <button onClick={handleClose} className="btn-danger">Close</button>
        )}
      </div>

      <div className="flex gap-2">
        <span className="text-sm text-gray-400">Display:</span>
        {(["ascii", "hex"] as DisplayMode[]).map((mode) => (
          <button
            key={mode}
            onClick={() => handleDisplayMode(mode)}
            className={`px-2 py-0.5 rounded text-xs font-mono uppercase ${
              displayMode === mode
                ? "bg-blue-600 text-white"
                : "bg-gray-700 text-gray-300 hover:bg-gray-600"
            }`}
          >
            {mode}
          </button>
        ))}
      </div>

      {error && <p className="text-red-400 text-xs font-mono">{error}</p>}
    </div>
  );
}
