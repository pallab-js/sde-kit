# SDE-KIT — Comprehensive Critical Review & Step-by-Step Implementation Guide

> **How to use this document with an AI coding assistant:**
> Work through each section sequentially. Each fix or feature is self-contained with exact file paths, exact code to write, and a verification step. Do NOT skip steps — later changes depend on earlier ones. When pasting code, replace only what is indicated. If the AI assistant hallucinates file paths, names, or APIs not present in this document, stop and re-read the relevant section before continuing.

---

## Project Overview (Ground Truth)

| Layer | Technology | Version |
|---|---|---|
| Frontend | SvelteKit 5 (Svelte 5 runes) + TypeScript | svelte ^5.55, kit ^2.57 |
| UI Styling | Custom CSS vars (no Tailwind used at runtime) | — |
| Code Editor | CodeMirror 6 | cm6 |
| Desktop runtime | Tauri 2 | tauri ^2 |
| Backend language | Rust 2021 | msrv 1.77.2 |
| Database | SQLite via rusqlite (bundled) | rusqlite 0.32 |
| Graph engine | Custom crate `sde-kit-graph` | local |
| File watching | `notify` crate | notify 8 |

**Monorepo layout (all paths relative to repo root):**

```
SDE-KIT/
  apps/desktop/                   ← SvelteKit + Tauri app
    src/lib/components/           ← Svelte UI components
    src/lib/services/             ← Tauri invoke wrappers
    src/lib/stores/               ← Svelte stores (state)
    src/lib/types/index.ts        ← Shared TypeScript types
    src-tauri/src/
      commands/mod.rs             ← CRUD Tauri commands
      commands/fs.rs              ← Filesystem Tauri commands
      commands/graph.rs           ← Graph Tauri commands
      models/mod.rs               ← Rust structs
      persistence/mod.rs          ← SQLite init
      watcher.rs                  ← FS watcher (unused)
      lib.rs                      ← Tauri builder + state
  crates/graph/src/
    types.rs                      ← Graph data structures
    layout.rs                     ← Force-directed layout
    lib.rs                        ← Tests
```

---

## Section 1 — Critical Bugs (Fix These First)

These are broken or data-corrupting behaviours that must be fixed before any feature work.

---

### Bug 1 — Graph state is never persisted (data loss on restart)

**Problem:** `GraphState(Mutex<Graph>)` lives only in process memory. Every app restart silently wipes all graph nodes and edges. This makes the SDLC visualiser useless.

**Fix: add a `graphs` table to SQLite and persist/reload the graph on startup.**

#### Step 1.1 — Add graph tables to the DB schema

**File:** `apps/desktop/src-tauri/src/persistence/mod.rs`

In the `initialize` function, append these two tables to the `execute_batch` string (add before the final `"`):

```sql
CREATE TABLE IF NOT EXISTS graph_nodes (
    id TEXT PRIMARY KEY,
    node_type TEXT NOT NULL,
    label TEXT NOT NULL,
    metadata TEXT
);

CREATE TABLE IF NOT EXISTS graph_edges (
    id TEXT PRIMARY KEY,
    source_id TEXT NOT NULL REFERENCES graph_nodes(id),
    target_id TEXT NOT NULL REFERENCES graph_nodes(id),
    edge_type TEXT NOT NULL,
    label TEXT
);
```

#### Step 1.2 — Add graph persistence helpers to persistence/mod.rs

Append these two public functions at the bottom of `persistence/mod.rs`:

```rust
use sde_kit_graph::types::{GraphEdge, GraphNode, GraphSnapshot};

pub fn save_graph(conn: &rusqlite::Connection, snap: &GraphSnapshot) -> Result<(), DbError> {
    conn.execute("DELETE FROM graph_edges", [])?;
    conn.execute("DELETE FROM graph_nodes", [])?;
    for n in &snap.nodes {
        conn.execute(
            "INSERT INTO graph_nodes (id, node_type, label, metadata) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![
                n.id, n.node_type, n.label,
                n.metadata.as_ref().map(|m| m.to_string())
            ],
        )?;
    }
    for e in &snap.edges {
        conn.execute(
            "INSERT INTO graph_edges (id, source_id, target_id, edge_type, label) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![e.id, e.source_id, e.target_id, e.edge_type, e.label],
        )?;
    }
    Ok(())
}

pub fn load_graph(conn: &rusqlite::Connection) -> Result<GraphSnapshot, DbError> {
    let mut stmt = conn.prepare(
        "SELECT id, node_type, label, metadata FROM graph_nodes"
    )?;
    let nodes: Vec<GraphNode> = stmt.query_map([], |row| {
        let meta_str: Option<String> = row.get(3)?;
        let metadata = meta_str
            .and_then(|s| serde_json::from_str(&s).ok());
        Ok(GraphNode {
            id: row.get(0)?,
            node_type: row.get(1)?,
            label: row.get(2)?,
            metadata,
        })
    })?.filter_map(|r| r.ok()).collect();

    let mut stmt = conn.prepare(
        "SELECT id, source_id, target_id, edge_type, label FROM graph_edges"
    )?;
    let edges: Vec<GraphEdge> = stmt.query_map([], |row| {
        Ok(GraphEdge {
            id: row.get(0)?,
            source_id: row.get(1)?,
            target_id: row.get(2)?,
            edge_type: row.get(3)?,
            label: row.get(4)?,
        })
    })?.filter_map(|r| r.ok()).collect();

    Ok(GraphSnapshot { nodes, edges })
}
```

#### Step 1.3 — Load graph from DB on startup in lib.rs

**File:** `apps/desktop/src-tauri/src/lib.rs`

In the `.setup(|app| { ... })` closure, after `app.manage(GraphState(Mutex::new(Graph::new())));`, add:

```rust
// Restore persisted graph
{
    let db = app.state::<Database>();
    let conn = db.conn.lock().expect("db lock");
    if let Ok(snap) = crate::persistence::load_graph(&conn) {
        let graph_state = app.state::<GraphState>();
        let mut g = graph_state.0.lock().expect("graph lock");
        for node in snap.nodes { g.add_node(node); }
        for edge in snap.edges { g.add_edge(edge); }
    }
}
```

#### Step 1.4 — Call save_graph after every mutating graph command

**File:** `apps/desktop/src-tauri/src/commands/graph.rs`

Add `use crate::persistence::{save_graph, Database};` and `use tauri::State;` at top (State is already imported).

Create a helper at the top of the file:

```rust
fn persist(g: &sde_kit_graph::types::Graph, db: &Database) -> Result<(), String> {
    let snap = g.snapshot();
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    save_graph(&conn, &snap).map_err(|e| e.to_string())
}
```

Then update each mutating command signature to also accept `db: State<Database>` and call `persist(&g, &db)?` before returning `Ok(...)`. Example for `add_graph_node`:

```rust
#[tauri::command]
pub fn add_graph_node(
    node_type: String, label: String,
    metadata: Option<serde_json::Value>,
    graph: State<GraphState>,
    db: State<Database>,
) -> Result<GraphNode, String> {
    let mut g = graph.0.lock().map_err(|e| e.to_string())?;
    let node = GraphNode { metadata, ..GraphNode::new(node_type, label) };
    g.add_node(node.clone());
    persist(&g, &db)?;
    Ok(node)
}
```

Apply the same pattern to: `remove_graph_node`, `add_graph_edge`, `remove_graph_edge`, `clear_graph`.

Also update the `tauri::generate_handler![]` macro signatures in `lib.rs` — Tauri will auto-inject the new `State<Database>` parameter; no change needed to the macro list.

**Verification:** Run app, add nodes and edges, quit, relaunch — graph must still be present.

---

### Bug 2 — File watcher is dead (watcher dropped immediately)

**Problem:** `watcher.rs` creates a `RecommendedWatcher` but the variable goes out of scope when `start_watching` returns, so the OS watcher is torn down instantly. The spawned thread then receives nothing. The function is also never called from `lib.rs`.

