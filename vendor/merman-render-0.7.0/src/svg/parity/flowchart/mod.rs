use super::*;
use rustc_hash::{FxHashMap, FxHashSet};

mod css;
mod debug_svg;
mod defs;
mod document;
mod edge;
mod edge_geom;
mod hierarchy;
mod label;
mod render;
mod render_config;
mod render_input;
mod style;
mod types;
mod util;
mod viewbox;
mod viewbox_node_bounds;

pub(super) use css::*;
use edge::*;
pub(in crate::svg::parity::flowchart) use edge_geom::{
    FlowchartEdgePathGeomRequest, flowchart_compute_edge_path_geom,
};
use hierarchy::*;
pub(super) use label::*;
pub(super) use style::*;

use render::{
    FlowchartRootRenderSession, render_flowchart_edge_path, render_flowchart_node,
    render_flowchart_root,
};
pub(super) use render::{render_flowchart_cluster, render_flowchart_edge_label};
use types::*;
use util::{OptionalStyleAttr, OptionalStyleXmlAttr};

// Flowchart SVG renderer implementation (split from parity.rs).

// In flowchart SVG emission, many attribute payloads are known to be short-lived (colors, inline
// `d` strings, etc). Avoid allocating an owned `String` for attribute escaping by default.
#[inline]
fn escape_attr(text: &str) -> super::util::EscapeAttrDisplay<'_> {
    escape_attr_display(text)
}

pub(super) fn render_flowchart_v2_debug_svg(
    layout: &FlowchartV2Layout,
    options: &SvgRenderOptions,
) -> String {
    debug_svg::render_flowchart_v2_debug_svg(layout, options)
}

pub(in crate::svg::parity::flowchart) fn flowchart_config_look(
    config: &merman_core::MermaidConfig,
) -> &str {
    flowchart_config_diagram_look(config).as_str()
}

pub(in crate::svg::parity::flowchart) fn flowchart_config_diagram_look(
    config: &merman_core::MermaidConfig,
) -> crate::config::DiagramLook<'_> {
    crate::config::mermaid_config_diagram_look(config)
}

// Entry points (split from parity.rs).

mod svg_emit;

pub(super) fn render_flowchart_v2_svg(
    layout: &FlowchartV2Layout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    svg_emit::render_flowchart_v2_svg(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub(super) fn render_flowchart_v2_svg_model_with_config(
    layout: &FlowchartV2Layout,
    model: &crate::flowchart::FlowchartV2Model,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    svg_emit::render_flowchart_v2_svg_model_with_config(
        layout,
        model,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub(super) fn render_flowchart_v2_svg_with_config(
    layout: &FlowchartV2Layout,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    svg_emit::render_flowchart_v2_svg_with_config(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}
