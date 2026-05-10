"use client";
import { useState } from "react";
import { api, LogEntry } from "@/lib/api";

interface Props {
  capturing: boolean;
  onCapturingChange: (v: boolean) => void;
}

export default function CapturePanel({ capturing, onCapturingChange }: Props) {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [showLogs, setShowLogs] = useState(false);

  const handleStart = async () => {
    await api.capture.start();
    onCapturingChange(true);
  };

  const handleStop = async () => {
    await api.capture.stop();
    onCapturingChange(false);
  };

  const handleLoadLogs = async () => {
    const res = await api.capture.logs();
    setLogs(res.logs);
    setShowLogs(true);
  };

  const handleClear = async () => {
    await api.capture.clear();
    setLogs([]);
  };

  return (
    <div className="bg-gray-900 border border-gray-700 rounded p-3 space-y-2">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <h2 className="text-sm font-mono text-gray-300 font-bold">Capture</h2>
          {capturing && (
            <span className="flex items-center gap-1">
              <span className="w-1.5 h-1.5 rounded-full bg-red-500 animate-pulse" />
              <span className="text-red-400 text-xs font-mono">REC</span>
            </span>
          )}
        </div>
        <div className="flex gap-2">
          {!capturing ? (
            <button onClick={handleStart} className="btn-primary text-xs">Start</button>
          ) : (
            <button onClick={handleStop} className="btn-danger text-xs">Stop</button>
          )}
          <button onClick={handleLoadLogs} className="btn-secondary text-xs">Logs</button>
          <button onClick={handleClear} className="btn-secondary text-xs">Clear</button>
        </div>
      </div>

      {showLogs && logs.length > 0 && (
        <div className="max-h-40 overflow-y-auto border border-gray-700 rounded p-2 space-y-0.5">
          {logs.map((l) => (
            <div key={l.id} className="flex gap-2 text-xs font-mono">
              <span className="text-gray-600 shrink-0">{l.timestamp.slice(11, 23)}</span>
              <span className={`shrink-0 ${l.direction === "rx" ? "text-green-400" : "text-yellow-400"}`}>
                {l.direction.toUpperCase()}
              </span>
              <span className="text-gray-300 break-all">{l.data}</span>
            </div>
          ))}
        </div>
      )}
      {showLogs && logs.length === 0 && (
        <p className="text-gray-600 text-xs font-mono text-center">No logs</p>
      )}
    </div>
  );
}
