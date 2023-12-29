use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_sulphurpool_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Sulphurpool Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe5e8f5),
                border: rgb(0xccd0e1),
                foreground: rgb(0x202746),
                secondary_foreground: Some(
                    rgb(0x606889),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xccd0e1),
                border: rgb(0xccd0e1),
                foreground: rgb(0x202746),
                secondary_foreground: Some(
                    rgb(0x606889),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb8bdd0),
                border: rgb(0xccd0e1),
                foreground: rgb(0x202746),
                secondary_foreground: Some(
                    rgb(0x606889),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x9ca1b8),
                border: rgb(0x9197ae),
                foreground: rgb(0x202746),
                secondary_foreground: Some(
                    rgb(0x202746),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe5e8f5),
                border: rgb(0xdfe2f1),
                foreground: rgb(0x898fa5),
                secondary_foreground: Some(
                    rgb(0x898fa5),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x202746),
                border: rgb(0xf5f7ff),
                foreground: rgb(0xa4a9bf),
                secondary_foreground: Some(
                    rgb(0xa4a9bf),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe5e8f5),
                border: rgb(0xccd0e1),
                foreground: rgb(0x202746),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xccd0e1),
                border: rgb(0xccd0e1),
                foreground: rgb(0x202746),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb8bdd0),
                border: rgb(0xccd0e1),
                foreground: rgb(0x202746),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9ca1b8),
                border: rgb(0x9197ae),
                foreground: rgb(0x202746),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe5e8f5),
                border: rgb(0xdfe2f1),
                foreground: rgb(0x898fa5),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x202746),
                border: rgb(0xf5f7ff),
                foreground: rgb(0xa4a9bf),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xc2c6d9),
                border: rgb(0x9a9fb6),
                foreground: rgb(0x202746),
                secondary_foreground: Some(
                    rgb(0x606889),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x9a9fb6),
                border: rgb(0x9a9fb6),
                foreground: rgb(0x202746),
                secondary_foreground: Some(
                    rgb(0x606889),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x9399b0),
                border: rgb(0x9a9fb6),
                foreground: rgb(0x202746),
                secondary_foreground: Some(
                    rgb(0x606889),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x8e94aa),
                border: rgb(0x878ca2),
                foreground: rgb(0x202746),
                secondary_foreground: Some(
                    rgb(0x202746),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xc2c6d9),
                border: rgb(0xaeb3c7),
                foreground: rgb(0x767d9a),
                secondary_foreground: Some(
                    rgb(0x767d9a),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x202746),
                border: rgb(0xf5f7ff),
                foreground: rgb(0x8f95ab),
                secondary_foreground: Some(
                    rgb(0x8f95ab),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf5f7ff),
                border: rgb(0xe9ebf7),
                foreground: rgb(0x202746),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe9ebf7),
                border: rgb(0xe9ebf7),
                foreground: rgb(0x202746),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe2e5f3),
                border: rgb(0xe9ebf7),
                foreground: rgb(0x202746),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xced2e3),
                border: rgb(0xaeb3c7),
                foreground: rgb(0x202746),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf5f7ff),
                border: rgb(0xeff1fb),
                foreground: rgb(0x9399b0),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x202746),
                border: rgb(0xf5f7ff),
                foreground: rgb(0xd7daea),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdde7f6),
                border: rgb(0xc2d5ef),
                foreground: rgb(0x3f8fd0),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc2d5ef),
                border: rgb(0xc2d5ef),
                foreground: rgb(0x3f8fd0),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb4cceb),
                border: rgb(0xc2d5ef),
                foreground: rgb(0x3f8fd0),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xa0bfe6),
                border: rgb(0x87b2e0),
                foreground: rgb(0x6090f),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdde7f6),
                border: rgb(0xd0def2),
                foreground: rgb(0x67a0d8),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x6090f),
                border: rgb(0xffffff),
                foreground: rgb(0xa6c3e7),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf1e9d6),
                border: rgb(0xe4d8b7),
                foreground: rgb(0xac973a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe4d8b7),
                border: rgb(0xe4d8b7),
                foreground: rgb(0xac973a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xdecfa7),
                border: rgb(0xe4d8b7),
                foreground: rgb(0xac973a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd5c491),
                border: rgb(0xcab778),
                foreground: rgb(0xb0904),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf1e9d6),
                border: rgb(0xeae0c7),
                foreground: rgb(0xbba759),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xb0904),
                border: rgb(0xffffff),
                foreground: rgb(0xd8c897),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf6e6d4),
                border: rgb(0xeed4b3),
                foreground: rgb(0xc08b31),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xeed4b3),
                border: rgb(0xeed4b3),
                foreground: rgb(0xc08b31),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe9caa3),
                border: rgb(0xeed4b3),
                foreground: rgb(0xc08b31),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xe2bd8c),
                border: rgb(0xd9af72),
                foreground: rgb(0x1b0804),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf6e6d4),
                border: rgb(0xf2ddc4),
                foreground: rgb(0xcd9d52),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x1b0804),
                border: rgb(0xffffff),
                foreground: rgb(0xe4c192),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfcdad0),
                border: rgb(0xf6beab),
                foreground: rgb(0xc94a23),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf6beab),
                border: rgb(0xf6beab),
                foreground: rgb(0xc94a23),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xf2b099),
                border: rgb(0xf6beab),
                foreground: rgb(0xc94a23),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xec9c81),
                border: rgb(0xe48565),
                foreground: rgb(0x260503),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfcdad0),
                border: rgb(0xf9ccbe),
                foreground: rgb(0xd76844),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x260503),
                border: rgb(0xffffff),
                foreground: rgb(0xeea187),
                secondary_foreground: None,
            },
        },
    }
}
