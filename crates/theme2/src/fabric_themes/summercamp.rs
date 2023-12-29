use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn summercamp() -> FabricTheme {
    FabricTheme {
        name: "Summercamp",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x231f16ff),
                border: rgba(0x29251bff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: Some(
                    rgba(0x736e55ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x29251bff),
                border: rgba(0x29251bff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: Some(
                    rgba(0x736e55ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2c281dff),
                border: rgba(0x29251bff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: Some(
                    rgba(0x736e55ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x302c20ff),
                border: rgba(0x373225ff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: Some(
                    rgba(0xf8f5deff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x231f16ff),
                border: rgba(0x262218ff),
                foreground: rgba(0x3d382aff),
                secondary_foreground: Some(
                    rgba(0x3d382aff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8f5deff),
                border: rgba(0x1c1810ff),
                foreground: rgba(0x302b20ff),
                secondary_foreground: Some(
                    rgba(0x302b20ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x231f16ff),
                border: rgba(0x29251bff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x29251bff),
                border: rgba(0x29251bff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2c281dff),
                border: rgba(0x29251bff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x302c20ff),
                border: rgba(0x373225ff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x231f16ff),
                border: rgba(0x262218ff),
                foreground: rgba(0x3d382aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8f5deff),
                border: rgba(0x1c1810ff),
                foreground: rgba(0x302b20ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2a261cff),
                border: rgba(0x312d21ff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: Some(
                    rgba(0x736e55ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x312d21ff),
                border: rgba(0x312d21ff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: Some(
                    rgba(0x736e55ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x353024ff),
                border: rgba(0x312d21ff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: Some(
                    rgba(0x736e55ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x393427ff),
                border: rgba(0x403b2cff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: Some(
                    rgba(0xf8f5deff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2a261cff),
                border: rgba(0x2e2a1fff),
                foreground: rgba(0x4c4735ff),
                secondary_foreground: Some(
                    rgba(0x4c4735ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8f5deff),
                border: rgba(0x1c1810ff),
                foreground: rgba(0x393426ff),
                secondary_foreground: Some(
                    rgba(0x393426ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1c1810ff),
                border: rgba(0x221e15ff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x221e15ff),
                border: rgba(0x221e15ff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x242017ff),
                border: rgba(0x221e15ff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x28241aff),
                border: rgba(0x2e2a1fff),
                foreground: rgba(0xf8f5deff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1c1810ff),
                border: rgba(0x1f1b12ff),
                foreground: rgba(0x353024ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8f5deff),
                border: rgba(0x1c1810ff),
                foreground: rgba(0x27231aff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x0e2242ff),
                border: rgba(0x193761ff),
                foreground: rgba(0x499befff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x193761ff),
                border: rgba(0x193761ff),
                foreground: rgba(0x499befff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1e4272ff),
                border: rgba(0x193761ff),
                foreground: rgba(0x499befff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x26538aff),
                border: rgba(0x2f67a6ff),
                foreground: rgba(0xf9fbffff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x0e2242ff),
                border: rgba(0x132d51ff),
                foreground: rgba(0x3c81caff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf9fbffff),
                border: rgba(0x00001eff),
                foreground: rgba(0x244e83ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x0a4d13ff),
                border: rgba(0x1a6a20ff),
                foreground: rgba(0x5dea5aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1a6a20ff),
                border: rgba(0x1a6a20ff),
                foreground: rgba(0x5dea5aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x227927ff),
                border: rgba(0x1a6a20ff),
                foreground: rgba(0x5dea5aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2d8e31ff),
                border: rgba(0x3aa83cff),
                foreground: rgba(0xfafef8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x0a4d13ff),
                border: rgba(0x125b1aff),
                foreground: rgba(0x4bc94bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfafef8ff),
                border: rgba(0x002b00ff),
                foreground: rgba(0x29882dff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x556305ff),
                border: rgba(0x727f0aff),
                foreground: rgba(0xf1fe29ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x727f0aff),
                border: rgba(0x727f0aff),
                foreground: rgba(0xf1fe29ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x818e0eff),
                border: rgba(0x727f0aff),
                foreground: rgba(0xf1fe29ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x96a412ff),
                border: rgba(0xb0be18ff),
                foreground: rgba(0xfffff8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x556305ff),
                border: rgba(0x637107ff),
                foreground: rgba(0xd0dd20ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfffff8ff),
                border: rgba(0x334000ff),
                foreground: rgba(0x909e11ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x491013ff),
                border: rgba(0x651c1cff),
                foreground: rgba(0xe35142ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x651c1cff),
                border: rgba(0x651c1cff),
                foreground: rgba(0xe35142ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x742221ff),
                border: rgba(0x651c1cff),
                foreground: rgba(0xe35142ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x892b27ff),
                border: rgba(0xa2352fff),
                foreground: rgba(0xfff8f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x491013ff),
                border: rgba(0x571618ff),
                foreground: rgba(0xc24338ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff8f7ff),
                border: rgba(0x2a0000ff),
                foreground: rgba(0x832825ff),
                secondary_foreground: None,
            },
        },
    }
}
