# Ultimate Local‑First SDLC Desktop Platform
## Complete AI Execution Blueprint

Version: 1.0
Target Stack: Rust + Tauri + TypeScript + SvelteKit + TailwindCSS
Target Hardware: Apple MacBook Air M1 (8GB RAM)
Development Methodology: AI-assisted vibecoding using terminal-based coding agents
Distribution Target: GitHub open-source distribution
Architecture Model: Offline-first, local-first, standalone desktop platform

---

# 1. Vision

Build a professional-grade, modern, extensible, high-performance desktop application that manages and visualizes the entire software development lifecycle locally without dependency on cloud infrastructure.

The platform must function as:

- IDE/workspace
- Project management system
- Task management system
- File management system
- Workspace visualization system
- Knowledge management system
- Graph relationship explorer
- Local operations dashboard
- Development orchestration environment

The application must:

- Run fully offline
- Never require internet access
- Never require user authentication
- Never depend on cloud APIs
- Never require Docker
- Never integrate AI/ML features
- Never introduce telemetry or tracking
- Remain lightweight and deterministic
- Be suitable for long-term maintainability
- Be production-grade and enterprise-quality

---

# 2. Core Product Principles

## 2.1 Non-Negotiable Constraints

The application SHALL:

- Use Rust for backend/native logic
- Use Tauri for desktop runtime
- Use SvelteKit for frontend
- Use TailwindCSS for styling
- Use TypeScript strictly
- Use SQLite for persistence
- Support macOS first
- Remain cross-platform compatible
- Be keyboard-first
- Be modular
- Be local-first
- Be responsive and performant
- Be deterministic and testable

The application SHALL NOT:

- Use Electron
- Use React
- Use Vue
- Use cloud services
- Use remote authentication
- Use analytics
- Use Docker runtime dependency
- Use AI/ML integration
- Use external APIs for core features
- Use hidden background services
- Use microservices
- Use unnecessary dependencies

---

# 3. High-Level Architecture

## 3.1 Layered Architecture

```txt
Frontend Layer
  ├── SvelteKit UI
  ├── TailwindCSS Design System
  ├── State Layer
  ├── Workspace Layer
  └── IPC Client Layer

IPC Boundary
  └── Typed Tauri Commands

Backend Layer
  ├── Core Engine
  ├── Workspace Engine
  ├── Project Engine
  ├── Graph Engine
  ├── File Engine
  ├── Task Engine
  ├── Layout Engine
  ├── Search Engine
  ├── Persistence Engine
  └── Event Bus

Persistence Layer
  ├── SQLite
  ├── JSON configs
  ├── File index cache
  └── Workspace metadata
```

---

# 4. Repository Structure

```txt
/root
├── apps/
│   └── desktop/
│       ├── src/
│       ├── src-tauri/
│       ├── static/
│       └── tests/
│
├── crates/
│   ├── core/
│   ├── workspace/
│   ├── graph/
│   ├── tasks/
│   ├── indexing/
│   ├── persistence/
│   ├── search/
│   ├── layout/
│   ├── filesystem/
│   ├── events/
│   └── shared/
│
├── packages/
│   ├── ui/
│   ├── types/
│   ├── stores/
│   ├── commands/
│   ├── utils/
│   └── themes/
│
├── docs/
│   ├── architecture/
│   ├── decisions/
│   ├── workflows/
│   └── standards/
│
├── scripts/
├── tests/
├── tools/
├── .github/
└── README.md
```

---

# 5. Technology Decisions

## 5.1 Frontend

### SvelteKit
Purpose:
- Reactive UI
- Modular component architecture
- Lightweight rendering
- Desktop-friendly performance

Rules:
- Use Svelte stores sparingly
- Avoid giant components
- Prefer composition over inheritance
- Avoid deeply nested reactive chains

### TailwindCSS
Purpose:
- Design consistency
- Token-driven styling
- Utility-first architecture

Rules:
- Never use inline styles
- Never use arbitrary values unless necessary
- Use semantic component wrappers
- Maintain spacing scale consistency

---

## 5.2 Backend

### Rust
Purpose:
- Native performance
- Strong typing
- Deterministic execution
- Safe concurrency

Rules:
- Avoid unsafe blocks unless absolutely required
- Prefer explicit ownership
- Avoid over-abstraction
- Prefer composition
- Prefer immutable structures

---

# 6. Development Methodology

