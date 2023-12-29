use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn solarized_dark() -> FabricTheme {
    FabricTheme {
        name: "Solarized Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x4313c),
                border: rgb(0x63541),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: Some(
                    rgb(0x93a1a1),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x63541),
                border: rgb(0x63541),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: Some(
                    rgb(0x93a1a1),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x103c47),
                border: rgb(0x63541),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: Some(
                    rgb(0x93a1a1),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x294e58),
                border: rgb(0x46626a),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: Some(
                    rgb(0xfdf6e3),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x4313c),
                border: rgb(0x5333e),
                foreground: rgb(0x5f757d),
                secondary_foreground: Some(
                    rgb(0x5f757d),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf6e3),
                border: rgb(0x2b36),
                foreground: rgb(0x224853),
                secondary_foreground: Some(
                    rgb(0x224853),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x4313c),
                border: rgb(0x63541),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x63541),
                border: rgb(0x63541),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x103c47),
                border: rgb(0x63541),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x294e58),
                border: rgb(0x46626a),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x4313c),
                border: rgb(0x5333e),
                foreground: rgb(0x5f757d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf6e3),
                border: rgb(0x2b36),
                foreground: rgb(0x224853),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x83743),
                border: rgb(0x2b4f58),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: Some(
                    rgb(0x93a1a1),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2b4f58),
                border: rgb(0x2b4f58),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: Some(
                    rgb(0x93a1a1),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3d5b64),
                border: rgb(0x2b4f58),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: Some(
                    rgb(0x93a1a1),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x566d74),
                border: rgb(0x637981),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: Some(
                    rgb(0xfdf6e3),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x83743),
                border: rgb(0x19424d),
                foreground: rgb(0x6f8389),
                secondary_foreground: Some(
                    rgb(0x6f8389),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf6e3),
                border: rgb(0x2b36),
                foreground: rgb(0x4f686f),
                secondary_foreground: Some(
                    rgb(0x4f686f),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2b36),
                border: rgb(0x32f3b),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x32f3b),
                border: rgb(0x32f3b),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4323d),
                border: rgb(0x32f3b),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x63440),
                border: rgb(0x19424d),
                foreground: rgb(0xfdf6e3),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2b36),
                border: rgb(0x12d38),
                foreground: rgb(0x3d5b64),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdf6e3),
                border: rgb(0x2b36),
                foreground: rgb(0x63440),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x141f2c),
                border: rgb(0x1c3249),
                foreground: rgb(0x288bd1),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1c3249),
                border: rgb(0x1c3249),
                foreground: rgb(0x288bd1),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1f3c59),
                border: rgb(0x1c3249),
                foreground: rgb(0x288bd1),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x224b6f),
                border: rgb(0x255d8b),
                foreground: rgb(0xf8fafd),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x141f2c),
                border: rgb(0x18283a),
                foreground: rgb(0x2774ad),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8fafd),
                border: rgb(0x0004),
                foreground: rgb(0x214669),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1f210c),
                border: rgb(0x323610),
                foreground: rgb(0x859904),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x323610),
                border: rgb(0x323610),
                foreground: rgb(0x859904),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3b4111),
                border: rgb(0x323610),
                foreground: rgb(0x859904),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x495211),
                border: rgb(0x5a6610),
                foreground: rgb(0xfbfbf5),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1f210c),
                border: rgb(0x292c0f),
                foreground: rgb(0x6f7f0b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbfbf5),
                border: rgb(0x0000),
                foreground: rgb(0x454d11),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2f1e0c),
                border: rgb(0x473110),
                foreground: rgb(0xb58903),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x473110),
                border: rgb(0x473110),
                foreground: rgb(0xb58903),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x533b10),
                border: rgb(0x473110),
                foreground: rgb(0xb58903),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x664a10),
                border: rgb(0x7c5b0f),
                foreground: rgb(0xfdfaf5),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2f1e0c),
                border: rgb(0x3b280e),
                foreground: rgb(0x98720a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdfaf5),
                border: rgb(0x130000),
                foreground: rgb(0x614510),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x4a090f),
                border: rgb(0x641116),
                foreground: rgb(0xdc3330),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x641116),
                border: rgb(0x641116),
                foreground: rgb(0xdc3330),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x721519),
                border: rgb(0x641116),
                foreground: rgb(0xdc3330),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x861a1d),
                border: rgb(0x9e2123),
                foreground: rgb(0xfff7f6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x4a090f),
                border: rgb(0x570d13),
                foreground: rgb(0xbc2a29),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff7f6),
                border: rgb(0x2c0000),
                foreground: rgb(0x81191c),
                secondary_foreground: None,
            },
        },
    }
}
