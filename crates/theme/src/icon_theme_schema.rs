#![allow(missing_docs)]

use gpui::SharedString;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IconAppearanceContent {
    Light,
    Dark,
    Monochrome,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IconThemeFamilyContent {
    pub name: String,
    pub author: String,
    pub themes: Vec<IconThemeContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IconThemeContent {
    pub name: String,
    pub appearance: IconAppearanceContent,
    #[serde(default)]
    pub directory_icons: DirectoryIconsContent,
    #[serde(default)]
    pub named_directory_icons: HashMap<String, DirectoryIconsContent>,
    #[serde(default)]
    pub chevron_icons: ChevronIconsContent,
    #[serde(default)]
    pub file_stems: HashMap<String, String>,
    #[serde(default)]
    pub file_suffixes: HashMap<String, String>,
    #[serde(default)]
    pub file_icons: HashMap<String, IconDefinitionContent>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct DirectoryIconsContent {
    pub collapsed: Option<SharedString>,
    pub expanded: Option<SharedString>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ChevronIconsContent {
    pub collapsed: Option<SharedString>,
    pub expanded: Option<SharedString>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IconDefinitionContent {
    pub path: SharedString,
}
