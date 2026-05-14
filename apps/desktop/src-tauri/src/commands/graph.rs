use sde_kit_graph::layout;
use sde_kit_graph::types::{Graph, GraphEdge, GraphNode, GraphSnapshot, NodePosition};
use std::sync::Mutex;
use tauri::State;

pub struct GraphState(pub Mutex<Graph>);

#[tauri::command]
pub fn add_graph_node(node_type: String, label: String, metadata: Option<serde_json::Value>, graph: State<GraphState>) -> Result<GraphNode, String> {
    let mut g = graph.0.lock().map_err(|e| e.to_string())?;
    let node = GraphNode::new(node_type, label);
    let node = GraphNode { metadata, ..node };
    g.add_node(node.clone());
    Ok(node)
}

#[tauri::command]
pub fn remove_graph_node(id: String, graph: State<GraphState>) -> Result<(), String> {
    let mut g = graph.0.lock().map_err(|e| e.to_string())?;
    g.remove_node(&id).ok_or("node not found")?;
    Ok(())
}

#[tauri::command]
pub fn add_graph_edge(source_id: String, target_id: String, edge_type: String, graph: State<GraphState>) -> Result<GraphEdge, String> {
    let mut g = graph.0.lock().map_err(|e| e.to_string())?;
    if g.get_node(&source_id).is_none() || g.get_node(&target_id).is_none() {
        return Err("source or target node not found".to_string());
    }
    let edge = GraphEdge::new(source_id, target_id, edge_type);
    g.add_edge(edge.clone());
    Ok(edge)
}

#[tauri::command]
pub fn remove_graph_edge(id: String, graph: State<GraphState>) -> Result<(), String> {
    let mut g = graph.0.lock().map_err(|e| e.to_string())?;
    g.remove_edge(&id).ok_or("edge not found")?;
    Ok(())
}

#[tauri::command]
pub fn get_graph_snapshot(graph: State<GraphState>) -> Result<GraphSnapshot, String> {
    let g = graph.0.lock().map_err(|e| e.to_string())?;
    Ok(g.snapshot())
}

#[tauri::command]
pub fn clear_graph(graph: State<GraphState>) -> Result<(), String> {
    let mut g = graph.0.lock().map_err(|e| e.to_string())?;
    g.clear();
    Ok(())
}

#[tauri::command]
pub fn compute_graph_layout(width: f64, height: f64, graph: State<GraphState>) -> Result<Vec<NodePosition>, String> {
    let g = graph.0.lock().map_err(|e| e.to_string())?;
    let snap = g.snapshot();
    if snap.nodes.is_empty() {
        return Ok(vec![]);
    }
    let positions = layout::force_directed(&snap.nodes, &snap.edges, width, height);
    Ok(positions)
}
