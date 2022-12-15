use gpui::{
    elements::{ContainerStyle, TooltipStyle},
    fonts::TextStyle,
};
use serde::Deserialize;

use crate::{ContainedText, Interactive};

#[derive(Clone, Deserialize, Default)]
pub struct LabelButton {
    pub label: String, // < or >
    pub tooltip_text: String,
    pub tooltip_style: TooltipStyle, //TODO: Make optional
    pub interactions: Interactive<ContainedText>,
    pub text: TextStyle,
    pub container: ContainerStyle,
}
