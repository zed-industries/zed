use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn rose_pine_moon() -> FabricTheme {
    FabricTheme {
        name: "Rosé Pine Moon".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x28253c),
                border: rgb(0x322f48),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x85819e),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x322f48),
                border: rgb(0x322f48),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x85819e),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3e3b55),
                border: rgb(0x322f48),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x85819e),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x4f4b66),
                border: rgb(0x4d4965),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0xe0def4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x28253c),
                border: rgb(0x2a273e),
                foreground: rgb(0x3a3653),
                secondary_foreground: Some(
                    rgb(0x3a3653),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xe0def4),
                border: rgb(0x232136),
                foreground: rgb(0x4a4661),
                secondary_foreground: Some(
                    rgb(0x4a4661),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x28253c),
                border: rgb(0x322f48),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x322f48),
                border: rgb(0x322f48),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3e3b55),
                border: rgb(0x322f48),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x4f4b66),
                border: rgb(0x4d4965),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x28253c),
                border: rgb(0x2a273e),
                foreground: rgb(0x3a3653),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xe0def4),
                border: rgb(0x232136),
                foreground: rgb(0x4a4661),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x38354e),
                border: rgb(0x504c68),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x85819e),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x504c68),
                border: rgb(0x504c68),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x85819e),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x524e6a),
                border: rgb(0x504c68),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0x85819e),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x45415d),
                border: rgb(0x3f3b58),
                foreground: rgb(0xe0def4),
                secondary_foreground: Some(
                    rgb(0xe0def4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x38354e),
                border: rgb(0x44415b),
                foreground: rgb(0x615d7a),
                secondary_foreground: Some(
                    rgb(0x615d7a),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xe0def4),
                border: rgb(0x232136),
                foreground: rgb(0x484461),
                secondary_foreground: Some(
                    rgb(0x484461),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x232136),
                border: rgb(0x27243b),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x27243b),
                border: rgb(0x27243b),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x29263d),
                border: rgb(0x27243b),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x312e47),
                border: rgb(0x44415b),
                foreground: rgb(0xe0def4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x232136),
                border: rgb(0x252338),
                foreground: rgb(0x524e6a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xe0def4),
                border: rgb(0x232136),
                foreground: rgb(0x2d2a42),
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
