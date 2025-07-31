use serde::Deserialize;
use vscode_theme::Colors;

use crate::vscode::VsCodeTokenColor;

#[derive(Deserialize, Debug)]
pub struct VsCodeTheme {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub maintainers: Option<Vec<String>>,
    #[serde(rename = "semanticClass")]
    pub semantic_class: Option<String>,
    #[serde(rename = "semanticHighlighting")]
    pub semantic_highlighting: Option<bool>,
    pub colors: Colors,
    #[serde(rename = "tokenColors")]
    pub token_colors: Vec<VsCodeTokenColor>,
}
