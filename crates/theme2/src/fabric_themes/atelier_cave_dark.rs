use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_cave_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Cave Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x221f26),
                border: rgb(0x332f38),
                foreground: rgb(0xefecf4),
                secondary_foreground: Some(
                    rgb(0x898591),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x332f38),
                border: rgb(0x332f38),
                foreground: rgb(0xefecf4),
                secondary_foreground: Some(
                    rgb(0x898591),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x413c47),
                border: rgb(0x332f38),
                foreground: rgb(0xefecf4),
                secondary_foreground: Some(
                    rgb(0x898591),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x544f5c),
                border: rgb(0x5d5765),
                foreground: rgb(0xefecf4),
                secondary_foreground: Some(
                    rgb(0xefecf4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x221f26),
                border: rgb(0x26232a),
                foreground: rgb(0x655f6d),
                secondary_foreground: Some(
                    rgb(0x655f6d),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xefecf4),
                border: rgb(0x19171c),
                foreground: rgb(0x4f4956),
                secondary_foreground: Some(
                    rgb(0x4f4956),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x221f26),
                border: rgb(0x332f38),
                foreground: rgb(0xefecf4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x332f38),
                border: rgb(0x332f38),
                foreground: rgb(0xefecf4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x413c47),
                border: rgb(0x332f38),
                foreground: rgb(0xefecf4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x544f5c),
                border: rgb(0x5d5765),
                foreground: rgb(0xefecf4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x221f26),
                border: rgb(0x26232a),
                foreground: rgb(0x655f6d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xefecf4),
                border: rgb(0x19171c),
                foreground: rgb(0x4f4956),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3a353f),
                border: rgb(0x56505e),
                foreground: rgb(0xefecf4),
                secondary_foreground: Some(
                    rgb(0x898591),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x56505e),
                border: rgb(0x56505e),
                foreground: rgb(0xefecf4),
                secondary_foreground: Some(
                    rgb(0x898591),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x5b5563),
                border: rgb(0x56505e),
                foreground: rgb(0xefecf4),
                secondary_foreground: Some(
                    rgb(0x898591),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x605a68),
                border: rgb(0x67616f),
                foreground: rgb(0xefecf4),
                secondary_foreground: Some(
                    rgb(0xefecf4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3a353f),
                border: rgb(0x48434f),
                foreground: rgb(0x756f7e),
                secondary_foreground: Some(
                    rgb(0x756f7e),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xefecf4),
                border: rgb(0x19171c),
                foreground: rgb(0x5f5967),
                secondary_foreground: Some(
                    rgb(0x5f5967),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x19171c),
                border: rgb(0x201e24),
                foreground: rgb(0xefecf4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x201e24),
                border: rgb(0x201e24),
                foreground: rgb(0xefecf4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x242128),
                border: rgb(0x201e24),
                foreground: rgb(0xefecf4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x322e37),
                border: rgb(0x48434f),
                foreground: rgb(0xefecf4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x19171c),
                border: rgb(0x1d1a20),
                foreground: rgb(0x5b5563),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xefecf4),
                border: rgb(0x19171c),
                foreground: rgb(0x2c2930),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x161a36),
                border: rgb(0x222953),
                foreground: rgb(0x576dda),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x222953),
                border: rgb(0x222953),
                foreground: rgb(0x576dda),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x293162),
                border: rgb(0x222953),
                foreground: rgb(0x576dda),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x313d79),
                border: rgb(0x3c4994),
                foreground: rgb(0xf9f9fe),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x161a36),
                border: rgb(0x1c2244),
                foreground: rgb(0x495bb7),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf9f9fe),
                border: rgb(0x0014),
                foreground: rgb(0x2e3873),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x132020),
                border: rgb(0x1a3434),
                foreground: rgb(0x2c9292),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1a3434),
                border: rgb(0x1a3434),
                foreground: rgb(0x2c9292),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1d3f3f),
                border: rgb(0x1a3434),
                foreground: rgb(0x2c9292),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x204f4f),
                border: rgb(0x246161),
                foreground: rgb(0xf7fafa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x132020),
                border: rgb(0x172a2a),
                foreground: rgb(0x287979),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7fafa),
                border: rgb(0x0000),
                foreground: rgb(0x204a4a),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x231a12),
                border: rgb(0x392a1a),
                foreground: rgb(0xa06e3b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x392a1a),
                border: rgb(0x392a1a),
                foreground: rgb(0xa06e3b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x45321e),
                border: rgb(0x392a1a),
                foreground: rgb(0xa06e3b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x563d23),
                border: rgb(0x6b4a2b),
                foreground: rgb(0xfcf9f6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x231a12),
                border: rgb(0x2e2216),
                foreground: rgb(0x855c33),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfcf9f6),
                border: rgb(0x0000),
                foreground: rgb(0x513922),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x28151c),
                border: rgb(0x421f2d),
                foreground: rgb(0xbe4678),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x421f2d),
                border: rgb(0x421f2d),
                foreground: rgb(0xbe4678),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x512435),
                border: rgb(0x421f2d),
                foreground: rgb(0xbe4678),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x652b42),
                border: rgb(0x7e3350),
                foreground: rgb(0xfdf8f9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x28151c),
                border: rgb(0x351a24),
                foreground: rgb(0x9d3d64),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf8f9),
                border: rgb(0x0000),
                foreground: rgb(0x5f293e),
                secondary_foreground: None,
            },
        },
    }
}
