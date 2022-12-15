use gpui::{elements::ContainerStyle, fonts::TextStyle};
use serde::Deserialize;

use crate::{ContainedText, Interactive};

#[derive(Clone, Deserialize, Default)]
pub struct LabelButton {
    pub label: String, // < or >
    pub tooltip_text: String,
    pub interactions: Interactive<ContainedText>,
    pub text: TextStyle,
    pub container: ContainerStyle,
}
