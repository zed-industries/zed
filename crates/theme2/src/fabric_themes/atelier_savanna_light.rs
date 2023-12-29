use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_savanna_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Savanna Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe3ebe6),
                border: rgb(0xc8d1cb),
                foreground: rgb(0x171c19),
                secondary_foreground: Some(
                    rgb(0x546259),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc8d1cb),
                border: rgb(0xc8d1cb),
                foreground: rgb(0x171c19),
                secondary_foreground: Some(
                    rgb(0x546259),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xafb9b2),
                border: rgb(0xc8d1cb),
                foreground: rgb(0x171c19),
                secondary_foreground: Some(
                    rgb(0x546259),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x8d9890),
                border: rgb(0x818e85),
                foreground: rgb(0x171c19),
                secondary_foreground: Some(
                    rgb(0x171c19),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe3ebe6),
                border: rgb(0xdfe7e2),
                foreground: rgb(0x79877d),
                secondary_foreground: Some(
                    rgb(0x79877d),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x171c19),
                border: rgb(0xecf4ee),
                foreground: rgb(0x97a29a),
                secondary_foreground: Some(
                    rgb(0x97a29a),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe3ebe6),
                border: rgb(0xc8d1cb),
                foreground: rgb(0x171c19),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc8d1cb),
                border: rgb(0xc8d1cb),
                foreground: rgb(0x171c19),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xafb9b2),
                border: rgb(0xc8d1cb),
                foreground: rgb(0x171c19),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x8d9890),
                border: rgb(0x818e85),
                foreground: rgb(0x171c19),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe3ebe6),
                border: rgb(0xdfe7e2),
                foreground: rgb(0x79877d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x171c19),
                border: rgb(0xecf4ee),
                foreground: rgb(0x97a29a),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xbcc5bf),
                border: rgb(0x8b968e),
                foreground: rgb(0x171c19),
                secondary_foreground: Some(
                    rgb(0x546259),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x8b968e),
                border: rgb(0x8b968e),
                foreground: rgb(0x171c19),
                secondary_foreground: Some(
                    rgb(0x546259),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x838f87),
                border: rgb(0x8b968e),
                foreground: rgb(0x171c19),
                secondary_foreground: Some(
                    rgb(0x546259),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x7e8b82),
                border: rgb(0x76857b),
                foreground: rgb(0x171c19),
                secondary_foreground: Some(
                    rgb(0x171c19),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xbcc5bf),
                border: rgb(0xa3ada6),
                foreground: rgb(0x68766d),
                secondary_foreground: Some(
                    rgb(0x68766d),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x171c19),
                border: rgb(0xecf4ee),
                foreground: rgb(0x7f8c83),
                secondary_foreground: Some(
                    rgb(0x7f8c83),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xecf4ee),
                border: rgb(0xe5ede7),
                foreground: rgb(0x171c19),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe5ede7),
                border: rgb(0xe5ede7),
                foreground: rgb(0x171c19),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe1e9e4),
                border: rgb(0xe5ede7),
                foreground: rgb(0x171c19),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xcad3cd),
                border: rgb(0xa3ada6),
                foreground: rgb(0x171c19),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xecf4ee),
                border: rgb(0xe8f0eb),
                foreground: rgb(0x838f87),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x171c19),
                border: rgb(0xecf4ee),
                foreground: rgb(0xd4ddd7),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdae7e8),
                border: rgb(0xbed4d6),
                foreground: rgb(0x488c90),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbed4d6),
                border: rgb(0xbed4d6),
                foreground: rgb(0x488c90),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xafcbcc),
                border: rgb(0xbed4d6),
                foreground: rgb(0x488c90),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9bbec0),
                border: rgb(0x84b0b2),
                foreground: rgb(0x50909),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdae7e8),
                border: rgb(0xccdede),
                foreground: rgb(0x679ea1),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x50909),
                border: rgb(0xffffff),
                foreground: rgb(0xa1c2c4),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdaeade),
                border: rgb(0xbedac5),
                foreground: rgb(0x499963),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbedac5),
                border: rgb(0xbedac5),
                foreground: rgb(0x499963),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb0d2b8),
                border: rgb(0xbedac5),
                foreground: rgb(0x499963),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9cc6a6),
                border: rgb(0x84ba93),
                foreground: rgb(0x50a06),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdaeade),
                border: rgb(0xcce2d1),
                foreground: rgb(0x68a97a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x50a06),
                border: rgb(0xffffff),
                foreground: rgb(0xa1caab),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xeee4d5),
                border: rgb(0xdfcfb6),
                foreground: rgb(0xa07e3c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdfcfb6),
                border: rgb(0xdfcfb6),
                foreground: rgb(0xa07e3c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd8c4a6),
                border: rgb(0xdfcfb6),
                foreground: rgb(0xa07e3c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xcdb590),
                border: rgb(0xc1a577),
                foreground: rgb(0xb0804),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xeee4d5),
                border: rgb(0xe7d9c6),
                foreground: rgb(0xb19159),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xb0804),
                border: rgb(0xffffff),
                foreground: rgb(0xd1ba96),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf3ded4),
                border: rgb(0xe8c5b4),
                foreground: rgb(0xb1623a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe8c5b4),
                border: rgb(0xe8c5b4),
                foreground: rgb(0xb1623a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe2b8a4),
                border: rgb(0xe8c5b4),
                foreground: rgb(0xb1623a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xdaa68e),
                border: rgb(0xcf9274),
                foreground: rgb(0xc0604),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf3ded4),
                border: rgb(0xeed1c4),
                foreground: rgb(0xc17957),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xc0604),
                border: rgb(0xffffff),
                foreground: rgb(0xddab94),
                secondary_foreground: None,
            },
        },
    }
}
