use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_seaside_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Seaside Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdaeedaff),
                border: rgba(0xbed7beff),
                foreground: rgba(0x131513ff),
                secondary_foreground: Some(
                    rgba(0x5f705fff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbed7beff),
                border: rgba(0xbed7beff),
                foreground: rgba(0x131513ff),
                secondary_foreground: Some(
                    rgba(0x5f705fff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xabc4abff),
                border: rgba(0xbed7beff),
                foreground: rgba(0x131513ff),
                secondary_foreground: Some(
                    rgba(0x5f705fff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x90aa90ff),
                border: rgba(0x87a187ff),
                foreground: rgba(0x131513ff),
                secondary_foreground: Some(
                    rgba(0x131513ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdaeedaff),
                border: rgba(0xcfe8cfff),
                foreground: rgba(0x809980ff),
                secondary_foreground: Some(
                    rgba(0x809980ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x131513ff),
                border: rgba(0xf4fbf4ff),
                foreground: rgba(0x98b298ff),
                secondary_foreground: Some(
                    rgba(0x98b298ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdaeedaff),
                border: rgba(0xbed7beff),
                foreground: rgba(0x131513ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbed7beff),
                border: rgba(0xbed7beff),
                foreground: rgba(0x131513ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xabc4abff),
                border: rgba(0xbed7beff),
                foreground: rgba(0x131513ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x90aa90ff),
                border: rgba(0x87a187ff),
                foreground: rgba(0x131513ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdaeedaff),
                border: rgba(0xcfe8cfff),
                foreground: rgba(0x809980ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x131513ff),
                border: rgba(0xf4fbf4ff),
                foreground: rgba(0x98b298ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xb4ceb4ff),
                border: rgba(0x8ea88eff),
                foreground: rgba(0x131513ff),
                secondary_foreground: Some(
                    rgba(0x5f705fff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x8ea88eff),
                border: rgba(0x8ea88eff),
                foreground: rgba(0x131513ff),
                secondary_foreground: Some(
                    rgba(0x5f705fff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x89a389ff),
                border: rgba(0x8ea88eff),
                foreground: rgba(0x131513ff),
                secondary_foreground: Some(
                    rgba(0x5f705fff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x859e85ff),
                border: rgba(0x7e977eff),
                foreground: rgba(0x131513ff),
                secondary_foreground: Some(
                    rgba(0x131513ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xb4ceb4ff),
                border: rgba(0xa1bba1ff),
                foreground: rgba(0x718771ff),
                secondary_foreground: Some(
                    rgba(0x718771ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x131513ff),
                border: rgba(0xf4fbf4ff),
                foreground: rgba(0x859f85ff),
                secondary_foreground: Some(
                    rgba(0x859f85ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf4fbf4ff),
                border: rgba(0xdff0dfff),
                foreground: rgba(0x131513ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdff0dfff),
                border: rgba(0xdff0dfff),
                foreground: rgba(0x131513ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd5ebd5ff),
                border: rgba(0xdff0dfff),
                foreground: rgba(0x131513ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xbfd9bfff),
                border: rgba(0xa1bba1ff),
                foreground: rgba(0x131513ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf4fbf4ff),
                border: rgba(0xeaf6eaff),
                foreground: rgba(0x89a389ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x131513ff),
                border: rgba(0xf4fbf4ff),
                foreground: rgba(0xc7e0c7ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe1ddfeff),
                border: rgba(0xc9c4fdff),
                foreground: rgba(0x3f62f4ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc9c4fdff),
                border: rgba(0xc9c4fdff),
                foreground: rgba(0x3f62f4ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xbbb7fcff),
                border: rgba(0xc9c4fdff),
                foreground: rgba(0x3f62f4ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xa8a5fbff),
                border: rgba(0x9091faff),
                foreground: rgba(0x03062cff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe1ddfeff),
                border: rgba(0xd5d0feff),
                foreground: rgba(0x6e79f7ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x03062cff),
                border: rgba(0xffffffff),
                foreground: rgba(0xaeaafcff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xd9edd4ff),
                border: rgba(0xbbdeb2ff),
                foreground: rgba(0x2ba32bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbbdeb2ff),
                border: rgba(0xbbdeb2ff),
                foreground: rgba(0x2ba32bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xabd7a1ff),
                border: rgba(0xbbdeb2ff),
                foreground: rgba(0x2ba32bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x96cd8aff),
                border: rgba(0x7bc26fff),
                foreground: rgba(0x050a04ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xd9edd4ff),
                border: rgba(0xcae6c3ff),
                foreground: rgba(0x58b24eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x050a04ff),
                border: rgba(0xffffffff),
                foreground: rgba(0x9cd090ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xede9d2ff),
                border: rgba(0xddd8afff),
                foreground: rgba(0x98981dff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xddd8afff),
                border: rgba(0xddd8afff),
                foreground: rgba(0x98981dff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd5d09dff),
                border: rgba(0xddd8afff),
                foreground: rgba(0x98981dff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xcac485ff),
                border: rgba(0xbdb869ff),
                foreground: rgba(0x0a0903ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xede9d2ff),
                border: rgba(0xe5e1c1ff),
                foreground: rgba(0xaba846ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0a0903ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xcec88cff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xffd8d4ff),
                border: rgba(0xffb9b4ff),
                foreground: rgba(0xe61c3dff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xffb9b4ff),
                border: rgba(0xffb9b4ff),
                foreground: rgba(0xe61c3dff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xffa9a4ff),
                border: rgba(0xffb9b4ff),
                foreground: rgba(0xe61c3dff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xff928eff),
                border: rgba(0xf97775ff),
                foreground: rgba(0x360204ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xffd8d4ff),
                border: rgba(0xffc9c4ff),
                foreground: rgba(0xf05258ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x360204ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xff9994ff),
                secondary_foreground: None,
            },
        },
    }
}
