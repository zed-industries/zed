use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_dune_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Dune Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xeeebd7ff),
                border: rgba(0xd7d3beff),
                foreground: rgba(0x20201dff),
                secondary_foreground: Some(
                    rgba(0x706d5fff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd7d3beff),
                border: rgba(0xd7d3beff),
                foreground: rgba(0x20201dff),
                secondary_foreground: Some(
                    rgba(0x706d5fff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc4c0abff),
                border: rgba(0xd7d3beff),
                foreground: rgba(0x20201dff),
                secondary_foreground: Some(
                    rgba(0x706d5fff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xaaa690ff),
                border: rgba(0xa19d87ff),
                foreground: rgba(0x20201dff),
                secondary_foreground: Some(
                    rgba(0x20201dff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xeeebd7ff),
                border: rgba(0xe8e4cfff),
                foreground: rgba(0x999580ff),
                secondary_foreground: Some(
                    rgba(0x999580ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x20201dff),
                border: rgba(0xfefbecff),
                foreground: rgba(0xb2ae98ff),
                secondary_foreground: Some(
                    rgba(0xb2ae98ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xeeebd7ff),
                border: rgba(0xd7d3beff),
                foreground: rgba(0x20201dff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd7d3beff),
                border: rgba(0xd7d3beff),
                foreground: rgba(0x20201dff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc4c0abff),
                border: rgba(0xd7d3beff),
                foreground: rgba(0x20201dff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xaaa690ff),
                border: rgba(0xa19d87ff),
                foreground: rgba(0x20201dff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xeeebd7ff),
                border: rgba(0xe8e4cfff),
                foreground: rgba(0x999580ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x20201dff),
                border: rgba(0xfefbecff),
                foreground: rgba(0xb2ae98ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xcecab4ff),
                border: rgba(0xa8a48eff),
                foreground: rgba(0x20201dff),
                secondary_foreground: Some(
                    rgba(0x706d5fff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xa8a48eff),
                border: rgba(0xa8a48eff),
                foreground: rgba(0x20201dff),
                secondary_foreground: Some(
                    rgba(0x706d5fff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xa39f89ff),
                border: rgba(0xa8a48eff),
                foreground: rgba(0x20201dff),
                secondary_foreground: Some(
                    rgba(0x706d5fff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x9e9a85ff),
                border: rgba(0x97937eff),
                foreground: rgba(0x20201dff),
                secondary_foreground: Some(
                    rgba(0x20201dff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xcecab4ff),
                border: rgba(0xbbb7a1ff),
                foreground: rgba(0x878471ff),
                secondary_foreground: Some(
                    rgba(0x878471ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x20201dff),
                border: rgba(0xfefbecff),
                foreground: rgba(0x9f9b85ff),
                secondary_foreground: Some(
                    rgba(0x9f9b85ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfefbecff),
                border: rgba(0xf2eedcff),
                foreground: rgba(0x20201dff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf2eedcff),
                border: rgba(0xf2eedcff),
                foreground: rgba(0x20201dff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xebe7d3ff),
                border: rgba(0xf2eedcff),
                foreground: rgba(0x20201dff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd9d5bfff),
                border: rgba(0xbbb7a1ff),
                foreground: rgba(0x20201dff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfefbecff),
                border: rgba(0xf8f4e4ff),
                foreground: rgba(0xa39f89ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x20201dff),
                border: rgba(0xfefbecff),
                foreground: rgba(0xe0dcc7ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe3e5faff),
                border: rgba(0xcdd1f5ff),
                foreground: rgba(0x6784e0ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcdd1f5ff),
                border: rgba(0xcdd1f5ff),
                foreground: rgba(0x6784e0ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc1c7f3ff),
                border: rgba(0xcdd1f5ff),
                foreground: rgba(0x6784e0ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xb0b9efff),
                border: rgba(0x9daaebff),
                foreground: rgba(0x07081dff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe3e5faff),
                border: rgba(0xd7daf7ff),
                foreground: rgba(0x8396e6ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x07081dff),
                border: rgba(0xffffffff),
                foreground: rgba(0xb5bdf0ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe0eed6ff),
                border: rgba(0xc9e1b7ff),
                foreground: rgba(0x61ac3aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc9e1b7ff),
                border: rgba(0xc9e1b7ff),
                foreground: rgba(0x61ac3aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xbcdaa7ff),
                border: rgba(0xc9e1b7ff),
                foreground: rgba(0x61ac3aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xabd192ff),
                border: rgba(0x96c779ff),
                foreground: rgba(0x070b04ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe0eed6ff),
                border: rgba(0xd4e8c7ff),
                foreground: rgba(0x7cb95aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x070b04ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xb0d498ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf2e8d1ff),
                border: rgba(0xe7d7aeff),
                foreground: rgba(0xae9515ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe7d7aeff),
                border: rgba(0xe7d7aeff),
                foreground: rgba(0xae9515ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe0ce9cff),
                border: rgba(0xe7d7aeff),
                foreground: rgba(0xae9515ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd8c283ff),
                border: rgba(0xcdb667ff),
                foreground: rgba(0x130903ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf2e8d1ff),
                border: rgba(0xede0c0ff),
                foreground: rgba(0xbea542ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x130903ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xdbc78aff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xffd9d4ff),
                border: rgba(0xfcbcb2ff),
                foreground: rgba(0xd73838ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xfcbcb2ff),
                border: rgba(0xfcbcb2ff),
                foreground: rgba(0xd73838ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xf9aca2ff),
                border: rgba(0xfcbcb2ff),
                foreground: rgba(0xd73838ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xf5978bff),
                border: rgba(0xee7e72ff),
                foreground: rgba(0x2c0404ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xffd9d4ff),
                border: rgba(0xfecbc3ff),
                foreground: rgba(0xe35e54ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x2c0404ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xf69d91ff),
                secondary_foreground: None,
            },
        },
    }
}
