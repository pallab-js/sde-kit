use notify::{Config, Event, EventKind, RecommendedWatcher, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use tauri::{AppHandle, Emitter};

pub fn start_watching(app: AppHandle, path: String) -> Result<(), String> {
    let (tx, rx) = mpsc::channel::<Result<Event, notify::Error>>();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.send(res);
        },
        Config::default(),
    )
    .map_err(|e| format!("failed to create watcher: {e}"))?;

    watcher
        .watch(Path::new(&path), notify::RecursiveMode::Recursive)
        .map_err(|e| format!("failed to watch path: {e}"))?;

    thread::spawn(move || {
        for res in rx {
            match res {
                Ok(event) => {
                    let kind = match event.kind {
                        EventKind::Create(_) => "created",
                        EventKind::Modify(_) => "modified",
                        EventKind::Remove(_) => "removed",
                        _ => continue,
                    };

                    let paths: Vec<String> = event
                        .paths
                        .iter()
                        .map(|p| p.to_string_lossy().to_string())
                        .collect();

                    let _ = app.emit("fs-event", serde_json::json!({
                        "kind": kind,
                        "paths": paths,
                    }));
                }
                Err(e) => {
                    log::error!("watch error: {e}");
                }
            }
        }
    });

    Ok(())
}
