use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn solarized_light() -> FabricTheme {
    FabricTheme {
        name: "Solarized Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf3eddaff),
                border: rgba(0xdcdacbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: Some(
                    rgba(0x34555eff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdcdacbff),
                border: rgba(0xdcdacbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: Some(
                    rgba(0x34555eff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc3c7bdff),
                border: rgba(0xdcdacbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: Some(
                    rgba(0x34555eff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xa2aca9ff),
                border: rgba(0x869798ff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: Some(
                    rgba(0x002b36ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf3eddaff),
                border: rgba(0xefe9d6ff),
                foreground: rgba(0x788b8fff),
                secondary_foreground: Some(
                    rgba(0x788b8fff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x002b36ff),
                border: rgba(0xfdf6e3ff),
                foreground: rgba(0xacb4afff),
                secondary_foreground: Some(
                    rgba(0xacb4afff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf3eddaff),
                border: rgba(0xdcdacbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdcdacbff),
                border: rgba(0xdcdacbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc3c7bdff),
                border: rgba(0xdcdacbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xa2aca9ff),
                border: rgba(0x869798ff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf3eddaff),
                border: rgba(0xefe9d6ff),
                foreground: rgba(0x788b8fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x002b36ff),
                border: rgba(0xfdf6e3ff),
                foreground: rgba(0xacb4afff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xcfd0c4ff),
                border: rgba(0x9faaa8ff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: Some(
                    rgba(0x34555eff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x9faaa8ff),
                border: rgba(0x9faaa8ff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: Some(
                    rgba(0x34555eff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x8d9c9dff),
                border: rgba(0x9faaa8ff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: Some(
                    rgba(0x34555eff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x7f9194ff),
                border: rgba(0x75888dff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: Some(
                    rgba(0x002b36ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xcfd0c4ff),
                border: rgba(0xb7bdb6ff),
                foreground: rgba(0x6a7f86ff),
                secondary_foreground: Some(
                    rgba(0x6a7f86ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x002b36ff),
                border: rgba(0xfdf6e3ff),
                foreground: rgba(0x819395ff),
                secondary_foreground: Some(
                    rgba(0x819395ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfdf6e3ff),
                border: rgba(0xf5eedbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf5eedbff),
                border: rgba(0xf5eedbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xf1ebd8ff),
                border: rgba(0xf5eedbff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xdedcccff),
                border: rgba(0xb7bdb6ff),
                foreground: rgba(0x002b36ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfdf6e3ff),
                border: rgba(0xf9f2dfff),
                foreground: rgba(0x8d9c9dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x002b36ff),
                border: rgba(0xfdf6e3ff),
                foreground: rgba(0xe8e4d1ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdbe6f6ff),
                border: rgba(0xbfd3efff),
                foreground: rgba(0x298bd1ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbfd3efff),
                border: rgba(0xbfd3efff),
                foreground: rgba(0x298bd1ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb0caecff),
                border: rgba(0xbfd3efff),
                foreground: rgba(0x298bd1ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9bbde7ff),
                border: rgba(0x81afe1ff),
                foreground: rgba(0x060810ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdbe6f6ff),
                border: rgba(0xcdddf3ff),
                foreground: rgba(0x5c9dd9ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x060810ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa1c1e8ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe9ead0ff),
                border: rgba(0xd6d9abff),
                foreground: rgba(0x859904ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd6d9abff),
                border: rgba(0xd6d9abff),
                foreground: rgba(0x859904ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xcdd099ff),
                border: rgba(0xd6d9abff),
                foreground: rgba(0x859904ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xbfc57fff),
                border: rgba(0xafb962ff),
                foreground: rgba(0x090903ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe9ead0ff),
                border: rgba(0xdfe1beff),
                foreground: rgba(0x9ba93cff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x090903ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xc4c986ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf5e6d0ff),
                border: rgba(0xebd3aaff),
                foreground: rgba(0xb58904ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xebd3aaff),
                border: rgba(0xebd3aaff),
                foreground: rgba(0xb58904ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe5c997ff),
                border: rgba(0xebd3aaff),
                foreground: rgba(0xb58904ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xddbb7eff),
                border: rgba(0xd3ad61ff),
                foreground: rgba(0x190802ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf5e6d0ff),
                border: rgba(0xf0dcbdff),
                foreground: rgba(0xc59b3aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x190802ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xe0c085ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xffd9d2ff),
                border: rgba(0xffbbafff),
                foreground: rgba(0xdc3330ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xffbbafff),
                border: rgba(0xffbbafff),
                foreground: rgba(0xdc3330ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xfcac9eff),
                border: rgba(0xffbbafff),
                foreground: rgba(0xdc3330ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xf99687ff),
                border: rgba(0xf27c6cff),
                foreground: rgba(0x310303ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xffd9d2ff),
                border: rgba(0xffcac1ff),
                foreground: rgba(0xe85b4dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x310303ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xfa9c8dff),
                secondary_foreground: None,
            },
        },
    }
}
