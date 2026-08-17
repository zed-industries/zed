use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

fn default_state_direction() -> String {
    "TB".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StateDiagramRenderModel {
    #[serde(default = "default_state_direction")]
    pub direction: String,
    #[serde(default, rename = "accTitle")]
    pub acc_title: Option<String>,
    #[serde(default, rename = "accDescr")]
    pub acc_descr: Option<String>,
    #[serde(default)]
    pub nodes: Vec<StateDiagramRenderNode>,
    #[serde(default)]
    pub edges: Vec<StateDiagramRenderEdge>,
    #[serde(default)]
    pub links: HashMap<String, StateDiagramRenderLinks>,
    #[serde(default)]
    pub states: HashMap<String, StateDiagramRenderState>,
    #[serde(default, rename = "styleClasses")]
    pub style_classes: IndexMap<String, StateDiagramRenderStyleClass>,
}

impl StateDiagramRenderModel {
    pub(crate) fn sanitize_common_db_fields(&mut self, config: &crate::MermaidConfig) {
        crate::common_db::sanitize_optional_acc_title(&mut self.acc_title, config);
        crate::common_db::sanitize_optional_acc_descr(&mut self.acc_descr, config);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateDiagramRenderStyleClass {
    pub id: String,
    #[serde(default)]
    pub styles: Vec<String>,
    #[serde(default, rename = "textStyles")]
    pub text_styles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StateDiagramRenderState {
    #[serde(default)]
    pub note: Option<StateDiagramRenderNote>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StateDiagramRenderNote {
    #[serde(default)]
    pub position: Option<String>,
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StateDiagramRenderLink {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub tooltip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StateDiagramRenderLinks {
    One(StateDiagramRenderLink),
    Many(Vec<StateDiagramRenderLink>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateDiagramRenderNode {
    pub id: String,
    #[serde(default, rename = "labelStyle")]
    pub label_style: String,
    #[serde(default)]
    pub label: Option<Value>,
    #[serde(default)]
    pub description: Option<Vec<String>>,
    #[serde(default, rename = "domId")]
    pub dom_id: String,
    #[serde(default, rename = "isGroup")]
    pub is_group: bool,
    #[serde(default, rename = "parentId")]
    pub parent_id: Option<String>,
    #[serde(default, rename = "cssClasses")]
    pub css_classes: String,
    #[serde(default, rename = "cssCompiledStyles")]
    pub css_compiled_styles: Vec<String>,
    #[serde(default, rename = "cssStyles")]
    pub css_styles: Vec<String>,
    #[serde(default)]
    pub dir: Option<String>,
    #[serde(default)]
    pub padding: Option<f64>,
    #[serde(default)]
    pub rx: Option<f64>,
    #[serde(default)]
    pub ry: Option<f64>,
    pub shape: String,
    #[serde(default)]
    pub position: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StateDiagramRenderEdge {
    pub id: String,
    pub start: String,
    pub end: String,
    #[serde(default)]
    pub classes: String,
    #[serde(default, rename = "arrowTypeEnd")]
    pub arrow_type_end: String,
    #[serde(default)]
    pub label: String,
}
