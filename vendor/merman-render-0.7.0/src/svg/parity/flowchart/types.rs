//! Shared flowchart rendering types.
//!
//! This module keeps `flowchart.rs` slimmer by housing context structs and scratch buffers that
//! are used across flowchart SVG emission (rendering + viewBox computation).

use super::super::*;
use rustc_hash::{FxHashMap, FxHashSet};

pub(in crate::svg::parity) struct FlowchartRenderCtx<'a> {
    pub(in crate::svg::parity::flowchart) diagram_id: &'a str,
    pub(in crate::svg::parity::flowchart) tx: f64,
    pub(in crate::svg::parity::flowchart) ty: f64,
    pub(in crate::svg::parity::flowchart) measurer: &'a dyn TextMeasurer,
    pub(in crate::svg::parity::flowchart) config: &'a merman_core::MermaidConfig,
    pub(in crate::svg::parity::flowchart) math_renderer:
        Option<&'a (dyn crate::math::MathRenderer + Send + Sync)>,
    pub(in crate::svg::parity::flowchart) icon_registry: Option<&'a crate::svg::IconRegistry>,
    pub(in crate::svg::parity::flowchart) node_html_labels: bool,
    pub(in crate::svg::parity::flowchart) edge_html_labels: bool,
    pub(in crate::svg::parity::flowchart) class_defs: &'a IndexMap<String, Vec<String>>,
    pub(in crate::svg::parity::flowchart) node_border_color: String,
    pub(in crate::svg::parity::flowchart) node_fill_color: String,
    pub(in crate::svg::parity::flowchart) default_edge_interpolate: String,
    pub(in crate::svg::parity::flowchart) default_edge_style: Vec<String>,
    pub(in crate::svg::parity::flowchart) trace_edge_id: Option<String>,
    pub(in crate::svg::parity::flowchart) subgraph_order: Vec<&'a str>,
    pub(in crate::svg::parity::flowchart) edge_order: Vec<&'a str>,
    pub(in crate::svg::parity::flowchart) nodes_by_id:
        FxHashMap<&'a str, &'a crate::flowchart::FlowNode>,
    pub(in crate::svg::parity::flowchart) edges_by_id:
        FxHashMap<&'a str, &'a crate::flowchart::FlowEdge>,
    pub(in crate::svg::parity::flowchart) subgraphs_by_id:
        FxHashMap<&'a str, &'a crate::flowchart::FlowSubgraph>,
    pub(in crate::svg::parity::flowchart) tooltips: &'a FxHashMap<String, String>,
    pub(in crate::svg::parity::flowchart) recursive_clusters: FxHashSet<&'a str>,
    pub(in crate::svg::parity::flowchart) parent: FxHashMap<&'a str, &'a str>,
    pub(in crate::svg::parity::flowchart) layout_nodes_by_id: FxHashMap<&'a str, &'a LayoutNode>,
    pub(in crate::svg::parity::flowchart) layout_edges_by_id:
        FxHashMap<&'a str, &'a crate::model::LayoutEdge>,
    pub(in crate::svg::parity::flowchart) layout_clusters_by_id:
        FxHashMap<&'a str, &'a LayoutCluster>,
    pub(in crate::svg::parity::flowchart) dom_node_order_by_root:
        &'a std::collections::HashMap<String, Vec<String>>,
    pub(in crate::svg::parity::flowchart) node_dom_index: FxHashMap<&'a str, usize>,
    pub(in crate::svg::parity::flowchart) node_padding: f64,
    pub(in crate::svg::parity::flowchart) wrapping_width: f64,
    pub(in crate::svg::parity::flowchart) node_wrap_mode: WrapMode,
    pub(in crate::svg::parity::flowchart) edge_wrap_mode: WrapMode,
    pub(in crate::svg::parity::flowchart) text_style: TextStyle,
    pub(in crate::svg::parity::flowchart) html_label_text_style: TextStyle,
}

