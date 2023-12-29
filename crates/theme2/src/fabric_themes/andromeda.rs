use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn andromeda() -> FabricTheme {
    FabricTheme {
        name: "Andromeda".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x21242b),
                border: rgb(0x252931),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: Some(
                    rgb(0xaca8ae),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x252931),
                border: rgb(0x252931),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: Some(
                    rgb(0xaca8ae),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x272c35),
                border: rgb(0x252931),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: Some(
                    rgb(0xaca8ae),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x2a2f39),
                border: rgb(0x2e323c),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: Some(
                    rgb(0xf7f7f8),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x21242b),
                border: rgb(0x23262d),
                foreground: rgb(0x474a53),
                secondary_foreground: Some(
                    rgb(0x474a53),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7f7f8),
                border: rgb(0x1e2025),
                foreground: rgb(0x2a2f39),
                secondary_foreground: Some(
                    rgb(0x2a2f39),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x21242b),
                border: rgb(0x252931),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x252931),
                border: rgb(0x252931),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x272c35),
                border: rgb(0x252931),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2a2f39),
                border: rgb(0x2e323c),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x21242b),
                border: rgb(0x23262d),
                foreground: rgb(0x474a53),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7f7f8),
                border: rgb(0x1e2025),
                foreground: rgb(0x2a2f39),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x262a33),
                border: rgb(0x2b2f39),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: Some(
                    rgb(0xaca8ae),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2b2f39),
                border: rgb(0x2b2f39),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: Some(
                    rgb(0xaca8ae),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2d313b),
                border: rgb(0x2b2f39),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: Some(
                    rgb(0xaca8ae),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x383b45),
                border: rgb(0x4e5059),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: Some(
                    rgb(0xf7f7f8),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x262a33),
                border: rgb(0x292d37),
                foreground: rgb(0x6b6b73),
                secondary_foreground: Some(
                    rgb(0x6b6b73),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7f7f8),
                border: rgb(0x1e2025),
                foreground: rgb(0x32363f),
                secondary_foreground: Some(
                    rgb(0x32363f),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1e2025),
                border: rgb(0x21232a),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x21232a),
                border: rgb(0x21232a),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x22252c),
                border: rgb(0x21232a),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x242830),
                border: rgb(0x292d37),
                foreground: rgb(0xf7f7f8),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1e2025),
                border: rgb(0x1f2227),
                foreground: rgb(0x2d313b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7f7f8),
                border: rgb(0x1e2025),
                foreground: rgb(0x24272f),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x122420),
                border: rgb(0x183a34),
                foreground: rgb(0x11a793),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x183a34),
                border: rgb(0x183a34),
                foreground: rgb(0x11a793),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1a473f),
                border: rgb(0x183a34),
                foreground: rgb(0x11a793),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x1b594f),
                border: rgb(0x1b6f62),
                foreground: rgb(0xf7fcfa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x122420),
                border: rgb(0x152f2a),
                foreground: rgb(0x178a7a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7fcfa),
                border: rgb(0x0000),
                foreground: rgb(0x1b544a),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x194618),
                border: rgb(0x306129),
                foreground: rgb(0x96df72),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x306129),
                border: rgb(0x306129),
                foreground: rgb(0x96df72),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3c7031),
                border: rgb(0x306129),
                foreground: rgb(0x96df72),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x4d853e),
                border: rgb(0x619f4c),
                foreground: rgb(0xfbfef9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x194618),
                border: rgb(0x255321),
                foreground: rgb(0x7bbf5f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbfef9),
                border: rgb(0x2500),
                foreground: rgb(0x488039),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x5c5015),
                border: rgb(0x796b26),
                foreground: rgb(0xfee56d),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x796b26),
                border: rgb(0x796b26),
                foreground: rgb(0xfee56d),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x89792e),
                border: rgb(0x796b26),
                foreground: rgb(0xfee56d),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xa08e3a),
                border: rgb(0xbaa748),
                foreground: rgb(0xfffef9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x5c5015),
                border: rgb(0x6a5d1e),
                foreground: rgb(0xdcc55a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfffef9),
                border: rgb(0x382f00),
                foreground: rgb(0x998836),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x55051b),
                border: rgb(0x720a2b),
                foreground: rgb(0xf82872),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x720a2b),
                border: rgb(0x720a2b),
                foreground: rgb(0xf82872),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x820d33),
                border: rgb(0x720a2b),
                foreground: rgb(0xf82872),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x99123f),
                border: rgb(0xb4184d),
                foreground: rgb(0xfff8f9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x55051b),
                border: rgb(0x630723),
                foreground: rgb(0xd61f5f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff8f9),
                border: rgb(0x320000),
                foreground: rgb(0x92113b),
                secondary_foreground: None,
            },
        },
    }
}
