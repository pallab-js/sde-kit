#!/usr/bin/env bash
set -euo pipefail

echo "==> Installing frontend dependencies..."
npm ci

echo "==> Building SvelteKit frontend..."
npm run build -w apps/desktop

echo "==> Building Tauri application (dev mode)..."
cd apps/desktop/src-tauri
cargo build
cd ../..

echo "==> Build complete. Binary at: apps/desktop/src-tauri/target/debug/sde-kit"
