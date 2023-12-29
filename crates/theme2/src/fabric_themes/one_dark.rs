use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn one_dark() -> FabricTheme {
    FabricTheme {
        name: "One Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2f343e),
                border: rgb(0x363c46),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: Some(
                    rgb(0x838994),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x363c46),
                border: rgb(0x363c46),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: Some(
                    rgb(0x838994),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3f4552),
                border: rgb(0x363c46),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: Some(
                    rgb(0x838994),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x454a56),
                border: rgb(0x4c515c),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: Some(
                    rgb(0xc8ccd4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2f343e),
                border: rgb(0x323841),
                foreground: rgb(0x545862),
                secondary_foreground: Some(
                    rgb(0x545862),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xc8ccd4),
                border: rgb(0x282c34),
                foreground: rgb(0x434955),
                secondary_foreground: Some(
                    rgb(0x434955),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2f343e),
                border: rgb(0x363c46),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x363c46),
                border: rgb(0x363c46),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3f4552),
                border: rgb(0x363c46),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x454a56),
                border: rgb(0x4c515c),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2f343e),
                border: rgb(0x323841),
                foreground: rgb(0x545862),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xc8ccd4),
                border: rgb(0x282c34),
                foreground: rgb(0x434955),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3b414d),
                border: rgb(0x464b57),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: Some(
                    rgb(0x838994),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x464b57),
                border: rgb(0x464b57),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: Some(
                    rgb(0x838994),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4a4f5a),
                border: rgb(0x464b57),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: Some(
                    rgb(0x838994),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x4f545e),
                border: rgb(0x545962),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: Some(
                    rgb(0xc8ccd4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3b414d),
                border: rgb(0x414754),
                foreground: rgb(0x555a63),
                secondary_foreground: Some(
                    rgb(0x555a63),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xc8ccd4),
                border: rgb(0x282c34),
                foreground: rgb(0x4e535d),
                secondary_foreground: Some(
                    rgb(0x4e535d),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x282c34),
                border: rgb(0x2e333c),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2e333c),
                border: rgb(0x2e333c),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x313640),
                border: rgb(0x2e333c),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x353b45),
                border: rgb(0x414754),
                foreground: rgb(0xc8ccd4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x282c34),
                border: rgb(0x2b2f38),
                foreground: rgb(0x4a4f5a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xc8ccd4),
                border: rgb(0x282c34),
                foreground: rgb(0x343a43),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x18243d),
                border: rgb(0x293c5b),
                foreground: rgb(0x74ade8),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x293c5b),
                border: rgb(0x293c5b),
                foreground: rgb(0x74ade8),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x32496c),
                border: rgb(0x293c5b),
                foreground: rgb(0x74ade8),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x3e5c83),
                border: rgb(0x4d729f),
                foreground: rgb(0xfafcfe),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x18243d),
                border: rgb(0x20304b),
                foreground: rgb(0x608fc3),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfafcfe),
                border: rgb(0x0019),
                foreground: rgb(0x3a577d),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x222e1d),
                border: rgb(0x38482f),
                foreground: rgb(0xa1c181),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x38482f),
                border: rgb(0x38482f),
                foreground: rgb(0xa1c181),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x445639),
                border: rgb(0x38482f),
                foreground: rgb(0xa1c181),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x566a46),
                border: rgb(0x6a8256),
                foreground: rgb(0xfbfcfa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x222e1d),
                border: rgb(0x2d3b26),
                foreground: rgb(0x85a16b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbfcfa),
                border: rgb(0x0f00),
                foreground: rgb(0x506542),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x41331d),
                border: rgb(0x5d4c2f),
                foreground: rgb(0xdec184),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x5d4c2f),
                border: rgb(0x5d4c2f),
                foreground: rgb(0xdec184),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x6c5939),
                border: rgb(0x5d4c2f),
                foreground: rgb(0xdec184),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x826d47),
                border: rgb(0x9c8458),
                foreground: rgb(0xfefcfa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x41331d),
                border: rgb(0x4e3f26),
                foreground: rgb(0xbda26e),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfefcfa),
                border: rgb(0x211500),
                foreground: rgb(0x7c6743),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x301b1c),
                border: rgb(0x4c2b2c),
                foreground: rgb(0xd07277),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x4c2b2c),
                border: rgb(0x4c2b2c),
                foreground: rgb(0xd07277),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x5b3335),
                border: rgb(0x4c2b2c),
                foreground: rgb(0xd07277),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x713f41),
                border: rgb(0x8b4d50),
                foreground: rgb(0xfef9f9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x301b1c),
                border: rgb(0x3d2324),
                foreground: rgb(0xad5f63),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfef9f9),
                border: rgb(0xd0000),
                foreground: rgb(0x6b3b3d),
                secondary_foreground: None,
            },
        },
    }
}
