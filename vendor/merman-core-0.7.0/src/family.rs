//! Diagram family facts for the pinned Mermaid baseline.
//!
//! This module owns release-facing Mermaid family facts and projects them into detector,
//! parser, render-model, and metadata surfaces.

use crate::baseline::BaselineRegistryProfile;
use crate::detect::DetectorFn;
use crate::diagram::{DiagramSemanticParser, RenderSemanticModel, RenderSemanticParser};
use crate::{MermaidConfig, ParseMetadata, Result};
use serde_json::Value;
use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub(crate) struct DetectorFact {
    pub(crate) id: &'static str,
    pub(crate) detector: DetectorFn,
}

#[derive(Clone, Copy)]
pub(crate) struct FastDetectKeywordFact {
    keyword: &'static str,
    id: &'static str,
}

#[derive(Clone, Copy)]
pub(crate) struct SemanticParserFact {
    pub(crate) id: &'static str,
    pub(crate) parser: DiagramSemanticParser,
}

#[derive(Clone, Copy)]
pub(crate) struct RenderParserFact {
    pub(crate) id: &'static str,
    pub(crate) metadata_id: Option<&'static str>,
    pub(crate) model_kind: &'static str,
    pub(crate) parser: RenderSemanticParser,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SupportedDiagramFact {
    pub(crate) metadata_id: &'static str,
    pub(crate) render_parser_ids: Vec<&'static str>,
}

pub(crate) fn detector_facts(profile: BaselineRegistryProfile) -> &'static [DetectorFact] {
    match profile {
        BaselineRegistryProfile::Tiny => DETECTOR_FACTS_TINY,
        BaselineRegistryProfile::Full => DETECTOR_FACTS_FULL,
    }
}

pub(crate) fn fast_detect_by_leading_keyword(
    text: &str,
    profile: BaselineRegistryProfile,
) -> Option<&'static str> {
    fn has_boundary(rest: &str) -> bool {
        rest.is_empty()
            || rest
                .chars()
                .next()
                .is_some_and(|c| c.is_whitespace() || c == ';')
    }

    let trimmed = text.trim_start();
    let keywords = match profile {
        BaselineRegistryProfile::Tiny => FAST_DETECT_KEYWORDS_TINY,
        BaselineRegistryProfile::Full => FAST_DETECT_KEYWORDS_FULL,
    };

    keywords.iter().find_map(|fact| {
        trimmed
            .strip_prefix(fact.keyword)
            .and_then(|rest| has_boundary(rest).then_some(fact.id))
    })
}

pub(crate) fn semantic_parser_facts() -> &'static [SemanticParserFact] {
    SEMANTIC_PARSER_FACTS
}

pub(crate) fn render_parser_facts() -> &'static [RenderParserFact] {
    RENDER_PARSER_FACTS
}

pub(crate) fn supported_diagram_facts() -> &'static [SupportedDiagramFact] {
    static FACTS: OnceLock<Vec<SupportedDiagramFact>> = OnceLock::new();
    FACTS
        .get_or_init(|| {
            SUPPORTED_DIAGRAM_METADATA_IDS
                .iter()
                .map(|metadata_id| SupportedDiagramFact {
                    metadata_id,
                    render_parser_ids: RENDER_PARSER_FACTS
                        .iter()
                        .filter_map(|fact| {
                            (fact.metadata_id == Some(*metadata_id)).then_some(fact.id)
                        })
                        .collect(),
                })
                .collect()
        })
        .as_slice()
}

pub(crate) fn supported_diagram_metadata_ids() -> &'static [&'static str] {
    static IDS: OnceLock<Vec<&'static str>> = OnceLock::new();
    IDS.get_or_init(|| {
        supported_diagram_facts()
            .iter()
            .inspect(|fact| debug_assert!(!fact.render_parser_ids.is_empty()))
            .map(|fact| fact.metadata_id)
            .collect()
    })
    .as_slice()
}

pub(crate) fn render_model_kind_supports_diagram_type(
    model_kind: &'static str,
    diagram_type: &str,
) -> bool {
    RENDER_PARSER_FACTS
        .iter()
        .any(|fact| fact.model_kind == model_kind && fact.id == diagram_type)
}

