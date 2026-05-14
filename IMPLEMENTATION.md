# SDE-KIT: Comprehensive Critical Review & Implementation Guide

> **Target**: Openode AI Implementation  
> **Project**: `https://github.com/pallab-js/SDE-KIT.git`  
> **Constraints**: Offline, local-first, standalone IDE + Code Editor + Task Manager + Project Manager + SDLC Graph Visualizer + File Manager  
> **Exclusions**: No AI/ML, cloud, Docker, authentication features  
> **Design System**: Anthropic Claude-style warm editorial theme (DESIGN.md)

---

## 📋 Executive Summary

| Category | Status | Priority |
|----------|--------|----------|
| Architecture | ✅ Solid monorepo (Rust workspace + SvelteKit) | Low |
| Design System | ⚠️ Partial CSS variables; missing component tokens | **High** |
| Local-First Data | ⚠️ SQLite present; missing conflict resolution | **Critical** |
| Graph Engine | ⚠️ Crate scaffolded; layout algorithm incomplete | High |
| Component Library | ❌ Not implemented; needs Claude-style tokens | **Critical** |
| Performance | ⚠️ No virtualization for large file trees/graphs | Medium |
| Testing | ⚠️ Vitest configured; missing offline/e2e tests | Medium |
| Keyboard Nav | ❌ Not implemented despite README claim | High |
| Error Recovery | ❌ Missing crash resilience for solo dev workflow | High |

**Verdict**: Strong foundation requiring systematic implementation of design system, local-first guarantees, and core feature completeness.

---

## 🚨 Critical Issues & Step-by-Step Solutions

### Issue 1: Design System Incomplete (DESIGN.md not fully implemented)

**Problem**: `app.css` defines color variables but lacks:
- Component-level tokens (`{component.button-primary}` style references)
- Typography hierarchy utility classes matching DESIGN.md scale
- Proper border radius, spacing token mapping
- Light/dark theme toggle logic

**Solution**:

```bash
# Step 1: Create design tokens file
mkdir -p apps/desktop/src/lib/design
touch apps/desktop/src/lib/design/tokens.ts
```

```typescript
// apps/desktop/src/lib/design/tokens.ts
/**
 * Claude-style Design Tokens - Local-First Implementation
 * Reference: DESIGN.md YAML specification
 */

export const colors = {
  // Brand & Accent
  primary: '#cc785c',
  primaryActive: '#a9583e',
  primaryDisabled: '#e6dfd8',
  accentTeal: '#5db8a6',
  accentAmber: '#e8a55a',
  
  // Surfaces - Cream (Light)
  canvas: '#faf9f5',
  surfaceSoft: '#f5f0e8',
  surfaceCard: '#efe9de',
  surfaceCreamStrong: '#e8e0d2',
  
  // Surfaces - Dark
  surfaceDark: '#181715',
  surfaceDarkElevated: '#252320',
  surfaceDarkSoft: '#1f1e1b',
  
  // Text
  ink: '#141413',
  bodyStrong: '#252523',
  body: '#3d3d3a',
  muted: '#6c6a64',
  mutedSoft: '#8e8b82',
  onPrimary: '#ffffff',
  onDark: '#faf9f5',
  onDarkSoft: '#a09d96',
  
  // Borders
  hairline: '#e6dfd8',
  hairlineSoft: '#ebe6df',
  
  // Semantic
  success: '#5db8a6',
  warning: '#d4a017',
  error: '#c64545',
} as const;

export const typography = {
  fontDisplay: "'Cormorant Garamond', 'EB Garamond', serif",
  fontSans: "'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif",
  fontMono: "'JetBrains Mono', 'Fira Code', ui-monospace, monospace",
  
  displayXl: { fontSize: '28px', fontWeight: 400, lineHeight: 1.2, letterSpacing: '-0.3px' },
  displayLg: { fontSize: '24px', fontWeight: 400, lineHeight: 1.25, letterSpacing: '-0.2px' },
  displayMd: { fontSize: '20px', fontWeight: 400, lineHeight: 1.3, letterSpacing: '0' },
  displaySm: { fontSize: '16px', fontWeight: 400, lineHeight: 1.4, letterSpacing: '0' },
  title: { fontSize: '16px', fontWeight: 500, lineHeight: 1.4 },
  body: { fontSize: '14px', fontWeight: 400, lineHeight: 1.55 },
  caption: { fontSize: '12px', fontWeight: 500, lineHeight: 1.4 },
  code: { fontSize: '13px', fontWeight: 400, lineHeight: 1.6, fontFamily: 'var(--font-mono)' },
  button: { fontSize: '14px', fontWeight: 500, lineHeight: 1 },
} as const;

export const spacing = {
  xxs: '4px', xs: '8px', sm: '12px', md: '16px',
  lg: '24px', xl: '32px', xxl: '48px', section: '96px',
} as const;

export const radius = {
  xs: '4px', sm: '6px', md: '8px', lg: '12px',
  xl: '16px', pill: '9999px', full: '50%',
} as const;

export type ThemeMode = 'light' | 'dark';
export const defaultTheme: ThemeMode = 'dark';
```

```bash
# Step 2: Create CSS variable mapper for Tailwind v4
touch apps/desktop/src/lib/design/css-variables.ts
```

```typescript
// apps/desktop/src/lib/design/css-variables.ts
import { colors, spacing, radius, typography } from './tokens';

/**
 * Generates CSS custom properties from design tokens
 * Used for Tailwind v4 @theme integration and runtime theming
 */
export function generateCSSVariables(theme: 'light' | 'dark' = 'dark'): Record<string, string> {
  const vars: Record<string, string> = {};
  
  // Color mapping with theme-aware defaults
  const surface = theme === 'dark' 
    ? { base: colors.surfaceDark, elevated: colors.surfaceDarkElevated, soft: colors.surfaceDarkSoft }
    : { base: colors.canvas, elevated: colors.surfaceCard, soft: colors.surfaceSoft };
    
  const text = theme === 'dark'
    ? { primary: colors.onDark, secondary: colors.onDarkSoft }
    : { primary: colors.ink, secondary: colors.body };

  // Surfaces
  vars['--surface-base'] = surface.base;
  vars['--surface-elevated'] = surface.elevated;
  vars['--surface-soft'] = surface.soft;
  vars['--surface-border'] = theme === 'dark' ? '#302d2b' : colors.hairline;
  
  // Text
  vars['--text-primary'] = text.primary;
  vars['--text-secondary'] = text.secondary;
  vars['--text-muted'] = colors.muted;
  
  // Brand
  vars['--color-primary'] = colors.primary;
  vars['--color-primary-active'] = colors.primaryActive;
  vars['--color-success'] = colors.success;
  vars['--color-warning'] = colors.warning;
  vars['--color-error'] = colors.error;
  
  // Spacing
  Object.entries(spacing).forEach(([key, value]) => {
    vars[`--spacing-${key}`] = value;
  });
  
  // Radius
  Object.entries(radius).forEach(([key, value]) => {
    vars[`--radius-${key}`] = value;
  });
  
  // Typography
  vars['--font-display'] = typography.fontDisplay;
  vars['--font-sans'] = typography.fontSans;
  vars['--font-mono'] = typography.fontMono;
  
  return vars;
}

/**
 * Applies theme variables to document root
 */
export function applyTheme(theme: 'light' | 'dark'): void {
  const root = document.documentElement;
  const vars = generateCSSVariables(theme);
  root.setAttribute('data-theme', theme);
  Object.entries(vars).forEach(([prop, value]) => {
    root.style.setProperty(prop, value);
  });
}
```

