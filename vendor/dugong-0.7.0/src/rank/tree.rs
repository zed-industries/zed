//! Tree labels used by ranking algorithms.

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeNodeLabel {
    pub low: i32,
    pub lim: i32,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TreeEdgeLabel {
    pub cutvalue: f64,
}
