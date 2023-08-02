use crate::color::{rgb, Rgba};

#[derive(Clone, Copy, Debug)]
pub struct ThemeColors {
    pub base: Rgba,
    pub surface: Rgba,
    pub overlay: Rgba,
    pub muted: Rgba,
    pub subtle: Rgba,
    pub text: Rgba,
    pub love: Rgba,
    pub gold: Rgba,
    pub rose: Rgba,
    pub pine: Rgba,
    pub foam: Rgba,
    pub iris: Rgba,
    pub highlight_low: Rgba,
    pub highlight_med: Rgba,
    pub highlight_high: Rgba,
}

pub struct RosePineThemes {
    pub default: ThemeColors,
    pub dawn: ThemeColors,
    pub moon: ThemeColors,
}

pub fn default() -> ThemeColors {
    ThemeColors {
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

pub fn moon() -> ThemeColors {
    ThemeColors {
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

pub fn dawn() -> ThemeColors {
    ThemeColors {
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
