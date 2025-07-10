use std::sync::Arc;

use crate::{
    AccentColors, Appearance, PlayerColor, PlayerColors, StatusColors, SyntaxTheme, SystemColors,
    Theme, ThemeColors, ThemeFamily, ThemeStyles, try_parse_color,
};
use gpui::{HighlightStyle, Hsla, WindowBackgroundAppearance, hsla};

pub const DEFAULT_DARK_THEME_NAME: &str = "One Dark";
pub const DEFAULT_LIGHT_THEME_NAME: &str = "One Light";

const ADDED_COLOR: Hsla = Hsla {
    h: 142. / 360.,
    s: 0.68,
    l: 0.45,
    a: 1.0,
};
const MODIFIED_COLOR: Hsla = Hsla {
    h: 48. / 360.,
    s: 0.76,
    l: 0.47,
    a: 1.0,
};
const REMOVED_COLOR: Hsla = Hsla {
    h: 355. / 360.,
    s: 0.65,
    l: 0.65,
    a: 1.0,
};

pub fn default_theme_family() -> ThemeFamily {
    ThemeFamily {
        id: "one".to_string(),
        name: "One".into(),
        author: "Zed Industries".into(),
        themes: vec![default_dark_theme(), default_light_theme()],
    }
}

pub fn default_dark_theme() -> Theme {
    Theme {
        id: "one-dark".to_string(),
        name: "One Dark".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            window_background_appearance: WindowBackgroundAppearance::Opaque,
            system: SystemColors::default(),
            // todo!("put in actual accent colors")
            accents: AccentColors(vec![
                try_parse_color("#61afef").unwrap_or(hsla(0., 0., 0.15, 1.)),
            ]),
            colors: ThemeColors {
                border: try_parse_color("#181a1f").unwrap_or(hsla(0., 0., 0.1, 1.)),
                border_variant: try_parse_color("#181a1f").unwrap_or(hsla(0., 0., 0.1, 1.)),
                border_focused: try_parse_color("#2e60bc").unwrap_or(hsla(0.6, 0.7, 0.5, 1.)),
                border_selected: try_parse_color("#4370c4").unwrap_or(hsla(0.6, 0.6, 0.6, 1.)),
                border_transparent: hsla(0., 0., 0., 0.),
                border_disabled: try_parse_color("#242629").unwrap_or(hsla(0., 0., 0.15, 1.)),
                elevated_surface_background: try_parse_color("#2c313a")
                    .unwrap_or(hsla(0., 0., 0.2, 1.)),
                surface_background: try_parse_color("#21252b").unwrap_or(hsla(0., 0., 0.15, 1.)),
                background: try_parse_color("#282c34").unwrap_or(hsla(0., 0., 0.2, 1.)),
                element_background: try_parse_color("#2c313a").unwrap_or(hsla(0., 0., 0.2, 1.)),
                element_hover: try_parse_color("#2c313a80").unwrap_or(hsla(0., 0., 0.2, 0.5)),
                element_active: try_parse_color("#2c313a20").unwrap_or(hsla(0., 0., 0.2, 0.125)),
                element_selected: try_parse_color("#2c313a").unwrap_or(hsla(0., 0., 0.2, 1.)),
                element_selection_background: try_parse_color("#4370c43d")
                    .unwrap_or(hsla(0.6, 0.6, 0.6, 0.24)),
                element_disabled: try_parse_color("#21252b").unwrap_or(hsla(0., 0., 0.15, 1.)),
                drop_target_background: try_parse_color("#FF00FF7F")
                    .unwrap_or(hsla(0.83, 1., 0.5, 0.5)),
                ghost_element_background: hsla(0., 0., 0., 0.),
                ghost_element_hover: try_parse_color("#2c313a80").unwrap_or(hsla(0., 0., 0.2, 0.5)),
                ghost_element_active: try_parse_color("#2c313a20")
                    .unwrap_or(hsla(0., 0., 0.2, 0.125)),
                ghost_element_selected: try_parse_color("#2c313a").unwrap_or(hsla(0., 0., 0.2, 1.)),
                ghost_element_disabled: try_parse_color("#21252b")
                    .unwrap_or(hsla(0., 0., 0.15, 1.)),
                text: try_parse_color("#aab2bf").unwrap_or(hsla(0., 0., 0.7, 1.)),
                text_muted: try_parse_color("#5e6670").unwrap_or(hsla(0., 0., 0.4, 1.)),
                text_placeholder: try_parse_color("#838994").unwrap_or(hsla(0., 0., 0.5, 1.)),
                text_disabled: try_parse_color("#4c5360").unwrap_or(hsla(0., 0., 0.3, 1.)),
                text_accent: try_parse_color("#5799e5").unwrap_or(hsla(0.6, 0.7, 0.6, 1.)),
                icon: try_parse_color("#aab2bf").unwrap_or(hsla(0., 0., 0.7, 1.)),
                icon_muted: try_parse_color("#5e6670").unwrap_or(hsla(0., 0., 0.4, 1.)),
                icon_disabled: try_parse_color("#4c5360").unwrap_or(hsla(0., 0., 0.3, 1.)),
                icon_placeholder: try_parse_color("#838994").unwrap_or(hsla(0., 0., 0.5, 1.)),
                icon_accent: try_parse_color("#5799e5").unwrap_or(hsla(0.6, 0.7, 0.6, 1.)),
                status_bar_background: try_parse_color("#21252b").unwrap_or(hsla(0., 0., 0.15, 1.)),
                title_bar_background: try_parse_color("#21252b").unwrap_or(hsla(0., 0., 0.15, 1.)),
                title_bar_inactive_background: try_parse_color("#282c34")
                    .unwrap_or(hsla(0., 0., 0.2, 1.)),
                toolbar_background: try_parse_color("#282c34").unwrap_or(hsla(0., 0., 0.2, 1.)),
                tab_bar_background: try_parse_color("#21252b").unwrap_or(hsla(0., 0., 0.15, 1.)),
                tab_inactive_background: try_parse_color("#21252b")
                    .unwrap_or(hsla(0., 0., 0.15, 1.)),
                tab_active_background: try_parse_color("#282c34").unwrap_or(hsla(0., 0., 0.2, 1.)),
                search_match_background: try_parse_color("#175fcc4d")
                    .unwrap_or(hsla(0.6, 0.8, 0.45, 0.3)),
                panel_background: try_parse_color("#21252b").unwrap_or(hsla(0., 0., 0.15, 1.)),
                panel_focused_border: try_parse_color("#2e60bc").unwrap_or(hsla(0.6, 0.7, 0.5, 1.)),
                panel_indent_guide: try_parse_color("#2a2e36").unwrap_or(hsla(0., 0., 0.17, 1.)),
                panel_indent_guide_hover: try_parse_color("#3a3f4b")
                    .unwrap_or(hsla(0., 0., 0.25, 1.)),
                panel_indent_guide_active: try_parse_color("#4a515e")
                    .unwrap_or(hsla(0., 0., 0.32, 1.)),
                pane_focused_border: try_parse_color("#2e60bc").unwrap_or(hsla(0.6, 0.7, 0.5, 1.)),
                pane_group_border: try_parse_color("#181a1f").unwrap_or(hsla(0., 0., 0.1, 1.)),
                scrollbar_thumb_background: try_parse_color("#F000F03d")
                    .unwrap_or(hsla(0., 1., 0.47, 0.24)),
                scrollbar_thumb_hover_background: try_parse_color("#F000F052")
                    .unwrap_or(hsla(0., 1., 0.47, 0.32)),
                scrollbar_thumb_active_background: try_parse_color("#F000F075")
                    .unwrap_or(hsla(0., 1., 0.47, 0.46)),
                scrollbar_thumb_border: try_parse_color("#2a2e36")
                    .unwrap_or(hsla(0., 0., 0.17, 1.)),
                scrollbar_track_background: hsla(0., 0., 0., 0.),
                scrollbar_track_border: try_parse_color("#2a2d38")
                    .unwrap_or(hsla(0., 0., 0.18, 1.)),
                minimap_thumb_background: try_parse_color("#4c526333")
                    .unwrap_or(hsla(0., 0., 0.35, 0.2)),
                minimap_thumb_hover_background: try_parse_color("#4c52634d")
                    .unwrap_or(hsla(0., 0., 0.35, 0.3)),
                minimap_thumb_active_background: try_parse_color("#4c526366")
                    .unwrap_or(hsla(0., 0., 0.35, 0.4)),
                minimap_thumb_border: try_parse_color("#2a2d38").unwrap_or(hsla(0., 0., 0.18, 1.)),
                editor_foreground: try_parse_color("#abb2bf").unwrap_or(hsla(0., 0., 0.7, 1.)),
                editor_background: try_parse_color("#282c34").unwrap_or(hsla(0., 0., 0.2, 1.)),
                editor_gutter_background: try_parse_color("#282c34")
                    .unwrap_or(hsla(0., 0., 0.2, 1.)),
                editor_subheader_background: try_parse_color("#21252b")
                    .unwrap_or(hsla(0., 0., 0.15, 1.)),
                editor_active_line_background: try_parse_color("#F0F0F00a")
                    .unwrap_or(hsla(0., 0., 0.94, 0.04)),
                editor_highlighted_line_background: try_parse_color("#F0F0F00f")
                    .unwrap_or(hsla(0., 0., 0.94, 0.06)),
                editor_debugger_active_line_background: try_parse_color("#7e6cca52")
                    .unwrap_or(hsla(0.71, 0.4, 0.6, 0.32)),
                editor_line_number: try_parse_color("#636b78").unwrap_or(hsla(0., 0., 0.42, 1.)),
                editor_active_line_number: try_parse_color("#abb2bf")
                    .unwrap_or(hsla(0., 0., 0.7, 1.)),
                editor_hover_line_number: try_parse_color("#abb2bf")
                    .unwrap_or(hsla(0., 0., 0.7, 1.)),
                editor_invisible: try_parse_color("#5e6670").unwrap_or(hsla(0., 0., 0.4, 1.)),
                editor_wrap_guide: try_parse_color("#2a2e36").unwrap_or(hsla(0., 0., 0.17, 1.)),
                editor_active_wrap_guide: try_parse_color("#3a3f4b")
                    .unwrap_or(hsla(0., 0., 0.25, 1.)),
                editor_indent_guide: try_parse_color("#2a2e36").unwrap_or(hsla(0., 0., 0.17, 1.)),
                editor_indent_guide_active: try_parse_color("#3a3f4b")
                    .unwrap_or(hsla(0., 0., 0.25, 1.)),
                editor_document_highlight_read_background: try_parse_color("#4370c40f")
                    .unwrap_or(hsla(0.6, 0.6, 0.6, 0.06)),
                editor_document_highlight_write_background: try_parse_color("#4370c466")
                    .unwrap_or(hsla(0.6, 0.6, 0.6, 0.4)),
                terminal_background: try_parse_color("#282c34").unwrap_or(hsla(0., 0., 0.2, 1.)),
                terminal_foreground: try_parse_color("#abb2bf").unwrap_or(hsla(0., 0., 0.7, 1.)),
                terminal_bright_foreground: try_parse_color("#c8ccd4")
                    .unwrap_or(hsla(0., 0., 0.8, 1.)),
                terminal_dim_foreground: try_parse_color("#5f697a")
                    .unwrap_or(hsla(0., 0., 0.4, 1.)),
                terminal_ansi_black: try_parse_color("#282c34").unwrap_or(hsla(0., 0., 0.2, 1.)),
                terminal_ansi_bright_black: try_parse_color("#3f4451")
                    .unwrap_or(hsla(0., 0., 0.27, 1.)),
                terminal_ansi_dim_black: try_parse_color("#5f697a")
                    .unwrap_or(hsla(0., 0., 0.4, 1.)),
                terminal_ansi_red: try_parse_color("#e06c75").unwrap_or(hsla(0.97, 0.65, 0.65, 1.)),
                terminal_ansi_bright_red: try_parse_color("#d07277")
                    .unwrap_or(hsla(0.97, 0.55, 0.65, 1.)),
                terminal_ansi_dim_red: try_parse_color("#c45660")
                    .unwrap_or(hsla(0.97, 0.5, 0.57, 1.)),
                terminal_ansi_green: try_parse_color("#98c379").unwrap_or(hsla(0.3, 0.4, 0.6, 1.)),
                terminal_ansi_bright_green: try_parse_color("#a9d88d")
                    .unwrap_or(hsla(0.3, 0.5, 0.7, 1.)),
                terminal_ansi_dim_green: try_parse_color("#76a85d")
                    .unwrap_or(hsla(0.3, 0.3, 0.5, 1.)),
                terminal_ansi_yellow: try_parse_color("#e5c07b")
                    .unwrap_or(hsla(0.11, 0.67, 0.67, 1.)),
                terminal_ansi_bright_yellow: try_parse_color("#f0d197")
                    .unwrap_or(hsla(0.11, 0.7, 0.77, 1.)),
                terminal_ansi_dim_yellow: try_parse_color("#d7a55f")
                    .unwrap_or(hsla(0.11, 0.6, 0.6, 1.)),
                terminal_ansi_blue: try_parse_color("#61afef")
                    .unwrap_or(hsla(0.58, 0.81, 0.68, 1.)),
                terminal_ansi_bright_blue: try_parse_color("#6cb3ff")
                    .unwrap_or(hsla(0.58, 1., 0.71, 1.)),
                terminal_ansi_dim_blue: try_parse_color("#3d8fd4")
                    .unwrap_or(hsla(0.58, 0.7, 0.54, 1.)),
                terminal_ansi_magenta: try_parse_color("#c678dd")
                    .unwrap_or(hsla(0.78, 0.6, 0.69, 1.)),
                terminal_ansi_bright_magenta: try_parse_color("#ca82e5")
                    .unwrap_or(hsla(0.78, 0.7, 0.7, 1.)),
                terminal_ansi_dim_magenta: try_parse_color("#a65cc8")
                    .unwrap_or(hsla(0.78, 0.5, 0.6, 1.)),
                terminal_ansi_cyan: try_parse_color("#56b6c2").unwrap_or(hsla(0.53, 0.5, 0.56, 1.)),
                terminal_ansi_bright_cyan: try_parse_color("#64c5d3")
                    .unwrap_or(hsla(0.53, 0.6, 0.61, 1.)),
                terminal_ansi_dim_cyan: try_parse_color("#4096a1")
                    .unwrap_or(hsla(0.53, 0.42, 0.44, 1.)),
                terminal_ansi_white: try_parse_color("#dcdfe4").unwrap_or(hsla(0., 0., 0.87, 1.)),
                terminal_ansi_bright_white: try_parse_color("#f0f0f0")
                    .unwrap_or(hsla(0., 0., 0.94, 1.)),
                terminal_ansi_dim_white: try_parse_color("#abb2bf")
                    .unwrap_or(hsla(0., 0., 0.7, 1.)),
                link_text_hover: try_parse_color("#5799e5").unwrap_or(hsla(0.6, 0.7, 0.6, 1.)),
                version_control_added: ADDED_COLOR,
                version_control_modified: MODIFIED_COLOR,
                version_control_deleted: REMOVED_COLOR,
                version_control_conflict_marker_ours: try_parse_color("#98c379")
                    .unwrap_or(hsla(0.3, 0.4, 0.6, 1.)),
                version_control_conflict_marker_theirs: try_parse_color("#61afef")
                    .unwrap_or(hsla(0.58, 0.81, 0.68, 1.)),
                debugger_accent: try_parse_color("#c678dd").unwrap_or(hsla(0.78, 0.6, 0.69, 1.)),
                editor_document_highlight_bracket_background: try_parse_color("#4370c419")
                    .unwrap_or(hsla(0.6, 0.6, 0.6, 0.1)),
                terminal_ansi_background: try_parse_color("#282c34")
                    .unwrap_or(hsla(0., 0., 0.2, 1.)),
                version_control_renamed: try_parse_color("#61afef")
                    .unwrap_or(hsla(0.58, 0.81, 0.68, 1.)),
                version_control_conflict: try_parse_color("#e5c07b")
                    .unwrap_or(hsla(0.11, 0.67, 0.67, 1.)),
                version_control_ignored: try_parse_color("#5e6670")
                    .unwrap_or(hsla(0., 0., 0.4, 1.)),
            },
            status: StatusColors {
                conflict: try_parse_color("#e5c07b").unwrap_or(hsla(0.11, 0.67, 0.67, 1.)),
                conflict_background: try_parse_color("#332412")
                    .unwrap_or(hsla(0.11, 0.5, 0.14, 1.)),
                conflict_border: try_parse_color("#5d4224").unwrap_or(hsla(0.11, 0.5, 0.25, 1.)),
                created: try_parse_color("#98c379").unwrap_or(hsla(0.3, 0.4, 0.6, 1.)),
                created_background: try_parse_color("#252d1f").unwrap_or(hsla(0.3, 0.2, 0.15, 1.)),
                created_border: try_parse_color("#436330").unwrap_or(hsla(0.3, 0.35, 0.3, 1.)),
                deleted: try_parse_color("#e06c75").unwrap_or(hsla(0.97, 0.65, 0.65, 1.)),
                deleted_background: try_parse_color("#301c1e")
                    .unwrap_or(hsla(0.97, 0.25, 0.15, 1.)),
                deleted_border: try_parse_color("#5e3135").unwrap_or(hsla(0.97, 0.3, 0.28, 1.)),
                error: try_parse_color("#e06c75").unwrap_or(hsla(0.97, 0.65, 0.65, 1.)),
                error_background: try_parse_color("#301c1e").unwrap_or(hsla(0.97, 0.25, 0.15, 1.)),
                error_border: try_parse_color("#5e3135").unwrap_or(hsla(0.97, 0.3, 0.28, 1.)),
                hidden: try_parse_color("#838994").unwrap_or(hsla(0., 0., 0.5, 1.)),
                hidden_background: try_parse_color("#23252a").unwrap_or(hsla(0., 0., 0.15, 1.)),
                hidden_border: try_parse_color("#3f4248").unwrap_or(hsla(0., 0., 0.26, 1.)),
                hint: try_parse_color("#969dad").unwrap_or(hsla(0., 0., 0.6, 1.)),
                hint_background: try_parse_color("#212843").unwrap_or(hsla(0.64, 0.34, 0.2, 1.)),
                hint_border: try_parse_color("#3d4466").unwrap_or(hsla(0.64, 0.24, 0.32, 1.)),
                ignored: try_parse_color("#838994").unwrap_or(hsla(0., 0., 0.5, 1.)),
                ignored_background: try_parse_color("#23252a").unwrap_or(hsla(0., 0., 0.15, 1.)),
                ignored_border: try_parse_color("#3f4248").unwrap_or(hsla(0., 0., 0.26, 1.)),
                info: try_parse_color("#61afef").unwrap_or(hsla(0.58, 0.81, 0.68, 1.)),
                info_background: try_parse_color("#1a2939").unwrap_or(hsla(0.58, 0.35, 0.17, 1.)),
                info_border: try_parse_color("#274d75").unwrap_or(hsla(0.58, 0.5, 0.31, 1.)),
                modified: try_parse_color("#e5c07b").unwrap_or(hsla(0.11, 0.67, 0.67, 1.)),
                modified_background: try_parse_color("#332412")
                    .unwrap_or(hsla(0.11, 0.5, 0.14, 1.)),
                modified_border: try_parse_color("#5d4224").unwrap_or(hsla(0.11, 0.5, 0.25, 1.)),
                predictive: try_parse_color("#808491").unwrap_or(hsla(0., 0., 0.55, 1.)),
                predictive_background: try_parse_color("#222326").unwrap_or(hsla(0., 0., 0.14, 1.)),
                predictive_border: try_parse_color("#3c3e45").unwrap_or(hsla(0., 0., 0.26, 1.)),
                renamed: try_parse_color("#61afef").unwrap_or(hsla(0.58, 0.81, 0.68, 1.)),
                renamed_background: try_parse_color("#1a2939")
                    .unwrap_or(hsla(0.58, 0.35, 0.17, 1.)),
                renamed_border: try_parse_color("#274d75").unwrap_or(hsla(0.58, 0.5, 0.31, 1.)),
                success: try_parse_color("#98c379").unwrap_or(hsla(0.3, 0.4, 0.6, 1.)),
                success_background: try_parse_color("#252d1f").unwrap_or(hsla(0.3, 0.2, 0.15, 1.)),
                success_border: try_parse_color("#436330").unwrap_or(hsla(0.3, 0.35, 0.3, 1.)),
                unreachable: try_parse_color("#838994").unwrap_or(hsla(0., 0., 0.5, 1.)),
                unreachable_background: try_parse_color("#23252a")
                    .unwrap_or(hsla(0., 0., 0.15, 1.)),
                unreachable_border: try_parse_color("#3f4248").unwrap_or(hsla(0., 0., 0.26, 1.)),
                warning: try_parse_color("#e5c07b").unwrap_or(hsla(0.11, 0.67, 0.67, 1.)),
                warning_background: try_parse_color("#332412").unwrap_or(hsla(0.11, 0.5, 0.14, 1.)),
                warning_border: try_parse_color("#5d4224").unwrap_or(hsla(0.11, 0.5, 0.25, 1.)),
            },
            player: PlayerColors(vec![
                PlayerColor {
                    cursor: try_parse_color("#61afef").unwrap_or(hsla(0.58, 0.81, 0.68, 1.)),
                    background: try_parse_color("#61afef").unwrap_or(hsla(0.58, 0.81, 0.68, 1.)),
                    selection: try_parse_color("#61afef3d").unwrap_or(hsla(0.58, 0.81, 0.68, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#a86fe8").unwrap_or(hsla(0.75, 0.75, 0.68, 1.)),
                    background: try_parse_color("#a86fe8").unwrap_or(hsla(0.75, 0.75, 0.68, 1.)),
                    selection: try_parse_color("#a86fe83d").unwrap_or(hsla(0.75, 0.75, 0.68, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#50d699").unwrap_or(hsla(0.42, 0.6, 0.59, 1.)),
                    background: try_parse_color("#50d699").unwrap_or(hsla(0.42, 0.6, 0.59, 1.)),
                    selection: try_parse_color("#50d6993d").unwrap_or(hsla(0.42, 0.6, 0.59, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#f35955").unwrap_or(hsla(0.005, 0.87, 0.65, 1.)),
                    background: try_parse_color("#f35955").unwrap_or(hsla(0.005, 0.87, 0.65, 1.)),
                    selection: try_parse_color("#f359553d")
                        .unwrap_or(hsla(0.005, 0.87, 0.65, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#fe9934").unwrap_or(hsla(0.08, 0.99, 0.6, 1.)),
                    background: try_parse_color("#fe9934").unwrap_or(hsla(0.08, 0.99, 0.6, 1.)),
                    selection: try_parse_color("#fe99343d").unwrap_or(hsla(0.08, 0.99, 0.6, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#ff69b4").unwrap_or(hsla(0.92, 1., 0.71, 1.)),
                    background: try_parse_color("#ff69b4").unwrap_or(hsla(0.92, 1., 0.71, 1.)),
                    selection: try_parse_color("#ff69b43d").unwrap_or(hsla(0.92, 1., 0.71, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#00bfff").unwrap_or(hsla(0.55, 1., 0.5, 1.)),
                    background: try_parse_color("#00bfff").unwrap_or(hsla(0.55, 1., 0.5, 1.)),
                    selection: try_parse_color("#00bfff3d").unwrap_or(hsla(0.55, 1., 0.5, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#00ff00").unwrap_or(hsla(0.33, 1., 0.5, 1.)),
                    background: try_parse_color("#00ff00").unwrap_or(hsla(0.33, 1., 0.5, 1.)),
                    selection: try_parse_color("#00ff003d").unwrap_or(hsla(0.33, 1., 0.5, 0.24)),
                },
            ]),
            syntax: Arc::new(SyntaxTheme {
                highlights: vec![
                    (
                        "attribute".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e5c07b")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.11, 0.67, 0.67, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "boolean".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#d19a66")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.08, 0.45, 0.61, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "comment".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#6f7380")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.45, 1.))),
                            font_style: Some(gpui::FontStyle::Italic),
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "comment.doc".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#848896")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.53, 1.))),
                            font_style: Some(gpui::FontStyle::Italic),
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "constant".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#d19a66")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.08, 0.45, 0.61, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "constructor".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#61afef")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.58, 0.81, 0.68, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "emphasis".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#5799e5")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.6, 0.7, 0.6, 1.))),
                            font_style: Some(gpui::FontStyle::Italic),
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "emphasis.strong".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#5799e5")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.6, 0.7, 0.6, 1.))),
                            font_style: None,
                            font_weight: Some(gpui::FontWeight::BOLD),
                            ..Default::default()
                        },
                    ),
                    (
                        "function".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#61afef")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.58, 0.81, 0.68, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "keyword".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#c678dd")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.78, 0.6, 0.69, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "label".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#61afef")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.58, 0.81, 0.68, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "link_text".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#93c1e8")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.58, 0.65, 0.74, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "link_uri".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#56b6c2")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.53, 0.5, 0.56, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "number".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#d19a66")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.08, 0.45, 0.61, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "operator".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#56b6c2")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.53, 0.5, 0.56, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "preproc".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#c678dd")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.78, 0.6, 0.69, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "property".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e06c75")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.97, 0.65, 0.65, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#abb2bf")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.7, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation.bracket".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#abb2bf")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.7, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation.delimiter".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#abb2bf")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.7, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation.list_marker".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e06c75")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.97, 0.65, 0.65, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation.special".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e06c75")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.97, 0.65, 0.65, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#98c379")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.3, 0.4, 0.6, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string.escape".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#56b6c2")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.53, 0.5, 0.56, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string.regex".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e06c75")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.97, 0.65, 0.65, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string.special".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e06c75")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.97, 0.65, 0.65, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string.special.symbol".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#98c379")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.3, 0.4, 0.6, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "tag".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e06c75")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.97, 0.65, 0.65, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "text.literal".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#98c379")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.3, 0.4, 0.6, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "title".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#61afef")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.58, 0.81, 0.68, 1.))),
                            font_style: None,
                            font_weight: Some(gpui::FontWeight::BOLD),
                            ..Default::default()
                        },
                    ),
                    (
                        "type".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e5c07b")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.11, 0.67, 0.67, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "variable".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e06c75")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.97, 0.65, 0.65, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "variable.special".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#c678dd")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.78, 0.6, 0.69, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "variant".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#61afef")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.58, 0.81, 0.68, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                ],
            }),
        },
    }
}

pub fn default_light_theme() -> Theme {
    Theme {
        id: "one-light".to_string(),
        name: "One Light".into(),
        appearance: Appearance::Light,
        styles: ThemeStyles {
            window_background_appearance: WindowBackgroundAppearance::Opaque,
            system: SystemColors::default(),
            // todo! fill out correct colors
            accents: AccentColors(vec![try_parse_color("red").unwrap_or(hsla(0., 0., 0., 0.))]),
            colors: ThemeColors {
                border: try_parse_color("#d9dce0").unwrap_or(hsla(0., 0., 0.86, 1.)),
                border_variant: try_parse_color("#d9dce0").unwrap_or(hsla(0., 0., 0.86, 1.)),
                border_focused: try_parse_color("#2188ff").unwrap_or(hsla(0.58, 1., 0.57, 1.)),
                border_selected: try_parse_color("#4198ff").unwrap_or(hsla(0.58, 1., 0.63, 1.)),
                border_transparent: hsla(0., 0., 0., 0.),
                border_disabled: try_parse_color("#c9cbd1").unwrap_or(hsla(0., 0., 0.79, 1.)),
                elevated_surface_background: try_parse_color("#e0e1e6")
                    .unwrap_or(hsla(0., 0., 0.88, 1.)),
                surface_background: try_parse_color("#f0f0f1").unwrap_or(hsla(0., 0., 0.94, 1.)),
                background: try_parse_color("#fafafa").unwrap_or(hsla(0., 0., 0.98, 1.)),
                element_background: try_parse_color("#e0e1e6").unwrap_or(hsla(0., 0., 0.88, 1.)),
                element_hover: try_parse_color("#e0e1e680").unwrap_or(hsla(0., 0., 0.88, 0.5)),
                element_active: try_parse_color("#e0e1e620").unwrap_or(hsla(0., 0., 0.88, 0.125)),
                element_selected: try_parse_color("#e0e1e6").unwrap_or(hsla(0., 0., 0.88, 1.)),
                element_selection_background: try_parse_color("#4198ff3d")
                    .unwrap_or(hsla(0.58, 1., 0.63, 0.24)),
                element_disabled: try_parse_color("#f0f0f1").unwrap_or(hsla(0., 0., 0.94, 1.)),
                drop_target_background: try_parse_color("#FF00FF7F")
                    .unwrap_or(hsla(0.83, 1., 0.5, 0.5)),
                ghost_element_background: hsla(0., 0., 0., 0.),
                ghost_element_hover: try_parse_color("#e0e1e680")
                    .unwrap_or(hsla(0., 0., 0.88, 0.5)),
                ghost_element_active: try_parse_color("#e0e1e620")
                    .unwrap_or(hsla(0., 0., 0.88, 0.125)),
                ghost_element_selected: try_parse_color("#e0e1e6")
                    .unwrap_or(hsla(0., 0., 0.88, 1.)),
                ghost_element_disabled: try_parse_color("#f0f0f1")
                    .unwrap_or(hsla(0., 0., 0.94, 1.)),
                text: try_parse_color("#383a42").unwrap_or(hsla(0., 0., 0.24, 1.)),
                text_muted: try_parse_color("#a2a3a7").unwrap_or(hsla(0., 0., 0.64, 1.)),
                text_placeholder: try_parse_color("#898b92").unwrap_or(hsla(0., 0., 0.55, 1.)),
                text_disabled: try_parse_color("#a8abb3").unwrap_or(hsla(0., 0., 0.67, 1.)),
                text_accent: try_parse_color("#0d74f0").unwrap_or(hsla(0.58, 0.9, 0.5, 1.)),
                icon: try_parse_color("#383a42").unwrap_or(hsla(0., 0., 0.24, 1.)),
                icon_muted: try_parse_color("#7f848e").unwrap_or(hsla(0., 0., 0.52, 1.)),
                icon_disabled: try_parse_color("#a8abb3").unwrap_or(hsla(0., 0., 0.67, 1.)),
                icon_placeholder: try_parse_color("#898b92").unwrap_or(hsla(0., 0., 0.55, 1.)),
                icon_accent: try_parse_color("#0d74f0").unwrap_or(hsla(0.58, 0.9, 0.5, 1.)),
                status_bar_background: try_parse_color("#f0f0f1").unwrap_or(hsla(0., 0., 0.94, 1.)),
                title_bar_background: try_parse_color("#f0f0f1").unwrap_or(hsla(0., 0., 0.94, 1.)),
                title_bar_inactive_background: try_parse_color("#fafafa")
                    .unwrap_or(hsla(0., 0., 0.98, 1.)),
                toolbar_background: try_parse_color("#fafafa").unwrap_or(hsla(0., 0., 0.98, 1.)),
                tab_bar_background: try_parse_color("#f0f0f1").unwrap_or(hsla(0., 0., 0.94, 1.)),
                tab_inactive_background: try_parse_color("#f0f0f1")
                    .unwrap_or(hsla(0., 0., 0.94, 1.)),
                tab_active_background: try_parse_color("#fafafa").unwrap_or(hsla(0., 0., 0.98, 1.)),
                search_match_background: try_parse_color("#175fcc4d")
                    .unwrap_or(hsla(0.58, 0.8, 0.45, 0.3)),
                panel_background: try_parse_color("#f0f0f1").unwrap_or(hsla(0., 0., 0.94, 1.)),
                panel_focused_border: try_parse_color("#2188ff")
                    .unwrap_or(hsla(0.58, 1., 0.57, 1.)),
                panel_indent_guide: try_parse_color("#e0e1e6").unwrap_or(hsla(0., 0., 0.88, 1.)),
                panel_indent_guide_hover: try_parse_color("#c9cbd1")
                    .unwrap_or(hsla(0., 0., 0.79, 1.)),
                panel_indent_guide_active: try_parse_color("#b2b5bd")
                    .unwrap_or(hsla(0., 0., 0.71, 1.)),
                pane_focused_border: try_parse_color("#2188ff").unwrap_or(hsla(0.58, 1., 0.57, 1.)),
                pane_group_border: try_parse_color("#d9dce0").unwrap_or(hsla(0., 0., 0.86, 1.)),
                scrollbar_thumb_background: try_parse_color("#F000F03d")
                    .unwrap_or(hsla(0., 1., 0.47, 0.24)),
                scrollbar_thumb_hover_background: try_parse_color("#F000F052")
                    .unwrap_or(hsla(0., 1., 0.47, 0.32)),
                scrollbar_thumb_active_background: try_parse_color("#F000F075")
                    .unwrap_or(hsla(0., 1., 0.47, 0.46)),
                scrollbar_thumb_border: try_parse_color("#e0e1e6")
                    .unwrap_or(hsla(0., 0., 0.88, 1.)),
                scrollbar_track_background: hsla(0., 0., 0., 0.),
                scrollbar_track_border: try_parse_color("#eff0f1")
                    .unwrap_or(hsla(0., 0., 0.94, 1.)),
                minimap_thumb_background: try_parse_color("#b2b5bd33")
                    .unwrap_or(hsla(0., 0., 0.71, 0.2)),
                minimap_thumb_hover_background: try_parse_color("#b2b5bd4d")
                    .unwrap_or(hsla(0., 0., 0.71, 0.3)),
                minimap_thumb_active_background: try_parse_color("#b2b5bd66")
                    .unwrap_or(hsla(0., 0., 0.71, 0.4)),
                minimap_thumb_border: try_parse_color("#eff0f1").unwrap_or(hsla(0., 0., 0.94, 1.)),
                editor_foreground: try_parse_color("#383a42").unwrap_or(hsla(0., 0., 0.24, 1.)),
                editor_background: try_parse_color("#fafafa").unwrap_or(hsla(0., 0., 0.98, 1.)),
                editor_gutter_background: try_parse_color("#fafafa")
                    .unwrap_or(hsla(0., 0., 0.98, 1.)),
                editor_subheader_background: try_parse_color("#f0f0f1")
                    .unwrap_or(hsla(0., 0., 0.94, 1.)),
                editor_active_line_background: try_parse_color("#F0F0F00a")
                    .unwrap_or(hsla(0., 0., 0.94, 0.04)),
                editor_highlighted_line_background: try_parse_color("#F0F0F00f")
                    .unwrap_or(hsla(0., 0., 0.94, 0.06)),
                editor_debugger_active_line_background: try_parse_color("#7e6cca52")
                    .unwrap_or(hsla(0.71, 0.4, 0.6, 0.32)),
                editor_line_number: try_parse_color("#9fa2a6").unwrap_or(hsla(0., 0., 0.63, 1.)),
                editor_active_line_number: try_parse_color("#383a42")
                    .unwrap_or(hsla(0., 0., 0.24, 1.)),
                editor_hover_line_number: try_parse_color("#383a42")
                    .unwrap_or(hsla(0., 0., 0.24, 1.)),
                editor_invisible: try_parse_color("#a2a3a7").unwrap_or(hsla(0., 0., 0.64, 1.)),
                editor_wrap_guide: try_parse_color("#e0e1e6").unwrap_or(hsla(0., 0., 0.88, 1.)),
                editor_active_wrap_guide: try_parse_color("#c9cbd1")
                    .unwrap_or(hsla(0., 0., 0.79, 1.)),
                editor_indent_guide: try_parse_color("#e0e1e6").unwrap_or(hsla(0., 0., 0.88, 1.)),
                editor_indent_guide_active: try_parse_color("#c9cbd1")
                    .unwrap_or(hsla(0., 0., 0.79, 1.)),
                editor_document_highlight_read_background: try_parse_color("#4198ff0f")
                    .unwrap_or(hsla(0.58, 1., 0.63, 0.06)),
                editor_document_highlight_write_background: try_parse_color("#4198ff66")
                    .unwrap_or(hsla(0.58, 1., 0.63, 0.4)),
                terminal_background: try_parse_color("#fafafa").unwrap_or(hsla(0., 0., 0.98, 1.)),
                terminal_foreground: try_parse_color("#383a42").unwrap_or(hsla(0., 0., 0.24, 1.)),
                terminal_bright_foreground: try_parse_color("#2f313a")
                    .unwrap_or(hsla(0., 0., 0.21, 1.)),
                terminal_dim_foreground: try_parse_color("#7e7f83")
                    .unwrap_or(hsla(0., 0., 0.5, 1.)),
                terminal_ansi_black: try_parse_color("#fafafa").unwrap_or(hsla(0., 0., 0.98, 1.)),
                terminal_ansi_bright_black: try_parse_color("#d8d9db")
                    .unwrap_or(hsla(0., 0., 0.85, 1.)),
                terminal_ansi_dim_black: try_parse_color("#7e7f83")
                    .unwrap_or(hsla(0., 0., 0.5, 1.)),
                terminal_ansi_red: try_parse_color("#e45649").unwrap_or(hsla(0.01, 0.74, 0.59, 1.)),
                terminal_ansi_bright_red: try_parse_color("#ca695a")
                    .unwrap_or(hsla(0.02, 0.54, 0.58, 1.)),
                terminal_ansi_dim_red: try_parse_color("#c24941")
                    .unwrap_or(hsla(0.01, 0.5, 0.5, 1.)),
                terminal_ansi_green: try_parse_color("#50a14f")
                    .unwrap_or(hsla(0.33, 0.34, 0.47, 1.)),
                terminal_ansi_bright_green: try_parse_color("#6db164")
                    .unwrap_or(hsla(0.35, 0.42, 0.55, 1.)),
                terminal_ansi_dim_green: try_parse_color("#418141")
                    .unwrap_or(hsla(0.33, 0.33, 0.38, 1.)),
                terminal_ansi_yellow: try_parse_color("#c18401")
                    .unwrap_or(hsla(0.11, 0.99, 0.38, 1.)),
                terminal_ansi_bright_yellow: try_parse_color("#d49c3d")
                    .unwrap_or(hsla(0.11, 0.67, 0.53, 1.)),
                terminal_ansi_dim_yellow: try_parse_color("#9e6a01")
                    .unwrap_or(hsla(0.11, 0.99, 0.31, 1.)),
                terminal_ansi_blue: try_parse_color("#4078f2").unwrap_or(hsla(0.6, 0.88, 0.6, 1.)),
                terminal_ansi_bright_blue: try_parse_color("#5085ce")
                    .unwrap_or(hsla(0.6, 0.6, 0.56, 1.)),
                terminal_ansi_dim_blue: try_parse_color("#2d60c8")
                    .unwrap_or(hsla(0.6, 0.65, 0.48, 1.)),
                terminal_ansi_magenta: try_parse_color("#a626a4")
                    .unwrap_or(hsla(0.83, 0.61, 0.4, 1.)),
                terminal_ansi_bright_magenta: try_parse_color("#a84db2")
                    .unwrap_or(hsla(0.82, 0.42, 0.49, 1.)),
                terminal_ansi_dim_magenta: try_parse_color("#841e83")
                    .unwrap_or(hsla(0.83, 0.64, 0.32, 1.)),
                terminal_ansi_cyan: try_parse_color("#0184bc")
                    .unwrap_or(hsla(0.55, 0.99, 0.37, 1.)),
                terminal_ansi_bright_cyan: try_parse_color("#4394c7")
                    .unwrap_or(hsla(0.55, 0.5, 0.52, 1.)),
                terminal_ansi_dim_cyan: try_parse_color("#006a97")
                    .unwrap_or(hsla(0.55, 1., 0.29, 1.)),
                terminal_ansi_white: try_parse_color("#1e2127").unwrap_or(hsla(0., 0., 0.13, 1.)),
                terminal_ansi_bright_white: try_parse_color("#F0F0F0")
                    .unwrap_or(hsla(0., 0., 0.94, 1.)),
                terminal_ansi_dim_white: try_parse_color("#383a42")
                    .unwrap_or(hsla(0., 0., 0.24, 1.)),
                link_text_hover: try_parse_color("#0d74f0").unwrap_or(hsla(0.58, 0.9, 0.5, 1.)),
                version_control_added: ADDED_COLOR,
                version_control_modified: MODIFIED_COLOR,
                version_control_deleted: REMOVED_COLOR,
                version_control_conflict_marker_ours: try_parse_color("#50a14f")
                    .unwrap_or(hsla(0.3, 0.34, 0.47, 1.)),
                version_control_conflict_marker_theirs: try_parse_color("#4078f2")
                    .unwrap_or(hsla(0.6, 0.88, 0.6, 1.)),
                debugger_accent: try_parse_color("#a626a4").unwrap_or(hsla(0.83, 0.61, 0.4, 1.)),
                editor_document_highlight_bracket_background: try_parse_color("#4198ff19")
                    .unwrap_or(hsla(0.58, 1., 0.63, 0.1)),
                terminal_ansi_background: try_parse_color("#fafafa")
                    .unwrap_or(hsla(0., 0., 0.98, 1.)),
                version_control_renamed: try_parse_color("#4078f2")
                    .unwrap_or(hsla(0.6, 0.88, 0.6, 1.)),
                version_control_conflict: try_parse_color("#c18401")
                    .unwrap_or(hsla(0.11, 0.99, 0.38, 1.)),
                version_control_ignored: try_parse_color("#a2a3a7")
                    .unwrap_or(hsla(0., 0., 0.64, 1.)),
            },
            status: StatusColors {
                conflict: try_parse_color("#e5c07b").unwrap_or(hsla(0.11, 0.67, 0.67, 1.)),
                conflict_background: try_parse_color("#f2eeda")
                    .unwrap_or(hsla(0.11, 0.5, 0.91, 1.)),
                conflict_border: try_parse_color("#e2d5b1").unwrap_or(hsla(0.11, 0.5, 0.78, 1.)),
                created: try_parse_color("#50a14f").unwrap_or(hsla(0.3, 0.34, 0.47, 1.)),
                created_background: try_parse_color("#e1eee1").unwrap_or(hsla(0.33, 0.23, 0.9, 1.)),
                created_border: try_parse_color("#b7d4b7").unwrap_or(hsla(0.33, 0.29, 0.75, 1.)),
                deleted: try_parse_color("#e45649").unwrap_or(hsla(0.01, 0.74, 0.59, 1.)),
                deleted_background: try_parse_color("#fae0dd").unwrap_or(hsla(0.01, 0.7, 0.92, 1.)),
                deleted_border: try_parse_color("#ecb8b2").unwrap_or(hsla(0.01, 0.6, 0.83, 1.)),
                error: try_parse_color("#e45649").unwrap_or(hsla(0.01, 0.74, 0.59, 1.)),
                error_background: try_parse_color("#fae0dd").unwrap_or(hsla(0.01, 0.7, 0.92, 1.)),
                error_border: try_parse_color("#ecb8b2").unwrap_or(hsla(0.01, 0.6, 0.83, 1.)),
                hidden: try_parse_color("#898b92").unwrap_or(hsla(0., 0., 0.55, 1.)),
                hidden_background: try_parse_color("#f4f4f5").unwrap_or(hsla(0., 0., 0.96, 1.)),
                hidden_border: try_parse_color("#e1e2e5").unwrap_or(hsla(0., 0., 0.89, 1.)),
                hint: try_parse_color("#6b6d76").unwrap_or(hsla(0., 0., 0.44, 1.)),
                hint_background: try_parse_color("#e8e8ed").unwrap_or(hsla(0.64, 0.15, 0.91, 1.)),
                hint_border: try_parse_color("#c8c9d5").unwrap_or(hsla(0.64, 0.18, 0.81, 1.)),
                ignored: try_parse_color("#898b92").unwrap_or(hsla(0., 0., 0.55, 1.)),
                ignored_background: try_parse_color("#f4f4f5").unwrap_or(hsla(0., 0., 0.96, 1.)),
                ignored_border: try_parse_color("#e1e2e5").unwrap_or(hsla(0., 0., 0.89, 1.)),
                info: try_parse_color("#4078f2").unwrap_or(hsla(0.6, 0.88, 0.6, 1.)),
                info_background: try_parse_color("#d9e4fa").unwrap_or(hsla(0.6, 0.75, 0.92, 1.)),
                info_border: try_parse_color("#a9c0ea").unwrap_or(hsla(0.6, 0.55, 0.77, 1.)),
                modified: try_parse_color("#e5c07b").unwrap_or(hsla(0.11, 0.67, 0.67, 1.)),
                modified_background: try_parse_color("#f2eeda")
                    .unwrap_or(hsla(0.11, 0.5, 0.91, 1.)),
                modified_border: try_parse_color("#e2d5b1").unwrap_or(hsla(0.11, 0.5, 0.78, 1.)),
                predictive: try_parse_color("#969799").unwrap_or(hsla(0., 0., 0.59, 1.)),
                predictive_background: try_parse_color("#f5f5f5").unwrap_or(hsla(0., 0., 0.96, 1.)),
                predictive_border: try_parse_color("#e4e4e5").unwrap_or(hsla(0., 0., 0.89, 1.)),
                renamed: try_parse_color("#4078f2").unwrap_or(hsla(0.6, 0.88, 0.6, 1.)),
                renamed_background: try_parse_color("#d9e4fa").unwrap_or(hsla(0.6, 0.75, 0.92, 1.)),
                renamed_border: try_parse_color("#a9c0ea").unwrap_or(hsla(0.6, 0.55, 0.77, 1.)),
                success: try_parse_color("#50a14f").unwrap_or(hsla(0.3, 0.34, 0.47, 1.)),
                success_background: try_parse_color("#e1eee1").unwrap_or(hsla(0.33, 0.23, 0.9, 1.)),
                success_border: try_parse_color("#b7d4b7").unwrap_or(hsla(0.33, 0.29, 0.75, 1.)),
                unreachable: try_parse_color("#898b92").unwrap_or(hsla(0., 0., 0.55, 1.)),
                unreachable_background: try_parse_color("#f4f4f5")
                    .unwrap_or(hsla(0., 0., 0.96, 1.)),
                unreachable_border: try_parse_color("#e1e2e5").unwrap_or(hsla(0., 0., 0.89, 1.)),
                warning: try_parse_color("#e5c07b").unwrap_or(hsla(0.11, 0.67, 0.67, 1.)),
                warning_background: try_parse_color("#f2eeda").unwrap_or(hsla(0.11, 0.5, 0.91, 1.)),
                warning_border: try_parse_color("#e2d5b1").unwrap_or(hsla(0.11, 0.5, 0.78, 1.)),
            },
            player: PlayerColors(vec![
                PlayerColor {
                    cursor: try_parse_color("#4078f2").unwrap_or(hsla(0.6, 0.88, 0.6, 1.)),
                    background: try_parse_color("#4078f2").unwrap_or(hsla(0.6, 0.88, 0.6, 1.)),
                    selection: try_parse_color("#4078f23d").unwrap_or(hsla(0.6, 0.88, 0.6, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#994cc3").unwrap_or(hsla(0.77, 0.5, 0.53, 1.)),
                    background: try_parse_color("#994cc3").unwrap_or(hsla(0.77, 0.5, 0.53, 1.)),
                    selection: try_parse_color("#994cc33d").unwrap_or(hsla(0.77, 0.5, 0.53, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#0aa579").unwrap_or(hsla(0.45, 0.9, 0.34, 1.)),
                    background: try_parse_color("#0aa579").unwrap_or(hsla(0.45, 0.9, 0.34, 1.)),
                    selection: try_parse_color("#0aa5793d").unwrap_or(hsla(0.45, 0.9, 0.34, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#d4333c").unwrap_or(hsla(0.98, 0.63, 0.52, 1.)),
                    background: try_parse_color("#d4333c").unwrap_or(hsla(0.98, 0.63, 0.52, 1.)),
                    selection: try_parse_color("#d4333c3d").unwrap_or(hsla(0.98, 0.63, 0.52, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#d46600").unwrap_or(hsla(0.08, 1., 0.42, 1.)),
                    background: try_parse_color("#d46600").unwrap_or(hsla(0.08, 1., 0.42, 1.)),
                    selection: try_parse_color("#d466003d").unwrap_or(hsla(0.08, 1., 0.42, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#ff69b4").unwrap_or(hsla(0.92, 1., 0.71, 1.)),
                    background: try_parse_color("#ff69b4").unwrap_or(hsla(0.92, 1., 0.71, 1.)),
                    selection: try_parse_color("#ff69b43d").unwrap_or(hsla(0.92, 1., 0.71, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#00a8cc").unwrap_or(hsla(0.54, 1., 0.4, 1.)),
                    background: try_parse_color("#00a8cc").unwrap_or(hsla(0.54, 1., 0.4, 1.)),
                    selection: try_parse_color("#00a8cc3d").unwrap_or(hsla(0.54, 1., 0.4, 0.24)),
                },
                PlayerColor {
                    cursor: try_parse_color("#00ff00").unwrap_or(hsla(0.33, 1., 0.5, 1.)),
                    background: try_parse_color("#00ff00").unwrap_or(hsla(0.33, 1., 0.5, 1.)),
                    selection: try_parse_color("#00ff003d").unwrap_or(hsla(0.33, 1., 0.5, 0.24)),
                },
            ]),
            syntax: Arc::new(SyntaxTheme {
                highlights: vec![
                    (
                        "attribute".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#c18401")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.11, 0.99, 0.38, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "boolean".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#986801")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.11, 0.99, 0.3, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "comment".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#b0b2b6")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.7, 1.))),
                            font_style: Some(gpui::FontStyle::Italic),
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "comment.doc".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#9b9da3")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.62, 1.))),
                            font_style: Some(gpui::FontStyle::Italic),
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "constant".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#986801")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.11, 0.99, 0.3, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "constructor".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#4078f2")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.6, 0.88, 0.6, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "emphasis".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#0d74f0")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.58, 0.9, 0.5, 1.))),
                            font_style: Some(gpui::FontStyle::Italic),
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "emphasis.strong".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#0d74f0")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.58, 0.9, 0.5, 1.))),
                            font_style: None,
                            font_weight: Some(gpui::FontWeight::BOLD),
                            ..Default::default()
                        },
                    ),
                    (
                        "function".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#4078f2")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.6, 0.88, 0.6, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "keyword".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#a626a4")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.83, 0.61, 0.4, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "label".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#4078f2")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.6, 0.88, 0.6, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "link_text".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#5685f5")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.6, 0.9, 0.65, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "link_uri".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#0184bc")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.55, 0.99, 0.37, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "number".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#986801")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.11, 0.99, 0.3, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "operator".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#0184bc")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.55, 0.99, 0.37, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "preproc".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#a626a4")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.83, 0.61, 0.4, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "property".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e45649")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.01, 0.74, 0.59, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#383a42")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.24, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation.bracket".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#383a42")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.24, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation.delimiter".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#383a42")
                                .map(Some)
                                .unwrap_or(Some(hsla(0., 0., 0.24, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation.list_marker".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e45649")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.01, 0.74, 0.59, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "punctuation.special".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e45649")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.01, 0.74, 0.59, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#50a14f")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.3, 0.34, 0.47, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string.escape".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#0184bc")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.55, 0.99, 0.37, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string.regex".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e45649")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.01, 0.74, 0.59, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string.special".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e45649")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.01, 0.74, 0.59, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "string.special.symbol".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#50a14f")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.3, 0.34, 0.47, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "tag".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e45649")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.01, 0.74, 0.59, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "text.literal".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#50a14f")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.3, 0.34, 0.47, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "title".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#4078f2")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.6, 0.88, 0.6, 1.))),
                            font_style: None,
                            font_weight: Some(gpui::FontWeight::BOLD),
                            ..Default::default()
                        },
                    ),
                    (
                        "type".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#c18401")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.11, 0.99, 0.38, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "variable".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#e45649")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.01, 0.74, 0.59, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "variable.special".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#a626a4")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.83, 0.61, 0.4, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                    (
                        "variant".to_string(),
                        HighlightStyle {
                            color: try_parse_color("#4078f2")
                                .map(Some)
                                .unwrap_or(Some(hsla(0.6, 0.88, 0.6, 1.))),
                            font_style: None,
                            font_weight: None,
                            ..Default::default()
                        },
                    ),
                ],
            }),
        },
    }
}
