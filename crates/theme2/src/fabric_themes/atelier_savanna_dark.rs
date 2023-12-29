use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_savanna_dark() -> FabricTheme {
    FabricTheme {
        name: "Atelier Savanna Dark",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1f2621ff),
                border: rgba(0x2f3832ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: Some(
                    rgba(0x859188ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2f3832ff),
                border: rgba(0x2f3832ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: Some(
                    rgba(0x859188ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3c4740ff),
                border: rgba(0x2f3832ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: Some(
                    rgba(0x859188ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x4f5c53ff),
                border: rgba(0x57655cff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: Some(
                    rgba(0xecf4eeff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1f2621ff),
                border: rgba(0x232a25ff),
                foreground: rgba(0x5f6d64ff),
                secondary_foreground: Some(
                    rgba(0x5f6d64ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xecf4eeff),
                border: rgba(0x171c19ff),
                foreground: rgba(0x49564eff),
                secondary_foreground: Some(
                    rgba(0x49564eff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x1f2621ff),
                border: rgba(0x2f3832ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x2f3832ff),
                border: rgba(0x2f3832ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x3c4740ff),
                border: rgba(0x2f3832ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x4f5c53ff),
                border: rgba(0x57655cff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x1f2621ff),
                border: rgba(0x232a25ff),
                foreground: rgba(0x5f6d64ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xecf4eeff),
                border: rgba(0x171c19ff),
                foreground: rgba(0x49564eff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x353f39ff),
                border: rgba(0x505e55ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: Some(
                    rgba(0x859188ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x505e55ff),
                border: rgba(0x505e55ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: Some(
                    rgba(0x859188ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x55635aff),
                border: rgba(0x505e55ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: Some(
                    rgba(0x859188ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x5a685fff),
                border: rgba(0x616f66ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: Some(
                    rgba(0xecf4eeff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0x353f39ff),
                border: rgba(0x434f47ff),
                foreground: rgba(0x6f7e74ff),
                secondary_foreground: Some(
                    rgba(0x6f7e74ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0xecf4eeff),
                border: rgba(0x171c19ff),
                foreground: rgba(0x59675eff),
                secondary_foreground: Some(
                    rgba(0x59675eff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x171c19ff),
                border: rgba(0x1e2420ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1e2420ff),
                border: rgba(0x1e2420ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x212823ff),
                border: rgba(0x1e2420ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2e3731ff),
                border: rgba(0x434f47ff),
                foreground: rgba(0xecf4eeff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x171c19ff),
                border: rgba(0x1a201cff),
                foreground: rgba(0x55635aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xecf4eeff),
                border: rgba(0x171c19ff),
                foreground: rgba(0x29302bff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x151f20ff),
                border: rgba(0x1f3233ff),
                foreground: rgba(0x478c90ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x1f3233ff),
                border: rgba(0x1f3233ff),
                foreground: rgba(0x478c90ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x243d3eff),
                border: rgba(0x1f3233ff),
                foreground: rgba(0x478c90ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2b4c4eff),
                border: rgba(0x335d60ff),
                foreground: rgba(0xf8fafaff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x151f20ff),
                border: rgba(0x1a292aff),
                foreground: rgba(0x3d7578ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8fafaff),
                border: rgba(0x000000ff),
                foreground: rgba(0x294749ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x162119ff),
                border: rgba(0x203626ff),
                foreground: rgba(0x489963ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x203626ff),
                border: rgba(0x203626ff),
                foreground: rgba(0x489963ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x25422eff),
                border: rgba(0x203626ff),
                foreground: rgba(0x489963ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x2b5238ff),
                border: rgba(0x346643ff),
                foreground: rgba(0xf8fbf8ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x162119ff),
                border: rgba(0x1b2c1fff),
                foreground: rgba(0x3e7f53ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xf8fbf8ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x2a4d34ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x231d12ff),
                border: rgba(0x392e1aff),
                foreground: rgba(0xa07e3bff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x392e1aff),
                border: rgba(0x392e1aff),
                foreground: rgba(0xa07e3bff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x45371fff),
                border: rgba(0x392e1aff),
                foreground: rgba(0xa07e3bff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x564524ff),
                border: rgba(0x6b542bff),
                foreground: rgba(0xfcf9f7ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x231d12ff),
                border: rgba(0x2e2516ff),
                foreground: rgba(0x856933ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfcf9f7ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x514023ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0x261811ff),
                border: rgba(0x3f2619ff),
                foreground: rgba(0xb16139ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0x3f2619ff),
                border: rgba(0x3f2619ff),
                foreground: rgba(0xb16139ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x4c2d1dff),
                border: rgba(0x3f2619ff),
                foreground: rgba(0xb16139ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x5f3722ff),
                border: rgba(0x764229ff),
                foreground: rgba(0xfdf8f6ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0x261811ff),
                border: rgba(0x331f16ff),
                foreground: rgba(0x935131ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0xfdf8f6ff),
                border: rgba(0x000000ff),
                foreground: rgba(0x5a3321ff),
                secondary_foreground: None,
            },
        },
    }
}
