use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn one_dark() -> FabricTheme {
    FabricTheme {
        name: "One Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2f343eff),
                border: rgba(0x363c46ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: Some(
                    rgba(0x838994ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x363c46ff),
                border: rgba(0x363c46ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: Some(
                    rgba(0x838994ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3f4552ff),
                border: rgba(0x363c46ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: Some(
                    rgba(0x838994ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x454a56ff),
                border: rgba(0x4c515cff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: Some(
                    rgba(0xc8ccd4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2f343eff),
                border: rgba(0x323841ff),
                foreground: rgba(0x545862ff),
                secondary_foreground: Some(
                    rgba(0x545862ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xc8ccd4ff),
                border: rgba(0x282c34ff),
                foreground: rgba(0x434955ff),
                secondary_foreground: Some(
                    rgba(0x434955ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2f343eff),
                border: rgba(0x363c46ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x363c46ff),
                border: rgba(0x363c46ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3f4552ff),
                border: rgba(0x363c46ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x454a56ff),
                border: rgba(0x4c515cff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2f343eff),
                border: rgba(0x323841ff),
                foreground: rgba(0x545862ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xc8ccd4ff),
                border: rgba(0x282c34ff),
                foreground: rgba(0x434955ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3b414dff),
                border: rgba(0x464b57ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: Some(
                    rgba(0x838994ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x464b57ff),
                border: rgba(0x464b57ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: Some(
                    rgba(0x838994ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4a4f5aff),
                border: rgba(0x464b57ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: Some(
                    rgba(0x838994ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x4f545eff),
                border: rgba(0x545962ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: Some(
                    rgba(0xc8ccd4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3b414dff),
                border: rgba(0x414754ff),
                foreground: rgba(0x555a63ff),
                secondary_foreground: Some(
                    rgba(0x555a63ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xc8ccd4ff),
                border: rgba(0x282c34ff),
                foreground: rgba(0x4e535dff),
                secondary_foreground: Some(
                    rgba(0x4e535dff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x282c34ff),
                border: rgba(0x2e333cff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2e333cff),
                border: rgba(0x2e333cff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x313640ff),
                border: rgba(0x2e333cff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x353b45ff),
                border: rgba(0x414754ff),
                foreground: rgba(0xc8ccd4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x282c34ff),
                border: rgba(0x2b2f38ff),
                foreground: rgba(0x4a4f5aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xc8ccd4ff),
                border: rgba(0x282c34ff),
                foreground: rgba(0x343a43ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x18243dff),
                border: rgba(0x293c5bff),
                foreground: rgba(0x74ade8ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x293c5bff),
                border: rgba(0x293c5bff),
                foreground: rgba(0x74ade8ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x32496cff),
                border: rgba(0x293c5bff),
                foreground: rgba(0x74ade8ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x3e5c83ff),
                border: rgba(0x4d729fff),
                foreground: rgba(0xfafcfeff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x18243dff),
                border: rgba(0x20304bff),
                foreground: rgba(0x608fc3ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfafcfeff),
                border: rgba(0x000019ff),
                foreground: rgba(0x3a577dff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x222e1dff),
                border: rgba(0x38482fff),
                foreground: rgba(0xa1c181ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x38482fff),
                border: rgba(0x38482fff),
                foreground: rgba(0xa1c181ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x445639ff),
                border: rgba(0x38482fff),
                foreground: rgba(0xa1c181ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x566a46ff),
                border: rgba(0x6a8256ff),
                foreground: rgba(0xfbfcfaff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x222e1dff),
                border: rgba(0x2d3b26ff),
                foreground: rgba(0x85a16bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbfcfaff),
                border: rgba(0x000f00ff),
                foreground: rgba(0x506542ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x41331dff),
                border: rgba(0x5d4c2fff),
                foreground: rgba(0xdec184ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x5d4c2fff),
                border: rgba(0x5d4c2fff),
                foreground: rgba(0xdec184ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x6c5939ff),
                border: rgba(0x5d4c2fff),
                foreground: rgba(0xdec184ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x826d47ff),
                border: rgba(0x9c8458ff),
                foreground: rgba(0xfefcfaff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x41331dff),
                border: rgba(0x4e3f26ff),
                foreground: rgba(0xbda26eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfefcfaff),
                border: rgba(0x211500ff),
                foreground: rgba(0x7c6743ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x301b1cff),
                border: rgba(0x4c2b2cff),
                foreground: rgba(0xd07277ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x4c2b2cff),
                border: rgba(0x4c2b2cff),
                foreground: rgba(0xd07277ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x5b3335ff),
                border: rgba(0x4c2b2cff),
                foreground: rgba(0xd07277ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x713f41ff),
                border: rgba(0x8b4d50ff),
                foreground: rgba(0xfef9f9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x301b1cff),
                border: rgba(0x3d2324ff),
                foreground: rgba(0xad5f63ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfef9f9ff),
                border: rgba(0x0d0000ff),
                foreground: rgba(0x6b3b3dff),
                secondary_foreground: None,
            },
        },
    }
}