**Fix: keep the watcher alive by moving it into the thread.**

**File:** `apps/desktop/src-tauri/src/watcher.rs` — replace entirely:

```rust
use notify::{Config, Event, EventKind, RecommendedWatcher, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use tauri::{AppHandle, Emitter};

/// Spawn a background thread that watches `path` recursively and emits
/// "fs-event" Tauri events. Returns immediately; watching continues
/// until the app exits.
pub fn start_watching(app: AppHandle, path: String) -> Result<(), String> {
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();

    // Watcher is moved into the thread so it lives as long as the thread.
    thread::spawn(move || {
        let mut watcher = RecommendedWatcher::new(
            move |res| { let _ = tx.send(res); },
            Config::default(),
        )
        .expect("failed to create watcher");

        watcher
            .watch(Path::new(&path), notify::RecursiveMode::Recursive)
            .expect("failed to watch path");

        for res in rx {
            if let Ok(event) = res {
                let kind = match event.kind {
                    EventKind::Create(_) => "created",
                    EventKind::Modify(_) => "modified",
                    EventKind::Remove(_) => "removed",
                    _ => continue,
                };
                let paths: Vec<String> = event
                    .paths
                    .iter()
                    .map(|p| p.to_string_lossy().replace('\\', "/"))
                    .collect();
                let _ = app.emit("fs-event", serde_json::json!({ "kind": kind, "paths": paths }));
            }
        }
        // watcher dropped here — watching ends cleanly on thread exit
    });

    Ok(())
}
```

**File:** `apps/desktop/src-tauri/src/commands/fs.rs` — update `set_workspace_root`:

```rust
#[tauri::command]
pub fn set_workspace_root(
    path: String,
    root: State<WorkspaceRoot>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let p = Path::new(&path).canonicalize().map_err(|e| format!("invalid path: {e}"))?;
    if !p.is_dir() {
        return Err("not a directory".to_string());
    }
    let mut state = root.0.lock().map_err(|e| e.to_string())?;
    *state = Some(p.clone());
    // Start watching the new workspace root
    crate::watcher::start_watching(app, normalize_path(&p))?;
    Ok(())
}
```

Add `use crate::watcher;` at top of `fs.rs` — and `use tauri;` is already available via the crate.

**Verification:** Open a folder in the app, edit a file externally — the FileTree must refresh (see Bug 3 for the frontend listener).

---

### Bug 3 — fs-event is emitted but no frontend listener updates the FileTree

**Problem:** The watcher now emits `fs-event` but `FileTree.svelte` never listens for it.

**File:** `apps/desktop/src/lib/components/FileTree.svelte`

In the `<script>` section's `onMount`, add after the initial `load()` call:

```typescript
import { onMount, onDestroy } from 'svelte';
import { listen } from '@tauri-apps/api/event';

let unlisten: (() => void) | undefined;

onMount(async () => {
    await load();
    // Refresh tree on any filesystem change within workspace
    unlisten = await listen('fs-event', () => {
        load();
    });
});

onDestroy(() => {
    unlisten?.();
});
```

**Verification:** Delete or create a file in the workspace folder externally — the tree updates within ~1 second.

---

### Bug 4 — update_project and update_task are not atomic

**Problem:** Both commands fire multiple separate `conn.execute` calls — one per field. If the second call fails (disk full, lock timeout), the row is left in a half-updated state.

**Fix: use COALESCE to update all fields in one statement.**

**File:** `apps/desktop/src-tauri/src/commands/mod.rs`

Replace `update_project`:

```rust
#[tauri::command]
pub fn update_project(
    id: String,
    name: Option<String>,
    path: Option<String>,
    description: Option<String>,
    db: State<Database>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE projects SET
            name        = COALESCE(?1, name),
            path        = COALESCE(?2, path),
            description = COALESCE(?3, description),
            updated_at  = ?4
         WHERE id = ?5",
        rusqlite::params![name, path, description, now, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
```

Replace `update_task`:

```rust
#[tauri::command]
pub fn update_task(
    id: String,
    title: Option<String>,
    description: Option<String>,
    priority: Option<String>,
    db: State<Database>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET
            title       = COALESCE(?1, title),
            description = COALESCE(?2, description),
            priority    = COALESCE(?3, priority),
            updated_at  = ?4
         WHERE id = ?5",
        rusqlite::params![title, description, priority, now, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
```

---

### Bug 5 — Foreign key constraints are silently ignored

**Problem:** SQLite does not enforce `FOREIGN KEY` constraints unless `PRAGMA foreign_keys = ON` is set per-connection. Deleting a node without cascading to edges, or assigning a non-existent `project_id` to a task, silently succeeds.

**File:** `apps/desktop/src-tauri/src/persistence/mod.rs`

In the `initialize` method, prepend this to the `execute_batch` string:

```sql
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
```

> `WAL` mode allows concurrent reads during writes and is significantly faster for desktop apps with multiple panels reading simultaneously.

---

### Bug 6 — Wrong language parsers for YAML, TOML, and Svelte files

**Problem:** In `CodeEditor.svelte`, `.yaml`, `.yml`, `.toml`, and `.svelte` all map to the JavaScript CodeMirror parser. This shows wrong syntax highlighting and wrong indent behaviour.

**File:** `apps/desktop/src/lib/components/CodeEditor.svelte`

Replace the `langLoaders` object:

```typescript
const langLoaders: Record<string, () => Promise<import('@codemirror/language').LanguageSupport>> = {
    '.js':     () => import('@codemirror/lang-javascript').then(m => m.javascript()),
    '.jsx':    () => import('@codemirror/lang-javascript').then(m => m.javascript({ jsx: true })),
    '.ts':     () => import('@codemirror/lang-javascript').then(m => m.javascript({ typescript: true })),
    '.tsx':    () => import('@codemirror/lang-javascript').then(m => m.javascript({ jsx: true, typescript: true })),
    '.css':    () => import('@codemirror/lang-css').then(m => m.css()),
    '.html':   () => import('@codemirror/lang-html').then(m => m.html()),
    '.svelte': () => import('@codemirror/lang-html').then(m => m.html()), // closest match; TODO: lang-svelte
    '.json':   () => import('@codemirror/lang-json').then(m => m.json()),
    '.md':     () => import('@codemirror/lang-markdown').then(m => m.markdown()),
    '.mdx':    () => import('@codemirror/lang-markdown').then(m => m.markdown()),
    '.rs':     () => import('@codemirror/lang-rust').then(m => m.rust()),
    '.py':     () => import('@codemirror/lang-python').then(m => m.python()),
    '.yaml':   () => import('@codemirror/lang-yaml').then(m => m.yaml()),
    '.yml':    () => import('@codemirror/lang-yaml').then(m => m.yaml()),
    '.toml':   () => import('@codemirror/lang-toml').then(m => m.toml()),
    '.xml':    () => import('@codemirror/lang-xml').then(m => m.xml()),
    '.sql':    () => import('@codemirror/lang-sql').then(m => m.sql()),
    '.java':   () => import('@codemirror/lang-java').then(m => m.java()),
    '.cpp':    () => import('@codemirror/lang-cpp').then(m => m.cpp()),
    '.c':      () => import('@codemirror/lang-cpp').then(m => m.cpp()),
    '.go':     () => import('@codemirror/lang-go').then(m => m.go()),
    '.php':    () => import('@codemirror/lang-php').then(m => m.php()),
};
```

Install the new packages:

```bash
cd apps/desktop
npm install @codemirror/lang-yaml @codemirror/lang-toml @codemirror/lang-xml \
            @codemirror/lang-sql @codemirror/lang-java @codemirror/lang-cpp \
            @codemirror/lang-go @codemirror/lang-php
```

**Change the fallback** for unknown extensions from JavaScript to plaintext:

