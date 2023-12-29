use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn solarized_light() -> FabricTheme {
    FabricTheme {
        name: "Solarized Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf3edda),
                border: rgb(0xdcdacb),
                foreground: rgb(0x2b36),
                secondary_foreground: Some(
                    rgb(0x34555e),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdcdacb),
                border: rgb(0xdcdacb),
                foreground: rgb(0x2b36),
                secondary_foreground: Some(
                    rgb(0x34555e),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc3c7bd),
                border: rgb(0xdcdacb),
                foreground: rgb(0x2b36),
                secondary_foreground: Some(
                    rgb(0x34555e),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xa2aca9),
                border: rgb(0x869798),
                foreground: rgb(0x2b36),
                secondary_foreground: Some(
                    rgb(0x2b36),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf3edda),
                border: rgb(0xefe9d6),
                foreground: rgb(0x788b8f),
                secondary_foreground: Some(
                    rgb(0x788b8f),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x2b36),
                border: rgb(0xfdf6e3),
                foreground: rgb(0xacb4af),
                secondary_foreground: Some(
                    rgb(0xacb4af),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf3edda),
                border: rgb(0xdcdacb),
                foreground: rgb(0x2b36),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdcdacb),
                border: rgb(0xdcdacb),
                foreground: rgb(0x2b36),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc3c7bd),
                border: rgb(0xdcdacb),
                foreground: rgb(0x2b36),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xa2aca9),
                border: rgb(0x869798),
                foreground: rgb(0x2b36),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf3edda),
                border: rgb(0xefe9d6),
                foreground: rgb(0x788b8f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x2b36),
                border: rgb(0xfdf6e3),
                foreground: rgb(0xacb4af),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xcfd0c4),
                border: rgb(0x9faaa8),
                foreground: rgb(0x2b36),
                secondary_foreground: Some(
                    rgb(0x34555e),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x9faaa8),
                border: rgb(0x9faaa8),
                foreground: rgb(0x2b36),
                secondary_foreground: Some(
                    rgb(0x34555e),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x8d9c9d),
                border: rgb(0x9faaa8),
                foreground: rgb(0x2b36),
                secondary_foreground: Some(
                    rgb(0x34555e),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x7f9194),
                border: rgb(0x75888d),
                foreground: rgb(0x2b36),
                secondary_foreground: Some(
                    rgb(0x2b36),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xcfd0c4),
                border: rgb(0xb7bdb6),
                foreground: rgb(0x6a7f86),
                secondary_foreground: Some(
                    rgb(0x6a7f86),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x2b36),
                border: rgb(0xfdf6e3),
                foreground: rgb(0x819395),
                secondary_foreground: Some(
                    rgb(0x819395),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfdf6e3),
                border: rgb(0xf5eedb),
                foreground: rgb(0x2b36),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf5eedb),
                border: rgb(0xf5eedb),
                foreground: rgb(0x2b36),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xf1ebd8),
                border: rgb(0xf5eedb),
                foreground: rgb(0x2b36),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xdedccc),
                border: rgb(0xb7bdb6),
                foreground: rgb(0x2b36),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfdf6e3),
                border: rgb(0xf9f2df),
                foreground: rgb(0x8d9c9d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x2b36),
                border: rgb(0xfdf6e3),
                foreground: rgb(0xe8e4d1),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdbe6f6),
                border: rgb(0xbfd3ef),
                foreground: rgb(0x298bd1),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbfd3ef),
                border: rgb(0xbfd3ef),
                foreground: rgb(0x298bd1),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb0caec),
                border: rgb(0xbfd3ef),
                foreground: rgb(0x298bd1),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9bbde7),
                border: rgb(0x81afe1),
                foreground: rgb(0x60810),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdbe6f6),
                border: rgb(0xcdddf3),
                foreground: rgb(0x5c9dd9),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x60810),
                border: rgb(0xffffff),
                foreground: rgb(0xa1c1e8),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe9ead0),
                border: rgb(0xd6d9ab),
                foreground: rgb(0x859904),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd6d9ab),
                border: rgb(0xd6d9ab),
                foreground: rgb(0x859904),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xcdd099),
                border: rgb(0xd6d9ab),
                foreground: rgb(0x859904),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xbfc57f),
                border: rgb(0xafb962),
                foreground: rgb(0x90903),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe9ead0),
                border: rgb(0xdfe1be),
                foreground: rgb(0x9ba93c),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x90903),
                border: rgb(0xffffff),
                foreground: rgb(0xc4c986),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf5e6d0),
                border: rgb(0xebd3aa),
                foreground: rgb(0xb58904),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xebd3aa),
                border: rgb(0xebd3aa),
                foreground: rgb(0xb58904),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe5c997),
                border: rgb(0xebd3aa),
                foreground: rgb(0xb58904),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xddbb7e),
                border: rgb(0xd3ad61),
                foreground: rgb(0x190802),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf5e6d0),
                border: rgb(0xf0dcbd),
                foreground: rgb(0xc59b3a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x190802),
                border: rgb(0xffffff),
                foreground: rgb(0xe0c085),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xffd9d2),
                border: rgb(0xffbbaf),
                foreground: rgb(0xdc3330),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xffbbaf),
                border: rgb(0xffbbaf),
                foreground: rgb(0xdc3330),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xfcac9e),
                border: rgb(0xffbbaf),
                foreground: rgb(0xdc3330),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xf99687),
                border: rgb(0xf27c6c),
                foreground: rgb(0x310303),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xffd9d2),
                border: rgb(0xffcac1),
                foreground: rgb(0xe85b4d),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x310303),
                border: rgb(0xffffff),
                foreground: rgb(0xfa9c8d),
                secondary_foreground: None,
            },
        },
    }
}
