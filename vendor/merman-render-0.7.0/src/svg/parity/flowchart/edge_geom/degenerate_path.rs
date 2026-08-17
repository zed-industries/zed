//! Flowchart edge path normalization for degenerate subgraph-descendant routes.
//!
//! Mermaid flowchart-v2 can emit a degenerate edge path when linking a subgraph to one of its
//! strict descendants (e.g. `Sub --> In` where `In` is declared inside `subgraph Sub`).
//! Upstream renders these as a single-point path (`M..Z`) while preserving the original
//! `data-points`; the path generator now handles the close-path behavior generically, so this
//! helper only collapses the rendered route to the single point that Mermaid keeps after
//! normalization.
//!
//! Owner: flowchart edge geometry parity.
//! Removal criteria: delete this helper when generic edge path normalization can derive the same
//! single-point route without special casing subgraph-to-strict-descendant edges.

use super::*;

pub(in crate::svg::parity::flowchart) fn maybe_collapse_degenerate_subgraph_edge_route(
    ctx: &FlowchartRenderCtx<'_>,
    edge: &crate::flowchart::FlowEdge,
    data_points: &[crate::model::LayoutPoint],
    line_data: &mut Vec<crate::model::LayoutPoint>,
) {
    let edge_is_between_subgraph_and_descendant = (ctx
        .subgraphs_by_id
        .contains_key(edge.from.as_str())
        && flowchart_is_strict_descendant(&ctx.parent, edge.to.as_str(), edge.from.as_str()))
        || (ctx.subgraphs_by_id.contains_key(edge.to.as_str())
            && flowchart_is_strict_descendant(&ctx.parent, edge.from.as_str(), edge.to.as_str()));
    if !edge_is_between_subgraph_and_descendant {
        return;
    }

    let Some(p) = data_points.last() else {
        return;
    };
    line_data.clear();
    line_data.push(crate::model::LayoutPoint {
        x: p.x + 4.0,
        y: p.y,
    });
}