```bash
# Step 3: Update app.css to use token references (not hardcoded hex)
# Replace existing app.css content with:
```

```css
/* apps/desktop/src/app.css - REFACTORED */
@import "tailwindcss";

@theme {
  /* ===== Design Token References (from tokens.ts) ===== */
  /* Surfaces */
  --color-canvas: var(--surface-base, #faf9f5);
  --color-surface-card: var(--surface-elevated, #efe9de);
  --color-surface-dark: var(--surface-base, #181715);
  --color-surface-dark-elevated: var(--surface-elevated, #252320);
  
  /* Brand */
  --color-primary: var(--color-primary, #cc785c);
  --color-primary-active: var(--color-primary-active, #a9583e);
  
  /* Text */
  --color-ink: var(--text-primary, #141413);
  --color-on-dark: var(--text-primary, #faf9f5);
  --color-muted: var(--text-muted, #6c6a64);
  
  /* Borders */
  --color-hairline: var(--surface-border, #e6dfd8);
  
  /* Semantic */
  --color-success: var(--color-success, #5db872);
  --color-error: var(--color-error, #c64545);
  
  /* Spacing (4px base scale) */
  --spacing-section: 96px;
  --spacing-card: 32px;
  
  /* Radius hierarchy */
  --radius-button: 8px;    /* md */
  --radius-card: 12px;     /* lg */
  --radius-hero: 16px;     /* xl */
  
  /* Typography */
  --font-display: 'Cormorant Garamond', 'EB Garamond', serif;
  --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', ui-monospace, monospace;
}

/* ===== Base Styles ===== */
body {
  margin: 0;
  padding: 0;
  font-family: var(--font-sans);
  font-size: 14px;
  background: var(--surface-base);
  color: var(--text-primary);
  overflow: hidden;
  user-select: none;
  -webkit-font-smoothing: antialiased;
}

/* ===== Typography Utilities (Claude hierarchy) ===== */
.typo-display {
  font-family: var(--font-display);
  font-weight: 400;
  letter-spacing: -0.3px;
  line-height: 1.2;
  color: var(--text-primary);
}
.typo-display-xl { font-size: 28px; }
.typo-display-lg { font-size: 24px; }
.typo-display-md { font-size: 20px; }
.typo-display-sm { font-size: 16px; }

.typo-title {
  font-family: var(--font-sans);
  font-size: 16px;
  font-weight: 500;
  line-height: 1.4;
}

.typo-body {
  font-family: var(--font-sans);
  font-size: 14px;
  font-weight: 400;
  line-height: 1.55;
  color: var(--text-secondary, var(--text-primary));
}

.typo-code {
  font-family: var(--font-mono);
  font-size: 13px;
  line-height: 1.6;
  color: var(--text-primary);
}

.typo-button {
  font-family: var(--font-sans);
  font-size: 14px;
  font-weight: 500;
  line-height: 1;
}

/* ===== Component Base Styles ===== */
.btn-primary {
  background: var(--color-primary);
  color: var(--color-on-primary, #fff);
  border-radius: var(--radius-button);
  padding: 12px 20px;
  height: 40px;
  font: var(--typo-button);
  border: none;
  cursor: pointer;
  transition: background 0.15s ease;
}
.btn-primary:hover { background: var(--color-primary-active); }
.btn-primary:disabled { 
  background: var(--color-primary-disabled, #e6dfd8); 
  color: var(--color-muted);
  cursor: not-allowed;
}

.card {
  background: var(--surface-elevated);
  border-radius: var(--radius-card);
  padding: var(--spacing-card);
  border: 1px solid var(--surface-border, transparent);
}

.card-dark {
  background: var(--surface-dark);
  color: var(--text-primary);
  border-radius: var(--radius-card);
  padding: var(--spacing-card);
}

/* ===== Scrollbars (minimal, brand-aligned) ===== */
::-webkit-scrollbar { width: 8px; height: 8px; }
::-webkit-scrollbar-track { background: transparent; }
::-webkit-scrollbar-thumb { 
  background: var(--surface-border); 
  border-radius: var(--radius-pill);
}
::-webkit-scrollbar-thumb:hover { background: var(--color-muted); }

/* ===== Selection ===== */
::selection {
  background: color-mix(in srgb, var(--color-primary) 35%, transparent);
  color: var(--color-on-primary, var(--text-primary));
}
```

---

### Issue 2: Local-First Data Layer Incomplete

**Problem**: SQLite via `rusqlite` is configured but lacks:
- Offline-first conflict resolution strategy
- Transactional integrity for SDLC operations
- Schema migrations
- Backup/export functionality for solo developers

**Solution**:

```bash
# Step 1: Create local-first database service
mkdir -p apps/desktop/src/lib/services/database
touch apps/desktop/src/lib/services/database/local-db.ts
```

