use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_heath_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Heath Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x252025),
                border: rgb(0x393239),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: Some(
                    rgb(0xa99aa9),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x393239),
                border: rgb(0x393239),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: Some(
                    rgb(0xa99aa9),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4c424c),
                border: rgb(0x393239),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: Some(
                    rgb(0xa99aa9),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x655965),
                border: rgb(0x6f626f),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: Some(
                    rgb(0xf7f3f7),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x252025),
                border: rgb(0x292329),
                foreground: rgb(0x776977),
                secondary_foreground: Some(
                    rgb(0x776977),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7f3f7),
                border: rgb(0x1b181b),
                foreground: rgb(0x5e535e),
                secondary_foreground: Some(
                    rgb(0x5e535e),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x252025),
                border: rgb(0x393239),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x393239),
                border: rgb(0x393239),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4c424c),
                border: rgb(0x393239),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x655965),
                border: rgb(0x6f626f),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x252025),
                border: rgb(0x292329),
                foreground: rgb(0x776977),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7f3f7),
                border: rgb(0x1b181b),
                foreground: rgb(0x5e535e),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x433a43),
                border: rgb(0x675b67),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: Some(
                    rgb(0xa99aa9),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x675b67),
                border: rgb(0x675b67),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: Some(
                    rgb(0xa99aa9),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x6d606d),
                border: rgb(0x675b67),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: Some(
                    rgb(0xa99aa9),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x726472),
                border: rgb(0x7a6c7a),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: Some(
                    rgb(0xf7f3f7),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x433a43),
                border: rgb(0x554a55),
                foreground: rgb(0x908190),
                secondary_foreground: Some(
                    rgb(0x908190),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7f3f7),
                border: rgb(0x1b181b),
                foreground: rgb(0x716471),
                secondary_foreground: Some(
                    rgb(0x716471),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1b181b),
                border: rgb(0x231e23),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x231e23),
                border: rgb(0x231e23),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x272127),
                border: rgb(0x231e23),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x383138),
                border: rgb(0x554a55),
                foreground: rgb(0xf7f3f7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1b181b),
                border: rgb(0x1f1b1f),
                foreground: rgb(0x6d606d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7f3f7),
                border: rgb(0x1b181b),
                foreground: rgb(0x302a30),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe1a43),
                border: rgb(0x1a2961),
                foreground: rgb(0x526aeb),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1a2961),
                border: rgb(0x1a2961),
                foreground: rgb(0x526aeb),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x213071),
                border: rgb(0x1a2961),
                foreground: rgb(0x526aeb),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2a3b88),
                border: rgb(0x3448a4),
                foreground: rgb(0xf9f9fe),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe1a43),
                border: rgb(0x142151),
                foreground: rgb(0x4259c7),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf9f9fe),
                border: rgb(0x0020),
                foreground: rgb(0x273782),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x211f12),
                border: rgb(0x34321b),
                foreground: rgb(0x918b3b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x34321b),
                border: rgb(0x34321b),
                foreground: rgb(0x918b3b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3f3c1f),
                border: rgb(0x34321b),
                foreground: rgb(0x918b3b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x4f4b24),
                border: rgb(0x615d2b),
                foreground: rgb(0xfbfaf7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x211f12),
                border: rgb(0x2b2817),
                foreground: rgb(0x797433),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbfaf7),
                border: rgb(0x0000),
                foreground: rgb(0x4a4623),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2d1e12),
                border: rgb(0x463219),
                foreground: rgb(0xbb8a36),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x463219),
                border: rgb(0x463219),
                foreground: rgb(0xbb8a36),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x543c1d),
                border: rgb(0x463219),
                foreground: rgb(0xbb8a36),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x674b21),
                border: rgb(0x7f5c27),
                foreground: rgb(0xfdfaf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2d1e12),
                border: rgb(0x3a2816),
                foreground: rgb(0x9c732e),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdfaf6),
                border: rgb(0xe0000),
                foreground: rgb(0x624620),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3c110e),
                border: rgb(0x551a15),
                foreground: rgb(0xca402c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x551a15),
                border: rgb(0x551a15),
                foreground: rgb(0xca402c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x631e18),
                border: rgb(0x551a15),
                foreground: rgb(0xca402c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x77241b),
                border: rgb(0x8e2c20),
                foreground: rgb(0xfff7f6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3c110e),
                border: rgb(0x491512),
                foreground: rgb(0xab3626),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff7f6),
                border: rgb(0x210000),
                foreground: rgb(0x71231b),
                secondary_foreground: None,
            },
        },
    }
}
