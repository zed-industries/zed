use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn rose_pine_dawn() -> FabricTheme {
    FabricTheme {
        name: "Rosé Pine Dawn",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfef9f2ff),
                border: rgba(0xe5e0dfff),
                foreground: rgba(0x575279ff),
                secondary_foreground: Some(
                    rgba(0x706c8cff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe5e0dfff),
                border: rgba(0xe5e0dfff),
                foreground: rgba(0x575279ff),
                secondary_foreground: Some(
                    rgba(0x706c8cff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd4d0d2ff),
                border: rgba(0xe5e0dfff),
                foreground: rgba(0x575279ff),
                secondary_foreground: Some(
                    rgba(0x706c8cff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xdbd5d4ff),
                border: rgba(0xdbd3d1ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: Some(
                    rgba(0x575279ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfef9f2ff),
                border: rgba(0xf6f1ebff),
                foreground: rgba(0xb1abb5ff),
                secondary_foreground: Some(
                    rgba(0xb1abb5ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x575279ff),
                border: rgba(0xfaf4edff),
                foreground: rgba(0xd6d1d1ff),
                secondary_foreground: Some(
                    rgba(0xd6d1d1ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfef9f2ff),
                border: rgba(0xe5e0dfff),
                foreground: rgba(0x575279ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe5e0dfff),
                border: rgba(0xe5e0dfff),
                foreground: rgba(0x575279ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd4d0d2ff),
                border: rgba(0xe5e0dfff),
                foreground: rgba(0x575279ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xdbd5d4ff),
                border: rgba(0xdbd3d1ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfef9f2ff),
                border: rgba(0xf6f1ebff),
                foreground: rgba(0xb1abb5ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x575279ff),
                border: rgba(0xfaf4edff),
                foreground: rgba(0xd6d1d1ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdcd8d8ff),
                border: rgba(0xdcd6d5ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: Some(
                    rgba(0x706c8cff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdcd6d5ff),
                border: rgba(0xdcd6d5ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: Some(
                    rgba(0x706c8cff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xefe6dfff),
                border: rgba(0xdcd6d5ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: Some(
                    rgba(0x706c8cff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xc1bac1ff),
                border: rgba(0xa9a3b0ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: Some(
                    rgba(0x575279ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdcd8d8ff),
                border: rgba(0xd0cccfff),
                foreground: rgba(0x938fa3ff),
                secondary_foreground: Some(
                    rgba(0x938fa3ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x575279ff),
                border: rgba(0xfaf4edff),
                foreground: rgba(0xc7c0c5ff),
                secondary_foreground: Some(
                    rgba(0xc7c0c5ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfaf4edff),
                border: rgba(0xfdf8f1ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xfdf8f1ff),
                border: rgba(0xfdf8f1ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xfdf8f2ff),
                border: rgba(0xfdf8f1ff),
                foreground: rgba(0x575279ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xe6e1e0ff),
                border: rgba(0xd0cccfff),
                foreground: rgba(0x575279ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfaf4edff),
                border: rgba(0xfcf6efff),
                foreground: rgba(0xefe6dfff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x575279ff),
                border: rgba(0xfaf4edff),
                foreground: rgba(0xede9e5ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdde9ebff),
                border: rgba(0xc3d7dbff),
                foreground: rgba(0x57949fff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc3d7dbff),
                border: rgba(0xc3d7dbff),
                foreground: rgba(0x57949fff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb6cfd3ff),
                border: rgba(0xc3d7dbff),
                foreground: rgba(0x57949fff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xa3c3c9ff),
                border: rgba(0x8db6bdff),
                foreground: rgba(0x06090aff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdde9ebff),
                border: rgba(0xd0e0e3ff),
                foreground: rgba(0x72a5aeff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x06090aff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa8c7cdff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdbeee7ff),
                border: rgba(0xbee0d5ff),
                foreground: rgba(0x3eaa8eff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbee0d5ff),
                border: rgba(0xbee0d5ff),
                foreground: rgba(0x3eaa8eff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb0dacbff),
                border: rgba(0xbee0d5ff),
                foreground: rgba(0x3eaa8eff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9bd0bfff),
                border: rgba(0x82c6b1ff),
                foreground: rgba(0x060a09ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdbeee7ff),
                border: rgba(0xcde7deff),
                foreground: rgba(0x63b89fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x060a09ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa1d4c3ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xffebd6ff),
                border: rgba(0xffdab7ff),
                foreground: rgba(0xe99d35ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xffdab7ff),
                border: rgba(0xffdab7ff),
                foreground: rgba(0xe99d35ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xfed2a6ff),
                border: rgba(0xffdab7ff),
                foreground: rgba(0xe99d35ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xfbc891ff),
                border: rgba(0xf7bc77ff),
                foreground: rgba(0x330704ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xffebd6ff),
                border: rgba(0xffe2c7ff),
                foreground: rgba(0xf1ac57ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x330704ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xfccb97ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf1dfe3ff),
                border: rgba(0xe6c6cdff),
                foreground: rgba(0xb4647aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe6c6cdff),
                border: rgba(0xe6c6cdff),
                foreground: rgba(0xb4647aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe0bac2ff),
                border: rgba(0xe6c6cdff),
                foreground: rgba(0xb4647aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd8a8b3ff),
                border: rgba(0xce94a3ff),
                foreground: rgba(0x0b0708ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf1dfe3ff),
                border: rgba(0xecd2d8ff),
                foreground: rgba(0xc17b8eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0b0708ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xdbadb8ff),
                secondary_foreground: None,
            },
        },
    }
}