pub(crate) fn permits_json_render_fallback(diagram_type: &str) -> bool {
    diagram_type == "error"
        || !SEMANTIC_PARSER_FACTS
            .iter()
            .any(|fact| fact.id == diagram_type)
}

pub(crate) fn apply_known_type_detector_side_effects(
    diagram_type: &str,
    effective_config: &mut MermaidConfig,
) {
    if diagram_type == "flowchart-elk" {
        effective_config.set_value("layout", Value::String("elk".to_string()));
        return;
    }

    if matches!(diagram_type, "flowchart-v2" | "flowchart")
        && effective_config.get_str("flowchart.defaultRenderer") == Some("elk")
    {
        effective_config.set_value("layout", Value::String("elk".to_string()));
    }
}

const DETECTOR_FACTS_FULL: &[DetectorFact] = &[
    DetectorFact {
        id: "error",
        detector: crate::detect::detector_error,
    },
    DetectorFact {
        id: "---",
        detector: crate::detect::detector_frontmatter_unparsed,
    },
    DetectorFact {
        id: "flowchart-elk",
        detector: crate::detect::detector_flowchart_elk,
    },
    DetectorFact {
        id: "mindmap",
        detector: crate::detect::detector_mindmap,
    },
    DetectorFact {
        id: "architecture",
        detector: crate::detect::detector_architecture,
    },
    DetectorFact {
        id: "zenuml",
        detector: crate::detect::detector_zenuml,
    },
    DetectorFact {
        id: "c4",
        detector: crate::detect::detector_c4,
    },
    DetectorFact {
        id: "kanban",
        detector: crate::detect::detector_kanban,
    },
    DetectorFact {
        id: "classDiagram",
        detector: crate::detect::detector_class_v2,
    },
    DetectorFact {
        id: "class",
        detector: crate::detect::detector_class_dagre_d3,
    },
    DetectorFact {
        id: "er",
        detector: crate::detect::detector_er,
    },
    DetectorFact {
        id: "gantt",
        detector: crate::detect::detector_gantt,
    },
    DetectorFact {
        id: "info",
        detector: crate::detect::detector_info,
    },
    DetectorFact {
        id: "pie",
        detector: crate::detect::detector_pie,
    },
    DetectorFact {
        id: "requirement",
        detector: crate::detect::detector_requirement,
    },
    DetectorFact {
        id: "sequence",
        detector: crate::detect::detector_sequence,
    },
    DetectorFact {
        id: "flowchart-v2",
        detector: crate::detect::detector_flowchart_v2,
    },
    DetectorFact {
        id: "flowchart",
        detector: crate::detect::detector_flowchart_dagre_d3_graph,
    },
    DetectorFact {
        id: "timeline",
        detector: crate::detect::detector_timeline,
    },
    DetectorFact {
        id: "gitGraph",
        detector: crate::detect::detector_git_graph,
    },
    DetectorFact {
        id: "stateDiagram",
        detector: crate::detect::detector_state_v2,
    },
    DetectorFact {
        id: "state",
        detector: crate::detect::detector_state_dagre_d3,
    },
    DetectorFact {
        id: "journey",
        detector: crate::detect::detector_journey,
    },
    DetectorFact {
        id: "quadrantChart",
        detector: crate::detect::detector_quadrant,
    },
    DetectorFact {
        id: "sankey",
        detector: crate::detect::detector_sankey,
    },
    DetectorFact {
        id: "packet",
        detector: crate::detect::detector_packet,
    },
    DetectorFact {
        id: "xychart",
        detector: crate::detect::detector_xychart,
    },
    DetectorFact {
        id: "block",
        detector: crate::detect::detector_block,
    },
    DetectorFact {
        id: "treeView",
        detector: crate::detect::detector_tree_view,
    },
    DetectorFact {
        id: "ishikawa",
        detector: crate::detect::detector_ishikawa,
    },
    DetectorFact {
        id: "eventmodeling",
        detector: crate::detect::detector_eventmodeling,
    },
    DetectorFact {
        id: "radar",
        detector: crate::detect::detector_radar,
    },
    DetectorFact {
        id: "treemap",
        detector: crate::detect::detector_treemap,
    },
    DetectorFact {
        id: "venn",
        detector: crate::detect::detector_venn,
    },
];

