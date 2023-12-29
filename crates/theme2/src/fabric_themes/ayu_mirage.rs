use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn ayu_mirage() -> FabricTheme {
    FabricTheme {
        name: "Ayu Mirage",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x353944ff),
                border: rgba(0x43464fff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: Some(
                    rgba(0x9a9a98ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x43464fff),
                border: rgba(0x43464fff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: Some(
                    rgba(0x9a9a98ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x494d55ff),
                border: rgba(0x43464fff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: Some(
                    rgba(0x9a9a98ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x53565dff),
                border: rgba(0x5d6066ff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: Some(
                    rgba(0xcccac2ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x353944ff),
                border: rgba(0x3c404aff),
                foreground: rgba(0x6b6d71ff),
                secondary_foreground: Some(
                    rgba(0x6b6d71ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xcccac2ff),
                border: rgba(0x242936ff),
                foreground: rgba(0x4f535aff),
                secondary_foreground: Some(
                    rgba(0x4f535aff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x353944ff),
                border: rgba(0x43464fff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x43464fff),
                border: rgba(0x43464fff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x494d55ff),
                border: rgba(0x43464fff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x53565dff),
                border: rgba(0x5d6066ff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x353944ff),
                border: rgba(0x3c404aff),
                foreground: rgba(0x6b6d71ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xcccac2ff),
                border: rgba(0x242936ff),
                foreground: rgba(0x4f535aff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x464a52ff),
                border: rgba(0x53565dff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: Some(
                    rgba(0x9a9a98ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x53565dff),
                border: rgba(0x53565dff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: Some(
                    rgba(0x9a9a98ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x5a5c63ff),
                border: rgba(0x53565dff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: Some(
                    rgba(0x9a9a98ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x63656aff),
                border: rgba(0x6e7074ff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: Some(
                    rgba(0xcccac2ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x464a52ff),
                border: rgba(0x4d5058ff),
                foreground: rgba(0x7b7d7fff),
                secondary_foreground: Some(
                    rgba(0x7b7d7fff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xcccac2ff),
                border: rgba(0x242936ff),
                foreground: rgba(0x606368ff),
                secondary_foreground: Some(
                    rgba(0x606368ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x242936ff),
                border: rgba(0x323641ff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x323641ff),
                border: rgba(0x323641ff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x383d47ff),
                border: rgba(0x323641ff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x42454eff),
                border: rgba(0x4d5058ff),
                foreground: rgba(0xcccac2ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x242936ff),
                border: rgba(0x2b303cff),
                foreground: rgba(0x5a5c63ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xcccac2ff),
                border: rgba(0x242936ff),
                foreground: rgba(0x3f434dff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x123a50ff),
                border: rgba(0x24556fff),
                foreground: rgba(0x73cffeff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x24556fff),
                border: rgba(0x24556fff),
                foreground: rgba(0x73cffeff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2d6380ff),
                border: rgba(0x24556fff),
                foreground: rgba(0x73cffeff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x3b7898ff),
                border: rgba(0x4a90b5ff),
                foreground: rgba(0xfafdffff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x123a50ff),
                border: rgba(0x1b475fff),
                foreground: rgba(0x5eafd9ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfafdffff),
                border: rgba(0x001b2bff),
                foreground: rgba(0x367292ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x426118ff),
                border: rgba(0x5d7e2cff),
                foreground: rgba(0xd5fe80ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x5d7e2cff),
                border: rgba(0x5d7e2cff),
                foreground: rgba(0xd5fe80ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x6b8d36ff),
                border: rgba(0x5d7e2cff),
                foreground: rgba(0xd5fe80ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x7fa344ff),
                border: rgba(0x97bd54ff),
                foreground: rgba(0xfefffaff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x426118ff),
                border: rgba(0x506f22ff),
                foreground: rgba(0xb5dd6aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfefffaff),
                border: rgba(0x223e00ff),
                foreground: rgba(0x7a9d3fff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x584018ff),
                border: rgba(0x765a29ff),
                foreground: rgba(0xfed073ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x765a29ff),
                border: rgba(0x765a29ff),
                foreground: rgba(0xfed073ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x876831ff),
                border: rgba(0x765a29ff),
                foreground: rgba(0xfed073ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9d7c3eff),
                border: rgba(0xb9944cff),
                foreground: rgba(0xfffdf9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x584018ff),
                border: rgba(0x674d21ff),
                foreground: rgba(0xdbb15fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfffdf9ff),
                border: rgba(0x342100ff),
                foreground: rgba(0x97763aff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x481b1cff),
                border: rgba(0x662e2dff),
                foreground: rgba(0xf18779ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x662e2dff),
                border: rgba(0x662e2dff),
                foreground: rgba(0xf18779ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x773936ff),
                border: rgba(0x662e2dff),
                foreground: rgba(0xf18779ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x8e4742ff),
                border: rgba(0xaa5951ff),
                foreground: rgba(0xfffaf9ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x481b1cff),
                border: rgba(0x572524ff),
                foreground: rgba(0xcd7065ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfffaf9ff),
                border: rgba(0x270000ff),
                foreground: rgba(0x88433eff),
                secondary_foreground: None,
            },
        },
    }
}
