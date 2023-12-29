use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_forest_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Forest Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe9e6e4),
                border: rgb(0xd6d1cf),
                foreground: rgb(0x1b1918),
                secondary_foreground: Some(
                    rgb(0x6a6360),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd6d1cf),
                border: rgb(0xd6d1cf),
                foreground: rgb(0x1b1918),
                secondary_foreground: Some(
                    rgb(0x6a6360),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc5bfbd),
                border: rgb(0xd6d1cf),
                foreground: rgb(0x1b1918),
                secondary_foreground: Some(
                    rgb(0x6a6360),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xaca5a3),
                border: rgb(0xa39c99),
                foreground: rgb(0x1b1918),
                secondary_foreground: Some(
                    rgb(0x1b1918),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe9e6e4),
                border: rgb(0xe6e2e0),
                foreground: rgb(0x9c9491),
                secondary_foreground: Some(
                    rgb(0x9c9491),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b1918),
                border: rgb(0xf1efee),
                foreground: rgb(0xb3adab),
                secondary_foreground: Some(
                    rgb(0xb3adab),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe9e6e4),
                border: rgb(0xd6d1cf),
                foreground: rgb(0x1b1918),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd6d1cf),
                border: rgb(0xd6d1cf),
                foreground: rgb(0x1b1918),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc5bfbd),
                border: rgb(0xd6d1cf),
                foreground: rgb(0x1b1918),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xaca5a3),
                border: rgb(0xa39c99),
                foreground: rgb(0x1b1918),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe9e6e4),
                border: rgb(0xe6e2e0),
                foreground: rgb(0x9c9491),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b1918),
                border: rgb(0xf1efee),
                foreground: rgb(0xb3adab),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xcdc8c6),
                border: rgb(0xaaa3a1),
                foreground: rgb(0x1b1918),
                secondary_foreground: Some(
                    rgb(0x6a6360),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xaaa3a1),
                border: rgb(0xaaa3a1),
                foreground: rgb(0x1b1918),
                secondary_foreground: Some(
                    rgb(0x6a6360),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xa59e9b),
                border: rgb(0xaaa3a1),
                foreground: rgb(0x1b1918),
                secondary_foreground: Some(
                    rgb(0x6a6360),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xa19996),
                border: rgb(0x99918e),
                foreground: rgb(0x1b1918),
                secondary_foreground: Some(
                    rgb(0x1b1918),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xcdc8c6),
                border: rgb(0xbcb6b4),
                foreground: rgb(0x847c79),
                secondary_foreground: Some(
                    rgb(0x847c79),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b1918),
                border: rgb(0xf1efee),
                foreground: rgb(0xa19a97),
                secondary_foreground: Some(
                    rgb(0xa19a97),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf1efee),
                border: rgb(0xebe8e6),
                foreground: rgb(0x1b1918),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xebe8e6),
                border: rgb(0xebe8e6),
                foreground: rgb(0x1b1918),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe8e4e2),
                border: rgb(0xebe8e6),
                foreground: rgb(0x1b1918),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd7d3d1),
                border: rgb(0xbcb6b4),
                foreground: rgb(0x1b1918),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf1efee),
                border: rgb(0xeeebea),
                foreground: rgb(0xa59e9b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b1918),
                border: rgb(0xf1efee),
                foreground: rgb(0xdfdad8),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdfe3fb),
                border: rgb(0xc6cef7),
                foreground: rgb(0x417ee6),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc6cef7),
                border: rgb(0xc6cef7),
                foreground: rgb(0x417ee6),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb8c4f5),
                border: rgb(0xc6cef7),
                foreground: rgb(0x417ee6),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xa5b5f3),
                border: rgb(0x8da5ef),
                foreground: rgb(0x60821),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdfe3fb),
                border: rgb(0xd2d8f9),
                foreground: rgb(0x6c91eb),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x60821),
                border: rgb(0xffffff),
                foreground: rgb(0xaabaf4),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe5e9d3),
                border: rgb(0xd1d8b1),
                foreground: rgb(0x7b9728),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd1d8b1),
                border: rgb(0xd1d8b1),
                foreground: rgb(0x7b9728),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc7d09f),
                border: rgb(0xd1d8b1),
                foreground: rgb(0x7b9728),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xb8c488),
                border: rgb(0xa7b86d),
                foreground: rgb(0x80903),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe5e9d3),
                border: rgb(0xdbe1c2),
                foreground: rgb(0x91a74b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x80903),
                border: rgb(0xffffff),
                foreground: rgb(0xbdc88e),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf8e5d1),
                border: rgb(0xf0d1ad),
                foreground: rgb(0xc3841a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf0d1ad),
                border: rgb(0xf0d1ad),
                foreground: rgb(0xc3841a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xecc79b),
                border: rgb(0xf0d1ad),
                foreground: rgb(0xc3841a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xe6b983),
                border: rgb(0xddaa66),
                foreground: rgb(0x200803),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf8e5d1),
                border: rgb(0xf4dbbf),
                foreground: rgb(0xd09743),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x200803),
                border: rgb(0xffffff),
                foreground: rgb(0xe7bd89),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xffdad5),
                border: rgb(0xffbdb6),
                foreground: rgb(0xf22e41),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xffbdb6),
                border: rgb(0xffbdb6),
                foreground: rgb(0xf22e41),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xffada6),
                border: rgb(0xffbdb6),
                foreground: rgb(0xf22e41),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xff9891),
                border: rgb(0xff7e78),
                foreground: rgb(0x3a0104),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xffdad5),
                border: rgb(0xffccc6),
                foreground: rgb(0xfa5b5c),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x3a0104),
                border: rgb(0xffffff),
                foreground: rgb(0xff9e97),
                secondary_foreground: None,
            },
        },
    }
}
