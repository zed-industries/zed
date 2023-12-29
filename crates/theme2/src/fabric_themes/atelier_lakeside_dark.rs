use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_lakeside_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Lakeside Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1c2529),
                border: rgb(0x2c3b42),
                foreground: rgb(0xebf8ff),
                secondary_foreground: Some(
                    rgb(0x7ca0b3),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2c3b42),
                border: rgb(0x2c3b42),
                foreground: rgb(0xebf8ff),
                secondary_foreground: Some(
                    rgb(0x7ca0b3),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3a4e58),
                border: rgb(0x2c3b42),
                foreground: rgb(0xebf8ff),
                secondary_foreground: Some(
                    rgb(0x7ca0b3),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x4d6976),
                border: rgb(0x557382),
                foreground: rgb(0xebf8ff),
                secondary_foreground: Some(
                    rgb(0xebf8ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1c2529),
                border: rgb(0x1f292e),
                foreground: rgb(0x5a7b8b),
                secondary_foreground: Some(
                    rgb(0x5a7b8b),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xebf8ff),
                border: rgb(0x161b1d),
                foreground: rgb(0x48616d),
                secondary_foreground: Some(
                    rgb(0x48616d),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1c2529),
                border: rgb(0x2c3b42),
                foreground: rgb(0xebf8ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x2c3b42),
                border: rgb(0x2c3b42),
                foreground: rgb(0xebf8ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3a4e58),
                border: rgb(0x2c3b42),
                foreground: rgb(0xebf8ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x4d6976),
                border: rgb(0x557382),
                foreground: rgb(0xebf8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1c2529),
                border: rgb(0x1f292e),
                foreground: rgb(0x5a7b8b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xebf8ff),
                border: rgb(0x161b1d),
                foreground: rgb(0x48616d),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x33444d),
                border: rgb(0x4f6b78),
                foreground: rgb(0xebf8ff),
                secondary_foreground: Some(
                    rgb(0x7ca0b3),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x4f6b78),
                border: rgb(0x4f6b78),
                foreground: rgb(0xebf8ff),
                secondary_foreground: Some(
                    rgb(0x7ca0b3),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x53717f),
                border: rgb(0x4f6b78),
                foreground: rgb(0xebf8ff),
                secondary_foreground: Some(
                    rgb(0x7ca0b3),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x577685),
                border: rgb(0x5c7d8e),
                foreground: rgb(0xebf8ff),
                secondary_foreground: Some(
                    rgb(0xebf8ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x33444d),
                border: rgb(0x415763),
                foreground: rgb(0x698c9e),
                secondary_foreground: Some(
                    rgb(0x698c9e),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xebf8ff),
                border: rgb(0x161b1d),
                foreground: rgb(0x567584),
                secondary_foreground: Some(
                    rgb(0x567584),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x161b1d),
                border: rgb(0x1b2327),
                foreground: rgb(0xebf8ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1b2327),
                border: rgb(0x1b2327),
                foreground: rgb(0xebf8ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1e272b),
                border: rgb(0x1b2327),
                foreground: rgb(0xebf8ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x2b3940),
                border: rgb(0x415763),
                foreground: rgb(0xebf8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x161b1d),
                border: rgb(0x191f22),
                foreground: rgb(0x53717f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xebf8ff),
                border: rgb(0x161b1d),
                foreground: rgb(0x253137),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x131d24),
                border: rgb(0x1a2f3c),
                foreground: rgb(0x277fad),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1a2f3c),
                border: rgb(0x1a2f3c),
                foreground: rgb(0x277fad),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1d3849),
                border: rgb(0x1a2f3c),
                foreground: rgb(0x277fad),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x20455c),
                border: rgb(0x235572),
                foreground: rgb(0xf7fafc),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x131d24),
                border: rgb(0x172630),
                foreground: rgb(0x256a8f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7fafc),
                border: rgb(0x0000),
                foreground: rgb(0x1f4156),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x171f12),
                border: rgb(0x23321b),
                foreground: rgb(0x568c3b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x23321b),
                border: rgb(0x23321b),
                foreground: rgb(0x568c3b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x293d1f),
                border: rgb(0x23321b),
                foreground: rgb(0x568c3b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x324c24),
                border: rgb(0x3c5d2b),
                foreground: rgb(0xf8faf7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x171f12),
                border: rgb(0x1d2917),
                foreground: rgb(0x497533),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8faf7),
                border: rgb(0x0000),
                foreground: rgb(0x2f4723),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x201f0c),
                border: rgb(0x333211),
                foreground: rgb(0x8a8a11),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x333211),
                border: rgb(0x333211),
                foreground: rgb(0x8a8a11),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x3d3c12),
                border: rgb(0x333211),
                foreground: rgb(0x8a8a11),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x4c4b13),
                border: rgb(0x5d5c14),
                foreground: rgb(0xfbfaf5),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x201f0c),
                border: rgb(0x2a280f),
                foreground: rgb(0x737313),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbfaf5),
                border: rgb(0x0000),
                foreground: rgb(0x474613),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3a101b),
                border: rgb(0x55162b),
                foreground: rgb(0xd22e72),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x55162b),
                border: rgb(0x55162b),
                foreground: rgb(0xd22e72),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x641a33),
                border: rgb(0x55162b),
                foreground: rgb(0xd22e72),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x781d3f),
                border: rgb(0x92224d),
                foreground: rgb(0xfef7f9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3a101b),
                border: rgb(0x471323),
                foreground: rgb(0xb1285f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfef7f9),
                border: rgb(0x1d0000),
                foreground: rgb(0x731d3b),
                secondary_foreground: None,
            },
        },
    }
}
