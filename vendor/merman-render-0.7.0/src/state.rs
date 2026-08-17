//! State diagram (stateDiagram-v2) layout.
//!
//! Baseline: Mermaid@11.12.2.

const STATE_END_NODE_DAGRE_WIDTH_PX_11_12_2: f64 = 14.013_293_266_296_387;

type StateDiagramModel = merman_core::diagrams::state::StateDiagramRenderModel;
type StateNode = merman_core::diagrams::state::StateDiagramRenderNode;

mod config;
mod layout;

pub(crate) use config::{state_html_label_wrapping_width, state_text_style};

pub use layout::{
    debug_build_state_diagram_v2_dagre_graph, debug_extract_state_diagram_v2_cluster_graph,
    layout_state_diagram_v2, layout_state_diagram_v2_typed,
};
