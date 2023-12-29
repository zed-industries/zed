use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_forest_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Forest Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x27211eff),
                border: rgba(0x3b3431ff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: Some(
                    rgba(0xa79f9dff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3b3431ff),
                border: rgba(0x3b3431ff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: Some(
                    rgba(0xa79f9dff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4c4542ff),
                border: rgba(0x3b3431ff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: Some(
                    rgba(0xa79f9dff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x645d5aff),
                border: rgba(0x6e6663ff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: Some(
                    rgba(0xf1efeeff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x27211eff),
                border: rgba(0x2c2421ff),
                foreground: rgba(0x766e6bff),
                secondary_foreground: Some(
                    rgba(0x766e6bff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf1efeeff),
                border: rgba(0x1b1918ff),
                foreground: rgba(0x5d5653ff),
                secondary_foreground: Some(
                    rgba(0x5d5653ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x27211eff),
                border: rgba(0x3b3431ff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3b3431ff),
                border: rgba(0x3b3431ff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4c4542ff),
                border: rgba(0x3b3431ff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x645d5aff),
                border: rgba(0x6e6663ff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x27211eff),
                border: rgba(0x2c2421ff),
                foreground: rgba(0x766e6bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf1efeeff),
                border: rgba(0x1b1918ff),
                foreground: rgba(0x5d5653ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x443c39ff),
                border: rgba(0x665f5cff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: Some(
                    rgba(0xa79f9dff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x665f5cff),
                border: rgba(0x665f5cff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: Some(
                    rgba(0xa79f9dff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x6c6461ff),
                border: rgba(0x665f5cff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: Some(
                    rgba(0xa79f9dff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x716966ff),
                border: rgba(0x79716eff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: Some(
                    rgba(0xf1efeeff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x443c39ff),
                border: rgba(0x554e4bff),
                foreground: rgba(0x8e8683ff),
                secondary_foreground: Some(
                    rgba(0x8e8683ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf1efeeff),
                border: rgba(0x1b1918ff),
                foreground: rgba(0x706865ff),
                secondary_foreground: Some(
                    rgba(0x706865ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1b1918ff),
                border: rgba(0x251f1dff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x251f1dff),
                border: rgba(0x251f1dff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x292220ff),
                border: rgba(0x251f1dff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x3a322fff),
                border: rgba(0x554e4bff),
                foreground: rgba(0xf1efeeff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1b1918ff),
                border: rgba(0x201c1bff),
                foreground: rgba(0x6c6461ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf1efeeff),
                border: rgba(0x1b1918ff),
                foreground: rgba(0x332b28ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x0f1d3dff),
                border: rgba(0x192e5bff),
                foreground: rgba(0x417ee6ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x192e5bff),
                border: rgba(0x192e5bff),
                foreground: rgba(0x417ee6ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1d386cff),
                border: rgba(0x192e5bff),
                foreground: rgba(0x417ee6ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x244583ff),
                border: rgba(0x2c549fff),
                foreground: rgba(0xf9f9feff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x0f1d3dff),
                border: rgba(0x14264cff),
                foreground: rgba(0x3669c2ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf9f9feff),
                border: rgba(0x00001bff),
                foreground: rgba(0x22417cff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1d2110ff),
                border: rgba(0x2e3516ff),
                foreground: rgba(0x7b9727ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2e3516ff),
                border: rgba(0x2e3516ff),
                foreground: rgba(0x7b9727ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x374118ff),
                border: rgba(0x2e3516ff),
                foreground: rgba(0x7b9727ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x44511bff),
                border: rgba(0x53641fff),
                foreground: rgba(0xfafbf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1d2110ff),
                border: rgba(0x262b13ff),
                foreground: rgba(0x677d23ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfafbf6ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x404c1bff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x371d0dff),
                border: rgba(0x4f2f12ff),
                foreground: rgba(0xc38419ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x4f2f12ff),
                border: rgba(0x4f2f12ff),
                foreground: rgba(0xc38419ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x5d3914ff),
                border: rgba(0x4f2f12ff),
                foreground: rgba(0xc38419ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x704716ff),
                border: rgba(0x875817ff),
                foreground: rgba(0xfefaf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x371d0dff),
                border: rgba(0x432611ff),
                foreground: rgba(0xa46e18ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfefaf6ff),
                border: rgba(0x1b0000ff),
                foreground: rgba(0x6b4315ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x550512ff),
                border: rgba(0x710c1bff),
                foreground: rgba(0xf22d40ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x710c1bff),
                border: rgba(0x710c1bff),
                foreground: rgba(0xf22d40ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x810f20ff),
                border: rgba(0x710c1bff),
                foreground: rgba(0xf22d40ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x961425ff),
                border: rgba(0xb01b2dff),
                foreground: rgba(0xfff8f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x550512ff),
                border: rgba(0x630817ff),
                foreground: rgba(0xd02437ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff8f7ff),
                border: rgba(0x340000ff),
                foreground: rgba(0x901324ff),
                secondary_foreground: None,
            },
        },
    }
}
