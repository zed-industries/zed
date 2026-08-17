use super::icon_registry::IconRegistry;
use super::pipeline::{ScopedCssPostprocessor, SvgPipeline, SvgPostprocessMetadata};
use crate::model::{
    ArchitectureDiagramLayout, BlockDiagramLayout, Bounds, ClassDiagramV2Layout, ErDiagramLayout,
    ErrorDiagramLayout, EventModelingDiagramLayout, FlowchartV2Layout, InfoDiagramLayout,
    IshikawaDiagramLayout, LayoutCluster, LayoutNode, MindmapDiagramLayout, PacketDiagramLayout,
    PieDiagramLayout, QuadrantChartDiagramLayout, RadarDiagramLayout, RequirementDiagramLayout,
    SankeyDiagramLayout, SequenceDiagramLayout, StateDiagramV2Layout, TimelineDiagramLayout,
    TreeViewDiagramLayout, VennDiagramLayout, XyChartDiagramLayout,
};
use crate::text::{TextMeasurer, TextStyle, WrapMode};
use crate::{Error, Result};
use base64::Engine as _;
use indexmap::IndexMap;
use std::fmt::Write as _;

mod architecture;
mod block;
mod c4;
mod class;
mod css;
mod curve;
mod emitted_bounds;
mod er;
mod error;
mod eventmodeling;
mod flowchart;
mod gantt;
mod gitgraph;
mod info;
mod ishikawa;
mod journey;
mod kanban;
mod layout_debug;
mod mindmap;
mod packet;
mod path_bounds;
mod pie;
mod quadrantchart;
mod radar;
mod requirement;
mod root_svg;
mod roughjs_common;
mod sankey;
mod sequence;
mod state;
mod style;
pub(crate) mod theme;
mod timeline;
mod timing;
mod tree_view;
mod treemap;
mod util;
mod venn;
mod xychart;
use crate::math::MathRenderer;
use css::{
    er_css, gantt_css, info_css_parts_with_config, info_css_parts_with_theme_font_size_only,
    info_css_with_config, pie_css, push_xychart_css, requirement_css, sankey_css, treemap_css,
};
use path_bounds::svg_path_bounds_from_d;
pub(crate) fn mindmap_cloud_rendered_bbox_size_px(w: f64, h: f64) -> Option<(f64, f64)> {
    mindmap::mindmap_cloud_rendered_bbox_size_px(w, h)
}
pub use emitted_bounds::{
    SvgEmittedBoundsContributor, SvgEmittedBoundsDebug, debug_svg_emitted_bounds,
};
use emitted_bounds::{svg_emitted_bounds_from_svg, svg_emitted_bounds_from_svg_inner};
use state::{roughjs_ops_to_svg_path_d, roughjs_parse_hex_color_to_srgba, roughjs_paths_for_rect};
use style::{is_rect_style_key, is_text_style_key, parse_style_decl};
use theme::PresentationTheme;
use util::{
    SvgTheme, apply_root_viewport_override, config_bool, config_diagram_look, config_f64,
    config_f64_css_px, config_string, css_rgba_fade, decode_mermaid_entities_for_render_text,
    escape_attr, escape_attr_display, escape_attr_into, escape_xml, escape_xml_display,
    escape_xml_into, fmt, fmt_debug_3dp, fmt_display, fmt_into, fmt_max_width_px, fmt_path,
    fmt_path_into, fmt_points, fmt_string, json_stringify_points, json_stringify_points_into,
    normalize_css_font_family, push_points_attr, scoped_svg_id, scoped_svg_url, theme_color,
};

const MERMAID_SEQUENCE_BASE_DEFS_11_12_2: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/sequence_base_defs_11_12_2.svgfrag"
));

