use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_seaside_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Seaside Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1f231fff),
                border: rgba(0x333b33ff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: Some(
                    rgba(0x8ba48bff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x333b33ff),
                border: rgba(0x333b33ff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: Some(
                    rgba(0x8ba48bff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x434e43ff),
                border: rgba(0x333b33ff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: Some(
                    rgba(0x8ba48bff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x5a6a5aff),
                border: rgba(0x627462ff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: Some(
                    rgba(0xf4fbf4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1f231fff),
                border: rgba(0x242924ff),
                foreground: rgba(0x687c68ff),
                secondary_foreground: Some(
                    rgba(0x687c68ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4fbf4ff),
                border: rgba(0x131513ff),
                foreground: rgba(0x546254ff),
                secondary_foreground: Some(
                    rgba(0x546254ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1f231fff),
                border: rgba(0x333b33ff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x333b33ff),
                border: rgba(0x333b33ff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x434e43ff),
                border: rgba(0x333b33ff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x5a6a5aff),
                border: rgba(0x627462ff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1f231fff),
                border: rgba(0x242924ff),
                foreground: rgba(0x687c68ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4fbf4ff),
                border: rgba(0x131513ff),
                foreground: rgba(0x546254ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3b453bff),
                border: rgba(0x5c6c5cff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: Some(
                    rgba(0x8ba48bff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x5c6c5cff),
                border: rgba(0x5c6c5cff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: Some(
                    rgba(0x8ba48bff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x617261ff),
                border: rgba(0x5c6c5cff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: Some(
                    rgba(0x8ba48bff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x647764ff),
                border: rgba(0x6a7f6aff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: Some(
                    rgba(0xf4fbf4ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3b453bff),
                border: rgba(0x4b584bff),
                foreground: rgba(0x778f77ff),
                secondary_foreground: Some(
                    rgba(0x778f77ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4fbf4ff),
                border: rgba(0x131513ff),
                foreground: rgba(0x637663ff),
                secondary_foreground: Some(
                    rgba(0x637663ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x131513ff),
                border: rgba(0x1d201dff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1d201dff),
                border: rgba(0x1d201dff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x212621ff),
                border: rgba(0x1d201dff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x323932ff),
                border: rgba(0x4b584bff),
                foreground: rgba(0xf4fbf4ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x131513ff),
                border: rgba(0x181b18ff),
                foreground: rgba(0x617261ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4fbf4ff),
                border: rgba(0x131513ff),
                foreground: rgba(0x2b312bff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x061949ff),
                border: rgba(0x102668ff),
                foreground: rgba(0x3e62f4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x102668ff),
                border: rgba(0x102668ff),
                foreground: rgba(0x3e62f4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x152d78ff),
                border: rgba(0x102668ff),
                foreground: rgba(0x3e62f4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x1c3790ff),
                border: rgba(0x2543acff),
                foreground: rgba(0xf9f8ffff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x061949ff),
                border: rgba(0x0b1f58ff),
                foreground: rgba(0x3152d0ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf9f8ffff),
                border: rgba(0x000025ff),
                foreground: rgba(0x1a3489ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x142310ff),
                border: rgba(0x1b3917ff),
                foreground: rgba(0x2ba32aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1b3917ff),
                border: rgba(0x1b3917ff),
                foreground: rgba(0x2ba32aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x1e4619ff),
                border: rgba(0x1b3917ff),
                foreground: rgba(0x2ba32aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x21571dff),
                border: rgba(0x256c21ff),
                foreground: rgba(0xf7fbf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x142310ff),
                border: rgba(0x172e14ff),
                foreground: rgba(0x288725ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf7fbf6ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x20521cff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x22210fff),
                border: rgba(0x373614ff),
                foreground: rgba(0x98981cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x373614ff),
                border: rgba(0x373614ff),
                foreground: rgba(0x98981cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x424116ff),
                border: rgba(0x373614ff),
                foreground: rgba(0x98981cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x535218ff),
                border: rgba(0x66651aff),
                foreground: rgba(0xfbfbf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x22210fff),
                border: rgba(0x2d2b12ff),
                foreground: rgba(0x7f7e1bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfbfbf6ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x4e4d17ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x500412ff),
                border: rgba(0x6b071aff),
                foreground: rgba(0xe61c3cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x6b071aff),
                border: rgba(0x6b071aff),
                foreground: rgba(0xe61c3cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x79091eff),
                border: rgba(0x6b071aff),
                foreground: rgba(0xe61c3cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x8e0c23ff),
                border: rgba(0xa7102bff),
                foreground: rgba(0xfff7f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x500412ff),
                border: rgba(0x5d0616ff),
                foreground: rgba(0xc51533ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff7f6ff),
                border: rgba(0x300000ff),
                foreground: rgba(0x880b22ff),
                secondary_foreground: None,
            },
        },
    }
}
