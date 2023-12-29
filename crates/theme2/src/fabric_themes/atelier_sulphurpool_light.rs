use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_sulphurpool_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Sulphurpool Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe5e8f5ff),
                border: rgba(0xccd0e1ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: Some(
                    rgba(0x606889ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xccd0e1ff),
                border: rgba(0xccd0e1ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: Some(
                    rgba(0x606889ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb8bdd0ff),
                border: rgba(0xccd0e1ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: Some(
                    rgba(0x606889ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x9ca1b8ff),
                border: rgba(0x9197aeff),
                foreground: rgba(0x202746ff),
                secondary_foreground: Some(
                    rgba(0x202746ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe5e8f5ff),
                border: rgba(0xdfe2f1ff),
                foreground: rgba(0x898fa5ff),
                secondary_foreground: Some(
                    rgba(0x898fa5ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x202746ff),
                border: rgba(0xf5f7ffff),
                foreground: rgba(0xa4a9bfff),
                secondary_foreground: Some(
                    rgba(0xa4a9bfff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe5e8f5ff),
                border: rgba(0xccd0e1ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xccd0e1ff),
                border: rgba(0xccd0e1ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb8bdd0ff),
                border: rgba(0xccd0e1ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9ca1b8ff),
                border: rgba(0x9197aeff),
                foreground: rgba(0x202746ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe5e8f5ff),
                border: rgba(0xdfe2f1ff),
                foreground: rgba(0x898fa5ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x202746ff),
                border: rgba(0xf5f7ffff),
                foreground: rgba(0xa4a9bfff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xc2c6d9ff),
                border: rgba(0x9a9fb6ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: Some(
                    rgba(0x606889ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x9a9fb6ff),
                border: rgba(0x9a9fb6ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: Some(
                    rgba(0x606889ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x9399b0ff),
                border: rgba(0x9a9fb6ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: Some(
                    rgba(0x606889ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x8e94aaff),
                border: rgba(0x878ca2ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: Some(
                    rgba(0x202746ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xc2c6d9ff),
                border: rgba(0xaeb3c7ff),
                foreground: rgba(0x767d9aff),
                secondary_foreground: Some(
                    rgba(0x767d9aff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x202746ff),
                border: rgba(0xf5f7ffff),
                foreground: rgba(0x8f95abff),
                secondary_foreground: Some(
                    rgba(0x8f95abff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf5f7ffff),
                border: rgba(0xe9ebf7ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe9ebf7ff),
                border: rgba(0xe9ebf7ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe2e5f3ff),
                border: rgba(0xe9ebf7ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xced2e3ff),
                border: rgba(0xaeb3c7ff),
                foreground: rgba(0x202746ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf5f7ffff),
                border: rgba(0xeff1fbff),
                foreground: rgba(0x9399b0ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x202746ff),
                border: rgba(0xf5f7ffff),
                foreground: rgba(0xd7daeaff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdde7f6ff),
                border: rgba(0xc2d5efff),
                foreground: rgba(0x3f8fd0ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc2d5efff),
                border: rgba(0xc2d5efff),
                foreground: rgba(0x3f8fd0ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb4ccebff),
                border: rgba(0xc2d5efff),
                foreground: rgba(0x3f8fd0ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xa0bfe6ff),
                border: rgba(0x87b2e0ff),
                foreground: rgba(0x06090fff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdde7f6ff),
                border: rgba(0xd0def2ff),
                foreground: rgba(0x67a0d8ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x06090fff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa6c3e7ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf1e9d6ff),
                border: rgba(0xe4d8b7ff),
                foreground: rgba(0xac973aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe4d8b7ff),
                border: rgba(0xe4d8b7ff),
                foreground: rgba(0xac973aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xdecfa7ff),
                border: rgba(0xe4d8b7ff),
                foreground: rgba(0xac973aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd5c491ff),
                border: rgba(0xcab778ff),
                foreground: rgba(0x0b0904ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf1e9d6ff),
                border: rgba(0xeae0c7ff),
                foreground: rgba(0xbba759ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0b0904ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xd8c897ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf6e6d4ff),
                border: rgba(0xeed4b3ff),
                foreground: rgba(0xc08b31ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xeed4b3ff),
                border: rgba(0xeed4b3ff),
                foreground: rgba(0xc08b31ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe9caa3ff),
                border: rgba(0xeed4b3ff),
                foreground: rgba(0xc08b31ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xe2bd8cff),
                border: rgba(0xd9af72ff),
                foreground: rgba(0x1b0804ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf6e6d4ff),
                border: rgba(0xf2ddc4ff),
                foreground: rgba(0xcd9d52ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b0804ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xe4c192ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfcdad0ff),
                border: rgba(0xf6beabff),
                foreground: rgba(0xc94a23ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf6beabff),
                border: rgba(0xf6beabff),
                foreground: rgba(0xc94a23ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xf2b099ff),
                border: rgba(0xf6beabff),
                foreground: rgba(0xc94a23ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xec9c81ff),
                border: rgba(0xe48565ff),
                foreground: rgba(0x260503ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfcdad0ff),
                border: rgba(0xf9ccbeff),
                foreground: rgba(0xd76844ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x260503ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xeea187ff),
                secondary_foreground: None,
            },
        },
    }
}