const DETECTOR_FACTS_TINY: &[DetectorFact] = &[
    DetectorFact {
        id: "error",
        detector: crate::detect::detector_error,
    },
    DetectorFact {
        id: "---",
        detector: crate::detect::detector_frontmatter_unparsed,
    },
    DetectorFact {
        id: "zenuml",
        detector: crate::detect::detector_zenuml,
    },
    DetectorFact {
        id: "c4",
        detector: crate::detect::detector_c4,
    },
    DetectorFact {
        id: "kanban",
        detector: crate::detect::detector_kanban,
    },
    DetectorFact {
        id: "classDiagram",
        detector: crate::detect::detector_class_v2,
    },
    DetectorFact {
        id: "class",
        detector: crate::detect::detector_class_dagre_d3,
    },
    DetectorFact {
        id: "er",
        detector: crate::detect::detector_er,
    },
    DetectorFact {
        id: "gantt",
        detector: crate::detect::detector_gantt,
    },
    DetectorFact {
        id: "info",
        detector: crate::detect::detector_info,
    },
    DetectorFact {
        id: "pie",
        detector: crate::detect::detector_pie,
    },
    DetectorFact {
        id: "requirement",
        detector: crate::detect::detector_requirement,
    },
    DetectorFact {
        id: "sequence",
        detector: crate::detect::detector_sequence,
    },
    DetectorFact {
        id: "flowchart-v2",
        detector: crate::detect::detector_flowchart_v2,
    },
    DetectorFact {
        id: "flowchart",
        detector: crate::detect::detector_flowchart_dagre_d3_graph,
    },
    DetectorFact {
        id: "timeline",
        detector: crate::detect::detector_timeline,
    },
    DetectorFact {
        id: "gitGraph",
        detector: crate::detect::detector_git_graph,
    },
    DetectorFact {
        id: "stateDiagram",
        detector: crate::detect::detector_state_v2,
    },
    DetectorFact {
        id: "state",
        detector: crate::detect::detector_state_dagre_d3,
    },
    DetectorFact {
        id: "journey",
        detector: crate::detect::detector_journey,
    },
    DetectorFact {
        id: "quadrantChart",
        detector: crate::detect::detector_quadrant,
    },
    DetectorFact {
        id: "sankey",
        detector: crate::detect::detector_sankey,
    },
    DetectorFact {
        id: "packet",
        detector: crate::detect::detector_packet,
    },
    DetectorFact {
        id: "xychart",
        detector: crate::detect::detector_xychart,
    },
    DetectorFact {
        id: "block",
        detector: crate::detect::detector_block,
    },
    DetectorFact {
        id: "treeView",
        detector: crate::detect::detector_tree_view,
    },
    DetectorFact {
        id: "ishikawa",
        detector: crate::detect::detector_ishikawa,
    },
    DetectorFact {
        id: "eventmodeling",
        detector: crate::detect::detector_eventmodeling,
    },
    DetectorFact {
        id: "radar",
        detector: crate::detect::detector_radar,
    },
    DetectorFact {
        id: "treemap",
        detector: crate::detect::detector_treemap,
    },
    DetectorFact {
        id: "venn",
        detector: crate::detect::detector_venn,
    },
];

