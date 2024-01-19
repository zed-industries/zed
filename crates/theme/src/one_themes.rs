use std::sync::Arc;

use gpui::{hsla, FontStyle, FontWeight, HighlightStyle};

use crate::{
    default_color_scales, Appearance, PlayerColors, StatusColors, SyntaxTheme, SystemColors, Theme,
    ThemeColors, ThemeFamily, ThemeStyles,
};

// Note: This theme family is not the one you see in Zed at the moment.
// This is a from-scratch rebuild that Nate started work on. We currently
// only use this in the tests, and the One family from the `themes/` directory
// is what gets loaded into Zed when running it.
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
    let bg = hsla(215. / 360., 12. / 100., 15. / 100., 1.);
    let editor = hsla(220. / 360., 12. / 100., 18. / 100., 1.);
    let elevated_surface = hsla(225. / 360., 12. / 100., 17. / 100., 1.);

    let blue = hsla(207.8 / 360., 81. / 100., 66. / 100., 1.0);
    let gray = hsla(218.8 / 360., 10. / 100., 40. / 100., 1.0);
    let green = hsla(95. / 360., 38. / 100., 62. / 100., 1.0);
    let orange = hsla(29. / 360., 54. / 100., 61. / 100., 1.0);
    let purple = hsla(286. / 360., 51. / 100., 64. / 100., 1.0);
    let red = hsla(355. / 360., 65. / 100., 65. / 100., 1.0);
    let teal = hsla(187. / 360., 47. / 100., 55. / 100., 1.0);
    let yellow = hsla(39. / 360., 67. / 100., 69. / 100., 1.0);

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
                elevated_surface_background: elevated_surface,
                surface_background: bg,
                background: bg,
                element_background: hsla(223.0 / 360., 13. / 100., 21. / 100., 1.0),
                element_hover: hsla(225.0 / 360., 11.8 / 100., 26.7 / 100., 1.0),
                element_active: hsla(220.0 / 360., 11.8 / 100., 20.0 / 100., 1.0),
                element_selected: hsla(224.0 / 360., 11.3 / 100., 26.1 / 100., 1.0),
                element_disabled: SystemColors::default().transparent,
                drop_target_background: hsla(220.0 / 360., 8.3 / 100., 21.4 / 100., 1.0),
                ghost_element_background: SystemColors::default().transparent,
                ghost_element_hover: hsla(225.0 / 360., 11.8 / 100., 26.7 / 100., 1.0),
                ghost_element_active: hsla(220.0 / 360., 11.8 / 100., 20.0 / 100., 1.0),
                ghost_element_selected: hsla(224.0 / 360., 11.3 / 100., 26.1 / 100., 1.0),
                ghost_element_disabled: SystemColors::default().transparent,
                text: hsla(221. / 360., 11. / 100., 86. / 100., 1.0),
                text_muted: hsla(218.0 / 360., 7. / 100., 46. / 100., 1.0),
                text_placeholder: hsla(220.0 / 360., 6.6 / 100., 44.5 / 100., 1.0),
                text_disabled: hsla(220.0 / 360., 6.6 / 100., 44.5 / 100., 1.0),
                text_accent: hsla(222.6 / 360., 77.5 / 100., 65.1 / 100., 1.0),
                icon: hsla(222.9 / 360., 9.9 / 100., 86.1 / 100., 1.0),
                icon_muted: hsla(220.0 / 360., 12.1 / 100., 66.1 / 100., 1.0),
                icon_disabled: hsla(220.0 / 360., 6.4 / 100., 45.7 / 100., 1.0),
                icon_placeholder: hsla(220.0 / 360., 6.4 / 100., 45.7 / 100., 1.0),
                icon_accent: blue.into(),
                status_bar_background: bg,
                title_bar_background: bg,
                toolbar_background: editor,
                tab_bar_background: bg,
                tab_inactive_background: bg,
                tab_active_background: editor,
                search_match_background: bg,

                editor_background: editor,
                editor_gutter_background: editor,
                editor_subheader_background: bg,
                editor_active_line_background: hsla(222.9 / 360., 13.5 / 100., 20.4 / 100., 1.0),
                editor_highlighted_line_background: hsla(207.8 / 360., 81. / 100., 66. / 100., 0.1),
                editor_line_number: hsla(222.0 / 360., 11.5 / 100., 34.1 / 100., 1.0),
                editor_active_line_number: hsla(216.0 / 360., 5.9 / 100., 49.6 / 100., 1.0),
                editor_invisible: hsla(222.0 / 360., 11.5 / 100., 34.1 / 100., 1.0),
                editor_wrap_guide: hsla(228. / 360., 8. / 100., 25. / 100., 1.),
                editor_active_wrap_guide: hsla(228. / 360., 8. / 100., 25. / 100., 1.),
                editor_document_highlight_read_background: hsla(
                    207.8 / 360.,
                    81. / 100.,
                    66. / 100.,
                    0.2,
                ),
                editor_document_highlight_write_background: gpui::red(),

                terminal_background: bg,
                // todo!("Use one colors for terminal")
                terminal_foreground: crate::white().dark().step_12(),
                terminal_bright_foreground: crate::white().dark().step_11(),
                terminal_dim_foreground: crate::white().dark().step_10(),
                terminal_ansi_black: crate::black().dark().step_12(),
                terminal_ansi_red: crate::red().dark().step_11(),
                terminal_ansi_green: crate::green().dark().step_11(),
                terminal_ansi_yellow: crate::yellow().dark().step_11(),
                terminal_ansi_blue: crate::blue().dark().step_11(),
                terminal_ansi_magenta: crate::violet().dark().step_11(),
                terminal_ansi_cyan: crate::cyan().dark().step_11(),
                terminal_ansi_white: crate::neutral().dark().step_12(),
                terminal_ansi_bright_black: crate::black().dark().step_11(),
                terminal_ansi_bright_red: crate::red().dark().step_10(),
                terminal_ansi_bright_green: crate::green().dark().step_10(),
                terminal_ansi_bright_yellow: crate::yellow().dark().step_10(),
                terminal_ansi_bright_blue: crate::blue().dark().step_10(),
                terminal_ansi_bright_magenta: crate::violet().dark().step_10(),
                terminal_ansi_bright_cyan: crate::cyan().dark().step_10(),
                terminal_ansi_bright_white: crate::neutral().dark().step_11(),
                terminal_ansi_dim_black: crate::black().dark().step_10(),
                terminal_ansi_dim_red: crate::red().dark().step_9(),
                terminal_ansi_dim_green: crate::green().dark().step_9(),
                terminal_ansi_dim_yellow: crate::yellow().dark().step_9(),
                terminal_ansi_dim_blue: crate::blue().dark().step_9(),
                terminal_ansi_dim_magenta: crate::violet().dark().step_9(),
                terminal_ansi_dim_cyan: crate::cyan().dark().step_9(),
                terminal_ansi_dim_white: crate::neutral().dark().step_10(),
                panel_background: bg,
                panel_focused_border: blue,
                pane_focused_border: blue,
                scrollbar_thumb_background: gpui::transparent_black(),
                scrollbar_thumb_hover_background: hsla(225.0 / 360., 11.8 / 100., 26.7 / 100., 1.0),
                scrollbar_thumb_border: hsla(228. / 360., 8. / 100., 25. / 100., 1.),
                scrollbar_track_background: gpui::transparent_black(),
                scrollbar_track_border: hsla(228. / 360., 8. / 100., 25. / 100., 1.),
                editor_foreground: hsla(218. / 360., 14. / 100., 71. / 100., 1.),
                link_text_hover: blue,
            },
            status: StatusColors {
                conflict: yellow,
                conflict_background: yellow,
                conflict_border: yellow,
                created: green,
                created_background: green,
                created_border: green,
                deleted: red,
                deleted_background: red,
                deleted_border: red,
                error: red,
                error_background: red,
                error_border: red,
                hidden: gray,
                hidden_background: gray,
                hidden_border: gray,
                hint: blue,
                hint_background: blue,
                hint_border: blue,
                ignored: gray,
                ignored_background: gray,
                ignored_border: gray,
                info: blue,
                info_background: blue,
                info_border: blue,
                modified: yellow,
                modified_background: yellow,
                modified_border: yellow,
                predictive: gray,
                predictive_background: gray,
                predictive_border: gray,
                renamed: blue,
                renamed_background: blue,
                renamed_border: blue,
                success: green,
                success_background: green,
                success_border: green,
                unreachable: gray,
                unreachable_background: gray,
                unreachable_border: gray,
                warning: yellow,
                warning_background: yellow,
                warning_border: yellow,
            },
            player: PlayerColors::dark(),
            syntax: Arc::new(SyntaxTheme {
                highlights: vec![
                    ("attribute".into(), purple.into()),
                    ("boolean".into(), orange.into()),
                    ("comment".into(), gray.into()),
                    ("comment.doc".into(), gray.into()),
                    ("constant".into(), yellow.into()),
                    ("constructor".into(), blue.into()),
                    ("embedded".into(), HighlightStyle::default()),
                    (
                        "emphasis".into(),
                        HighlightStyle {
                            font_style: Some(FontStyle::Italic),
                            ..HighlightStyle::default()
                        },
                    ),
                    (
                        "emphasis.strong".into(),
                        HighlightStyle {
                            font_weight: Some(FontWeight::BOLD),
                            ..HighlightStyle::default()
                        },
                    ),
                    ("enum".into(), HighlightStyle::default()),
                    ("function".into(), blue.into()),
                    ("function.method".into(), blue.into()),
                    ("function.definition".into(), blue.into()),
                    ("hint".into(), blue.into()),
                    ("keyword".into(), purple.into()),
                    ("label".into(), HighlightStyle::default()),
                    ("link_text".into(), blue.into()),
                    (
                        "link_uri".into(),
                        HighlightStyle {
                            color: Some(teal.into()),
                            font_style: Some(FontStyle::Italic),
                            ..HighlightStyle::default()
                        },
                    ),
                    ("number".into(), orange.into()),
                    ("operator".into(), HighlightStyle::default()),
                    ("predictive".into(), HighlightStyle::default()),
                    ("preproc".into(), HighlightStyle::default()),
                    ("primary".into(), HighlightStyle::default()),
                    ("property".into(), red.into()),
                    ("punctuation".into(), HighlightStyle::default()),
                    ("punctuation.bracket".into(), HighlightStyle::default()),
                    ("punctuation.delimiter".into(), HighlightStyle::default()),
                    ("punctuation.list_marker".into(), HighlightStyle::default()),
                    ("punctuation.special".into(), HighlightStyle::default()),
                    ("string".into(), green.into()),
                    ("string.escape".into(), HighlightStyle::default()),
                    ("string.regex".into(), red.into()),
                    ("string.special".into(), HighlightStyle::default()),
                    ("string.special.symbol".into(), HighlightStyle::default()),
                    ("tag".into(), HighlightStyle::default()),
                    ("text.literal".into(), HighlightStyle::default()),
                    ("title".into(), HighlightStyle::default()),
                    ("type".into(), teal.into()),
                    ("variable".into(), HighlightStyle::default()),
                    ("variable.special".into(), red.into()),
                    ("variant".into(), HighlightStyle::default()),
                ],
            }),
            accents: vec![blue, orange, purple, teal],
        },
    }
}
