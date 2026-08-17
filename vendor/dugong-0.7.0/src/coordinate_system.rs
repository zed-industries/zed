//! Coordinate system adjustment helpers.
//!
//! Dagre internally assumes a top-to-bottom coordinate system. For left-to-right / right-to-left
//! layouts we swap axes and restore them afterwards. This module mirrors upstream's
//! `coordinate-system.js`.

use crate::graphlib::Graph;
use crate::{EdgeLabel, GraphLabel, NodeLabel, RankDir};

pub fn adjust(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    match g.graph().rankdir {
        RankDir::LR | RankDir::RL => swap_width_height(g),
        RankDir::TB | RankDir::BT => {}
    }
}

pub fn undo(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    match g.graph().rankdir {
        RankDir::BT | RankDir::RL => reverse_y(g),
        RankDir::TB | RankDir::LR => {}
    }

    match g.graph().rankdir {
        RankDir::LR | RankDir::RL => {
            swap_xy(g);
            swap_width_height(g);
        }
        RankDir::TB | RankDir::BT => {}
    }
}

fn swap_width_height(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    g.for_each_node_mut(|_id, n| {
        (n.width, n.height) = (n.height, n.width);
    });
    g.for_each_edge_mut(|_ek, e| {
        (e.width, e.height) = (e.height, e.width);
    });

    // Self-loop edges can be temporarily removed and stored on nodes (see `self_edges`).
    // Keep their label box dimensions consistent with the coordinate system transforms.
    g.for_each_node_mut(|_id, n| {
        for se in &mut n.self_edges {
            (se.label.width, se.label.height) = (se.label.height, se.label.width);
        }
    });
}

fn reverse_y(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    g.for_each_node_mut(|_id, n| {
        if let Some(y) = n.y {
            n.y = Some(-y);
        }
    });
    g.for_each_edge_mut(|_ek, e| {
        for p in &mut e.points {
            p.y = -p.y;
        }
        if let Some(y) = e.y {
            e.y = Some(-y);
        }
    });
}

fn swap_xy(g: &mut Graph<NodeLabel, EdgeLabel, GraphLabel>) {
    g.for_each_node_mut(|_id, n| {
        if let (Some(x), Some(y)) = (n.x, n.y) {
            n.x = Some(y);
            n.y = Some(x);
        }
    });
    g.for_each_edge_mut(|_ek, e| {
        for p in &mut e.points {
            (p.x, p.y) = (p.y, p.x);
        }
        if let (Some(x), Some(y)) = (e.x, e.y) {
            e.x = Some(y);
            e.y = Some(x);
        }
    });
}
