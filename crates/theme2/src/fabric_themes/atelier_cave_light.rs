use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_cave_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Cave Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe6e3eb),
                border: rgb(0xcbc8d1),
                foreground: rgb(0x19171c),
                secondary_foreground: Some(
                    rgb(0x5a5462),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcbc8d1),
                border: rgb(0xcbc8d1),
                foreground: rgb(0x19171c),
                secondary_foreground: Some(
                    rgb(0x5a5462),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb3afb9),
                border: rgb(0xcbc8d1),
                foreground: rgb(0x19171c),
                secondary_foreground: Some(
                    rgb(0x5a5462),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x918d98),
                border: rgb(0x86818e),
                foreground: rgb(0x19171c),
                secondary_foreground: Some(
                    rgb(0x19171c),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe6e3eb),
                border: rgb(0xe2dfe7),
                foreground: rgb(0x7e7987),
                secondary_foreground: Some(
                    rgb(0x7e7987),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x19171c),
                border: rgb(0xefecf4),
                foreground: rgb(0x9b97a2),
                secondary_foreground: Some(
                    rgb(0x9b97a2),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe6e3eb),
                border: rgb(0xcbc8d1),
                foreground: rgb(0x19171c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcbc8d1),
                border: rgb(0xcbc8d1),
                foreground: rgb(0x19171c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb3afb9),
                border: rgb(0xcbc8d1),
                foreground: rgb(0x19171c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x918d98),
                border: rgb(0x86818e),
                foreground: rgb(0x19171c),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe6e3eb),
                border: rgb(0xe2dfe7),
                foreground: rgb(0x7e7987),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x19171c),
                border: rgb(0xefecf4),
                foreground: rgb(0x9b97a2),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xbfbcc5),
                border: rgb(0x8f8b96),
                foreground: rgb(0x19171c),
                secondary_foreground: Some(
                    rgb(0x5a5462),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x8f8b96),
                border: rgb(0x8f8b96),
                foreground: rgb(0x19171c),
                secondary_foreground: Some(
                    rgb(0x5a5462),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x88838f),
                border: rgb(0x8f8b96),
                foreground: rgb(0x19171c),
                secondary_foreground: Some(
                    rgb(0x5a5462),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x837e8b),
                border: rgb(0x7c7685),
                foreground: rgb(0x19171c),
                secondary_foreground: Some(
                    rgb(0x19171c),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xbfbcc5),
                border: rgb(0xa7a3ad),
                foreground: rgb(0x6e6876),
                secondary_foreground: Some(
                    rgb(0x6e6876),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x19171c),
                border: rgb(0xefecf4),
                foreground: rgb(0x847f8c),
                secondary_foreground: Some(
                    rgb(0x847f8c),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xefecf4),
                border: rgb(0xe8e5ed),
                foreground: rgb(0x19171c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe8e5ed),
                border: rgb(0xe8e5ed),
                foreground: rgb(0x19171c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe4e1e9),
                border: rgb(0xe8e5ed),
                foreground: rgb(0x19171c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xcdcad3),
                border: rgb(0xa7a3ad),
                foreground: rgb(0x19171c),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xefecf4),
                border: rgb(0xebe8f0),
                foreground: rgb(0x88838f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x19171c),
                border: rgb(0xefecf4),
                foreground: rgb(0xd8d4dd),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe1e0f9),
                border: rgb(0xc9c8f3),
                foreground: rgb(0x586dda),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc9c8f3),
                border: rgb(0xc9c8f3),
                foreground: rgb(0x586dda),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xbdbcf0),
                border: rgb(0xc9c8f3),
                foreground: rgb(0x586dda),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xababed),
                border: rgb(0x9599e7),
                foreground: rgb(0x7071a),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe1e0f9),
                border: rgb(0xd5d3f6),
                foreground: rgb(0x7982e1),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x7071a),
                border: rgb(0xffffff),
                foreground: rgb(0xb0b0ed),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xd7e9e8),
                border: rgb(0xb9d7d6),
                foreground: rgb(0x2c9292),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xb9d7d6),
                border: rgb(0xb9d7d6),
                foreground: rgb(0x2c9292),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xa9cecd),
                border: rgb(0xb9d7d6),
                foreground: rgb(0x2c9292),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x93c2c1),
                border: rgb(0x78b5b4),
                foreground: rgb(0x50909),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xd7e9e8),
                border: rgb(0xc9e0df),
                foreground: rgb(0x56a3a3),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x50909),
                border: rgb(0xffffff),
                foreground: rgb(0x99c6c5),
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
                background: rgb(0xf5dae2),
                border: rgb(0xecbecd),
                foreground: rgb(0xbe4778),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xecbecd),
                border: rgb(0xecbecd),
                foreground: rgb(0xbe4778),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe7afc1),
                border: rgb(0xecbecd),
                foreground: rgb(0xbe4778),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xe09bb2),
                border: rgb(0xd783a1),
                foreground: rgb(0xd0507),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf5dae2),
                border: rgb(0xf1ccd7),
                foreground: rgb(0xcb668c),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xd0507),
                border: rgb(0xffffff),
                foreground: rgb(0xe2a1b7),
                secondary_foreground: None,
            },
        },
    }
}
