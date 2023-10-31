use crate::{colors::ThemeStyle, Appearance, ColorScales};

pub struct ThemeFamily {
    pub name: String,
    pub author: String,
    pub themes: Vec<ThemeVariant>,
    pub scales: ColorScales,
}

impl ThemeFamily {}

pub struct ThemeVariant {
    pub name: String,
    pub appearance: Appearance,
    pub styles: ThemeStyle,
}
