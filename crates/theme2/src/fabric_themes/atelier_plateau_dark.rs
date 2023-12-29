use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_plateau_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Plateau Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x252020ff),
                border: rgba(0x352f2fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: Some(
                    rgba(0x898383ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x352f2fff),
                border: rgba(0x352f2fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: Some(
                    rgba(0x898383ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x423b3bff),
                border: rgba(0x352f2fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: Some(
                    rgba(0x898383ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x554d4dff),
                border: rgba(0x5d5555ff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: Some(
                    rgba(0xf4ececff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x252020ff),
                border: rgba(0x292424ff),
                foreground: rgba(0x655d5dff),
                secondary_foreground: Some(
                    rgba(0x655d5dff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4ececff),
                border: rgba(0x1b1818ff),
                foreground: rgba(0x4f4848ff),
                secondary_foreground: Some(
                    rgba(0x4f4848ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x252020ff),
                border: rgba(0x352f2fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x352f2fff),
                border: rgba(0x352f2fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x423b3bff),
                border: rgba(0x352f2fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x554d4dff),
                border: rgba(0x5d5555ff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x252020ff),
                border: rgba(0x292424ff),
                foreground: rgba(0x655d5dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4ececff),
                border: rgba(0x1b1818ff),
                foreground: rgba(0x4f4848ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x3b3535ff),
                border: rgba(0x564e4eff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: Some(
                    rgba(0x898383ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x564e4eff),
                border: rgba(0x564e4eff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: Some(
                    rgba(0x898383ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x5b5353ff),
                border: rgba(0x564e4eff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: Some(
                    rgba(0x898383ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x605858ff),
                border: rgba(0x675f5fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: Some(
                    rgba(0xf4ececff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x3b3535ff),
                border: rgba(0x494242ff),
                foreground: rgba(0x756e6eff),
                secondary_foreground: Some(
                    rgba(0x756e6eff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4ececff),
                border: rgba(0x1b1818ff),
                foreground: rgba(0x5f5757ff),
                secondary_foreground: Some(
                    rgba(0x5f5757ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1b1818ff),
                border: rgba(0x231f1fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x231f1fff),
                border: rgba(0x231f1fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x272222ff),
                border: rgba(0x231f1fff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x342e2eff),
                border: rgba(0x494242ff),
                foreground: rgba(0xf4ececff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1b1818ff),
                border: rgba(0x1f1b1bff),
                foreground: rgba(0x5b5353ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf4ececff),
                border: rgba(0x1b1818ff),
                foreground: rgba(0x2f2a2aff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1c1b29ff),
                border: rgba(0x2c2b45ff),
                foreground: rgba(0x7272caff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2c2b45ff),
                border: rgba(0x2c2b45ff),
                foreground: rgba(0x7272caff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x353354ff),
                border: rgba(0x2c2b45ff),
                foreground: rgba(0x7272caff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x403f6aff),
                border: rgba(0x4e4d85ff),
                foreground: rgba(0xfaf9fdff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1c1b29ff),
                border: rgba(0x242336ff),
                foreground: rgba(0x605fa6ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfaf9fdff),
                border: rgba(0x000000ff),
                foreground: rgba(0x3c3b64ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x161f1fff),
                border: rgba(0x203232ff),
                foreground: rgba(0x4b8b8bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x203232ff),
                border: rgba(0x203232ff),
                foreground: rgba(0x4b8b8bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x263c3cff),
                border: rgba(0x203232ff),
                foreground: rgba(0x4b8b8bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2d4b4bff),
                border: rgba(0x355d5dff),
                foreground: rgba(0xf8fafaff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x161f1fff),
                border: rgba(0x1b2929ff),
                foreground: rgba(0x407474ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8fafaff),
                border: rgba(0x000000ff),
                foreground: rgba(0x2b4747ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x231a12ff),
                border: rgba(0x392a1aff),
                foreground: rgba(0xa06e3bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x392a1aff),
                border: rgba(0x392a1aff),
                foreground: rgba(0xa06e3bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x45321eff),
                border: rgba(0x392a1aff),
                foreground: rgba(0xa06e3bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x563d23ff),
                border: rgba(0x6b4a2bff),
                foreground: rgba(0xfcf9f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x231a12ff),
                border: rgba(0x2e2216ff),
                foreground: rgba(0x855c33ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfcf9f6ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x513922ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x361414ff),
                border: rgba(0x501e1eff),
                foreground: rgba(0xca4949ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x501e1eff),
                border: rgba(0x501e1eff),
                foreground: rgba(0xca4949ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x5e2323ff),
                border: rgba(0x501e1eff),
                foreground: rgba(0xca4949ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x722a2aff),
                border: rgba(0x8b3333ff),
                foreground: rgba(0xfef8f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x361414ff),
                border: rgba(0x431919ff),
                foreground: rgba(0xa93e3eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfef8f7ff),
                border: rgba(0x190000ff),
                foreground: rgba(0x6d2828ff),
                secondary_foreground: None,
            },
        },
    }
}