#[derive(Debug, Clone)]
pub struct SvgRenderOptions {
    /// Adds extra space around the computed viewBox.
    pub viewbox_padding: f64,
    /// Optional diagram id used for Mermaid-like marker ids.
    pub diagram_id: Option<String>,
    /// Optional override for the root SVG `aria-roledescription` attribute.
    ///
    /// This is primarily used to reproduce Mermaid's per-header accessibility metadata quirks
    /// (e.g. `classDiagram-v2` differs from `classDiagram` at Mermaid 11.12.2).
    pub aria_roledescription: Option<String>,
    /// When true, include edge polylines.
    pub include_edges: bool,
    /// When true, include node bounding boxes and ids.
    pub include_nodes: bool,
    /// When true, include cluster bounding boxes and titles.
    pub include_clusters: bool,
    /// When true, draw markers that visualize Mermaid cluster positioning metadata.
    pub include_cluster_debug_markers: bool,
    /// When true, label edge routes with edge ids.
    pub include_edge_id_labels: bool,
    /// Optional override for "current time" used by diagrams that render time-dependent markers
    /// (e.g. Gantt `today` line). This exists to make parity/golden comparisons reproducible.
    pub now_ms_override: Option<i64>,
    /// Optional math renderer for `$$...$$` style labels.
    pub math_renderer: Option<std::sync::Arc<dyn MathRenderer + Send + Sync>>,
    /// Optional Iconify-compatible registry used by icon-capable renderers.
    pub icon_registry: Option<std::sync::Arc<IconRegistry>>,
    /// When false, renderers that support root viewport override lookup emit computed bounds.
    pub apply_root_overrides: bool,
}

impl Default for SvgRenderOptions {
    fn default() -> Self {
        Self {
            viewbox_padding: 8.0,
            diagram_id: None,
            aria_roledescription: None,
            include_edges: true,
            include_nodes: true,
            include_clusters: true,
            include_cluster_debug_markers: false,
            include_edge_id_labels: false,
            now_ms_override: None,
            math_renderer: None,
            icon_registry: None,
            apply_root_overrides: true,
        }
    }
}

pub fn render_layouted_svg(
    diagram: &crate::model::LayoutedDiagram,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    render_layout_svg_parts(
        &diagram.layout,
        &diagram.semantic,
        &diagram.meta.effective_config,
        diagram.meta.title.as_deref(),
        measurer,
        options,
    )
}

pub fn render_layout_svg_parts(
    layout: &crate::model::LayoutDiagram,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let svg =
        render_layout_svg_parts_raw(layout, semantic, effective_config, title, measurer, options)?;
    apply_theme_css(svg, effective_config)
}

