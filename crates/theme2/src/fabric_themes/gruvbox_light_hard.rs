use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn gruvbox_light_hard() -> FabricTheme {
    FabricTheme {
        name: "Gruvbox Light Hard",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xecddb5ff),
                border: rgba(0xddcca7ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: Some(
                    rgba(0x5f5650ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xddcca7ff),
                border: rgba(0xddcca7ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: Some(
                    rgba(0x5f5650ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd5c4a1ff),
                border: rgba(0xddcca7ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: Some(
                    rgba(0x5f5650ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xc9b99aff),
                border: rgba(0xbcad92ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: Some(
                    rgba(0x282828ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xecddb5ff),
                border: rgba(0xe5d5adff),
                foreground: rgba(0xad9e87ff),
                secondary_foreground: Some(
                    rgba(0xad9e87ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x282828ff),
                border: rgba(0xf9f5d7ff),
                foreground: rgba(0xcdbd9cff),
                secondary_foreground: Some(
                    rgba(0xcdbd9cff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xecddb5ff),
                border: rgba(0xddcca7ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xddcca7ff),
                border: rgba(0xddcca7ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd5c4a1ff),
                border: rgba(0xddcca7ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xc9b99aff),
                border: rgba(0xbcad92ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xecddb5ff),
                border: rgba(0xe5d5adff),
                foreground: rgba(0xad9e87ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x282828ff),
                border: rgba(0xf9f5d7ff),
                foreground: rgba(0xcdbd9cff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xd9c8a4ff),
                border: rgba(0xc9b99aff),
                foreground: rgba(0x282828ff),
                secondary_foreground: Some(
                    rgba(0x5f5650ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc9b99aff),
                border: rgba(0xc9b99aff),
                foreground: rgba(0x282828ff),
                secondary_foreground: Some(
                    rgba(0x5f5650ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc0b195ff),
                border: rgba(0xc9b99aff),
                foreground: rgba(0x282828ff),
                secondary_foreground: Some(
                    rgba(0x5f5650ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xb5a68eff),
                border: rgba(0xa99a84ff),
                foreground: rgba(0x282828ff),
                secondary_foreground: Some(
                    rgba(0x282828ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xd9c8a4ff),
                border: rgba(0xd1c09eff),
                foreground: rgba(0x8a7c6fff),
                secondary_foreground: Some(
                    rgba(0x8a7c6fff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x282828ff),
                border: rgba(0xf9f5d7ff),
                foreground: rgba(0xb8a98fff),
                secondary_foreground: Some(
                    rgba(0xb8a98fff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf9f5d7ff),
                border: rgba(0xefe2bcff),
                foreground: rgba(0x282828ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xefe2bcff),
                border: rgba(0xefe2bcff),
                foreground: rgba(0x282828ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe9d9b0ff),
                border: rgba(0xefe2bcff),
                foreground: rgba(0x282828ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xdecda8ff),
                border: rgba(0xd1c09eff),
                foreground: rgba(0x282828ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf9f5d7ff),
                border: rgba(0xf4eccaff),
                foreground: rgba(0xc0b195ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x282828ff),
                border: rgba(0xf9f5d7ff),
                foreground: rgba(0xe1d1aaff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xd2dee2ff),
                border: rgba(0xaec6cdff),
                foreground: rgba(0x0b6678ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xaec6cdff),
                border: rgba(0xaec6cdff),
                foreground: rgba(0x0b6678ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x9db9c2ff),
                border: rgba(0xaec6cdff),
                foreground: rgba(0x0b6678ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x84a8b2ff),
                border: rgba(0x6794a1ff),
                foreground: rgba(0x030607ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xd2dee2ff),
                border: rgba(0xc0d2d7ff),
                foreground: rgba(0x417d8cff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x030607ff),
                border: rgba(0xffffffff),
                foreground: rgba(0x8badb7ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe5e1ceff),
                border: rgba(0xd1cba8ff),
                foreground: rgba(0x797410ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd1cba8ff),
                border: rgba(0xd1cba8ff),
                foreground: rgba(0x797410ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc7bf95ff),
                border: rgba(0xd1cba8ff),
                foreground: rgba(0x797410ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xb7af7bff),
                border: rgba(0xa69e5eff),
                foreground: rgba(0x080702ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe5e1ceff),
                border: rgba(0xdbd5bbff),
                foreground: rgba(0x90893aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x080702ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xbcb482ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf5e2d0ff),
                border: rgba(0xebccabff),
                foreground: rgba(0xb57616ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xebccabff),
                border: rgba(0xebccabff),
                foreground: rgba(0xb57616ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe6c198ff),
                border: rgba(0xebccabff),
                foreground: rgba(0xb57616ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xddb17fff),
                border: rgba(0xd3a063ff),
                foreground: rgba(0x170702ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf5e2d0ff),
                border: rgba(0xf0d6bdff),
                foreground: rgba(0xc58b3fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x170702ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xe0b686ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf4d1c9ff),
                border: rgba(0xe8ac9eff),
                foreground: rgba(0x9d0408ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe8ac9eff),
                border: rgba(0xe8ac9eff),
                foreground: rgba(0x9d0408ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe19a8aff),
                border: rgba(0xe8ac9eff),
                foreground: rgba(0x9d0408ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd6816eff),
                border: rgba(0xc86450ff),
                foreground: rgba(0x0d0301ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf4d1c9ff),
                border: rgba(0xeebfb3ff),
                foreground: rgba(0xb33d2bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0d0301ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xda8876ff),
                secondary_foreground: None,
            },
        },
    }
}
