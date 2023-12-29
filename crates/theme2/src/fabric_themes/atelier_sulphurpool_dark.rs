use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_sulphurpool_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Sulphurpool Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x262f51ff),
                border: rgba(0x363f62ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: Some(
                    rgba(0x959bb2ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x363f62ff),
                border: rgba(0x363f62ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: Some(
                    rgba(0x959bb2ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x454e70ff),
                border: rgba(0x363f62ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: Some(
                    rgba(0x959bb2ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x5a6284ff),
                border: rgba(0x636b8cff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: Some(
                    rgba(0xf5f7ffff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x262f51ff),
                border: rgba(0x293256ff),
                foreground: rgba(0x6b7394ff),
                secondary_foreground: Some(
                    rgba(0x6b7394ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf5f7ffff),
                border: rgba(0x202746ff),
                foreground: rgba(0x545d7eff),
                secondary_foreground: Some(
                    rgba(0x545d7eff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x262f51ff),
                border: rgba(0x363f62ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x363f62ff),
                border: rgba(0x363f62ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x454e70ff),
                border: rgba(0x363f62ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x5a6284ff),
                border: rgba(0x636b8cff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x262f51ff),
                border: rgba(0x293256ff),
                foreground: rgba(0x6b7394ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf5f7ffff),
                border: rgba(0x202746ff),
                foreground: rgba(0x545d7eff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3e4769ff),
                border: rgba(0x5c6485ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: Some(
                    rgba(0x959bb2ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x5c6485ff),
                border: rgba(0x5c6485ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: Some(
                    rgba(0x959bb2ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x61698aff),
                border: rgba(0x5c6485ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: Some(
                    rgba(0x959bb2ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x666e8fff),
                border: rgba(0x6d7596ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: Some(
                    rgba(0xf5f7ffff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3e4769ff),
                border: rgba(0x4d5577ff),
                foreground: rgba(0x7e849eff),
                secondary_foreground: Some(
                    rgba(0x7e849eff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf5f7ffff),
                border: rgba(0x202746ff),
                foreground: rgba(0x656d8eff),
                secondary_foreground: Some(
                    rgba(0x656d8eff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x202746ff),
                border: rgba(0x252d4fff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x252d4fff),
                border: rgba(0x252d4fff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x283054ff),
                border: rgba(0x252d4fff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x353e61ff),
                border: rgba(0x4d5577ff),
                foreground: rgba(0xf5f7ffff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x202746ff),
                border: rgba(0x232a4bff),
                foreground: rgba(0x61698aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf5f7ffff),
                border: rgba(0x202746ff),
                foreground: rgba(0x2f385cff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x161f2bff),
                border: rgba(0x203348ff),
                foreground: rgba(0x3e8fd0ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x203348ff),
                border: rgba(0x203348ff),
                foreground: rgba(0x3e8fd0ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x243e58ff),
                border: rgba(0x203348ff),
                foreground: rgba(0x3e8fd0ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x294d6eff),
                border: rgba(0x305f8aff),
                foreground: rgba(0xf8fafdff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x161f2bff),
                border: rgba(0x1b2939ff),
                foreground: rgba(0x3777acff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8fafdff),
                border: rgba(0x000003ff),
                foreground: rgba(0x284868ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x252113ff),
                border: rgba(0x3d351bff),
                foreground: rgba(0xac973aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3d351bff),
                border: rgba(0x3d351bff),
                foreground: rgba(0xac973aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4a411fff),
                border: rgba(0x3d351bff),
                foreground: rgba(0xac973aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x5c5123ff),
                border: rgba(0x72642aff),
                foreground: rgba(0xfcfbf7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x252113ff),
                border: rgba(0x312b17ff),
                foreground: rgba(0x8e7d32ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfcfbf7ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x574c22ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x311e11ff),
                border: rgba(0x4b3218ff),
                foreground: rgba(0xc08b31ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x4b3218ff),
                border: rgba(0x4b3218ff),
                foreground: rgba(0xc08b31ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x583c1bff),
                border: rgba(0x4b3218ff),
                foreground: rgba(0xc08b31ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x6c4b1fff),
                border: rgba(0x835c24ff),
                foreground: rgba(0xfdfaf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x311e11ff),
                border: rgba(0x3e2815ff),
                foreground: rgba(0xa1742aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdfaf6ff),
                border: rgba(0x150000ff),
                foreground: rgba(0x66461eff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3c120dff),
                border: rgba(0x551c13ff),
                foreground: rgba(0xc94923ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x551c13ff),
                border: rgba(0x551c13ff),
                foreground: rgba(0xc94923ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x632215ff),
                border: rgba(0x551c13ff),
                foreground: rgba(0xc94923ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x762917ff),
                border: rgba(0x8d321bff),
                foreground: rgba(0xfff8f5ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3c120dff),
                border: rgba(0x491710ff),
                foreground: rgba(0xaa3d1fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff8f5ff),
                border: rgba(0x210000ff),
                foreground: rgba(0x712717ff),
                secondary_foreground: None,
            },
        },
    }
}
