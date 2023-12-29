use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_dune_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Dune Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x262622ff),
                border: rgba(0x3b3933ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: Some(
                    rgba(0xa4a08bff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3b3933ff),
                border: rgba(0x3b3933ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: Some(
                    rgba(0xa4a08bff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4e4c43ff),
                border: rgba(0x3b3933ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: Some(
                    rgba(0xa4a08bff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x6a675aff),
                border: rgba(0x747162ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: Some(
                    rgba(0xfefbecff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x262622ff),
                border: rgba(0x292824ff),
                foreground: rgba(0x7c7968ff),
                secondary_foreground: Some(
                    rgba(0x7c7968ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfefbecff),
                border: rgba(0x20201dff),
                foreground: rgba(0x625f54ff),
                secondary_foreground: Some(
                    rgba(0x625f54ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x262622ff),
                border: rgba(0x3b3933ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3b3933ff),
                border: rgba(0x3b3933ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4e4c43ff),
                border: rgba(0x3b3933ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x6a675aff),
                border: rgba(0x747162ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x262622ff),
                border: rgba(0x292824ff),
                foreground: rgba(0x7c7968ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfefbecff),
                border: rgba(0x20201dff),
                foreground: rgba(0x625f54ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x45433bff),
                border: rgba(0x6c695cff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: Some(
                    rgba(0xa4a08bff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x6c695cff),
                border: rgba(0x6c695cff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: Some(
                    rgba(0xa4a08bff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x726f61ff),
                border: rgba(0x6c695cff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: Some(
                    rgba(0xa4a08bff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x777464ff),
                border: rgba(0x7f7c6aff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: Some(
                    rgba(0xfefbecff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x45433bff),
                border: rgba(0x58564bff),
                foreground: rgba(0x8f8b77ff),
                secondary_foreground: Some(
                    rgba(0x8f8b77ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfefbecff),
                border: rgba(0x20201dff),
                foreground: rgba(0x767363ff),
                secondary_foreground: Some(
                    rgba(0x767363ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x20201dff),
                border: rgba(0x252521ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x252521ff),
                border: rgba(0x252521ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x282723ff),
                border: rgba(0x252521ff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x393832ff),
                border: rgba(0x58564bff),
                foreground: rgba(0xfefbecff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x20201dff),
                border: rgba(0x23221fff),
                foreground: rgba(0x726f61ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfefbecff),
                border: rgba(0x20201dff),
                foreground: rgba(0x31302bff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x171e39ff),
                border: rgba(0x263056ff),
                foreground: rgba(0x6684e0ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x263056ff),
                border: rgba(0x263056ff),
                foreground: rgba(0x6684e0ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2d3a66ff),
                border: rgba(0x263056ff),
                foreground: rgba(0x6684e0ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x38487dff),
                border: rgba(0x445899ff),
                foreground: rgba(0xf9fafeff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x171e39ff),
                border: rgba(0x1e2747ff),
                foreground: rgba(0x556ebcff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf9fafeff),
                border: rgba(0x000016ff),
                foreground: rgba(0x344377ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1a2413ff),
                border: rgba(0x273c1bff),
                foreground: rgba(0x60ac3aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x273c1bff),
                border: rgba(0x273c1bff),
                foreground: rgba(0x60ac3aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x2e491fff),
                border: rgba(0x273c1bff),
                foreground: rgba(0x60ac3aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x385c24ff),
                border: rgba(0x43722aff),
                foreground: rgba(0xf9fcf7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1a2413ff),
                border: rgba(0x203017ff),
                foreground: rgba(0x518e32ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf9fcf7ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x345623ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x2a200eff),
                border: rgba(0x413513ff),
                foreground: rgba(0xae9515ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x413513ff),
                border: rgba(0x413513ff),
                foreground: rgba(0xae9515ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4e4014ff),
                border: rgba(0x413513ff),
                foreground: rgba(0xae9515ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x605015ff),
                border: rgba(0x766316ff),
                foreground: rgba(0xfdfaf6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x2a200eff),
                border: rgba(0x362b11ff),
                foreground: rgba(0x917c16ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdfaf6ff),
                border: rgba(0x0a0000ff),
                foreground: rgba(0x5b4b15ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x450d11ff),
                border: rgba(0x5f1519ff),
                foreground: rgba(0xd73837ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x5f1519ff),
                border: rgba(0x5f1519ff),
                foreground: rgba(0xd73837ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x6d191cff),
                border: rgba(0x5f1519ff),
                foreground: rgba(0xd73837ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x811e21ff),
                border: rgba(0x992528ff),
                foreground: rgba(0xfff7f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x450d11ff),
                border: rgba(0x521115ff),
                foreground: rgba(0xb72e2fff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfff7f6ff),
                border: rgba(0x280000ff),
                foreground: rgba(0x7b1d20ff),
                secondary_foreground: None,
            },
        },
    }
}
