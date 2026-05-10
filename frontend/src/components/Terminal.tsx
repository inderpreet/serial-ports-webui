"use client";
import { useState, useEffect, useRef } from "react";
import { api, DisplayMode } from "@/lib/api";

interface DataLine {
  id: number;
  timestamp: string;
  direction: "rx" | "tx";
  data: string;
  mode: string;
}

interface Props {
  portOpen: boolean;
  displayMode: DisplayMode;
}

let lineId = 0;

export default function Terminal({ portOpen, displayMode }: Props) {
  const [lines, setLines] = useState<DataLine[]>([]);
  const [input, setInput] = useState("");
  const [sendMode, setSendMode] = useState<DisplayMode>("ascii");
  const bottomRef = useRef<HTMLDivElement>(null);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    const ws = new WebSocket(`ws://localhost:8080/ws`);
    wsRef.current = ws;

    ws.onmessage = (evt) => {
      try {
        const msg = JSON.parse(evt.data);
        if (msg.type === "Data") {
          setLines((prev) => [
            ...prev.slice(-999),
            {
              id: lineId++,
              timestamp: new Date().toISOString().slice(11, 23),
              direction: "rx",
              data: msg.data.display,
              mode: msg.data.mode,
            },
          ]);
        }
      } catch {}
    };

    return () => ws.close();
  }, []);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [lines]);

  const handleSend = async () => {
    if (!input.trim() || !portOpen) return;
    const res = await api.send(input, sendMode);
    if (res.ok) {
      setLines((prev) => [
        ...prev.slice(-999),
        {
          id: lineId++,
          timestamp: new Date().toISOString().slice(11, 23),
          direction: "tx",
          data: input,
          mode: sendMode,
        },
      ]);
      setInput("");
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="flex flex-col h-full bg-gray-950 border border-gray-700 rounded">
      <div className="flex-1 overflow-y-auto p-3 font-mono text-xs space-y-0.5 min-h-0">
        {lines.map((line) => (
          <div key={line.id} className="flex gap-2">
            <span className="text-gray-600 shrink-0">{line.timestamp}</span>
            <span className={`shrink-0 font-bold ${line.direction === "rx" ? "text-green-400" : "text-yellow-400"}`}>
              {line.direction.toUpperCase()}
            </span>
            <span className={`break-all ${line.direction === "rx" ? "text-gray-200" : "text-gray-400"}`}>
              {line.data}
            </span>
          </div>
        ))}
        <div ref={bottomRef} />
      </div>

      <div className="border-t border-gray-700 p-2 flex gap-2">
        <select
          value={sendMode}
          onChange={(e) => setSendMode(e.target.value as DisplayMode)}
          className="bg-gray-800 text-gray-300 border border-gray-600 rounded px-2 py-1 text-xs font-mono"
        >
          <option value="ascii">ASCII</option>
          <option value="hex">HEX</option>
        </select>
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          disabled={!portOpen}
          placeholder={portOpen ? (sendMode === "hex" ? "DE AD BE EF" : "send data...") : "port closed"}
          className="flex-1 bg-gray-800 text-gray-200 border border-gray-600 rounded px-2 py-1 text-xs font-mono placeholder-gray-600 disabled:opacity-50"
        />
        <button
          onClick={handleSend}
          disabled={!portOpen || !input.trim()}
          className="btn-primary text-xs"
        >
          Send
        </button>
        <button
          onClick={() => setLines([])}
          className="btn-secondary text-xs"
        >
          Clear
        </button>
      </div>
    </div>
  );
}
