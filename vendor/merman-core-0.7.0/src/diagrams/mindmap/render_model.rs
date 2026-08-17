use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MindmapDiagramRenderModel {
    #[serde(default)]
    pub nodes: Vec<MindmapDiagramRenderNode>,
    #[serde(default)]
    pub edges: Vec<MindmapDiagramRenderEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapDiagramRenderNode {
    pub id: String,
    #[serde(rename = "domId")]
    pub dom_id: String,
    pub label: String,
    #[serde(default, rename = "labelType")]
    pub label_type: String,
    #[serde(default, rename = "isGroup")]
    pub is_group: bool,
    pub shape: String,
    #[serde(default)]
    pub width: f64,
    #[serde(default)]
    pub height: f64,
    #[serde(default)]
    pub padding: f64,
    #[serde(rename = "cssClasses")]
    pub css_classes: String,
    #[serde(default, rename = "cssStyles")]
    pub css_styles: Vec<String>,
    #[serde(default)]
    pub look: String,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    #[serde(default)]
    pub level: i64,
    #[serde(default, rename = "nodeId")]
    pub node_id: String,
    #[serde(default, rename = "type")]
    pub node_type: i32,
    #[serde(default)]
    pub section: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MindmapDiagramRenderEdge {
    pub id: String,
    pub start: String,
    pub end: String,
    #[serde(default, rename = "type")]
    pub edge_type: String,
    #[serde(default)]
    pub curve: String,
    #[serde(default)]
    pub thickness: String,
    #[serde(default)]
    pub look: String,
    #[serde(default)]
    pub classes: String,
    #[serde(default)]
    pub depth: i64,
    #[serde(default)]
    pub section: Option<i32>,
}