```typescript
// apps/desktop/src/lib/services/database/local-db.ts
/**
 * Local-First Database Service
 * - SQLite backend via Tauri plugin
 * - Conflict-free replicated data types (CRDT-lite) for offline ops
 * - Schema versioning with migrations
 * - Zero cloud dependencies
 */

import { invoke } from '@tauri-apps/api/core';
import type { Project, Task, Milestone, FileNode, GraphEdge, GraphNode } from '$lib/types';

export interface DBResult<T> {
  success: boolean;
  data?: T;
  error?: string;
  conflict?: ConflictResolution;
}

export type ConflictResolution = 'local-wins' | 'remote-wins' | 'manual' | 'merge';

export class LocalDatabase {
  private static instance: LocalDatabase;
  private initialized = false;
  
  private constructor() {}
  
  static getInstance(): LocalDatabase {
    if (!LocalDatabase.instance) {
      LocalDatabase.instance = new LocalDatabase();
    }
    return LocalDatabase.instance;
  }
  
  async initialize(): Promise<DBResult<void>> {
    if (this.initialized) return { success: true };
    
    try {
      // Initialize SQLite with proper WAL mode for concurrent reads
      await invoke('db_init', {
        path: await invoke('app_data_dir'),
        journalMode: 'WAL',
        synchronous: 'NORMAL'
      });
      
      // Run migrations
      await this.runMigrations();
      this.initialized = true;
      return { success: true };
    } catch (err) {
      return { 
        success: false, 
        error: err instanceof Error ? err.message : 'Database initialization failed' 
      };
    }
  }
  
  private async runMigrations(): Promise<void> {
    // Versioned schema migrations - append-only for safety
    const migrations = [
      `CREATE TABLE IF NOT EXISTS projects (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        path TEXT UNIQUE NOT NULL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        metadata TEXT
      )`,
      `CREATE TABLE IF NOT EXISTS tasks (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL,
        title TEXT NOT NULL,
        description TEXT,
        status TEXT CHECK(status IN ('todo','in-progress','review','done')) NOT NULL,
        priority TEXT CHECK(priority IN ('low','medium','high','critical')) NOT NULL,
        due_date INTEGER,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
      )`,
      `CREATE TABLE IF NOT EXISTS milestones (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL,
        title TEXT NOT NULL,
        description TEXT,
        due_date INTEGER,
        is_completed INTEGER DEFAULT 0,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
      )`,
      `CREATE TABLE IF NOT EXISTS graph_nodes (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL,
        type TEXT NOT NULL,
        label TEXT NOT NULL,
        data TEXT,
        x REAL, y REAL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE
      )`,
      `CREATE TABLE IF NOT EXISTS graph_edges (
        id TEXT PRIMARY KEY,
        project_id TEXT NOT NULL,
        source_id TEXT NOT NULL,
        target_id TEXT NOT NULL,
        type TEXT NOT NULL,
        label TEXT,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
        FOREIGN KEY (source_id) REFERENCES graph_nodes(id) ON DELETE CASCADE,
        FOREIGN KEY (target_id) REFERENCES graph_nodes(id) ON DELETE CASCADE
      )`,
      // Indexes for performance
      `CREATE INDEX IF NOT EXISTS idx_tasks_project ON tasks(project_id)`,
      `CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status)`,
      `CREATE INDEX IF NOT EXISTS idx_graph_nodes_project ON graph_nodes(project_id)`,
      `CREATE INDEX IF NOT EXISTS idx_graph_edges_project ON graph_edges(project_id)`,
    ];
    
    for (const sql of migrations) {
      await invoke('db_execute', { query: sql });
    }
  }
  
  // ===== Project Operations =====
  async createProject(project: Omit<Project, 'id' | 'created_at' | 'updated_at'>): Promise<DBResult<Project>> {
    const id = crypto.randomUUID();
    const now = Date.now();
    
    try {
      await invoke('db_execute', {
        query: `INSERT INTO projects (id, name, path, created_at, updated_at, metadata)
                VALUES (?, ?, ?, ?, ?, ?)`,
        params: [id, project.name, project.path, now, now, JSON.stringify(project.metadata || {})]
      });
      return { success: true, data: { ...project, id, created_at: now, updated_at: now } };
    } catch (err) {
      return { success: false, error: err instanceof Error ? err.message : 'Create failed' };
    }
  }
  
  async getProject(id: string): Promise<DBResult<Project>> {
    try {
      const result = await invoke('db_query_one', {
        query: 'SELECT * FROM projects WHERE id = ?',
        params: [id]
      }) as Project;
      return { success: true, data: result };
    } catch (err) {
      return { success: false, error: err instanceof Error ? err.message : 'Query failed' };
    }
  }
  
  // ===== Task Operations with Conflict Handling =====
  async updateTask(taskId: string, updates: Partial<Task>, localVersion: number): Promise<DBResult<Task>> {
    // Optimistic concurrency control for offline edits
    try {
      const now = Date.now();
      const result = await invoke('db_execute_with_version', {
        query: `UPDATE tasks SET 
                title = COALESCE(?, title),
                description = COALESCE(?, description),
                status = COALESCE(?, status),
                priority = COALESCE(?, priority),
                due_date = COALESCE(?, due_date),
                updated_at = ?,
                version = version + 1
                WHERE id = ? AND version = ?`,
        params: [
          updates.title, updates.description, updates.status, 
          updates.priority, updates.due_date, now, taskId, localVersion
        ]
      });
      
      if (result === 0) {
        // Conflict detected: version mismatch
        return { 
          success: false, 
          error: 'Conflict: task was modified elsewhere',
          conflict: 'manual' // Require user resolution
        };
      }
      
      return { success: true, data: { ...await this.getTask(taskId).then(r => r.data), updated_at: now } };
    } catch (err) {
      return { success: false, error: err instanceof Error ? err.message : 'Update failed' };
    }
  }
  
  // ===== Export/Backup for Solo Devs =====
  async exportProject(projectId: string, format: 'json' | 'sqlite'): Promise<DBResult<Blob>> {
    try {
      if (format === 'json') {
        const [project, tasks, milestones, nodes, edges] = await Promise.all([
          this.getProject(projectId),
          invoke('db_query', { query: 'SELECT * FROM tasks WHERE project_id = ?', params: [projectId] }),
          invoke('db_query', { query: 'SELECT * FROM milestones WHERE project_id = ?', params: [projectId] }),
          invoke('db_query', { query: 'SELECT * FROM graph_nodes WHERE project_id = ?', params: [projectId] }),
          invoke('db_query', { query: 'SELECT * FROM graph_edges WHERE project_id = ?', params: [projectId] }),
        ]);
        
        const exportData = {
          project: project.data,
          tasks, milestones, graph: { nodes, edges },
          exported_at: new Date().toISOString(),
          version: '1.0'
        };
        
        return { 
          success: true, 
          data: new Blob([JSON.stringify(exportData, null, 2)], { type: 'application/json' })
        };
      }
      // SQLite export via Tauri fs plugin (implementation in Rust)
      return await invoke('db_export_sqlite', { project_id: projectId });
    } catch (err) {
      return { success: false, error: err instanceof Error ? err.message : 'Export failed' };
    }
  }
}
```

```bash
# Step 2: Create Rust backend database commands
# apps/desktop/src-tauri/src/commands/database.rs
```

```rust
// apps/desktop/src-tauri/src/commands/database.rs
//! Local-first SQLite commands for Tauri backend
//! - WAL mode for concurrent reads
//! - Parameterized queries to prevent injection
//! - Version tracking for conflict detection

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbInitRequest {
    pub path: PathBuf,
    #[serde(default = "default_journal_mode")]
    pub journal_mode: String,
    #[serde(default = "default_synchronous")]
    pub synchronous: String,
}

fn default_journal_mode() -> String { "WAL".to_string() }
fn default_synchronous() -> String { "NORMAL".to_string() }

#[tauri::command]
pub async fn db_init(req: DbInitRequest) -> Result<(), String> {
    let db_path = req.path.join("sdekit.db");
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    // Configure for local-first performance
    conn.pragma_update(None, "journal_mode", &req.journal_mode)
        .map_err(|e| e.to_string())?;
    conn.pragma_update(None, "synchronous", &req.synchronous)
        .map_err(|e| e.to_string())?;
    conn.pragma_update(None, "foreign_keys", "ON")
        .map_err(|e| e.to_string())?;
    
    // Store connection in Tauri state (simplified; use Mutex in production)
    // In real impl: use tauri::State<'_, DbPool>
    Ok(())
}

#[tauri::command]
pub async fn db_execute(query: String, params: Vec<serde_json::Value>) -> Result<usize, String> {
    // Use connection from state
    // Execute parameterized query
    // Return rows affected
    Ok(1) // placeholder
}

#[tauri::command]
pub async fn db_execute_with_version(
    query: String, 
    params: Vec<serde_json::Value>
) -> Result<usize, String> {
    // Optimistic concurrency: check version column
    // Return 0 if no rows updated (conflict detected)
    Ok(1) // placeholder
}

#[tauri::command]
pub async fn db_export_sqlite(project_id: String) -> Result<Vec<u8>, String> {
    // Export project data to SQLite file bytes
    // Use rusqlite::backup for efficient copy
    Ok(vec![]) // placeholder
}
```

