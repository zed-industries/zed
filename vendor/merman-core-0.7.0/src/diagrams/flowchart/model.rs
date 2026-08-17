use indexmap::IndexMap;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowchartV2Model {
    #[serde(default, rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default, rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(default, rename = "classDefs")]
    pub class_defs: IndexMap<String, Vec<String>>,
    #[serde(default)]
    pub direction: Option<String>,
    #[serde(default, rename = "edgeDefaults")]
    pub edge_defaults: Option<FlowEdgeDefaults>,
    #[serde(default, rename = "vertexCalls")]
    pub vertex_calls: Vec<String>,
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
    #[serde(default)]
    pub subgraphs: Vec<FlowSubgraph>,
    #[serde(default)]
    pub tooltips: FxHashMap<String, String>,
}

impl FlowchartV2Model {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdgeDefaults {
    #[serde(default)]
    pub interpolate: Option<String>,
    #[serde(default)]
    pub style: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowNode {
    pub id: String,
    pub label: Option<String>,
    #[serde(default, rename = "labelType")]
    pub label_type: Option<String>,
    #[serde(rename = "layoutShape")]
    pub layout_shape: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub form: Option<String>,
    #[serde(default)]
    pub pos: Option<String>,
    #[serde(default)]
    pub img: Option<String>,
    #[serde(default)]
    pub constraint: Option<String>,
    #[serde(default, rename = "assetWidth")]
    pub asset_width: Option<f64>,
    #[serde(default, rename = "assetHeight")]
    pub asset_height: Option<f64>,
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default, rename = "linkTarget")]
    pub link_target: Option<String>,
    #[serde(default, rename = "haveCallback")]
    pub have_callback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub label: Option<String>,
    #[serde(default, rename = "labelType")]
    pub label_type: Option<String>,
    #[serde(default, rename = "type")]
    pub edge_type: Option<String>,
    #[serde(default)]
    pub stroke: Option<String>,
    #[serde(default)]
    pub interpolate: Option<String>,
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub style: Vec<String>,
    #[serde(default)]
    pub animate: Option<bool>,
    #[serde(default)]
    pub animation: Option<String>,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSubgraph {
    pub id: String,
    pub title: String,
    pub dir: Option<String>,
    #[serde(default, rename = "labelType")]
    pub label_type: Option<String>,
    #[serde(default)]
    pub classes: Vec<String>,
    #[serde(default)]
    pub styles: Vec<String>,
    pub nodes: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub id: String,
    pub label: Option<String>,
    pub label_type: TitleKind,
    pub shape: Option<String>,
    pub shape_data: Option<String>,
    pub icon: Option<String>,
    pub form: Option<String>,
    pub pos: Option<String>,
    pub img: Option<String>,
    pub constraint: Option<String>,
    pub asset_width: Option<f64>,
    pub asset_height: Option<f64>,
    pub styles: Vec<String>,
    pub classes: Vec<String>,
    pub link: Option<String>,
    pub link_target: Option<String>,
    pub have_callback: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct Edge {
    pub from: String,
    pub to: String,
    pub id: Option<String>,
    pub link: LinkToken,
    pub label: Option<String>,
    pub label_type: TitleKind,
    pub style: Vec<String>,
    pub classes: Vec<String>,
    pub interpolate: Option<String>,
    pub is_user_defined_id: bool,
    pub animate: Option<bool>,
    pub animation: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct LinkToken {
    pub end: String,
    pub edge_type: String,
    pub stroke: String,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct EdgeDefaults {
    pub style: Vec<String>,
    pub interpolate: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TitleKind {
    Text,
    String,
    Markdown,
}

#[derive(Debug, Clone)]
pub(crate) struct LabeledText {
    pub text: String,
    pub kind: TitleKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SubgraphHeader {
    pub raw_id: String,
    pub raw_title: String,
    pub title_kind: TitleKind,
    pub id_equals_title: bool,
}

impl Default for SubgraphHeader {
    fn default() -> Self {
        Self {
            raw_id: String::new(),
            raw_title: String::new(),
            title_kind: TitleKind::Text,
            id_equals_title: true,
        }
    }
}
