#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
FRONTEND="$ROOT/frontend"
BACKEND="$ROOT/backend"
OUT="$BACKEND/target/release/serial-port-backend"

echo "==> Building frontend (static export)..."
cd "$FRONTEND"
npm ci --prefer-offline 2>/dev/null || npm install
NEXT_EXPORT=1 npm run build

echo "==> Building backend (release + bundled frontend)..."
cd "$BACKEND"
cargo build --release --features bundle-frontend

echo ""
echo "Done. Single binary: $OUT"
echo "Run:  $OUT"
echo "Open: http://localhost:8080"
