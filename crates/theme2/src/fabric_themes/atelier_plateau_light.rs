use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_plateau_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Plateau Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xebe3e3ff),
                border: rgba(0xcfc7c7ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: Some(
                    rgba(0x5a5252ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcfc7c7ff),
                border: rgba(0xcfc7c7ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: Some(
                    rgba(0x5a5252ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb5aeaeff),
                border: rgba(0xcfc7c7ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: Some(
                    rgba(0x5a5252ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x908b8bff),
                border: rgba(0x857f7fff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: Some(
                    rgba(0x1b1818ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xebe3e3ff),
                border: rgba(0xe7dfdfff),
                foreground: rgba(0x7e7777ff),
                secondary_foreground: Some(
                    rgba(0x7e7777ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b1818ff),
                border: rgba(0xf4ececff),
                foreground: rgba(0x9b9696ff),
                secondary_foreground: Some(
                    rgba(0x9b9696ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xebe3e3ff),
                border: rgba(0xcfc7c7ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcfc7c7ff),
                border: rgba(0xcfc7c7ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb5aeaeff),
                border: rgba(0xcfc7c7ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x908b8bff),
                border: rgba(0x857f7fff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xebe3e3ff),
                border: rgba(0xe7dfdfff),
                foreground: rgba(0x7e7777ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b1818ff),
                border: rgba(0xf4ececff),
                foreground: rgba(0x9b9696ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xc1bbbbff),
                border: rgba(0x8e8989ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: Some(
                    rgba(0x5a5252ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x8e8989ff),
                border: rgba(0x8e8989ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: Some(
                    rgba(0x5a5252ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x878181ff),
                border: rgba(0x8e8989ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: Some(
                    rgba(0x5a5252ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x837c7cff),
                border: rgba(0x7c7575ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: Some(
                    rgba(0x1b1818ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xc1bbbbff),
                border: rgba(0xa8a2a2ff),
                foreground: rgba(0x6e6666ff),
                secondary_foreground: Some(
                    rgba(0x6e6666ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b1818ff),
                border: rgba(0xf4ececff),
                foreground: rgba(0x837d7dff),
                secondary_foreground: Some(
                    rgba(0x837d7dff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf4ececff),
                border: rgba(0xede5e5ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xede5e5ff),
                border: rgba(0xede5e5ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe9e1e1ff),
                border: rgba(0xede5e5ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd1cacaff),
                border: rgba(0xa8a2a2ff),
                foreground: rgba(0x1b1818ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf4ececff),
                border: rgba(0xf0e8e8ff),
                foreground: rgba(0x878181ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b1818ff),
                border: rgba(0xf4ececff),
                foreground: rgba(0xdcd4d4ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe4e1f5ff),
                border: rgba(0xcecaecff),
                foreground: rgba(0x7372caff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcecaecff),
                border: rgba(0xcecaecff),
                foreground: rgba(0x7372caff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc3bfe8ff),
                border: rgba(0xcecaecff),
                foreground: rgba(0x7372caff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xb4afe3ff),
                border: rgba(0xa29ddbff),
                foreground: rgba(0x08070cff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe4e1f5ff),
                border: rgba(0xd9d5f1ff),
                foreground: rgba(0x8a87d2ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x08070cff),
                border: rgba(0xffffffff),
                foreground: rgba(0xb8b3e4ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdae7e7ff),
                border: rgba(0xbfd4d4ff),
                foreground: rgba(0x4c8b8bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbfd4d4ff),
                border: rgba(0xbfd4d4ff),
                foreground: rgba(0x4c8b8bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb0cbcaff),
                border: rgba(0xbfd4d4ff),
                foreground: rgba(0x4c8b8bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9cbebdff),
                border: rgba(0x85afafff),
                foreground: rgba(0x050909ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdae7e7ff),
                border: rgba(0xcdddddff),
                foreground: rgba(0x699d9dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x050909ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa2c2c1ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xeee0d5ff),
                border: rgba(0xe0c9b5ff),
                foreground: rgba(0xa06e3cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe0c9b5ff),
                border: rgba(0xe0c9b5ff),
                foreground: rgba(0xa06e3cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd8bda5ff),
                border: rgba(0xe0c9b5ff),
                foreground: rgba(0xa06e3cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xceac8fff),
                border: rgba(0xc29a76ff),
                foreground: rgba(0x0b0704ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xeee0d5ff),
                border: rgba(0xe7d4c5ff),
                foreground: rgba(0xb18458ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0b0704ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xd2b195ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfadbd7ff),
                border: rgba(0xf4bfbaff),
                foreground: rgba(0xca4a4aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf4bfbaff),
                border: rgba(0xf4bfbaff),
                foreground: rgba(0xca4a4aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xf0b1abff),
                border: rgba(0xf4bfbaff),
                foreground: rgba(0xca4a4aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xea9d96ff),
                border: rgba(0xe2857fff),
                foreground: rgba(0x1f0605ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfadbd7ff),
                border: rgba(0xf7cdc9ff),
                foreground: rgba(0xd76863ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1f0605ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xeca29cff),
                secondary_foreground: None,
            },
        },
    }
}
