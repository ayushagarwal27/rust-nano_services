#!/usr/bin/env bash
set -e

# Resolve paths safely
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
INGRESS_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Ingress dir: $INGRESS_DIR"

# ---------- Frontend ----------
if [ -d "$INGRESS_DIR/frontend" ]; then
  echo "Starting frontend..."
  cd "$INGRESS_DIR/frontend"
  npm install
  npm run dev &
else
  echo "No frontend directory found, skipping frontend"
fi

# ---------- Backend ----------
echo "Starting backend..."
cd "$INGRESS_DIR"
cargo run --bin ingress