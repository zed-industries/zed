use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_dune_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Dune Dark".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x262622),
                border: rgb(0x3b3933),
                foreground: rgb(0xfefbec),
                secondary_foreground: Some(
                    rgb(0xa4a08b),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3b3933),
                border: rgb(0x3b3933),
                foreground: rgb(0xfefbec),
                secondary_foreground: Some(
                    rgb(0xa4a08b),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4e4c43),
                border: rgb(0x3b3933),
                foreground: rgb(0xfefbec),
                secondary_foreground: Some(
                    rgb(0xa4a08b),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x6a675a),
                border: rgb(0x747162),
                foreground: rgb(0xfefbec),
                secondary_foreground: Some(
                    rgb(0xfefbec),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x262622),
                border: rgb(0x292824),
                foreground: rgb(0x7c7968),
                secondary_foreground: Some(
                    rgb(0x7c7968),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfefbec),
                border: rgb(0x20201d),
                foreground: rgb(0x625f54),
                secondary_foreground: Some(
                    rgb(0x625f54),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x262622),
                border: rgb(0x3b3933),
                foreground: rgb(0xfefbec),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x3b3933),
                border: rgb(0x3b3933),
                foreground: rgb(0xfefbec),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4e4c43),
                border: rgb(0x3b3933),
                foreground: rgb(0xfefbec),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x6a675a),
                border: rgb(0x747162),
                foreground: rgb(0xfefbec),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x262622),
                border: rgb(0x292824),
                foreground: rgb(0x7c7968),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfefbec),
                border: rgb(0x20201d),
                foreground: rgb(0x625f54),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x45433b),
                border: rgb(0x6c695c),
                foreground: rgb(0xfefbec),
                secondary_foreground: Some(
                    rgb(0xa4a08b),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x6c695c),
                border: rgb(0x6c695c),
                foreground: rgb(0xfefbec),
                secondary_foreground: Some(
                    rgb(0xa4a08b),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x726f61),
                border: rgb(0x6c695c),
                foreground: rgb(0xfefbec),
                secondary_foreground: Some(
                    rgb(0xa4a08b),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x777464),
                border: rgb(0x7f7c6a),
                foreground: rgb(0xfefbec),
                secondary_foreground: Some(
                    rgb(0xfefbec),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0x45433b),
                border: rgb(0x58564b),
                foreground: rgb(0x8f8b77),
                secondary_foreground: Some(
                    rgb(0x8f8b77),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfefbec),
                border: rgb(0x20201d),
                foreground: rgb(0x767363),
                secondary_foreground: Some(
                    rgb(0x767363),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x20201d),
                border: rgb(0x252521),
                foreground: rgb(0xfefbec),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x252521),
                border: rgb(0x252521),
                foreground: rgb(0xfefbec),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x282723),
                border: rgb(0x252521),
                foreground: rgb(0xfefbec),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x393832),
                border: rgb(0x58564b),
                foreground: rgb(0xfefbec),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x20201d),
                border: rgb(0x23221f),
                foreground: rgb(0x726f61),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfefbec),
                border: rgb(0x20201d),
                foreground: rgb(0x31302b),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x171e39),
                border: rgb(0x263056),
                foreground: rgb(0x6684e0),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x263056),
                border: rgb(0x263056),
                foreground: rgb(0x6684e0),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2d3a66),
                border: rgb(0x263056),
                foreground: rgb(0x6684e0),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x38487d),
                border: rgb(0x445899),
                foreground: rgb(0xf9fafe),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x171e39),
                border: rgb(0x1e2747),
                foreground: rgb(0x556ebc),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf9fafe),
                border: rgb(0x0016),
                foreground: rgb(0x344377),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x1a2413),
                border: rgb(0x273c1b),
                foreground: rgb(0x60ac3a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x273c1b),
                border: rgb(0x273c1b),
                foreground: rgb(0x60ac3a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x2e491f),
                border: rgb(0x273c1b),
                foreground: rgb(0x60ac3a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x385c24),
                border: rgb(0x43722a),
                foreground: rgb(0xf9fcf7),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x1a2413),
                border: rgb(0x203017),
                foreground: rgb(0x518e32),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xf9fcf7),
                border: rgb(0x0000),
                foreground: rgb(0x345623),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x2a200e),
                border: rgb(0x413513),
                foreground: rgb(0xae9515),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x413513),
                border: rgb(0x413513),
                foreground: rgb(0xae9515),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x4e4014),
                border: rgb(0x413513),
                foreground: rgb(0xae9515),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x605015),
                border: rgb(0x766316),
                foreground: rgb(0xfdfaf6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x2a200e),
                border: rgb(0x362b11),
                foreground: rgb(0x917c16),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfdfaf6),
                border: rgb(0xa0000),
                foreground: rgb(0x5b4b15),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0x450d11),
                border: rgb(0x5f1519),
                foreground: rgb(0xd73837),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0x5f1519),
                border: rgb(0x5f1519),
                foreground: rgb(0xd73837),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0x6d191c),
                border: rgb(0x5f1519),
                foreground: rgb(0xd73837),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x811e21),
                border: rgb(0x992528),
                foreground: rgb(0xfff7f6),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0x450d11),
                border: rgb(0x521115),
                foreground: rgb(0xb72e2f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xfff7f6),
                border: rgb(0x280000),
                foreground: rgb(0x7b1d20),
                secondary_foreground: None,
            },
        },
    }
}
