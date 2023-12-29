use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn ayu_light() -> FabricTheme {
    FabricTheme {
        name: "Ayu Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xececedff),
                border: rgba(0xdfe0e1ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: Some(
                    rgba(0x8c8f93ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdfe0e1ff),
                border: rgba(0xdfe0e1ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: Some(
                    rgba(0x8c8f93ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd8dadbff),
                border: rgba(0xdfe0e1ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: Some(
                    rgba(0x8c8f93ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xd0d1d3ff),
                border: rgba(0xc6c7c9ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: Some(
                    rgba(0x5c6166ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xececedff),
                border: rgba(0xe5e6e7ff),
                foreground: rgba(0xb9bbbdff),
                secondary_foreground: Some(
                    rgba(0xb9bbbdff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x5c6166ff),
                border: rgba(0xfcfcfcff),
                foreground: rgba(0xd3d4d5ff),
                secondary_foreground: Some(
                    rgba(0xd3d4d5ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xececedff),
                border: rgba(0xdfe0e1ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdfe0e1ff),
                border: rgba(0xdfe0e1ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd8dadbff),
                border: rgba(0xdfe0e1ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd0d1d3ff),
                border: rgba(0xc6c7c9ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xececedff),
                border: rgba(0xe5e6e7ff),
                foreground: rgba(0xb9bbbdff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x5c6166ff),
                border: rgba(0xfcfcfcff),
                foreground: rgba(0xd3d4d5ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdcdddeff),
                border: rgba(0xcfd1d2ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: Some(
                    rgba(0x8c8f93ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xcfd1d2ff),
                border: rgba(0xcfd1d2ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: Some(
                    rgba(0x8c8f93ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc9caccff),
                border: rgba(0xcfd1d2ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: Some(
                    rgba(0x8c8f93ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xc0c2c4ff),
                border: rgba(0xb6b8baff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: Some(
                    rgba(0x5c6166ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdcdddeff),
                border: rgba(0xd5d6d8ff),
                foreground: rgba(0xa9acaeff),
                secondary_foreground: Some(
                    rgba(0xa9acaeff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x5c6166ff),
                border: rgba(0xfcfcfcff),
                foreground: rgba(0xc2c4c6ff),
                secondary_foreground: Some(
                    rgba(0xc2c4c6ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfcfcfcff),
                border: rgba(0xefeff0ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xefeff0ff),
                border: rgba(0xefeff0ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe9e9eaff),
                border: rgba(0xefeff0ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xe0e1e2ff),
                border: rgba(0xd5d6d8ff),
                foreground: rgba(0x5c6166ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfcfcfcff),
                border: rgba(0xf6f6f6ff),
                foreground: rgba(0xc9caccff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x5c6166ff),
                border: rgba(0xfcfcfcff),
                foreground: rgba(0xe2e3e4ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdeebfaff),
                border: rgba(0xc4daf6ff),
                foreground: rgba(0x3b9ee5ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc4daf6ff),
                border: rgba(0xc4daf6ff),
                foreground: rgba(0x3b9ee5ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb7d3f4ff),
                border: rgba(0xc4daf6ff),
                foreground: rgba(0x3b9ee5ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xa3c8f2ff),
                border: rgba(0x8abceeff),
                foreground: rgba(0x060a1eff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdeebfaff),
                border: rgba(0xd1e2f8ff),
                foreground: rgba(0x68adeaff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x060a1eff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa8ccf2ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe9efd2ff),
                border: rgba(0xd7e3aeff),
                foreground: rgba(0x86b305ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd7e3aeff),
                border: rgba(0xd7e3aeff),
                foreground: rgba(0x86b305ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xcedd9dff),
                border: rgba(0xd7e3aeff),
                foreground: rgba(0x86b305ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xc0d584ff),
                border: rgba(0xb1cb67ff),
                foreground: rgba(0x090b03ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe9efd2ff),
                border: rgba(0xe0e9c0ff),
                foreground: rgba(0x9cbf40ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x090b03ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xc5d78bff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xffeedaff),
                border: rgba(0xffe1beff),
                foreground: rgba(0xf1ae4aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xffe1beff),
                border: rgba(0xffe1beff),
                foreground: rgba(0xf1ae4aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xffdaafff),
                border: rgba(0xffe1beff),
                foreground: rgba(0xf1ae4aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xfed19bff),
                border: rgba(0xfcc784ff),
                foreground: rgba(0x340806ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xffeedaff),
                border: rgba(0xffe7ccff),
                foreground: rgba(0xf7ba67ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x340806ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xffd4a1ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xffe3e1ff),
                border: rgba(0xffcdcaff),
                foreground: rgba(0xef7271ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xffcdcaff),
                border: rgba(0xffcdcaff),
                foreground: rgba(0xef7271ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xffc2beff),
                border: rgba(0xffcdcaff),
                foreground: rgba(0xef7271ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xfeb2aeff),
                border: rgba(0xfaa09cff),
                foreground: rgba(0x2d0607ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xffe3e1ff),
                border: rgba(0xffd8d5ff),
                foreground: rgba(0xf68986ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x2d0607ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xfeb7b3ff),
                secondary_foreground: None,
            },
        },
    }
}
