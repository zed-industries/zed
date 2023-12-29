use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_seaside_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Seaside Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1f231f),
                border: rgb(0x333b33),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: Some(
                    rgb(0x8ba48b),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x333b33),
                border: rgb(0x333b33),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: Some(
                    rgb(0x8ba48b),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x434e43),
                border: rgb(0x333b33),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: Some(
                    rgb(0x8ba48b),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x5a6a5a),
                border: rgb(0x627462),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: Some(
                    rgb(0xf4fbf4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1f231f),
                border: rgb(0x242924),
                foreground: rgb(0x687c68),
                secondary_foreground: Some(
                    rgb(0x687c68),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4fbf4),
                border: rgb(0x131513),
                foreground: rgb(0x546254),
                secondary_foreground: Some(
                    rgb(0x546254),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1f231f),
                border: rgb(0x333b33),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x333b33),
                border: rgb(0x333b33),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x434e43),
                border: rgb(0x333b33),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x5a6a5a),
                border: rgb(0x627462),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1f231f),
                border: rgb(0x242924),
                foreground: rgb(0x687c68),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4fbf4),
                border: rgb(0x131513),
                foreground: rgb(0x546254),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x3b453b),
                border: rgb(0x5c6c5c),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: Some(
                    rgb(0x8ba48b),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x5c6c5c),
                border: rgb(0x5c6c5c),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: Some(
                    rgb(0x8ba48b),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x617261),
                border: rgb(0x5c6c5c),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: Some(
                    rgb(0x8ba48b),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x647764),
                border: rgb(0x6a7f6a),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: Some(
                    rgb(0xf4fbf4),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x3b453b),
                border: rgb(0x4b584b),
                foreground: rgb(0x778f77),
                secondary_foreground: Some(
                    rgb(0x778f77),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4fbf4),
                border: rgb(0x131513),
                foreground: rgb(0x637663),
                secondary_foreground: Some(
                    rgb(0x637663),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x131513),
                border: rgb(0x1d201d),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1d201d),
                border: rgb(0x1d201d),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x212621),
                border: rgb(0x1d201d),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x323932),
                border: rgb(0x4b584b),
                foreground: rgb(0xf4fbf4),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x131513),
                border: rgb(0x181b18),
                foreground: rgb(0x617261),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf4fbf4),
                border: rgb(0x131513),
                foreground: rgb(0x2b312b),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x61949),
                border: rgb(0x102668),
                foreground: rgb(0x3e62f4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x102668),
                border: rgb(0x102668),
                foreground: rgb(0x3e62f4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x152d78),
                border: rgb(0x102668),
                foreground: rgb(0x3e62f4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x1c3790),
                border: rgb(0x2543ac),
                foreground: rgb(0xf9f8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x61949),
                border: rgb(0xb1f58),
                foreground: rgb(0x3152d0),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf9f8ff),
                border: rgb(0x0025),
                foreground: rgb(0x1a3489),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x142310),
                border: rgb(0x1b3917),
                foreground: rgb(0x2ba32a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x1b3917),
                border: rgb(0x1b3917),
                foreground: rgb(0x2ba32a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x1e4619),
                border: rgb(0x1b3917),
                foreground: rgb(0x2ba32a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x21571d),
                border: rgb(0x256c21),
                foreground: rgb(0xf7fbf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x142310),
                border: rgb(0x172e14),
                foreground: rgb(0x288725),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf7fbf6),
                border: rgb(0x0000),
                foreground: rgb(0x20521c),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x22210f),
                border: rgb(0x373614),
                foreground: rgb(0x98981c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x373614),
                border: rgb(0x373614),
                foreground: rgb(0x98981c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x424116),
                border: rgb(0x373614),
                foreground: rgb(0x98981c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x535218),
                border: rgb(0x66651a),
                foreground: rgb(0xfbfbf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x22210f),
                border: rgb(0x2d2b12),
                foreground: rgb(0x7f7e1b),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfbfbf6),
                border: rgb(0x0000),
                foreground: rgb(0x4e4d17),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x500412),
                border: rgb(0x6b071a),
                foreground: rgb(0xe61c3c),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x6b071a),
                border: rgb(0x6b071a),
                foreground: rgb(0xe61c3c),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x79091e),
                border: rgb(0x6b071a),
                foreground: rgb(0xe61c3c),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x8e0c23),
                border: rgb(0xa7102b),
                foreground: rgb(0xfff7f6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x500412),
                border: rgb(0x5d0616),
                foreground: rgb(0xc51533),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff7f6),
                border: rgb(0x300000),
                foreground: rgb(0x880b22),
                secondary_foreground: None,
            },
        },
    }
}
