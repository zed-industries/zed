use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_forest_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Forest Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe9e6e4ff),
                border: rgba(0xd6d1cfff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: Some(
                    rgba(0x6a6360ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd6d1cfff),
                border: rgba(0xd6d1cfff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: Some(
                    rgba(0x6a6360ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc5bfbdff),
                border: rgba(0xd6d1cfff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: Some(
                    rgba(0x6a6360ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xaca5a3ff),
                border: rgba(0xa39c99ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: Some(
                    rgba(0x1b1918ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe9e6e4ff),
                border: rgba(0xe6e2e0ff),
                foreground: rgba(0x9c9491ff),
                secondary_foreground: Some(
                    rgba(0x9c9491ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b1918ff),
                border: rgba(0xf1efeeff),
                foreground: rgba(0xb3adabff),
                secondary_foreground: Some(
                    rgba(0xb3adabff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe9e6e4ff),
                border: rgba(0xd6d1cfff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd6d1cfff),
                border: rgba(0xd6d1cfff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc5bfbdff),
                border: rgba(0xd6d1cfff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xaca5a3ff),
                border: rgba(0xa39c99ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe9e6e4ff),
                border: rgba(0xe6e2e0ff),
                foreground: rgba(0x9c9491ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b1918ff),
                border: rgba(0xf1efeeff),
                foreground: rgba(0xb3adabff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xcdc8c6ff),
                border: rgba(0xaaa3a1ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: Some(
                    rgba(0x6a6360ff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xaaa3a1ff),
                border: rgba(0xaaa3a1ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: Some(
                    rgba(0x6a6360ff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0xa59e9bff),
                border: rgba(0xaaa3a1ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: Some(
                    rgba(0x6a6360ff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0xa19996ff),
                border: rgba(0x99918eff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: Some(
                    rgba(0x1b1918ff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xcdc8c6ff),
                border: rgba(0xbcb6b4ff),
                foreground: rgba(0x847c79ff),
                secondary_foreground: Some(
                    rgba(0x847c79ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b1918ff),
                border: rgba(0xf1efeeff),
                foreground: rgba(0xa19a97ff),
                secondary_foreground: Some(
                    rgba(0xa19a97ff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf1efeeff),
                border: rgba(0xebe8e6ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xebe8e6ff),
                border: rgba(0xebe8e6ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xe8e4e2ff),
                border: rgba(0xebe8e6ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xd7d3d1ff),
                border: rgba(0xbcb6b4ff),
                foreground: rgba(0x1b1918ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf1efeeff),
                border: rgba(0xeeebeaff),
                foreground: rgba(0xa59e9bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x1b1918ff),
                border: rgba(0xf1efeeff),
                foreground: rgba(0xdfdad8ff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdfe3fbff),
                border: rgba(0xc6cef7ff),
                foreground: rgba(0x417ee6ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc6cef7ff),
                border: rgba(0xc6cef7ff),
                foreground: rgba(0x417ee6ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb8c4f5ff),
                border: rgba(0xc6cef7ff),
                foreground: rgba(0x417ee6ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xa5b5f3ff),
                border: rgba(0x8da5efff),
                foreground: rgba(0x060821ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdfe3fbff),
                border: rgba(0xd2d8f9ff),
                foreground: rgba(0x6c91ebff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x060821ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xaabaf4ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xe5e9d3ff),
                border: rgba(0xd1d8b1ff),
                foreground: rgba(0x7b9728ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd1d8b1ff),
                border: rgba(0xd1d8b1ff),
                foreground: rgba(0x7b9728ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc7d09fff),
                border: rgba(0xd1d8b1ff),
                foreground: rgba(0x7b9728ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xb8c488ff),
                border: rgba(0xa7b86dff),
                foreground: rgba(0x080903ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xe5e9d3ff),
                border: rgba(0xdbe1c2ff),
                foreground: rgba(0x91a74bff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x080903ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xbdc88eff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xf8e5d1ff),
                border: rgba(0xf0d1adff),
                foreground: rgba(0xc3841aff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf0d1adff),
                border: rgba(0xf0d1adff),
                foreground: rgba(0xc3841aff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xecc79bff),
                border: rgba(0xf0d1adff),
                foreground: rgba(0xc3841aff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xe6b983ff),
                border: rgba(0xddaa66ff),
                foreground: rgba(0x200803ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xf8e5d1ff),
                border: rgba(0xf4dbbfff),
                foreground: rgba(0xd09743ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x200803ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xe7bd89ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xffdad5ff),
                border: rgba(0xffbdb6ff),
                foreground: rgba(0xf22e41ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xffbdb6ff),
                border: rgba(0xffbdb6ff),
                foreground: rgba(0xf22e41ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xffada6ff),
                border: rgba(0xffbdb6ff),
                foreground: rgba(0xf22e41ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xff9891ff),
                border: rgba(0xff7e78ff),
                foreground: rgba(0x3a0104ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xffdad5ff),
                border: rgba(0xffccc6ff),
                foreground: rgba(0xfa5b5cff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x3a0104ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xff9e97ff),
                secondary_foreground: None,
            },
        },
    }
}
