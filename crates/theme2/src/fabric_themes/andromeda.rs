use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn andromeda() -> FabricTheme {
    FabricTheme {
        name: "Andromeda",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x21242bff),
                border: rgba(0x252931ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: Some(
                    rgba(0xaca8aeff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x252931ff),
                border: rgba(0x252931ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: Some(
                    rgba(0xaca8aeff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x272c35ff),
                border: rgba(0x252931ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: Some(
                    rgba(0xaca8aeff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x2a2f39ff),
                border: rgba(0x2e323cff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: Some(
                    rgba(0xf7f7f8ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x21242bff),
                border: rgba(0x23262dff),
                foreground: rgba(0x474a53ff),
                secondary_foreground: Some(
                    rgba(0x474a53ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7f7f8ff),
                border: rgba(0x1e2025ff),
                foreground: rgba(0x2a2f39ff),
                secondary_foreground: Some(
                    rgba(0x2a2f39ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x21242bff),
                border: rgba(0x252931ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x252931ff),
                border: rgba(0x252931ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x272c35ff),
                border: rgba(0x252931ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2a2f39ff),
                border: rgba(0x2e323cff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x21242bff),
                border: rgba(0x23262dff),
                foreground: rgba(0x474a53ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7f7f8ff),
                border: rgba(0x1e2025ff),
                foreground: rgba(0x2a2f39ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x262a33ff),
                border: rgba(0x2b2f39ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: Some(
                    rgba(0xaca8aeff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2b2f39ff),
                border: rgba(0x2b2f39ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: Some(
                    rgba(0xaca8aeff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2d313bff),
                border: rgba(0x2b2f39ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: Some(
                    rgba(0xaca8aeff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x383b45ff),
                border: rgba(0x4e5059ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: Some(
                    rgba(0xf7f7f8ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x262a33ff),
                border: rgba(0x292d37ff),
                foreground: rgba(0x6b6b73ff),
                secondary_foreground: Some(
                    rgba(0x6b6b73ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7f7f8ff),
                border: rgba(0x1e2025ff),
                foreground: rgba(0x32363fff),
                secondary_foreground: Some(
                    rgba(0x32363fff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1e2025ff),
                border: rgba(0x21232aff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x21232aff),
                border: rgba(0x21232aff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x22252cff),
                border: rgba(0x21232aff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x242830ff),
                border: rgba(0x292d37ff),
                foreground: rgba(0xf7f7f8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1e2025ff),
                border: rgba(0x1f2227ff),
                foreground: rgba(0x2d313bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7f7f8ff),
                border: rgba(0x1e2025ff),
                foreground: rgba(0x24272fff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x122420ff),
                border: rgba(0x183a34ff),
                foreground: rgba(0x11a793ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x183a34ff),
                border: rgba(0x183a34ff),
                foreground: rgba(0x11a793ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1a473fff),
                border: rgba(0x183a34ff),
                foreground: rgba(0x11a793ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x1b594fff),
                border: rgba(0x1b6f62ff),
                foreground: rgba(0xf7fcfaff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x122420ff),
                border: rgba(0x152f2aff),
                foreground: rgba(0x178a7aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7fcfaff),
                border: rgba(0x000000ff),
                foreground: rgba(0x1b544aff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x194618ff),
                border: rgba(0x306129ff),
                foreground: rgba(0x96df72ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x306129ff),
                border: rgba(0x306129ff),
                foreground: rgba(0x96df72ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3c7031ff),
                border: rgba(0x306129ff),
                foreground: rgba(0x96df72ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x4d853eff),
                border: rgba(0x619f4cff),
                foreground: rgba(0xfbfef9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x194618ff),
                border: rgba(0x255321ff),
                foreground: rgba(0x7bbf5fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbfef9ff),
                border: rgba(0x002500ff),
                foreground: rgba(0x488039ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x5c5015ff),
                border: rgba(0x796b26ff),
                foreground: rgba(0xfee56dff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x796b26ff),
                border: rgba(0x796b26ff),
                foreground: rgba(0xfee56dff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x89792eff),
                border: rgba(0x796b26ff),
                foreground: rgba(0xfee56dff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xa08e3aff),
                border: rgba(0xbaa748ff),
                foreground: rgba(0xfffef9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x5c5015ff),
                border: rgba(0x6a5d1eff),
                foreground: rgba(0xdcc55aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfffef9ff),
                border: rgba(0x382f00ff),
                foreground: rgba(0x998836ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x55051bff),
                border: rgba(0x720a2bff),
                foreground: rgba(0xf82872ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x720a2bff),
                border: rgba(0x720a2bff),
                foreground: rgba(0xf82872ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x820d33ff),
                border: rgba(0x720a2bff),
                foreground: rgba(0xf82872ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x99123fff),
                border: rgba(0xb4184dff),
                foreground: rgba(0xfff8f9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x55051bff),
                border: rgba(0x630723ff),
                foreground: rgba(0xd61f5fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff8f9ff),
                border: rgba(0x320000ff),
                foreground: rgba(0x92113bff),
                secondary_foreground: None,
            },
        },
    }
}