const FAST_DETECT_KEYWORDS_FULL: &[FastDetectKeywordFact] = &[
    FastDetectKeywordFact {
        keyword: "sequenceDiagram",
        id: "sequence",
    },
    FastDetectKeywordFact {
        keyword: "classDiagram",
        id: "classDiagram",
    },
    FastDetectKeywordFact {
        keyword: "stateDiagram",
        id: "stateDiagram",
    },
    FastDetectKeywordFact {
        keyword: "mindmap",
        id: "mindmap",
    },
    FastDetectKeywordFact {
        keyword: "architecture",
        id: "architecture",
    },
    FastDetectKeywordFact {
        keyword: "erDiagram",
        id: "er",
    },
    FastDetectKeywordFact {
        keyword: "gantt",
        id: "gantt",
    },
    FastDetectKeywordFact {
        keyword: "timeline",
        id: "timeline",
    },
    FastDetectKeywordFact {
        keyword: "journey",
        id: "journey",
    },
    FastDetectKeywordFact {
        keyword: "gitGraph",
        id: "gitGraph",
    },
    FastDetectKeywordFact {
        keyword: "quadrantChart",
        id: "quadrantChart",
    },
    FastDetectKeywordFact {
        keyword: "packet-beta",
        id: "packet",
    },
    FastDetectKeywordFact {
        keyword: "xychart-beta",
        id: "xychart",
    },
    FastDetectKeywordFact {
        keyword: "treeView-beta",
        id: "treeView",
    },
    FastDetectKeywordFact {
        keyword: "ishikawa-beta",
        id: "ishikawa",
    },
    FastDetectKeywordFact {
        keyword: "ishikawa",
        id: "ishikawa",
    },
    FastDetectKeywordFact {
        keyword: "eventmodeling",
        id: "eventmodeling",
    },
];

const FAST_DETECT_KEYWORDS_TINY: &[FastDetectKeywordFact] = &[
    FastDetectKeywordFact {
        keyword: "sequenceDiagram",
        id: "sequence",
    },
    FastDetectKeywordFact {
        keyword: "classDiagram",
        id: "classDiagram",
    },
    FastDetectKeywordFact {
        keyword: "stateDiagram",
        id: "stateDiagram",
    },
    FastDetectKeywordFact {
        keyword: "erDiagram",
        id: "er",
    },
    FastDetectKeywordFact {
        keyword: "gantt",
        id: "gantt",
    },
    FastDetectKeywordFact {
        keyword: "timeline",
        id: "timeline",
    },
    FastDetectKeywordFact {
        keyword: "journey",
        id: "journey",
    },
    FastDetectKeywordFact {
        keyword: "gitGraph",
        id: "gitGraph",
    },
    FastDetectKeywordFact {
        keyword: "quadrantChart",
        id: "quadrantChart",
    },
    FastDetectKeywordFact {
        keyword: "packet-beta",
        id: "packet",
    },
    FastDetectKeywordFact {
        keyword: "xychart-beta",
        id: "xychart",
    },
    FastDetectKeywordFact {
        keyword: "treeView-beta",
        id: "treeView",
    },
    FastDetectKeywordFact {
        keyword: "ishikawa-beta",
        id: "ishikawa",
    },
    FastDetectKeywordFact {
        keyword: "ishikawa",
        id: "ishikawa",
    },
    FastDetectKeywordFact {
        keyword: "eventmodeling",
        id: "eventmodeling",
    },
];

