use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_cave_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Cave Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe6e3ebff),
                border: rgba(0xcbc8d1ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: Some(
                    rgba(0x5a5462ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcbc8d1ff),
                border: rgba(0xcbc8d1ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: Some(
                    rgba(0x5a5462ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb3afb9ff),
                border: rgba(0xcbc8d1ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: Some(
                    rgba(0x5a5462ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x918d98ff),
                border: rgba(0x86818eff),
                foreground: rgba(0x19171cff),
                secondary_foreground: Some(
                    rgba(0x19171cff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe6e3ebff),
                border: rgba(0xe2dfe7ff),
                foreground: rgba(0x7e7987ff),
                secondary_foreground: Some(
                    rgba(0x7e7987ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x19171cff),
                border: rgba(0xefecf4ff),
                foreground: rgba(0x9b97a2ff),
                secondary_foreground: Some(
                    rgba(0x9b97a2ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe6e3ebff),
                border: rgba(0xcbc8d1ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcbc8d1ff),
                border: rgba(0xcbc8d1ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb3afb9ff),
                border: rgba(0xcbc8d1ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x918d98ff),
                border: rgba(0x86818eff),
                foreground: rgba(0x19171cff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe6e3ebff),
                border: rgba(0xe2dfe7ff),
                foreground: rgba(0x7e7987ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x19171cff),
                border: rgba(0xefecf4ff),
                foreground: rgba(0x9b97a2ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xbfbcc5ff),
                border: rgba(0x8f8b96ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: Some(
                    rgba(0x5a5462ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x8f8b96ff),
                border: rgba(0x8f8b96ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: Some(
                    rgba(0x5a5462ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x88838fff),
                border: rgba(0x8f8b96ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: Some(
                    rgba(0x5a5462ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x837e8bff),
                border: rgba(0x7c7685ff),
                foreground: rgba(0x19171cff),
                secondary_foreground: Some(
                    rgba(0x19171cff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xbfbcc5ff),
                border: rgba(0xa7a3adff),
                foreground: rgba(0x6e6876ff),
                secondary_foreground: Some(
                    rgba(0x6e6876ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x19171cff),
                border: rgba(0xefecf4ff),
                foreground: rgba(0x847f8cff),
                secondary_foreground: Some(
                    rgba(0x847f8cff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xefecf4ff),
                border: rgba(0xe8e5edff),
                foreground: rgba(0x19171cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe8e5edff),
                border: rgba(0xe8e5edff),
                foreground: rgba(0x19171cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe4e1e9ff),
                border: rgba(0xe8e5edff),
                foreground: rgba(0x19171cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xcdcad3ff),
                border: rgba(0xa7a3adff),
                foreground: rgba(0x19171cff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xefecf4ff),
                border: rgba(0xebe8f0ff),
                foreground: rgba(0x88838fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x19171cff),
                border: rgba(0xefecf4ff),
                foreground: rgba(0xd8d4ddff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe1e0f9ff),
                border: rgba(0xc9c8f3ff),
                foreground: rgba(0x586ddaff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc9c8f3ff),
                border: rgba(0xc9c8f3ff),
                foreground: rgba(0x586ddaff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xbdbcf0ff),
                border: rgba(0xc9c8f3ff),
                foreground: rgba(0x586ddaff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xababedff),
                border: rgba(0x9599e7ff),
                foreground: rgba(0x07071aff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe1e0f9ff),
                border: rgba(0xd5d3f6ff),
                foreground: rgba(0x7982e1ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x07071aff),
                border: rgba(0xffffffff),
                foreground: rgba(0xb0b0edff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xd7e9e8ff),
                border: rgba(0xb9d7d6ff),
                foreground: rgba(0x2c9292ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xb9d7d6ff),
                border: rgba(0xb9d7d6ff),
                foreground: rgba(0x2c9292ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xa9cecdff),
                border: rgba(0xb9d7d6ff),
                foreground: rgba(0x2c9292ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x93c2c1ff),
                border: rgba(0x78b5b4ff),
                foreground: rgba(0x050909ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xd7e9e8ff),
                border: rgba(0xc9e0dfff),
                foreground: rgba(0x56a3a3ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x050909ff),
                border: rgba(0xffffffff),
                foreground: rgba(0x99c6c5ff),
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
                background: rgba(0xf5dae2ff),
                border: rgba(0xecbecdff),
                foreground: rgba(0xbe4778ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xecbecdff),
                border: rgba(0xecbecdff),
                foreground: rgba(0xbe4778ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe7afc1ff),
                border: rgba(0xecbecdff),
                foreground: rgba(0xbe4778ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xe09bb2ff),
                border: rgba(0xd783a1ff),
                foreground: rgba(0x0d0507ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf5dae2ff),
                border: rgba(0xf1ccd7ff),
                foreground: rgba(0xcb668cff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0d0507ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xe2a1b7ff),
                secondary_foreground: None,
            },
        },
    }
}
