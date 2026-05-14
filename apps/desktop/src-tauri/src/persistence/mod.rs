use rusqlite::Connection;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(app_dir: &std::path::Path) -> Result<Self, DbError> {
        std::fs::create_dir_all(app_dir).ok();
        let db_path = app_dir.join("sde-kit.db");
        let conn = Connection::open(db_path)?;
        let db = Database { conn: Mutex::new(conn) };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;

            CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER NOT NULL
            );
            INSERT OR IGNORE INTO schema_version (version) VALUES (1);

            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                path TEXT NOT NULL,
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                tags TEXT NOT NULL DEFAULT '[]'
            );

            CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'todo',
                priority TEXT NOT NULL DEFAULT 'medium',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                project_id TEXT,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

            CREATE TABLE IF NOT EXISTS workspace_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

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

            CREATE TABLE IF NOT EXISTS milestones (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                due_date TEXT,
                status TEXT NOT NULL DEFAULT 'open',
                project_id TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects(id)
            );

            CREATE UNIQUE INDEX IF NOT EXISTS idx_schema_version ON schema_version(version);
            CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id);
            CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
            CREATE INDEX IF NOT EXISTS idx_milestones_project_id ON milestones(project_id);
            CREATE INDEX IF NOT EXISTS idx_projects_updated_at ON projects(updated_at DESC);
            CREATE INDEX IF NOT EXISTS idx_graph_edges_source ON graph_edges(source_id);
            CREATE INDEX IF NOT EXISTS idx_graph_edges_target ON graph_edges(target_id);"
        )?;

        // Migration: add milestone_id to tasks
        let ver = schema_version(&conn);
        if ver < 2 {
            conn.execute_batch(
                "ALTER TABLE tasks ADD COLUMN milestone_id TEXT REFERENCES milestones(id);
                 UPDATE schema_version SET version = 2;"
            ).ok();
        }

        Ok(())
    }
}

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

pub fn schema_version(conn: &rusqlite::Connection) -> u32 {
    conn.query_row("SELECT version FROM schema_version", [], |r| r.get(0))
        .unwrap_or(0)
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