const SEMANTIC_PARSER_FACTS: &[SemanticParserFact] = &[
    SemanticParserFact {
        id: "error",
        parser: crate::diagrams::error_diagram::parse_error,
    },
    SemanticParserFact {
        id: "flowchart-v2",
        parser: crate::diagrams::flowchart::parse_flowchart,
    },
    SemanticParserFact {
        id: "flowchart",
        parser: crate::diagrams::flowchart::parse_flowchart,
    },
    SemanticParserFact {
        id: "flowchart-elk",
        parser: crate::diagrams::flowchart::parse_flowchart,
    },
    SemanticParserFact {
        id: "info",
        parser: crate::diagrams::info::parse_info,
    },
    SemanticParserFact {
        id: "pie",
        parser: crate::diagrams::pie::parse_pie,
    },
    SemanticParserFact {
        id: "c4",
        parser: crate::diagrams::c4::parse_c4,
    },
    SemanticParserFact {
        id: "requirement",
        parser: crate::diagrams::requirement::parse_requirement,
    },
    SemanticParserFact {
        id: "sequence",
        parser: crate::diagrams::sequence::parse_sequence,
    },
    SemanticParserFact {
        id: "zenuml",
        parser: crate::diagrams::zenuml::parse_zenuml,
    },
    SemanticParserFact {
        id: "classDiagram",
        parser: crate::diagrams::class::parse_class,
    },
    SemanticParserFact {
        id: "class",
        parser: crate::diagrams::class::parse_class,
    },
    SemanticParserFact {
        id: "er",
        parser: crate::diagrams::er::parse_er,
    },
    SemanticParserFact {
        id: "erDiagram",
        parser: crate::diagrams::er::parse_er,
    },
    SemanticParserFact {
        id: "stateDiagram",
        parser: crate::diagrams::state::parse_state,
    },
    SemanticParserFact {
        id: "state",
        parser: crate::diagrams::state::parse_state,
    },
    SemanticParserFact {
        id: "mindmap",
        parser: crate::diagrams::mindmap::parse_mindmap,
    },
    SemanticParserFact {
        id: "gantt",
        parser: crate::diagrams::gantt::parse_gantt,
    },
    SemanticParserFact {
        id: "timeline",
        parser: crate::diagrams::timeline::parse_timeline,
    },
    SemanticParserFact {
        id: "journey",
        parser: crate::diagrams::journey::parse_journey,
    },
    SemanticParserFact {
        id: "kanban",
        parser: crate::diagrams::kanban::parse_kanban,
    },
    SemanticParserFact {
        id: "architecture",
        parser: crate::diagrams::architecture::parse_architecture,
    },
    SemanticParserFact {
        id: "block",
        parser: crate::diagrams::block::parse_block,
    },
    SemanticParserFact {
        id: "gitGraph",
        parser: crate::diagrams::git_graph::parse_git_graph,
    },
    SemanticParserFact {
        id: "quadrantChart",
        parser: crate::diagrams::quadrant_chart::parse_quadrant_chart,
    },
    SemanticParserFact {
        id: "packet",
        parser: crate::diagrams::packet::parse_packet,
    },
    SemanticParserFact {
        id: "radar",
        parser: crate::diagrams::radar::parse_radar,
    },
    SemanticParserFact {
        id: "treeView",
        parser: crate::diagrams::tree_view::parse_tree_view,
    },
    SemanticParserFact {
        id: "ishikawa",
        parser: crate::diagrams::ishikawa::parse_ishikawa,
    },
    SemanticParserFact {
        id: "eventmodeling",
        parser: crate::diagrams::eventmodeling::parse_eventmodeling,
    },
    SemanticParserFact {
        id: "treemap",
        parser: crate::diagrams::treemap::parse_treemap,
    },
    SemanticParserFact {
        id: "venn",
        parser: crate::diagrams::venn::parse_venn,
    },
    SemanticParserFact {
        id: "sankey",
        parser: crate::diagrams::sankey::parse_sankey,
    },
    SemanticParserFact {
        id: "xychart",
        parser: crate::diagrams::xychart::parse_xychart,
    },
];

macro_rules! render_parser {
    ($fn_name:ident, $parser:path, $variant:path) => {
        fn $fn_name(code: &str, meta: &ParseMetadata) -> Result<RenderSemanticModel> {
            $parser(code, meta).map($variant)
        }
    };
}

