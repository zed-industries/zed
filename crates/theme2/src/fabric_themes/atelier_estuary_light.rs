use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_estuary_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Estuary Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xebeae3ff),
                border: rgba(0xd1d0c6ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: Some(
                    rgba(0x61604fff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd1d0c6ff),
                border: rgba(0xd1d0c6ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: Some(
                    rgba(0x61604fff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb9b8acff),
                border: rgba(0xd1d0c6ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: Some(
                    rgba(0x61604fff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x989788ff),
                border: rgba(0x8e8c7bff),
                foreground: rgba(0x22221bff),
                secondary_foreground: Some(
                    rgba(0x22221bff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xebeae3ff),
                border: rgba(0xe7e6dfff),
                foreground: rgba(0x878573ff),
                secondary_foreground: Some(
                    rgba(0x878573ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x22221bff),
                border: rgba(0xf4f3ecff),
                foreground: rgba(0xa2a192ff),
                secondary_foreground: Some(
                    rgba(0xa2a192ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xebeae3ff),
                border: rgba(0xd1d0c6ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd1d0c6ff),
                border: rgba(0xd1d0c6ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb9b8acff),
                border: rgba(0xd1d0c6ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x989788ff),
                border: rgba(0x8e8c7bff),
                foreground: rgba(0x22221bff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xebeae3ff),
                border: rgba(0xe7e6dfff),
                foreground: rgba(0x878573ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x22221bff),
                border: rgba(0xf4f3ecff),
                foreground: rgba(0xa2a192ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xc5c4b9ff),
                border: rgba(0x969585ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: Some(
                    rgba(0x61604fff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x969585ff),
                border: rgba(0x969585ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: Some(
                    rgba(0x61604fff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x8f8e7dff),
                border: rgba(0x969585ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: Some(
                    rgba(0x61604fff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x8b8a78ff),
                border: rgba(0x858371ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: Some(
                    rgba(0x22221bff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xc5c4b9ff),
                border: rgba(0xadac9fff),
                foreground: rgba(0x767463ff),
                secondary_foreground: Some(
                    rgba(0x767463ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x22221bff),
                border: rgba(0xf4f3ecff),
                foreground: rgba(0x8c8a79ff),
                secondary_foreground: Some(
                    rgba(0x8c8a79ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf4f3ecff),
                border: rgba(0xedece5ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xedece5ff),
                border: rgba(0xedece5ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe9e8e1ff),
                border: rgba(0xedece5ff),
                foreground: rgba(0x22221bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd3d2c9ff),
                border: rgba(0xadac9fff),
                foreground: rgba(0x22221bff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf4f3ecff),
                border: rgba(0xf0efe8ff),
                foreground: rgba(0x8f8e7dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x22221bff),
                border: rgba(0xf4f3ecff),
                foreground: rgba(0xdddcd4ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xd9ecdfff),
                border: rgba(0xbbddc6ff),
                foreground: rgba(0x38a166ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbbddc6ff),
                border: rgba(0xbbddc6ff),
                foreground: rgba(0x38a166ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xacd6baff),
                border: rgba(0xbbddc6ff),
                foreground: rgba(0x38a166ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x97cba8ff),
                border: rgba(0x7dc095ff),
                foreground: rgba(0x050a06ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xd9ecdfff),
                border: rgba(0xcae5d2ff),
                foreground: rgba(0x5db07dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x050a06ff),
                border: rgba(0xffffffff),
                foreground: rgba(0x9dcfadff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe6e9d3ff),
                border: rgba(0xd2d8b1ff),
                foreground: rgba(0x7d9728ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd2d8b1ff),
                border: rgba(0xd2d8b1ff),
                foreground: rgba(0x7d9728ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc8d0a0ff),
                border: rgba(0xd2d8b1ff),
                foreground: rgba(0x7d9728ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xb9c488ff),
                border: rgba(0xa9b86dff),
                foreground: rgba(0x080903ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe6e9d3ff),
                border: rgba(0xdce1c2ff),
                foreground: rgba(0x93a74bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x080903ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xbec88eff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf0e9d1ff),
                border: rgba(0xe3d8adff),
                foreground: rgba(0xa59810ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe3d8adff),
                border: rgba(0xe3d8adff),
                foreground: rgba(0xa59810ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xdcd09bff),
                border: rgba(0xe3d8adff),
                foreground: rgba(0xa59810ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd2c482ff),
                border: rgba(0xc6b865ff),
                foreground: rgba(0x0b0903ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf0e9d1ff),
                border: rgba(0xeae1bfff),
                foreground: rgba(0xb6a840ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0b0903ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xd6c889ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf6ded4ff),
                border: rgba(0xedc5b3ff),
                foreground: rgba(0xba6337ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xedc5b3ff),
                border: rgba(0xedc5b3ff),
                foreground: rgba(0xba6337ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe7b9a2ff),
                border: rgba(0xedc5b3ff),
                foreground: rgba(0xba6337ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xe0a78cff),
                border: rgba(0xd69372ff),
                foreground: rgba(0x120604ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf6ded4ff),
                border: rgba(0xf1d1c4ff),
                foreground: rgba(0xc97a54ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x120604ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xe2ac92ff),
                secondary_foreground: None,
            },
        },
    }
}
