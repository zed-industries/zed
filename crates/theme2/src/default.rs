use std::collections::HashMap;

use gpui2::Rgba;

use crate::scale::{ColorScaleName, ColorScaleSet, ColorScales};

struct DefaultColorScaleSet {
    scale: ColorScaleName,
    light: [&'static str; 12],
    light_alpha: [&'static str; 12],
    dark: [&'static str; 12],
    dark_alpha: [&'static str; 12],
}

impl From<DefaultColorScaleSet> for ColorScaleSet {
    fn from(default: DefaultColorScaleSet) -> Self {
        Self::new(
            default.scale,
            default
                .light
                .map(|color| Rgba::try_from(color).unwrap().into()),
            default
                .light_alpha
                .map(|color| Rgba::try_from(color).unwrap().into()),
            default
                .dark
                .map(|color| Rgba::try_from(color).unwrap().into()),
            default
                .dark_alpha
                .map(|color| Rgba::try_from(color).unwrap().into()),
        )
    }
}

pub fn default_color_scales() -> ColorScales {
    use ColorScaleName::*;

    HashMap::from_iter([(Red, red().into())])
}

fn red() -> DefaultColorScaleSet {
    DefaultColorScaleSet {
        scale: ColorScaleName::Red,
        light: [
            "#fffcfc00",
            "#fff7f700",
            "#feebec00",
            "#ffdbdc00",
            "#ffcdce00",
            "#fdbdbe00",
            "#f4a9aa00",
            "#eb8e9000",
            "#e5484d00",
            "#dc3e4200",
            "#ce2c3100",
            "#64172300",
        ],
        light_alpha: [
            "#ff000003",
            "#ff000008",
            "#f3000d14",
            "#ff000824",
            "#ff000632",
            "#f8000442",
            "#df000356",
            "#d2000571",
            "#db0007b7",
            "#d10005c1",
            "#c40006d3",
            "#55000de8",
        ],
        dark: [
            "#19111100",
            "#20131400",
            "#3b121900",
            "#500f1c00",
            "#61162300",
            "#72232d00",
            "#8c333a00",
            "#b5454800",
            "#e5484d00",
            "#ec5d5e00",
            "#ff959200",
            "#ffd1d900",
        ],
        dark_alpha: [
            "#f4121209",
            "#f22f3e11",
            "#ff173f2d",
            "#fe0a3b44",
            "#ff204756",
            "#ff3e5668",
            "#ff536184",
            "#ff5d61b0",
            "#fe4e54e4",
            "#ff6465eb",
            "#ff959200",
            "#ffd1d900",
        ],
    }
}
