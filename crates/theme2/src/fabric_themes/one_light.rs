use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn one_light() -> FabricTheme {
    FabricTheme {
        name: "One Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xebebecff),
                border: rgba(0xdfdfe0ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: Some(
                    rgba(0x7f8188ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdfdfe0ff),
                border: rgba(0xdfdfe0ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: Some(
                    rgba(0x7f8188ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd9d9daff),
                border: rgba(0xdfdfe0ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: Some(
                    rgba(0x7f8188ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xcacacaff),
                border: rgba(0xb9b9b9ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: Some(
                    rgba(0x383a41ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xebebecff),
                border: rgba(0xe5e5e6ff),
                foreground: rgba(0xa7a7a8ff),
                secondary_foreground: Some(
                    rgba(0xa7a7a8ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x383a41ff),
                border: rgba(0xfafafaff),
                foreground: rgba(0xcececfff),
                secondary_foreground: Some(
                    rgba(0xcececfff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xebebecff),
                border: rgba(0xdfdfe0ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdfdfe0ff),
                border: rgba(0xdfdfe0ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd9d9daff),
                border: rgba(0xdfdfe0ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xcacacaff),
                border: rgba(0xb9b9b9ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xebebecff),
                border: rgba(0xe5e5e6ff),
                foreground: rgba(0xa7a7a8ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x383a41ff),
                border: rgba(0xfafafaff),
                foreground: rgba(0xcececfff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdcdcddff),
                border: rgba(0xc9c9caff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: Some(
                    rgba(0x7f8188ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc9c9caff),
                border: rgba(0xc9c9caff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: Some(
                    rgba(0x7f8188ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xbebebfff),
                border: rgba(0xc9c9caff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: Some(
                    rgba(0x7f8188ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xafafafff),
                border: rgba(0xa6a6a7ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: Some(
                    rgba(0x383a41ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdcdcddff),
                border: rgba(0xd3d3d4ff),
                foreground: rgba(0xa1a1a3ff),
                secondary_foreground: Some(
                    rgba(0xa1a1a3ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x383a41ff),
                border: rgba(0xfafafaff),
                foreground: rgba(0xb4b4b4ff),
                secondary_foreground: Some(
                    rgba(0xb4b4b4ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfafafaff),
                border: rgba(0xeeeeeeff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xeeeeeeff),
                border: rgba(0xeeeeeeff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe8e8e9ff),
                border: rgba(0xeeeeeeff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xe0e0e1ff),
                border: rgba(0xd3d3d4ff),
                foreground: rgba(0x383a41ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfafafaff),
                border: rgba(0xf4f4f4ff),
                foreground: rgba(0xbebebfff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x383a41ff),
                border: rgba(0xfafafaff),
                foreground: rgba(0xe2e2e3ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe2e2faff),
                border: rgba(0xcbcdf6ff),
                foreground: rgba(0x5c79e2ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcbcdf6ff),
                border: rgba(0xcbcdf6ff),
                foreground: rgba(0x5c79e2ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xbec2f4ff),
                border: rgba(0xcbcdf6ff),
                foreground: rgba(0x5c79e2ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xadb2f1ff),
                border: rgba(0x98a2edff),
                foreground: rgba(0x07071fff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe2e2faff),
                border: rgba(0xd6d7f8ff),
                foreground: rgba(0x7c8de8ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x07071fff),
                border: rgba(0xffffffff),
                foreground: rgba(0xb2b7f1ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe0ebdcff),
                border: rgba(0xc8dcc1ff),
                foreground: rgba(0x669f59ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc8dcc1ff),
                border: rgba(0xc8dcc1ff),
                foreground: rgba(0x669f59ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xbcd4b3ff),
                border: rgba(0xc8dcc1ff),
                foreground: rgba(0x669f59ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xaac9a0ff),
                border: rgba(0x97be8bff),
                foreground: rgba(0x070a06ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe0ebdcff),
                border: rgba(0xd3e4ceff),
                foreground: rgba(0x7eae71ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x070a06ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xafcda6ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfaf2e6ff),
                border: rgba(0xf5e8d2ff),
                foreground: rgba(0xdec184ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf5e8d2ff),
                border: rgba(0xf5e8d2ff),
                foreground: rgba(0xdec184ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xf3e3c8ff),
                border: rgba(0xf5e8d2ff),
                foreground: rgba(0xdec184ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xf0dcbaff),
                border: rgba(0xebd4abff),
                foreground: rgba(0x261b08ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfaf2e6ff),
                border: rgba(0xf8eddbff),
                foreground: rgba(0xe5ca97ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x261b08ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xf0debfff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfbdfd9ff),
                border: rgba(0xf6c6bdff),
                foreground: rgba(0xd36151ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf6c6bdff),
                border: rgba(0xf6c6bdff),
                foreground: rgba(0xd36151ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xf3b9aeff),
                border: rgba(0xf6c6bdff),
                foreground: rgba(0xd36151ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xefa89bff),
                border: rgba(0xe79384ff),
                foreground: rgba(0x210705ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfbdfd9ff),
                border: rgba(0xf9d2cbff),
                foreground: rgba(0xde7a6aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x210705ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xefada0ff),
                secondary_foreground: None,
            },
        },
    }
}
