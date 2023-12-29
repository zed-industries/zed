use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn solarized_dark() -> FabricTheme {
    FabricTheme {
        name: "Solarized Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x04313cff),
                border: rgba(0x063541ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: Some(
                    rgba(0x93a1a1ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x063541ff),
                border: rgba(0x063541ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: Some(
                    rgba(0x93a1a1ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x103c47ff),
                border: rgba(0x063541ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: Some(
                    rgba(0x93a1a1ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x294e58ff),
                border: rgba(0x46626aff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: Some(
                    rgba(0xfdf6e3ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x04313cff),
                border: rgba(0x05333eff),
                foreground: rgba(0x5f757dff),
                secondary_foreground: Some(
                    rgba(0x5f757dff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf6e3ff),
                border: rgba(0x002b36ff),
                foreground: rgba(0x224853ff),
                secondary_foreground: Some(
                    rgba(0x224853ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x04313cff),
                border: rgba(0x063541ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x063541ff),
                border: rgba(0x063541ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x103c47ff),
                border: rgba(0x063541ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x294e58ff),
                border: rgba(0x46626aff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x04313cff),
                border: rgba(0x05333eff),
                foreground: rgba(0x5f757dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf6e3ff),
                border: rgba(0x002b36ff),
                foreground: rgba(0x224853ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x083743ff),
                border: rgba(0x2b4f58ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: Some(
                    rgba(0x93a1a1ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2b4f58ff),
                border: rgba(0x2b4f58ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: Some(
                    rgba(0x93a1a1ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3d5b64ff),
                border: rgba(0x2b4f58ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: Some(
                    rgba(0x93a1a1ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x566d74ff),
                border: rgba(0x637981ff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: Some(
                    rgba(0xfdf6e3ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x083743ff),
                border: rgba(0x19424dff),
                foreground: rgba(0x6f8389ff),
                secondary_foreground: Some(
                    rgba(0x6f8389ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf6e3ff),
                border: rgba(0x002b36ff),
                foreground: rgba(0x4f686fff),
                secondary_foreground: Some(
                    rgba(0x4f686fff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x002b36ff),
                border: rgba(0x032f3bff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x032f3bff),
                border: rgba(0x032f3bff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x04323dff),
                border: rgba(0x032f3bff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x063440ff),
                border: rgba(0x19424dff),
                foreground: rgba(0xfdf6e3ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x002b36ff),
                border: rgba(0x012d38ff),
                foreground: rgba(0x3d5b64ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf6e3ff),
                border: rgba(0x002b36ff),
                foreground: rgba(0x063440ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x141f2cff),
                border: rgba(0x1c3249ff),
                foreground: rgba(0x288bd1ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1c3249ff),
                border: rgba(0x1c3249ff),
                foreground: rgba(0x288bd1ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1f3c59ff),
                border: rgba(0x1c3249ff),
                foreground: rgba(0x288bd1ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x224b6fff),
                border: rgba(0x255d8bff),
                foreground: rgba(0xf8fafdff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x141f2cff),
                border: rgba(0x18283aff),
                foreground: rgba(0x2774adff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8fafdff),
                border: rgba(0x000004ff),
                foreground: rgba(0x214669ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1f210cff),
                border: rgba(0x323610ff),
                foreground: rgba(0x859904ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x323610ff),
                border: rgba(0x323610ff),
                foreground: rgba(0x859904ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3b4111ff),
                border: rgba(0x323610ff),
                foreground: rgba(0x859904ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x495211ff),
                border: rgba(0x5a6610ff),
                foreground: rgba(0xfbfbf5ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1f210cff),
                border: rgba(0x292c0fff),
                foreground: rgba(0x6f7f0bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbfbf5ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x454d11ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2f1e0cff),
                border: rgba(0x473110ff),
                foreground: rgba(0xb58903ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x473110ff),
                border: rgba(0x473110ff),
                foreground: rgba(0xb58903ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x533b10ff),
                border: rgba(0x473110ff),
                foreground: rgba(0xb58903ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x664a10ff),
                border: rgba(0x7c5b0fff),
                foreground: rgba(0xfdfaf5ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2f1e0cff),
                border: rgba(0x3b280eff),
                foreground: rgba(0x98720aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdfaf5ff),
                border: rgba(0x130000ff),
                foreground: rgba(0x614510ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x4a090fff),
                border: rgba(0x641116ff),
                foreground: rgba(0xdc3330ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x641116ff),
                border: rgba(0x641116ff),
                foreground: rgba(0xdc3330ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x721519ff),
                border: rgba(0x641116ff),
                foreground: rgba(0xdc3330ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x861a1dff),
                border: rgba(0x9e2123ff),
                foreground: rgba(0xfff7f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x4a090fff),
                border: rgba(0x570d13ff),
                foreground: rgba(0xbc2a29ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff7f6ff),
                border: rgba(0x2c0000ff),
                foreground: rgba(0x81191cff),
                secondary_foreground: None,
            },
        },
    }
}