```typescript
async function loadLang(path: string) {
    const ext = '.' + path.split('.').pop()?.toLowerCase();
    const loader = langLoaders[ext];
    if (loader) {
        try { return await loader(); } catch {}
    }
    // Return empty extension list — plain text, no misleading highlighting
    const { EditorState: ES } = await import('@codemirror/state');
    return ES.create({ doc: '' }).facet(import('@codemirror/language').language);
}
```

Actually, the simplest correct fallback is to return no language extension at all. Replace the fallback block with:

```typescript
// No loader found: return an empty array so CodeMirror uses plain text
return [] as any;
```

---

### Bug 7 — Silent save failures give no user feedback

**Problem:** Both the debounced auto-save and the `Ctrl+S` save call `catch {}` with no user feedback. A disk-full or permission error is invisible.

**File:** `apps/desktop/src/lib/components/CodeEditor.svelte`

Add a writable store for the save status at the top of `<script>`:

```typescript
import { writable } from 'svelte/store';
const saveStatus = writable<'saved' | 'saving' | 'error' | 'dirty'>('saved');
```

Replace the `save` function:

```typescript
async function save(content?: string) {
    saveStatus.set('saving');
    try {
        const current = content ?? view?.state.doc.toString();
        if (current !== undefined) {
            await writeFile(filePath, current);
            markDirty(filePath, false);
            saveStatus.set('saved');
        }
    } catch (e) {
        saveStatus.set('error');
        console.error('Save failed:', e);
    }
}
```

In the `EditorView.updateListener` callback, when `update.docChanged` is true, add:
```typescript
saveStatus.set('dirty');
```

Add a status indicator to the template (below `<div bind:this={container}>` or as an overlay):

```svelte
<div class="editor-status" class:error={$saveStatus === 'error'}>
    {#if $saveStatus === 'saving'}⟳ Saving{:else if $saveStatus === 'error'}⚠ Save failed{:else if $saveStatus === 'dirty'}● Unsaved{:else}✓{/if}
</div>

<style>
    .editor-status {
        position: absolute; bottom: 4px; right: 8px;
        font-size: 11px; color: var(--color-muted);
        pointer-events: none; font-family: var(--font-mono);
    }
    .editor-status.error { color: var(--color-error); }
    .editor-container { position: relative; }
</style>
```

---

### Bug 8 — Edge creation requires typing raw UUIDs (completely unusable)

**Problem:** The "Add Edge" form in `GraphPanel.svelte` has two text inputs labelled "Source node ID" and "Target node ID". Node IDs are UUIDs (`f4e3a2b1-...`). No user will type these.

**Fix: replace the UUID text inputs with `<select>` dropdowns populated from the current `nodes` list.**

**File:** `apps/desktop/src/lib/components/GraphPanel.svelte`

Replace the `showNewEdge` form block:

```svelte
{#if showNewEdge}
    <div class="form-overlay">
        <div class="form-row">
            <select class="typo-caption" bind:value={newEdgeSource}>
                <option value="">— Source node —</option>
                {#each nodes as n (n.id)}
                    <option value={n.id}>{n.label} ({n.node_type})</option>
                {/each}
            </select>
            <select class="typo-caption" bind:value={newEdgeTarget}>
                <option value="">— Target node —</option>
                {#each nodes as n (n.id)}
                    <option value={n.id}>{n.label} ({n.node_type})</option>
                {/each}
            </select>
            <select class="typo-caption" bind:value={newEdgeType}>
                <option value="related">Related</option>
                <option value="depends">Depends On</option>
                <option value="contains">Contains</option>
                <option value="blocks">Blocks</option>
                <option value="implements">Implements</option>
            </select>
            <button class="btn-primary typo-caption" onclick={addEdge}>Add</button>
            <button class="btn-secondary typo-caption" onclick={() => (showNewEdge = false)}>✕</button>
        </div>
    </div>
{/if}
```

---

### Bug 9 — Graph layout resets manual node positions on every reload

**Problem:** `load()` in `GraphPanel.svelte` calls `computeGraphLayout` every time, discarding any positions the user has dragged nodes to.

**Fix: only compute layout for nodes that have no known position.**

**File:** `apps/desktop/src/lib/components/GraphPanel.svelte`

Replace the `load` function:

```typescript
async function load() {
    loading = true;
    try {
        const snap = await getGraphSnapshot();
        const prevPositions = new Map(positions); // preserve existing positions
        nodes = snap.nodes;
        edges = snap.edges;

        // Only lay out nodes we haven't positioned yet
        const unpositioned = nodes.filter(n => !prevPositions.has(n.id));
        if (unpositioned.length > 0) {
            const w = canvas?.width || 600;
            const h = canvas?.height || 400;
            const pos = await computeGraphLayout(w, h);
            const newPos = new Map(pos.map((p: NodePosition) => [p.id, { x: p.x, y: p.y }]));
            positions = new Map([...newPos, ...prevPositions]); // prevPositions wins
        } else {
            positions = prevPositions;
        }
    } catch {
        nodes = [];
        edges = [];
    }
    loading = false;
    draw();
}
```

---

### Bug 10 — getFileState uses subscribe anti-pattern

**Problem:** In `apps/desktop/src/lib/stores/editor.ts`, `getFileState` uses `subscribe` to synchronously read a value, leaving a dangling unsubscription if not called correctly.

**File:** `apps/desktop/src/lib/stores/editor.ts`

Replace `getFileState`:

```typescript
import { get } from 'svelte/store';

export function getFileState(path: string): EditorState | undefined {
    return get(fileContents).get(path);
}
```

---

## Section 2 — Performance & Database Improvements

---

### Perf 1 — Add database indexes

**File:** `apps/desktop/src-tauri/src/persistence/mod.rs`

Add to the `execute_batch` string (after the table `CREATE` statements):

```sql
CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_milestones_project_id ON milestones(project_id);
CREATE INDEX IF NOT EXISTS idx_projects_updated_at ON projects(updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_graph_edges_source ON graph_edges(source_id);
CREATE INDEX IF NOT EXISTS idx_graph_edges_target ON graph_edges(target_id);
```

---

### Perf 2 — Add a schema version table for future migrations

Without schema versioning, adding columns or tables later requires manual user intervention.

**File:** `apps/desktop/src-tauri/src/persistence/mod.rs`

Add a version table at the start of `execute_batch`:

```sql
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER NOT NULL
);
INSERT OR IGNORE INTO schema_version (version) VALUES (1);
```

Add a public function:

```rust
pub fn schema_version(conn: &rusqlite::Connection) -> u32 {
    conn.query_row("SELECT version FROM schema_version", [], |r| r.get(0))
        .unwrap_or(0)
}
```

Future migrations are gated: `if schema_version(&conn) < 2 { /* apply migration */ conn.execute("UPDATE schema_version SET version = 2", [])?; }`.

---

### Perf 3 — fileContents Map grows unboundedly

**Problem:** Every opened file is kept in `fileContents` (Svelte store) forever. Opening 200 large files = 200 file contents in memory.

**File:** `apps/desktop/src/lib/stores/editor.ts`

Add an eviction function and call it from `closeTab` in `workspace.ts`:

```typescript
const MAX_CACHED_FILES = 50;

export function evictFileContent(path: string) {
    fileContents.update(map => {
        map.delete(path);
        return new Map(map);
    });
}

export function evictLruIfNeeded() {
    fileContents.update(map => {
        if (map.size > MAX_CACHED_FILES) {
            // Remove oldest entries (Map preserves insertion order)
            const toDelete = [...map.keys()].slice(0, map.size - MAX_CACHED_FILES);
            for (const k of toDelete) map.delete(k);
        }
        return new Map(map);
    });
}
```

**File:** `apps/desktop/src/lib/stores/workspace.ts`

In `closeTab`:

```typescript
import { evictFileContent } from './editor';

export function closeTab(id: string) {
    openTabs.update((tabs) => {
        const tab = tabs.find(t => t.id === id);
        if (tab?.filePath) evictFileContent(tab.filePath); // free memory
        const idx = tabs.findIndex((t) => t.id === id);
        const updated = tabs.filter((t) => t.id !== id);
        const current = get(activeTabId);
        if (current === id && updated.length > 0) {
            activeTabId.set(updated[Math.min(idx, updated.length - 1)].id);
        } else if (updated.length === 0) {
            activeTabId.set(null);
        }
        return updated;
    });
}
```

---

## Section 3 — Missing Core Features (Implement in Order)

---

### Feature 1 — Task ↔ Milestone relationship

**Problem:** Tasks and milestones both have `project_id` but are not linked. Users cannot see which tasks belong to a milestone.

#### Step 3.1.1 — Add milestone_id column to tasks table

**File:** `apps/desktop/src-tauri/src/persistence/mod.rs`

Add to the migration logic (guarded by schema version):

```rust
let conn = self.conn.lock().unwrap();
// ... existing CREATE TABLE ...
// Migration: add milestone_id to tasks
let ver = schema_version(&conn);
if ver < 2 {
    conn.execute_batch(
        "ALTER TABLE tasks ADD COLUMN milestone_id TEXT REFERENCES milestones(id);
         UPDATE schema_version SET version = 2;"
    ).ok(); // ok() because column may already exist
}
```

#### Step 3.1.2 — Add Rust model field

**File:** `apps/desktop/src-tauri/src/models/mod.rs`

```rust
pub struct Task {
    // ... existing fields ...
    pub milestone_id: Option<String>,  // ADD THIS
}
```

#### Step 3.1.3 — Update queries

**File:** `apps/desktop/src-tauri/src/commands/mod.rs`

In `get_tasks` and `get_tasks_by_project`, update the SELECT to include `milestone_id` (column index 8) and add `milestone_id: row.get(8)?` to the struct construction.

Update `create_task` signature to accept `milestone_id: Option<String>` and pass it to INSERT.

Add a new command:

```rust
#[tauri::command]
pub fn assign_task_to_milestone(task_id: String, milestone_id: Option<String>, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET milestone_id = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![milestone_id, now, task_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
```

Register in `lib.rs`: add `commands::assign_task_to_milestone` to `invoke_handler!`.

#### Step 3.1.4 — Update TypeScript types

**File:** `apps/desktop/src/lib/types/index.ts`

```typescript
export interface Task {
    // ... existing fields ...
    milestoneId?: string;  // ADD THIS
}
```

**File:** `apps/desktop/src/lib/services/api.ts`

```typescript
export function assignTaskToMilestone(taskId: string, milestoneId: string | null): Promise<void> {
    return invoke('assign_task_to_milestone', { taskId, milestoneId });
}
```

#### Step 3.1.5 — Add milestone filter to TasksPanel

**File:** `apps/desktop/src/lib/components/TasksPanel.svelte`

Add a milestone filter dropdown at the top of the panel (after fetching milestones via `getMilestones()`). Filter the `tasks` array using `selectedMilestoneId`.

---

### Feature 2 — Integrated terminal / shell panel

This is a solo-developer essential. The bottom panel is scaffolded but empty.

#### Step 3.2.1 — Add Tauri shell plugin

**File:** `apps/desktop/src-tauri/Cargo.toml` — add:

```toml
tauri-plugin-shell = "2"
```

**File:** `apps/desktop/src-tauri/src/lib.rs` — in builder:

```rust
.plugin(tauri_plugin_shell::init())
```

**File:** `apps/desktop/package.json` — add:

```bash
cd apps/desktop && npm install @tauri-apps/plugin-shell
```

#### Step 3.2.2 — Add TerminalPanel component

Create `apps/desktop/src/lib/components/TerminalPanel.svelte`:

```svelte
<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { workspaceRoot } from '$lib/stores/workspace';
    import { get } from 'svelte/store';

    let output = $state<string[]>([]);
    let inputValue = $state('');
    let child: import('@tauri-apps/plugin-shell').Child | null = null;
    let outputEl: HTMLDivElement | undefined;

    async function runCommand() {
        const cmd = inputValue.trim();
        if (!cmd) return;
        inputValue = '';
        output = [...output, `$ ${cmd}`];

        try {
            const { Command } = await import('@tauri-apps/plugin-shell');
            const cwd = get(workspaceRoot) ?? undefined;
            // Use sh -c on unix, cmd /C on windows
            const shell = navigator.userAgent.includes('Win') ? ['cmd', '/C', cmd] : ['sh', '-c', cmd];
            const command = Command.create(shell[0], shell.slice(1), { cwd });

            command.stdout.on('data', (line: string) => {
                output = [...output, line];
                scrollToBottom();
            });
            command.stderr.on('data', (line: string) => {
                output = [...output, `[err] ${line}`];
                scrollToBottom();
            });
            command.on('close', ({ code }) => {
                output = [...output, `[exit ${code}]`];
                child = null;
            });

            child = await command.spawn();
        } catch (e) {
            output = [...output, `Error: ${e}`];
        }
    }

    function scrollToBottom() {
        setTimeout(() => outputEl?.scrollTo({ top: outputEl.scrollHeight }), 10);
    }

    function clearOutput() {
        output = [];
    }

    onDestroy(async () => {
        await child?.kill();
    });
</script>

<div class="terminal-panel">
    <div class="term-toolbar">
        <span class="term-title typo-overline">TERMINAL</span>
        <button class="clear-btn typo-caption" onclick={clearOutput}>Clear</button>
    </div>
    <div class="term-output" bind:this={outputEl}>
        {#each output as line, i (i)}
            <div class="term-line typo-mono" class:command={line.startsWith('$')}>{line}</div>
        {/each}
    </div>
    <div class="term-input-row">
        <span class="prompt typo-mono">$</span>
        <input
            class="term-input typo-mono"
            type="text"
            placeholder="Enter command..."
            bind:value={inputValue}
            onkeydown={(e) => e.key === 'Enter' && runCommand()}
        />
    </div>
</div>

<style>
    .terminal-panel { display: flex; flex-direction: column; height: 100%; background: var(--color-surface-dark); }
    .term-toolbar { display: flex; align-items: center; justify-content: space-between; padding: 4px var(--spacing-3); border-bottom: 1px solid var(--color-surface-dark-border); }
    .term-title { color: var(--color-muted); }
    .clear-btn { background: none; border: none; color: var(--color-muted); cursor: pointer; }
    .clear-btn:hover { color: var(--color-on-dark); }
    .term-output { flex: 1; overflow-y: auto; padding: var(--spacing-2) var(--spacing-3); }
    .term-line { font-size: 12px; color: var(--color-on-dark-soft); white-space: pre-wrap; word-break: break-all; line-height: 1.5; }
    .term-line.command { color: var(--color-primary); }
    .term-input-row { display: flex; align-items: center; gap: var(--spacing-2); padding: var(--spacing-1) var(--spacing-3); border-top: 1px solid var(--color-surface-dark-border); }
    .prompt { color: var(--color-primary); }
    .term-input { flex: 1; background: none; border: none; outline: none; color: var(--color-on-dark); font-size: 13px; }
</style>
```

**File:** `apps/desktop/src-tauri/capabilities/default.json` — add shell permissions:

```json
"tauri:shell:execute"
```

(Exact capability format per Tauri 2 docs — add to the `permissions` array.)

#### Step 3.2.3 — Wire TerminalPanel into BottomPanel

**File:** `apps/desktop/src/lib/components/BottomPanel.svelte`

Import and render `TerminalPanel` inside the bottom panel container.

---

### Feature 3 — Global search (grep-based)

**Problem:** `PanelId` includes `'search'` but the panel shows nothing.

#### Step 3.3.1 — Add Rust search command

**File:** `apps/desktop/src-tauri/src/commands/fs.rs`

