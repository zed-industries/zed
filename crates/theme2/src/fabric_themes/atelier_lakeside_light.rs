use crate::{FabricSurface, FabricSurfaceState, FabricTheme};
use gpui::rgba;

pub fn atelier_lakeside_light() -> FabricTheme {
    FabricTheme {
        name: "Atelier Lakeside Light",
        cotton: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xcdeaf9ff),
                border: rgba(0xb0d3e5ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: Some(
                    rgba(0x526f7dff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0xb0d3e5ff),
                border: rgba(0xb0d3e5ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: Some(
                    rgba(0x526f7dff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x9dc0d2ff),
                border: rgba(0xb0d3e5ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: Some(
                    rgba(0x526f7dff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x82a6b8ff),
                border: rgba(0x799dafff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: Some(
                    rgba(0x161b1dff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xcdeaf9ff),
                border: rgba(0xc1e4f6ff),
                foreground: rgba(0x7195a8ff),
                secondary_foreground: Some(
                    rgba(0x7195a8ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x161b1dff),
                border: rgba(0xebf8ffff),
                foreground: rgba(0x8aaec0ff),
                secondary_foreground: Some(
                    rgba(0x8aaec0ff),
                ),
            },
        },
        linen: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xcdeaf9ff),
                border: rgba(0xb0d3e5ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xb0d3e5ff),
                border: rgba(0xb0d3e5ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0x9dc0d2ff),
                border: rgba(0xb0d3e5ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x82a6b8ff),
                border: rgba(0x799dafff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xcdeaf9ff),
                border: rgba(0xc1e4f6ff),
                foreground: rgba(0x7195a8ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x161b1dff),
                border: rgba(0xebf8ffff),
                foreground: rgba(0x8aaec0ff),
                secondary_foreground: None,
            },
        },
        denim: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xa6cadcff),
                border: rgba(0x80a4b6ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: Some(
                    rgba(0x526f7dff),
                ),
            },
            hovered: FabricSurfaceState {
                background: rgba(0x80a4b6ff),
                border: rgba(0x80a4b6ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: Some(
                    rgba(0x526f7dff),
                ),
            },
            pressed: FabricSurfaceState {
                background: rgba(0x7b9fb1ff),
                border: rgba(0x80a4b6ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: Some(
                    rgba(0x526f7dff),
                ),
            },
            active: FabricSurfaceState {
                background: rgba(0x769aadff),
                border: rgba(0x6f93a6ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: Some(
                    rgba(0x161b1dff),
                ),
            },
            disabled: FabricSurfaceState {
                background: rgba(0xa6cadcff),
                border: rgba(0x93b7c9ff),
                foreground: rgba(0x628496ff),
                secondary_foreground: Some(
                    rgba(0x628496ff),
                ),
            },
            inverted: FabricSurfaceState {
                background: rgba(0x161b1dff),
                border: rgba(0xebf8ffff),
                foreground: rgba(0x779badff),
                secondary_foreground: Some(
                    rgba(0x779badff),
                ),
            },
        },
        silk: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xebf8ffff),
                border: rgba(0xd3edfaff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd3edfaff),
                border: rgba(0xd3edfaff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xc7e7f7ff),
                border: rgba(0xd3edfaff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xb1d5e7ff),
                border: rgba(0x93b7c9ff),
                foreground: rgba(0x161b1dff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xebf8ffff),
                border: rgba(0xdff2fcff),
                foreground: rgba(0x7b9fb1ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x161b1dff),
                border: rgba(0xebf8ffff),
                foreground: rgba(0xb9dceeff),
                secondary_foreground: None,
            },
        },
        satin: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xd8e4eeff),
                border: rgba(0xbacfe1ff),
                foreground: rgba(0x277fadff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xbacfe1ff),
                border: rgba(0xbacfe1ff),
                foreground: rgba(0x277fadff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xaac5daff),
                border: rgba(0xbacfe1ff),
                foreground: rgba(0x277fadff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0x94b6d1ff),
                border: rgba(0x79a6c7ff),
                foreground: rgba(0x05080bff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xd8e4eeff),
                border: rgba(0xc9d9e8ff),
                foreground: rgba(0x5592baff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x05080bff),
                border: rgba(0xffffffff),
                foreground: rgba(0x9abbd4ff),
                secondary_foreground: None,
            },
        },
        positive: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xdde7d5ff),
                border: rgba(0xc2d5b6ff),
                foreground: rgba(0x578c3cff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xc2d5b6ff),
                border: rgba(0xc2d5b6ff),
                foreground: rgba(0x578c3cff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xb5cba6ff),
                border: rgba(0xc2d5b6ff),
                foreground: rgba(0x578c3cff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xa2be90ff),
                border: rgba(0x8cb077ff),
                foreground: rgba(0x060904ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xdde7d5ff),
                border: rgba(0xcfdec6ff),
                foreground: rgba(0x729e59ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x060904ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xa7c396ff),
                secondary_foreground: None,
            },
        },
        warning: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xeae6d0ff),
                border: rgba(0xd8d3abff),
                foreground: rgba(0x8a8a11ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xd8d3abff),
                border: rgba(0xd8d3abff),
                foreground: rgba(0x8a8a11ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xcfc999ff),
                border: rgba(0xd8d3abff),
                foreground: rgba(0x8a8a11ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xc2bc7fff),
                border: rgba(0xb3ae63ff),
                foreground: rgba(0x090803ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xeae6d0ff),
                border: rgba(0xe1dcbdff),
                foreground: rgba(0x9f9c3eff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x090803ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xc6c186ff),
                secondary_foreground: None,
            },
        },
        negative: FabricSurface {
            default: FabricSurfaceState {
                background: rgba(0xfbd8e1ff),
                border: rgba(0xf6bacaff),
                foreground: rgba(0xd22f72ff),
                secondary_foreground: None,
            },
            hovered: FabricSurfaceState {
                background: rgba(0xf6bacaff),
                border: rgba(0xf6bacaff),
                foreground: rgba(0xd22f72ff),
                secondary_foreground: None,
            },
            pressed: FabricSurfaceState {
                background: rgba(0xf3abbfff),
                border: rgba(0xf6bacaff),
                foreground: rgba(0xd22f72ff),
                secondary_foreground: None,
            },
            active: FabricSurfaceState {
                background: rgba(0xee95aeff),
                border: rgba(0xe77b9dff),
                foreground: rgba(0x220507ff),
                secondary_foreground: None,
            },
            disabled: FabricSurfaceState {
                background: rgba(0xfbd8e1ff),
                border: rgba(0xf9cad5ff),
                foreground: rgba(0xdd5987ff),
                secondary_foreground: None,
            },
            inverted: FabricSurfaceState {
                background: rgba(0x220507ff),
                border: rgba(0xffffffff),
                foreground: rgba(0xef9bb3ff),
                secondary_foreground: None,
            },
        },
    }
}
