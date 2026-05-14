# SDE-KIT Design System

## Core Principles
- **Local-first**: All data persists to local SQLite; zero cloud dependencies
- **Offline-capable**: Full functionality without network
- **Solo-dev optimized**: Minimal UI chrome, keyboard-first navigation
- **Performance**: <100ms response for core actions on M1 8GB RAM

## Color Tokens (defined in `app.css` via `@theme`)

### Surfaces
| Token | Value | Usage |
|-------|-------|-------|
| `--color-surface-dark` | `#181715` | Main background |
| `--color-surface-dark-soft` | `#1f1e1b` | Panel/card background |
| `--color-surface-dark-elevated` | `#252320` | Hover/active surfaces |
| `--color-surface-dark-border` | `#302d2b` | Borders |

### Text
| Token | Value | Usage |
|-------|-------|-------|
| `--color-on-dark` | `#faf9f5` | Primary text |
| `--color-on-dark-soft` | `#a09d96` | Secondary text |
| `--color-muted` | `#6c6a64` | Muted text |
| `--color-muted-soft` | `#8e8b82` | Placeholder text |

### Accents
| Token | Value | Usage |
|-------|-------|-------|
| `--color-primary` | `#cc785c` | Primary actions, active states |
| `--color-primary-active` | `#a9583e` | Hover/pressed primary |
| `--color-accent-teal` | `#5db8a6` | Secondary accent |
| `--color-accent-amber` | `#e8a55a` | Warning accent |

### Semantic
| Token | Value |
|-------|-------|
| `--color-success` | `#5db872` |
| `--color-warning` | `#d4a017` |
| `--color-error` | `#c64545` |

### Light theme
Override `[data-theme="light"]` swaps surface/ink tokens while preserving brand colors.

## Typography
```
--font-display: Cormorant Garamond, serif   (headings, display)
--font-sans:    Inter, sans-serif            (UI, body)
--font-mono:    JetBrains Mono, monospace    (code, technical)
```

### Scale
| Class | Size/Weight | Usage |
|-------|-------------|-------|
| `.typo-display-xl` | 28px/400 | Hero headings |
| `.typo-display-lg` | 24px/400 | Section headings |
| `.typo-display-md` | 20px/400 | Panel headings |
| `.typo-title` | 16px/500 | Subheadings |
| `.typo-body` | 14px/400 | Body text |
| `.typo-caption` | 12px/500 | Labels, buttons |
| `.typo-small` | 11px/500 | Status, metadata |
| `.typo-mono` | 13px/400 | Code, file paths |
| `.typo-overline` | 12px/500+1.5px ls | Section overlines |

## Spacing (4px base)
`--spacing-{1,2,3,4,5,6,7,8,10,12,14,16,24}` = `{4,8,12,16,20,24,28,32,40,48,56,64,96}px`

## Component Tokens
| Token | Value |
|-------|-------|
| `--radius-xs` | 4px |
| `--radius-sm` | 6px |
| `--radius-md` | 8px |
| `--radius-lg` | 12px |
| `--radius-xl` | 16px |
| `--radius-pill` | 9999px |

## Architecture
```
┌──────────────────────────────────────────────────┐
│  CommandBar (top bar)                             │
├────────┬──────────────────────────┬───────────────┤
│Activity│  MainContent             │  (hidden)      │
│ Bar     │  (CodeMirror editor +    │               │
│ (left)  │   tabbed panels)        │               │
│        │                          │               │
│        │  ┌─────────────────────┐ │               │
│        │  │  Bottom Panel       │ │               │
│        │  │  (console/problems) │ │               │
│        │  └─────────────────────┘ │               │
└────────┴──────────────────────────┴───────────────┘
```

## UI Patterns
- **Command Palette** (`Cmd+K`): Primary navigation mechanism
- **Activity Bar**: Left sidebar for panel switching (explorer, search, projects, tasks, milestones, graph, notes, git)
- **Split Editor**: Optional side-by-side CodeMirror views
- **Error Boundary**: Wraps entire app; shows recovery UI on crash
- **Toast Notifications**: Non-blocking, auto-dismiss
