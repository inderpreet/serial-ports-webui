"use client";
import { useState } from "react";
import PortSelector from "@/components/PortSelector";
import Terminal from "@/components/Terminal";
import MacroPanel from "@/components/MacroPanel";
import CapturePanel from "@/components/CapturePanel";
import { PortStatus, DisplayMode } from "@/lib/api";

export default function Home() {
  const [portStatus, setPortStatus] = useState<PortStatus>({
    open: false,
    port_name: null,
    baud_rate: null,
    capturing: false,
  });
  const [displayMode, setDisplayMode] = useState<DisplayMode>("ascii");
  const [capturing, setCapturing] = useState(false);

  return (
    <div className="h-screen flex flex-col p-3 gap-3">
      <header className="flex items-center gap-3">
        <h1 className="text-lg font-mono font-bold text-blue-400">Web Serial Port</h1>
      </header>

      <PortSelector
        onStatusChange={setPortStatus}
        onDisplayModeChange={setDisplayMode}
        displayMode={displayMode}
      />

      <div className="flex-1 flex gap-3 min-h-0">
        <div className="flex-1 min-h-0">
          <Terminal portOpen={portStatus.open} displayMode={displayMode} />
        </div>

        <div className="w-72 flex flex-col gap-3">
          <MacroPanel portOpen={portStatus.open} />
          <CapturePanel capturing={capturing} onCapturingChange={setCapturing} />
        </div>
      </div>
    </div>
  );
}
