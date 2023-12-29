use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn sandcastle() -> FabricTheme {
    FabricTheme {
        name: "Sandcastle".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2b3039),
                border: rgb(0x313741),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: Some(
                    rgb(0xa69782),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x313741),
                border: rgb(0x313741),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: Some(
                    rgb(0xa69782),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x363c47),
                border: rgb(0x313741),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: Some(
                    rgb(0xa69782),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x3d4350),
                border: rgb(0x4d4d52),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: Some(
                    rgb(0xfdf4c1),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2b3039),
                border: rgb(0x2c323b),
                foreground: rgb(0x645b54),
                secondary_foreground: Some(
                    rgb(0x645b54),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf4c1),
                border: rgb(0x282c34),
                foreground: rgb(0x3b414d),
                secondary_foreground: Some(
                    rgb(0x3b414d),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2b3039),
                border: rgb(0x313741),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x313741),
                border: rgb(0x313741),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x363c47),
                border: rgb(0x313741),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x3d4350),
                border: rgb(0x4d4d52),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2b3039),
                border: rgb(0x2c323b),
                foreground: rgb(0x645b54),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf4c1),
                border: rgb(0x282c34),
                foreground: rgb(0x3b414d),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x333944),
                border: rgb(0x3d4350),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: Some(
                    rgb(0xa69782),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3d4350),
                border: rgb(0x3d4350),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: Some(
                    rgb(0xa69782),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x484a52),
                border: rgb(0x3d4350),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: Some(
                    rgb(0xa69782),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x575353),
                border: rgb(0x6a5f57),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: Some(
                    rgb(0xfdf4c1),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x333944),
                border: rgb(0x393f4a),
                foreground: rgb(0x827568),
                secondary_foreground: Some(
                    rgb(0x827568),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf4c1),
                border: rgb(0x282c34),
                foreground: rgb(0x535053),
                secondary_foreground: Some(
                    rgb(0x535053),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x282c34),
                border: rgb(0x2a2f38),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2a2f38),
                border: rgb(0x2a2f38),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2b313a),
                border: rgb(0x2a2f38),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x303640),
                border: rgb(0x393f4a),
                foreground: rgb(0xfdf4c1),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x282c34),
                border: rgb(0x292e36),
                foreground: rgb(0x484a52),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf4c1),
                border: rgb(0x282c34),
                foreground: rgb(0x2e343e),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x171f1f),
                border: rgb(0x223232),
                foreground: rgb(0x528b8b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x223232),
                border: rgb(0x223232),
                foreground: rgb(0x528b8b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x283c3c),
                border: rgb(0x223232),
                foreground: rgb(0x528b8b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x304b4b),
                border: rgb(0x395d5d),
                foreground: rgb(0xf8fafa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x171f1f),
                border: rgb(0x1c2929),
                foreground: rgb(0x467474),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8fafa),
                border: rgb(0x0000),
                foreground: rgb(0x2d4747),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
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
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x231d12),
                border: rgb(0x392e1a),
                foreground: rgb(0xa07e3b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x392e1a),
                border: rgb(0x392e1a),
                foreground: rgb(0xa07e3b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x45371f),
                border: rgb(0x392e1a),
                foreground: rgb(0xa07e3b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x564524),
                border: rgb(0x6b542b),
                foreground: rgb(0xfcf9f7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x231d12),
                border: rgb(0x2e2516),
                foreground: rgb(0x856933),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfcf9f7),
                border: rgb(0x0000),
                foreground: rgb(0x514023),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x26191c),
                border: rgb(0x3f272d),
                foreground: rgb(0xb4637a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3f272d),
                border: rgb(0x3f272d),
                foreground: rgb(0xb4637a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4c2e36),
                border: rgb(0x3f272d),
                foreground: rgb(0xb4637a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x603843),
                border: rgb(0x774352),
                foreground: rgb(0xfcf8f9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x26191c),
                border: rgb(0x322025),
                foreground: rgb(0x955366),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfcf8f9),
                border: rgb(0x0000),
                foreground: rgb(0x5a343f),
                secondary_foreground: None,
            },
        },
    }
}