```rust
#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchResult {
    pub path: String,
    pub line_number: usize,
    pub line: String,
}

#[tauri::command]
pub fn search_in_files(
    query: String,
    root: State<WorkspaceRoot>,
    case_sensitive: bool,
) -> Result<Vec<SearchResult>, String> {
    let root_guard = root.0.lock().map_err(|e| e.to_string())?;
    let base = root_guard.clone().ok_or("no workspace root set")?;
    drop(root_guard);

    let mut results = Vec::new();
    search_dir(&base, &base, &query, case_sensitive, &mut results, 0)?;
    Ok(results)
}

fn search_dir(
    dir: &std::path::Path,
    base: &std::path::Path,
    query: &str,
    case_sensitive: bool,
    results: &mut Vec<SearchResult>,
    depth: usize,
) -> Result<(), String> {
    if depth > 10 { return Ok(()); } // prevent deep recursion
    let entries = std::fs::read_dir(dir).map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        // Skip hidden dirs and common noise
        if name.starts_with('.') || name == "node_modules" || name == "target" || name == "dist" {
            continue;
        }
        if path.is_dir() {
            let _ = search_dir(&path, base, query, case_sensitive, results, depth + 1);
        } else if let Ok(content) = std::fs::read_to_string(&path) {
            let relative = path.strip_prefix(base).unwrap_or(&path);
            for (i, line) in content.lines().enumerate() {
                let matches = if case_sensitive {
                    line.contains(query)
                } else {
                    line.to_lowercase().contains(&query.to_lowercase())
                };
                if matches {
                    results.push(SearchResult {
                        path: normalize_path(relative),
                        line_number: i + 1,
                        line: line.trim().to_string(),
                    });
                    if results.len() >= 500 { return Ok(()); } // safety cap
                }
            }
        }
    }
    Ok(())
}
```

Register: `commands::fs::search_in_files` in `lib.rs`.

#### Step 3.3.2 — Build SearchPanel component

Create `apps/desktop/src/lib/components/SearchPanel.svelte`:

```svelte
<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { openTab } from '$lib/stores/workspace';

    interface SearchResult { path: string; line_number: number; line: string; }

    let query = $state('');
    let caseSensitive = $state(false);
    let results = $state<SearchResult[]>([]);
    let loading = $state(false);
    let searched = $state(false);

    async function search() {
        if (!query.trim()) return;
        loading = true;
        searched = true;
        try {
            results = await invoke<SearchResult[]>('search_in_files', {
                query: query.trim(), caseSensitive
            });
        } catch { results = []; }
        loading = false;
    }

    function openResult(r: SearchResult) {
        openTab({ id: r.path, title: r.path.split('/').pop() ?? r.path, closable: true, filePath: r.path });
    }
</script>

<div class="search-panel">
    <div class="search-bar">
        <input class="search-input typo-caption" type="text" placeholder="Search in files..."
            bind:value={query} onkeydown={(e) => e.key === 'Enter' && search()} />
        <label class="case-label typo-small">
            <input type="checkbox" bind:checked={caseSensitive} /> Aa
        </label>
        <button class="search-btn typo-caption" onclick={search}>Search</button>
    </div>
    <div class="results">
        {#if loading}
            <div class="empty typo-body">Searching...</div>
        {:else if searched && results.length === 0}
            <div class="empty typo-body">No results</div>
        {:else}
            {#each results as r (r.path + ':' + r.line_number)}
                <button class="result-row" onclick={() => openResult(r)}>
                    <span class="result-path typo-mono">{r.path}:{r.line_number}</span>
                    <span class="result-line typo-small">{r.line}</span>
                </button>
            {/each}
        {/if}
    </div>
</div>

<style>
    .search-panel { display: flex; flex-direction: column; height: 100%; }
    .search-bar { display: flex; gap: var(--spacing-1); padding: var(--spacing-2); border-bottom: 1px solid var(--color-surface-dark-border); }
    .search-input { flex: 1; padding: 4px var(--spacing-2); border: 1px solid var(--color-surface-dark-border); background: var(--color-surface-dark); color: var(--color-on-dark); border-radius: var(--radius-xs); outline: none; }
    .search-input:focus { border-color: var(--color-primary); }
    .case-label { display: flex; align-items: center; gap: 2px; color: var(--color-on-dark-soft); cursor: pointer; font-size: 11px; }
    .search-btn { padding: 4px 10px; border: 1px solid var(--color-surface-dark-border); background: var(--color-surface-dark-elevated); color: var(--color-on-dark); border-radius: var(--radius-xs); cursor: pointer; }
    .results { flex: 1; overflow-y: auto; }
    .result-row { display: flex; flex-direction: column; gap: 2px; width: 100%; padding: 5px var(--spacing-3); border: none; background: none; text-align: left; cursor: pointer; border-bottom: 1px solid var(--color-surface-dark-border); }
    .result-row:hover { background: var(--color-surface-dark-elevated); }
    .result-path { color: var(--color-primary); font-size: 11px; }
    .result-line { color: var(--color-on-dark-soft); font-size: 12px; white-space: pre; overflow: hidden; text-overflow: ellipsis; }
    .empty { padding: var(--spacing-6); text-align: center; color: var(--color-muted); }
</style>
```

Wire into `Sidebar.svelte` for the `'search'` panel case.

---

### Feature 4 — Notes / Scratch Pad panel

**Problem:** `PanelId` defines `'notes'` but nothing renders it.

#### Step 3.4.1 — Add notes persistence command

**File:** `apps/desktop/src-tauri/src/commands/mod.rs`

Add two commands using the existing `workspace_state` table:

```rust
#[tauri::command]
pub fn get_note(note_id: String, db: State<Database>) -> Result<Option<String>, String> {
    get_workspace_state(format!("note:{note_id}"), db)
}

#[tauri::command]
pub fn save_note(note_id: String, content: String, db: State<Database>) -> Result<(), String> {
    set_workspace_state(format!("note:{note_id}"), content, db)
}
```

Register both in `lib.rs`.

#### Step 3.4.2 — Build NotesPanel component

Create `apps/desktop/src/lib/components/NotesPanel.svelte`:

```svelte
<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';

    const NOTE_ID = 'scratch';
    let content = $state('');
    let status = $state<'saved' | 'saving' | 'unsaved'>('saved');
    let saveTimer: ReturnType<typeof setTimeout> | undefined;

    async function load() {
        try {
            const val = await invoke<string | null>('get_note', { noteId: NOTE_ID });
            content = val ?? '';
        } catch {}
    }

    async function save() {
        status = 'saving';
        try {
            await invoke('save_note', { noteId: NOTE_ID, content });
            status = 'saved';
        } catch { status = 'unsaved'; }
    }

    function onInput() {
        status = 'unsaved';
        clearTimeout(saveTimer);
        saveTimer = setTimeout(save, 800);
    }

    load();
</script>

<div class="notes-panel">
    <div class="notes-header">
        <span class="notes-title typo-overline">SCRATCH PAD</span>
        <span class="notes-status typo-small">{status === 'saving' ? '⟳ Saving' : status === 'saved' ? '✓' : '●'}</span>
    </div>
    <textarea
        class="notes-body typo-body"
        placeholder="Write notes, ideas, snippets..."
        bind:value={content}
        oninput={onInput}
    ></textarea>
</div>

<style>
    .notes-panel { display: flex; flex-direction: column; height: 100%; }
    .notes-header { display: flex; justify-content: space-between; align-items: center; padding: 6px var(--spacing-3); border-bottom: 1px solid var(--color-surface-dark-border); }
    .notes-title { color: var(--color-muted); }
    .notes-status { color: var(--color-muted); }
    .notes-body { flex: 1; background: var(--color-surface-dark); color: var(--color-on-dark); border: none; outline: none; resize: none; padding: var(--spacing-3); font-family: var(--font-mono); font-size: 13px; line-height: 1.6; }
</style>
```

Wire into `Sidebar.svelte` for the `'notes'` panel case.

---

### Feature 5 — Project → File Explorer integration

**Problem:** The Projects panel shows a list of projects with their filesystem paths, but clicking a project does nothing — it doesn't open the folder in the file explorer.

