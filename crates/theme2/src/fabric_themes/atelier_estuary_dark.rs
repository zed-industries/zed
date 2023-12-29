use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_estuary_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Estuary Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2c2b23),
                border: rgb(0x3c3b31),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: Some(
                    rgb(0x91907f),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3c3b31),
                border: rgb(0x3c3b31),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: Some(
                    rgb(0x91907f),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x49483c),
                border: rgb(0x3c3b31),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: Some(
                    rgb(0x91907f),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x5c5b4b),
                border: rgb(0x646353),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: Some(
                    rgb(0xf4f3ec),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2c2b23),
                border: rgb(0x302f27),
                foreground: rgb(0x6c6b5a),
                secondary_foreground: Some(
                    rgb(0x6c6b5a),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4f3ec),
                border: rgb(0x22221b),
                foreground: rgb(0x565547),
                secondary_foreground: Some(
                    rgb(0x565547),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2c2b23),
                border: rgb(0x3c3b31),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3c3b31),
                border: rgb(0x3c3b31),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x49483c),
                border: rgb(0x3c3b31),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x5c5b4b),
                border: rgb(0x646353),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2c2b23),
                border: rgb(0x302f27),
                foreground: rgb(0x6c6b5a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4f3ec),
                border: rgb(0x22221b),
                foreground: rgb(0x565547),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x424136),
                border: rgb(0x5d5c4c),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: Some(
                    rgb(0x91907f),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x5d5c4c),
                border: rgb(0x5d5c4c),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: Some(
                    rgb(0x91907f),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x626151),
                border: rgb(0x5d5c4c),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: Some(
                    rgb(0x91907f),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x676655),
                border: rgb(0x6e6d5c),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: Some(
                    rgb(0xf4f3ec),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x424136),
                border: rgb(0x504f41),
                foreground: rgb(0x7d7c6a),
                secondary_foreground: Some(
                    rgb(0x7d7c6a),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4f3ec),
                border: rgb(0x22221b),
                foreground: rgb(0x666555),
                secondary_foreground: Some(
                    rgb(0x666555),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x22221b),
                border: rgb(0x2a2922),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2a2922),
                border: rgb(0x2a2922),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2e2d25),
                border: rgb(0x2a2922),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x3b3a30),
                border: rgb(0x504f41),
                foreground: rgb(0xf4f3ec),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x22221b),
                border: rgb(0x26261e),
                foreground: rgb(0x626151),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4f3ec),
                border: rgb(0x22221b),
                foreground: rgb(0x36352c),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x142319),
                border: rgb(0x1c3927),
                foreground: rgb(0x37a166),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1c3927),
                border: rgb(0x1c3927),
                foreground: rgb(0x37a166),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x20452f),
                border: rgb(0x1c3927),
                foreground: rgb(0x37a166),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x255639),
                border: rgb(0x2a6b45),
                foreground: rgb(0xf7fbf8),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x142319),
                border: rgb(0x182e20),
                foreground: rgb(0x318555),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7fbf8),
                border: rgb(0x0000),
                foreground: rgb(0x245135),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1e2110),
                border: rgb(0x2f3516),
                foreground: rgb(0x7d9727),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2f3516),
                border: rgb(0x2f3516),
                foreground: rgb(0x7d9727),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x384118),
                border: rgb(0x2f3516),
                foreground: rgb(0x7d9727),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x45511b),
                border: rgb(0x54641f),
                foreground: rgb(0xfafbf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1e2110),
                border: rgb(0x262b13),
                foreground: rgb(0x697d23),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfafbf6),
                border: rgb(0x0000),
                foreground: rgb(0x414c1b),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x25210d),
                border: rgb(0x3b3612),
                foreground: rgb(0xa59810),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3b3612),
                border: rgb(0x3b3612),
                foreground: rgb(0xa59810),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x474113),
                border: rgb(0x3b3612),
                foreground: rgb(0xa59810),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x595114),
                border: rgb(0x6e6514),
                foreground: rgb(0xfcfbf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x25210d),
                border: rgb(0x302b10),
                foreground: rgb(0x897e12),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfcfbf6),
                border: rgb(0x0000),
                foreground: rgb(0x544d14),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2b1811),
                border: rgb(0x442619),
                foreground: rgb(0xba6237),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x442619),
                border: rgb(0x442619),
                foreground: rgb(0xba6237),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x522d1c),
                border: rgb(0x442619),
                foreground: rgb(0xba6237),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x653721),
                border: rgb(0x7d4327),
                foreground: rgb(0xfdf8f6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2b1811),
                border: rgb(0x371f15),
                foreground: rgb(0x9b522f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf8f6),
                border: rgb(0x80000),
                foreground: rgb(0x603420),
                secondary_foreground: None,
            },
        },
    }
}
