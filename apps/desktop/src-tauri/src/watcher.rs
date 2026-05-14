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