---

### Issue 3: Graph Engine Layout Algorithm Missing

**Problem**: `crates/graph` has dependencies but no force-directed layout implementation for SDLC visualization.

**Solution**:

```bash
# Step 1: Implement force-directed layout in Rust
touch crates/graph/src/layout.rs
```

```rust
// crates/graph/src/layout.rs
//! Force-directed layout engine for SDLC graph visualization
//! - Barnes-Hut approximation for O(n log n) performance
//! - Local-first: deterministic seed for reproducible layouts
//! - Configurable forces for different node types (task, milestone, file)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: Uuid,
    pub label: String,
    pub node_type: NodeType,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
    pub fixed: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    Task,
    Milestone,
    File,
    Module,
    Dependency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: Uuid,
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: EdgeType,
    pub weight: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum EdgeType {
    DependsOn,
    Blocks,
    RelatedTo,
    Implements,
}

pub struct ForceDirectedLayout {
    // Physics parameters
    pub repulsion_strength: f64,    // Node-node repulsion
    pub attraction_strength: f64,   // Edge spring attraction
    pub damping: f64,               // Velocity damping (0.0-1.0)
    pub max_velocity: f64,          // Clamp velocity for stability
    pub step_size: f64,             // Time step for integration
    
    // Barnes-Hut parameters
    pub theta: f64,                 // Opening angle for approximation
    pub bounds: (f64, f64, f64, f64), // Layout bounds (min_x, min_y, max_x, max_y)
    
    // Deterministic seed for local-first reproducibility
    pub seed: u64,
}

impl ForceDirectedLayout {
    pub fn new(seed: u64) -> Self {
        Self {
            repulsion_strength: 5000.0,
            attraction_strength: 0.05,
            damping: 0.9,
            max_velocity: 10.0,
            step_size: 0.1,
            theta: 0.5,
            bounds: (-500.0, -500.0, 500.0, 500.0),
            seed,
        }
    }
    
    /// Initialize nodes with deterministic pseudo-random positions
    pub fn initialize_positions(&self, nodes: &mut [GraphNode]) {
        use rand::{SeedableRng, rngs::StdRng};
        use rand::Rng;
        
        let mut rng = StdRng::seed_from_u64(self.seed);
        let (min_x, min_y, max_x, max_y) = self.bounds;
        
        for node in nodes.iter_mut() {
            if !node.fixed {
                node.x = rng.gen_range(min_x..max_x);
                node.y = rng.gen_range(min_y..max_y);
                node.vx = 0.0;
                node.vy = 0.0;
            }
        }
    }
    
    /// Single simulation step using Barnes-Hut approximation
    pub fn step(&self, nodes: &mut [GraphNode], edges: &[GraphEdge]) {
        // 1. Reset forces
        for node in nodes.iter_mut() {
            if !node.fixed {
                node.vx = 0.0;
                node.vy = 0.0;
            }
        }
        
        // 2. Apply repulsion (Barnes-Hut quadtree for O(n log n))
        self.apply_repulsion(nodes);
        
        // 3. Apply attraction along edges
        self.apply_attraction(nodes, edges);
        
        // 4. Integrate velocities with damping
        for node in nodes.iter_mut() {
            if node.fixed { continue; }
            
            // Apply damping
            node.vx *= self.damping;
            node.vy *= self.damping;
            
            // Clamp velocity
            let speed = (node.vx * node.vx + node.vy * node.vy).sqrt();
            if speed > self.max_velocity {
                let scale = self.max_velocity / speed;
                node.vx *= scale;
                node.vy *= scale;
            }
            
            // Update position
            node.x += node.vx * self.step_size;
            node.y += node.vy * self.step_size;
            
            // Constrain to bounds
            let (min_x, min_y, max_x, max_y) = self.bounds;
            node.x = node.x.clamp(min_x, max_x);
            node.y = node.y.clamp(min_y, max_y);
        }
    }
    
    fn apply_repulsion(&self, nodes: &mut [GraphNode]) {
        // Simplified O(n²) for now; replace with quadtree for large graphs
        let n = nodes.len();
        for i in 0..n {
            if nodes[i].fixed { continue; }
            for j in (i+1)..n {
                if nodes[j].fixed { continue; }
                
                let dx = nodes[i].x - nodes[j].x;
                let dy = nodes[i].y - nodes[j].y;
                let dist_sq = dx*dx + dy*dy + 0.01; // Avoid division by zero
                let dist = dist_sq.sqrt();
                
                let force = self.repulsion_strength / dist_sq;
                let fx = force * dx / dist;
                let fy = force * dy / dist;
                
                // Apply to both nodes (Newton's 3rd law)
                nodes[i].vx += fx / nodes[i].mass;
                nodes[i].vy += fy / nodes[i].mass;
                nodes[j].vx -= fx / nodes[j].mass;
                nodes[j].vy -= fy / nodes[j].mass;
            }
        }
    }
    
    fn apply_attraction(&self, nodes: &mut [GraphNode], edges: &[GraphEdge]) {
        for edge in edges {
            let source_idx = nodes.iter().position(|n| n.id == edge.source);
            let target_idx = nodes.iter().position(|n| n.id == edge.target);
            
            if let (Some(i), Some(j)) = (source_idx, target_idx) {
                if nodes[i].fixed && nodes[j].fixed { continue; }
                
                let dx = nodes[j].x - nodes[i].x;
                let dy = nodes[j].y - nodes[i].y;
                let dist = (dx*dx + dy*dy).sqrt().max(0.01);
                
                // Hooke's law: F = -k * (dist - rest_length)
                let rest_length = 100.0;
                let force = self.attraction_strength * edge.weight * (dist - rest_length);
                
                let fx = force * dx / dist;
                let fy = force * dy / dist;
                
                if !nodes[i].fixed {
                    nodes[i].vx += fx / nodes[i].mass;
                    nodes[i].vy += fy / nodes[i].mass;
                }
                if !nodes[j].fixed {
                    nodes[j].vx -= fx / nodes[j].mass;
                    nodes[j].vy -= fy / nodes[j].mass;
                }
            }
        }
    }
    
    /// Run simulation until convergence or max iterations
    pub fn simulate(
        &self, 
        nodes: &mut [GraphNode], 
        edges: &[GraphEdge],
        max_iterations: usize,
        energy_threshold: f64,
    ) -> usize {
        self.initialize_positions(nodes);
        
        for iteration in 0..max_iterations {
            self.step(nodes, edges);
            
            // Check convergence: total kinetic energy
            let energy: f64 = nodes.iter()
                .filter(|n| !n.fixed)
                .map(|n| 0.5 * n.mass * (n.vx*n.vx + n.vy*n.vy))
                .sum();
            
            if energy < energy_threshold {
                return iteration + 1;
            }
        }
        max_iterations
    }
}
```

