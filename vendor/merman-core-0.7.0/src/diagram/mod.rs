use crate::{Error, MermaidConfig, ParseMetadata, Result};
use serde_json::Value;

/// Parser used by the semantic JSON path for one Mermaid diagram family.
pub type DiagramSemanticParser = fn(code: &str, meta: &ParseMetadata) -> Result<Value>;

/// Parser used by the typed render-model path for one Mermaid diagram family.
pub type RenderSemanticParser = fn(code: &str, meta: &ParseMetadata) -> Result<RenderSemanticModel>;

/// Registry for semantic JSON parsers keyed by Mermaid diagram type id.
#[derive(Debug, Clone, Default)]
pub struct DiagramRegistry {
    parsers: std::collections::HashMap<&'static str, DiagramSemanticParser>,
}

impl DiagramRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers or replaces the parser for a Mermaid diagram type id.
    pub fn insert(&mut self, diagram_type: &'static str, parser: DiagramSemanticParser) {
        self.parsers.insert(diagram_type, parser);
    }

    /// Looks up a parser by Mermaid diagram type id.
    pub fn get(&self, diagram_type: &str) -> Option<DiagramSemanticParser> {
        self.parsers.get(diagram_type).copied()
    }

    /// Builds the semantic parser registry for the repository's pinned Mermaid baseline.
    pub fn for_pinned_mermaid_baseline() -> Self {
        let mut reg = Self::new();
        for fact in crate::family::semantic_parser_facts() {
            reg.insert(fact.id, fact.parser);
        }

        reg
    }

    #[cfg(test)]
    pub(crate) fn parser_ids(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.parsers.keys().copied()
    }
}

/// Parsed diagram metadata plus the Mermaid-compatible semantic JSON model.
#[derive(Debug, Clone)]
pub struct ParsedDiagram {
    /// Diagram type and effective configuration extracted during preprocessing.
    pub meta: ParseMetadata,
    /// Semantic JSON model matching Mermaid's parser/database output shape where possible.
    pub model: Value,
}

/// Typed semantic model used by the headless renderer.
///
/// Most public callers should use [`ParsedDiagram`] when they need JSON output. This enum is for
/// render paths that benefit from typed data and avoiding a JSON round trip.
#[derive(Debug, Clone)]
pub enum RenderSemanticModel {
    Json(Value),
    Mindmap(crate::diagrams::mindmap::MindmapDiagramRenderModel),
    State(crate::diagrams::state::StateDiagramRenderModel),
    Sequence(crate::diagrams::sequence::SequenceDiagramRenderModel),
    Flowchart(crate::diagrams::flowchart::FlowchartV2Model),
    Architecture(crate::diagrams::architecture::ArchitectureDiagramRenderModel),
    Class(crate::models::class_diagram::ClassDiagram),
    C4(crate::diagrams::c4::C4DiagramRenderModel),
    Kanban(crate::diagrams::kanban::KanbanDiagramRenderModel),
    Gantt(crate::diagrams::gantt::GanttDiagramRenderModel),
    Pie(crate::diagrams::pie::PieDiagramRenderModel),
    Packet(crate::diagrams::packet::PacketDiagramRenderModel),
    Timeline(crate::diagrams::timeline::TimelineDiagramRenderModel),
    Journey(crate::diagrams::journey::JourneyDiagramRenderModel),
    Requirement(crate::diagrams::requirement::RequirementDiagramRenderModel),
    Sankey(crate::diagrams::sankey::SankeyDiagramRenderModel),
    Radar(crate::diagrams::radar::RadarDiagramRenderModel),
    Info(crate::diagrams::info::InfoDiagramRenderModel),
    Treemap(crate::diagrams::treemap::TreemapDiagramRenderModel),
    Block(crate::diagrams::block::BlockDiagramRenderModel),
    Er(crate::diagrams::er::ErDiagramRenderModel),
    QuadrantChart(crate::diagrams::quadrant_chart::QuadrantChartRenderModel),
    XyChart(crate::diagrams::xychart::XyChartDiagramRenderModel),
    GitGraph(crate::diagrams::git_graph::GitGraphRenderModel),
    TreeView(crate::diagrams::tree_view::TreeViewDiagramRenderModel),
    Ishikawa(crate::diagrams::ishikawa::IshikawaDiagramRenderModel),
    EventModeling(crate::diagrams::eventmodeling::EventModelingDiagramRenderModel),
    Venn(crate::diagrams::venn::VennDiagramRenderModel),
}

