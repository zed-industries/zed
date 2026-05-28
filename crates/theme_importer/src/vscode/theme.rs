use serde::Deserialize;
use vscode_theme::Colors;

use crate::vscode::VsCodeTokenColor;

#[derive(Deserialize, Debug)]
pub struct VsCodeTheme {
    #[serde(rename = "$schema")]
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub schema: Option<String>,
    pub name: Option<String>,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub author: Option<String>,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub maintainers: Option<Vec<String>>,
    #[serde(rename = "semanticClass")]
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    pub semantic_class: Option<String>,
    #[expect(
        unused,
        reason = "This field was found to be unused with serde library bump; it's left as is due to insufficient context on PO's side, but it *may* be fine to remove"
    )]
    #[serde(rename = "semanticHighlighting")]
    pub semantic_highlighting: Option<bool>,
    pub colors: Colors,
    #[serde(rename = "tokenColors")]
    pub token_colors: Vec<VsCodeTokenColor>,
}