## 6.1 Vibecoding Governance

AI coding agents MUST:

- Implement only requested scope
- Never introduce unrequested features
- Never refactor unrelated modules
- Never introduce speculative abstractions
- Never create hidden dependencies
- Never duplicate business logic
- Never bypass architecture rules
- Never add dependencies without justification

Every generated code change MUST:

- Compile successfully
- Pass linting
- Pass tests
- Preserve architecture boundaries
- Maintain strict typing
- Include documentation

---

# 7. AI Operational Rules

## 7.1 Hard AI Constraints

AI SHALL NOT:

- Add cloud features
- Add authentication
- Add telemetry
- Add analytics
- Add AI integrations
- Add websocket dependencies
- Add remote sync
- Add runtime package downloading
- Add Docker dependencies
- Add hidden state systems
- Add complex plugin runtimes prematurely

AI SHALL:

- Prefer simple deterministic solutions
- Prefer maintainable code
- Prefer strongly typed interfaces
- Prefer modular architecture
- Prefer local persistence
- Prefer low-memory approaches

---

# 8. UI/UX System

## 8.1 Design Philosophy

Design principles:

- Minimal
- Professional
- Functional
- Dense but readable
- Keyboard-first
- Dockable workspace-oriented
- Low visual noise
- Enterprise-grade clarity

Visual characteristics:

- Muted dark theme by default
- Subtle contrast
- Soft shadows
- Thin borders
- Consistent spacing
- Smooth transitions
- Fast interaction feedback

---

## 8.2 Layout System

Primary regions:

```txt
┌───────────────────────────────┐
│ Top Command Bar              │
├───────┬───────────────────────┤
│ Left  │ Main Workspace        │
│ Panel │                       │
├───────┼───────────────────────┤
│ Bottom Console / Inspector    │
└───────────────────────────────┘
```

Panels:

- Explorer
- Projects
- Tasks
- Graphs
- Search
- Activity
- Layouts
- Notes
- Inspector

Main workspace:

- Tab-based
- Multi-pane split support
- Drag-and-drop docking
- Persistent layouts

---

# 9. Design Tokens

## 9.1 Spacing Scale

```txt
1 = 4px
2 = 8px
3 = 12px
4 = 16px
5 = 20px
6 = 24px
8 = 32px
10 = 40px
12 = 48px
```

---

## 9.2 Border Radius

```txt
sm = 4px
md = 8px
lg = 12px
xl = 16px
2xl = 24px
```

---

## 9.3 Typography

```txt
Heading XL = 28
Heading L = 24
Heading M = 20
Heading S = 16
Body = 14
Caption = 12
Mono = 13
```

---

# 10. State Management Architecture

## 10.1 Rules

State ownership must be explicit.

Hierarchy:

```txt
Server State
  ↓
Workspace State
  ↓
Feature State
  ↓
UI State
```

Rules:

- Avoid global mutable state
- Use derived stores carefully
- Separate persistence state from UI state
- Never create circular store dependencies
- Keep stores small and focused

---

# 11. Persistence Strategy

## 11.1 Storage Layers

### SQLite
Used for:

- Projects
- Tasks
- Relationships
- Workspace metadata
- Layouts
- Event history

### File System
Used for:

- Notes
- User assets
- Attachments
- Config snapshots
- Exported reports

### JSON
Used for:

- User preferences
- Themes
- Temporary cache
- Layout presets

---

# 12. Data Model Definitions

## 12.1 Project

```ts
interface Project {
  id: string;
  name: string;
  path: string;
  description?: string;
  createdAt: string;
  updatedAt: string;
  tags: string[];
}
```

---

## 12.2 Task

```ts
interface Task {
  id: string;
  title: string;
  description?: string;
  status: 'todo' | 'doing' | 'done';
  priority: 'low' | 'medium' | 'high';
  createdAt: string;
  updatedAt: string;
  projectId?: string;
}
```

---

## 12.3 Graph Node

```ts
interface GraphNode {
  id: string;
  type: string;
  label: string;
  metadata?: Record<string, unknown>;
}
```

---

# 13. Feature Modules

## 13.1 Workspace Engine

Responsibilities:

- Workspace lifecycle
- Layout restoration
- Tab persistence
- Context restoration
- Window state management

---

## 13.2 File Manager

Responsibilities:

- File browsing
- File indexing
- File watching
- Workspace navigation
- Safe filesystem operations

