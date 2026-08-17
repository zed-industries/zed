#![forbid(unsafe_code)]

//! Headless layout + rendering for Mermaid diagrams.
//!
//! This crate consumes `merman-core`'s semantic models and produces:
//! - a layout JSON (geometry + routes)
//! - Mermaid-like SVG output with DOM parity checks against upstream baselines

pub mod architecture;
pub(crate) mod architecture_metrics;
pub mod block;
pub mod c4;
mod chart_palette;
pub mod class;
mod config;
mod entities;
pub mod er;
pub mod error;
pub mod eventmodeling;
pub mod flowchart;
pub mod gantt;
mod generated;
pub mod gitgraph;
pub mod info;
pub mod ishikawa;
pub mod journey;
mod json;
pub mod kanban;
pub mod math;
mod mermaid_style;
pub mod mindmap;
pub mod model;
pub mod packet;
pub mod pie;
pub mod quadrantchart;
pub mod radar;
pub mod requirement;
pub mod sankey;
pub mod sequence;
pub mod state;
pub mod svg;
pub mod text;
mod theme;
pub mod timeline;
pub mod tree_view;
pub mod treemap;
mod trig_tables;
pub mod venn;
pub mod xychart;

use crate::math::MathRenderer;
use crate::model::{LayoutDiagram, LayoutMeta, LayoutedDiagram};
use crate::text::{DeterministicTextMeasurer, TextMeasurer};
use merman_core::{ParsedDiagram, ParsedDiagramRender, RenderSemanticModel};
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported diagram type for layout: {diagram_type}")]
    UnsupportedDiagram { diagram_type: String },
    #[error("invalid semantic model: {message}")]
    InvalidModel { message: String },
    #[error("SVG postprocessor `{pass}` failed: {message}")]
    SvgPostprocess { pass: String, message: String },
    #[error("semantic model JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn svg_postprocess(pass: impl Into<String>, message: impl Into<String>) -> Self {
        Self::SvgPostprocess {
            pass: pass.into(),
            message: message.into(),
        }
    }
}

#[derive(Clone)]
pub struct LayoutOptions {
    pub text_measurer: Arc<dyn TextMeasurer + Send + Sync>,
    /// Optional math renderer for `$$...$$` style labels.
    pub math_renderer: Option<Arc<dyn MathRenderer + Send + Sync>>,
    pub viewport_width: f64,
    pub viewport_height: f64,
    /// Enable experimental layout engines (e.g. Cytoscape COSE/FCoSE ports) for diagrams that
    /// currently use placeholder layouts in merman.
    pub use_manatee_layout: bool,
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self {
            text_measurer: Arc::new(DeterministicTextMeasurer::default()),
            math_renderer: None,
            viewport_width: 800.0,
            viewport_height: 600.0,
            use_manatee_layout: false,
        }
    }
}

impl LayoutOptions {
    /// Returns layout defaults suitable for headless SVG rendering in UI integrations.
    ///
    /// Compared to `Default`, this uses a Mermaid-like text measurer backed by vendored font
    /// metrics (instead of deterministic placeholder metrics).
    pub fn headless_svg_defaults() -> Self {
        Self {
            text_measurer: Arc::new(crate::text::VendoredFontMetricsTextMeasurer::default()),
            // Mermaid parity fixtures for diagrams like mindmap/architecture rely on the COSE
            // layout port (manatee). Make the headless defaults "just work" for UI integrations.
            use_manatee_layout: true,
            ..Default::default()
        }
    }

    pub fn with_text_measurer(mut self, measurer: Arc<dyn TextMeasurer + Send + Sync>) -> Self {
        self.text_measurer = measurer;
        self
    }

    pub fn with_math_renderer(mut self, renderer: Arc<dyn MathRenderer + Send + Sync>) -> Self {
        self.math_renderer = Some(renderer);
        self
    }
}