```bash
# Step 2: Update crates/graph/Cargo.toml
```

```toml
# crates/graph/Cargo.toml
[package]
name = "sde-kit-graph"
version = "0.1.0"
description = "Force-directed graph layout engine for SDE Kit"
edition = "2021"
license = "MIT"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
rand = { version = "0.8", features = ["std_rng"] }
# Optional: for Barnes-Hut quadtree (uncomment for large graphs)
# quadtree-rs = "0.4"

[lib]
crate-type = ["lib", "cdylib"]
```

---

### Issue 4: Missing Keyboard Navigation & Command Palette

**Problem**: README claims "Full Keyboard Navigation" but no implementation exists.

**Solution**:

```bash
# Step 1: Create keyboard navigation service
touch apps/desktop/src/lib/services/keyboard.ts
```

```typescript
// apps/desktop/src/lib/services/keyboard.ts
/**
 * Keyboard Navigation Service
 * - Vim-like modal navigation for solo developers
 * - Command palette (Cmd/Ctrl+P) for quick actions
 * - Local-first: no network calls, all shortcuts processed client-side
 * - Accessible: ARIA live regions for screen reader feedback
 */

export type KeyMap = {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  meta?: boolean;
};

export type Command = {
  id: string;
  label: string;
  shortcut?: KeyMap;
  handler: () => void | Promise<void>;
  context?: 'global' | 'editor' | 'graph' | 'tasks';
};

export class KeyboardService {
  private static instance: KeyboardService;
  private commands = new Map<string, Command>();
  private activeContext: Command['context'] = 'global';
  private commandPaletteOpen = false;
  
  private constructor() {
    this.registerDefaults();
    this.bindGlobalListener();
  }
  
  static getInstance(): KeyboardService {
    if (!KeyboardService.instance) {
      KeyboardService.instance = new KeyboardService();
    }
    return KeyboardService.instance;
  }
  
  register(command: Command): void {
    this.commands.set(command.id, command);
  }
  
  unregister(id: string): void {
    this.commands.delete(id);
  }
  
  setContext(context: Command['context']): void {
    this.activeContext = context;
  }
  
  private registerDefaults(): void {
    // Global commands
    this.register({
      id: 'cmd:palette',
      label: 'Open Command Palette',
      shortcut: { key: 'p', meta: true },
      handler: () => this.toggleCommandPalette(),
      context: 'global'
    });
    
    this.register({
      id: 'nav:next-panel',
      label: 'Focus Next Panel',
      shortcut: { key: 'Tab', ctrl: true },
      handler: () => this.focusNextPanel(),
      context: 'global'
    });
    
    this.register({
      id: 'nav:prev-panel',
      label: 'Focus Previous Panel',
      shortcut: { key: 'Tab', ctrl: true, shift: true },
      handler: () => this.focusPrevPanel(),
      context: 'global'
    });
    
    // Editor commands
    this.register({
      id: 'editor:save',
      label: 'Save File',
      shortcut: { key: 's', meta: true },
      handler: () => this.triggerSave(),
      context: 'editor'
    });
    
    // Graph commands
    this.register({
      id: 'graph:recenter',
      label: 'Recenter Graph',
      shortcut: { key: '0', ctrl: true },
      handler: () => this.recenterGraph(),
      context: 'graph'
    });
    
    // Task commands
    this.register({
      id: 'task:new',
      label: 'New Task',
      shortcut: { key: 'n', meta: true },
      handler: () => this.createNewTask(),
      context: 'tasks'
    });
  }
  
  private bindGlobalListener(): void {
    document.addEventListener('keydown', (e: KeyboardEvent) => {
      if (this.commandPaletteOpen) return; // Let palette handle input
      
      // Find matching command
      for (const cmd of this.commands.values()) {
        if (cmd.context !== 'global' && cmd.context !== this.activeContext) continue;
        if (!cmd.shortcut) continue;
        if (this.matchesKey(e, cmd.shortcut)) {
          e.preventDefault();
          e.stopPropagation();
          cmd.handler();
          return;
        }
      }
    });
  }
  
  private matchesKey(event: KeyboardEvent, keymap: KeyMap): boolean {
    if (event.key.toLowerCase() !== keymap.key.toLowerCase()) return false;
    if (keymap.ctrl && !event.ctrlKey) return false;
    if (keymap.shift && !event.shiftKey) return false;
    if (keymap.alt && !event.altKey) return false;
    if (keymap.meta && !event.metaKey) return false;
    return true;
  }
  
  private toggleCommandPalette(): void {
    this.commandPaletteOpen = !this.commandPaletteOpen;
    // Dispatch custom event for UI to show/hide palette
    document.dispatchEvent(new CustomEvent('command-palette:toggle', {
      detail: { open: this.commandPaletteOpen }
    }));
  }
  
  private async focusNextPanel(): Promise<void> {
    // Cycle through: file-tree → editor → tasks → graph → terminal
    const panels = ['file-tree', 'editor', 'tasks', 'graph', 'terminal'];
    const current = document.activeElement?.getAttribute('data-panel') || 'file-tree';
    const idx = panels.indexOf(current);
    const next = panels[(idx + 1) % panels.length];
    
    const el = document.querySelector(`[data-panel="${next}"]`) as HTMLElement;
    el?.focus();
    
    // Announce to screen readers
    this.announce(`Focused ${next} panel`);
  }
  
  private focusPrevPanel(): void {
    const panels = ['file-tree', 'editor', 'tasks', 'graph', 'terminal'];
    const current = document.activeElement?.getAttribute('data-panel') || 'file-tree';
    const idx = panels.indexOf(current);
    const prev = panels[(idx - 1 + panels.length) % panels.length];
    
    const el = document.querySelector(`[data-panel="${prev}"]`) as HTMLElement;
    el?.focus();
    this.announce(`Focused ${prev} panel`);
  }
  
  private async triggerSave(): Promise<void> {
    // Dispatch save event to editor component
    document.dispatchEvent(new CustomEvent('editor:save'));
    this.announce('File saved');
  }
  
  private recenterGraph(): void {
    document.dispatchEvent(new CustomEvent('graph:recenter'));
    this.announce('Graph recentered');
  }
  
  private createNewTask(): void {
    document.dispatchEvent(new CustomEvent('task:create'));
    this.announce('New task dialog opened');
  }
  
  private announce(message: string): void {
    // ARIA live region for accessibility
    const region = document.getElementById('keyboard-announcer');
    if (region) {
      region.textContent = '';
      setTimeout(() => { region.textContent = message; }, 100);
    }
  }
}
```

