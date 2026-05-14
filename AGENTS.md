# AGENT INSTRUCTIONS: SDE-KIT Development

## STRICT PROHIBITIONS (Never Suggest/Implement)
- Cloud services (AWS, GCP, Azure, Firebase, etc.)
- Authentication systems (OAuth, JWT, sessions, etc.)
- Docker, Kubernetes, containerization
- AI/ML features (LLM integration, code completion APIs, etc.)
- Real-time sync, WebSockets, P2P networking
- Telemetry, analytics, crash reporting services
- Third-party API integrations requiring network

## REQUIRED PRINCIPLES
- **Local-first**: All data persists to local SQLite; zero cloud dependencies
- **Offline-capable**: Full functionality without network
- **Standalone**: Single executable; no external services
- **Privacy**: No data leaves the device
- **Solo-dev focus**: Minimize configuration; maximize defaults

## IMPLEMENTATION GUARDRAILS
1. When adding features, ask: "Does this work with airplane mode ON?"
2. Prefer SQLite transactions over complex state management
3. Use Tauri's custom Rust commands for fs ops; never suggest Node.js `fs`
4. For code editing: CodeMirror 6 extensions only; no LSP servers requiring network
5. Graph visualization: Canvas/SVG only; avoid WebGL libraries with heavy dependencies

## ERROR HANDLING PROTOCOL
- All Rust commands return `Result<T, String>` with user-friendly messages
- Frontend displays errors in non-blocking toast; never crash the app

## TESTING REQUIREMENTS
- Unit tests: `cargo test` for Rust; `npm run test` for Svelte
- Integration: test offline mode by disabling network during e2e
- Performance: core actions <100ms on M1 8GB RAM
