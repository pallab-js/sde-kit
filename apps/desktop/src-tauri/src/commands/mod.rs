pub mod fs;
pub mod graph;

use crate::models::{Milestone, MilestoneStatus, Project, Task, TaskPriority, TaskStatus};
use crate::persistence::Database;
use chrono::Utc;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

pub fn parse_status(s: &str) -> TaskStatus {
    match s {
        "doing" => TaskStatus::Doing,
        "done" => TaskStatus::Done,
        _ => TaskStatus::Todo,
    }
}

pub fn parse_priority(s: &str) -> TaskPriority {
    match s {
        "low" => TaskPriority::Low,
        "high" => TaskPriority::High,
        _ => TaskPriority::Medium,
    }
}

pub fn parse_milestone_status(s: &str) -> MilestoneStatus {
    match s {
        "closed" => MilestoneStatus::Closed,
        _ => MilestoneStatus::Open,
    }
}

// --- Projects ---

#[tauri::command]
pub fn get_projects(db: State<Database>) -> Result<Vec<Project>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, path, description, created_at, updated_at, tags FROM projects ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let projects = stmt
        .query_map([], |row| {
            let tags_str: String = row.get(6)?;
            let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
            Ok(Project {
                id: row.get(0)?, name: row.get(1)?, path: row.get(2)?,
                description: row.get(3)?, created_at: row.get(4)?,
                updated_at: row.get(5)?, tags,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(projects)
}

#[tauri::command]
pub fn create_project(name: String, path: String, description: Option<String>, db: State<Database>) -> Result<Project, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO projects (id, name, path, description, created_at, updated_at, tags) VALUES (?1, ?2, ?3, ?4, ?5, ?6, '[]')",
        params![id, name, path, description, now, now],
    ).map_err(|e| e.to_string())?;
    Ok(Project { id, name, path, description, created_at: now.clone(), updated_at: now, tags: vec![] })
}

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
        params![name, path, description, now, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_project(id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks WHERE project_id = ?1", params![id]).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM milestones WHERE project_id = ?1", params![id]).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM projects WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

// --- Tasks ---

#[tauri::command]
pub fn get_tasks(db: State<Database>) -> Result<Vec<Task>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, status, priority, created_at, updated_at, project_id, milestone_id FROM tasks ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([], |row| {
            let s: String = row.get(3)?;
            let p: String = row.get(4)?;
            Ok(Task {
                id: row.get(0)?, title: row.get(1)?, description: row.get(2)?,
                status: parse_status(&s), priority: parse_priority(&p),
                created_at: row.get(5)?, updated_at: row.get(6)?, project_id: row.get(7)?,
                milestone_id: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(tasks)
}

#[tauri::command]
pub fn get_tasks_by_project(project_id: String, db: State<Database>) -> Result<Vec<Task>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, status, priority, created_at, updated_at, project_id, milestone_id FROM tasks WHERE project_id = ?1 ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map(params![project_id], |row| {
            let s: String = row.get(3)?;
            let p: String = row.get(4)?;
            Ok(Task {
                id: row.get(0)?, title: row.get(1)?, description: row.get(2)?,
                status: parse_status(&s), priority: parse_priority(&p),
                created_at: row.get(5)?, updated_at: row.get(6)?, project_id: row.get(7)?,
                milestone_id: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(tasks)
}

#[tauri::command]
pub fn create_task(
    title: String, description: Option<String>, priority: Option<String>,
    project_id: Option<String>, milestone_id: Option<String>,
    db: State<Database>,
) -> Result<Task, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let p = priority.unwrap_or_else(|| "medium".to_string());
    conn.execute(
        "INSERT INTO tasks (id, title, description, status, priority, created_at, updated_at, project_id, milestone_id) VALUES (?1, ?2, ?3, 'todo', ?4, ?5, ?6, ?7, ?8)",
        params![id, title, description, p, now, now, project_id, milestone_id],
    ).map_err(|e| e.to_string())?;
    Ok(Task { id, title, description, status: TaskStatus::Todo, priority: parse_priority(&p), created_at: now.clone(), updated_at: now, project_id, milestone_id })
}

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
        params![title, description, priority, now, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_task_status(id: String, status: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute("UPDATE tasks SET status = ?1, updated_at = ?2 WHERE id = ?3", params![status, now, id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_task(id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

// --- Milestones ---

#[tauri::command]
pub fn get_milestones(db: State<Database>) -> Result<Vec<Milestone>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, description, due_date, status, project_id, created_at, updated_at FROM milestones ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let milestones = stmt
        .query_map([], |row| {
            let s: String = row.get(4)?;
            Ok(Milestone {
                id: row.get(0)?, title: row.get(1)?, description: row.get(2)?,
                due_date: row.get(3)?, status: parse_milestone_status(&s),
                project_id: row.get(5)?, created_at: row.get(6)?, updated_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(milestones)
}

#[tauri::command]
pub fn create_milestone(title: String, description: Option<String>, due_date: Option<String>, project_id: Option<String>, db: State<Database>) -> Result<Milestone, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO milestones (id, title, description, due_date, status, project_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, 'open', ?5, ?6, ?7)",
        params![id, title, description, due_date, project_id, now, now],
    ).map_err(|e| e.to_string())?;
    Ok(Milestone { id, title, description, due_date, status: MilestoneStatus::Open, project_id, created_at: now.clone(), updated_at: now })
}

#[tauri::command]
pub fn update_milestone_status(id: String, status: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute("UPDATE milestones SET status = ?1, updated_at = ?2 WHERE id = ?3", params![status, now, id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_milestone(id: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM milestones WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

// --- Task ↔ Milestone ---

#[tauri::command]
pub fn assign_task_to_milestone(task_id: String, milestone_id: Option<String>, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE tasks SET milestone_id = ?1, updated_at = ?2 WHERE id = ?3",
        params![milestone_id, now, task_id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// --- Notes ---

#[tauri::command]
pub fn get_note(note_id: String, db: State<Database>) -> Result<Option<String>, String> {
    get_workspace_state(format!("note:{note_id}"), db)
}

#[tauri::command]
pub fn save_note(note_id: String, content: String, db: State<Database>) -> Result<(), String> {
    set_workspace_state(format!("note:{note_id}"), content, db)
}

// --- Workspace State ---

#[tauri::command]
pub fn get_workspace_state(key: String, db: State<Database>) -> Result<Option<String>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT value FROM workspace_state WHERE key = ?1").map_err(|e| e.to_string())?;
    let result = stmt.query_row(params![key], |row| row.get::<_, String>(0));
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn set_workspace_state(key: String, value: String, db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO workspace_state (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = ?2",
        params![key, value],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
