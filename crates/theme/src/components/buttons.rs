use gpui::color::Color;
use serde::Deserialize;

use crate::{ContainedText, Interactive};

#[derive(Clone, Deserialize, Default)]
pub struct Icon {
    pub location: String,
    pub size: f32,
    pub color: Color,
}

#[derive(Clone, Deserialize, Default)]
pub struct ButtonStyle {
    pub label: Option<String>,
    pub icon: Option<Icon>,
    pub tooltip_text: String,
    pub container: Interactive<ContainedText>,
}
