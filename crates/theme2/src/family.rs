use crate::{Appearance, ColorScales};

pub struct ThemeFamily {
    name: String,
    author: String,
    themes: Vec<ThemeVariant>,
    scales: ColorScales,
}

impl ThemeFamily {}

pub struct ThemeVariant {
    name: String,
    appearance: Appearance,
    colors: ThemeColors,
}