**File:** `apps/desktop/src/lib/components/ProjectsPanel.svelte`

Add an "open" button per project card:

```svelte
import { workspaceRoot } from '$lib/stores/workspace';
import { invoke } from '@tauri-apps/api/core';

async function openProject(project: Project) {
    workspaceRoot.set(project.path);
    try {
        await invoke('set_workspace_root', { path: project.path });
    } catch {}
    // Switch sidebar to file explorer
    import { togglePanel } from '$lib/stores/workspace';
    togglePanel('explorer');
}
```

In the project card template, add:

```svelte
<button class="open-project-btn typo-small" onclick={() => openProject(project)} title="Open in Explorer">→</button>
```

---

### Feature 6 — Split editor

**Problem:** `splitMode` and `splitRatio` exist in `editor.ts` store but `MainContent.svelte` never uses them.

**File:** `apps/desktop/src/lib/components/MainContent.svelte`

Import stores and implement split layout:

```svelte
<script lang="ts">
    import { openTabs, activeTabId } from '$lib/stores/workspace';
    import { splitMode, splitRatio } from '$lib/stores/editor';
    import CodeEditor from './CodeEditor.svelte';

    // When split mode is on, show active tab on left, second tab on right
    const leftTab = $derived($openTabs.find(t => t.id === $activeTabId));
    const rightTab = $derived(
        $splitMode ? $openTabs.find(t => t.id !== $activeTabId && t.filePath) : null
    );
</script>

<div class="main-content">
    {#if leftTab?.filePath}
        <div class="editor-pane" style="flex: {$splitRatio}">
            <CodeEditor path={leftTab.filePath} />
        </div>
    {/if}
    {#if $splitMode && rightTab?.filePath}
        <div class="split-divider" role="separator"></div>
        <div class="editor-pane" style="flex: {1 - $splitRatio}">
            <CodeEditor path={rightTab.filePath} />
        </div>
    {/if}
</div>

<style>
    .main-content { display: flex; flex: 1; overflow: hidden; }
    .editor-pane { overflow: hidden; min-width: 0; }
    .split-divider { width: 3px; background: var(--color-surface-dark-border); cursor: col-resize; flex-shrink: 0; }
</style>
```

Add a "Split Editor" command to `CommandPalette.svelte`:

```typescript
import { splitMode } from '$lib/stores/editor';
import { get } from 'svelte/store';

{ id: 'split-editor', label: 'Toggle Split Editor', category: 'editor', icon: '⧉',
  action: () => { splitMode.update(v => !v); onClose(); } },
```

---

### Feature 7 — File right-click context menu (create / delete / rename)

**Problem:** FileTree has no context menu. Users cannot create new files or rename existing ones from the UI.

**File:** `apps/desktop/src/lib/components/FileTree.svelte`

Add context menu state:

```svelte
<script lang="ts">
    let contextMenu = $state<{ x: number; y: number; entry: FileEntry; open: boolean }>({ x: 0, y: 0, entry: null as any, open: false });

    function openContextMenu(e: MouseEvent, entry: FileEntry) {
        e.preventDefault();
        contextMenu = { x: e.clientX, y: e.clientY, entry, open: true };
    }

    async function newFile() {
        const name = prompt('New file name:');
        if (!name) return;
        const path = contextMenu.entry.is_dir
            ? contextMenu.entry.path + '/' + name
            : contextMenu.entry.path.split('/').slice(0, -1).join('/') + '/' + name;
        await invoke('write_file', { path, content: '' });
        contextMenu.open = false;
    }

    async function renameEntry() {
        const newName = prompt('New name:', contextMenu.entry.name);
        if (!newName || newName === contextMenu.entry.name) return;
        const dir = contextMenu.entry.path.split('/').slice(0, -1).join('/');
        await invoke('rename_file', { oldPath: contextMenu.entry.path, newPath: dir + '/' + newName });
        contextMenu.open = false;
    }

    async function deleteEntry() {
        if (!confirm(`Delete "${contextMenu.entry.name}"?`)) return;
        await invoke('delete_file', { path: contextMenu.entry.path });
        contextMenu.open = false;
    }
</script>
```

In the file/folder row template, add `oncontextmenu={(e) => openContextMenu(e, entry)}`.

Add context menu overlay at bottom of template:

```svelte
{#if contextMenu.open}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="ctx-backdrop" onclick={() => (contextMenu.open = false)} onkeydown={() => {}}></div>
    <div class="ctx-menu" style="left: {contextMenu.x}px; top: {contextMenu.y}px;">
        <button onclick={newFile}>New File Here</button>
        <button onclick={renameEntry}>Rename</button>
        <button class="danger" onclick={deleteEntry}>Delete</button>
    </div>
{/if}

<style>
    .ctx-backdrop { position: fixed; inset: 0; z-index: 99; }
    .ctx-menu {
        position: fixed; z-index: 100; background: var(--color-surface-dark-elevated);
        border: 1px solid var(--color-surface-dark-border); border-radius: var(--radius-md);
        padding: 4px; min-width: 160px; box-shadow: 0 4px 16px rgba(0,0,0,0.4);
    }
    .ctx-menu button { display: block; width: 100%; padding: 6px 12px; border: none; background: none; color: var(--color-on-dark); text-align: left; cursor: pointer; border-radius: var(--radius-xs); font-size: 13px; }
    .ctx-menu button:hover { background: var(--color-surface-dark); }
    .ctx-menu button.danger:hover { color: var(--color-error); }
</style>
```

---

### Feature 8 — SDLC Graph auto-population from Projects/Tasks/Milestones

**Problem:** The graph panel is entirely manual. It has no connection to the actual SDLC data already in the DB. Users have to manually add nodes representing things that already exist.

#### Step 3.8.1 — Add a "Sync from SDLC data" command

**File:** `apps/desktop/src-tauri/src/commands/graph.rs`

