use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn one_dark() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "One Dark".into(),
            is_light: false,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x464b57ff).into(),
        border_variant: rgba(0x464b57ff).into(),
        border_focused: rgba(0x293b5bff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0x3b414dff).into(),
        surface: rgba(0x2f343eff).into(),
        background: rgba(0x3b414dff).into(),
        filled_element: rgba(0x3b414dff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0x18243dff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0x18243dff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0xc8ccd4ff).into(),
        text_muted: rgba(0x838994ff).into(),
        text_placeholder: rgba(0xd07277ff).into(),
        text_disabled: rgba(0x555a63ff).into(),
        text_accent: rgba(0x74ade8ff).into(),
        icon_muted: rgba(0x838994ff).into(),
        syntax: SyntaxTheme {
            comment: rgba(0x5d636fff).into(),
            string: rgba(0xa1c181ff).into(),
            function: rgba(0x73ade9ff).into(),
            keyword: rgba(0xb477cfff).into(),
            highlights: vec![],
        },
        status_bar: rgba(0x3b414dff).into(),
        title_bar: rgba(0x3b414dff).into(),
        toolbar: rgba(0x282c33ff).into(),
        tab_bar: rgba(0x2f343eff).into(),
        editor: rgba(0x282c33ff).into(),
        editor_subheader: rgba(0x2f343eff).into(),
        editor_active_line: rgba(0x2f343eff).into(),
        terminal: rgba(0x282c33ff).into(),
        image_fallback_background: rgba(0x3b414dff).into(),
        git_created: rgba(0xa1c181ff).into(),
        git_modified: rgba(0x74ade8ff).into(),
        git_deleted: rgba(0xd07277ff).into(),
        git_conflict: rgba(0xdec184ff).into(),
        git_ignored: rgba(0x555a63ff).into(),
        git_renamed: rgba(0xdec184ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x74ade8ff).into(),
                selection: rgba(0x74ade83d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xa1c181ff).into(),
                selection: rgba(0xa1c1813d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xbe5046ff).into(),
                selection: rgba(0xbe50463d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xbf956aff).into(),
                selection: rgba(0xbf956a3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xb477cfff).into(),
                selection: rgba(0xb477cf3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x6eb4bfff).into(),
                selection: rgba(0x6eb4bf3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xd07277ff).into(),
                selection: rgba(0xd072773d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xdec184ff).into(),
                selection: rgba(0xdec1843d).into(),
            },
        ],
    }
}
