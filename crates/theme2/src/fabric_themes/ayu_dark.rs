use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn ayu_dark() -> FabricTheme {
    FabricTheme {
        name: "Ayu Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1f2127),
                border: rgb(0x2d2f34),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: Some(
                    rgb(0x8a8986),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2d2f34),
                border: rgb(0x2d2f34),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: Some(
                    rgb(0x8a8986),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x35363a),
                border: rgb(0x2d2f34),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: Some(
                    rgb(0x8a8986),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x3e4043),
                border: rgb(0x494b4d),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: Some(
                    rgb(0xbfbdb6),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1f2127),
                border: rgb(0x26282d),
                foreground: rgb(0x58595a),
                secondary_foreground: Some(
                    rgb(0x58595a),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xbfbdb6),
                border: rgb(0xd1017),
                foreground: rgb(0x3b3d40),
                secondary_foreground: Some(
                    rgb(0x3b3d40),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1f2127),
                border: rgb(0x2d2f34),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2d2f34),
                border: rgb(0x2d2f34),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x35363a),
                border: rgb(0x2d2f34),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x3e4043),
                border: rgb(0x494b4d),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1f2127),
                border: rgb(0x26282d),
                foreground: rgb(0x58595a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xbfbdb6),
                border: rgb(0xd1017),
                foreground: rgb(0x3b3d40),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x313337),
                border: rgb(0x3f4043),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: Some(
                    rgb(0x8a8986),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3f4043),
                border: rgb(0x3f4043),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: Some(
                    rgb(0x8a8986),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x46474a),
                border: rgb(0x3f4043),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: Some(
                    rgb(0x8a8986),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x505152),
                border: rgb(0x5b5c5d),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: Some(
                    rgb(0xbfbdb6),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x313337),
                border: rgb(0x383a3e),
                foreground: rgb(0x696a6a),
                secondary_foreground: Some(
                    rgb(0x696a6a),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xbfbdb6),
                border: rgb(0xd1017),
                foreground: rgb(0x4d4e50),
                secondary_foreground: Some(
                    rgb(0x4d4e50),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xd1017),
                border: rgb(0x1b1e24),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1b1e24),
                border: rgb(0x1b1e24),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x23252a),
                border: rgb(0x1b1e24),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2c2e33),
                border: rgb(0x383a3e),
                foreground: rgb(0xbfbdb6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xd1017),
                border: rgb(0x14171d),
                foreground: rgb(0x46474a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xbfbdb6),
                border: rgb(0xd1017),
                foreground: rgb(0x2a2c31),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xd304f),
                border: rgb(0x1b4a6e),
                foreground: rgb(0x5ac2fe),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1b4a6e),
                border: rgb(0x1b4a6e),
                foreground: rgb(0x5ac2fe),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x23587f),
                border: rgb(0x1b4a6e),
                foreground: rgb(0x5ac2fe),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2d6c98),
                border: rgb(0x3984b4),
                foreground: rgb(0xfafcff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xd304f),
                border: rgb(0x143d5e),
                foreground: rgb(0x49a2d9),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfafcff),
                border: rgb(0x1129),
                foreground: rgb(0x296691),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x294113),
                border: rgb(0x405c1d),
                foreground: rgb(0xaad84c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x405c1d),
                border: rgb(0x405c1d),
                foreground: rgb(0xaad84c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4c6b23),
                border: rgb(0x405c1d),
                foreground: rgb(0xaad84c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x5e802a),
                border: rgb(0x739934),
                foreground: rgb(0xfcfdf8),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x294113),
                border: rgb(0x354f18),
                foreground: rgb(0x8eb840),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfcfdf8),
                border: rgb(0x102100),
                foreground: rgb(0x597a28),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x572916),
                border: rgb(0x754221),
                foreground: rgb(0xfeb454),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x754221),
                border: rgb(0x754221),
                foreground: rgb(0xfeb454),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x864f27),
                border: rgb(0x754221),
                foreground: rgb(0xfeb454),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9d6230),
                border: rgb(0xb8793a),
                foreground: rgb(0xfffcf8),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x572916),
                border: rgb(0x65361b),
                foreground: rgb(0xdb9647),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfffcf8),
                border: rgb(0x330a00),
                foreground: rgb(0x965d2d),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x48171c),
                border: rgb(0x66272d),
                foreground: rgb(0xef7178),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x66272d),
                border: rgb(0x66272d),
                foreground: rgb(0xef7178),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x773035),
                border: rgb(0x66272d),
                foreground: rgb(0xef7178),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x8d3c42),
                border: rgb(0xa94a50),
                foreground: rgb(0xfff9f9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x48171c),
                border: rgb(0x571f24),
                foreground: rgb(0xcc5d64),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff9f9),
                border: rgb(0x270000),
                foreground: rgb(0x87383e),
                secondary_foreground: None,
            },
        },
    }
}
