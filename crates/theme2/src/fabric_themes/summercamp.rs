use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn summercamp() -> FabricTheme {
    FabricTheme {
        name: "Summercamp".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x231f16),
                border: rgb(0x29251b),
                foreground: rgb(0xf8f5de),
                secondary_foreground: Some(
                    rgb(0x736e55),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x29251b),
                border: rgb(0x29251b),
                foreground: rgb(0xf8f5de),
                secondary_foreground: Some(
                    rgb(0x736e55),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2c281d),
                border: rgb(0x29251b),
                foreground: rgb(0xf8f5de),
                secondary_foreground: Some(
                    rgb(0x736e55),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x302c20),
                border: rgb(0x373225),
                foreground: rgb(0xf8f5de),
                secondary_foreground: Some(
                    rgb(0xf8f5de),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x231f16),
                border: rgb(0x262218),
                foreground: rgb(0x3d382a),
                secondary_foreground: Some(
                    rgb(0x3d382a),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8f5de),
                border: rgb(0x1c1810),
                foreground: rgb(0x302b20),
                secondary_foreground: Some(
                    rgb(0x302b20),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x231f16),
                border: rgb(0x29251b),
                foreground: rgb(0xf8f5de),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x29251b),
                border: rgb(0x29251b),
                foreground: rgb(0xf8f5de),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2c281d),
                border: rgb(0x29251b),
                foreground: rgb(0xf8f5de),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x302c20),
                border: rgb(0x373225),
                foreground: rgb(0xf8f5de),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x231f16),
                border: rgb(0x262218),
                foreground: rgb(0x3d382a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8f5de),
                border: rgb(0x1c1810),
                foreground: rgb(0x302b20),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2a261c),
                border: rgb(0x312d21),
                foreground: rgb(0xf8f5de),
                secondary_foreground: Some(
                    rgb(0x736e55),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x312d21),
                border: rgb(0x312d21),
                foreground: rgb(0xf8f5de),
                secondary_foreground: Some(
                    rgb(0x736e55),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x353024),
                border: rgb(0x312d21),
                foreground: rgb(0xf8f5de),
                secondary_foreground: Some(
                    rgb(0x736e55),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x393427),
                border: rgb(0x403b2c),
                foreground: rgb(0xf8f5de),
                secondary_foreground: Some(
                    rgb(0xf8f5de),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2a261c),
                border: rgb(0x2e2a1f),
                foreground: rgb(0x4c4735),
                secondary_foreground: Some(
                    rgb(0x4c4735),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8f5de),
                border: rgb(0x1c1810),
                foreground: rgb(0x393426),
                secondary_foreground: Some(
                    rgb(0x393426),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1c1810),
                border: rgb(0x221e15),
                foreground: rgb(0xf8f5de),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x221e15),
                border: rgb(0x221e15),
                foreground: rgb(0xf8f5de),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x242017),
                border: rgb(0x221e15),
                foreground: rgb(0xf8f5de),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x28241a),
                border: rgb(0x2e2a1f),
                foreground: rgb(0xf8f5de),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1c1810),
                border: rgb(0x1f1b12),
                foreground: rgb(0x353024),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8f5de),
                border: rgb(0x1c1810),
                foreground: rgb(0x27231a),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe2242),
                border: rgb(0x193761),
                foreground: rgb(0x499bef),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x193761),
                border: rgb(0x193761),
                foreground: rgb(0x499bef),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1e4272),
                border: rgb(0x193761),
                foreground: rgb(0x499bef),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x26538a),
                border: rgb(0x2f67a6),
                foreground: rgb(0xf9fbff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe2242),
                border: rgb(0x132d51),
                foreground: rgb(0x3c81ca),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf9fbff),
                border: rgb(0x001e),
                foreground: rgb(0x244e83),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xa4d13),
                border: rgb(0x1a6a20),
                foreground: rgb(0x5dea5a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1a6a20),
                border: rgb(0x1a6a20),
                foreground: rgb(0x5dea5a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x227927),
                border: rgb(0x1a6a20),
                foreground: rgb(0x5dea5a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2d8e31),
                border: rgb(0x3aa83c),
                foreground: rgb(0xfafef8),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xa4d13),
                border: rgb(0x125b1a),
                foreground: rgb(0x4bc94b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfafef8),
                border: rgb(0x2b00),
                foreground: rgb(0x29882d),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x556305),
                border: rgb(0x727f0a),
                foreground: rgb(0xf1fe29),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x727f0a),
                border: rgb(0x727f0a),
                foreground: rgb(0xf1fe29),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x818e0e),
                border: rgb(0x727f0a),
                foreground: rgb(0xf1fe29),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x96a412),
                border: rgb(0xb0be18),
                foreground: rgb(0xfffff8),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x556305),
                border: rgb(0x637107),
                foreground: rgb(0xd0dd20),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfffff8),
                border: rgb(0x334000),
                foreground: rgb(0x909e11),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x491013),
                border: rgb(0x651c1c),
                foreground: rgb(0xe35142),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x651c1c),
                border: rgb(0x651c1c),
                foreground: rgb(0xe35142),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x742221),
                border: rgb(0x651c1c),
                foreground: rgb(0xe35142),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x892b27),
                border: rgb(0xa2352f),
                foreground: rgb(0xfff8f7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x491013),
                border: rgb(0x571618),
                foreground: rgb(0xc24338),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff8f7),
                border: rgb(0x2a0000),
                foreground: rgb(0x832825),
                secondary_foreground: None,
            },
        },
    }
}
