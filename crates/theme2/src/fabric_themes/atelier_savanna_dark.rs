use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_savanna_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Savanna Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1f2621),
                border: rgb(0x2f3832),
                foreground: rgb(0xecf4ee),
                secondary_foreground: Some(
                    rgb(0x859188),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2f3832),
                border: rgb(0x2f3832),
                foreground: rgb(0xecf4ee),
                secondary_foreground: Some(
                    rgb(0x859188),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3c4740),
                border: rgb(0x2f3832),
                foreground: rgb(0xecf4ee),
                secondary_foreground: Some(
                    rgb(0x859188),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x4f5c53),
                border: rgb(0x57655c),
                foreground: rgb(0xecf4ee),
                secondary_foreground: Some(
                    rgb(0xecf4ee),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1f2621),
                border: rgb(0x232a25),
                foreground: rgb(0x5f6d64),
                secondary_foreground: Some(
                    rgb(0x5f6d64),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xecf4ee),
                border: rgb(0x171c19),
                foreground: rgb(0x49564e),
                secondary_foreground: Some(
                    rgb(0x49564e),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1f2621),
                border: rgb(0x2f3832),
                foreground: rgb(0xecf4ee),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2f3832),
                border: rgb(0x2f3832),
                foreground: rgb(0xecf4ee),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3c4740),
                border: rgb(0x2f3832),
                foreground: rgb(0xecf4ee),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x4f5c53),
                border: rgb(0x57655c),
                foreground: rgb(0xecf4ee),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1f2621),
                border: rgb(0x232a25),
                foreground: rgb(0x5f6d64),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xecf4ee),
                border: rgb(0x171c19),
                foreground: rgb(0x49564e),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x353f39),
                border: rgb(0x505e55),
                foreground: rgb(0xecf4ee),
                secondary_foreground: Some(
                    rgb(0x859188),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x505e55),
                border: rgb(0x505e55),
                foreground: rgb(0xecf4ee),
                secondary_foreground: Some(
                    rgb(0x859188),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x55635a),
                border: rgb(0x505e55),
                foreground: rgb(0xecf4ee),
                secondary_foreground: Some(
                    rgb(0x859188),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x5a685f),
                border: rgb(0x616f66),
                foreground: rgb(0xecf4ee),
                secondary_foreground: Some(
                    rgb(0xecf4ee),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x353f39),
                border: rgb(0x434f47),
                foreground: rgb(0x6f7e74),
                secondary_foreground: Some(
                    rgb(0x6f7e74),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xecf4ee),
                border: rgb(0x171c19),
                foreground: rgb(0x59675e),
                secondary_foreground: Some(
                    rgb(0x59675e),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x171c19),
                border: rgb(0x1e2420),
                foreground: rgb(0xecf4ee),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1e2420),
                border: rgb(0x1e2420),
                foreground: rgb(0xecf4ee),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x212823),
                border: rgb(0x1e2420),
                foreground: rgb(0xecf4ee),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2e3731),
                border: rgb(0x434f47),
                foreground: rgb(0xecf4ee),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x171c19),
                border: rgb(0x1a201c),
                foreground: rgb(0x55635a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xecf4ee),
                border: rgb(0x171c19),
                foreground: rgb(0x29302b),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x151f20),
                border: rgb(0x1f3233),
                foreground: rgb(0x478c90),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1f3233),
                border: rgb(0x1f3233),
                foreground: rgb(0x478c90),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x243d3e),
                border: rgb(0x1f3233),
                foreground: rgb(0x478c90),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2b4c4e),
                border: rgb(0x335d60),
                foreground: rgb(0xf8fafa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x151f20),
                border: rgb(0x1a292a),
                foreground: rgb(0x3d7578),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8fafa),
                border: rgb(0x0000),
                foreground: rgb(0x294749),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x162119),
                border: rgb(0x203626),
                foreground: rgb(0x489963),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x203626),
                border: rgb(0x203626),
                foreground: rgb(0x489963),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x25422e),
                border: rgb(0x203626),
                foreground: rgb(0x489963),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2b5238),
                border: rgb(0x346643),
                foreground: rgb(0xf8fbf8),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x162119),
                border: rgb(0x1b2c1f),
                foreground: rgb(0x3e7f53),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8fbf8),
                border: rgb(0x0000),
                foreground: rgb(0x2a4d34),
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
                background: rgb(0x261811),
                border: rgb(0x3f2619),
                foreground: rgb(0xb16139),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3f2619),
                border: rgb(0x3f2619),
                foreground: rgb(0xb16139),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4c2d1d),
                border: rgb(0x3f2619),
                foreground: rgb(0xb16139),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x5f3722),
                border: rgb(0x764229),
                foreground: rgb(0xfdf8f6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x261811),
                border: rgb(0x331f16),
                foreground: rgb(0x935131),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf8f6),
                border: rgb(0x0000),
                foreground: rgb(0x5a3321),
                secondary_foreground: None,
            },
        },
    }
}
