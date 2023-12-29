use crate::{{FabricSurface, FabricSurfaceState, FabricTheme}};
use gpui::rgb;

pub fn atelier_seaside_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Seaside Light".into(),
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdaeeda),
                border: rgb(0xbed7be),
                foreground: rgb(0x131513),
                secondary_foreground: Some(
                    rgb(0x5f705f),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbed7be),
                border: rgb(0xbed7be),
                foreground: rgb(0x131513),
                secondary_foreground: Some(
                    rgb(0x5f705f),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0xabc4ab),
                border: rgb(0xbed7be),
                foreground: rgb(0x131513),
                secondary_foreground: Some(
                    rgb(0x5f705f),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x90aa90),
                border: rgb(0x87a187),
                foreground: rgb(0x131513),
                secondary_foreground: Some(
                    rgb(0x131513),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdaeeda),
                border: rgb(0xcfe8cf),
                foreground: rgb(0x809980),
                secondary_foreground: Some(
                    rgb(0x809980),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x131513),
                border: rgb(0xf4fbf4),
                foreground: rgb(0x98b298),
                secondary_foreground: Some(
                    rgb(0x98b298),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xdaeeda),
                border: rgb(0xbed7be),
                foreground: rgb(0x131513),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbed7be),
                border: rgb(0xbed7be),
                foreground: rgb(0x131513),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xabc4ab),
                border: rgb(0xbed7be),
                foreground: rgb(0x131513),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x90aa90),
                border: rgb(0x87a187),
                foreground: rgb(0x131513),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xdaeeda),
                border: rgb(0xcfe8cf),
                foreground: rgb(0x809980),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x131513),
                border: rgb(0xf4fbf4),
                foreground: rgb(0x98b298),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xb4ceb4),
                border: rgb(0x8ea88e),
                foreground: rgb(0x131513),
                secondary_foreground: Some(
                    rgb(0x5f705f),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgb(0x8ea88e),
                border: rgb(0x8ea88e),
                foreground: rgb(0x131513),
                secondary_foreground: Some(
                    rgb(0x5f705f),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgb(0x89a389),
                border: rgb(0x8ea88e),
                foreground: rgb(0x131513),
                secondary_foreground: Some(
                    rgb(0x5f705f),
                ),
            },
            active: FabricSurfaceState {
                background: rgb(0x859e85),
                border: rgb(0x7e977e),
                foreground: rgb(0x131513),
                secondary_foreground: Some(
                    rgb(0x131513),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgb(0xb4ceb4),
                border: rgb(0xa1bba1),
                foreground: rgb(0x718771),
                secondary_foreground: Some(
                    rgb(0x718771),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgb(0x131513),
                border: rgb(0xf4fbf4),
                foreground: rgb(0x859f85),
                secondary_foreground: Some(
                    rgb(0x859f85),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xf4fbf4),
                border: rgb(0xdff0df),
                foreground: rgb(0x131513),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xdff0df),
                border: rgb(0xdff0df),
                foreground: rgb(0x131513),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd5ebd5),
                border: rgb(0xdff0df),
                foreground: rgb(0x131513),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xbfd9bf),
                border: rgb(0xa1bba1),
                foreground: rgb(0x131513),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xf4fbf4),
                border: rgb(0xeaf6ea),
                foreground: rgb(0x89a389),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x131513),
                border: rgb(0xf4fbf4),
                foreground: rgb(0xc7e0c7),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xe1ddfe),
                border: rgb(0xc9c4fd),
                foreground: rgb(0x3f62f4),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xc9c4fd),
                border: rgb(0xc9c4fd),
                foreground: rgb(0x3f62f4),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xbbb7fc),
                border: rgb(0xc9c4fd),
                foreground: rgb(0x3f62f4),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xa8a5fb),
                border: rgb(0x9091fa),
                foreground: rgb(0x3062c),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xe1ddfe),
                border: rgb(0xd5d0fe),
                foreground: rgb(0x6e79f7),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x3062c),
                border: rgb(0xffffff),
                foreground: rgb(0xaeaafc),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xd9edd4),
                border: rgb(0xbbdeb2),
                foreground: rgb(0x2ba32b),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xbbdeb2),
                border: rgb(0xbbdeb2),
                foreground: rgb(0x2ba32b),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xabd7a1),
                border: rgb(0xbbdeb2),
                foreground: rgb(0x2ba32b),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0x96cd8a),
                border: rgb(0x7bc26f),
                foreground: rgb(0x50a04),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xd9edd4),
                border: rgb(0xcae6c3),
                foreground: rgb(0x58b24e),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x50a04),
                border: rgb(0xffffff),
                foreground: rgb(0x9cd090),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xede9d2),
                border: rgb(0xddd8af),
                foreground: rgb(0x98981d),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xddd8af),
                border: rgb(0xddd8af),
                foreground: rgb(0x98981d),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xd5d09d),
                border: rgb(0xddd8af),
                foreground: rgb(0x98981d),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xcac485),
                border: rgb(0xbdb869),
                foreground: rgb(0xa0903),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xede9d2),
                border: rgb(0xe5e1c1),
                foreground: rgb(0xaba846),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0xa0903),
                border: rgb(0xffffff),
                foreground: rgb(0xcec88c),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgb(0xffd8d4),
                border: rgb(0xffb9b4),
                foreground: rgb(0xe61c3d),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgb(0xffb9b4),
                border: rgb(0xffb9b4),
                foreground: rgb(0xe61c3d),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgb(0xffa9a4),
                border: rgb(0xffb9b4),
                foreground: rgb(0xe61c3d),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgb(0xff928e),
                border: rgb(0xf97775),
                foreground: rgb(0x360204),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgb(0xffd8d4),
                border: rgb(0xffc9c4),
                foreground: rgb(0xf05258),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgb(0x360204),
                border: rgb(0xffffff),
                foreground: rgb(0xff9994),
                secondary_foreground: None,
            },
        },
    }
}
