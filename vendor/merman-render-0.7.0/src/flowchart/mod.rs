mod label;
mod layout;
mod node;
mod self_loop;
mod style;

pub(crate) type FlowchartV2Model = merman_core::diagrams::flowchart::FlowchartV2Model;
pub(crate) type FlowNode = merman_core::diagrams::flowchart::FlowNode;
pub(crate) type FlowEdge = merman_core::diagrams::flowchart::FlowEdge;
pub(crate) type FlowSubgraph = merman_core::diagrams::flowchart::FlowSubgraph;

pub use layout::{layout_flowchart_v2, layout_flowchart_v2_typed};

pub(crate) use label::{
    FlowchartLabelMetricsRequest, flowchart_decode_label_escapes,
    flowchart_label_metrics_for_layout, flowchart_label_plain_text_for_layout,
    flowchart_normalize_plain_multiline_label_for_html,
    flowchart_whole_label_font_style_requests_italic,
};
pub(crate) use node::flowchart_node_render_dimensions;
pub(crate) use self_loop::{FlowchartSelfLoopEdgeOptions, flowchart_self_loop_helper_edges};
pub(crate) use style::{
    flowchart_effective_font_style_for_classes, flowchart_effective_font_style_for_node_classes,
    flowchart_effective_html_labels, flowchart_effective_node_class_names,
    flowchart_effective_node_html_labels, flowchart_effective_text_style_for_classes,
    flowchart_effective_text_style_for_node_classes, flowchart_html_label_measurement_base_style,
    flowchart_node_has_span_css_height_parity,
};