fn render_layout_svg_parts_raw(
    layout: &crate::model::LayoutDiagram,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    use crate::model::LayoutDiagram;

    match layout {
        LayoutDiagram::ErrorDiagram(layout) => {
            render_error_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::BlockDiagram(layout) => {
            render_block_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::RequirementDiagram(layout) => {
            render_requirement_diagram_svg(layout, semantic, effective_config, title, options)
        }
        LayoutDiagram::ArchitectureDiagram(layout) => {
            render_architecture_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::MindmapDiagram(layout) => {
            render_mindmap_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::SankeyDiagram(layout) => {
            render_sankey_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::RadarDiagram(layout) => {
            render_radar_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::TreemapDiagram(layout) => {
            render_treemap_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::VennDiagram(layout) => {
            render_venn_diagram_svg(layout, semantic, effective_config, title, options)
        }
        LayoutDiagram::XyChartDiagram(layout) => {
            render_xychart_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::QuadrantChartDiagram(layout) => {
            render_quadrantchart_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::FlowchartV2(layout) => {
            render_flowchart_v2_svg(layout, semantic, effective_config, title, measurer, options)
        }
        LayoutDiagram::StateDiagramV2(layout) => render_state_diagram_v2_svg(
            layout,
            semantic,
            effective_config,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::ClassDiagramV2(layout) => render_class_diagram_v2_svg(
            layout,
            semantic,
            effective_config,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::ErDiagram(layout) => {
            render_er_diagram_svg(layout, semantic, effective_config, title, measurer, options)
        }
        LayoutDiagram::SequenceDiagram(layout) => render_sequence_diagram_svg(
            layout,
            semantic,
            effective_config,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::InfoDiagram(layout) => {
            render_info_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::PacketDiagram(layout) => {
            render_packet_diagram_svg(layout, semantic, effective_config, title, options)
        }
        LayoutDiagram::TimelineDiagram(layout) => render_timeline_diagram_svg(
            layout,
            semantic,
            effective_config,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::PieDiagram(layout) => {
            render_pie_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::JourneyDiagram(layout) => {
            render_journey_diagram_svg(layout, semantic, effective_config, title, measurer, options)
        }
        LayoutDiagram::KanbanDiagram(layout) => {
            render_kanban_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::GitGraphDiagram(layout) => render_gitgraph_diagram_svg(
            layout,
            semantic,
            effective_config,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::GanttDiagram(layout) => {
            render_gantt_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::TreeViewDiagram(layout) => {
            render_tree_view_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::IshikawaDiagram(layout) => {
            render_ishikawa_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::EventModelingDiagram(layout) => {
            render_eventmodeling_diagram_svg(layout, semantic, effective_config, options)
        }
        LayoutDiagram::C4Diagram(layout) => {
            render_c4_diagram_svg(layout, semantic, effective_config, title, measurer, options)
        }
    }
}

pub fn render_layout_svg_parts_with_config(
    layout: &crate::model::LayoutDiagram,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let svg = render_layout_svg_parts_with_config_raw(
        layout,
        semantic,
        effective_config,
        title,
        measurer,
        options,
    )?;
    apply_theme_css(svg, effective_config.as_value())
}

fn render_layout_svg_parts_with_config_raw(
    layout: &crate::model::LayoutDiagram,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    use crate::model::LayoutDiagram;

    let effective_config_value = effective_config.as_value();

    match layout {
        LayoutDiagram::ErrorDiagram(layout) => {
            render_error_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::BlockDiagram(layout) => {
            render_block_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::RequirementDiagram(layout) => {
            render_requirement_diagram_svg(layout, semantic, effective_config_value, title, options)
        }
        LayoutDiagram::ArchitectureDiagram(layout) => {
            architecture::render_architecture_diagram_svg_with_config(
                layout,
                semantic,
                effective_config,
                options,
            )
        }
        LayoutDiagram::MindmapDiagram(layout) => {
            render_mindmap_diagram_svg_with_config(layout, semantic, effective_config, options)
        }
        LayoutDiagram::SankeyDiagram(layout) => {
            render_sankey_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::RadarDiagram(layout) => {
            render_radar_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::TreemapDiagram(layout) => {
            render_treemap_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::VennDiagram(layout) => {
            render_venn_diagram_svg(layout, semantic, effective_config_value, title, options)
        }
        LayoutDiagram::XyChartDiagram(layout) => {
            render_xychart_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::QuadrantChartDiagram(layout) => {
            render_quadrantchart_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::FlowchartV2(layout) => render_flowchart_v2_svg_with_config(
            layout,
            semantic,
            effective_config,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::StateDiagramV2(layout) => render_state_diagram_v2_svg(
            layout,
            semantic,
            effective_config_value,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::ClassDiagramV2(layout) => {
            let model = crate::json::from_value_ref(semantic)?;
            class::render_class_diagram_v2_svg_model_with_config(
                layout,
                &model,
                effective_config,
                title,
                measurer,
                options,
            )
        }
        LayoutDiagram::ErDiagram(layout) => render_er_diagram_svg(
            layout,
            semantic,
            effective_config_value,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::SequenceDiagram(layout) => {
            sequence::render_sequence_diagram_svg_with_config(
                layout,
                semantic,
                effective_config,
                title,
                measurer,
                options,
            )
        }
        LayoutDiagram::InfoDiagram(layout) => {
            render_info_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::PacketDiagram(layout) => {
            render_packet_diagram_svg(layout, semantic, effective_config_value, title, options)
        }
        LayoutDiagram::TimelineDiagram(layout) => render_timeline_diagram_svg(
            layout,
            semantic,
            effective_config_value,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::PieDiagram(layout) => {
            render_pie_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::JourneyDiagram(layout) => render_journey_diagram_svg(
            layout,
            semantic,
            effective_config_value,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::KanbanDiagram(layout) => {
            render_kanban_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::GitGraphDiagram(layout) => render_gitgraph_diagram_svg(
            layout,
            semantic,
            effective_config_value,
            title,
            measurer,
            options,
        ),
        LayoutDiagram::GanttDiagram(layout) => {
            render_gantt_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::TreeViewDiagram(layout) => {
            render_tree_view_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::IshikawaDiagram(layout) => {
            render_ishikawa_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::EventModelingDiagram(layout) => {
            render_eventmodeling_diagram_svg(layout, semantic, effective_config_value, options)
        }
        LayoutDiagram::C4Diagram(layout) => render_c4_diagram_svg(
            layout,
            semantic,
            effective_config_value,
            title,
            measurer,
            options,
        ),
    }
}

pub fn render_layout_svg_parts_for_render_model_with_config(
    layout: &crate::model::LayoutDiagram,
    semantic: &merman_core::RenderSemanticModel,
    effective_config: &merman_core::MermaidConfig,
    title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let svg = render_layout_svg_parts_for_render_model_with_config_raw(
        layout,
        semantic,
        effective_config,
        title,
        measurer,
        options,
    )?;
    apply_theme_css(svg, effective_config.as_value())
}

fn render_layout_svg_parts_for_render_model_with_config_raw(
    layout: &crate::model::LayoutDiagram,
    semantic: &merman_core::RenderSemanticModel,
    effective_config: &merman_core::MermaidConfig,
    title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    use crate::model::LayoutDiagram;
    use merman_core::RenderSemanticModel;

    match (layout, semantic) {
        (LayoutDiagram::ArchitectureDiagram(layout), RenderSemanticModel::Architecture(model)) => {
            architecture::render_architecture_diagram_svg_typed_with_config(
                layout,
                model,
                effective_config,
                options,
            )
        }
        (LayoutDiagram::FlowchartV2(layout), RenderSemanticModel::Flowchart(model)) => {
            render_flowchart_v2_svg_model_with_config(
                layout,
                model,
                effective_config,
                title,
                measurer,
                options,
            )
        }
        (LayoutDiagram::MindmapDiagram(layout), RenderSemanticModel::Mindmap(model)) => {
            mindmap::render_mindmap_diagram_svg_model_with_config(
                layout,
                model,
                effective_config,
                options,
            )
        }
        (LayoutDiagram::StateDiagramV2(layout), RenderSemanticModel::State(model)) => {
            state::render_state_diagram_v2_svg_model(
                layout,
                model,
                effective_config.as_value(),
                title,
                measurer,
                options,
            )
        }
        (LayoutDiagram::ClassDiagramV2(layout), RenderSemanticModel::Class(model)) => {
            class::render_class_diagram_v2_svg_model_with_config(
                layout,
                model,
                effective_config,
                title,
                measurer,
                options,
            )
        }
        (LayoutDiagram::SequenceDiagram(layout), RenderSemanticModel::Sequence(model)) => {
            sequence::render_sequence_diagram_svg_model_with_config(
                layout,
                model,
                effective_config,
                title,
                measurer,
                options,
            )
        }
        (LayoutDiagram::KanbanDiagram(layout), RenderSemanticModel::Kanban(_)) => {
            render_kanban_diagram_svg(
                layout,
                &serde_json::Value::Null,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::GanttDiagram(layout), RenderSemanticModel::Gantt(model)) => {
            gantt::render_gantt_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::PieDiagram(layout), RenderSemanticModel::Pie(model)) => {
            pie::render_pie_diagram_svg_model(layout, model, effective_config.as_value(), options)
        }
        (LayoutDiagram::PacketDiagram(layout), RenderSemanticModel::Packet(model)) => {
            packet::render_packet_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                title,
                options,
            )
        }
        (LayoutDiagram::TimelineDiagram(layout), RenderSemanticModel::Timeline(model)) => {
            timeline::render_timeline_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                title,
                measurer,
                options,
            )
        }
        (LayoutDiagram::JourneyDiagram(layout), RenderSemanticModel::Journey(model)) => {
            journey::render_journey_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                title,
                measurer,
                options,
            )
        }
        (LayoutDiagram::RequirementDiagram(layout), RenderSemanticModel::Requirement(model)) => {
            requirement::render_requirement_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                title,
                options,
            )
        }
        (LayoutDiagram::SankeyDiagram(layout), RenderSemanticModel::Sankey(_)) => {
            render_sankey_diagram_svg(
                layout,
                &serde_json::Value::Null,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::RadarDiagram(layout), RenderSemanticModel::Radar(model)) => {
            radar::render_radar_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::InfoDiagram(layout), RenderSemanticModel::Info(_)) => {
            render_info_diagram_svg(
                layout,
                &serde_json::Value::Null,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::TreemapDiagram(layout), RenderSemanticModel::Treemap(_)) => {
            render_treemap_diagram_svg(
                layout,
                &serde_json::Value::Null,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::VennDiagram(layout), RenderSemanticModel::Venn(model)) => {
            venn::render_venn_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                title,
                options,
            )
        }
        (LayoutDiagram::BlockDiagram(layout), RenderSemanticModel::Block(model)) => {
            render_block_diagram_svg_model(layout, model, effective_config.as_value(), options)
        }
        (LayoutDiagram::ErDiagram(layout), RenderSemanticModel::Er(model)) => {
            render_er_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                title,
                measurer,
                options,
            )
        }
        (LayoutDiagram::QuadrantChartDiagram(layout), RenderSemanticModel::QuadrantChart(_)) => {
            render_quadrantchart_diagram_svg(
                layout,
                &serde_json::Value::Null,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::XyChartDiagram(layout), RenderSemanticModel::XyChart(_)) => {
            render_xychart_diagram_svg(
                layout,
                &serde_json::Value::Null,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::GitGraphDiagram(layout), RenderSemanticModel::GitGraph(model)) => {
            gitgraph::render_gitgraph_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                title,
                measurer,
                options,
            )
        }
        (LayoutDiagram::TreeViewDiagram(layout), RenderSemanticModel::TreeView(model)) => {
            tree_view::render_tree_view_diagram_svg_model(
                layout,
                model,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::IshikawaDiagram(layout), RenderSemanticModel::Ishikawa(_)) => {
            render_ishikawa_diagram_svg(
                layout,
                &serde_json::Value::Null,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::EventModelingDiagram(layout), RenderSemanticModel::EventModeling(_)) => {
            render_eventmodeling_diagram_svg(
                layout,
                &serde_json::Value::Null,
                effective_config.as_value(),
                options,
            )
        }
        (LayoutDiagram::C4Diagram(layout), RenderSemanticModel::C4(model)) => {
            c4::render_c4_diagram_svg_typed(
                layout,
                model,
                effective_config.as_value(),
                title,
                measurer,
                options,
            )
        }
        (_, RenderSemanticModel::Json(semantic)) => render_layout_svg_parts_with_config_raw(
            layout,
            semantic,
            effective_config,
            title,
            measurer,
            options,
        ),
        _ => Err(Error::InvalidModel {
            message: "semantic model does not match layout diagram type".to_string(),
        }),
    }
}

fn apply_theme_css(svg: String, effective_config: &serde_json::Value) -> Result<String> {
    let Some(theme_css) = effective_config
        .get("themeCSS")
        .and_then(serde_json::Value::as_str)
        .filter(|css| !css.trim().is_empty())
    else {
        return Ok(svg);
    };

    let metadata = SvgPostprocessMetadata::from_svg(&svg);
    let pipeline = SvgPipeline::parity().with_postprocessor(ScopedCssPostprocessor::new(theme_css));
    pipeline.process_to_string_with_metadata(&svg, &metadata)
}

pub fn render_flowchart_v2_debug_svg(
    layout: &FlowchartV2Layout,
    options: &SvgRenderOptions,
) -> String {
    flowchart::render_flowchart_v2_debug_svg(layout, options)
}

pub fn render_sequence_diagram_debug_svg(
    layout: &SequenceDiagramLayout,
    options: &SvgRenderOptions,
) -> String {
    sequence::render_sequence_diagram_debug_svg(layout, options)
}

pub fn render_sequence_diagram_svg(
    layout: &SequenceDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    sequence::render_sequence_diagram_svg(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_error_diagram_svg(
    layout: &ErrorDiagramLayout,
    _semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    error::render_error_diagram_svg(layout, _semantic, _effective_config, options)
}

pub fn render_info_diagram_svg(
    layout: &InfoDiagramLayout,
    _semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    info::render_info_diagram_svg(layout, _semantic, _effective_config, options)
}

pub fn render_pie_diagram_svg(
    layout: &PieDiagramLayout,
    semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    pie::render_pie_diagram_svg(layout, semantic, _effective_config, options)
}

pub fn render_requirement_diagram_svg(
    layout: &RequirementDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    options: &SvgRenderOptions,
) -> Result<String> {
    requirement::render_requirement_diagram_svg(
        layout,
        semantic,
        effective_config,
        diagram_title,
        options,
    )
}

pub fn render_block_diagram_svg(
    layout: &BlockDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    block::render_block_diagram_svg(layout, semantic, effective_config, options)
}

pub fn render_block_diagram_svg_model(
    layout: &BlockDiagramLayout,
    model: &merman_core::diagrams::block::BlockDiagramRenderModel,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    block::render_block_diagram_svg_model(layout, model, effective_config, options)
}

pub fn render_er_diagram_svg_model(
    layout: &ErDiagramLayout,
    model: &merman_core::diagrams::er::ErDiagramRenderModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    er::render_er_diagram_svg_model(
        layout,
        model,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_radar_diagram_svg(
    layout: &RadarDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    radar::render_radar_diagram_svg(layout, semantic, effective_config, options)
}

pub fn render_quadrantchart_diagram_svg(
    layout: &QuadrantChartDiagramLayout,
    _semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    quadrantchart::render_quadrantchart_diagram_svg(layout, _semantic, _effective_config, options)
}

pub fn render_xychart_diagram_svg(
    layout: &XyChartDiagramLayout,
    _semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    xychart::render_xychart_diagram_svg(layout, _semantic, _effective_config, options)
}

pub fn render_treemap_diagram_svg(
    layout: &crate::model::TreemapDiagramLayout,
    _semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    treemap::render_treemap_diagram_svg(layout, _semantic, effective_config, options)
}

pub fn render_venn_diagram_svg(
    layout: &VennDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    options: &SvgRenderOptions,
) -> Result<String> {
    venn::render_venn_diagram_svg(layout, semantic, effective_config, diagram_title, options)
}

pub fn render_venn_diagram_svg_model(
    layout: &VennDiagramLayout,
    model: &merman_core::diagrams::venn::VennDiagramRenderModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    options: &SvgRenderOptions,
) -> Result<String> {
    venn::render_venn_diagram_svg_model(layout, model, effective_config, diagram_title, options)
}

pub fn render_packet_diagram_svg(
    layout: &PacketDiagramLayout,
    semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    options: &SvgRenderOptions,
) -> Result<String> {
    packet::render_packet_diagram_svg(layout, semantic, _effective_config, diagram_title, options)
}

pub fn render_timeline_diagram_svg(
    layout: &TimelineDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    _diagram_title: Option<&str>,
    _measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    timeline::render_timeline_diagram_svg(
        layout,
        semantic,
        effective_config,
        _diagram_title,
        _measurer,
        options,
    )
}

pub fn render_journey_diagram_svg(
    layout: &crate::model::JourneyDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    _diagram_title: Option<&str>,
    _measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    journey::render_journey_diagram_svg(
        layout,
        semantic,
        effective_config,
        _diagram_title,
        _measurer,
        options,
    )
}

pub fn render_kanban_diagram_svg(
    layout: &crate::model::KanbanDiagramLayout,
    _semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    kanban::render_kanban_diagram_svg(layout, _semantic, _effective_config, options)
}

pub fn render_gitgraph_diagram_svg(
    layout: &crate::model::GitGraphDiagramLayout,
    semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    gitgraph::render_gitgraph_diagram_svg(
        layout,
        semantic,
        _effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_tree_view_diagram_svg(
    layout: &TreeViewDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    tree_view::render_tree_view_diagram_svg(layout, semantic, effective_config, options)
}

pub fn render_ishikawa_diagram_svg(
    layout: &IshikawaDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    ishikawa::render_ishikawa_diagram_svg(layout, semantic, effective_config, options)
}

pub fn render_eventmodeling_diagram_svg(
    layout: &EventModelingDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    eventmodeling::render_eventmodeling_diagram_svg(layout, semantic, effective_config, options)
}

pub fn render_gantt_diagram_svg(
    layout: &crate::model::GanttDiagramLayout,
    semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    gantt::render_gantt_diagram_svg(layout, semantic, _effective_config, options)
}

pub fn render_mindmap_diagram_svg(
    layout: &MindmapDiagramLayout,
    semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    mindmap::render_mindmap_diagram_svg(layout, semantic, _effective_config, options)
}

pub fn render_mindmap_diagram_svg_with_config(
    layout: &MindmapDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    options: &SvgRenderOptions,
) -> Result<String> {
    mindmap::render_mindmap_diagram_svg_with_config(layout, semantic, effective_config, options)
}

pub fn render_architecture_diagram_svg(
    layout: &ArchitectureDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    architecture::render_architecture_diagram_svg(layout, semantic, effective_config, options)
}

pub fn render_c4_diagram_svg(
    layout: &crate::model::C4DiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    _measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    c4::render_c4_diagram_svg(
        layout,
        semantic,
        effective_config,
        diagram_title,
        _measurer,
        options,
    )
}

pub fn render_flowchart_v2_svg(
    layout: &FlowchartV2Layout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    flowchart::render_flowchart_v2_svg(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_flowchart_v2_svg_with_config(
    layout: &FlowchartV2Layout,
    semantic: &serde_json::Value,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    flowchart::render_flowchart_v2_svg_with_config(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_flowchart_v2_svg_model_with_config(
    layout: &FlowchartV2Layout,
    model: &merman_core::diagrams::flowchart::FlowchartV2Model,
    effective_config: &merman_core::MermaidConfig,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    flowchart::render_flowchart_v2_svg_model_with_config(
        layout,
        model,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_state_diagram_v2_svg(
    layout: &StateDiagramV2Layout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    state::render_state_diagram_v2_svg(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_state_diagram_v2_debug_svg(
    layout: &StateDiagramV2Layout,
    options: &SvgRenderOptions,
) -> String {
    state::render_state_diagram_v2_debug_svg(layout, options)
}

pub fn render_class_diagram_v2_debug_svg(
    layout: &ClassDiagramV2Layout,
    options: &SvgRenderOptions,
) -> String {
    class::render_class_diagram_v2_debug_svg(layout, options)
}

pub fn render_class_diagram_v2_svg(
    layout: &ClassDiagramV2Layout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    class::render_class_diagram_v2_svg(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_er_diagram_debug_svg(layout: &ErDiagramLayout, options: &SvgRenderOptions) -> String {
    er::render_er_diagram_debug_svg(layout, options)
}

pub fn render_er_diagram_svg(
    layout: &ErDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    er::render_er_diagram_svg(
        layout,
        semantic,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub fn render_sankey_diagram_svg(
    layout: &SankeyDiagramLayout,
    _semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    sankey::render_sankey_diagram_svg(layout, _semantic, effective_config, options)
}

// Ported from D3 `curveBasis` (d3-shape v3.x), used by Mermaid ER renderer `@11.12.2`.
fn curve_basis_path_d(points: &[crate::model::LayoutPoint]) -> String {
    curve::curve_basis_path_d(points)
}
fn render_node(out: &mut String, n: &LayoutNode) {
    layout_debug::render_node(out, n)
}

fn render_state_node(out: &mut String, n: &LayoutNode) {
    layout_debug::render_state_node(out, n)
}

fn render_cluster(out: &mut String, c: &LayoutCluster, include_markers: bool) {
    layout_debug::render_cluster(out, c, include_markers)
}

fn compute_layout_bounds(
    clusters: &[LayoutCluster],
    nodes: &[LayoutNode],
    edges: &[crate::model::LayoutEdge],
) -> Option<Bounds> {
    layout_debug::compute_layout_bounds(clusters, nodes, edges)
}
