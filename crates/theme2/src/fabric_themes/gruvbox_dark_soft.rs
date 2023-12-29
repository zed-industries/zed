use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn gruvbox_dark_soft() -> FabricTheme {
    FabricTheme {
        name: "Gruvbox Dark Soft".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3b3735),
                border: rgb(0x494340),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: Some(
                    rgb(0xc5b597),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x494340),
                border: rgb(0x494340),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: Some(
                    rgb(0xc5b597),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x504945),
                border: rgb(0x494340),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: Some(
                    rgb(0xc5b597),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x5b524c),
                border: rgb(0x675d55),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: Some(
                    rgb(0xfbf1c7),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3b3735),
                border: rgb(0x413d3a),
                foreground: rgb(0x776b61),
                secondary_foreground: Some(
                    rgb(0x776b61),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbf1c7),
                border: rgb(0x32302f),
                foreground: rgb(0x574f4a),
                secondary_foreground: Some(
                    rgb(0x574f4a),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3b3735),
                border: rgb(0x494340),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x494340),
                border: rgb(0x494340),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x504945),
                border: rgb(0x494340),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x5b524c),
                border: rgb(0x675d55),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3b3735),
                border: rgb(0x413d3a),
                foreground: rgb(0x776b61),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbf1c7),
                border: rgb(0x32302f),
                foreground: rgb(0x574f4a),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x4c4642),
                border: rgb(0x5b534d),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: Some(
                    rgb(0xc5b597),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x5b534d),
                border: rgb(0x5b534d),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: Some(
                    rgb(0xc5b597),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x635a52),
                border: rgb(0x5b534d),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: Some(
                    rgb(0xc5b597),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x6e635a),
                border: rgb(0x7b6e64),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: Some(
                    rgb(0xfbf1c7),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x4c4642),
                border: rgb(0x544c48),
                foreground: rgb(0x9a8c79),
                secondary_foreground: Some(
                    rgb(0x9a8c79),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbf1c7),
                border: rgb(0x32302f),
                foreground: rgb(0x6b6058),
                secondary_foreground: Some(
                    rgb(0x6b6058),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x32302f),
                border: rgb(0x393634),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x393634),
                border: rgb(0x393634),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3e3a37),
                border: rgb(0x393634),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x48423f),
                border: rgb(0x544c48),
                foreground: rgb(0xfbf1c7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x32302f),
                border: rgb(0x363332),
                foreground: rgb(0x635a52),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbf1c7),
                border: rgb(0x32302f),
                foreground: rgb(0x45403d),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1e2321),
                border: rgb(0x303a36),
                foreground: rgb(0x83a598),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x303a36),
                border: rgb(0x303a36),
                foreground: rgb(0x83a598),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3a4641),
                border: rgb(0x303a36),
                foreground: rgb(0x83a598),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x485852),
                border: rgb(0x586d65),
                foreground: rgb(0xfafbfb),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1e2321),
                border: rgb(0x272f2c),
                foreground: rgb(0x6d887e),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfafbfb),
                border: rgb(0x0000),
                foreground: rgb(0x43534d),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x332b11),
                border: rgb(0x4a4516),
                foreground: rgb(0xb8bb27),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x4a4516),
                border: rgb(0x4a4516),
                foreground: rgb(0xb8bb27),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x575219),
                border: rgb(0x4a4516),
                foreground: rgb(0xb8bb27),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x6a661c),
                border: rgb(0x807e20),
                foreground: rgb(0xfdfcf7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x332b11),
                border: rgb(0x3f3814),
                foreground: rgb(0x9b9b23),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdfcf7),
                border: rgb(0x180b00),
                foreground: rgb(0x64601b),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x582f10),
                border: rgb(0x754916),
                foreground: rgb(0xf9bd30),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x754916),
                border: rgb(0x754916),
                foreground: rgb(0xf9bd30),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x85561a),
                border: rgb(0x754916),
                foreground: rgb(0xf9bd30),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9b6a1e),
                border: rgb(0xb68123),
                foreground: rgb(0xfffcf7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x582f10),
                border: rgb(0x663c13),
                foreground: rgb(0xd79e29),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfffcf7),
                border: rgb(0x351100),
                foreground: rgb(0x95641d),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x5a0a10),
                border: rgb(0x771618),
                foreground: rgb(0xfb4a35),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x771618),
                border: rgb(0x771618),
                foreground: rgb(0xfb4a35),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x871c1b),
                border: rgb(0x771618),
                foreground: rgb(0xfb4a35),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9d251f),
                border: rgb(0xb72f26),
                foreground: rgb(0xfff8f6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x5a0a10),
                border: rgb(0x681014),
                foreground: rgb(0xd83c2d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff8f6),
                border: rgb(0x380000),
                foreground: rgb(0x97221f),
                secondary_foreground: None,
            },
        },
    }
}