render_parser!(
    render_mindmap,
    crate::diagrams::mindmap::parse_mindmap_model_for_render,
    RenderSemanticModel::Mindmap
);
render_parser!(
    render_state,
    crate::diagrams::state::parse_state_model_for_render,
    RenderSemanticModel::State
);
render_parser!(
    render_zenuml,
    crate::diagrams::zenuml::parse_zenuml_model_for_render,
    RenderSemanticModel::Sequence
);
render_parser!(
    render_sequence,
    crate::diagrams::sequence::parse_sequence_model_for_render,
    RenderSemanticModel::Sequence
);
render_parser!(
    render_flowchart,
    crate::diagrams::flowchart::parse_flowchart_model_for_render,
    RenderSemanticModel::Flowchart
);
render_parser!(
    render_class,
    crate::diagrams::class::parse_class_typed,
    RenderSemanticModel::Class
);
render_parser!(
    render_c4,
    crate::diagrams::c4::parse_c4_model_for_render,
    RenderSemanticModel::C4
);
render_parser!(
    render_architecture,
    crate::diagrams::architecture::parse_architecture_model_for_render,
    RenderSemanticModel::Architecture
);
render_parser!(
    render_kanban,
    crate::diagrams::kanban::parse_kanban_model_for_render,
    RenderSemanticModel::Kanban
);
render_parser!(
    render_gantt,
    crate::diagrams::gantt::parse_gantt_model_for_render,
    RenderSemanticModel::Gantt
);
render_parser!(
    render_pie,
    crate::diagrams::pie::parse_pie_model_for_render,
    RenderSemanticModel::Pie
);
render_parser!(
    render_packet,
    crate::diagrams::packet::parse_packet_model_for_render,
    RenderSemanticModel::Packet
);
render_parser!(
    render_timeline,
    crate::diagrams::timeline::parse_timeline_model_for_render,
    RenderSemanticModel::Timeline
);
render_parser!(
    render_journey,
    crate::diagrams::journey::parse_journey_model_for_render,
    RenderSemanticModel::Journey
);
render_parser!(
    render_requirement,
    crate::diagrams::requirement::parse_requirement_model_for_render,
    RenderSemanticModel::Requirement
);
render_parser!(
    render_sankey,
    crate::diagrams::sankey::parse_sankey_model_for_render,
    RenderSemanticModel::Sankey
);
render_parser!(
    render_radar,
    crate::diagrams::radar::parse_radar_model_for_render,
    RenderSemanticModel::Radar
);
render_parser!(
    render_info,
    crate::diagrams::info::parse_info_model_for_render,
    RenderSemanticModel::Info
);
render_parser!(
    render_treemap,
    crate::diagrams::treemap::parse_treemap_model_for_render,
    RenderSemanticModel::Treemap
);
render_parser!(
    render_block,
    crate::diagrams::block::parse_block_model_for_render,
    RenderSemanticModel::Block
);
render_parser!(
    render_er,
    crate::diagrams::er::parse_er_model_for_render,
    RenderSemanticModel::Er
);
render_parser!(
    render_quadrant_chart,
    crate::diagrams::quadrant_chart::parse_quadrant_chart_model_for_render,
    RenderSemanticModel::QuadrantChart
);
render_parser!(
    render_xychart,
    crate::diagrams::xychart::parse_xychart_model_for_render,
    RenderSemanticModel::XyChart
);
render_parser!(
    render_git_graph,
    crate::diagrams::git_graph::parse_git_graph_model_for_render,
    RenderSemanticModel::GitGraph
);
render_parser!(
    render_tree_view,
    crate::diagrams::tree_view::parse_tree_view_model_for_render,
    RenderSemanticModel::TreeView
);
render_parser!(
    render_ishikawa,
    crate::diagrams::ishikawa::parse_ishikawa_model_for_render,
    RenderSemanticModel::Ishikawa
);
render_parser!(
    render_eventmodeling,
    crate::diagrams::eventmodeling::parse_eventmodeling_model_for_render,
    RenderSemanticModel::EventModeling
);
render_parser!(
    render_venn,
    crate::diagrams::venn::parse_venn_model_for_render,
    RenderSemanticModel::Venn
);