```rust
#[tauri::command]
pub fn sync_graph_from_sdlc(
    graph: State<GraphState>,
    db: State<Database>,
) -> Result<(), String> {
    use crate::models::{MilestoneStatus, TaskStatus};
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Load all data
    let projects: Vec<crate::models::Project> = {
        let mut s = conn.prepare("SELECT id, name, path, description, created_at, updated_at, tags FROM projects").map_err(|e| e.to_string())?;
        s.query_map([], |r| {
            let tags_str: String = r.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            Ok(crate::models::Project { id: r.get(0)?, name: r.get(1)?, path: r.get(2)?, description: r.get(3)?, created_at: r.get(4)?, updated_at: r.get(5)?, tags })
        }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect()
    };

    let tasks: Vec<crate::models::Task> = {
        let mut s = conn.prepare("SELECT id, title, description, status, priority, created_at, updated_at, project_id FROM tasks").map_err(|e| e.to_string())?;
        s.query_map([], |r| {
            let st: String = r.get(3)?;
            let pr: String = r.get(4)?;
            Ok(crate::models::Task {
                id: r.get(0)?, title: r.get(1)?, description: r.get(2)?,
                status: super::super::commands::parse_status(&st),
                priority: super::super::commands::parse_priority(&pr),
                created_at: r.get(5)?, updated_at: r.get(6)?, project_id: r.get(7)?,
            })
        }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect()
    };

    let milestones: Vec<crate::models::Milestone> = {
        let mut s = conn.prepare("SELECT id, title, description, due_date, status, project_id, created_at, updated_at FROM milestones").map_err(|e| e.to_string())?;
        s.query_map([], |r| {
            let st: String = r.get(4)?;
            Ok(crate::models::Milestone {
                id: r.get(0)?, title: r.get(1)?, description: r.get(2)?,
                due_date: r.get(3)?,
                status: super::super::commands::parse_milestone_status(&st),
                project_id: r.get(5)?, created_at: r.get(6)?, updated_at: r.get(7)?,
            })
        }).map_err(|e| e.to_string())?.filter_map(|r| r.ok()).collect()
    };
    drop(conn);

    let mut g = graph.0.lock().map_err(|e| e.to_string())?;

    // Upsert nodes (add only if not already present by ID)
    for p in &projects {
        if g.get_node(&p.id).is_none() {
            g.add_node(sde_kit_graph::types::GraphNode { id: p.id.clone(), node_type: "project".into(), label: p.name.clone(), metadata: None });
        }
    }
    for m in &milestones {
        if g.get_node(&m.id).is_none() {
            g.add_node(sde_kit_graph::types::GraphNode { id: m.id.clone(), node_type: "milestone".into(), label: m.title.clone(), metadata: None });
        }
        // Add project → milestone edge
        if let Some(pid) = &m.project_id {
            let edge_id = format!("edge-{}-{}", pid, m.id);
            if g.get_node(pid).is_some() {
                // Check edge doesn't exist
                let snap = g.snapshot();
                if !snap.edges.iter().any(|e| e.source_id == *pid && e.target_id == m.id) {
                    g.add_edge(sde_kit_graph::types::GraphEdge { id: edge_id, source_id: pid.clone(), target_id: m.id.clone(), edge_type: "contains".into(), label: None });
                }
            }
        }
    }
    for t in &tasks {
        if g.get_node(&t.id).is_none() {
            g.add_node(sde_kit_graph::types::GraphNode { id: t.id.clone(), node_type: "task".into(), label: t.title.clone(), metadata: None });
        }
        if let Some(pid) = &t.project_id {
            if g.get_node(pid).is_some() {
                let snap = g.snapshot();
                if !snap.edges.iter().any(|e| e.source_id == *pid && e.target_id == t.id) {
                    let edge_id = format!("edge-{}-{}", pid, t.id);
                    g.add_edge(sde_kit_graph::types::GraphEdge { id: edge_id, source_id: pid.clone(), target_id: t.id.clone(), edge_type: "contains".into(), label: None });
                }
            }
        }
    }

    persist(&g, &db)?;
    Ok(())
}
```

Register: `commands::graph::sync_graph_from_sdlc` in `lib.rs`.

#### Step 3.8.2 — Add Sync button to GraphPanel

**File:** `apps/desktop/src/lib/components/GraphPanel.svelte`

Import and add:

```typescript
import { invoke } from '@tauri-apps/api/core';

async function syncFromSdlc() {
    try {
        await invoke('sync_graph_from_sdlc');
        await load();
    } catch (e) {
        console.error('Sync failed:', e);
    }
}
```

In the header actions:

```svelte
<button class="tool-btn typo-caption" onclick={syncFromSdlc} title="Sync from Projects/Tasks/Milestones">⟲SDLC</button>
```

---

## Section 4 — Code Quality & Architecture

---

### Quality 1 — Centralise command registry (remove hardcoded commands from CommandPalette)

**Problem:** Commands are a hardcoded array inside `CommandPalette.svelte`. Adding a command requires editing that component. Instead, commands should be a store.

**Create:** `apps/desktop/src/lib/stores/commands.ts`:

```typescript
import { writable, get } from 'svelte/store';
import type { Command } from '$lib/types';

const registry = writable<Command[]>([]);

export const commands = { subscribe: registry.subscribe };

export function registerCommand(cmd: Command) {
    registry.update(cmds => {
        const existing = cmds.findIndex(c => c.id === cmd.id);
        if (existing >= 0) {
            const updated = [...cmds];
            updated[existing] = cmd;
            return updated;
        }
        return [...cmds, cmd];
    });
}

export function unregisterCommand(id: string) {
    registry.update(cmds => cmds.filter(c => c.id !== id));
}

export function executeCommand(id: string) {
    const cmd = get(registry).find(c => c.id === id);
    cmd?.action();
}
```

In `CommandPalette.svelte`, replace the local `commands` array with `import { commands } from '$lib/stores/commands';` and use `$commands` reactively.

Register built-in commands from `Workspace.svelte` `onMount`:

```typescript
import { registerCommand } from '$lib/stores/commands';

registerCommand({ id: 'toggle-sidebar', label: 'Toggle Sidebar', shortcut: 'Cmd+B', category: 'view', icon: '⊞', action: toggleSidebar });
// ... etc
```

---

### Quality 2 — Error boundary for Tauri invoke failures

All services silently swallow errors with `catch {}`. Create a toast notification store.

**Create:** `apps/desktop/src/lib/stores/notifications.ts`:

```typescript
import { writable } from 'svelte/store';

export type NotifLevel = 'info' | 'warn' | 'error';
export interface Notification { id: string; message: string; level: NotifLevel; }

const store = writable<Notification[]>([]);
export const notifications = { subscribe: store.subscribe };

export function notify(message: string, level: NotifLevel = 'info') {
    const id = crypto.randomUUID();
    store.update(n => [...n, { id, message, level }]);
    setTimeout(() => dismiss(id), level === 'error' ? 8000 : 3000);
}

export function dismiss(id: string) {
    store.update(n => n.filter(x => x.id !== id));
}
```

**Create:** `apps/desktop/src/lib/components/NotificationToast.svelte` and render it in `Workspace.svelte`. Import `notify` in service calls to replace silent `catch {}`.

---

### Quality 3 — Expose `parse_status` and `parse_priority` as pub functions

**Problem:** `sync_graph_from_sdlc` (Feature 8) needs to call `parse_status` and `parse_priority` which are private functions in `commands/mod.rs`.

**File:** `apps/desktop/src-tauri/src/commands/mod.rs`

Change:
```rust
fn parse_status(...)
fn parse_priority(...)
fn parse_milestone_status(...)
```
To:
```rust
pub fn parse_status(...)
pub fn parse_priority(...)
pub fn parse_milestone_status(...)
```

---

### Quality 4 — Type-safe Tauri command errors (replace String errors with enum)

This prevents error messages from being human-readable strings parsed by frontend JS.

**File:** `apps/desktop/src-tauri/src/commands/mod.rs`

At the top, add:

```rust
#[derive(Debug, thiserror::Error, serde::Serialize)]
pub enum CommandError {
    #[error("database error: {0}")] Db(String),
    #[error("not found: {0}")] NotFound(String),
    #[error("invalid input: {0}")] Invalid(String),
}

impl From<rusqlite::Error> for CommandError {
    fn from(e: rusqlite::Error) -> Self { CommandError::Db(e.to_string()) }
}
```

Then replace `Result<T, String>` return types with `Result<T, CommandError>`. Tauri will serialize the enum variant as a structured error object on the frontend.

---

## Section 5 — UX Enhancements

---

### UX 1 — Show unsaved tab indicator in tab bar

**Problem:** Tabs show no visual indicator when a file has unsaved changes. Users cannot tell which files need saving.

**File:** `apps/desktop/src/lib/stores/editor.ts`

The `dirty` field already exists on `EditorState`. Expose a derived store:

```typescript
import { derived } from 'svelte/store';

export const dirtyPaths = derived(fileContents, $fc => {
    return new Set([...$fc.entries()].filter(([_, v]) => v.dirty).map(([k]) => k));
});
```

In the tab bar component, for each tab where `tab.filePath && $dirtyPaths.has(tab.filePath)`, add a `●` indicator before the tab title.

---

### UX 2 — Keyboard shortcut: Ctrl+W closes active tab

**File:** `apps/desktop/src/lib/components/Workspace.svelte` (or `KeyboardShortcuts.svelte`)

```typescript
function handleKeyboard(e: KeyboardEvent) {
    // ... existing shortcuts ...
    if ((e.metaKey || e.ctrlKey) && e.key === 'w') {
        e.preventDefault();
        const id = get(activeTabId);
        if (id) closeTab(id);
    }
    if ((e.metaKey || e.ctrlKey) && e.key === 'Tab') {
        e.preventDefault();
        const tabs = get(openTabs);
        const current = get(activeTabId);
        const idx = tabs.findIndex(t => t.id === current);
        if (tabs.length > 1) {
            const next = e.shiftKey
                ? tabs[(idx - 1 + tabs.length) % tabs.length]
                : tabs[(idx + 1) % tabs.length];
            activeTabId.set(next.id);
        }
    }
}
```

