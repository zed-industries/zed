use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_savanna_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Savanna Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe3ebe6ff),
                border: rgba(0xc8d1cbff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: Some(
                    rgba(0x546259ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc8d1cbff),
                border: rgba(0xc8d1cbff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: Some(
                    rgba(0x546259ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xafb9b2ff),
                border: rgba(0xc8d1cbff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: Some(
                    rgba(0x546259ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x8d9890ff),
                border: rgba(0x818e85ff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: Some(
                    rgba(0x171c19ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe3ebe6ff),
                border: rgba(0xdfe7e2ff),
                foreground: rgba(0x79877dff),
                secondary_foreground: Some(
                    rgba(0x79877dff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x171c19ff),
                border: rgba(0xecf4eeff),
                foreground: rgba(0x97a29aff),
                secondary_foreground: Some(
                    rgba(0x97a29aff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe3ebe6ff),
                border: rgba(0xc8d1cbff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc8d1cbff),
                border: rgba(0xc8d1cbff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xafb9b2ff),
                border: rgba(0xc8d1cbff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x8d9890ff),
                border: rgba(0x818e85ff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe3ebe6ff),
                border: rgba(0xdfe7e2ff),
                foreground: rgba(0x79877dff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x171c19ff),
                border: rgba(0xecf4eeff),
                foreground: rgba(0x97a29aff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xbcc5bfff),
                border: rgba(0x8b968eff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: Some(
                    rgba(0x546259ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x8b968eff),
                border: rgba(0x8b968eff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: Some(
                    rgba(0x546259ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x838f87ff),
                border: rgba(0x8b968eff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: Some(
                    rgba(0x546259ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x7e8b82ff),
                border: rgba(0x76857bff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: Some(
                    rgba(0x171c19ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xbcc5bfff),
                border: rgba(0xa3ada6ff),
                foreground: rgba(0x68766dff),
                secondary_foreground: Some(
                    rgba(0x68766dff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x171c19ff),
                border: rgba(0xecf4eeff),
                foreground: rgba(0x7f8c83ff),
                secondary_foreground: Some(
                    rgba(0x7f8c83ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xecf4eeff),
                border: rgba(0xe5ede7ff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe5ede7ff),
                border: rgba(0xe5ede7ff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe1e9e4ff),
                border: rgba(0xe5ede7ff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xcad3cdff),
                border: rgba(0xa3ada6ff),
                foreground: rgba(0x171c19ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xecf4eeff),
                border: rgba(0xe8f0ebff),
                foreground: rgba(0x838f87ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x171c19ff),
                border: rgba(0xecf4eeff),
                foreground: rgba(0xd4ddd7ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdae7e8ff),
                border: rgba(0xbed4d6ff),
                foreground: rgba(0x488c90ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbed4d6ff),
                border: rgba(0xbed4d6ff),
                foreground: rgba(0x488c90ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xafcbccff),
                border: rgba(0xbed4d6ff),
                foreground: rgba(0x488c90ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9bbec0ff),
                border: rgba(0x84b0b2ff),
                foreground: rgba(0x050909ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdae7e8ff),
                border: rgba(0xccdedeff),
                foreground: rgba(0x679ea1ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x050909ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa1c2c4ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdaeadeff),
                border: rgba(0xbedac5ff),
                foreground: rgba(0x499963ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbedac5ff),
                border: rgba(0xbedac5ff),
                foreground: rgba(0x499963ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb0d2b8ff),
                border: rgba(0xbedac5ff),
                foreground: rgba(0x499963ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x9cc6a6ff),
                border: rgba(0x84ba93ff),
                foreground: rgba(0x050a06ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdaeadeff),
                border: rgba(0xcce2d1ff),
                foreground: rgba(0x68a97aff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x050a06ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa1caabff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xeee4d5ff),
                border: rgba(0xdfcfb6ff),
                foreground: rgba(0xa07e3cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xdfcfb6ff),
                border: rgba(0xdfcfb6ff),
                foreground: rgba(0xa07e3cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xd8c4a6ff),
                border: rgba(0xdfcfb6ff),
                foreground: rgba(0xa07e3cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xcdb590ff),
                border: rgba(0xc1a577ff),
                foreground: rgba(0x0b0804ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xeee4d5ff),
                border: rgba(0xe7d9c6ff),
                foreground: rgba(0xb19159ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0b0804ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xd1ba96ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf3ded4ff),
                border: rgba(0xe8c5b4ff),
                foreground: rgba(0xb1623aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xe8c5b4ff),
                border: rgba(0xe8c5b4ff),
                foreground: rgba(0xb1623aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe2b8a4ff),
                border: rgba(0xe8c5b4ff),
                foreground: rgba(0xb1623aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xdaa68eff),
                border: rgba(0xcf9274ff),
                foreground: rgba(0x0c0604ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf3ded4ff),
                border: rgba(0xeed1c4ff),
                foreground: rgba(0xc17957ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x0c0604ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xddab94ff),
                secondary_foreground: None,
            },
        },
    }
}