const RENDER_PARSER_FACTS: &[RenderParserFact] = &[
    RenderParserFact {
        id: "mindmap",
        metadata_id: Some("mindmap"),
        model_kind: "mindmap",
        parser: render_mindmap,
    },
    RenderParserFact {
        id: "stateDiagram",
        metadata_id: Some("state"),
        model_kind: "state",
        parser: render_state,
    },
    RenderParserFact {
        id: "state",
        metadata_id: Some("state"),
        model_kind: "state",
        parser: render_state,
    },
    RenderParserFact {
        id: "zenuml",
        metadata_id: Some("zenuml"),
        model_kind: "sequence",
        parser: render_zenuml,
    },
    RenderParserFact {
        id: "sequence",
        metadata_id: Some("sequence"),
        model_kind: "sequence",
        parser: render_sequence,
    },
    RenderParserFact {
        id: "flowchart-v2",
        metadata_id: Some("flowchart"),
        model_kind: "flowchart",
        parser: render_flowchart,
    },
    RenderParserFact {
        id: "flowchart",
        metadata_id: Some("flowchart"),
        model_kind: "flowchart",
        parser: render_flowchart,
    },
    RenderParserFact {
        id: "flowchart-elk",
        metadata_id: Some("flowchart"),
        model_kind: "flowchart",
        parser: render_flowchart,
    },
    RenderParserFact {
        id: "classDiagram",
        metadata_id: Some("class"),
        model_kind: "class",
        parser: render_class,
    },
    RenderParserFact {
        id: "class",
        metadata_id: Some("class"),
        model_kind: "class",
        parser: render_class,
    },
    RenderParserFact {
        id: "c4",
        metadata_id: Some("c4"),
        model_kind: "c4",
        parser: render_c4,
    },
    RenderParserFact {
        id: "architecture",
        metadata_id: Some("architecture"),
        model_kind: "architecture",
        parser: render_architecture,
    },
    RenderParserFact {
        id: "kanban",
        metadata_id: Some("kanban"),
        model_kind: "kanban",
        parser: render_kanban,
    },
    RenderParserFact {
        id: "gantt",
        metadata_id: Some("gantt"),
        model_kind: "gantt",
        parser: render_gantt,
    },
    RenderParserFact {
        id: "pie",
        metadata_id: Some("pie"),
        model_kind: "pie",
        parser: render_pie,
    },
    RenderParserFact {
        id: "packet",
        metadata_id: Some("packet"),
        model_kind: "packet",
        parser: render_packet,
    },
    RenderParserFact {
        id: "timeline",
        metadata_id: Some("timeline"),
        model_kind: "timeline",
        parser: render_timeline,
    },
    RenderParserFact {
        id: "journey",
        metadata_id: Some("journey"),
        model_kind: "journey",
        parser: render_journey,
    },
    RenderParserFact {
        id: "requirement",
        metadata_id: Some("requirement"),
        model_kind: "requirement",
        parser: render_requirement,
    },
    RenderParserFact {
        id: "sankey",
        metadata_id: Some("sankey"),
        model_kind: "sankey",
        parser: render_sankey,
    },
    RenderParserFact {
        id: "radar",
        metadata_id: Some("radar"),
        model_kind: "radar",
        parser: render_radar,
    },
    RenderParserFact {
        id: "info",
        metadata_id: Some("info"),
        model_kind: "info",
        parser: render_info,
    },
    RenderParserFact {
        id: "treemap",
        metadata_id: Some("treemap"),
        model_kind: "treemap",
        parser: render_treemap,
    },
    RenderParserFact {
        id: "block",
        metadata_id: Some("block"),
        model_kind: "block",
        parser: render_block,
    },
    RenderParserFact {
        id: "er",
        metadata_id: Some("er"),
        model_kind: "er",
        parser: render_er,
    },
    RenderParserFact {
        id: "erDiagram",
        metadata_id: Some("er"),
        model_kind: "er",
        parser: render_er,
    },
    RenderParserFact {
        id: "quadrantChart",
        metadata_id: Some("quadrantchart"),
        model_kind: "quadrantChart",
        parser: render_quadrant_chart,
    },
    RenderParserFact {
        id: "xychart",
        metadata_id: Some("xychart"),
        model_kind: "xychart",
        parser: render_xychart,
    },
    RenderParserFact {
        id: "gitGraph",
        metadata_id: Some("gitgraph"),
        model_kind: "gitGraph",
        parser: render_git_graph,
    },
    RenderParserFact {
        id: "treeView",
        metadata_id: None,
        model_kind: "treeView",
        parser: render_tree_view,
    },
    RenderParserFact {
        id: "ishikawa",
        metadata_id: None,
        model_kind: "ishikawa",
        parser: render_ishikawa,
    },
    RenderParserFact {
        id: "eventmodeling",
        metadata_id: None,
        model_kind: "eventmodeling",
        parser: render_eventmodeling,
    },
    RenderParserFact {
        id: "venn",
        metadata_id: Some("venn"),
        model_kind: "venn",
        parser: render_venn,
    },
];

const SUPPORTED_DIAGRAM_METADATA_IDS: &[&str] = &[
    "architecture",
    "block",
    "c4",
    "class",
    "er",
    "flowchart",
    "gantt",
    "gitgraph",
    "info",
    "journey",
    "kanban",
    "mindmap",
    "packet",
    "pie",
    "quadrantchart",
    "radar",
    "requirement",
    "sankey",
    "sequence",
    "state",
    "timeline",
    "treemap",
    "venn",
    "xychart",
    "zenuml",
];
