use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_heath_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Heath Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe1d6e1),
                border: rgb(0xcdbecd),
                foreground: rgb(0x1b181b),
                secondary_foreground: Some(
                    rgb(0x6b5e6b),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcdbecd),
                border: rgb(0xcdbecd),
                foreground: rgb(0x1b181b),
                secondary_foreground: Some(
                    rgb(0x6b5e6b),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc0b1c0),
                border: rgb(0xcdbecd),
                foreground: rgb(0x1b181b),
                secondary_foreground: Some(
                    rgb(0x6b5e6b),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xae9eae),
                border: rgb(0xa696a6),
                foreground: rgb(0x1b181b),
                secondary_foreground: Some(
                    rgb(0x1b181b),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe1d6e1),
                border: rgb(0xd8cad8),
                foreground: rgb(0x9e8f9e),
                secondary_foreground: Some(
                    rgb(0x9e8f9e),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b181b),
                border: rgb(0xf7f3f7),
                foreground: rgb(0xb3a4b3),
                secondary_foreground: Some(
                    rgb(0xb3a4b3),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe1d6e1),
                border: rgb(0xcdbecd),
                foreground: rgb(0x1b181b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcdbecd),
                border: rgb(0xcdbecd),
                foreground: rgb(0x1b181b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc0b1c0),
                border: rgb(0xcdbecd),
                foreground: rgb(0x1b181b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xae9eae),
                border: rgb(0xa696a6),
                foreground: rgb(0x1b181b),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe1d6e1),
                border: rgb(0xd8cad8),
                foreground: rgb(0x9e8f9e),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b181b),
                border: rgb(0xf7f3f7),
                foreground: rgb(0xb3a4b3),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xc6b8c6),
                border: rgb(0xad9dad),
                foreground: rgb(0x1b181b),
                secondary_foreground: Some(
                    rgb(0x6b5e6b),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xad9dad),
                border: rgb(0xad9dad),
                foreground: rgb(0x1b181b),
                secondary_foreground: Some(
                    rgb(0x6b5e6b),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xa898a8),
                border: rgb(0xad9dad),
                foreground: rgb(0x1b181b),
                secondary_foreground: Some(
                    rgb(0x6b5e6b),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xa394a3),
                border: rgb(0x9b8c9b),
                foreground: rgb(0x1b181b),
                secondary_foreground: Some(
                    rgb(0x1b181b),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xc6b8c6),
                border: rgb(0xbaaaba),
                foreground: rgb(0x857785),
                secondary_foreground: Some(
                    rgb(0x857785),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b181b),
                border: rgb(0xf7f3f7),
                foreground: rgb(0xa494a4),
                secondary_foreground: Some(
                    rgb(0xa494a4),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf7f3f7),
                border: rgb(0xe5dce5),
                foreground: rgb(0x1b181b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe5dce5),
                border: rgb(0xe5dce5),
                foreground: rgb(0x1b181b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xddd0dd),
                border: rgb(0xe5dce5),
                foreground: rgb(0x1b181b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xcebfce),
                border: rgb(0xbaaaba),
                foreground: rgb(0x1b181b),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf7f3f7),
                border: rgb(0xeee7ee),
                foreground: rgb(0xa898a8),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b181b),
                border: rgb(0xf7f3f7),
                foreground: rgb(0xd2c4d2),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe2dffc),
                border: rgb(0xcac7fa),
                foreground: rgb(0x526aeb),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcac7fa),
                border: rgb(0xcac7fa),
                foreground: rgb(0x526aeb),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xbebbf8),
                border: rgb(0xcac7fa),
                foreground: rgb(0x526aeb),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xacaaf6),
                border: rgb(0x9597f3),
                foreground: rgb(0x50726),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe2dffc),
                border: rgb(0xd6d3fb),
                foreground: rgb(0x7780f0),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x50726),
                border: rgb(0xffffff),
                foreground: rgb(0xb1aff7),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xeae6d6),
                border: rgb(0xd9d4b6),
                foreground: rgb(0x918b3c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd9d4b6),
                border: rgb(0xd9d4b6),
                foreground: rgb(0x918b3c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd0caa6),
                border: rgb(0xd9d4b6),
                foreground: rgb(0x918b3c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xc4bd91),
                border: rgb(0xb6af78),
                foreground: rgb(0x90804),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xeae6d6),
                border: rgb(0xe1ddc6),
                foreground: rgb(0xa49d5a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x90804),
                border: rgb(0xffffff),
                foreground: rgb(0xc8c197),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf5e6d5),
                border: rgb(0xebd3b5),
                foreground: rgb(0xbb8a36),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xebd3b5),
                border: rgb(0xebd3b5),
                foreground: rgb(0xbb8a36),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe6caa4),
                border: rgb(0xebd3b5),
                foreground: rgb(0xbb8a36),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xdfbc8e),
                border: rgb(0xd5ae75),
                foreground: rgb(0x160804),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf5e6d5),
                border: rgb(0xf0dcc5),
                foreground: rgb(0xc99c56),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x160804),
                border: rgb(0xffffff),
                foreground: rgb(0xe1c194),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfcd9d1),
                border: rgb(0xf7bcae),
                foreground: rgb(0xca412c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf7bcae),
                border: rgb(0xf7bcae),
                foreground: rgb(0xca412c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xf3ad9c),
                border: rgb(0xf7bcae),
                foreground: rgb(0xca412c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xed9885),
                border: rgb(0xe4806a),
                foreground: rgb(0x250503),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfcd9d1),
                border: rgb(0xfacbc0),
                foreground: rgb(0xd8624a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x250503),
                border: rgb(0xffffff),
                foreground: rgb(0xef9e8b),
                secondary_foreground: None,
            },
        },
    }
}
