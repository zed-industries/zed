use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn sandcastle() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Sandcastle".into(),
            is_light: false,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x3d4350ff).into(),
        border_variant: rgba(0x3d4350ff).into(),
        border_focused: rgba(0x223131ff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0x333944ff).into(),
        surface: rgba(0x2b3038ff).into(),
        background: rgba(0x333944ff).into(),
        filled_element: rgba(0x333944ff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0x171e1eff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0x171e1eff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0xfdf4c1ff).into(),
        text_muted: rgba(0xa69782ff).into(),
        text_placeholder: rgba(0xb3627aff).into(),
        text_disabled: rgba(0x827568ff).into(),
        text_accent: rgba(0x518b8bff).into(),
        icon_muted: rgba(0xa69782ff).into(),
        syntax: SyntaxTheme { highlights: vec![] },
        status_bar: rgba(0x333944ff).into(),
        title_bar: rgba(0x333944ff).into(),
        toolbar: rgba(0x282c33ff).into(),
        tab_bar: rgba(0x2b3038ff).into(),
        editor: rgba(0x282c33ff).into(),
        editor_subheader: rgba(0x2b3038ff).into(),
        editor_active_line: rgba(0x2b3038ff).into(),
        terminal: rgba(0x282c33ff).into(),
        image_fallback_background: rgba(0x333944ff).into(),
        git_created: rgba(0x83a598ff).into(),
        git_modified: rgba(0x518b8bff).into(),
        git_deleted: rgba(0xb3627aff).into(),
        git_conflict: rgba(0xa07d3aff).into(),
        git_ignored: rgba(0x827568ff).into(),
        git_renamed: rgba(0xa07d3aff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x518b8bff).into(),
                selection: rgba(0x518b8b3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x83a598ff).into(),
                selection: rgba(0x83a5983d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xa87222ff).into(),
                selection: rgba(0xa872223d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xa07d3aff).into(),
                selection: rgba(0xa07d3a3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xd75f5fff).into(),
                selection: rgba(0xd75f5f3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x83a598ff).into(),
                selection: rgba(0x83a5983d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xb3627aff).into(),
                selection: rgba(0xb3627a3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xa07d3aff).into(),
                selection: rgba(0xa07d3a3d).into(),
            },
        ],
    }
}
