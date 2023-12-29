use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn rose_pine_moon() -> FabricTheme {
    FabricTheme {
        name: "Rosé Pine Moon",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x28253cff),
                border: rgba(0x322f48ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x85819eff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x322f48ff),
                border: rgba(0x322f48ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x85819eff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3e3b55ff),
                border: rgba(0x322f48ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x85819eff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x4f4b66ff),
                border: rgba(0x4d4965ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0xe0def4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x28253cff),
                border: rgba(0x2a273eff),
                foreground: rgba(0x3a3653ff),
                secondary_foreground: Some(
                    rgba(0x3a3653ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xe0def4ff),
                border: rgba(0x232136ff),
                foreground: rgba(0x4a4661ff),
                secondary_foreground: Some(
                    rgba(0x4a4661ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x28253cff),
                border: rgba(0x322f48ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x322f48ff),
                border: rgba(0x322f48ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3e3b55ff),
                border: rgba(0x322f48ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x4f4b66ff),
                border: rgba(0x4d4965ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x28253cff),
                border: rgba(0x2a273eff),
                foreground: rgba(0x3a3653ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xe0def4ff),
                border: rgba(0x232136ff),
                foreground: rgba(0x4a4661ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x38354eff),
                border: rgba(0x504c68ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x85819eff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x504c68ff),
                border: rgba(0x504c68ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x85819eff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x524e6aff),
                border: rgba(0x504c68ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x85819eff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x45415dff),
                border: rgba(0x3f3b58ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0xe0def4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x38354eff),
                border: rgba(0x44415bff),
                foreground: rgba(0x615d7aff),
                secondary_foreground: Some(
                    rgba(0x615d7aff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xe0def4ff),
                border: rgba(0x232136ff),
                foreground: rgba(0x484461ff),
                secondary_foreground: Some(
                    rgba(0x484461ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x232136ff),
                border: rgba(0x27243bff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x27243bff),
                border: rgba(0x27243bff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x29263dff),
                border: rgba(0x27243bff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x312e47ff),
                border: rgba(0x44415bff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x232136ff),
                border: rgba(0x252338ff),
                foreground: rgba(0x524e6aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xe0def4ff),
                border: rgba(0x232136ff),
                foreground: rgba(0x2d2a42ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2f3739ff),
                border: rgba(0x435255ff),
                foreground: rgba(0x9cced7ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x435255ff),
                border: rgba(0x435255ff),
                foreground: rgba(0x9cced7ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4e6164ff),
                border: rgba(0x435255ff),
                foreground: rgba(0x9cced7ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x5d757aff),
                border: rgba(0x6e8f94ff),
                foreground: rgba(0xfbfdfdff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2f3739ff),
                border: rgba(0x3a4446ff),
                foreground: rgba(0x85aeb5ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbfdfdff),
                border: rgba(0x171717ff),
                foreground: rgba(0x587074ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x182e23ff),
                border: rgba(0x254839ff),
                foreground: rgba(0x5dc2a3ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x254839ff),
                border: rgba(0x254839ff),
                foreground: rgba(0x5dc2a3ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2c5645ff),
                border: rgba(0x254839ff),
                foreground: rgba(0x5dc2a3ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x356b57ff),
                border: rgba(0x40836cff),
                foreground: rgba(0xf9fdfbff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x182e23ff),
                border: rgba(0x1e3b2eff),
                foreground: rgba(0x4ea287ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf9fdfbff),
                border: rgba(0x000e00ff),
                foreground: rgba(0x326552ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x50341aff),
                border: rgba(0x6d4d2bff),
                foreground: rgba(0xf5c177ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x6d4d2bff),
                border: rgba(0x6d4d2bff),
                foreground: rgba(0xf5c177ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x7e5a34ff),
                border: rgba(0x6d4d2bff),
                foreground: rgba(0xf5c177ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x946e41ff),
                border: rgba(0xb0854fff),
                foreground: rgba(0xfffcf9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x50341aff),
                border: rgba(0x5e4023ff),
                foreground: rgba(0xd2a263ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfffcf9ff),
                border: rgba(0x2c1600ff),
                foreground: rgba(0x8e683cff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x431820ff),
                border: rgba(0x612834ff),
                foreground: rgba(0xea6f92ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x612834ff),
                border: rgba(0x612834ff),
                foreground: rgba(0xea6f92ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x71303fff),
                border: rgba(0x612834ff),
                foreground: rgba(0xea6f92ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x883c4fff),
                border: rgba(0xa44961ff),
                foreground: rgba(0xfff9faff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x431820ff),
                border: rgba(0x52202aff),
                foreground: rgba(0xc75c79ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff9faff),
                border: rgba(0x230000ff),
                foreground: rgba(0x82384aff),
                secondary_foreground: None,
            },
        },
    }
}
