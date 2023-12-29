use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_sulphurpool_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Sulphurpool Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x262f51),
                border: rgb(0x363f62),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: Some(
                    rgb(0x959bb2),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x363f62),
                border: rgb(0x363f62),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: Some(
                    rgb(0x959bb2),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x454e70),
                border: rgb(0x363f62),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: Some(
                    rgb(0x959bb2),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x5a6284),
                border: rgb(0x636b8c),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: Some(
                    rgb(0xf5f7ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x262f51),
                border: rgb(0x293256),
                foreground: rgb(0x6b7394),
                secondary_foreground: Some(
                    rgb(0x6b7394),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf5f7ff),
                border: rgb(0x202746),
                foreground: rgb(0x545d7e),
                secondary_foreground: Some(
                    rgb(0x545d7e),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x262f51),
                border: rgb(0x363f62),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x363f62),
                border: rgb(0x363f62),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x454e70),
                border: rgb(0x363f62),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x5a6284),
                border: rgb(0x636b8c),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x262f51),
                border: rgb(0x293256),
                foreground: rgb(0x6b7394),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf5f7ff),
                border: rgb(0x202746),
                foreground: rgb(0x545d7e),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3e4769),
                border: rgb(0x5c6485),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: Some(
                    rgb(0x959bb2),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x5c6485),
                border: rgb(0x5c6485),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: Some(
                    rgb(0x959bb2),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x61698a),
                border: rgb(0x5c6485),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: Some(
                    rgb(0x959bb2),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x666e8f),
                border: rgb(0x6d7596),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: Some(
                    rgb(0xf5f7ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3e4769),
                border: rgb(0x4d5577),
                foreground: rgb(0x7e849e),
                secondary_foreground: Some(
                    rgb(0x7e849e),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf5f7ff),
                border: rgb(0x202746),
                foreground: rgb(0x656d8e),
                secondary_foreground: Some(
                    rgb(0x656d8e),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x202746),
                border: rgb(0x252d4f),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x252d4f),
                border: rgb(0x252d4f),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x283054),
                border: rgb(0x252d4f),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x353e61),
                border: rgb(0x4d5577),
                foreground: rgb(0xf5f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x202746),
                border: rgb(0x232a4b),
                foreground: rgb(0x61698a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf5f7ff),
                border: rgb(0x202746),
                foreground: rgb(0x2f385c),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x161f2b),
                border: rgb(0x203348),
                foreground: rgb(0x3e8fd0),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x203348),
                border: rgb(0x203348),
                foreground: rgb(0x3e8fd0),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x243e58),
                border: rgb(0x203348),
                foreground: rgb(0x3e8fd0),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x294d6e),
                border: rgb(0x305f8a),
                foreground: rgb(0xf8fafd),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x161f2b),
                border: rgb(0x1b2939),
                foreground: rgb(0x3777ac),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf8fafd),
                border: rgb(0x0003),
                foreground: rgb(0x284868),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x252113),
                border: rgb(0x3d351b),
                foreground: rgb(0xac973a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3d351b),
                border: rgb(0x3d351b),
                foreground: rgb(0xac973a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4a411f),
                border: rgb(0x3d351b),
                foreground: rgb(0xac973a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x5c5123),
                border: rgb(0x72642a),
                foreground: rgb(0xfcfbf7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x252113),
                border: rgb(0x312b17),
                foreground: rgb(0x8e7d32),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfcfbf7),
                border: rgb(0x0000),
                foreground: rgb(0x574c22),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x311e11),
                border: rgb(0x4b3218),
                foreground: rgb(0xc08b31),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x4b3218),
                border: rgb(0x4b3218),
                foreground: rgb(0xc08b31),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x583c1b),
                border: rgb(0x4b3218),
                foreground: rgb(0xc08b31),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x6c4b1f),
                border: rgb(0x835c24),
                foreground: rgb(0xfdfaf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x311e11),
                border: rgb(0x3e2815),
                foreground: rgb(0xa1742a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdfaf6),
                border: rgb(0x150000),
                foreground: rgb(0x66461e),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3c120d),
                border: rgb(0x551c13),
                foreground: rgb(0xc94923),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x551c13),
                border: rgb(0x551c13),
                foreground: rgb(0xc94923),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x632215),
                border: rgb(0x551c13),
                foreground: rgb(0xc94923),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x762917),
                border: rgb(0x8d321b),
                foreground: rgb(0xfff8f5),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3c120d),
                border: rgb(0x491710),
                foreground: rgb(0xaa3d1f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff8f5),
                border: rgb(0x210000),
                foreground: rgb(0x712717),
                secondary_foreground: None,
            },
        },
    }
}
