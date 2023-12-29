use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_estuary_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Estuary Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2c2b23ff),
                border: rgba(0x3c3b31ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: Some(
                    rgba(0x91907fff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3c3b31ff),
                border: rgba(0x3c3b31ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: Some(
                    rgba(0x91907fff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x49483cff),
                border: rgba(0x3c3b31ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: Some(
                    rgba(0x91907fff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x5c5b4bff),
                border: rgba(0x646353ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: Some(
                    rgba(0xf4f3ecff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2c2b23ff),
                border: rgba(0x302f27ff),
                foreground: rgba(0x6c6b5aff),
                secondary_foreground: Some(
                    rgba(0x6c6b5aff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4f3ecff),
                border: rgba(0x22221bff),
                foreground: rgba(0x565547ff),
                secondary_foreground: Some(
                    rgba(0x565547ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2c2b23ff),
                border: rgba(0x3c3b31ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3c3b31ff),
                border: rgba(0x3c3b31ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x49483cff),
                border: rgba(0x3c3b31ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x5c5b4bff),
                border: rgba(0x646353ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2c2b23ff),
                border: rgba(0x302f27ff),
                foreground: rgba(0x6c6b5aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4f3ecff),
                border: rgba(0x22221bff),
                foreground: rgba(0x565547ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x424136ff),
                border: rgba(0x5d5c4cff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: Some(
                    rgba(0x91907fff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x5d5c4cff),
                border: rgba(0x5d5c4cff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: Some(
                    rgba(0x91907fff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x626151ff),
                border: rgba(0x5d5c4cff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: Some(
                    rgba(0x91907fff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x676655ff),
                border: rgba(0x6e6d5cff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: Some(
                    rgba(0xf4f3ecff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x424136ff),
                border: rgba(0x504f41ff),
                foreground: rgba(0x7d7c6aff),
                secondary_foreground: Some(
                    rgba(0x7d7c6aff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4f3ecff),
                border: rgba(0x22221bff),
                foreground: rgba(0x666555ff),
                secondary_foreground: Some(
                    rgba(0x666555ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x22221bff),
                border: rgba(0x2a2922ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2a2922ff),
                border: rgba(0x2a2922ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2e2d25ff),
                border: rgba(0x2a2922ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x3b3a30ff),
                border: rgba(0x504f41ff),
                foreground: rgba(0xf4f3ecff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x22221bff),
                border: rgba(0x26261eff),
                foreground: rgba(0x626151ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4f3ecff),
                border: rgba(0x22221bff),
                foreground: rgba(0x36352cff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x142319ff),
                border: rgba(0x1c3927ff),
                foreground: rgba(0x37a166ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1c3927ff),
                border: rgba(0x1c3927ff),
                foreground: rgba(0x37a166ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x20452fff),
                border: rgba(0x1c3927ff),
                foreground: rgba(0x37a166ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x255639ff),
                border: rgba(0x2a6b45ff),
                foreground: rgba(0xf7fbf8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x142319ff),
                border: rgba(0x182e20ff),
                foreground: rgba(0x318555ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7fbf8ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x245135ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1e2110ff),
                border: rgba(0x2f3516ff),
                foreground: rgba(0x7d9727ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2f3516ff),
                border: rgba(0x2f3516ff),
                foreground: rgba(0x7d9727ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x384118ff),
                border: rgba(0x2f3516ff),
                foreground: rgba(0x7d9727ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x45511bff),
                border: rgba(0x54641fff),
                foreground: rgba(0xfafbf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1e2110ff),
                border: rgba(0x262b13ff),
                foreground: rgba(0x697d23ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfafbf6ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x414c1bff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x25210dff),
                border: rgba(0x3b3612ff),
                foreground: rgba(0xa59810ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3b3612ff),
                border: rgba(0x3b3612ff),
                foreground: rgba(0xa59810ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x474113ff),
                border: rgba(0x3b3612ff),
                foreground: rgba(0xa59810ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x595114ff),
                border: rgba(0x6e6514ff),
                foreground: rgba(0xfcfbf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x25210dff),
                border: rgba(0x302b10ff),
                foreground: rgba(0x897e12ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfcfbf6ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x544d14ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2b1811ff),
                border: rgba(0x442619ff),
                foreground: rgba(0xba6237ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x442619ff),
                border: rgba(0x442619ff),
                foreground: rgba(0xba6237ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x522d1cff),
                border: rgba(0x442619ff),
                foreground: rgba(0xba6237ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x653721ff),
                border: rgba(0x7d4327ff),
                foreground: rgba(0xfdf8f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2b1811ff),
                border: rgba(0x371f15ff),
                foreground: rgba(0x9b522fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf8f6ff),
                border: rgba(0x080000ff),
                foreground: rgba(0x603420ff),
                secondary_foreground: None,
            },
        },
    }
}
