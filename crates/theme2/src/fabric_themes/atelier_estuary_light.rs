use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_estuary_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Estuary Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xebeae3),
                border: rgb(0xd1d0c6),
                foreground: rgb(0x22221b),
                secondary_foreground: Some(
                    rgb(0x61604f),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd1d0c6),
                border: rgb(0xd1d0c6),
                foreground: rgb(0x22221b),
                secondary_foreground: Some(
                    rgb(0x61604f),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb9b8ac),
                border: rgb(0xd1d0c6),
                foreground: rgb(0x22221b),
                secondary_foreground: Some(
                    rgb(0x61604f),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x989788),
                border: rgb(0x8e8c7b),
                foreground: rgb(0x22221b),
                secondary_foreground: Some(
                    rgb(0x22221b),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xebeae3),
                border: rgb(0xe7e6df),
                foreground: rgb(0x878573),
                secondary_foreground: Some(
                    rgb(0x878573),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x22221b),
                border: rgb(0xf4f3ec),
                foreground: rgb(0xa2a192),
                secondary_foreground: Some(
                    rgb(0xa2a192),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xebeae3),
                border: rgb(0xd1d0c6),
                foreground: rgb(0x22221b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd1d0c6),
                border: rgb(0xd1d0c6),
                foreground: rgb(0x22221b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb9b8ac),
                border: rgb(0xd1d0c6),
                foreground: rgb(0x22221b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x989788),
                border: rgb(0x8e8c7b),
                foreground: rgb(0x22221b),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xebeae3),
                border: rgb(0xe7e6df),
                foreground: rgb(0x878573),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x22221b),
                border: rgb(0xf4f3ec),
                foreground: rgb(0xa2a192),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xc5c4b9),
                border: rgb(0x969585),
                foreground: rgb(0x22221b),
                secondary_foreground: Some(
                    rgb(0x61604f),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x969585),
                border: rgb(0x969585),
                foreground: rgb(0x22221b),
                secondary_foreground: Some(
                    rgb(0x61604f),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x8f8e7d),
                border: rgb(0x969585),
                foreground: rgb(0x22221b),
                secondary_foreground: Some(
                    rgb(0x61604f),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x8b8a78),
                border: rgb(0x858371),
                foreground: rgb(0x22221b),
                secondary_foreground: Some(
                    rgb(0x22221b),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xc5c4b9),
                border: rgb(0xadac9f),
                foreground: rgb(0x767463),
                secondary_foreground: Some(
                    rgb(0x767463),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x22221b),
                border: rgb(0xf4f3ec),
                foreground: rgb(0x8c8a79),
                secondary_foreground: Some(
                    rgb(0x8c8a79),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf4f3ec),
                border: rgb(0xedece5),
                foreground: rgb(0x22221b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xedece5),
                border: rgb(0xedece5),
                foreground: rgb(0x22221b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe9e8e1),
                border: rgb(0xedece5),
                foreground: rgb(0x22221b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd3d2c9),
                border: rgb(0xadac9f),
                foreground: rgb(0x22221b),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf4f3ec),
                border: rgb(0xf0efe8),
                foreground: rgb(0x8f8e7d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x22221b),
                border: rgb(0xf4f3ec),
                foreground: rgb(0xdddcd4),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xd9ecdf),
                border: rgb(0xbbddc6),
                foreground: rgb(0x38a166),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbbddc6),
                border: rgb(0xbbddc6),
                foreground: rgb(0x38a166),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xacd6ba),
                border: rgb(0xbbddc6),
                foreground: rgb(0x38a166),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x97cba8),
                border: rgb(0x7dc095),
                foreground: rgb(0x50a06),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xd9ecdf),
                border: rgb(0xcae5d2),
                foreground: rgb(0x5db07d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x50a06),
                border: rgb(0xffffff),
                foreground: rgb(0x9dcfad),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe6e9d3),
                border: rgb(0xd2d8b1),
                foreground: rgb(0x7d9728),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd2d8b1),
                border: rgb(0xd2d8b1),
                foreground: rgb(0x7d9728),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc8d0a0),
                border: rgb(0xd2d8b1),
                foreground: rgb(0x7d9728),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xb9c488),
                border: rgb(0xa9b86d),
                foreground: rgb(0x80903),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe6e9d3),
                border: rgb(0xdce1c2),
                foreground: rgb(0x93a74b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x80903),
                border: rgb(0xffffff),
                foreground: rgb(0xbec88e),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf0e9d1),
                border: rgb(0xe3d8ad),
                foreground: rgb(0xa59810),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe3d8ad),
                border: rgb(0xe3d8ad),
                foreground: rgb(0xa59810),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xdcd09b),
                border: rgb(0xe3d8ad),
                foreground: rgb(0xa59810),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd2c482),
                border: rgb(0xc6b865),
                foreground: rgb(0xb0903),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf0e9d1),
                border: rgb(0xeae1bf),
                foreground: rgb(0xb6a840),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xb0903),
                border: rgb(0xffffff),
                foreground: rgb(0xd6c889),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf6ded4),
                border: rgb(0xedc5b3),
                foreground: rgb(0xba6337),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xedc5b3),
                border: rgb(0xedc5b3),
                foreground: rgb(0xba6337),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe7b9a2),
                border: rgb(0xedc5b3),
                foreground: rgb(0xba6337),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xe0a78c),
                border: rgb(0xd69372),
                foreground: rgb(0x120604),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf6ded4),
                border: rgb(0xf1d1c4),
                foreground: rgb(0xc97a54),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x120604),
                border: rgb(0xffffff),
                foreground: rgb(0xe2ac92),
                secondary_foreground: None,
            },
        },
    }
}
