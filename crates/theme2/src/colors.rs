use gpui::Hsla;
use refineable::Refineable;

use crate::SyntaxTheme;

#[derive(Clone)]
pub struct SystemColors {
    pub transparent: Hsla,
    pub mac_os_traffic_light_red: Hsla,
    pub mac_os_traffic_light_yellow: Hsla,
    pub mac_os_traffic_light_green: Hsla,
}

#[derive(Debug, Clone, Copy)]
pub struct PlayerColor {
    pub cursor: Hsla,
    pub background: Hsla,
    pub selection: Hsla,
}

#[derive(Clone)]
pub struct PlayerColors(pub Vec<PlayerColor>);

impl PlayerColors {
    pub fn local(&self) -> PlayerColor {
        *self.0.first().unwrap()
    }

    pub fn absent(&self) -> PlayerColor {
        todo!("use a valid color");
        *self.0.last().unwrap()
    }

    pub fn color_for_participant(&self, participant_index: u32) -> PlayerColor {
        let len = self.0.len() - 1;
        self.0[(participant_index as usize % len) + 1]
    }
}

#[derive(Refineable, Clone, Debug)]
#[refineable(debug)]
pub struct StatusColors {
    pub conflict: Hsla,
    pub created: Hsla,
    pub deleted: Hsla,
    pub error: Hsla,
    pub hidden: Hsla,
    pub ignored: Hsla,
    pub info: Hsla,
    pub modified: Hsla,
    pub renamed: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
}

#[derive(Refineable, Clone, Debug)]
#[refineable(debug)]
pub struct GitStatusColors {
    pub conflict: Hsla,
    pub created: Hsla,
    pub deleted: Hsla,
    pub ignored: Hsla,
    pub modified: Hsla,
    pub renamed: Hsla,
}

#[derive(Refineable, Clone, Debug)]
#[refineable(debug, deserialize)]
pub struct ThemeColors {
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub border_selected: Hsla,
    pub border_transparent: Hsla,
    pub border_disabled: Hsla,
    pub elevated_surface_background: Hsla,
    pub surface_background: Hsla,
    pub background: Hsla,
    pub element_background: Hsla,
    pub element_hover: Hsla,
    pub element_active: Hsla,
    pub element_selected: Hsla,
    pub element_disabled: Hsla,
    pub element_placeholder: Hsla,
    pub element_drop_target: Hsla,
    pub ghost_element_background: Hsla,
    pub ghost_element_hover: Hsla,
    pub ghost_element_active: Hsla,
    pub ghost_element_selected: Hsla,
    pub ghost_element_disabled: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_placeholder: Hsla,
    pub text_disabled: Hsla,
    pub text_accent: Hsla,
    pub icon: Hsla,
    pub icon_muted: Hsla,
    pub icon_disabled: Hsla,
    pub icon_placeholder: Hsla,
    pub icon_accent: Hsla,
    pub status_bar_background: Hsla,
    pub title_bar_background: Hsla,
    pub toolbar_background: Hsla,
    pub tab_bar_background: Hsla,
    pub tab_inactive_background: Hsla,
    pub tab_active_background: Hsla,
    pub editor_background: Hsla,
    pub editor_gutter_background: Hsla,
    pub editor_subheader_background: Hsla,
    pub editor_active_line_background: Hsla,
    pub editor_highlighted_line_background: Hsla,
    pub editor_line_number: Hsla,
    pub editor_active_line_number: Hsla,
    pub editor_invisible: Hsla,
    pub editor_wrap_guide: Hsla,
    pub editor_active_wrap_guide: Hsla,
    pub terminal_background: Hsla,
    pub terminal_ansi_bright_black: Hsla,
    pub terminal_ansi_bright_red: Hsla,
    pub terminal_ansi_bright_green: Hsla,
    pub terminal_ansi_bright_yellow: Hsla,
    pub terminal_ansi_bright_blue: Hsla,
    pub terminal_ansi_bright_magenta: Hsla,
    pub terminal_ansi_bright_cyan: Hsla,
    pub terminal_ansi_bright_white: Hsla,
    pub terminal_ansi_black: Hsla,
    pub terminal_ansi_red: Hsla,
    pub terminal_ansi_green: Hsla,
    pub terminal_ansi_yellow: Hsla,
    pub terminal_ansi_blue: Hsla,
    pub terminal_ansi_magenta: Hsla,
    pub terminal_ansi_cyan: Hsla,
    pub terminal_ansi_white: Hsla,
}

#[derive(Refineable, Clone)]
pub struct ThemeStyles {
    pub system: SystemColors,

    #[refineable]
    pub colors: ThemeColors,
    pub status: StatusColors,
    pub git: GitStatusColors,
    pub player: PlayerColors,
    pub syntax: SyntaxTheme,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn override_a_single_theme_color() {
        let mut colors = ThemeColors::default_light();

        let magenta: Hsla = gpui::rgb(0xff00ff);

        assert_ne!(colors.text, magenta);

        let overrides = ThemeColorsRefinement {
            text: Some(magenta),
            ..Default::default()
        };

        colors.refine(&overrides);

        assert_eq!(colors.text, magenta);
    }

    #[test]
    fn override_multiple_theme_colors() {
        let mut colors = ThemeColors::default_light();

        let magenta: Hsla = gpui::rgb(0xff00ff);
        let green: Hsla = gpui::rgb(0x00ff00);

        assert_ne!(colors.text, magenta);
        assert_ne!(colors.background, green);

        let overrides = ThemeColorsRefinement {
            text: Some(magenta),
            background: Some(green),
            ..Default::default()
        };

        colors.refine(&overrides);

        assert_eq!(colors.text, magenta);
        assert_eq!(colors.background, green);
    }

    #[test]
    fn deserialize_theme_colors_refinement_from_json() {
        let colors: ThemeColorsRefinement = serde_json::from_value(json!({
            "background": "#ff00ff",
            "text": "#ff0000"
        }))
        .unwrap();

        assert_eq!(colors.background, Some(gpui::rgb(0xff00ff)));
        assert_eq!(colors.text, Some(gpui::rgb(0xff0000)));
    }
}
