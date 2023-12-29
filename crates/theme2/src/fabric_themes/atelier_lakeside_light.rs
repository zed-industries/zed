use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_lakeside_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Lakeside Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xcdeaf9),
                border: rgb(0xb0d3e5),
                foreground: rgb(0x161b1d),
                secondary_foreground: Some(
                    rgb(0x526f7d),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xb0d3e5),
                border: rgb(0xb0d3e5),
                foreground: rgb(0x161b1d),
                secondary_foreground: Some(
                    rgb(0x526f7d),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x9dc0d2),
                border: rgb(0xb0d3e5),
                foreground: rgb(0x161b1d),
                secondary_foreground: Some(
                    rgb(0x526f7d),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x82a6b8),
                border: rgb(0x799daf),
                foreground: rgb(0x161b1d),
                secondary_foreground: Some(
                    rgb(0x161b1d),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xcdeaf9),
                border: rgb(0xc1e4f6),
                foreground: rgb(0x7195a8),
                secondary_foreground: Some(
                    rgb(0x7195a8),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x161b1d),
                border: rgb(0xebf8ff),
                foreground: rgb(0x8aaec0),
                secondary_foreground: Some(
                    rgb(0x8aaec0),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xcdeaf9),
                border: rgb(0xb0d3e5),
                foreground: rgb(0x161b1d),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xb0d3e5),
                border: rgb(0xb0d3e5),
                foreground: rgb(0x161b1d),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x9dc0d2),
                border: rgb(0xb0d3e5),
                foreground: rgb(0x161b1d),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x82a6b8),
                border: rgb(0x799daf),
                foreground: rgb(0x161b1d),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xcdeaf9),
                border: rgb(0xc1e4f6),
                foreground: rgb(0x7195a8),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x161b1d),
                border: rgb(0xebf8ff),
                foreground: rgb(0x8aaec0),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xa6cadc),
                border: rgb(0x80a4b6),
                foreground: rgb(0x161b1d),
                secondary_foreground: Some(
                    rgb(0x526f7d),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x80a4b6),
                border: rgb(0x80a4b6),
                foreground: rgb(0x161b1d),
                secondary_foreground: Some(
                    rgb(0x526f7d),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x7b9fb1),
                border: rgb(0x80a4b6),
                foreground: rgb(0x161b1d),
                secondary_foreground: Some(
                    rgb(0x526f7d),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x769aad),
                border: rgb(0x6f93a6),
                foreground: rgb(0x161b1d),
                secondary_foreground: Some(
                    rgb(0x161b1d),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xa6cadc),
                border: rgb(0x93b7c9),
                foreground: rgb(0x628496),
                secondary_foreground: Some(
                    rgb(0x628496),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x161b1d),
                border: rgb(0xebf8ff),
                foreground: rgb(0x779bad),
                secondary_foreground: Some(
                    rgb(0x779bad),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xebf8ff),
                border: rgb(0xd3edfa),
                foreground: rgb(0x161b1d),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd3edfa),
                border: rgb(0xd3edfa),
                foreground: rgb(0x161b1d),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc7e7f7),
                border: rgb(0xd3edfa),
                foreground: rgb(0x161b1d),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xb1d5e7),
                border: rgb(0x93b7c9),
                foreground: rgb(0x161b1d),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xebf8ff),
                border: rgb(0xdff2fc),
                foreground: rgb(0x7b9fb1),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x161b1d),
                border: rgb(0xebf8ff),
                foreground: rgb(0xb9dcee),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xd8e4ee),
                border: rgb(0xbacfe1),
                foreground: rgb(0x277fad),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbacfe1),
                border: rgb(0xbacfe1),
                foreground: rgb(0x277fad),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xaac5da),
                border: rgb(0xbacfe1),
                foreground: rgb(0x277fad),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x94b6d1),
                border: rgb(0x79a6c7),
                foreground: rgb(0x5080b),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xd8e4ee),
                border: rgb(0xc9d9e8),
                foreground: rgb(0x5592ba),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x5080b),
                border: rgb(0xffffff),
                foreground: rgb(0x9abbd4),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdde7d5),
                border: rgb(0xc2d5b6),
                foreground: rgb(0x578c3c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc2d5b6),
                border: rgb(0xc2d5b6),
                foreground: rgb(0x578c3c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb5cba6),
                border: rgb(0xc2d5b6),
                foreground: rgb(0x578c3c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xa2be90),
                border: rgb(0x8cb077),
                foreground: rgb(0x60904),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdde7d5),
                border: rgb(0xcfdec6),
                foreground: rgb(0x729e59),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x60904),
                border: rgb(0xffffff),
                foreground: rgb(0xa7c396),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xeae6d0),
                border: rgb(0xd8d3ab),
                foreground: rgb(0x8a8a11),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd8d3ab),
                border: rgb(0xd8d3ab),
                foreground: rgb(0x8a8a11),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xcfc999),
                border: rgb(0xd8d3ab),
                foreground: rgb(0x8a8a11),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xc2bc7f),
                border: rgb(0xb3ae63),
                foreground: rgb(0x90803),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xeae6d0),
                border: rgb(0xe1dcbd),
                foreground: rgb(0x9f9c3e),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x90803),
                border: rgb(0xffffff),
                foreground: rgb(0xc6c186),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfbd8e1),
                border: rgb(0xf6baca),
                foreground: rgb(0xd22f72),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf6baca),
                border: rgb(0xf6baca),
                foreground: rgb(0xd22f72),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xf3abbf),
                border: rgb(0xf6baca),
                foreground: rgb(0xd22f72),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xee95ae),
                border: rgb(0xe77b9d),
                foreground: rgb(0x220507),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfbd8e1),
                border: rgb(0xf9cad5),
                foreground: rgb(0xdd5987),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x220507),
                border: rgb(0xffffff),
                foreground: rgb(0xef9bb3),
                secondary_foreground: None,
            },
        },
    }
}
