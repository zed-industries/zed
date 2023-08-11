use std::ops::Range;

use crate::{
    color::{hsla, rgb, Hsla},
    ThemeColors,
};

pub struct RosePineThemes {
    pub default: RosePinePalette,
    pub dawn: RosePinePalette,
    pub moon: RosePinePalette,
}

#[derive(Clone, Copy, Debug)]
pub struct RosePinePalette {
    pub base: Hsla,
    pub surface: Hsla,
    pub overlay: Hsla,
    pub muted: Hsla,
    pub subtle: Hsla,
    pub text: Hsla,
    pub love: Hsla,
    pub gold: Hsla,
    pub rose: Hsla,
    pub pine: Hsla,
    pub foam: Hsla,
    pub iris: Hsla,
    pub highlight_low: Hsla,
    pub highlight_med: Hsla,
    pub highlight_high: Hsla,
}

impl RosePinePalette {
    pub fn default() -> RosePinePalette {
        RosePinePalette {
            base: rgb(0x191724),
            surface: rgb(0x1f1d2e),
            overlay: rgb(0x26233a),
            muted: rgb(0x6e6a86),
            subtle: rgb(0x908caa),
            text: rgb(0xe0def4),
            love: rgb(0xeb6f92),
            gold: rgb(0xf6c177),
            rose: rgb(0xebbcba),
            pine: rgb(0x31748f),
            foam: rgb(0x9ccfd8),
            iris: rgb(0xc4a7e7),
            highlight_low: rgb(0x21202e),
            highlight_med: rgb(0x403d52),
            highlight_high: rgb(0x524f67),
        }
    }

    pub fn moon() -> RosePinePalette {
        RosePinePalette {
            base: rgb(0x232136),
            surface: rgb(0x2a273f),
            overlay: rgb(0x393552),
            muted: rgb(0x6e6a86),
            subtle: rgb(0x908caa),
            text: rgb(0xe0def4),
            love: rgb(0xeb6f92),
            gold: rgb(0xf6c177),
            rose: rgb(0xea9a97),
            pine: rgb(0x3e8fb0),
            foam: rgb(0x9ccfd8),
            iris: rgb(0xc4a7e7),
            highlight_low: rgb(0x2a283e),
            highlight_med: rgb(0x44415a),
            highlight_high: rgb(0x56526e),
        }
    }

    pub fn dawn() -> RosePinePalette {
        RosePinePalette {
            base: rgb(0xfaf4ed),
            surface: rgb(0xfffaf3),
            overlay: rgb(0xf2e9e1),
            muted: rgb(0x9893a5),
            subtle: rgb(0x797593),
            text: rgb(0x575279),
            love: rgb(0xb4637a),
            gold: rgb(0xea9d34),
            rose: rgb(0xd7827e),
            pine: rgb(0x286983),
            foam: rgb(0x56949f),
            iris: rgb(0x907aa9),
            highlight_low: rgb(0xf4ede8),
            highlight_med: rgb(0xdfdad9),
            highlight_high: rgb(0xcecacd),
        }
    }
}

pub fn default() -> ThemeColors {
    theme_colors(&RosePinePalette::default())
}

pub fn moon() -> ThemeColors {
    theme_colors(&RosePinePalette::moon())
}

pub fn dawn() -> ThemeColors {
    theme_colors(&RosePinePalette::dawn())
}

fn theme_colors(p: &RosePinePalette) -> ThemeColors {
    ThemeColors {
        base: scale_sl(p.base, (0.8, 0.8), (1.2, 1.2)),
        surface: scale_sl(p.surface, (0.8, 0.8), (1.2, 1.2)),
        overlay: scale_sl(p.overlay, (0.8, 0.8), (1.2, 1.2)),
        muted: scale_sl(p.muted, (0.8, 0.8), (1.2, 1.2)),
        subtle: scale_sl(p.subtle, (0.8, 0.8), (1.2, 1.2)),
        text: scale_sl(p.text, (0.8, 0.8), (1.2, 1.2)),
        highlight_low: scale_sl(p.highlight_low, (0.8, 0.8), (1.2, 1.2)),
        highlight_med: scale_sl(p.highlight_med, (0.8, 0.8), (1.2, 1.2)),
        highlight_high: scale_sl(p.highlight_high, (0.8, 0.8), (1.2, 1.2)),
        success: scale_sl(p.foam, (0.8, 0.8), (1.2, 1.2)),
        warning: scale_sl(p.gold, (0.8, 0.8), (1.2, 1.2)),
        error: scale_sl(p.love, (0.8, 0.8), (1.2, 1.2)),
        inserted: scale_sl(p.foam, (0.8, 0.8), (1.2, 1.2)),
        deleted: scale_sl(p.love, (0.8, 0.8), (1.2, 1.2)),
        modified: scale_sl(p.rose, (0.8, 0.8), (1.2, 1.2)),
    }
}

/// Produces a range by multiplying the saturation and lightness of the base color by the given
/// start and end factors.
fn scale_sl(base: Hsla, (start_s, start_l): (f32, f32), (end_s, end_l): (f32, f32)) -> Range<Hsla> {
    let start = hsla(base.h, base.s * start_s, base.l * start_l, base.a);
    let end = hsla(base.h, base.s * end_s, base.l * end_l, base.a);
    Range { start, end }
}
