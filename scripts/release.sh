#!/usr/bin/env bash
set -euo pipefail

echo "==> Installing frontend dependencies..."
npm ci

echo "==> Building production frontend..."
npm run build -w apps/desktop

echo "==> Bundling Tauri application for distribution..."
cd apps/desktop/src-tauri
npx -y @tauri-apps/cli@latest build
cd ../..

echo "==> Release build complete."
echo "    DMG/App bundles available in: apps/desktop/src-tauri/target/release/bundle/"
