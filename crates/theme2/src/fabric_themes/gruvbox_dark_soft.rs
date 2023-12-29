use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn gruvbox_dark_soft() -> FabricTheme {
    FabricTheme {
        name: "Gruvbox Dark Soft",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3b3735ff),
                border: rgba(0x494340ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: Some(
                    rgba(0xc5b597ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x494340ff),
                border: rgba(0x494340ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: Some(
                    rgba(0xc5b597ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x504945ff),
                border: rgba(0x494340ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: Some(
                    rgba(0xc5b597ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x5b524cff),
                border: rgba(0x675d55ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: Some(
                    rgba(0xfbf1c7ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3b3735ff),
                border: rgba(0x413d3aff),
                foreground: rgba(0x776b61ff),
                secondary_foreground: Some(
                    rgba(0x776b61ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbf1c7ff),
                border: rgba(0x32302fff),
                foreground: rgba(0x574f4aff),
                secondary_foreground: Some(
                    rgba(0x574f4aff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3b3735ff),
                border: rgba(0x494340ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x494340ff),
                border: rgba(0x494340ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x504945ff),
                border: rgba(0x494340ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x5b524cff),
                border: rgba(0x675d55ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3b3735ff),
                border: rgba(0x413d3aff),
                foreground: rgba(0x776b61ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbf1c7ff),
                border: rgba(0x32302fff),
                foreground: rgba(0x574f4aff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x4c4642ff),
                border: rgba(0x5b534dff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: Some(
                    rgba(0xc5b597ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x5b534dff),
                border: rgba(0x5b534dff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: Some(
                    rgba(0xc5b597ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x635a52ff),
                border: rgba(0x5b534dff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: Some(
                    rgba(0xc5b597ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x6e635aff),
                border: rgba(0x7b6e64ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: Some(
                    rgba(0xfbf1c7ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x4c4642ff),
                border: rgba(0x544c48ff),
                foreground: rgba(0x9a8c79ff),
                secondary_foreground: Some(
                    rgba(0x9a8c79ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbf1c7ff),
                border: rgba(0x32302fff),
                foreground: rgba(0x6b6058ff),
                secondary_foreground: Some(
                    rgba(0x6b6058ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x32302fff),
                border: rgba(0x393634ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x393634ff),
                border: rgba(0x393634ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3e3a37ff),
                border: rgba(0x393634ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x48423fff),
                border: rgba(0x544c48ff),
                foreground: rgba(0xfbf1c7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x32302fff),
                border: rgba(0x363332ff),
                foreground: rgba(0x635a52ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbf1c7ff),
                border: rgba(0x32302fff),
                foreground: rgba(0x45403dff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1e2321ff),
                border: rgba(0x303a36ff),
                foreground: rgba(0x83a598ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x303a36ff),
                border: rgba(0x303a36ff),
                foreground: rgba(0x83a598ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3a4641ff),
                border: rgba(0x303a36ff),
                foreground: rgba(0x83a598ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x485852ff),
                border: rgba(0x586d65ff),
                foreground: rgba(0xfafbfbff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1e2321ff),
                border: rgba(0x272f2cff),
                foreground: rgba(0x6d887eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfafbfbff),
                border: rgba(0x000000ff),
                foreground: rgba(0x43534dff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x332b11ff),
                border: rgba(0x4a4516ff),
                foreground: rgba(0xb8bb27ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x4a4516ff),
                border: rgba(0x4a4516ff),
                foreground: rgba(0xb8bb27ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x575219ff),
                border: rgba(0x4a4516ff),
                foreground: rgba(0xb8bb27ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x6a661cff),
                border: rgba(0x807e20ff),
                foreground: rgba(0xfdfcf7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x332b11ff),
                border: rgba(0x3f3814ff),
                foreground: rgba(0x9b9b23ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdfcf7ff),
                border: rgba(0x180b00ff),
                foreground: rgba(0x64601bff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x582f10ff),
                border: rgba(0x754916ff),
                foreground: rgba(0xf9bd30ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x754916ff),
                border: rgba(0x754916ff),
                foreground: rgba(0xf9bd30ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x85561aff),
                border: rgba(0x754916ff),
                foreground: rgba(0xf9bd30ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9b6a1eff),
                border: rgba(0xb68123ff),
                foreground: rgba(0xfffcf7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x582f10ff),
                border: rgba(0x663c13ff),
                foreground: rgba(0xd79e29ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfffcf7ff),
                border: rgba(0x351100ff),
                foreground: rgba(0x95641dff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x5a0a10ff),
                border: rgba(0x771618ff),
                foreground: rgba(0xfb4a35ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x771618ff),
                border: rgba(0x771618ff),
                foreground: rgba(0xfb4a35ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x871c1bff),
                border: rgba(0x771618ff),
                foreground: rgba(0xfb4a35ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9d251fff),
                border: rgba(0xb72f26ff),
                foreground: rgba(0xfff8f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x5a0a10ff),
                border: rgba(0x681014ff),
                foreground: rgba(0xd83c2dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff8f6ff),
                border: rgba(0x380000ff),
                foreground: rgba(0x97221fff),
                secondary_foreground: None,
            },
        },
    }
}
