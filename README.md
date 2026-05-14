# SDE Kit

A **local-first SDLC desktop platform** — manage your entire software development lifecycle offline, without cloud dependencies.

Built with Rust + Tauri 2.x + SvelteKit + TailwindCSS v4.

## Features

- **Workspace Management** — Open and manage project folders with full file tree navigation
- **Code Editor** — Syntax-highlighted editing via CodeMirror with multi-language support
- **Task Board** — Kanban-style task management with drag-and-drop, priority levels, and status tracking
- **Milestone Tracking** — Track project milestones with due dates and open/close status
- **Graph Visualization** — Interactive node-edge graph for relationship mapping (pan, zoom, drag)
- **Command Palette** — Quick access to all actions via `Cmd+P`
- **SQLite Persistence** — All data stored locally, zero cloud dependencies
- **Split Editor** — Side-by-side file editing
- **Full Keyboard Navigation** — Vim-like shortcuts for panel switching and commands

## Architecture

```
apps/desktop/     → Tauri desktop application (SvelteKit frontend + Rust backend)
crates/graph/     → Graph engine (Rust workspace crate)
scripts/          → Build and release automation
```

## Prerequisites

- **Node.js** 22+
- **Rust** 1.77+
- **Tauri CLI** 2.x — `cargo install tauri-cli`

## Development

```bash
# Install frontend dependencies
npm install

# Run in development mode (hot-reload)
npm run dev

# Type-check frontend
npm run check

# Type-check everything (frontend + Rust)
npm run check:all

# Run tests
npm run test

# Build production DMG/app bundle
npm run release
```

## Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Run in development mode with hot-reload |
| `npm run build` | Build for development |
| `npm run release` | Build for production (DMG/app bundle) |
| `npm run check` | Type-check Svelte/TypeScript frontend |
| `npm run check:all` | Type-check frontend + Rust (`cargo check`) |
| `npm run test` | Run Rust unit + integration tests |
| `npm run clean` | Remove all build artifacts |

## Design System

Typography follows a simplified IDE scale defined in `plan.md`:

| Token | Size | Usage |
|-------|------|-------|
| Heading XL | 28px | Welcome screen, error pages |
| Heading L | 24px | Empty state icons |
| Heading M | 20px | Section headings |
| Heading S | 16px | Panel titles, action icons |
| Body | 14px | Body text, buttons, inputs |
| Caption | 12px | Tabs, file tree, metadata |
| Mono | 13px | Code editor, log output, shortcuts |

All font sizes are enforced via `.typo-*` utility classes for consistency across all components.

## Stack

| Layer | Technology |
|-------|-----------|
| Desktop Shell | Tauri 2.x |
| Frontend | SvelteKit 2 + Svelte 5 (runes) |
| Styling | TailwindCSS v4 (CSS-first config) |
| Editor | CodeMirror 6 |
| Backend | Rust (SQLite via rusqlite) |
| Graph | Custom force-directed layout engine (Rust) |
| Target | macOS 14+ (Apple Silicon), cross-platform compatible |
