use crate::types::{GraphEdge, GraphNode, NodePosition};
use std::collections::HashMap;

const REPULSION: f64 = 5000.0;
const ATTRACTION: f64 = 0.01;
const DAMPING: f64 = 0.85;
const ITERATIONS: usize = 100;
const CENTER_GRAVITY: f64 = 0.01;
const MIN_DIST: f64 = 10.0;

pub fn force_directed(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    width: f64,
    height: f64,
) -> Vec<NodePosition> {
    let mut positions: HashMap<String, (f64, f64)> = HashMap::new();
    let mut velocities: HashMap<String, (f64, f64)> = HashMap::new();

    let count = nodes.len() as f64;
    let cx = width / 2.0;
    let cy = height / 2.0;
    let radius = (width.min(height) / 2.0) * 0.6;

    for (i, node) in nodes.iter().enumerate() {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / count;
        let x = cx + radius * angle.cos();
        let y = cy + radius * angle.sin();
        positions.insert(node.id.clone(), (x, y));
        velocities.insert(node.id.clone(), (0.0, 0.0));
    }

    let edge_set: Vec<(String, String)> = edges
        .iter()
        .map(|e| (e.source_id.clone(), e.target_id.clone()))
        .collect();

    for _ in 0..ITERATIONS {
        let mut forces: HashMap<String, (f64, f64)> = HashMap::new();
        for node in nodes {
            forces.insert(node.id.clone(), (0.0, 0.0));
        }

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let a = &nodes[i];
                let b = &nodes[j];
                let (ax, ay) = positions[&a.id];
                let (bx, by) = positions[&b.id];
                let dx = bx - ax;
                let dy = by - ay;
                let dist = dx.hypot(dy).max(MIN_DIST);
                let force = REPULSION / (dist * dist);
                let fx = force * dx / dist;
                let fy = force * dy / dist;
                let f = forces.get_mut(&a.id).unwrap();
                f.0 -= fx;
                f.1 -= fy;
                let f = forces.get_mut(&b.id).unwrap();
                f.0 += fx;
                f.1 += fy;
            }
        }

        for (src, tgt) in &edge_set {
            if let (Some(&sp), Some(&tp)) = (positions.get(src), positions.get(tgt)) {
                let dx = tp.0 - sp.0;
                let dy = tp.1 - sp.1;
                let dist = dx.hypot(dy).max(MIN_DIST);
                let force = ATTRACTION * (dist - 100.0);
                let fx = force * dx / dist;
                let fy = force * dy / dist;
                let f = forces.get_mut(src).unwrap();
                f.0 += fx;
                f.1 += fy;
                let f = forces.get_mut(tgt).unwrap();
                f.0 -= fx;
                f.1 -= fy;
            }
        }

        for node in nodes {
            let (x, y) = positions[&node.id];
            let f = forces.get_mut(&node.id).unwrap();
            f.0 -= (x - cx) * CENTER_GRAVITY;
            f.1 -= (y - cy) * CENTER_GRAVITY;

            let v = velocities.get_mut(&node.id).unwrap();
            v.0 = (v.0 + f.0) * DAMPING;
            v.1 = (v.1 + f.1) * DAMPING;

            let pos = positions.get_mut(&node.id).unwrap();
            pos.0 += v.0;
            pos.1 += v.1;
            pos.0 = pos.0.clamp(20.0, width - 20.0);
            pos.1 = pos.1.clamp(20.0, height - 20.0);
        }
    }

    nodes
        .iter()
        .map(|n| {
            let (x, y) = positions[&n.id];
            NodePosition {
                id: n.id.clone(),
                x,
                y,
            }
        })
        .collect()
}
