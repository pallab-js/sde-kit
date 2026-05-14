mod commands;
mod models;
mod persistence;

use commands::fs::WorkspaceRoot;
use commands::graph::GraphState;
use persistence::Database;
use sde_kit_graph::types::Graph;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