---

### UX 3 — Graph node tooltip on hover

**File:** `apps/desktop/src/lib/components/GraphPanel.svelte`

In `onMouseMove`, check if pointer is over a node and show a tooltip:

```typescript
let tooltip = $state<{ x: number; y: number; node: GraphNode } | null>(null);

function onMouseMove(e: MouseEvent) {
    if (dragNode || isPanning) { /* existing code */ return; }
    const rect = canvas.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;
    let hovered: typeof tooltip = null;
    for (const node of nodes) {
        const pos = positions.get(node.id);
        if (!pos) continue;
        const p = toScreen(pos.x, pos.y);
        if ((mx - p.sx) ** 2 + (my - p.sy) ** 2 < (NODE_RADIUS * zoom + 5) ** 2) {
            hovered = { x: e.clientX, y: e.clientY, node };
            break;
        }
    }
    tooltip = hovered;
    // existing pan code...
}
```

In template (outside canvas, fixed position):

```svelte
{#if tooltip}
    <div class="node-tooltip typo-small" style="left: {tooltip.x + 12}px; top: {tooltip.y - 8}px;">
        <strong>{tooltip.node.label}</strong>
        <span>{tooltip.node.node_type}</span>
    </div>
{/if}

<style>
    .node-tooltip {
        position: fixed; z-index: 200;
        background: var(--color-surface-dark-elevated);
        border: 1px solid var(--color-surface-dark-border);
        border-radius: var(--radius-sm); padding: 4px 8px;
        color: var(--color-on-dark); pointer-events: none;
        display: flex; flex-direction: column; gap: 2px;
    }
</style>
```

---

### UX 4 — Milestone progress bar

**File:** `apps/desktop/src/lib/components/MilestonesPanel.svelte`

For each milestone, show what fraction of its tasks are done. Requires fetching tasks per milestone. Since task-milestone link is added in Feature 1, use it here:

```typescript
async function loadTaskProgress(milestoneId: string): Promise<{ done: number; total: number }> {
    try {
        const all = await getTasks();
        const relevant = all.filter(t => t.milestoneId === milestoneId);
        return { done: relevant.filter(t => t.status === 'done').length, total: relevant.length };
    } catch { return { done: 0, total: 0 }; }
}
```

In milestone card template:

```svelte
{#await loadTaskProgress(ms.id) then progress}
    {#if progress.total > 0}
        <div class="ms-progress">
            <div class="ms-bar" style="width: {Math.round(progress.done / progress.total * 100)}%"></div>
            <span class="ms-progress-label typo-small">{progress.done}/{progress.total} tasks</span>
        </div>
    {/if}
{/await}

<style>
    .ms-progress { margin: 4px 0 0 22px; display: flex; align-items: center; gap: 6px; }
    .ms-bar-track { height: 4px; flex: 1; background: var(--color-surface-dark-border); border-radius: 2px; overflow: hidden; }
    .ms-bar { height: 100%; background: var(--color-success); border-radius: 2px; transition: width 0.3s; }
    .ms-progress-label { color: var(--color-muted); font-size: 11px; }
</style>
```

---

## Section 6 — Build & Developer Experience

---

### DX 1 — Add Rust clippy lint pass to CI scripts

**File:** `scripts/build.sh` — add before the tauri build step:

```bash
echo "→ Rust lints"
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml -- -D warnings
cargo clippy -p sde-kit-graph -- -D warnings
```

---

### DX 2 — Add ESLint + Prettier

```bash
cd apps/desktop
npm install -D eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin \
    eslint-plugin-svelte prettier prettier-plugin-svelte
```

Create `apps/desktop/.eslintrc.json`:

```json
{
  "parser": "@typescript-eslint/parser",
  "plugins": ["@typescript-eslint", "svelte"],
  "extends": ["plugin:@typescript-eslint/recommended", "plugin:svelte/recommended"],
  "rules": {
    "@typescript-eslint/no-explicit-any": "warn",
    "no-console": ["warn", { "allow": ["error", "warn"] }]
  }
}
```

Add to `package.json` scripts:

```json
"lint:js": "eslint src --ext .ts,.svelte",
"format": "prettier --write src"
```

---

### DX 3 — Fix repository field in Cargo.toml

**File:** `apps/desktop/src-tauri/Cargo.toml`

```toml
repository = "https://github.com/pallab-js/SDE-KIT"
```

---

### DX 4 — .gitignore additions

Ensure `.gitignore` (root) contains:

```
target/
apps/desktop/.svelte-kit/
apps/desktop/build/
apps/desktop/node_modules/
*.db
*.db-wal
*.db-shm
```

The SQLite WAL files (`-wal`, `-shm`) must not be committed.

---

## Implementation Order (for AI assistant)

Work exactly in this order to avoid dependency failures:

1. **Bug 2** (watcher fix) — no dependencies
2. **Bug 5** (PRAGMA FK + WAL) — no dependencies  
3. **Bug 4** (atomic updates) — no dependencies
4. **Bug 1** (graph persistence) — depends on Bug 5 being done first (WAL mode helps here)
5. **Quality 3** (pub fn parse_*) — needed by Feature 8
6. **Perf 1** (indexes) — no dependencies
7. **Perf 2** (schema version) — must come before Feature 1's migration
8. **Feature 1** (task-milestone link) — depends on Perf 2
9. **Bug 6** (language parsers) — no dependencies
10. **Bug 7** (save feedback) — no dependencies
11. **Bug 8** (edge creation UX) — no dependencies
12. **Bug 9** (layout preservation) — no dependencies
13. **Bug 10** (getFileState) — no dependencies
14. **Bug 3** (fs-event listener) — depends on Bug 2
15. **Quality 1** (command registry store) — no dependencies
16. **Feature 2** (terminal) — no dependencies on other features
17. **Feature 3** (search) — no dependencies
18. **Feature 4** (notes) — no dependencies
19. **Feature 5** (project → explorer) — no dependencies
20. **Feature 6** (split editor) — no dependencies
21. **Feature 7** (file context menu) — no dependencies
22. **Feature 8** (SDLC graph sync) — depends on Quality 3 and Feature 1
23. **UX 1–4** — all independent, implement in any order
24. **DX 1–4** — implement last

---

## Anti-Hallucination Checklist for AI Implementation

Before generating any code, verify:

- [ ] File path matches one of the paths listed in the **Project Overview** section exactly
- [ ] Any Rust function referenced (e.g. `parse_status`) is marked `pub` before it is called cross-module
- [ ] Any new Tauri command is both defined with `#[tauri::command]` AND listed in `tauri::generate_handler![]` in `lib.rs`
- [ ] Any new Svelte component is imported in the parent component that uses it
- [ ] Any new npm package is installed before it is imported in TypeScript
- [ ] Any new Rust crate dependency is added to `Cargo.toml` before it is used
- [ ] Do not invent Tauri plugin names — only use `tauri-plugin-shell`, `tauri-plugin-dialog`, `tauri-plugin-log` (all already in scope)
- [ ] Do not use `localStorage` for persistent app state — always use the `workspace_state` SQLite table via `get_workspace_state` / `set_workspace_state`
- [ ] Do not add `wasm` or `web` targets — this is a native desktop Tauri app
- [ ] The graph crate is at `crates/graph/` and referenced as `sde-kit-graph` in `Cargo.toml` — do not invent a different name
- [ ] Svelte 5 runes syntax (`$state`, `$derived`, `$effect`, `$props`) is in use — do not use Svelte 4 `$:` reactive syntax or `export let` for props

---

*Document generated by comprehensive static analysis of the SDE-KIT repository at commit HEAD, May 2026.*