impl RenderSemanticModel {
    /// Applies Mermaid common DB sanitization to family-owned typed fields.
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &MermaidConfig) {
        match self {
            Self::Json(v) => crate::common_db::apply_common_db_sanitization(v, config),
            Self::Mindmap(_) => {}
            Self::State(v) => v.sanitize_common_db_fields(config),
            Self::Sequence(v) => v.sanitize_common_db_fields(config),
            Self::Flowchart(v) => v.sanitize_common_db_fields(config),
            Self::Architecture(v) => v.sanitize_common_db_fields(config),
            Self::Class(v) => v.sanitize_common_db_fields(config),
            Self::C4(v) => v.sanitize_common_db_fields(config),
            Self::Kanban(_) => {}
            Self::Gantt(v) => v.sanitize_common_db_fields(config),
            Self::Pie(v) => v.sanitize_common_db_fields(config),
            Self::Packet(v) => v.sanitize_common_db_fields(config),
            Self::Timeline(v) => v.sanitize_common_db_fields(config),
            Self::Journey(v) => v.sanitize_common_db_fields(config),
            Self::Requirement(v) => v.sanitize_common_db_fields(config),
            Self::Sankey(_) => {}
            Self::Radar(v) => v.sanitize_common_db_fields(config),
            Self::Info(_) => {}
            Self::Treemap(v) => v.sanitize_common_db_fields(config),
            Self::Block(_) => {}
            Self::Er(v) => v.sanitize_common_db_fields(config),
            Self::QuadrantChart(v) => v.sanitize_common_db_fields(config),
            Self::XyChart(v) => v.sanitize_common_db_fields(config),
            Self::GitGraph(v) => v.sanitize_common_db_fields(config),
            Self::TreeView(v) => v.sanitize_common_db_fields(config),
            Self::Ishikawa(v) => v.sanitize_common_db_fields(config),
            Self::EventModeling(v) => v.sanitize_common_db_fields(config),
            Self::Venn(v) => v.sanitize_common_db_fields(config),
        }
    }

    /// Returns a stable family label for diagnostics and timing output.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Json(_) => "json",
            Self::Mindmap(_) => "mindmap",
            Self::State(_) => "state",
            Self::Sequence(_) => "sequence",
            Self::Flowchart(_) => "flowchart",
            Self::Architecture(_) => "architecture",
            Self::Class(_) => "class",
            Self::C4(_) => "c4",
            Self::Kanban(_) => "kanban",
            Self::Gantt(_) => "gantt",
            Self::Pie(_) => "pie",
            Self::Packet(_) => "packet",
            Self::Timeline(_) => "timeline",
            Self::Journey(_) => "journey",
            Self::Requirement(_) => "requirement",
            Self::Sankey(_) => "sankey",
            Self::Radar(_) => "radar",
            Self::Info(_) => "info",
            Self::Treemap(_) => "treemap",
            Self::Block(_) => "block",
            Self::Er(_) => "er",
            Self::QuadrantChart(_) => "quadrantChart",
            Self::XyChart(_) => "xychart",
            Self::GitGraph(_) => "gitGraph",
            Self::TreeView(_) => "treeView",
            Self::Ishikawa(_) => "ishikawa",
            Self::EventModeling(_) => "eventmodeling",
            Self::Venn(_) => "venn",
        }
    }

    /// Returns whether this typed model can represent the given Mermaid diagram type id.
    pub fn supports_diagram_type(&self, diagram_type: &str) -> bool {
        match self {
            Self::Json(_) => true,
            other => {
                crate::family::render_model_kind_supports_diagram_type(other.kind(), diagram_type)
            }
        }
    }
}

/// Registry for typed render-model parsers keyed by Mermaid diagram type id.
#[derive(Debug, Clone, Default)]
pub struct RenderDiagramRegistry {
    parsers: std::collections::HashMap<&'static str, RenderSemanticParser>,
}

impl RenderDiagramRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers or replaces the typed render parser for a Mermaid diagram type id.
    pub fn insert(&mut self, diagram_type: &'static str, parser: RenderSemanticParser) {
        self.parsers.insert(diagram_type, parser);
    }

    /// Looks up a typed render parser by Mermaid diagram type id.
    pub fn get(&self, diagram_type: &str) -> Option<RenderSemanticParser> {
        self.parsers.get(diagram_type).copied()
    }

    #[cfg(test)]
    pub(crate) fn remove(&mut self, diagram_type: &str) -> Option<RenderSemanticParser> {
        self.parsers.remove(diagram_type)
    }

    /// Builds the typed render parser registry for the repository's pinned Mermaid baseline.
    pub fn for_pinned_mermaid_baseline() -> Self {
        let mut reg = Self::new();
        for fact in crate::family::render_parser_facts() {
            reg.insert(fact.id, fact.parser);
        }

        reg
    }

    #[cfg(test)]
    pub(crate) fn parser_ids(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.parsers.keys().copied()
    }
}

/// Parsed diagram metadata plus a typed render model.
#[derive(Debug, Clone)]
pub struct ParsedDiagramRender {
    /// Diagram type and effective configuration extracted during preprocessing.
    pub meta: ParseMetadata,
    /// Typed model consumed by layout and SVG renderers.
    pub model: RenderSemanticModel,
}

/// Parses with a registry entry or reports an unsupported Mermaid diagram type.
pub fn parse_or_unsupported(
    registry: &DiagramRegistry,
    diagram_type: &str,
    code: &str,
    meta: &ParseMetadata,
) -> Result<Value> {
    let Some(parser) = registry.get(diagram_type) else {
        return Err(Error::UnsupportedDiagram {
            diagram_type: diagram_type.to_string(),
        });
    };
    parser(code, meta)
}
