use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn ayu_dark() -> FabricTheme {
    FabricTheme {
        name: "Ayu Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1f2127ff),
                border: rgba(0x2d2f34ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: Some(
                    rgba(0x8a8986ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2d2f34ff),
                border: rgba(0x2d2f34ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: Some(
                    rgba(0x8a8986ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x35363aff),
                border: rgba(0x2d2f34ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: Some(
                    rgba(0x8a8986ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x3e4043ff),
                border: rgba(0x494b4dff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: Some(
                    rgba(0xbfbdb6ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1f2127ff),
                border: rgba(0x26282dff),
                foreground: rgba(0x58595aff),
                secondary_foreground: Some(
                    rgba(0x58595aff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xbfbdb6ff),
                border: rgba(0x0d1017ff),
                foreground: rgba(0x3b3d40ff),
                secondary_foreground: Some(
                    rgba(0x3b3d40ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1f2127ff),
                border: rgba(0x2d2f34ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2d2f34ff),
                border: rgba(0x2d2f34ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x35363aff),
                border: rgba(0x2d2f34ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x3e4043ff),
                border: rgba(0x494b4dff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1f2127ff),
                border: rgba(0x26282dff),
                foreground: rgba(0x58595aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xbfbdb6ff),
                border: rgba(0x0d1017ff),
                foreground: rgba(0x3b3d40ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x313337ff),
                border: rgba(0x3f4043ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: Some(
                    rgba(0x8a8986ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3f4043ff),
                border: rgba(0x3f4043ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: Some(
                    rgba(0x8a8986ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x46474aff),
                border: rgba(0x3f4043ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: Some(
                    rgba(0x8a8986ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x505152ff),
                border: rgba(0x5b5c5dff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: Some(
                    rgba(0xbfbdb6ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x313337ff),
                border: rgba(0x383a3eff),
                foreground: rgba(0x696a6aff),
                secondary_foreground: Some(
                    rgba(0x696a6aff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xbfbdb6ff),
                border: rgba(0x0d1017ff),
                foreground: rgba(0x4d4e50ff),
                secondary_foreground: Some(
                    rgba(0x4d4e50ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x0d1017ff),
                border: rgba(0x1b1e24ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1b1e24ff),
                border: rgba(0x1b1e24ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x23252aff),
                border: rgba(0x1b1e24ff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2c2e33ff),
                border: rgba(0x383a3eff),
                foreground: rgba(0xbfbdb6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x0d1017ff),
                border: rgba(0x14171dff),
                foreground: rgba(0x46474aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xbfbdb6ff),
                border: rgba(0x0d1017ff),
                foreground: rgba(0x2a2c31ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x0d304fff),
                border: rgba(0x1b4a6eff),
                foreground: rgba(0x5ac2feff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1b4a6eff),
                border: rgba(0x1b4a6eff),
                foreground: rgba(0x5ac2feff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x23587fff),
                border: rgba(0x1b4a6eff),
                foreground: rgba(0x5ac2feff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2d6c98ff),
                border: rgba(0x3984b4ff),
                foreground: rgba(0xfafcffff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x0d304fff),
                border: rgba(0x143d5eff),
                foreground: rgba(0x49a2d9ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfafcffff),
                border: rgba(0x001129ff),
                foreground: rgba(0x296691ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x294113ff),
                border: rgba(0x405c1dff),
                foreground: rgba(0xaad84cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x405c1dff),
                border: rgba(0x405c1dff),
                foreground: rgba(0xaad84cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4c6b23ff),
                border: rgba(0x405c1dff),
                foreground: rgba(0xaad84cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x5e802aff),
                border: rgba(0x739934ff),
                foreground: rgba(0xfcfdf8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x294113ff),
                border: rgba(0x354f18ff),
                foreground: rgba(0x8eb840ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfcfdf8ff),
                border: rgba(0x102100ff),
                foreground: rgba(0x597a28ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x572916ff),
                border: rgba(0x754221ff),
                foreground: rgba(0xfeb454ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x754221ff),
                border: rgba(0x754221ff),
                foreground: rgba(0xfeb454ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x864f27ff),
                border: rgba(0x754221ff),
                foreground: rgba(0xfeb454ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9d6230ff),
                border: rgba(0xb8793aff),
                foreground: rgba(0xfffcf8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x572916ff),
                border: rgba(0x65361bff),
                foreground: rgba(0xdb9647ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfffcf8ff),
                border: rgba(0x330a00ff),
                foreground: rgba(0x965d2dff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x48171cff),
                border: rgba(0x66272dff),
                foreground: rgba(0xef7178ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x66272dff),
                border: rgba(0x66272dff),
                foreground: rgba(0xef7178ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x773035ff),
                border: rgba(0x66272dff),
                foreground: rgba(0xef7178ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x8d3c42ff),
                border: rgba(0xa94a50ff),
                foreground: rgba(0xfff9f9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x48171cff),
                border: rgba(0x571f24ff),
                foreground: rgba(0xcc5d64ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff9f9ff),
                border: rgba(0x270000ff),
                foreground: rgba(0x87383eff),
                secondary_foreground: None,
            },
        },
    }
}
