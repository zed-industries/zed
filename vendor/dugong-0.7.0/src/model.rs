//! Core Dagre label types and geometry primitives.
//!
//! These are intentionally lightweight and `Clone`-friendly to support deterministic tests and
//! parity-oriented porting from upstream JS.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RankDir {
    #[default]
    TB,
    BT,
    LR,
    RL,
}

#[derive(Debug, Clone)]
pub struct GraphLabel {
    pub rankdir: RankDir,
    pub nodesep: f64,
    pub ranksep: f64,
    pub edgesep: f64,
    pub marginx: f64,
    pub marginy: f64,
    pub width: f64,
    pub height: f64,
    pub align: Option<String>,
    pub ranker: Option<String>,
    pub acyclicer: Option<String>,
    pub dummy_chains: Vec<String>,
    pub nesting_root: Option<String>,
    pub node_rank_factor: Option<usize>,
}

impl Default for GraphLabel {
    fn default() -> Self {
        Self {
            rankdir: RankDir::TB,
            nodesep: 50.0,
            ranksep: 50.0,
            edgesep: 20.0,
            marginx: 0.0,
            marginy: 0.0,
            width: 0.0,
            height: 0.0,
            align: None,
            ranker: None,
            acyclicer: None,
            dummy_chains: Vec::new(),
            nesting_root: None,
            node_rank_factor: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NodeLabel {
    pub width: f64,
    pub height: f64,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub rank: Option<i32>,
    pub order: Option<usize>,
    pub dummy: Option<String>,
    pub labelpos: Option<LabelPos>,
    pub edge_label: Option<EdgeLabel>,
    pub edge_obj: Option<crate::graphlib::EdgeKey>,
    pub min_rank: Option<i32>,
    pub max_rank: Option<i32>,
    pub border_type: Option<String>,
    pub border_left: Vec<Option<String>>,
    pub border_right: Vec<Option<String>>,
    pub border_top: Option<String>,
    pub border_bottom: Option<String>,
    pub self_edges: Vec<SelfEdge>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelPos {
    #[default]
    C,
    L,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeLabel {
    pub width: f64,
    pub height: f64,
    pub labelpos: LabelPos,
    pub labeloffset: f64,
    pub label_rank: Option<i32>,
    pub minlen: usize,
    pub weight: f64,
    pub nesting_edge: bool,
    pub reversed: bool,
    pub forward_name: Option<String>,
    pub extras: std::collections::BTreeMap<String, serde_json::Value>,

    pub x: Option<f64>,
    pub y: Option<f64>,
    pub points: Vec<Point>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelfEdge {
    pub edge_obj: crate::graphlib::EdgeKey,
    pub label: EdgeLabel,
}

impl Default for EdgeLabel {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            labelpos: LabelPos::C,
            labeloffset: 0.0,
            label_rank: None,
            minlen: 1,
            weight: 0.0,
            nesting_edge: false,
            reversed: false,
            forward_name: None,
            extras: std::collections::BTreeMap::new(),
            x: None,
            y: None,
            points: Vec::new(),
        }
    }
}
