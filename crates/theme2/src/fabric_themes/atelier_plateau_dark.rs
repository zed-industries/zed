use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_plateau_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Plateau Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x252020),
                border: rgb(0x352f2f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: Some(
                    rgb(0x898383),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x352f2f),
                border: rgb(0x352f2f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: Some(
                    rgb(0x898383),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x423b3b),
                border: rgb(0x352f2f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: Some(
                    rgb(0x898383),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x554d4d),
                border: rgb(0x5d5555),
                foreground: rgb(0xf4ecec),
                secondary_foreground: Some(
                    rgb(0xf4ecec),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x252020),
                border: rgb(0x292424),
                foreground: rgb(0x655d5d),
                secondary_foreground: Some(
                    rgb(0x655d5d),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4ecec),
                border: rgb(0x1b1818),
                foreground: rgb(0x4f4848),
                secondary_foreground: Some(
                    rgb(0x4f4848),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x252020),
                border: rgb(0x352f2f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x352f2f),
                border: rgb(0x352f2f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x423b3b),
                border: rgb(0x352f2f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x554d4d),
                border: rgb(0x5d5555),
                foreground: rgb(0xf4ecec),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x252020),
                border: rgb(0x292424),
                foreground: rgb(0x655d5d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4ecec),
                border: rgb(0x1b1818),
                foreground: rgb(0x4f4848),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3b3535),
                border: rgb(0x564e4e),
                foreground: rgb(0xf4ecec),
                secondary_foreground: Some(
                    rgb(0x898383),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x564e4e),
                border: rgb(0x564e4e),
                foreground: rgb(0xf4ecec),
                secondary_foreground: Some(
                    rgb(0x898383),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x5b5353),
                border: rgb(0x564e4e),
                foreground: rgb(0xf4ecec),
                secondary_foreground: Some(
                    rgb(0x898383),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x605858),
                border: rgb(0x675f5f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: Some(
                    rgb(0xf4ecec),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3b3535),
                border: rgb(0x494242),
                foreground: rgb(0x756e6e),
                secondary_foreground: Some(
                    rgb(0x756e6e),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4ecec),
                border: rgb(0x1b1818),
                foreground: rgb(0x5f5757),
                secondary_foreground: Some(
                    rgb(0x5f5757),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1b1818),
                border: rgb(0x231f1f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x231f1f),
                border: rgb(0x231f1f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x272222),
                border: rgb(0x231f1f),
                foreground: rgb(0xf4ecec),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x342e2e),
                border: rgb(0x494242),
                foreground: rgb(0xf4ecec),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1b1818),
                border: rgb(0x1f1b1b),
                foreground: rgb(0x5b5353),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4ecec),
                border: rgb(0x1b1818),
                foreground: rgb(0x2f2a2a),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1c1b29),
                border: rgb(0x2c2b45),
                foreground: rgb(0x7272ca),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2c2b45),
                border: rgb(0x2c2b45),
                foreground: rgb(0x7272ca),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x353354),
                border: rgb(0x2c2b45),
                foreground: rgb(0x7272ca),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x403f6a),
                border: rgb(0x4e4d85),
                foreground: rgb(0xfaf9fd),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1c1b29),
                border: rgb(0x242336),
                foreground: rgb(0x605fa6),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfaf9fd),
                border: rgb(0x0000),
                foreground: rgb(0x3c3b64),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x161f1f),
                border: rgb(0x203232),
                foreground: rgb(0x4b8b8b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x203232),
                border: rgb(0x203232),
                foreground: rgb(0x4b8b8b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x263c3c),
                border: rgb(0x203232),
                foreground: rgb(0x4b8b8b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2d4b4b),
                border: rgb(0x355d5d),
                foreground: rgb(0xf8fafa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x161f1f),
                border: rgb(0x1b2929),
                foreground: rgb(0x407474),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8fafa),
                border: rgb(0x0000),
                foreground: rgb(0x2b4747),
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
                background: rgb(0x361414),
                border: rgb(0x501e1e),
                foreground: rgb(0xca4949),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x501e1e),
                border: rgb(0x501e1e),
                foreground: rgb(0xca4949),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x5e2323),
                border: rgb(0x501e1e),
                foreground: rgb(0xca4949),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x722a2a),
                border: rgb(0x8b3333),
                foreground: rgb(0xfef8f7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x361414),
                border: rgb(0x431919),
                foreground: rgb(0xa93e3e),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfef8f7),
                border: rgb(0x190000),
                foreground: rgb(0x6d2828),
                secondary_foreground: None,
            },
        },
    }
}
