//! Layout algorithms for git graph visualization
//!
//! Assigns lanes/columns to commits to minimize line crossings

use crate::graph::GitGraph;
use collections::HashMap;
use gpui::SharedString;

/// A column in the graph layout
#[derive(Clone, Debug)]
pub struct LayoutColumn {
    /// Commits in this column (by index in ordered_commits)
    pub commit_indices: Vec<usize>,
    /// Color index for this column's line
    pub color_index: usize,
}

/// Graph layout with lane assignments
#[derive(Clone, Debug)]
pub struct GraphLayout {
    /// Lane assignment for each commit SHA
    pub commit_lanes: HashMap<SharedString, usize>,
    /// Column information
    pub columns: Vec<LayoutColumn>,
    /// Total number of lanes needed
    pub lane_count: usize,
    /// Row height in pixels
    pub row_height: f32,
    /// Column width in pixels
    pub column_width: f32,
}

impl Default for GraphLayout {
    fn default() -> Self {
        Self {
            commit_lanes: HashMap::default(),
            columns: Vec::new(),
            lane_count: 1,
            row_height: 24.0,
            column_width: 16.0,
        }
    }
}

impl GraphLayout {
    /// Create a layout from a git graph
    pub fn from_graph(graph: &GitGraph) -> Self {
        let mut layout = Self::default();

        // Copy lane assignments from graph
        for (sha, commit) in &graph.commits {
            layout.commit_lanes.insert(sha.clone(), commit.lane);
        }

        layout.lane_count = graph.lane_count;

        // Build columns
        layout.columns = (0..layout.lane_count)
            .map(|lane| LayoutColumn {
                commit_indices: graph
                    .ordered_commits
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, sha)| {
                        graph.commits.get(sha).and_then(|c| {
                            if c.lane == lane {
                                Some(idx)
                            } else {
                                None
                            }
                        })
                    })
                    .collect(),
                color_index: lane,
            })
            .collect();

        layout
    }

    /// Get the lane for a commit
    pub fn get_lane(&self, sha: &str) -> Option<usize> {
        self.commit_lanes.get(sha).copied()
    }

    /// Calculate X position for a lane
    pub fn lane_x(&self, lane: usize) -> f32 {
        lane as f32 * self.column_width + self.column_width / 2.0
    }

    /// Calculate Y position for a row
    pub fn row_y(&self, row: usize) -> f32 {
        row as f32 * self.row_height + self.row_height / 2.0
    }

    /// Get total width needed for graph lines
    pub fn graph_width(&self) -> f32 {
        self.lane_count as f32 * self.column_width
    }
}

/// Edge between two commits (parent-child relationship)
#[derive(Clone, Debug)]
pub struct GraphEdge {
    /// Source commit row (child)
    pub from_row: usize,
    /// Source lane
    pub from_lane: usize,
    /// Target commit row (parent)
    pub to_row: usize,
    /// Target lane
    pub to_lane: usize,
    /// Edge type
    pub edge_type: EdgeType,
    /// Color index for this edge
    pub color_index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeType {
    /// Straight vertical line (same lane)
    Straight,
    /// Line going left (merge from right)
    MergeLeft,
    /// Line going right (branch off)
    MergeRight,
}

impl GraphLayout {
    /// Calculate all edges for rendering
    pub fn calculate_edges(&self, graph: &GitGraph) -> Vec<GraphEdge> {
        let mut edges = Vec::new();

        for (row, sha) in graph.ordered_commits.iter().enumerate() {
            if let Some(commit) = graph.commits.get(sha) {
                let from_lane = commit.lane;

                for parent_sha in &commit.parent_shas {
                    // Find parent row
                    if let Some(parent_row) = graph
                        .ordered_commits
                        .iter()
                        .position(|s| s == parent_sha)
                    {
                        if let Some(parent) = graph.commits.get(parent_sha) {
                            let to_lane = parent.lane;
                            let edge_type = if from_lane == to_lane {
                                EdgeType::Straight
                            } else if from_lane > to_lane {
                                EdgeType::MergeLeft
                            } else {
                                EdgeType::MergeRight
                            };

                            edges.push(GraphEdge {
                                from_row: row,
                                from_lane,
                                to_row: parent_row,
                                to_lane,
                                edge_type,
                                color_index: from_lane, // Use child's lane color
                            });
                        }
                    }
                }
            }
        }

        edges
    }
}
