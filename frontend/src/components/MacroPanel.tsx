"use client";
import { useState, useEffect } from "react";
import { api, Macro, DisplayMode } from "@/lib/api";

interface Props {
  portOpen: boolean;
}

export default function MacroPanel({ portOpen }: Props) {
  const [macros, setMacros] = useState<Macro[]>([]);
  const [name, setName] = useState("");
  const [data, setData] = useState("");
  const [mode, setMode] = useState<DisplayMode>("ascii");
  const [creating, setCreating] = useState(false);

  const load = async () => {
    const res = await api.macros.list();
    setMacros(res.macros);
  };

  useEffect(() => { load(); }, []);

  const handleCreate = async () => {
    if (!name.trim() || !data.trim()) return;
    await api.macros.create(name, data, mode);
    setName("");
    setData("");
    setCreating(false);
    await load();
  };

  const handleDelete = async (id: string) => {
    await api.macros.delete(id);
    await load();
  };

  const handleRun = async (m: Macro) => {
    await api.send(m.data, m.mode);
  };

  return (
    <div className="bg-gray-900 border border-gray-700 rounded p-3 space-y-2">
      <div className="flex items-center justify-between">
        <h2 className="text-sm font-mono text-gray-300 font-bold">Macros</h2>
        <button onClick={() => setCreating((v) => !v)} className="btn-secondary text-xs">
          {creating ? "Cancel" : "+ New"}
        </button>
      </div>

      {creating && (
        <div className="space-y-2 border border-gray-700 rounded p-2">
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Macro name"
            className="w-full bg-gray-800 text-gray-200 border border-gray-600 rounded px-2 py-1 text-xs font-mono"
          />
          <div className="flex gap-2">
            <input
              type="text"
              value={data}
              onChange={(e) => setData(e.target.value)}
              placeholder={mode === "hex" ? "DE AD BE EF" : "data to send"}
              className="flex-1 bg-gray-800 text-gray-200 border border-gray-600 rounded px-2 py-1 text-xs font-mono"
            />
            <select
              value={mode}
              onChange={(e) => setMode(e.target.value as DisplayMode)}
              className="bg-gray-800 text-gray-300 border border-gray-600 rounded px-2 py-1 text-xs font-mono"
            >
              <option value="ascii">ASCII</option>
              <option value="hex">HEX</option>
            </select>
          </div>
          <button onClick={handleCreate} className="btn-primary text-xs w-full">Save</button>
        </div>
      )}

      <div className="space-y-1 max-h-48 overflow-y-auto">
        {macros.length === 0 && (
          <p className="text-gray-600 text-xs font-mono text-center py-2">No macros</p>
        )}
        {macros.map((m) => (
          <div key={m.id} className="flex items-center gap-2 group">
            <button
              onClick={() => handleRun(m)}
              disabled={!portOpen}
              className="flex-1 text-left bg-gray-800 hover:bg-gray-700 disabled:opacity-40 border border-gray-700 rounded px-2 py-1 text-xs font-mono text-gray-200"
            >
              <span className="text-blue-400">{m.name}</span>
              <span className="text-gray-500 ml-2 text-xs uppercase">[{m.mode}]</span>
              <span className="text-gray-500 ml-2 truncate">{m.data}</span>
            </button>
            <button
              onClick={() => handleDelete(m.id)}
              className="text-red-500 hover:text-red-400 text-xs opacity-0 group-hover:opacity-100 transition-opacity"
            >
              ×
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
