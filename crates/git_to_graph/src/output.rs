//! Output structures for git graph rendering.

use super::node::Node;
use serde::{Serialize, Serializer};

/// Main output structure containing nodes and partial paths.
#[derive(Debug)]
pub struct Out {
    pub first_sha: String,
    pub nodes: Vec<Node>,
    pub partial_paths: Vec<PartialPath>,
}

/// Partial path from outside the current page view.
#[derive(Debug, Clone)]
pub struct PartialPath {
    pub points: Vec<(i32, i32, u8)>,
    pub color: String,
}

impl Serialize for PartialPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.points, &self.color).serialize(serializer)
    }
}

/// Row structure for row-based rendering.
#[derive(Debug, Clone)]
pub struct Row {
    pub initial_node: Option<Node>,
    pub x: i32,
    pub color: String,
    pub lines: Vec<RowLine>,
}

impl Serialize for Row {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.x, &self.color, &self.lines).serialize(serializer)
    }
}

/// Line within a row.
#[derive(Debug, Clone)]
pub struct RowLine {
    pub x1: i32,
    pub x2: i32,
    pub typ: i32,
    pub color: String,
}

impl Serialize for RowLine {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.x1, &self.x2, &self.typ, &self.color).serialize(serializer)
    }
}
