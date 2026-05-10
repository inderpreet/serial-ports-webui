import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Web Serial Port",
  description: "Portable WebUI for serial ports",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
