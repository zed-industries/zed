use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_cave_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Cave Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x221f26ff),
                border: rgba(0x332f38ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: Some(
                    rgba(0x898591ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x332f38ff),
                border: rgba(0x332f38ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: Some(
                    rgba(0x898591ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x413c47ff),
                border: rgba(0x332f38ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: Some(
                    rgba(0x898591ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x544f5cff),
                border: rgba(0x5d5765ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: Some(
                    rgba(0xefecf4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x221f26ff),
                border: rgba(0x26232aff),
                foreground: rgba(0x655f6dff),
                secondary_foreground: Some(
                    rgba(0x655f6dff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xefecf4ff),
                border: rgba(0x19171cff),
                foreground: rgba(0x4f4956ff),
                secondary_foreground: Some(
                    rgba(0x4f4956ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x221f26ff),
                border: rgba(0x332f38ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x332f38ff),
                border: rgba(0x332f38ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x413c47ff),
                border: rgba(0x332f38ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x544f5cff),
                border: rgba(0x5d5765ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x221f26ff),
                border: rgba(0x26232aff),
                foreground: rgba(0x655f6dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xefecf4ff),
                border: rgba(0x19171cff),
                foreground: rgba(0x4f4956ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3a353fff),
                border: rgba(0x56505eff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: Some(
                    rgba(0x898591ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x56505eff),
                border: rgba(0x56505eff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: Some(
                    rgba(0x898591ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x5b5563ff),
                border: rgba(0x56505eff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: Some(
                    rgba(0x898591ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x605a68ff),
                border: rgba(0x67616fff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: Some(
                    rgba(0xefecf4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3a353fff),
                border: rgba(0x48434fff),
                foreground: rgba(0x756f7eff),
                secondary_foreground: Some(
                    rgba(0x756f7eff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xefecf4ff),
                border: rgba(0x19171cff),
                foreground: rgba(0x5f5967ff),
                secondary_foreground: Some(
                    rgba(0x5f5967ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x19171cff),
                border: rgba(0x201e24ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x201e24ff),
                border: rgba(0x201e24ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x242128ff),
                border: rgba(0x201e24ff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x322e37ff),
                border: rgba(0x48434fff),
                foreground: rgba(0xefecf4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x19171cff),
                border: rgba(0x1d1a20ff),
                foreground: rgba(0x5b5563ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xefecf4ff),
                border: rgba(0x19171cff),
                foreground: rgba(0x2c2930ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x161a36ff),
                border: rgba(0x222953ff),
                foreground: rgba(0x576ddaff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x222953ff),
                border: rgba(0x222953ff),
                foreground: rgba(0x576ddaff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x293162ff),
                border: rgba(0x222953ff),
                foreground: rgba(0x576ddaff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x313d79ff),
                border: rgba(0x3c4994ff),
                foreground: rgba(0xf9f9feff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x161a36ff),
                border: rgba(0x1c2244ff),
                foreground: rgba(0x495bb7ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf9f9feff),
                border: rgba(0x000014ff),
                foreground: rgba(0x2e3873ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x132020ff),
                border: rgba(0x1a3434ff),
                foreground: rgba(0x2c9292ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1a3434ff),
                border: rgba(0x1a3434ff),
                foreground: rgba(0x2c9292ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1d3f3fff),
                border: rgba(0x1a3434ff),
                foreground: rgba(0x2c9292ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x204f4fff),
                border: rgba(0x246161ff),
                foreground: rgba(0xf7fafaff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x132020ff),
                border: rgba(0x172a2aff),
                foreground: rgba(0x287979ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7fafaff),
                border: rgba(0x000000ff),
                foreground: rgba(0x204a4aff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x231a12ff),
                border: rgba(0x392a1aff),
                foreground: rgba(0xa06e3bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x392a1aff),
                border: rgba(0x392a1aff),
                foreground: rgba(0xa06e3bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x45321eff),
                border: rgba(0x392a1aff),
                foreground: rgba(0xa06e3bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x563d23ff),
                border: rgba(0x6b4a2bff),
                foreground: rgba(0xfcf9f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x231a12ff),
                border: rgba(0x2e2216ff),
                foreground: rgba(0x855c33ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfcf9f6ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x513922ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x28151cff),
                border: rgba(0x421f2dff),
                foreground: rgba(0xbe4678ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x421f2dff),
                border: rgba(0x421f2dff),
                foreground: rgba(0xbe4678ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x512435ff),
                border: rgba(0x421f2dff),
                foreground: rgba(0xbe4678ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x652b42ff),
                border: rgba(0x7e3350ff),
                foreground: rgba(0xfdf8f9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x28151cff),
                border: rgba(0x351a24ff),
                foreground: rgba(0x9d3d64ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf8f9ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x5f293eff),
                secondary_foreground: None,
            },
        },
    }
}
