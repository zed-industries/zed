use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_dune_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Dune Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xeeebd7),
                border: rgb(0xd7d3be),
                foreground: rgb(0x20201d),
                secondary_foreground: Some(
                    rgb(0x706d5f),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd7d3be),
                border: rgb(0xd7d3be),
                foreground: rgb(0x20201d),
                secondary_foreground: Some(
                    rgb(0x706d5f),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc4c0ab),
                border: rgb(0xd7d3be),
                foreground: rgb(0x20201d),
                secondary_foreground: Some(
                    rgb(0x706d5f),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xaaa690),
                border: rgb(0xa19d87),
                foreground: rgb(0x20201d),
                secondary_foreground: Some(
                    rgb(0x20201d),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xeeebd7),
                border: rgb(0xe8e4cf),
                foreground: rgb(0x999580),
                secondary_foreground: Some(
                    rgb(0x999580),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x20201d),
                border: rgb(0xfefbec),
                foreground: rgb(0xb2ae98),
                secondary_foreground: Some(
                    rgb(0xb2ae98),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xeeebd7),
                border: rgb(0xd7d3be),
                foreground: rgb(0x20201d),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd7d3be),
                border: rgb(0xd7d3be),
                foreground: rgb(0x20201d),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc4c0ab),
                border: rgb(0xd7d3be),
                foreground: rgb(0x20201d),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xaaa690),
                border: rgb(0xa19d87),
                foreground: rgb(0x20201d),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xeeebd7),
                border: rgb(0xe8e4cf),
                foreground: rgb(0x999580),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x20201d),
                border: rgb(0xfefbec),
                foreground: rgb(0xb2ae98),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xcecab4),
                border: rgb(0xa8a48e),
                foreground: rgb(0x20201d),
                secondary_foreground: Some(
                    rgb(0x706d5f),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xa8a48e),
                border: rgb(0xa8a48e),
                foreground: rgb(0x20201d),
                secondary_foreground: Some(
                    rgb(0x706d5f),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xa39f89),
                border: rgb(0xa8a48e),
                foreground: rgb(0x20201d),
                secondary_foreground: Some(
                    rgb(0x706d5f),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x9e9a85),
                border: rgb(0x97937e),
                foreground: rgb(0x20201d),
                secondary_foreground: Some(
                    rgb(0x20201d),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xcecab4),
                border: rgb(0xbbb7a1),
                foreground: rgb(0x878471),
                secondary_foreground: Some(
                    rgb(0x878471),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x20201d),
                border: rgb(0xfefbec),
                foreground: rgb(0x9f9b85),
                secondary_foreground: Some(
                    rgb(0x9f9b85),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfefbec),
                border: rgb(0xf2eedc),
                foreground: rgb(0x20201d),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf2eedc),
                border: rgb(0xf2eedc),
                foreground: rgb(0x20201d),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xebe7d3),
                border: rgb(0xf2eedc),
                foreground: rgb(0x20201d),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd9d5bf),
                border: rgb(0xbbb7a1),
                foreground: rgb(0x20201d),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfefbec),
                border: rgb(0xf8f4e4),
                foreground: rgb(0xa39f89),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x20201d),
                border: rgb(0xfefbec),
                foreground: rgb(0xe0dcc7),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe3e5fa),
                border: rgb(0xcdd1f5),
                foreground: rgb(0x6784e0),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcdd1f5),
                border: rgb(0xcdd1f5),
                foreground: rgb(0x6784e0),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc1c7f3),
                border: rgb(0xcdd1f5),
                foreground: rgb(0x6784e0),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xb0b9ef),
                border: rgb(0x9daaeb),
                foreground: rgb(0x7081d),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe3e5fa),
                border: rgb(0xd7daf7),
                foreground: rgb(0x8396e6),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x7081d),
                border: rgb(0xffffff),
                foreground: rgb(0xb5bdf0),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe0eed6),
                border: rgb(0xc9e1b7),
                foreground: rgb(0x61ac3a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc9e1b7),
                border: rgb(0xc9e1b7),
                foreground: rgb(0x61ac3a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xbcdaa7),
                border: rgb(0xc9e1b7),
                foreground: rgb(0x61ac3a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xabd192),
                border: rgb(0x96c779),
                foreground: rgb(0x70b04),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe0eed6),
                border: rgb(0xd4e8c7),
                foreground: rgb(0x7cb95a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x70b04),
                border: rgb(0xffffff),
                foreground: rgb(0xb0d498),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf2e8d1),
                border: rgb(0xe7d7ae),
                foreground: rgb(0xae9515),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe7d7ae),
                border: rgb(0xe7d7ae),
                foreground: rgb(0xae9515),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe0ce9c),
                border: rgb(0xe7d7ae),
                foreground: rgb(0xae9515),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd8c283),
                border: rgb(0xcdb667),
                foreground: rgb(0x130903),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf2e8d1),
                border: rgb(0xede0c0),
                foreground: rgb(0xbea542),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x130903),
                border: rgb(0xffffff),
                foreground: rgb(0xdbc78a),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xffd9d4),
                border: rgb(0xfcbcb2),
                foreground: rgb(0xd73838),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xfcbcb2),
                border: rgb(0xfcbcb2),
                foreground: rgb(0xd73838),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xf9aca2),
                border: rgb(0xfcbcb2),
                foreground: rgb(0xd73838),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xf5978b),
                border: rgb(0xee7e72),
                foreground: rgb(0x2c0404),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xffd9d4),
                border: rgb(0xfecbc3),
                foreground: rgb(0xe35e54),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x2c0404),
                border: rgb(0xffffff),
                foreground: rgb(0xf69d91),
                secondary_foreground: None,
            },
        },
    }
}
