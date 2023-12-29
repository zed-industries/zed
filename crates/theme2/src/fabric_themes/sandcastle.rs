use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn sandcastle() -> FabricTheme {
    FabricTheme {
        name: "Sandcastle",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2b3039ff),
                border: rgba(0x313741ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: Some(
                    rgba(0xa69782ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x313741ff),
                border: rgba(0x313741ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: Some(
                    rgba(0xa69782ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x363c47ff),
                border: rgba(0x313741ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: Some(
                    rgba(0xa69782ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x3d4350ff),
                border: rgba(0x4d4d52ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: Some(
                    rgba(0xfdf4c1ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2b3039ff),
                border: rgba(0x2c323bff),
                foreground: rgba(0x645b54ff),
                secondary_foreground: Some(
                    rgba(0x645b54ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf4c1ff),
                border: rgba(0x282c34ff),
                foreground: rgba(0x3b414dff),
                secondary_foreground: Some(
                    rgba(0x3b414dff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2b3039ff),
                border: rgba(0x313741ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x313741ff),
                border: rgba(0x313741ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x363c47ff),
                border: rgba(0x313741ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x3d4350ff),
                border: rgba(0x4d4d52ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2b3039ff),
                border: rgba(0x2c323bff),
                foreground: rgba(0x645b54ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf4c1ff),
                border: rgba(0x282c34ff),
                foreground: rgba(0x3b414dff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x333944ff),
                border: rgba(0x3d4350ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: Some(
                    rgba(0xa69782ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3d4350ff),
                border: rgba(0x3d4350ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: Some(
                    rgba(0xa69782ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x484a52ff),
                border: rgba(0x3d4350ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: Some(
                    rgba(0xa69782ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x575353ff),
                border: rgba(0x6a5f57ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: Some(
                    rgba(0xfdf4c1ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x333944ff),
                border: rgba(0x393f4aff),
                foreground: rgba(0x827568ff),
                secondary_foreground: Some(
                    rgba(0x827568ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf4c1ff),
                border: rgba(0x282c34ff),
                foreground: rgba(0x535053ff),
                secondary_foreground: Some(
                    rgba(0x535053ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x282c34ff),
                border: rgba(0x2a2f38ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2a2f38ff),
                border: rgba(0x2a2f38ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2b313aff),
                border: rgba(0x2a2f38ff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x303640ff),
                border: rgba(0x393f4aff),
                foreground: rgba(0xfdf4c1ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x282c34ff),
                border: rgba(0x292e36ff),
                foreground: rgba(0x484a52ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf4c1ff),
                border: rgba(0x282c34ff),
                foreground: rgba(0x2e343eff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x171f1fff),
                border: rgba(0x223232ff),
                foreground: rgba(0x528b8bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x223232ff),
                border: rgba(0x223232ff),
                foreground: rgba(0x528b8bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x283c3cff),
                border: rgba(0x223232ff),
                foreground: rgba(0x528b8bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x304b4bff),
                border: rgba(0x395d5dff),
                foreground: rgba(0xf8fafaff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x171f1fff),
                border: rgba(0x1c2929ff),
                foreground: rgba(0x467474ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8fafaff),
                border: rgba(0x000000ff),
                foreground: rgba(0x2d4747ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1e2321ff),
                border: rgba(0x303a36ff),
                foreground: rgba(0x83a598ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x303a36ff),
                border: rgba(0x303a36ff),
                foreground: rgba(0x83a598ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3a4641ff),
                border: rgba(0x303a36ff),
                foreground: rgba(0x83a598ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x485852ff),
                border: rgba(0x586d65ff),
                foreground: rgba(0xfafbfbff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1e2321ff),
                border: rgba(0x272f2cff),
                foreground: rgba(0x6d887eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfafbfbff),
                border: rgba(0x000000ff),
                foreground: rgba(0x43534dff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x231d12ff),
                border: rgba(0x392e1aff),
                foreground: rgba(0xa07e3bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x392e1aff),
                border: rgba(0x392e1aff),
                foreground: rgba(0xa07e3bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x45371fff),
                border: rgba(0x392e1aff),
                foreground: rgba(0xa07e3bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x564524ff),
                border: rgba(0x6b542bff),
                foreground: rgba(0xfcf9f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x231d12ff),
                border: rgba(0x2e2516ff),
                foreground: rgba(0x856933ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfcf9f7ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x514023ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x26191cff),
                border: rgba(0x3f272dff),
                foreground: rgba(0xb4637aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3f272dff),
                border: rgba(0x3f272dff),
                foreground: rgba(0xb4637aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4c2e36ff),
                border: rgba(0x3f272dff),
                foreground: rgba(0xb4637aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x603843ff),
                border: rgba(0x774352ff),
                foreground: rgba(0xfcf8f9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x26191cff),
                border: rgba(0x322025ff),
                foreground: rgba(0x955366ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfcf8f9ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x5a343fff),
                secondary_foreground: None,
            },
        },
    }
}
