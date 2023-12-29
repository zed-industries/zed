use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_plateau_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Plateau Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xebe3e3),
                border: rgb(0xcfc7c7),
                foreground: rgb(0x1b1818),
                secondary_foreground: Some(
                    rgb(0x5a5252),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcfc7c7),
                border: rgb(0xcfc7c7),
                foreground: rgb(0x1b1818),
                secondary_foreground: Some(
                    rgb(0x5a5252),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb5aeae),
                border: rgb(0xcfc7c7),
                foreground: rgb(0x1b1818),
                secondary_foreground: Some(
                    rgb(0x5a5252),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x908b8b),
                border: rgb(0x857f7f),
                foreground: rgb(0x1b1818),
                secondary_foreground: Some(
                    rgb(0x1b1818),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xebe3e3),
                border: rgb(0xe7dfdf),
                foreground: rgb(0x7e7777),
                secondary_foreground: Some(
                    rgb(0x7e7777),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b1818),
                border: rgb(0xf4ecec),
                foreground: rgb(0x9b9696),
                secondary_foreground: Some(
                    rgb(0x9b9696),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xebe3e3),
                border: rgb(0xcfc7c7),
                foreground: rgb(0x1b1818),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcfc7c7),
                border: rgb(0xcfc7c7),
                foreground: rgb(0x1b1818),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb5aeae),
                border: rgb(0xcfc7c7),
                foreground: rgb(0x1b1818),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x908b8b),
                border: rgb(0x857f7f),
                foreground: rgb(0x1b1818),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xebe3e3),
                border: rgb(0xe7dfdf),
                foreground: rgb(0x7e7777),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b1818),
                border: rgb(0xf4ecec),
                foreground: rgb(0x9b9696),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xc1bbbb),
                border: rgb(0x8e8989),
                foreground: rgb(0x1b1818),
                secondary_foreground: Some(
                    rgb(0x5a5252),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x8e8989),
                border: rgb(0x8e8989),
                foreground: rgb(0x1b1818),
                secondary_foreground: Some(
                    rgb(0x5a5252),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x878181),
                border: rgb(0x8e8989),
                foreground: rgb(0x1b1818),
                secondary_foreground: Some(
                    rgb(0x5a5252),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x837c7c),
                border: rgb(0x7c7575),
                foreground: rgb(0x1b1818),
                secondary_foreground: Some(
                    rgb(0x1b1818),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xc1bbbb),
                border: rgb(0xa8a2a2),
                foreground: rgb(0x6e6666),
                secondary_foreground: Some(
                    rgb(0x6e6666),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b1818),
                border: rgb(0xf4ecec),
                foreground: rgb(0x837d7d),
                secondary_foreground: Some(
                    rgb(0x837d7d),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf4ecec),
                border: rgb(0xede5e5),
                foreground: rgb(0x1b1818),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xede5e5),
                border: rgb(0xede5e5),
                foreground: rgb(0x1b1818),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe9e1e1),
                border: rgb(0xede5e5),
                foreground: rgb(0x1b1818),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd1caca),
                border: rgb(0xa8a2a2),
                foreground: rgb(0x1b1818),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf4ecec),
                border: rgb(0xf0e8e8),
                foreground: rgb(0x878181),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b1818),
                border: rgb(0xf4ecec),
                foreground: rgb(0xdcd4d4),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe4e1f5),
                border: rgb(0xcecaec),
                foreground: rgb(0x7372ca),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcecaec),
                border: rgb(0xcecaec),
                foreground: rgb(0x7372ca),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc3bfe8),
                border: rgb(0xcecaec),
                foreground: rgb(0x7372ca),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xb4afe3),
                border: rgb(0xa29ddb),
                foreground: rgb(0x8070c),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe4e1f5),
                border: rgb(0xd9d5f1),
                foreground: rgb(0x8a87d2),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x8070c),
                border: rgb(0xffffff),
                foreground: rgb(0xb8b3e4),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdae7e7),
                border: rgb(0xbfd4d4),
                foreground: rgb(0x4c8b8b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbfd4d4),
                border: rgb(0xbfd4d4),
                foreground: rgb(0x4c8b8b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb0cbca),
                border: rgb(0xbfd4d4),
                foreground: rgb(0x4c8b8b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9cbebd),
                border: rgb(0x85afaf),
                foreground: rgb(0x50909),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdae7e7),
                border: rgb(0xcddddd),
                foreground: rgb(0x699d9d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x50909),
                border: rgb(0xffffff),
                foreground: rgb(0xa2c2c1),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xeee0d5),
                border: rgb(0xe0c9b5),
                foreground: rgb(0xa06e3c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe0c9b5),
                border: rgb(0xe0c9b5),
                foreground: rgb(0xa06e3c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd8bda5),
                border: rgb(0xe0c9b5),
                foreground: rgb(0xa06e3c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xceac8f),
                border: rgb(0xc29a76),
                foreground: rgb(0xb0704),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xeee0d5),
                border: rgb(0xe7d4c5),
                foreground: rgb(0xb18458),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xb0704),
                border: rgb(0xffffff),
                foreground: rgb(0xd2b195),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfadbd7),
                border: rgb(0xf4bfba),
                foreground: rgb(0xca4a4a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf4bfba),
                border: rgb(0xf4bfba),
                foreground: rgb(0xca4a4a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xf0b1ab),
                border: rgb(0xf4bfba),
                foreground: rgb(0xca4a4a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xea9d96),
                border: rgb(0xe2857f),
                foreground: rgb(0x1f0605),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfadbd7),
                border: rgb(0xf7cdc9),
                foreground: rgb(0xd76863),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1f0605),
                border: rgb(0xffffff),
                foreground: rgb(0xeca29c),
                secondary_foreground: None,
            },
        },
    }
}
