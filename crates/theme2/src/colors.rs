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
        // todo!("use a valid color");
        *self.0.first().unwrap()
    }

    pub fn absent(&self) -> PlayerColor {
        // todo!("use a valid color");
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
    /// Border color used for deemphasized borders, like a visual divider between two sections
    pub border_variant: Hsla,
    /// Border color used for focused elements, like keyboard focused list item.
    pub border_focused: Hsla,
    /// Border color used for selected elements, like an active search filter or selected checkbox.
    pub border_selected: Hsla,
    /// Border color used for transparent borders. Used for placeholder borders when an element gains a border on state change.
    pub border_transparent: Hsla,
    /// Border color used for disabled elements, like a disabled input or button.
    pub border_disabled: Hsla,
    /// Used for elevated surfaces, like a context menu, popup, or dialog.
    pub elevated_surface_background: Hsla,
    /// Used for grounded surfaces like a panel or tab.
    pub surface_background: Hsla,
    /// Used the app background and blank panels or windows.
    pub background: Hsla,
    /// Used for the background of an element that should have a different background than the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have the same background as the surface it's on, use `ghost_element_background`.
    pub element_background: Hsla,
    /// Used for the hover state of an element that should have a different background than the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    pub element_hover: Hsla,
    /// Used for the active state of an element that should have a different background than the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
    pub element_active: Hsla,
    /// Used for the selected state of an element that should have a different background than the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    pub element_selected: Hsla,
    /// Used for the disabled state of an element that should have a different background than the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
    pub element_disabled: Hsla,
    /// Used for the text color of an element that should have a different background than the surface it's on.
    ///
    /// Example: A input with some default placeholder text.
    pub element_placeholder_text: Hsla,
    /// Background color of the area that shows where a dragged element will be dropped.
    pub drop_target_background: Hsla,
    /// Border color of the area that shows where a dragged element will be dropped.
    // pub drop_target_border: Hsla,
    /// Used for the background of a ghost element that should have the same background as the surface it's on.
    ///
    /// Elements might include: Buttons, Inputs, Checkboxes, Radio Buttons...
    ///
    /// For an element that should have a different background than the surface it's on, use `element_background`.
    pub ghost_element_background: Hsla,
    /// Used for the hover state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Hover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.
    pub ghost_element_hover: Hsla,
    /// Used for the active state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Active states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.
    pub ghost_element_active: Hsla,
    /// Used for the selected state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Selected states are triggered by the element being selected (or "activated") by the user.
    ///
    /// This could include a selected checkbox, a toggleable button that is toggled on, etc.
    pub ghost_element_selected: Hsla,
    /// Used for the disabled state of a ghost element that should have the same background as the surface it's on.
    ///
    /// Disabled states are shown when a user cannot interact with an element, like a disabled button or input.
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
    pub editor_document_highlight_read_background: Hsla,
    pub editor_document_highlight_write_background: Hsla,
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
    // new colors

    // == elevation ==
    // elevatation_0_shadow
    // elevatation_0_shadow_color
    // elevatation_1_shadow
    // elevatation_1_shadow_color
    // elevatation_2_shadow
    // elevatation_2_shadow_color
    // elevatation_3_shadow
    // elevatation_3_shadow_color
    // elevatation_4_shadow
    // elevatation_4_shadow_color
    // elevatation_5_shadow
    // elevatation_5_shadow_color

    // == rich text ==
    // headline
    // paragraph
    // link
    // link_hover
    // code_block_background
    // code_block_border

    // == misc ==
    // inverted_element_*
    // foreground: Overall foreground color. This color is only used if not overridden by a component.
    // disabledForeground: Overall foreground for disabled elements. This color is only used if not overridden by a component.
    // widget.border: Border color of widgets such as Find/Replace inside the editor.
    // widget.shadow: Shadow color of widgets such as Find/Replace inside the editor.
    // selection - foreground, background
    // active_element_border
    // inactive_element_border
    // element_seperator
    // scrollbar_thumb_background
    // scrollbar_thumb_hover_background
    // scrollbar_thumb_border
    // scrollbar_track_background
    // scrollbar_track_border
    // scrollbar_status_opacity
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
