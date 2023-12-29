use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_heath_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Heath Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe1d6e1ff),
                border: rgba(0xcdbecdff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: Some(
                    rgba(0x6b5e6bff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcdbecdff),
                border: rgba(0xcdbecdff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: Some(
                    rgba(0x6b5e6bff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc0b1c0ff),
                border: rgba(0xcdbecdff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: Some(
                    rgba(0x6b5e6bff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xae9eaeff),
                border: rgba(0xa696a6ff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: Some(
                    rgba(0x1b181bff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe1d6e1ff),
                border: rgba(0xd8cad8ff),
                foreground: rgba(0x9e8f9eff),
                secondary_foreground: Some(
                    rgba(0x9e8f9eff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b181bff),
                border: rgba(0xf7f3f7ff),
                foreground: rgba(0xb3a4b3ff),
                secondary_foreground: Some(
                    rgba(0xb3a4b3ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe1d6e1ff),
                border: rgba(0xcdbecdff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcdbecdff),
                border: rgba(0xcdbecdff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc0b1c0ff),
                border: rgba(0xcdbecdff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xae9eaeff),
                border: rgba(0xa696a6ff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe1d6e1ff),
                border: rgba(0xd8cad8ff),
                foreground: rgba(0x9e8f9eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b181bff),
                border: rgba(0xf7f3f7ff),
                foreground: rgba(0xb3a4b3ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xc6b8c6ff),
                border: rgba(0xad9dadff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: Some(
                    rgba(0x6b5e6bff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xad9dadff),
                border: rgba(0xad9dadff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: Some(
                    rgba(0x6b5e6bff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xa898a8ff),
                border: rgba(0xad9dadff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: Some(
                    rgba(0x6b5e6bff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xa394a3ff),
                border: rgba(0x9b8c9bff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: Some(
                    rgba(0x1b181bff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xc6b8c6ff),
                border: rgba(0xbaaabaff),
                foreground: rgba(0x857785ff),
                secondary_foreground: Some(
                    rgba(0x857785ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b181bff),
                border: rgba(0xf7f3f7ff),
                foreground: rgba(0xa494a4ff),
                secondary_foreground: Some(
                    rgba(0xa494a4ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf7f3f7ff),
                border: rgba(0xe5dce5ff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe5dce5ff),
                border: rgba(0xe5dce5ff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xddd0ddff),
                border: rgba(0xe5dce5ff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xcebfceff),
                border: rgba(0xbaaabaff),
                foreground: rgba(0x1b181bff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf7f3f7ff),
                border: rgba(0xeee7eeff),
                foreground: rgba(0xa898a8ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b181bff),
                border: rgba(0xf7f3f7ff),
                foreground: rgba(0xd2c4d2ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe2dffcff),
                border: rgba(0xcac7faff),
                foreground: rgba(0x526aebff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcac7faff),
                border: rgba(0xcac7faff),
                foreground: rgba(0x526aebff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xbebbf8ff),
                border: rgba(0xcac7faff),
                foreground: rgba(0x526aebff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xacaaf6ff),
                border: rgba(0x9597f3ff),
                foreground: rgba(0x050726ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe2dffcff),
                border: rgba(0xd6d3fbff),
                foreground: rgba(0x7780f0ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x050726ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xb1aff7ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xeae6d6ff),
                border: rgba(0xd9d4b6ff),
                foreground: rgba(0x918b3cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd9d4b6ff),
                border: rgba(0xd9d4b6ff),
                foreground: rgba(0x918b3cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd0caa6ff),
                border: rgba(0xd9d4b6ff),
                foreground: rgba(0x918b3cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xc4bd91ff),
                border: rgba(0xb6af78ff),
                foreground: rgba(0x090804ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xeae6d6ff),
                border: rgba(0xe1ddc6ff),
                foreground: rgba(0xa49d5aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x090804ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xc8c197ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf5e6d5ff),
                border: rgba(0xebd3b5ff),
                foreground: rgba(0xbb8a36ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xebd3b5ff),
                border: rgba(0xebd3b5ff),
                foreground: rgba(0xbb8a36ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe6caa4ff),
                border: rgba(0xebd3b5ff),
                foreground: rgba(0xbb8a36ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xdfbc8eff),
                border: rgba(0xd5ae75ff),
                foreground: rgba(0x160804ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf5e6d5ff),
                border: rgba(0xf0dcc5ff),
                foreground: rgba(0xc99c56ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x160804ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xe1c194ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfcd9d1ff),
                border: rgba(0xf7bcaeff),
                foreground: rgba(0xca412cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf7bcaeff),
                border: rgba(0xf7bcaeff),
                foreground: rgba(0xca412cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xf3ad9cff),
                border: rgba(0xf7bcaeff),
                foreground: rgba(0xca412cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xed9885ff),
                border: rgba(0xe4806aff),
                foreground: rgba(0x250503ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfcd9d1ff),
                border: rgba(0xfacbc0ff),
                foreground: rgba(0xd8624aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x250503ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xef9e8bff),
                secondary_foreground: None,
            },
        },
    }
}
