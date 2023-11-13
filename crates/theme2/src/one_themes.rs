use std::sync::Arc;

use gpui::{hsla, rgba};

use crate::{
    black, blue, cyan, default_color_scales, green, neutral, red, violet, yellow, Appearance,
    PlayerColors, StatusColors, SyntaxTheme, SystemColors, Theme, ThemeColors, ThemeFamily,
    ThemeStyles,
};

pub fn one_family() -> ThemeFamily {
    ThemeFamily {
        id: "one".to_string(),
        name: "One".into(),
        author: "".into(),
        themes: vec![one_dark()],
        scales: default_color_scales(),
    }
}

pub(crate) fn one_dark() -> Theme {
    // let bg = rgba(0x22252A).into();
    // let editor = rgba(0x292C33).into();

    let bg = hsla(218. / 360., 11. / 100., 15. / 100., 1.);
    let editor = hsla(222. / 360., 11. / 100., 18. / 100., 1.);

    Theme {
        id: "one_dark".to_string(),
        name: "One Dark".into(),
        appearance: Appearance::Dark,
        styles: ThemeStyles {
            system: SystemColors::default(),
            colors: ThemeColors {
                border: hsla(225. / 360., 13. / 100., 12. / 100., 1.),
                border_variant: hsla(228. / 360., 8. / 100., 25. / 100., 1.),
                border_focused: hsla(223. / 360., 78. / 100., 65. / 100., 1.),
                border_selected: hsla(222.6 / 360., 77.5 / 100., 65.1 / 100., 1.0),
                border_transparent: SystemColors::default().transparent,
                border_disabled: hsla(222.0 / 360., 11.6 / 100., 33.7 / 100., 1.0),
                elevated_surface_background: bg,
                surface_background: bg,
                background: bg,
                element_background: hsla(222.9 / 360., 11.1 / 100., 24.7 / 100., 1.0),
                element_hover: hsla(225.0 / 360., 11.8 / 100., 26.7 / 100., 1.0),
                element_active: hsla(220.0 / 360., 11.8 / 100., 20.0 / 100., 1.0),
                element_selected: hsla(224.0 / 360., 11.3 / 100., 26.1 / 100., 1.0),
                element_disabled: hsla(224.0 / 360., 11.3 / 100., 26.1 / 100., 1.0),
                drop_target_background: hsla(220.0 / 360., 8.3 / 100., 21.4 / 100., 1.0),
                ghost_element_background: SystemColors::default().transparent,
                ghost_element_hover: hsla(225.0 / 360., 11.8 / 100., 26.7 / 100., 1.0),
                ghost_element_active: hsla(220.0 / 360., 11.8 / 100., 20.0 / 100., 1.0),
                ghost_element_selected: hsla(224.0 / 360., 11.3 / 100., 26.1 / 100., 1.0),
                ghost_element_disabled: hsla(224.0 / 360., 11.3 / 100., 26.1 / 100., 1.0),
                text: hsla(222.9 / 360., 9.1 / 100., 84.9 / 100., 1.0),
                text_muted: hsla(220.0 / 360., 6.4 / 100., 45.7 / 100., 1.0),
                text_placeholder: hsla(220.0 / 360., 6.6 / 100., 44.5 / 100., 1.0),
                text_disabled: hsla(220.0 / 360., 6.6 / 100., 44.5 / 100., 1.0),
                text_accent: hsla(222.6 / 360., 77.5 / 100., 65.1 / 100., 1.0),
                icon: hsla(222.9 / 360., 9.9 / 100., 86.1 / 100., 1.0),
                icon_muted: hsla(220.0 / 360., 12.1 / 100., 66.1 / 100., 1.0),
                icon_disabled: hsla(220.0 / 360., 6.4 / 100., 45.7 / 100., 1.0),
                icon_placeholder: hsla(220.0 / 360., 6.4 / 100., 45.7 / 100., 1.0),
                icon_accent: hsla(222.6 / 360., 77.5 / 100., 65.1 / 100., 1.0),
                status_bar_background: bg,
                title_bar_background: bg,
                toolbar_background: editor,
                tab_bar_background: bg,
                tab_inactive_background: bg,
                tab_active_background: editor,
                editor_background: editor,
                editor_gutter_background: editor,
                editor_subheader_background: bg,
                editor_active_line_background: hsla(222.9 / 360., 13.5 / 100., 20.4 / 100., 1.0),
                editor_highlighted_line_background: gpui::red(),
                editor_line_number: hsla(222.0 / 360., 11.5 / 100., 34.1 / 100., 1.0),
                editor_active_line_number: hsla(216.0 / 360., 5.9 / 100., 49.6 / 100., 1.0),
                editor_invisible: hsla(222.0 / 360., 11.5 / 100., 34.1 / 100., 1.0),
                editor_wrap_guide: gpui::red(),
                editor_active_wrap_guide: gpui::red(),
                editor_document_highlight_read_background: gpui::red(),
                editor_document_highlight_write_background: gpui::red(),
                terminal_background: bg,
                // todo!("Use one colors for terminal")
                terminal_ansi_black: black().dark().step_12(),
                terminal_ansi_red: red().dark().step_11(),
                terminal_ansi_green: green().dark().step_11(),
                terminal_ansi_yellow: yellow().dark().step_11(),
                terminal_ansi_blue: blue().dark().step_11(),
                terminal_ansi_magenta: violet().dark().step_11(),
                terminal_ansi_cyan: cyan().dark().step_11(),
                terminal_ansi_white: neutral().dark().step_12(),
                terminal_ansi_bright_black: black().dark().step_11(),
                terminal_ansi_bright_red: red().dark().step_10(),
                terminal_ansi_bright_green: green().dark().step_10(),
                terminal_ansi_bright_yellow: yellow().dark().step_10(),
                terminal_ansi_bright_blue: blue().dark().step_10(),
                terminal_ansi_bright_magenta: violet().dark().step_10(),
                terminal_ansi_bright_cyan: cyan().dark().step_10(),
                terminal_ansi_bright_white: neutral().dark().step_11(),
            },
            status: StatusColors::dark(),
            player: PlayerColors::dark(),
            syntax: Arc::new(SyntaxTheme::dark()),
        },
    }
}
