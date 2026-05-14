use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String,
}

pub struct WorkspaceRoot(pub Mutex<Option<PathBuf>>);

fn resolve_within_root(root: &Path, requested: &str) -> Result<PathBuf, String> {
    let root_canonical = root.canonicalize().map_err(|_| "invalid workspace root".to_string())?;
    let joined = root_canonical.join(requested);
    if joined.exists() {
        let canonical = joined.canonicalize().map_err(|_| "path does not exist".to_string())?;
        if canonical.starts_with(&root_canonical) {
            return Ok(canonical);
        }
        return Err("path traversal denied".to_string());
    }
    if !joined.starts_with(&root_canonical) {
        return Err("path traversal denied".to_string());
    }
    Ok(joined)
}

fn check_root(path: &str, root: &Option<PathBuf>) -> Result<PathBuf, String> {
    let p = Path::new(path);
    match root {
        Some(r) => resolve_within_root(r, path),
        None => {
            if !p.exists() {
                let parent = p.parent().unwrap_or(p);
                if !parent.exists() {
                    return Err("path does not exist".to_string());
                }
            }
            Ok(p.to_path_buf())
        }
    }
}

fn normalize_path(raw: &Path) -> String {
    raw.to_string_lossy().replace("\\", "/")
}

fn fmt_time(meta: &std::fs::Metadata) -> String {
    let sys = meta.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH);
    let dt: chrono::DateTime<chrono::Utc> = chrono::DateTime::from(sys);
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tauri::command]
pub fn set_workspace_root(path: String, root: State<WorkspaceRoot>) -> Result<(), String> {
    let p = Path::new(&path).canonicalize().map_err(|e| format!("invalid path: {e}"))?;
    if !p.is_dir() {
        return Err("not a directory".to_string());
    }
    let mut state = root.0.lock().map_err(|e| e.to_string())?;
    *state = Some(p);
    Ok(())
}

#[tauri::command]
pub fn list_directory(path: String, root: State<WorkspaceRoot>) -> Result<Vec<FileEntry>, String> {
    let root_guard = root.0.lock().map_err(|e| e.to_string())?;
    let dir = check_root(&path, &root_guard)?;
    drop(root_guard);
    if !dir.is_dir() {
        return Err("not a directory".to_string());
    }

    let mut entries: Vec<FileEntry> = Vec::new();
    let mut read_dir = std::fs::read_dir(&dir).map_err(|e| format!("failed to read directory: {e}"))?;

    while let Some(entry) = read_dir.next().transpose().map_err(|e| e.to_string())? {
        let meta = entry.metadata().map_err(|e| e.to_string())?;
        let file_type = entry.file_type().map_err(|e| e.to_string())?;
        entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: normalize_path(&entry.path()),
            is_dir: file_type.is_dir(),
            size: meta.len(),
            modified: fmt_time(&meta),
        });
    }

    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });

    Ok(entries)
}

#[tauri::command]
pub fn read_file(path: String, root: State<WorkspaceRoot>) -> Result<String, String> {
    let root_guard = root.0.lock().map_err(|e| e.to_string())?;
    let resolved = check_root(&path, &root_guard)?;
    drop(root_guard);
    std::fs::read_to_string(&resolved).map_err(|e| format!("failed to read file: {e}"))
}

#[tauri::command]
pub fn write_file(path: String, content: String, root: State<WorkspaceRoot>) -> Result<(), String> {
    let root_guard = root.0.lock().map_err(|e| e.to_string())?;
    let resolved = check_root(&path, &root_guard)?;
    drop(root_guard);
    std::fs::write(&resolved, &content).map_err(|e| format!("failed to write file: {e}"))
}

#[tauri::command]
pub fn create_directory(path: String, root: State<WorkspaceRoot>) -> Result<(), String> {
    let root_guard = root.0.lock().map_err(|e| e.to_string())?;
    let resolved = check_root(&path, &root_guard)?;
    drop(root_guard);
    std::fs::create_dir_all(&resolved).map_err(|e| format!("failed to create directory: {e}"))
}

#[tauri::command]
pub fn delete_file(path: String, root: State<WorkspaceRoot>) -> Result<(), String> {
    let root_guard = root.0.lock().map_err(|e| e.to_string())?;
    let resolved = check_root(&path, &root_guard)?;
    drop(root_guard);
    if resolved.is_dir() {
        std::fs::remove_dir_all(&resolved).map_err(|e| format!("failed to delete directory: {e}"))
    } else {
        std::fs::remove_file(&resolved).map_err(|e| format!("failed to delete file: {e}"))
    }
}

#[tauri::command]
pub fn rename_file(old_path: String, new_path: String, root: State<WorkspaceRoot>) -> Result<(), String> {
    let root_guard = root.0.lock().map_err(|e| e.to_string())?;
    let old_resolved = check_root(&old_path, &root_guard)?;
    let new_resolved = check_root(&new_path, &root_guard)?;
    drop(root_guard);
    std::fs::rename(&old_resolved, &new_resolved).map_err(|e| format!("failed to rename: {e}"))
}

#[tauri::command]
pub fn get_file_info(path: String, root: State<WorkspaceRoot>) -> Result<FileEntry, String> {
    let root_guard = root.0.lock().map_err(|e| e.to_string())?;
    let resolved = check_root(&path, &root_guard)?;
    drop(root_guard);
    let meta = resolved.metadata().map_err(|e| format!("failed to read metadata: {e}"))?;
    Ok(FileEntry {
        name: resolved.file_name().unwrap_or_default().to_string_lossy().to_string(),
        path: normalize_path(&resolved),
        is_dir: resolved.is_dir(),
        size: meta.len(),
        modified: fmt_time(&meta),
    })
}