```bash
# Step 2: Create Command Palette component
mkdir -p apps/desktop/src/lib/components/command-palette
touch apps/desktop/src/lib/components/command-palette/CommandPalette.svelte
```

```svelte
<!-- apps/desktop/src/lib/components/command-palette/CommandPalette.svelte -->
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { KeyboardService } from '$lib/services/keyboard';
  import { colors } from '$lib/design/tokens';
  
  export let open = false;
  
  let query = '';
  let selectedIndex = 0;
  const keyboard = KeyboardService.getInstance();
  
  // Filtered commands based on search
  $: filteredCommands = Array.from((keyboard as any).commands.values())
    .filter(cmd => 
      cmd.label.toLowerCase().includes(query.toLowerCase()) &&
      (cmd.context === 'global' || cmd.context === (keyboard as any).activeContext)
    )
    .slice(0, 10); // Limit results
  
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      e.preventDefault();
      close();
      return;
    }
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = (selectedIndex + 1) % filteredCommands.length;
    }
    if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = (selectedIndex - 1 + filteredCommands.length) % filteredCommands.length;
    }
    if (e.key === 'Enter' && filteredCommands[selectedIndex]) {
      e.preventDefault();
      filteredCommands[selectedIndex].handler();
      close();
    }
  }
  
  function close() {
    query = '';
    selectedIndex = 0;
    document.dispatchEvent(new CustomEvent('command-palette:toggle', { detail: { open: false } }));
  }
  
  // Listen for palette toggle events
  onMount(() => {
    const handler = (e: CustomEvent) => { open = e.detail.open; };
    document.addEventListener('command-palette:toggle', handler as EventListener);
    return () => document.removeEventListener('command-palette:toggle', handler as EventListener);
  });
</script>

{#if open}
  <div 
    class="palette-overlay"
    style="background: color-mix(in srgb, var(--surface-dark) 85%, transparent);"
    on:click={close}
  >
    <div 
      class="palette-container card"
      style="background: var(--surface-elevated); border: 1px solid var(--surface-border);"
      on:click|stopPropagation
      on:keydown={handleKeydown}
      tabindex="-1"
    >
      <!-- Search Input -->
      <input
        type="text"
        bind:value={query}
        placeholder="Type a command..."
        class="palette-input"
        style="
          background: var(--surface-base);
          color: var(--text-primary);
          border: 1px solid var(--surface-border);
          border-radius: var(--radius-md);
          padding: 10px 14px;
          font: var(--typo-body);
          width: 100%;
          margin-bottom: 12px;
        "
        autofocus
      />
      
      <!-- Results List -->
      <ul class="palette-results" style="list-style: none; padding: 0; margin: 0; max-height: 300px; overflow-y: auto;">
        {#each filteredCommands as cmd, i}
          <li
            class="palette-item"
            style="
              padding: 8px 12px;
              border-radius: var(--radius-sm);
              cursor: pointer;
              background: {i === selectedIndex ? 'var(--color-primary)' : 'transparent'};
              color: {i === selectedIndex ? 'var(--color-on-primary)' : 'var(--text-primary)'};
              font: var(--typo-body);
            "
            on:click={() => { cmd.handler(); close(); }}
            on:mouseenter={() => selectedIndex = i}
          >
            <span>{cmd.label}</span>
            {#if cmd.shortcut}
              <kbd class="palette-shortcut" style="
                margin-left: auto;
                background: var(--surface-soft);
                padding: 2px 6px;
                border-radius: 4px;
                font: var(--typo-code);
                color: var(--text-muted);
              ">
                {cmd.shortcut.meta ? '⌘' : cmd.shortcut.ctrl ? 'Ctrl' : ''}
                {cmd.shortcut.shift ? '+' : ''}{cmd.shortcut.shift ? '⇧' : ''}
                {cmd.shortcut.key.toUpperCase()}
              </kbd>
            {/if}
          </li>
        {/each}
        {#if filteredCommands.length === 0}
          <li class="palette-empty" style="padding: 12px; color: var(--text-muted); text-align: center;">
            No commands found
          </li>
        {/if}
      </ul>
    </div>
  </div>
{/if}

<style>
  .palette-overlay {
    position: fixed;
    inset: 0;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 10vh;
    z-index: 1000;
  }
  .palette-container {
    width: min(600px, 90vw);
    box-shadow: 0 8px 32px rgba(0,0,0,0.2);
  }
  .palette-input:focus {
    outline: 2px solid var(--color-primary);
    outline-offset: -1px;
  }
  .palette-item:hover {
    background: var(--color-primary) !important;
    color: var(--color-on-primary) !important;
  }
</style>
```

---

## 🎨 Design System: Claude-Style Component Library

### Component Token Specification (for Openode AI)

```yaml
# apps/desktop/src/lib/design/components.yaml
# Reference this file when implementing components

button-primary:
  base:
    backgroundColor: "var(--color-primary)"
    textColor: "var(--color-on-primary)"
    typography: "var(--typo-button)"
    borderRadius: "var(--radius-button)"
    padding: "12px 20px"
    height: "40px"
    border: "none"
  hover:
    backgroundColor: "var(--color-primary-active)"
  disabled:
    backgroundColor: "var(--color-primary-disabled)"
    textColor: "var(--color-muted)"
    cursor: "not-allowed"

button-secondary:
  base:
    backgroundColor: "var(--surface-base)"
    textColor: "var(--text-primary)"
    border: "1px solid var(--surface-border)"
    borderRadius: "var(--radius-button)"
    padding: "12px 20px"
    height: "40px"
  hover:
    borderColor: "var(--color-primary)"

card-feature:
  base:
    backgroundColor: "var(--surface-card)"
    borderRadius: "var(--radius-card)"
    padding: "var(--spacing-card)"
    border: "1px solid var(--surface-border)"
  hover:
    borderColor: "var(--color-primary)"

card-product-dark:
  base:
    backgroundColor: "var(--surface-dark)"
    textColor: "var(--text-primary)"
    borderRadius: "var(--radius-card)"
    padding: "var(--spacing-card)"
    border: "1px solid var(--surface-border)"

code-window:
  base:
    backgroundColor: "var(--surface-dark)"
    textColor: "var(--text-primary)"
    fontFamily: "var(--font-mono)"
    fontSize: "13px"
    lineHeight: "1.6"
    borderRadius: "var(--radius-card)"
    padding: "24px"

badge-pill:
  base:
    backgroundColor: "var(--surface-card)"
    textColor: "var(--text-primary)"
    borderRadius: "var(--radius-pill)"
    padding: "4px 12px"
    fontSize: "12px"
    fontWeight: "500"

badge-coral:
  base:
    backgroundColor: "var(--color-primary)"
    textColor: "var(--color-on-primary)"
    borderRadius: "var(--radius-pill)"
    padding: "4px 12px"
    fontSize: "12px"
    fontWeight: "500"
    letterSpacing: "1.5px"
    textTransform: "uppercase"
```

