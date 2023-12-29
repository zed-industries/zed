use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn rose_pine() -> FabricTheme {
    FabricTheme {
        name: "Rosé Pine".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1d1b2a),
                border: rgb(0x232132),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x75718e),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x232132),
                border: rgb(0x232132),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x75718e),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2f2d40),
                border: rgb(0x232132),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x75718e),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x403e53),
                border: rgb(0x504d65),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0xe0def4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1d1b2a),
                border: rgb(0x1e1c2c),
                foreground: rgb(0x3b384f),
                secondary_foreground: Some(
                    rgb(0x3b384f),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xe0def4),
                border: rgb(0x191724),
                foreground: rgb(0x3b394e),
                secondary_foreground: Some(
                    rgb(0x3b394e),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1d1b2a),
                border: rgb(0x232132),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x232132),
                border: rgb(0x232132),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2f2d40),
                border: rgb(0x232132),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x403e53),
                border: rgb(0x504d65),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1d1b2a),
                border: rgb(0x1e1c2c),
                foreground: rgb(0x3b384f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xe0def4),
                border: rgb(0x191724),
                foreground: rgb(0x3b394e),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x292739),
                border: rgb(0x423f55),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x75718e),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x423f55),
                border: rgb(0x423f55),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x75718e),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4e4b63),
                border: rgb(0x423f55),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x75718e),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x47445b),
                border: rgb(0x36334a),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0xe0def4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x292739),
                border: rgb(0x353347),
                foreground: rgb(0x2f2b43),
                secondary_foreground: Some(
                    rgb(0x2f2b43),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xe0def4),
                border: rgb(0x191724),
                foreground: rgb(0x4b4860),
                secondary_foreground: Some(
                    rgb(0x4b4860),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x191724),
                border: rgb(0x1c1a29),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1c1a29),
                border: rgb(0x1c1a29),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1d1b2b),
                border: rgb(0x1c1a29),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x222031),
                border: rgb(0x353347),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x191724),
                border: rgb(0x1a1826),
                foreground: rgb(0x4e4b63),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xe0def4),
                border: rgb(0x191724),
                foreground: rgb(0x1f1d2e),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2f3739),
                border: rgb(0x435255),
                foreground: rgb(0x9cced7),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x435255),
                border: rgb(0x435255),
                foreground: rgb(0x9cced7),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4e6164),
                border: rgb(0x435255),
                foreground: rgb(0x9cced7),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x5d757a),
                border: rgb(0x6e8f94),
                foreground: rgb(0xfbfdfd),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2f3739),
                border: rgb(0x3a4446),
                foreground: rgb(0x85aeb5),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbfdfd),
                border: rgb(0x171717),
                foreground: rgb(0x587074),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x182e23),
                border: rgb(0x254839),
                foreground: rgb(0x5dc2a3),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x254839),
                border: rgb(0x254839),
                foreground: rgb(0x5dc2a3),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2c5645),
                border: rgb(0x254839),
                foreground: rgb(0x5dc2a3),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x356b57),
                border: rgb(0x40836c),
                foreground: rgb(0xf9fdfb),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x182e23),
                border: rgb(0x1e3b2e),
                foreground: rgb(0x4ea287),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf9fdfb),
                border: rgb(0x0e00),
                foreground: rgb(0x326552),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x50341a),
                border: rgb(0x6d4d2b),
                foreground: rgb(0xf5c177),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x6d4d2b),
                border: rgb(0x6d4d2b),
                foreground: rgb(0xf5c177),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x7e5a34),
                border: rgb(0x6d4d2b),
                foreground: rgb(0xf5c177),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x946e41),
                border: rgb(0xb0854f),
                foreground: rgb(0xfffcf9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x50341a),
                border: rgb(0x5e4023),
                foreground: rgb(0xd2a263),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfffcf9),
                border: rgb(0x2c1600),
                foreground: rgb(0x8e683c),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x431820),
                border: rgb(0x612834),
                foreground: rgb(0xea6f92),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x612834),
                border: rgb(0x612834),
                foreground: rgb(0xea6f92),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x71303f),
                border: rgb(0x612834),
                foreground: rgb(0xea6f92),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x883c4f),
                border: rgb(0xa44961),
                foreground: rgb(0xfff9fa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x431820),
                border: rgb(0x52202a),
                foreground: rgb(0xc75c79),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff9fa),
                border: rgb(0x230000),
                foreground: rgb(0x82384a),
                secondary_foreground: None,
            },
        },
    }
}
