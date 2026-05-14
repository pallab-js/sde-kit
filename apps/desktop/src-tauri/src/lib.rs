mod commands;
mod models;
mod persistence;
mod watcher;

use commands::fs::WorkspaceRoot;
use commands::graph::GraphState;
use persistence::Database;
use sde_kit_graph::types::Graph;
use std::sync::Mutex;
use tauri::Manager;

fn setup_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            format!("{:?}", info)
        };
        let location = info.location().map(|l| l.to_string()).unwrap_or_else(|| "unknown".into());
        log::error!("PANIC: {} at {}", msg, location);
        let _ = std::fs::write(
            std::env::temp_dir().join("sde-kit-crash.log"),
            format!("SDE Kit Crash Report\nTime: {}\nMessage: {}\nLocation: {}\n",
                chrono::Utc::now().to_rfc3339(), msg, location),
        );
    }));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    setup_panic_hook();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            let db = Database::new(&app_dir).expect("failed to initialize database");
            app.manage(db);
            app.manage(GraphState(Mutex::new(Graph::new())));
            app.manage(WorkspaceRoot(Mutex::new(None)));

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

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_projects,
            commands::create_project,
            commands::update_project,
            commands::delete_project,
            commands::get_tasks,
            commands::get_tasks_by_project,
            commands::create_task,
            commands::update_task,
            commands::update_task_status,
            commands::delete_task,
            commands::get_milestones,
            commands::create_milestone,
            commands::update_milestone_status,
            commands::delete_milestone,
            commands::get_workspace_state,
            commands::set_workspace_state,
            commands::fs::list_directory,
            commands::fs::read_file,
            commands::fs::write_file,
            commands::fs::create_directory,
            commands::fs::delete_file,
            commands::fs::rename_file,
            commands::fs::get_file_info,
            commands::fs::set_workspace_root,
            commands::graph::add_graph_node,
            commands::graph::remove_graph_node,
            commands::graph::add_graph_edge,
            commands::graph::remove_graph_edge,
            commands::graph::get_graph_snapshot,
            commands::graph::clear_graph,
            commands::graph::compute_graph_layout,
            commands::graph::sync_graph_from_sdlc,
            commands::assign_task_to_milestone,
            commands::fs::search_in_files,
            commands::get_note,
            commands::save_note,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
