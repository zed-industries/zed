use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn gruvbox_light_hard() -> FabricTheme {
    FabricTheme {
        name: "Gruvbox Light Hard".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xecddb5),
                border: rgb(0xddcca7),
                foreground: rgb(0x282828),
                secondary_foreground: Some(
                    rgb(0x5f5650),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xddcca7),
                border: rgb(0xddcca7),
                foreground: rgb(0x282828),
                secondary_foreground: Some(
                    rgb(0x5f5650),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd5c4a1),
                border: rgb(0xddcca7),
                foreground: rgb(0x282828),
                secondary_foreground: Some(
                    rgb(0x5f5650),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xc9b99a),
                border: rgb(0xbcad92),
                foreground: rgb(0x282828),
                secondary_foreground: Some(
                    rgb(0x282828),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xecddb5),
                border: rgb(0xe5d5ad),
                foreground: rgb(0xad9e87),
                secondary_foreground: Some(
                    rgb(0xad9e87),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x282828),
                border: rgb(0xf9f5d7),
                foreground: rgb(0xcdbd9c),
                secondary_foreground: Some(
                    rgb(0xcdbd9c),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xecddb5),
                border: rgb(0xddcca7),
                foreground: rgb(0x282828),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xddcca7),
                border: rgb(0xddcca7),
                foreground: rgb(0x282828),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd5c4a1),
                border: rgb(0xddcca7),
                foreground: rgb(0x282828),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xc9b99a),
                border: rgb(0xbcad92),
                foreground: rgb(0x282828),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xecddb5),
                border: rgb(0xe5d5ad),
                foreground: rgb(0xad9e87),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x282828),
                border: rgb(0xf9f5d7),
                foreground: rgb(0xcdbd9c),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xd9c8a4),
                border: rgb(0xc9b99a),
                foreground: rgb(0x282828),
                secondary_foreground: Some(
                    rgb(0x5f5650),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc9b99a),
                border: rgb(0xc9b99a),
                foreground: rgb(0x282828),
                secondary_foreground: Some(
                    rgb(0x5f5650),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc0b195),
                border: rgb(0xc9b99a),
                foreground: rgb(0x282828),
                secondary_foreground: Some(
                    rgb(0x5f5650),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xb5a68e),
                border: rgb(0xa99a84),
                foreground: rgb(0x282828),
                secondary_foreground: Some(
                    rgb(0x282828),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xd9c8a4),
                border: rgb(0xd1c09e),
                foreground: rgb(0x8a7c6f),
                secondary_foreground: Some(
                    rgb(0x8a7c6f),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x282828),
                border: rgb(0xf9f5d7),
                foreground: rgb(0xb8a98f),
                secondary_foreground: Some(
                    rgb(0xb8a98f),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf9f5d7),
                border: rgb(0xefe2bc),
                foreground: rgb(0x282828),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xefe2bc),
                border: rgb(0xefe2bc),
                foreground: rgb(0x282828),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe9d9b0),
                border: rgb(0xefe2bc),
                foreground: rgb(0x282828),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xdecda8),
                border: rgb(0xd1c09e),
                foreground: rgb(0x282828),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf9f5d7),
                border: rgb(0xf4ecca),
                foreground: rgb(0xc0b195),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x282828),
                border: rgb(0xf9f5d7),
                foreground: rgb(0xe1d1aa),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xd2dee2),
                border: rgb(0xaec6cd),
                foreground: rgb(0xb6678),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xaec6cd),
                border: rgb(0xaec6cd),
                foreground: rgb(0xb6678),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x9db9c2),
                border: rgb(0xaec6cd),
                foreground: rgb(0xb6678),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x84a8b2),
                border: rgb(0x6794a1),
                foreground: rgb(0x30607),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xd2dee2),
                border: rgb(0xc0d2d7),
                foreground: rgb(0x417d8c),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x30607),
                border: rgb(0xffffff),
                foreground: rgb(0x8badb7),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe5e1ce),
                border: rgb(0xd1cba8),
                foreground: rgb(0x797410),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xd1cba8),
                border: rgb(0xd1cba8),
                foreground: rgb(0x797410),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xc7bf95),
                border: rgb(0xd1cba8),
                foreground: rgb(0x797410),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xb7af7b),
                border: rgb(0xa69e5e),
                foreground: rgb(0x80702),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe5e1ce),
                border: rgb(0xdbd5bb),
                foreground: rgb(0x90893a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x80702),
                border: rgb(0xffffff),
                foreground: rgb(0xbcb482),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf5e2d0),
                border: rgb(0xebccab),
                foreground: rgb(0xb57616),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xebccab),
                border: rgb(0xebccab),
                foreground: rgb(0xb57616),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe6c198),
                border: rgb(0xebccab),
                foreground: rgb(0xb57616),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xddb17f),
                border: rgb(0xd3a063),
                foreground: rgb(0x170702),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf5e2d0),
                border: rgb(0xf0d6bd),
                foreground: rgb(0xc58b3f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x170702),
                border: rgb(0xffffff),
                foreground: rgb(0xe0b686),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf4d1c9),
                border: rgb(0xe8ac9e),
                foreground: rgb(0x9d0408),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe8ac9e),
                border: rgb(0xe8ac9e),
                foreground: rgb(0x9d0408),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe19a8a),
                border: rgb(0xe8ac9e),
                foreground: rgb(0x9d0408),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd6816e),
                border: rgb(0xc86450),
                foreground: rgb(0xd0301),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf4d1c9),
                border: rgb(0xeebfb3),
                foreground: rgb(0xb33d2b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xd0301),
                border: rgb(0xffffff),
                foreground: rgb(0xda8876),
                secondary_foreground: None,
            },
        },
    }
}
