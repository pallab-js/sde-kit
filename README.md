# SDE Kit

A local-first SDLC desktop platform built with Rust + Tauri + SvelteKit + TailwindCSS.

## Architecture

```
apps/desktop/     → Tauri desktop application (SvelteKit frontend + Rust backend)
crates/           → Rust workspace crates
packages/         → TypeScript packages
docs/             → Documentation
scripts/          → Build and dev scripts
```

## Development

```bash
# Install frontend dependencies
npm install

# Run in development mode
npm run dev

# Build for production
npm run build
```

## Prerequisites

- Node.js 22+
- Rust 1.77+
- Tauri CLI 2.x (`brew install tauri` or `cargo install tauri-cli`)

## Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Run in development mode |
| `npm run build` | Build for development |
| `npm run release` | Build production DMG/app bundle |
| `npm run check` | Type-check frontend |
| `npm run check:all` | Type-check frontend + Rust |
| `npm run test` | Run Rust tests |
| `npm run clean` | Remove build artifacts |
