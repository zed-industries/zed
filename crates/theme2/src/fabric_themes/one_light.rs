use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn one_light() -> FabricTheme {
    FabricTheme {
        name: "One Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xebebec),
                border: rgb(0xdfdfe0),
                foreground: rgb(0x383a41),
                secondary_foreground: Some(
                    rgb(0x7f8188),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdfdfe0),
                border: rgb(0xdfdfe0),
                foreground: rgb(0x383a41),
                secondary_foreground: Some(
                    rgb(0x7f8188),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd9d9da),
                border: rgb(0xdfdfe0),
                foreground: rgb(0x383a41),
                secondary_foreground: Some(
                    rgb(0x7f8188),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xcacaca),
                border: rgb(0xb9b9b9),
                foreground: rgb(0x383a41),
                secondary_foreground: Some(
                    rgb(0x383a41),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xebebec),
                border: rgb(0xe5e5e6),
                foreground: rgb(0xa7a7a8),
                secondary_foreground: Some(
                    rgb(0xa7a7a8),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x383a41),
                border: rgb(0xfafafa),
                foreground: rgb(0xcececf),
                secondary_foreground: Some(
                    rgb(0xcececf),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xebebec),
                border: rgb(0xdfdfe0),
                foreground: rgb(0x383a41),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdfdfe0),
                border: rgb(0xdfdfe0),
                foreground: rgb(0x383a41),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd9d9da),
                border: rgb(0xdfdfe0),
                foreground: rgb(0x383a41),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xcacaca),
                border: rgb(0xb9b9b9),
                foreground: rgb(0x383a41),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xebebec),
                border: rgb(0xe5e5e6),
                foreground: rgb(0xa7a7a8),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x383a41),
                border: rgb(0xfafafa),
                foreground: rgb(0xcececf),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdcdcdd),
                border: rgb(0xc9c9ca),
                foreground: rgb(0x383a41),
                secondary_foreground: Some(
                    rgb(0x7f8188),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc9c9ca),
                border: rgb(0xc9c9ca),
                foreground: rgb(0x383a41),
                secondary_foreground: Some(
                    rgb(0x7f8188),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xbebebf),
                border: rgb(0xc9c9ca),
                foreground: rgb(0x383a41),
                secondary_foreground: Some(
                    rgb(0x7f8188),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0xafafaf),
                border: rgb(0xa6a6a7),
                foreground: rgb(0x383a41),
                secondary_foreground: Some(
                    rgb(0x383a41),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdcdcdd),
                border: rgb(0xd3d3d4),
                foreground: rgb(0xa1a1a3),
                secondary_foreground: Some(
                    rgb(0xa1a1a3),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x383a41),
                border: rgb(0xfafafa),
                foreground: rgb(0xb4b4b4),
                secondary_foreground: Some(
                    rgb(0xb4b4b4),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfafafa),
                border: rgb(0xeeeeee),
                foreground: rgb(0x383a41),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xeeeeee),
                border: rgb(0xeeeeee),
                foreground: rgb(0x383a41),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xe8e8e9),
                border: rgb(0xeeeeee),
                foreground: rgb(0x383a41),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xe0e0e1),
                border: rgb(0xd3d3d4),
                foreground: rgb(0x383a41),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfafafa),
                border: rgb(0xf4f4f4),
                foreground: rgb(0xbebebf),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x383a41),
                border: rgb(0xfafafa),
                foreground: rgb(0xe2e2e3),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe2e2fa),
                border: rgb(0xcbcdf6),
                foreground: rgb(0x5c79e2),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xcbcdf6),
                border: rgb(0xcbcdf6),
                foreground: rgb(0x5c79e2),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xbec2f4),
                border: rgb(0xcbcdf6),
                foreground: rgb(0x5c79e2),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xadb2f1),
                border: rgb(0x98a2ed),
                foreground: rgb(0x7071f),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe2e2fa),
                border: rgb(0xd6d7f8),
                foreground: rgb(0x7c8de8),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x7071f),
                border: rgb(0xffffff),
                foreground: rgb(0xb2b7f1),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe0ebdc),
                border: rgb(0xc8dcc1),
                foreground: rgb(0x669f59),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc8dcc1),
                border: rgb(0xc8dcc1),
                foreground: rgb(0x669f59),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xbcd4b3),
                border: rgb(0xc8dcc1),
                foreground: rgb(0x669f59),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xaac9a0),
                border: rgb(0x97be8b),
                foreground: rgb(0x70a06),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe0ebdc),
                border: rgb(0xd3e4ce),
                foreground: rgb(0x7eae71),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x70a06),
                border: rgb(0xffffff),
                foreground: rgb(0xafcda6),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfaf2e6),
                border: rgb(0xf5e8d2),
                foreground: rgb(0xdec184),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf5e8d2),
                border: rgb(0xf5e8d2),
                foreground: rgb(0xdec184),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xf3e3c8),
                border: rgb(0xf5e8d2),
                foreground: rgb(0xdec184),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xf0dcba),
                border: rgb(0xebd4ab),
                foreground: rgb(0x261b08),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfaf2e6),
                border: rgb(0xf8eddb),
                foreground: rgb(0xe5ca97),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x261b08),
                border: rgb(0xffffff),
                foreground: rgb(0xf0debf),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xfbdfd9),
                border: rgb(0xf6c6bd),
                foreground: rgb(0xd36151),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xf6c6bd),
                border: rgb(0xf6c6bd),
                foreground: rgb(0xd36151),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xf3b9ae),
                border: rgb(0xf6c6bd),
                foreground: rgb(0xd36151),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xefa89b),
                border: rgb(0xe79384),
                foreground: rgb(0x210705),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xfbdfd9),
                border: rgb(0xf9d2cb),
                foreground: rgb(0xde7a6a),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x210705),
                border: rgb(0xffffff),
                foreground: rgb(0xefada0),
                secondary_foreground: None,
            },
        },
    }
}
