use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_forest_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Forest Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x27211e),
                border: rgb(0x3b3431),
                foreground: rgb(0xf1efee),
                secondary_foreground: Some(
                    rgb(0xa79f9d),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3b3431),
                border: rgb(0x3b3431),
                foreground: rgb(0xf1efee),
                secondary_foreground: Some(
                    rgb(0xa79f9d),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4c4542),
                border: rgb(0x3b3431),
                foreground: rgb(0xf1efee),
                secondary_foreground: Some(
                    rgb(0xa79f9d),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x645d5a),
                border: rgb(0x6e6663),
                foreground: rgb(0xf1efee),
                secondary_foreground: Some(
                    rgb(0xf1efee),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x27211e),
                border: rgb(0x2c2421),
                foreground: rgb(0x766e6b),
                secondary_foreground: Some(
                    rgb(0x766e6b),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf1efee),
                border: rgb(0x1b1918),
                foreground: rgb(0x5d5653),
                secondary_foreground: Some(
                    rgb(0x5d5653),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x27211e),
                border: rgb(0x3b3431),
                foreground: rgb(0xf1efee),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3b3431),
                border: rgb(0x3b3431),
                foreground: rgb(0xf1efee),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4c4542),
                border: rgb(0x3b3431),
                foreground: rgb(0xf1efee),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x645d5a),
                border: rgb(0x6e6663),
                foreground: rgb(0xf1efee),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x27211e),
                border: rgb(0x2c2421),
                foreground: rgb(0x766e6b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf1efee),
                border: rgb(0x1b1918),
                foreground: rgb(0x5d5653),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x443c39),
                border: rgb(0x665f5c),
                foreground: rgb(0xf1efee),
                secondary_foreground: Some(
                    rgb(0xa79f9d),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x665f5c),
                border: rgb(0x665f5c),
                foreground: rgb(0xf1efee),
                secondary_foreground: Some(
                    rgb(0xa79f9d),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x6c6461),
                border: rgb(0x665f5c),
                foreground: rgb(0xf1efee),
                secondary_foreground: Some(
                    rgb(0xa79f9d),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x716966),
                border: rgb(0x79716e),
                foreground: rgb(0xf1efee),
                secondary_foreground: Some(
                    rgb(0xf1efee),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x443c39),
                border: rgb(0x554e4b),
                foreground: rgb(0x8e8683),
                secondary_foreground: Some(
                    rgb(0x8e8683),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf1efee),
                border: rgb(0x1b1918),
                foreground: rgb(0x706865),
                secondary_foreground: Some(
                    rgb(0x706865),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1b1918),
                border: rgb(0x251f1d),
                foreground: rgb(0xf1efee),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x251f1d),
                border: rgb(0x251f1d),
                foreground: rgb(0xf1efee),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x292220),
                border: rgb(0x251f1d),
                foreground: rgb(0xf1efee),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x3a322f),
                border: rgb(0x554e4b),
                foreground: rgb(0xf1efee),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1b1918),
                border: rgb(0x201c1b),
                foreground: rgb(0x6c6461),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf1efee),
                border: rgb(0x1b1918),
                foreground: rgb(0x332b28),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf1d3d),
                border: rgb(0x192e5b),
                foreground: rgb(0x417ee6),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x192e5b),
                border: rgb(0x192e5b),
                foreground: rgb(0x417ee6),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1d386c),
                border: rgb(0x192e5b),
                foreground: rgb(0x417ee6),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x244583),
                border: rgb(0x2c549f),
                foreground: rgb(0xf9f9fe),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf1d3d),
                border: rgb(0x14264c),
                foreground: rgb(0x3669c2),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf9f9fe),
                border: rgb(0x001b),
                foreground: rgb(0x22417c),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1d2110),
                border: rgb(0x2e3516),
                foreground: rgb(0x7b9727),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2e3516),
                border: rgb(0x2e3516),
                foreground: rgb(0x7b9727),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x374118),
                border: rgb(0x2e3516),
                foreground: rgb(0x7b9727),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x44511b),
                border: rgb(0x53641f),
                foreground: rgb(0xfafbf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1d2110),
                border: rgb(0x262b13),
                foreground: rgb(0x677d23),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfafbf6),
                border: rgb(0x0000),
                foreground: rgb(0x404c1b),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x371d0d),
                border: rgb(0x4f2f12),
                foreground: rgb(0xc38419),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x4f2f12),
                border: rgb(0x4f2f12),
                foreground: rgb(0xc38419),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x5d3914),
                border: rgb(0x4f2f12),
                foreground: rgb(0xc38419),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x704716),
                border: rgb(0x875817),
                foreground: rgb(0xfefaf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x371d0d),
                border: rgb(0x432611),
                foreground: rgb(0xa46e18),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfefaf6),
                border: rgb(0x1b0000),
                foreground: rgb(0x6b4315),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x550512),
                border: rgb(0x710c1b),
                foreground: rgb(0xf22d40),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x710c1b),
                border: rgb(0x710c1b),
                foreground: rgb(0xf22d40),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x810f20),
                border: rgb(0x710c1b),
                foreground: rgb(0xf22d40),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x961425),
                border: rgb(0xb01b2d),
                foreground: rgb(0xfff8f7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x550512),
                border: rgb(0x630817),
                foreground: rgb(0xd02437),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff8f7),
                border: rgb(0x340000),
                foreground: rgb(0x901324),
                secondary_foreground: None,
            },
        },
    }
}