Constraints:

- Lazy-load directories
- Avoid recursive memory loading
- Limit watcher count

---

## 13.3 Code Editor

Responsibilities:

- Syntax highlighting
- Tabs
- Split views
- Search
- File editing

Constraints:

- Use lightweight editor engine
- Avoid giant editor abstractions
- Support large files efficiently

---

## 13.4 Graph Engine

Responsibilities:

- Relationship visualization
- Dependency mapping
- Workspace topology
- Interactive graph navigation

Constraints:

- Virtualize rendering
- Limit graph memory growth
- Progressive rendering

---

## 13.5 Task System

Responsibilities:

- Task tracking
- Kanban visualization
- Milestone tracking
- Project linkage
- Timeline support

---

# 14. Command System

## 14.1 Command Registry

All user actions must be command-based.

Example:

```txt
workspace.open
workspace.close
project.create
project.delete
file.open
file.rename
task.create
graph.center
layout.restore
```

Benefits:

- Keyboard accessibility
- Undo/redo support
- Automation compatibility
- Internal consistency

---

# 15. Event System

## 15.1 Rules

Use lightweight event-driven communication.

Rules:

- Avoid event explosion
- Use typed payloads
- Keep events domain-specific
- Prevent cascading side effects

---

# 16. Performance Budgets

## 16.1 Targets

Cold startup:
- Under 3 seconds

Warm startup:
- Under 1 second

Idle RAM usage:
- Under 350MB

Large workspace indexing:
- Non-blocking

Frame rendering:
- Maintain 60 FPS interaction

Graph rendering:
- Progressive virtualization

---

# 17. Security Model

## 17.1 Tauri Security Rules

STRICT RULES:

- Never expose unrestricted shell access
- Never expose unrestricted filesystem access
- Use allowlists
- Validate all IPC inputs
- Use typed serialization
- Avoid unsafe Rust
- Sanitize file paths
- Prevent path traversal

---

# 18. Dependency Governance

## 18.1 Dependency Rules

Every dependency must:

- Solve a real problem
- Be actively maintained
- Be lightweight
- Have acceptable license compatibility
- Avoid hidden transitive complexity

Avoid:

- Massive UI frameworks
- Experimental runtimes
- Abandoned crates
- Duplicate utility libraries

---

# 19. Testing Standards

## 19.1 Required Coverage

Every feature requires:

- Unit tests
- Integration tests
- IPC tests
- Store tests
- Serialization tests

Critical flows require:

- E2E testing
- Workspace restoration tests
- File safety tests
- Persistence tests

---

# 20. Coding Standards

## 20.1 TypeScript Rules

- Strict mode enabled
- No any types
- Explicit return types
- Small focused modules
- No giant files
- Avoid deep inheritance

---

## 20.2 Rust Rules

- Clippy clean
- Rustfmt enforced
- Prefer Result-based error handling
- Avoid panic usage
- Use explicit lifetimes when needed
- Avoid hidden mutation

---

# 21. Error Handling

## 21.1 Rules

Errors must:

- Be typed
- Be actionable
- Be logged locally
- Preserve application stability
- Never crash silently

---

# 22. Logging Strategy

## 22.1 Logging Rules

Local-only logging.

Never:

- Upload logs
- Send telemetry
- Transmit usage analytics

Log categories:

- App lifecycle
- Errors
- Performance
- Workspace events
- File operations

---

# 23. Accessibility Standards

## 23.1 Requirements

Must support:

- Keyboard navigation
- High contrast themes
- Focus visibility
- Screen-reader semantics
- Reduced motion preference

---

# 24. Keyboard System

## 24.1 Philosophy

The entire application must be operable without mouse dependency.

Core shortcuts:

```txt
Cmd+P = Command palette
Cmd+B = Toggle sidebar
Cmd+Shift+F = Global search
Cmd+T = New tab
Cmd+W = Close tab
Cmd+K = Actions
```

---

# 25. Workspace Persistence

## 25.1 Persisted Data

Persist:

- Open tabs
- Window layout
- Panel visibility
- Workspace state
- Recent projects
- Session history

Do not persist:

- Temporary UI glitches
- Unstable transient state

---

# 26. Roadmap

## Phase 1 — Core Infrastructure

Goals:

- Tauri shell
- SvelteKit setup
- Tailwind integration
- SQLite integration
- IPC foundation
- Workspace shell

