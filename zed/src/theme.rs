mod highlight_map;
mod theme_registry;

use anyhow::Result;
use gpui::{
    color::Color,
    elements::{ContainerStyle, LabelStyle},
    fonts::TextStyle,
};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

pub use highlight_map::*;
pub use theme_registry::*;

pub const DEFAULT_THEME_NAME: &'static str = "dark";

#[derive(Debug, Default, Deserialize)]
pub struct Theme {
    #[serde(default)]
    pub name: String,
    pub workspace: Workspace,
    pub tab: Tab,
    pub active_tab: Tab,
    pub sidebar: ContainerStyle,
    pub sidebar_icon: SidebarIcon,
    pub active_sidebar_icon: SidebarIcon,
    pub selector: Selector,
    pub editor: Editor,
    #[serde(deserialize_with = "deserialize_syntax_theme")]
    pub syntax: Vec<(String, TextStyle)>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Workspace {
    pub background: Color,
}

#[derive(Debug, Deserialize)]
pub struct Editor {
    pub background: Color,
    pub gutter_background: Color,
    pub active_line_background: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub text: Color,
    pub replicas: Vec<Replica>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
pub struct Replica {
    pub cursor: Color,
    pub selection: Color,
}

#[derive(Debug, Default, Deserialize)]
pub struct Tab {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
    pub icon_close: Color,
    pub icon_dirty: Color,
    pub icon_conflict: Color,
}

#[derive(Debug, Default, Deserialize)]
pub struct SidebarIcon {
    pub color: Color,
}

#[derive(Debug, Default, Deserialize)]
pub struct Selector {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,

    pub item: SelectorItem,
    pub active_item: SelectorItem,
}

#[derive(Debug, Default, Deserialize)]
pub struct SelectorItem {
    #[serde(flatten)]
    pub container: ContainerStyle,
    #[serde(flatten)]
    pub label: LabelStyle,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            background: Default::default(),
            gutter_background: Default::default(),
            active_line_background: Default::default(),
            line_number: Default::default(),
            line_number_active: Default::default(),
            text: Default::default(),
            replicas: vec![Replica::default()],
        }
    }
}

impl Theme {
    pub fn highlight_style(&self, id: HighlightId) -> TextStyle {
        self.syntax
            .get(id.0 as usize)
            .map(|entry| entry.1.clone())
            .unwrap_or_else(|| TextStyle {
                color: self.editor.text,
                font_properties: Default::default(),
            })
    }

    #[cfg(test)]
    pub fn highlight_name(&self, id: HighlightId) -> Option<&str> {
        self.syntax.get(id.0 as usize).map(|e| e.0.as_str())
    }
}

pub fn deserialize_syntax_theme<'de, D>(
    deserializer: D,
) -> Result<Vec<(String, TextStyle)>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut result = Vec::<(String, TextStyle)>::new();

    let syntax_data: HashMap<String, TextStyle> = Deserialize::deserialize(deserializer)?;
    for (key, style) in syntax_data {
        match result.binary_search_by(|(needle, _)| needle.cmp(&key)) {
            Ok(i) | Err(i) => {
                result.insert(i, (key, style));
            }
        }
    }

    Ok(result)
}
