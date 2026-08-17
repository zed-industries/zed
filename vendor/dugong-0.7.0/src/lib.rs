//! Dagre-compatible graph layout algorithms.
//!
//! Baseline: `repo-ref/dagre` (see `tools/upstreams/REPOS.lock.json`).

pub use dugong_graphlib as graphlib;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod data;

mod model;
pub use model::{EdgeLabel, GraphLabel, LabelPos, NodeLabel, Point, RankDir, SelfEdge};

pub mod acyclic;
pub mod add_border_segments;
pub mod coordinate_system;
pub mod greedy_fas;
pub mod nesting_graph;
pub mod normalize;
pub mod order;
pub mod parent_dummy_chains;
pub mod position;
pub mod rank;
pub mod self_edges;
pub mod util;

mod pipeline;
pub use pipeline::layout;
#[cfg(feature = "dagreish")]
pub use pipeline::layout_dagreish;