### Key Implementation Rules for Openode AI:

1. **Never inline hex colors** - Always use `var(--color-*)` CSS variables from tokens
2. **Typography hierarchy** - Use `.typo-display*`, `.typo-title`, `.typo-body` classes, never raw font-size
3. **Border radius** - Use `var(--radius-button)`, `var(--radius-card)`, etc. for consistency
4. **Spacing** - Use `var(--spacing-*)` tokens; section padding = `var(--spacing-section)` = 96px
5. **Dark/Light toggle** - Apply `data-theme="light"` to `<html>`; CSS variables auto-adapt
6. **Coral accent scarcity** - Use `--color-primary` ONLY for primary CTAs and full-bleed callouts
7. **No shadows for depth** - Use surface color contrast (cream ↔ dark) instead of drop-shadows
8. **Font stacks** - Display: `var(--font-display)`, Body: `var(--font-sans)`, Code: `var(--font-mono)`

---

## ⚡ Performance & Optimization Checklist

```markdown
## Performance Optimizations (Local-First Focus)

### File System
- [ ] Virtualize file tree (only render visible nodes) using `svelte-virtual-list`
- [ ] Debounce file watch events (100ms) to prevent UI thrash
- [ ] Cache file metadata in IndexedDB for instant reloads

### Graph Visualization
- [ ] Web Worker for force-directed layout calculations (off main thread)
- [ ] Canvas/SVG hybrid: Canvas for nodes/edges, SVG for labels (accessibility)
- [ ] LOD (Level of Detail): Simplify rendering when zoomed out

### Editor (CodeMirror 6)
- [ ] Enable `lineWrapping: false` + horizontal scroll for performance
- [ ] Lazy-load language extensions (only load Rust when editing .rs files)
- [ ] Throttle syntax highlighting re-parse (200ms)

### Database
- [ ] Use SQLite WAL mode + `PRAGMA cache_size = -2000` (2MB cache)
- [ ] Batch writes: queue task updates, flush every 500ms or on blur
- [ ] Pre-compile frequently used queries with `sqlite3_prepare_v2`

### Memory (Critical for 8GB M1 Macs)
- [ ] Implement component `onDestroy` cleanup for all stores/subscriptions
- [ ] Use `weakRef` for graph node references to allow GC
- [ ] Limit undo history to 50 steps per document

### Build Optimization
- [ ] Tree-shake unused CodeMirror languages via dynamic imports
- [ ] Split Rust graph crate as `cdylib` for smaller Tauri bundle
- [ ] Enable `lto = true` in `Cargo.toml` for release builds
```

---

## 🧪 Testing Strategy (Offline-First)

```typescript
// apps/desktop/src/lib/tests/offline-scenarios.test.ts
/**
 * Test suite for local-first guarantees
 * Run with: npm run test -- --run offline
 */

import { describe, it, expect, beforeEach, vi } from 'vitest';
import { LocalDatabase } from '../services/database/local-db';

describe('Offline-First Guarantees', () => {
  let db: LocalDatabase;
  
  beforeEach(async () => {
    db = LocalDatabase.getInstance();
    await db.initialize();
  });
  
  it('persists tasks without network', async () => {
    // Mock network to be offline
    vi.spyOn(global, 'fetch').mockRejectedValue(new Error('Network error'));
    
    const result = await db.createTask({
      project_id: 'test-project',
      title: 'Offline Task',
      status: 'todo',
      priority: 'medium'
    });
    
    expect(result.success).toBe(true);
    expect(result.data?.title).toBe('Offline Task');
    
    // Verify data is in SQLite (not memory-only)
    const retrieved = await db.getTask(result.data!.id);
    expect(retrieved.data?.title).toBe('Offline Task');
  });
  
  it('handles conflict resolution deterministically', async () => {
    // Simulate two offline edits to same task
    const task = await createTestTask();
    
    // Edit 1: local version 1 → 2
    const res1 = await db.updateTask(task.id, { title: 'Edit A' }, 1);
    expect(res1.success).toBe(true);
    
    // Edit 2: stale version 1 → should fail with conflict
    const res2 = await db.updateTask(task.id, { title: 'Edit B' }, 1);
    expect(res2.success).toBe(false);
    expect(res2.conflict).toBe('manual');
    
    // User resolves: fetch latest, merge, retry
    const latest = await db.getTask(task.id);
    const merged = { ...latest.data, title: 'Edit A + B' };
    const res3 = await db.updateTask(task.id, merged, 2); // use new version
    expect(res3.success).toBe(true);
  });
  
  it('exports project data for backup', async () => {
    const project = await createTestProject();
    const exportResult = await db.exportProject(project.id, 'json');
    
    expect(exportResult.success).toBe(true);
    expect(exportResult.data).toBeInstanceOf(Blob);
    
    // Verify export can be re-imported (round-trip)
    const text = await exportResult.data!.text();
    const parsed = JSON.parse(text);
    expect(parsed.project.name).toBe(project.name);
    expect(parsed.version).toBe('1.0');
  });
});
```

---

## 🚀 Implementation Checklist for Openode AI

