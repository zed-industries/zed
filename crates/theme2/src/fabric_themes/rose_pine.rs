use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn rose_pine() -> FabricTheme {
    FabricTheme {
        name: "Rosé Pine",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1d1b2aff),
                border: rgba(0x232132ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x75718eff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x232132ff),
                border: rgba(0x232132ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x75718eff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2f2d40ff),
                border: rgba(0x232132ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x75718eff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x403e53ff),
                border: rgba(0x504d65ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0xe0def4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1d1b2aff),
                border: rgba(0x1e1c2cff),
                foreground: rgba(0x3b384fff),
                secondary_foreground: Some(
                    rgba(0x3b384fff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xe0def4ff),
                border: rgba(0x191724ff),
                foreground: rgba(0x3b394eff),
                secondary_foreground: Some(
                    rgba(0x3b394eff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1d1b2aff),
                border: rgba(0x232132ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x232132ff),
                border: rgba(0x232132ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2f2d40ff),
                border: rgba(0x232132ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x403e53ff),
                border: rgba(0x504d65ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1d1b2aff),
                border: rgba(0x1e1c2cff),
                foreground: rgba(0x3b384fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xe0def4ff),
                border: rgba(0x191724ff),
                foreground: rgba(0x3b394eff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x292739ff),
                border: rgba(0x423f55ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x75718eff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x423f55ff),
                border: rgba(0x423f55ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x75718eff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4e4b63ff),
                border: rgba(0x423f55ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0x75718eff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x47445bff),
                border: rgba(0x36334aff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: Some(
                    rgba(0xe0def4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x292739ff),
                border: rgba(0x353347ff),
                foreground: rgba(0x2f2b43ff),
                secondary_foreground: Some(
                    rgba(0x2f2b43ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xe0def4ff),
                border: rgba(0x191724ff),
                foreground: rgba(0x4b4860ff),
                secondary_foreground: Some(
                    rgba(0x4b4860ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x191724ff),
                border: rgba(0x1c1a29ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1c1a29ff),
                border: rgba(0x1c1a29ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1d1b2bff),
                border: rgba(0x1c1a29ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x222031ff),
                border: rgba(0x353347ff),
                foreground: rgba(0xe0def4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x191724ff),
                border: rgba(0x1a1826ff),
                foreground: rgba(0x4e4b63ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xe0def4ff),
                border: rgba(0x191724ff),
                foreground: rgba(0x1f1d2eff),
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
