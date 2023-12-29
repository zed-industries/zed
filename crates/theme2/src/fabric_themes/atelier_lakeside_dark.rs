use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_lakeside_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Lakeside Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1c2529ff),
                border: rgba(0x2c3b42ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: Some(
                    rgba(0x7ca0b3ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2c3b42ff),
                border: rgba(0x2c3b42ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: Some(
                    rgba(0x7ca0b3ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3a4e58ff),
                border: rgba(0x2c3b42ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: Some(
                    rgba(0x7ca0b3ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x4d6976ff),
                border: rgba(0x557382ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: Some(
                    rgba(0xebf8ffff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1c2529ff),
                border: rgba(0x1f292eff),
                foreground: rgba(0x5a7b8bff),
                secondary_foreground: Some(
                    rgba(0x5a7b8bff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xebf8ffff),
                border: rgba(0x161b1dff),
                foreground: rgba(0x48616dff),
                secondary_foreground: Some(
                    rgba(0x48616dff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1c2529ff),
                border: rgba(0x2c3b42ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2c3b42ff),
                border: rgba(0x2c3b42ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3a4e58ff),
                border: rgba(0x2c3b42ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x4d6976ff),
                border: rgba(0x557382ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1c2529ff),
                border: rgba(0x1f292eff),
                foreground: rgba(0x5a7b8bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xebf8ffff),
                border: rgba(0x161b1dff),
                foreground: rgba(0x48616dff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x33444dff),
                border: rgba(0x4f6b78ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: Some(
                    rgba(0x7ca0b3ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x4f6b78ff),
                border: rgba(0x4f6b78ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: Some(
                    rgba(0x7ca0b3ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x53717fff),
                border: rgba(0x4f6b78ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: Some(
                    rgba(0x7ca0b3ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x577685ff),
                border: rgba(0x5c7d8eff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: Some(
                    rgba(0xebf8ffff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x33444dff),
                border: rgba(0x415763ff),
                foreground: rgba(0x698c9eff),
                secondary_foreground: Some(
                    rgba(0x698c9eff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xebf8ffff),
                border: rgba(0x161b1dff),
                foreground: rgba(0x567584ff),
                secondary_foreground: Some(
                    rgba(0x567584ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x161b1dff),
                border: rgba(0x1b2327ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1b2327ff),
                border: rgba(0x1b2327ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1e272bff),
                border: rgba(0x1b2327ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2b3940ff),
                border: rgba(0x415763ff),
                foreground: rgba(0xebf8ffff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x161b1dff),
                border: rgba(0x191f22ff),
                foreground: rgba(0x53717fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xebf8ffff),
                border: rgba(0x161b1dff),
                foreground: rgba(0x253137ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x131d24ff),
                border: rgba(0x1a2f3cff),
                foreground: rgba(0x277fadff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1a2f3cff),
                border: rgba(0x1a2f3cff),
                foreground: rgba(0x277fadff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1d3849ff),
                border: rgba(0x1a2f3cff),
                foreground: rgba(0x277fadff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x20455cff),
                border: rgba(0x235572ff),
                foreground: rgba(0xf7fafcff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x131d24ff),
                border: rgba(0x172630ff),
                foreground: rgba(0x256a8fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7fafcff),
                border: rgba(0x000000ff),
                foreground: rgba(0x1f4156ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x171f12ff),
                border: rgba(0x23321bff),
                foreground: rgba(0x568c3bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x23321bff),
                border: rgba(0x23321bff),
                foreground: rgba(0x568c3bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x293d1fff),
                border: rgba(0x23321bff),
                foreground: rgba(0x568c3bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x324c24ff),
                border: rgba(0x3c5d2bff),
                foreground: rgba(0xf8faf7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x171f12ff),
                border: rgba(0x1d2917ff),
                foreground: rgba(0x497533ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8faf7ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x2f4723ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x201f0cff),
                border: rgba(0x333211ff),
                foreground: rgba(0x8a8a11ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x333211ff),
                border: rgba(0x333211ff),
                foreground: rgba(0x8a8a11ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3d3c12ff),
                border: rgba(0x333211ff),
                foreground: rgba(0x8a8a11ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x4c4b13ff),
                border: rgba(0x5d5c14ff),
                foreground: rgba(0xfbfaf5ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x201f0cff),
                border: rgba(0x2a280fff),
                foreground: rgba(0x737313ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbfaf5ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x474613ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3a101bff),
                border: rgba(0x55162bff),
                foreground: rgba(0xd22e72ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x55162bff),
                border: rgba(0x55162bff),
                foreground: rgba(0xd22e72ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x641a33ff),
                border: rgba(0x55162bff),
                foreground: rgba(0xd22e72ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x781d3fff),
                border: rgba(0x92224dff),
                foreground: rgba(0xfef7f9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3a101bff),
                border: rgba(0x471323ff),
                foreground: rgba(0xb1285fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfef7f9ff),
                border: rgba(0x1d0000ff),
                foreground: rgba(0x731d3bff),
                secondary_foreground: None,
            },
        },
    }
}
