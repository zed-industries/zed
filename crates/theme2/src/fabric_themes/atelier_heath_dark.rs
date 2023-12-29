use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_heath_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Heath Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x252025ff),
                border: rgba(0x393239ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: Some(
                    rgba(0xa99aa9ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x393239ff),
                border: rgba(0x393239ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: Some(
                    rgba(0xa99aa9ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4c424cff),
                border: rgba(0x393239ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: Some(
                    rgba(0xa99aa9ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x655965ff),
                border: rgba(0x6f626fff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: Some(
                    rgba(0xf7f3f7ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x252025ff),
                border: rgba(0x292329ff),
                foreground: rgba(0x776977ff),
                secondary_foreground: Some(
                    rgba(0x776977ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7f3f7ff),
                border: rgba(0x1b181bff),
                foreground: rgba(0x5e535eff),
                secondary_foreground: Some(
                    rgba(0x5e535eff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x252025ff),
                border: rgba(0x393239ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x393239ff),
                border: rgba(0x393239ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4c424cff),
                border: rgba(0x393239ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x655965ff),
                border: rgba(0x6f626fff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x252025ff),
                border: rgba(0x292329ff),
                foreground: rgba(0x776977ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7f3f7ff),
                border: rgba(0x1b181bff),
                foreground: rgba(0x5e535eff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x433a43ff),
                border: rgba(0x675b67ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: Some(
                    rgba(0xa99aa9ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x675b67ff),
                border: rgba(0x675b67ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: Some(
                    rgba(0xa99aa9ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x6d606dff),
                border: rgba(0x675b67ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: Some(
                    rgba(0xa99aa9ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x726472ff),
                border: rgba(0x7a6c7aff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: Some(
                    rgba(0xf7f3f7ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x433a43ff),
                border: rgba(0x554a55ff),
                foreground: rgba(0x908190ff),
                secondary_foreground: Some(
                    rgba(0x908190ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7f3f7ff),
                border: rgba(0x1b181bff),
                foreground: rgba(0x716471ff),
                secondary_foreground: Some(
                    rgba(0x716471ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1b181bff),
                border: rgba(0x231e23ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x231e23ff),
                border: rgba(0x231e23ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x272127ff),
                border: rgba(0x231e23ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x383138ff),
                border: rgba(0x554a55ff),
                foreground: rgba(0xf7f3f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1b181bff),
                border: rgba(0x1f1b1fff),
                foreground: rgba(0x6d606dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7f3f7ff),
                border: rgba(0x1b181bff),
                foreground: rgba(0x302a30ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x0e1a43ff),
                border: rgba(0x1a2961ff),
                foreground: rgba(0x526aebff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1a2961ff),
                border: rgba(0x1a2961ff),
                foreground: rgba(0x526aebff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x213071ff),
                border: rgba(0x1a2961ff),
                foreground: rgba(0x526aebff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2a3b88ff),
                border: rgba(0x3448a4ff),
                foreground: rgba(0xf9f9feff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x0e1a43ff),
                border: rgba(0x142151ff),
                foreground: rgba(0x4259c7ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf9f9feff),
                border: rgba(0x000020ff),
                foreground: rgba(0x273782ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x211f12ff),
                border: rgba(0x34321bff),
                foreground: rgba(0x918b3bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x34321bff),
                border: rgba(0x34321bff),
                foreground: rgba(0x918b3bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3f3c1fff),
                border: rgba(0x34321bff),
                foreground: rgba(0x918b3bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x4f4b24ff),
                border: rgba(0x615d2bff),
                foreground: rgba(0xfbfaf7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x211f12ff),
                border: rgba(0x2b2817ff),
                foreground: rgba(0x797433ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbfaf7ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x4a4623ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2d1e12ff),
                border: rgba(0x463219ff),
                foreground: rgba(0xbb8a36ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x463219ff),
                border: rgba(0x463219ff),
                foreground: rgba(0xbb8a36ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x543c1dff),
                border: rgba(0x463219ff),
                foreground: rgba(0xbb8a36ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x674b21ff),
                border: rgba(0x7f5c27ff),
                foreground: rgba(0xfdfaf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2d1e12ff),
                border: rgba(0x3a2816ff),
                foreground: rgba(0x9c732eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdfaf6ff),
                border: rgba(0x0e0000ff),
                foreground: rgba(0x624620ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3c110eff),
                border: rgba(0x551a15ff),
                foreground: rgba(0xca402cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x551a15ff),
                border: rgba(0x551a15ff),
                foreground: rgba(0xca402cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x631e18ff),
                border: rgba(0x551a15ff),
                foreground: rgba(0xca402cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x77241bff),
                border: rgba(0x8e2c20ff),
                foreground: rgba(0xfff7f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3c110eff),
                border: rgba(0x491512ff),
                foreground: rgba(0xab3626ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff7f6ff),
                border: rgba(0x210000ff),
                foreground: rgba(0x71231bff),
                secondary_foreground: None,
            },
        },
    }
}