#[derive(Debug, Default, Clone)]
pub(in crate::svg::parity::flowchart) struct FlowchartRenderDetails {
    pub(in crate::svg::parity::flowchart) root_calls: u32,
    pub(in crate::svg::parity::flowchart) clusters: web_time::Duration,
    pub(in crate::svg::parity::flowchart) edges_select: web_time::Duration,
    pub(in crate::svg::parity::flowchart) edge_paths: web_time::Duration,
    pub(in crate::svg::parity::flowchart) edge_labels: web_time::Duration,
    pub(in crate::svg::parity::flowchart) dom_order: web_time::Duration,
    pub(in crate::svg::parity::flowchart) nodes: web_time::Duration,
    pub(in crate::svg::parity::flowchart) node_style_compile: web_time::Duration,
    pub(in crate::svg::parity::flowchart) node_roughjs: web_time::Duration,
    pub(in crate::svg::parity::flowchart) node_roughjs_calls: u32,
    pub(in crate::svg::parity::flowchart) node_label_html: web_time::Duration,
    pub(in crate::svg::parity::flowchart) node_label_html_calls: u32,
    pub(in crate::svg::parity::flowchart) nested_roots: web_time::Duration,
    pub(in crate::svg::parity::flowchart) viewbox_edge_curve_lca: web_time::Duration,
    pub(in crate::svg::parity::flowchart) viewbox_edge_curve_offsets: web_time::Duration,
    pub(in crate::svg::parity::flowchart) viewbox_edge_curve_geom: web_time::Duration,
    pub(in crate::svg::parity::flowchart) viewbox_edge_curve_bbox_union: web_time::Duration,
    pub(in crate::svg::parity::flowchart) viewbox_edge_curve_geom_calls: u32,
    pub(in crate::svg::parity::flowchart) viewbox_edge_curve_geom_skipped_bounds: u32,
}

pub(in crate::svg::parity::flowchart) struct FlowchartEdgeDataPointsScratch {
    pub(in crate::svg::parity::flowchart) json: String,
    pub(in crate::svg::parity::flowchart) style_escaped: String,
    pub(in crate::svg::parity::flowchart) ryu: ryu_js::Buffer,
    pub(in crate::svg::parity::flowchart) local_points: Vec<crate::model::LayoutPoint>,
    pub(in crate::svg::parity::flowchart) tmp_points_a: Vec<crate::model::LayoutPoint>,
    pub(in crate::svg::parity::flowchart) tmp_points_b: Vec<crate::model::LayoutPoint>,
    pub(in crate::svg::parity::flowchart) tmp_points_c: Vec<crate::model::LayoutPoint>,
    pub(in crate::svg::parity::flowchart) tmp_points_rev: Vec<crate::model::LayoutPoint>,
}

impl Default for FlowchartEdgeDataPointsScratch {
    fn default() -> Self {
        Self {
            json: String::new(),
            style_escaped: String::new(),
            ryu: ryu_js::Buffer::new(),
            local_points: Vec::new(),
            tmp_points_a: Vec::new(),
            tmp_points_b: Vec::new(),
            tmp_points_c: Vec::new(),
            tmp_points_rev: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::svg::parity::flowchart) struct FlowchartEdgePathGeom {
    pub(in crate::svg::parity::flowchart) d: String,
    pub(in crate::svg::parity::flowchart) pb: Option<path_bounds::SvgPathBounds>,
    pub(in crate::svg::parity::flowchart) data_points_b64: String,
    pub(in crate::svg::parity::flowchart) label_position: Option<crate::model::LayoutPoint>,
    pub(in crate::svg::parity::flowchart) bounds_skipped_for_viewbox: bool,
}

#[derive(Debug, Clone)]
pub(in crate::svg::parity) struct FlowchartEdgePathCacheEntry {
    pub(in crate::svg::parity::flowchart) origin_x: f64,
    pub(in crate::svg::parity::flowchart) origin_y: f64,
    pub(in crate::svg::parity::flowchart) geom: FlowchartEdgePathGeom,
}

#[inline]
pub(in crate::svg::parity::flowchart) fn detail_guard<'a>(
    enabled: bool,
    dst: &'a mut web_time::Duration,
) -> Option<timing::TimingGuard<'a>> {
    enabled.then(|| timing::TimingGuard::new(dst))
}

#[derive(Debug, Clone, Copy)]
pub(in crate::svg::parity::flowchart) struct FlowchartRootOffsets {
    pub(in crate::svg::parity::flowchart) origin_x: f64,
    pub(in crate::svg::parity::flowchart) origin_y: f64,
    pub(in crate::svg::parity::flowchart) abs_top_transform: f64,
}