Deliverables:

- App boots successfully
- Core layout operational
- Persistence operational

---

## Phase 2 — Workspace System

Goals:

- Docking system
- Tabs
- Panels
- Layout persistence
- Command system

---

## Phase 3 — File System Layer

Goals:

- File explorer
- File indexing
- File watching
- Safe file operations

---

## Phase 4 — Code Editing

Goals:

- Editor integration
- Search
- Syntax support
- Split panes

---

## Phase 5 — Project Management

Goals:

- Project entities
- Task system
- Kanban boards
- Milestones

---

## Phase 6 — Graph Visualization

Goals:

- Node rendering
- Relationship mapping
- Workspace graph
- Navigation engine

---

## Phase 7 — Optimization

Goals:

- Performance tuning
- Memory optimization
- Startup optimization
- Virtualization

---

## Phase 8 — Production Hardening

Goals:

- Testing completion
- Accessibility completion
- Security review
- Packaging
- Documentation

---

# 27. Definition of Done

A feature is complete ONLY IF:

- Typed
- Tested
- Linted
- Documented
- Keyboard accessible
- Responsive
- Stable
- No TODO comments
- No mock logic
- No console warnings
- No dead code
- No architectural violations

---

# 28. Release Engineering

## 28.1 Build Requirements

Release builds must:

- Pass CI
- Pass linting
- Pass tests
- Produce reproducible builds
- Use version tagging
- Generate changelogs

---

# 29. Git Strategy

## 29.1 Branch Rules

Main branches:

```txt
main
develop
feature/*
fix/*
release/*
```

Commit format:

```txt
feat:
fix:
refactor:
perf:
docs:
test:
```

---

# 30. CI/CD Strategy

## 30.1 CI Pipeline

Required checks:

- Type checking
- Rust compilation
- Unit tests
- Integration tests
- Linting
- Formatting
- Build validation

---

# 31. Anti-Patterns

NEVER:

- Create god components
- Create giant stores
- Create hidden coupling
- Use premature abstractions
- Use massive dependencies
- Add speculative systems
- Mix UI and business logic
- Bypass typed contracts

---

# 32. Plugin Architecture Decision

Initial release SHALL NOT include third-party plugin runtime.

However:

Internal architecture must remain extensible.

This means:

- Commands are modular
- Events are typed
- Features are isolated
- Registries are centralized

---

# 33. Memory Optimization Rules

For M1 8GB target:

- Avoid loading large trees eagerly
- Virtualize lists
- Virtualize graph rendering
- Dispose watchers aggressively
- Cache carefully
- Avoid duplicate data copies

---

# 34. AI Task Execution Protocol

Every AI coding session must:

1. Read architecture rules
2. Read current feature scope
3. Implement only requested task
4. Run tests
5. Run linting
6. Validate boundaries
7. Produce concise summary

AI must NEVER:

- Change unrelated modules
- Reorganize architecture arbitrarily
- Rename systems unnecessarily
- Add hidden dependencies

---

# 35. Recommended Development Order

Strict implementation order:

1. Core shell
2. Persistence
3. Layout engine
4. Command system
5. File engine
6. Workspace management
7. Editor integration
8. Task system
9. Graph engine
10. Optimization
11. Hardening
12. Packaging

Do NOT skip order.

---

# 36. Documentation Standards

Every module requires:

- README
- Public API docs
- Architecture notes
- Usage examples
- Constraints section

---

# 37. Final Engineering Philosophy

This platform is NOT:

- A web app wrapped as desktop
- A cloud dashboard
- An AI assistant
- A startup MVP
- A rapid prototype

This platform IS:

- A local-first engineering environment
- A deterministic desktop system
- A modular professional tool
- A long-term maintainable platform
- A performance-conscious workspace

Primary priorities:

1. Stability
2. Simplicity
3. Maintainability
4. Determinism
5. Performance
6. User control
7. Offline reliability

---

# 38. Final Execution Instructions For AI Agents

When implementing this system:

- Follow architecture exactly
- Respect module boundaries
- Avoid speculative engineering
- Prefer boring reliable solutions
- Keep code explicit
- Keep systems composable
- Keep memory usage controlled
- Keep interfaces typed
- Keep behavior deterministic

The success metric is NOT feature count.

The success metric is:

- reliability,
- maintainability,
- performance,
- architectural clarity,
- and long-term scalability.

