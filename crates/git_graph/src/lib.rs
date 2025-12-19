//! Git Graph - Visualize git commit history as a graph
//!
//! This crate provides data structures and rendering logic for visualizing
//! git commit history as a graph similar to VS Code's Git Graph extension.

mod graph;
mod layout;
mod render;

pub use graph::{CommitNode, GitGraph, GraphBranch};
pub use layout::{GraphLayout, LayoutColumn};
pub use render::GitGraphView;
