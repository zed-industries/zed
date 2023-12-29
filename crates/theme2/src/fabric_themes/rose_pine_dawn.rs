use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn rose_pine_dawn() -> FabricTheme {
    FabricTheme {
        name: "Rosé Pine Dawn".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfef9f2),
                border: rgb(0xe5e0df),
                foreground: rgb(0x575279),
                secondary_foreground: Some(
                    rgb(0x706c8c),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe5e0df),
                border: rgb(0xe5e0df),
                foreground: rgb(0x575279),
                secondary_foreground: Some(
                    rgb(0x706c8c),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd4d0d2),
                border: rgb(0xe5e0df),
                foreground: rgb(0x575279),
                secondary_foreground: Some(
                    rgb(0x706c8c),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xdbd5d4),
                border: rgb(0xdbd3d1),
                foreground: rgb(0x575279),
                secondary_foreground: Some(
                    rgb(0x575279),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfef9f2),
                border: rgb(0xf6f1eb),
                foreground: rgb(0xb1abb5),
                secondary_foreground: Some(
                    rgb(0xb1abb5),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x575279),
                border: rgb(0xfaf4ed),
                foreground: rgb(0xd6d1d1),
                secondary_foreground: Some(
                    rgb(0xd6d1d1),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfef9f2),
                border: rgb(0xe5e0df),
                foreground: rgb(0x575279),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe5e0df),
                border: rgb(0xe5e0df),
                foreground: rgb(0x575279),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd4d0d2),
                border: rgb(0xe5e0df),
                foreground: rgb(0x575279),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xdbd5d4),
                border: rgb(0xdbd3d1),
                foreground: rgb(0x575279),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfef9f2),
                border: rgb(0xf6f1eb),
                foreground: rgb(0xb1abb5),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x575279),
                border: rgb(0xfaf4ed),
                foreground: rgb(0xd6d1d1),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdcd8d8),
                border: rgb(0xdcd6d5),
                foreground: rgb(0x575279),
                secondary_foreground: Some(
                    rgb(0x706c8c),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdcd6d5),
                border: rgb(0xdcd6d5),
                foreground: rgb(0x575279),
                secondary_foreground: Some(
                    rgb(0x706c8c),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xefe6df),
                border: rgb(0xdcd6d5),
                foreground: rgb(0x575279),
                secondary_foreground: Some(
                    rgb(0x706c8c),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xc1bac1),
                border: rgb(0xa9a3b0),
                foreground: rgb(0x575279),
                secondary_foreground: Some(
                    rgb(0x575279),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdcd8d8),
                border: rgb(0xd0cccf),
                foreground: rgb(0x938fa3),
                secondary_foreground: Some(
                    rgb(0x938fa3),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x575279),
                border: rgb(0xfaf4ed),
                foreground: rgb(0xc7c0c5),
                secondary_foreground: Some(
                    rgb(0xc7c0c5),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfaf4ed),
                border: rgb(0xfdf8f1),
                foreground: rgb(0x575279),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xfdf8f1),
                border: rgb(0xfdf8f1),
                foreground: rgb(0x575279),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xfdf8f2),
                border: rgb(0xfdf8f1),
                foreground: rgb(0x575279),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xe6e1e0),
                border: rgb(0xd0cccf),
                foreground: rgb(0x575279),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfaf4ed),
                border: rgb(0xfcf6ef),
                foreground: rgb(0xefe6df),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x575279),
                border: rgb(0xfaf4ed),
                foreground: rgb(0xede9e5),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdde9eb),
                border: rgb(0xc3d7db),
                foreground: rgb(0x57949f),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc3d7db),
                border: rgb(0xc3d7db),
                foreground: rgb(0x57949f),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb6cfd3),
                border: rgb(0xc3d7db),
                foreground: rgb(0x57949f),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xa3c3c9),
                border: rgb(0x8db6bd),
                foreground: rgb(0x6090a),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdde9eb),
                border: rgb(0xd0e0e3),
                foreground: rgb(0x72a5ae),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x6090a),
                border: rgb(0xffffff),
                foreground: rgb(0xa8c7cd),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdbeee7),
                border: rgb(0xbee0d5),
                foreground: rgb(0x3eaa8e),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbee0d5),
                border: rgb(0xbee0d5),
                foreground: rgb(0x3eaa8e),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xb0dacb),
                border: rgb(0xbee0d5),
                foreground: rgb(0x3eaa8e),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x9bd0bf),
                border: rgb(0x82c6b1),
                foreground: rgb(0x60a09),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdbeee7),
                border: rgb(0xcde7de),
                foreground: rgb(0x63b89f),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x60a09),
                border: rgb(0xffffff),
                foreground: rgb(0xa1d4c3),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xffebd6),
                border: rgb(0xffdab7),
                foreground: rgb(0xe99d35),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xffdab7),
                border: rgb(0xffdab7),
                foreground: rgb(0xe99d35),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xfed2a6),
                border: rgb(0xffdab7),
                foreground: rgb(0xe99d35),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xfbc891),
                border: rgb(0xf7bc77),
                foreground: rgb(0x330704),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xffebd6),
                border: rgb(0xffe2c7),
                foreground: rgb(0xf1ac57),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x330704),
                border: rgb(0xffffff),
                foreground: rgb(0xfccb97),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf1dfe3),
                border: rgb(0xe6c6cd),
                foreground: rgb(0xb4647a),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xe6c6cd),
                border: rgb(0xe6c6cd),
                foreground: rgb(0xb4647a),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe0bac2),
                border: rgb(0xe6c6cd),
                foreground: rgb(0xb4647a),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xd8a8b3),
                border: rgb(0xce94a3),
                foreground: rgb(0xb0708),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf1dfe3),
                border: rgb(0xecd2d8),
                foreground: rgb(0xc17b8e),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xb0708),
                border: rgb(0xffffff),
                foreground: rgb(0xdbadb8),
                secondary_foreground: None,
            },
        },
    }
}
