use gpui::color::Color;
use gpui::elements::{ContainerStyle, LabelStyle};
use gpui::fonts::Properties as FontProperties;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Theme {
    pub ui: Ui,
    pub editor: Editor,
    pub syntax: Vec<(String, Color, FontProperties)>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Ui {
    pub background: Color,
    pub tab: Tab,
    pub active_tab: Tab,
    pub selector: Selector,
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
