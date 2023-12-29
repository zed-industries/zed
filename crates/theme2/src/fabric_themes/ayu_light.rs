use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn ayu_light() -> FabricTheme {
    FabricTheme {
        name: "Ayu Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xececed),
                border: rgb(0xdfe0e1),
                foreground: rgb(0x5c6166),
                secondary_foreground: Some(
                    rgb(0x8c8f93),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdfe0e1),
                border: rgb(0xdfe0e1),
                foreground: rgb(0x5c6166),
                secondary_foreground: Some(
                    rgb(0x8c8f93),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd8dadb),
                border: rgb(0xdfe0e1),
                foreground: rgb(0x5c6166),
                secondary_foreground: Some(
                    rgb(0x8c8f93),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xd0d1d3),
                border: rgb(0xc6c7c9),
                foreground: rgb(0x5c6166),
                secondary_foreground: Some(
                    rgb(0x5c6166),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xececed),
                border: rgb(0xe5e6e7),
                foreground: rgb(0xb9bbbd),
                secondary_foreground: Some(
                    rgb(0xb9bbbd),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x5c6166),
                border: rgb(0xfcfcfc),
                foreground: rgb(0xd3d4d5),
                secondary_foreground: Some(
                    rgb(0xd3d4d5),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xececed),
                border: rgb(0xdfe0e1),
                foreground: rgb(0x5c6166),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdfe0e1),
                border: rgb(0xdfe0e1),
                foreground: rgb(0x5c6166),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd8dadb),
                border: rgb(0xdfe0e1),
                foreground: rgb(0x5c6166),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd0d1d3),
                border: rgb(0xc6c7c9),
                foreground: rgb(0x5c6166),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xececed),
                border: rgb(0xe5e6e7),
                foreground: rgb(0xb9bbbd),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x5c6166),
                border: rgb(0xfcfcfc),
                foreground: rgb(0xd3d4d5),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdcddde),
                border: rgb(0xcfd1d2),
                foreground: rgb(0x5c6166),
                secondary_foreground: Some(
                    rgb(0x8c8f93),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcfd1d2),
                border: rgb(0xcfd1d2),
                foreground: rgb(0x5c6166),
                secondary_foreground: Some(
                    rgb(0x8c8f93),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc9cacc),
                border: rgb(0xcfd1d2),
                foreground: rgb(0x5c6166),
                secondary_foreground: Some(
                    rgb(0x8c8f93),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xc0c2c4),
                border: rgb(0xb6b8ba),
                foreground: rgb(0x5c6166),
                secondary_foreground: Some(
                    rgb(0x5c6166),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdcddde),
                border: rgb(0xd5d6d8),
                foreground: rgb(0xa9acae),
                secondary_foreground: Some(
                    rgb(0xa9acae),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x5c6166),
                border: rgb(0xfcfcfc),
                foreground: rgb(0xc2c4c6),
                secondary_foreground: Some(
                    rgb(0xc2c4c6),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfcfcfc),
                border: rgb(0xefeff0),
                foreground: rgb(0x5c6166),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xefeff0),
                border: rgb(0xefeff0),
                foreground: rgb(0x5c6166),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe9e9ea),
                border: rgb(0xefeff0),
                foreground: rgb(0x5c6166),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xe0e1e2),
                border: rgb(0xd5d6d8),
                foreground: rgb(0x5c6166),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfcfcfc),
                border: rgb(0xf6f6f6),
                foreground: rgb(0xc9cacc),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x5c6166),
                border: rgb(0xfcfcfc),
                foreground: rgb(0xe2e3e4),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdeebfa),
                border: rgb(0xc4daf6),
                foreground: rgb(0x3b9ee5),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc4daf6),
                border: rgb(0xc4daf6),
                foreground: rgb(0x3b9ee5),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb7d3f4),
                border: rgb(0xc4daf6),
                foreground: rgb(0x3b9ee5),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xa3c8f2),
                border: rgb(0x8abcee),
                foreground: rgb(0x60a1e),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdeebfa),
                border: rgb(0xd1e2f8),
                foreground: rgb(0x68adea),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x60a1e),
                border: rgb(0xffffff),
                foreground: rgb(0xa8ccf2),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe9efd2),
                border: rgb(0xd7e3ae),
                foreground: rgb(0x86b305),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd7e3ae),
                border: rgb(0xd7e3ae),
                foreground: rgb(0x86b305),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xcedd9d),
                border: rgb(0xd7e3ae),
                foreground: rgb(0x86b305),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xc0d584),
                border: rgb(0xb1cb67),
                foreground: rgb(0x90b03),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe9efd2),
                border: rgb(0xe0e9c0),
                foreground: rgb(0x9cbf40),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x90b03),
                border: rgb(0xffffff),
                foreground: rgb(0xc5d78b),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xffeeda),
                border: rgb(0xffe1be),
                foreground: rgb(0xf1ae4a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xffe1be),
                border: rgb(0xffe1be),
                foreground: rgb(0xf1ae4a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xffdaaf),
                border: rgb(0xffe1be),
                foreground: rgb(0xf1ae4a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xfed19b),
                border: rgb(0xfcc784),
                foreground: rgb(0x340806),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xffeeda),
                border: rgb(0xffe7cc),
                foreground: rgb(0xf7ba67),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x340806),
                border: rgb(0xffffff),
                foreground: rgb(0xffd4a1),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xffe3e1),
                border: rgb(0xffcdca),
                foreground: rgb(0xef7271),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xffcdca),
                border: rgb(0xffcdca),
                foreground: rgb(0xef7271),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xffc2be),
                border: rgb(0xffcdca),
                foreground: rgb(0xef7271),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xfeb2ae),
                border: rgb(0xfaa09c),
                foreground: rgb(0x2d0607),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xffe3e1),
                border: rgb(0xffd8d5),
                foreground: rgb(0xf68986),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x2d0607),
                border: rgb(0xffffff),
                foreground: rgb(0xfeb7b3),
                secondary_foreground: None,
            },
        },
    }
}