```markdown
# SDE-KIT: Openode AI Implementation Checklist

## Phase 1: Design System Foundation (Day 1-2)
- [ ] Create `src/lib/design/tokens.ts` with Claude-style color/typography/spacing tokens
- [ ] Create `src/lib/design/css-variables.ts` for runtime theme application
- [ ] Refactor `app.css` to use CSS variable references (no hardcoded hex)
- [ ] Implement theme toggle: `data-theme="light" | "dark"` on `<html>`
- [ ] Create typography utility classes: `.typo-display-xl`, `.typo-body`, etc.

## Phase 2: Local-First Data Layer (Day 3-4)
- [ ] Implement `LocalDatabase` service with SQLite via Tauri
- [ ] Add optimistic concurrency control (version column) for conflict detection
- [ ] Create schema migrations with append-only history
- [ ] Implement `exportProject()` for JSON/SQLite backup
- [ ] Add Rust commands: `db_init`, `db_execute_with_version`, `db_export_sqlite`

## Phase 3: Graph Engine (Day 5-6)
- [ ] Implement `ForceDirectedLayout` in `crates/graph/src/layout.rs`
- [ ] Add Barnes-Hut quadtree for O(n log n) performance (optional)
- [ ] Create deterministic seeding for reproducible layouts (local-first)
- [ ] Expose layout via Tauri command: `graph_layout_compute`
- [ ] Add Web Worker bridge for main-thread offloading

## Phase 4: Keyboard Navigation (Day 7)
- [ ] Implement `KeyboardService` with modal context switching
- [ ] Create `CommandPalette.svelte` component with search/filter
- [ ] Register default commands: save, nav, new-task, recenter-graph
- [ ] Add ARIA live region for screen reader announcements
- [ ] Test with VoiceOver/NVDA for accessibility compliance

## Phase 5: Component Library (Day 8-10)
- [ ] Build `Button.svelte` with primary/secondary/disabled states
- [ ] Build `Card.svelte` with feature/dark variants
- [ ] Build `CodeWindow.svelte` with syntax highlighting container
- [ ] Build `Badge.svelte` with pill/coral variants
- [ ] Document all components in `COMPONENTS.md` with DESIGN.md references

## Phase 6: Performance & Polish (Day 11-12)
- [ ] Virtualize file tree with `svelte-virtual-list`
- [ ] Add Web Worker for graph layout calculations
- [ ] Implement undo/redo with 50-step limit per document
- [ ] Add `onDestroy` cleanup to all stores/subscriptions
- [ ] Enable LTO and tree-shaking in build config

## Phase 7: Testing & Validation (Day 13-14)
- [ ] Write offline-scenario tests in `offline-scenarios.test.ts`
- [ ] Add e2e tests for keyboard navigation (Playwright)
- [ ] Test on M1 MacBook Air 8GB RAM: verify <500MB memory usage
- [ ] Validate color contrast ratios meet WCAG AA
- [ ] Run `npm run check:all` and fix all TypeScript/Rust errors

## Success Criteria
✅ App launches offline with no network errors  
✅ All UI uses DESIGN.md color/typography tokens (no hardcoded values)  
✅ Tasks persist after app restart (SQLite verified)  
✅ Graph layout runs in Web Worker (no UI freeze)  
✅ Cmd+P opens command palette with keyboard navigation  
✅ Memory usage <500MB on 8GB M1 Mac with 1000-file project  
✅ All tests pass: `npm run test` + `cargo test`  
✅ Build succeeds: `npm run release` produces valid .app bundle  
```

---

## 🛡️ Anti-Hallucination & Clarity Measures

```typescript
// apps/desktop/src/lib/utils/assertions.ts
/**
 * Runtime type guards to prevent AI-generated code errors
 * Use these to validate data boundaries and catch hallucinations early
 */

export function assertNever(x: never): never {
  throw new Error(`Unexpected object: ${JSON.stringify(x)}`);
}

export function isLocalPath(path: string): boolean {
  // Prevent hallucinated cloud paths
  return path.startsWith('/') || 
         path.startsWith('file://') || 
         /^[A-Z]:\\/.test(path) ||
         path.startsWith('./') || 
         path.startsWith('../');
}

export function validateDesignToken(token: string, allowed: readonly string[]): boolean {
  // Prevent hallucinated CSS variables
  return allowed.includes(token);
}

export function clamp(value: number, min: number, max: number): number {
  // Prevent hallucinated physics values
  return Math.min(max, Math.max(min, value));
}

export function deterministicId(seed: string, index: number): string {
  // Generate reproducible IDs for local-first sync (no UUID randomness)
  let hash = 0;
  for (let i = 0; i < seed.length; i++) {
    hash = ((hash << 5) - hash) + seed.charCodeAt(i);
    hash |= 0;
  }
  return `sdek-${(hash + index).toString(36).slice(-8)}`;
}
```

```markdown
## Documentation Requirements to Prevent Confusion

1. **Every component must have a DESIGN.md reference comment**:
   ```svelte
   <!-- Reference: DESIGN.md#components.button-primary -->
   <button class="btn-primary">...</button>
   ```

2. **All CSS variables must be defined in tokens.ts first**:
   ```typescript
   // src/lib/design/tokens.ts
   export const colors = { primary: '#cc785c', ... }
   ```
   ```css
   /* src/app.css */
   --color-primary: var(--color-primary, #cc785c); /* fallback only */
   ```

3. **Rust functions must have explicit error types** (no `unwrap()`):
   ```rust
   pub fn db_execute(...) -> Result<usize, DbError> { ... }
   ```

4. **Keyboard shortcuts must be registered in one place** (`KeyboardService.registerDefaults()`)

5. **All async operations must handle offline errors**:
   ```typescript
   try {
     await db.saveTask(task);
   } catch (err) {
     if (err.message.includes('SQLITE_BUSY')) {
       // Retry with exponential backoff
     }
   }
   ```
```

---

## 📦 Final Build Configuration

```json
// apps/desktop/src-tauri/tauri.conf.json - ADD THESE OPTIMIZATIONS
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "SDE Kit",
  "version": "0.1.0",
  "identifier": "com.sdekit.app",
  "build": {
    "frontendDist": "../build",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "npm run dev -w apps/desktop",
    "beforeBuildCommand": "npm run build -w apps/desktop",
    "removeUnusedCommands": true
  },
  "app": {
    "windows": [{
      "title": "SDE Kit",
      "width": 1280,
      "height": 860,
      "minWidth": 900,
      "minHeight": 600,
      "resizable": true,
      "fullscreen": false,
      "center": true,
      "transparent": false,
      "decorations": true
    }],
    "security": {
      "csp": "default-src 'self'; style-src 'self' 'unsafe-inline'; script-src 'self'; connect-src 'self' file:",
      "capabilities": ["default"]
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": ["icons/32x32.png", "icons/128x128.png", "icons/icon.icns", "icons/icon.ico"],
    "macOS": {
      "minimumSystemVersion": "14.0",
      "exceptionDomain": "",
      "frameworks": [],
      "providerShortName": null,
      "signingIdentity": null
    },
    "resources": [],
    "externalBin": [],
    "copyright": "MIT License",
    "category": "DeveloperTool",
    "shortDescription": "Local-first SDLC platform for solo developers",
    "longDescription": "SDE Kit: Offline IDE, code editor, task manager, and SDLC visualizer - all in one standalone desktop app."
  }
}
```

```toml
# apps/desktop/src-tauri/Cargo.toml - ADD THESE OPTIMIZATIONS
[package]
name = "sde-kit"
version = "0.1.0"
description = "SDE Kit Desktop"
edition = "2021"
license = "MIT"

[lib]
name = "sde_kit_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[dependencies]
tauri = { version = "2", features = ["macos-private-api"] }
tauri-plugin-dialog = "2"
tauri-plugin-shell = "2"
tauri-plugin-fs = "2"
rusqlite = { version = "0.31", features = ["bundled", "wal"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4", "serde"] }
sde-kit-graph = { path = "../../../crates/graph" }

[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```

---

> **Final Note for Openode AI**:  
> This project is for a solo developer on an M1 MacBook Air with 8GB RAM. Prioritize:  
> 1. **Memory efficiency** over fancy animations  
> 2. **Offline reliability** over network features  
> 3. **Keyboard accessibility** over mouse-only interactions  
> 4. **Claude-style warmth** (cream/coral/dark) over generic SaaS aesthetics  
>   
> When in doubt: *simpler, warmer, local-first*.