pub fn layout_parsed(parsed: &ParsedDiagram, options: &LayoutOptions) -> Result<LayoutedDiagram> {
    let meta = LayoutMeta::from_parse_metadata(&parsed.meta);
    let layout = layout_parsed_layout_only(parsed, options)?;

    Ok(LayoutedDiagram {
        meta,
        semantic: crate::json::clone_value_nonrecursive(&parsed.model),
        layout,
    })
}

pub fn layout_parsed_layout_only(
    parsed: &ParsedDiagram,
    options: &LayoutOptions,
) -> Result<LayoutDiagram> {
    let diagram_type = parsed.meta.diagram_type.as_str();
    let title = parsed.meta.title.as_deref();
    layout_json_by_type(
        diagram_type,
        &parsed.model,
        &parsed.meta.effective_config,
        title,
        options,
    )
}

pub fn layout_parsed_render_layout_only(
    parsed: &ParsedDiagramRender,
    options: &LayoutOptions,
) -> Result<LayoutDiagram> {
    let diagram_type = parsed.meta.diagram_type.as_str();
    let effective_config = parsed.meta.effective_config.as_value();
    let title = parsed.meta.title.as_deref();

    if !parsed.model.supports_diagram_type(diagram_type) {
        return Err(Error::InvalidModel {
            message: format!(
                "unexpected render model variant {} for diagram type: {diagram_type}",
                parsed.model.kind()
            ),
        });
    }

    match &parsed.model {
        RenderSemanticModel::Mindmap(model) => Ok(LayoutDiagram::MindmapDiagram(Box::new(
            mindmap::layout_mindmap_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
                options.use_manatee_layout,
            )?,
        ))),
        RenderSemanticModel::Architecture(model) => Ok(LayoutDiagram::ArchitectureDiagram(
            Box::new(architecture::layout_architecture_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
                options.use_manatee_layout,
            )?),
        )),
        RenderSemanticModel::Flowchart(model) => Ok(LayoutDiagram::FlowchartV2(Box::new(
            flowchart::layout_flowchart_v2_typed(
                model,
                &parsed.meta.effective_config,
                options.text_measurer.as_ref(),
                options.math_renderer.as_deref(),
            )?,
        ))),
        RenderSemanticModel::State(model) => Ok(LayoutDiagram::StateDiagramV2(Box::new(
            state::layout_state_diagram_v2_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Sequence(model) => Ok(LayoutDiagram::SequenceDiagram(Box::new(
            sequence::layout_sequence_diagram_typed_with_title(
                model,
                title,
                effective_config,
                options.text_measurer.as_ref(),
                options.math_renderer.as_deref(),
            )?,
        ))),
        RenderSemanticModel::Class(model) => Ok(LayoutDiagram::ClassDiagramV2(Box::new(
            class::layout_class_diagram_v2_typed_with_config(
                model,
                &parsed.meta.effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::C4(model) => Ok(LayoutDiagram::C4Diagram(Box::new(
            c4::layout_c4_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
                options.viewport_width,
                options.viewport_height,
            )?,
        ))),
        RenderSemanticModel::Kanban(model) => Ok(LayoutDiagram::KanbanDiagram(Box::new(
            kanban::layout_kanban_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Gantt(model) => Ok(LayoutDiagram::GanttDiagram(Box::new(
            gantt::layout_gantt_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Pie(model) => Ok(LayoutDiagram::PieDiagram(Box::new(
            pie::layout_pie_diagram_typed(model, effective_config, options.text_measurer.as_ref())?,
        ))),
        RenderSemanticModel::Packet(model) => Ok(LayoutDiagram::PacketDiagram(Box::new(
            packet::layout_packet_diagram_typed(
                model,
                title,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Timeline(model) => Ok(LayoutDiagram::TimelineDiagram(Box::new(
            timeline::layout_timeline_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Journey(model) => Ok(LayoutDiagram::JourneyDiagram(Box::new(
            journey::layout_journey_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Requirement(model) => Ok(LayoutDiagram::RequirementDiagram(Box::new(
            requirement::layout_requirement_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Sankey(model) => Ok(LayoutDiagram::SankeyDiagram(Box::new(
            sankey::layout_sankey_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Radar(model) => Ok(LayoutDiagram::RadarDiagram(Box::new(
            radar::layout_radar_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Info(model) => Ok(LayoutDiagram::InfoDiagram(Box::new(
            info::layout_info_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Treemap(model) => Ok(LayoutDiagram::TreemapDiagram(Box::new(
            treemap::layout_treemap_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Block(model) => Ok(LayoutDiagram::BlockDiagram(Box::new(
            block::layout_block_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Er(model) => Ok(LayoutDiagram::ErDiagram(Box::new(
            er::layout_er_diagram_typed(model, effective_config, options.text_measurer.as_ref())?,
        ))),
        RenderSemanticModel::QuadrantChart(model) => Ok(LayoutDiagram::QuadrantChartDiagram(
            Box::new(quadrantchart::layout_quadrantchart_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?),
        )),
        RenderSemanticModel::XyChart(model) => Ok(LayoutDiagram::XyChartDiagram(Box::new(
            xychart::layout_xychart_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::GitGraph(model) => Ok(LayoutDiagram::GitGraphDiagram(Box::new(
            gitgraph::layout_gitgraph_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::TreeView(model) => Ok(LayoutDiagram::TreeViewDiagram(Box::new(
            tree_view::layout_tree_view_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::Ishikawa(model) => Ok(LayoutDiagram::IshikawaDiagram(Box::new(
            ishikawa::layout_ishikawa_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        RenderSemanticModel::EventModeling(model) => Ok(LayoutDiagram::EventModelingDiagram(
            Box::new(eventmodeling::layout_eventmodeling_diagram_typed(
                model,
                effective_config,
                options.text_measurer.as_ref(),
            )?),
        )),
        RenderSemanticModel::Venn(model) => Ok(LayoutDiagram::VennDiagram(Box::new(
            venn::layout_venn_diagram_typed(model, title, effective_config)?,
        ))),
        RenderSemanticModel::Json(semantic) => layout_json_by_type(
            diagram_type,
            semantic,
            &parsed.meta.effective_config,
            title,
            options,
        ),
    }
}

fn layout_json_by_type(
    diagram_type: &str,
    semantic: &Value,
    effective_config: &merman_core::MermaidConfig,
    title: Option<&str>,
    options: &LayoutOptions,
) -> Result<LayoutDiagram> {
    let effective_config_value = effective_config.as_value();

    match diagram_type {
        "error" => Ok(LayoutDiagram::ErrorDiagram(Box::new(
            error::layout_error_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "block" => Ok(LayoutDiagram::BlockDiagram(Box::new(
            block::layout_block_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "architecture" => Ok(LayoutDiagram::ArchitectureDiagram(Box::new(
            architecture::layout_architecture_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
                options.use_manatee_layout,
            )?,
        ))),
        "requirement" => Ok(LayoutDiagram::RequirementDiagram(Box::new(
            requirement::layout_requirement_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "radar" => Ok(LayoutDiagram::RadarDiagram(Box::new(
            radar::layout_radar_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "treemap" => Ok(LayoutDiagram::TreemapDiagram(Box::new(
            treemap::layout_treemap_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "venn" => Ok(LayoutDiagram::VennDiagram(Box::new(
            venn::layout_venn_diagram(semantic, title, effective_config_value)?,
        ))),
        "flowchart-v2" => Ok(LayoutDiagram::FlowchartV2(Box::new(
            flowchart::layout_flowchart_v2(
                semantic,
                effective_config,
                options.text_measurer.as_ref(),
                options.math_renderer.as_deref(),
            )?,
        ))),
        "stateDiagram" => Ok(LayoutDiagram::StateDiagramV2(Box::new(
            state::layout_state_diagram_v2(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "classDiagram" | "class" => Ok(LayoutDiagram::ClassDiagramV2(Box::new(
            class::layout_class_diagram_v2_with_config(
                semantic,
                effective_config,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "er" | "erDiagram" => Ok(LayoutDiagram::ErDiagram(Box::new(er::layout_er_diagram(
            semantic,
            effective_config_value,
            options.text_measurer.as_ref(),
        )?))),
        "sequence" | "zenuml" => Ok(LayoutDiagram::SequenceDiagram(Box::new(
            sequence::layout_sequence_diagram_with_title(
                semantic,
                title,
                effective_config_value,
                options.text_measurer.as_ref(),
                options.math_renderer.as_deref(),
            )?,
        ))),
        "info" => Ok(LayoutDiagram::InfoDiagram(Box::new(
            info::layout_info_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "packet" => Ok(LayoutDiagram::PacketDiagram(Box::new(
            packet::layout_packet_diagram(
                semantic,
                title,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "timeline" => Ok(LayoutDiagram::TimelineDiagram(Box::new(
            timeline::layout_timeline_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "gantt" => Ok(LayoutDiagram::GanttDiagram(Box::new(
            gantt::layout_gantt_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "c4" => Ok(LayoutDiagram::C4Diagram(Box::new(c4::layout_c4_diagram(
            semantic,
            effective_config_value,
            options.text_measurer.as_ref(),
            options.viewport_width,
            options.viewport_height,
        )?))),
        "journey" => Ok(LayoutDiagram::JourneyDiagram(Box::new(
            journey::layout_journey_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "gitGraph" => Ok(LayoutDiagram::GitGraphDiagram(Box::new(
            gitgraph::layout_gitgraph_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "kanban" => Ok(LayoutDiagram::KanbanDiagram(Box::new(
            kanban::layout_kanban_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "pie" => Ok(LayoutDiagram::PieDiagram(Box::new(
            pie::layout_pie_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "xychart" => Ok(LayoutDiagram::XyChartDiagram(Box::new(
            xychart::layout_xychart_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "quadrantChart" => Ok(LayoutDiagram::QuadrantChartDiagram(Box::new(
            quadrantchart::layout_quadrantchart_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "mindmap" => Ok(LayoutDiagram::MindmapDiagram(Box::new(
            mindmap::layout_mindmap_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
                options.use_manatee_layout,
            )?,
        ))),
        "sankey" => Ok(LayoutDiagram::SankeyDiagram(Box::new(
            sankey::layout_sankey_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "treeView" => Ok(LayoutDiagram::TreeViewDiagram(Box::new(
            tree_view::layout_tree_view_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "ishikawa" => Ok(LayoutDiagram::IshikawaDiagram(Box::new(
            ishikawa::layout_ishikawa_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        "eventmodeling" => Ok(LayoutDiagram::EventModelingDiagram(Box::new(
            eventmodeling::layout_eventmodeling_diagram(
                semantic,
                effective_config_value,
                options.text_measurer.as_ref(),
            )?,
        ))),
        other => Err(Error::UnsupportedDiagram {
            diagram_type: other.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use merman_core::{Engine, ParseOptions};

    #[test]
    fn render_model_dispatch_accepts_diagram_type_aliases() {
        let parsed = Engine::new()
            .parse_diagram_for_render_model_with_type_sync(
                "flowchart-elk",
                "flowchart-elk TD\nA-->B;",
                ParseOptions::strict(),
            )
            .unwrap()
            .unwrap();

        let layout = layout_parsed_render_layout_only(&parsed, &LayoutOptions::default()).unwrap();
        assert!(matches!(layout, LayoutDiagram::FlowchartV2(_)));
    }

    #[test]
    fn render_model_dispatch_rejects_mismatched_typed_model() {
        let mut parsed = Engine::new()
            .parse_diagram_for_render_model_sync(
                "sequenceDiagram\nAlice->>Bob: Hi",
                ParseOptions::strict(),
            )
            .unwrap()
            .unwrap();
        parsed.meta.diagram_type = "flowchart-v2".to_string();

        let err = layout_parsed_render_layout_only(&parsed, &LayoutOptions::default()).unwrap_err();
        let message = err.to_string();
        assert!(message.contains("sequence"));
        assert!(message.contains("flowchart-v2"));
    }
}
