use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn ayu_mirage() -> FabricTheme {
    FabricTheme {
        name: "Ayu Mirage".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x353944),
                border: rgb(0x43464f),
                foreground: rgb(0xcccac2),
                secondary_foreground: Some(
                    rgb(0x9a9a98),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x43464f),
                border: rgb(0x43464f),
                foreground: rgb(0xcccac2),
                secondary_foreground: Some(
                    rgb(0x9a9a98),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x494d55),
                border: rgb(0x43464f),
                foreground: rgb(0xcccac2),
                secondary_foreground: Some(
                    rgb(0x9a9a98),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x53565d),
                border: rgb(0x5d6066),
                foreground: rgb(0xcccac2),
                secondary_foreground: Some(
                    rgb(0xcccac2),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x353944),
                border: rgb(0x3c404a),
                foreground: rgb(0x6b6d71),
                secondary_foreground: Some(
                    rgb(0x6b6d71),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xcccac2),
                border: rgb(0x242936),
                foreground: rgb(0x4f535a),
                secondary_foreground: Some(
                    rgb(0x4f535a),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x353944),
                border: rgb(0x43464f),
                foreground: rgb(0xcccac2),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x43464f),
                border: rgb(0x43464f),
                foreground: rgb(0xcccac2),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x494d55),
                border: rgb(0x43464f),
                foreground: rgb(0xcccac2),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x53565d),
                border: rgb(0x5d6066),
                foreground: rgb(0xcccac2),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x353944),
                border: rgb(0x3c404a),
                foreground: rgb(0x6b6d71),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xcccac2),
                border: rgb(0x242936),
                foreground: rgb(0x4f535a),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x464a52),
                border: rgb(0x53565d),
                foreground: rgb(0xcccac2),
                secondary_foreground: Some(
                    rgb(0x9a9a98),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x53565d),
                border: rgb(0x53565d),
                foreground: rgb(0xcccac2),
                secondary_foreground: Some(
                    rgb(0x9a9a98),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x5a5c63),
                border: rgb(0x53565d),
                foreground: rgb(0xcccac2),
                secondary_foreground: Some(
                    rgb(0x9a9a98),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x63656a),
                border: rgb(0x6e7074),
                foreground: rgb(0xcccac2),
                secondary_foreground: Some(
                    rgb(0xcccac2),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x464a52),
                border: rgb(0x4d5058),
                foreground: rgb(0x7b7d7f),
                secondary_foreground: Some(
                    rgb(0x7b7d7f),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xcccac2),
                border: rgb(0x242936),
                foreground: rgb(0x606368),
                secondary_foreground: Some(
                    rgb(0x606368),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x242936),
                border: rgb(0x323641),
                foreground: rgb(0xcccac2),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x323641),
                border: rgb(0x323641),
                foreground: rgb(0xcccac2),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x383d47),
                border: rgb(0x323641),
                foreground: rgb(0xcccac2),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x42454e),
                border: rgb(0x4d5058),
                foreground: rgb(0xcccac2),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x242936),
                border: rgb(0x2b303c),
                foreground: rgb(0x5a5c63),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xcccac2),
                border: rgb(0x242936),
                foreground: rgb(0x3f434d),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x123a50),
                border: rgb(0x24556f),
                foreground: rgb(0x73cffe),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x24556f),
                border: rgb(0x24556f),
                foreground: rgb(0x73cffe),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2d6380),
                border: rgb(0x24556f),
                foreground: rgb(0x73cffe),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x3b7898),
                border: rgb(0x4a90b5),
                foreground: rgb(0xfafdff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x123a50),
                border: rgb(0x1b475f),
                foreground: rgb(0x5eafd9),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfafdff),
                border: rgb(0x1b2b),
                foreground: rgb(0x367292),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x426118),
                border: rgb(0x5d7e2c),
                foreground: rgb(0xd5fe80),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x5d7e2c),
                border: rgb(0x5d7e2c),
                foreground: rgb(0xd5fe80),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x6b8d36),
                border: rgb(0x5d7e2c),
                foreground: rgb(0xd5fe80),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x7fa344),
                border: rgb(0x97bd54),
                foreground: rgb(0xfefffa),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x426118),
                border: rgb(0x506f22),
                foreground: rgb(0xb5dd6a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfefffa),
                border: rgb(0x223e00),
                foreground: rgb(0x7a9d3f),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x584018),
                border: rgb(0x765a29),
                foreground: rgb(0xfed073),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x765a29),
                border: rgb(0x765a29),
                foreground: rgb(0xfed073),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x876831),
                border: rgb(0x765a29),
                foreground: rgb(0xfed073),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9d7c3e),
                border: rgb(0xb9944c),
                foreground: rgb(0xfffdf9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x584018),
                border: rgb(0x674d21),
                foreground: rgb(0xdbb15f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfffdf9),
                border: rgb(0x342100),
                foreground: rgb(0x97763a),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x481b1c),
                border: rgb(0x662e2d),
                foreground: rgb(0xf18779),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x662e2d),
                border: rgb(0x662e2d),
                foreground: rgb(0xf18779),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x773936),
                border: rgb(0x662e2d),
                foreground: rgb(0xf18779),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x8e4742),
                border: rgb(0xaa5951),
                foreground: rgb(0xfffaf9),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x481b1c),
                border: rgb(0x572524),
                foreground: rgb(0xcd7065),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfffaf9),
                border: rgb(0x270000),
                foreground: rgb(0x88433e),
                secondary_foreground: None,
            },
        },
    }
}
