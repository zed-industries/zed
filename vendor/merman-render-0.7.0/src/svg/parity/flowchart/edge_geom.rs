//! Flowchart edge geometry helpers.
//!
//! This module is a façade to keep the flowchart renderer organized.

use super::*;

mod basis;
mod boundary;
mod compute;
mod curve_path;
mod cyclic_special;
mod data_points;
mod degenerate_path;
mod fix_corners;
mod intersect;
mod line_with_offset;
mod rect_clip;
mod trace;

pub(super) use basis::maybe_remove_redundant_cluster_run_point;
pub(super) use boundary::{
    BoundaryNode, boundary_for_cluster, boundary_for_node, maybe_normalize_selfedge_loop_points,
};
pub(super) use curve_path::curve_path_d_and_bounds;
pub(super) use cyclic_special::normalize_cyclic_special_data_points;
pub(super) use data_points::{maybe_snap_data_point_to_f32, maybe_truncate_data_point};
pub(super) use degenerate_path::maybe_collapse_degenerate_subgraph_edge_route;
pub(super) use fix_corners::maybe_fix_corners;
pub(super) use intersect::{
    force_intersect_for_layout_shape, intersect_for_layout_shape, is_rounded_intersect_shift_shape,
};
pub(super) use line_with_offset::{
    line_with_offset_for_edge_type, maybe_snap_shallow_basis_triplet_y_to_f32,
    rounded_line_with_marker_offsets_for_edge_type,
};
pub(super) use rect_clip::{cut_path_at_intersect_into, dedup_consecutive_points_into};
pub(super) use trace::{
    FlowchartEdgeTraceInput, TraceEndpointIntersection, tb, tp, write_flowchart_edge_trace,
};

pub(in crate::svg::parity::flowchart) struct FlowchartEdgePathGeomRequest<'a> {
    pub(super) ctx: &'a FlowchartRenderCtx<'a>,
    pub(super) edge: &'a crate::flowchart::FlowEdge,
    pub(super) origin_x: f64,
    pub(super) origin_y: f64,
    pub(super) abs_top_transform: f64,
    pub(super) trace_enabled: bool,
    pub(super) viewbox_current_bounds: Option<(f64, f64, f64, f64)>,
}

pub(in crate::svg::parity::flowchart) fn flowchart_compute_edge_path_geom(
    request: FlowchartEdgePathGeomRequest<'_>,
    scratch: &mut FlowchartEdgeDataPointsScratch,
) -> Option<FlowchartEdgePathGeom> {
    compute::flowchart_compute_edge_path_geom(request, scratch)
}
