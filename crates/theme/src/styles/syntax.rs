use std::sync::Arc;

use gpui::{HighlightStyle, Hsla};

use crate::{
    blue, cyan, gold, indigo, iris, jade, lime, mint, neutral, orange, plum, purple, red, sky,
    tomato, yellow,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
}

impl SyntaxTheme {
    pub fn light() -> Self {
        Self {
            highlights: vec![
                ("attribute".into(), cyan().light().step_11().into()),
                ("boolean".into(), tomato().light().step_11().into()),
                ("comment".into(), neutral().light().step_10().into()),
                ("comment.doc".into(), iris().light().step_11().into()),
                ("constant".into(), red().light().step_9().into()),
                ("constructor".into(), red().light().step_9().into()),
                ("embedded".into(), red().light().step_9().into()),
                ("emphasis".into(), red().light().step_9().into()),
                ("emphasis.strong".into(), red().light().step_9().into()),
                ("enum".into(), red().light().step_9().into()),
                ("function".into(), red().light().step_9().into()),
                ("hint".into(), red().light().step_9().into()),
                ("keyword".into(), orange().light().step_9().into()),
                ("label".into(), red().light().step_9().into()),
                ("link_text".into(), red().light().step_9().into()),
                ("link_uri".into(), red().light().step_9().into()),
                ("number".into(), purple().light().step_10().into()),
                ("operator".into(), red().light().step_9().into()),
                ("predictive".into(), red().light().step_9().into()),
                ("preproc".into(), red().light().step_9().into()),
                ("primary".into(), red().light().step_9().into()),
                ("property".into(), red().light().step_9().into()),
                ("punctuation".into(), neutral().light().step_11().into()),
                (
                    "punctuation.bracket".into(),
                    neutral().light().step_11().into(),
                ),
                (
                    "punctuation.delimiter".into(),
                    neutral().light().step_10().into(),
                ),
                (
                    "punctuation.list_marker".into(),
                    blue().light().step_11().into(),
                ),
                ("punctuation.special".into(), red().light().step_9().into()),
                ("string".into(), jade().light().step_9().into()),
                ("string.escape".into(), red().light().step_9().into()),
                ("string.regex".into(), tomato().light().step_9().into()),
                ("string.special".into(), red().light().step_9().into()),
                (
                    "string.special.symbol".into(),
                    red().light().step_9().into(),
                ),
                ("tag".into(), red().light().step_9().into()),
                ("text.literal".into(), red().light().step_9().into()),
                ("title".into(), red().light().step_9().into()),
                ("type".into(), cyan().light().step_9().into()),
                ("variable".into(), red().light().step_9().into()),
                ("variable.special".into(), red().light().step_9().into()),
                ("variant".into(), red().light().step_9().into()),
            ],
        }
    }

    pub fn dark() -> Self {
        Self {
            highlights: vec![
                ("attribute".into(), tomato().dark().step_11().into()),
                ("boolean".into(), tomato().dark().step_11().into()),
                ("comment".into(), neutral().dark().step_11().into()),
                ("comment.doc".into(), iris().dark().step_12().into()),
                ("constant".into(), orange().dark().step_11().into()),
                ("constructor".into(), gold().dark().step_11().into()),
                ("embedded".into(), red().dark().step_11().into()),
                ("emphasis".into(), red().dark().step_11().into()),
                ("emphasis.strong".into(), red().dark().step_11().into()),
                ("enum".into(), yellow().dark().step_11().into()),
                ("function".into(), blue().dark().step_11().into()),
                ("hint".into(), indigo().dark().step_11().into()),
                ("keyword".into(), plum().dark().step_11().into()),
                ("label".into(), red().dark().step_11().into()),
                ("link_text".into(), red().dark().step_11().into()),
                ("link_uri".into(), red().dark().step_11().into()),
                ("number".into(), red().dark().step_11().into()),
                ("operator".into(), red().dark().step_11().into()),
                ("predictive".into(), red().dark().step_11().into()),
                ("preproc".into(), red().dark().step_11().into()),
                ("primary".into(), red().dark().step_11().into()),
                ("property".into(), red().dark().step_11().into()),
                ("punctuation".into(), neutral().dark().step_11().into()),
                (
                    "punctuation.bracket".into(),
                    neutral().dark().step_11().into(),
                ),
                (
                    "punctuation.delimiter".into(),
                    neutral().dark().step_11().into(),
                ),
                (
                    "punctuation.list_marker".into(),
                    blue().dark().step_11().into(),
                ),
                ("punctuation.special".into(), red().dark().step_11().into()),
                ("string".into(), lime().dark().step_11().into()),
                ("string.escape".into(), orange().dark().step_11().into()),
                ("string.regex".into(), tomato().dark().step_11().into()),
                ("string.special".into(), red().dark().step_11().into()),
                (
                    "string.special.symbol".into(),
                    red().dark().step_11().into(),
                ),
                ("tag".into(), red().dark().step_11().into()),
                ("text.literal".into(), purple().dark().step_11().into()),
                ("title".into(), sky().dark().step_11().into()),
                ("type".into(), mint().dark().step_11().into()),
                ("variable".into(), red().dark().step_11().into()),
                ("variable.special".into(), red().dark().step_11().into()),
                ("variant".into(), red().dark().step_11().into()),
            ],
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn new_test(colors: impl IntoIterator<Item = (&'static str, Hsla)>) -> Self {
        Self::new_test_styles(colors.into_iter().map(|(key, color)| {
            (
                key,
                HighlightStyle {
                    color: Some(color),
                    ..Default::default()
                },
            )
        }))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn new_test_styles(
        colors: impl IntoIterator<Item = (&'static str, HighlightStyle)>,
    ) -> Self {
        Self {
            highlights: colors
                .into_iter()
                .map(|(key, style)| (key.to_owned(), style))
                .collect(),
        }
    }

    pub fn get(&self, name: &str) -> HighlightStyle {
        self.highlights
            .iter()
            .find_map(|entry| if entry.0 == name { Some(entry.1) } else { None })
            .unwrap_or_default()
    }

    pub fn color(&self, name: &str) -> Hsla {
        self.get(name).color.unwrap_or_default()
    }

    /// Returns a new [`Arc<SyntaxTheme>`] with the given syntax styles merged in.
    pub fn merge(base: Arc<Self>, user_syntax_styles: Vec<(String, HighlightStyle)>) -> Arc<Self> {
        if user_syntax_styles.is_empty() {
            return base;
        }

        let mut merged_highlights = base.highlights.clone();

        for (name, highlight) in user_syntax_styles {
            if let Some((_, existing_highlight)) = merged_highlights
                .iter_mut()
                .find(|(existing_name, _)| existing_name == &name)
            {
                existing_highlight.color = highlight.color.or(existing_highlight.color);
                existing_highlight.font_weight =
                    highlight.font_weight.or(existing_highlight.font_weight);
                existing_highlight.font_style =
                    highlight.font_style.or(existing_highlight.font_style);
                existing_highlight.background_color = highlight
                    .background_color
                    .or(existing_highlight.background_color);
                existing_highlight.underline = highlight.underline.or(existing_highlight.underline);
                existing_highlight.strikethrough =
                    highlight.strikethrough.or(existing_highlight.strikethrough);
                existing_highlight.fade_out = highlight.fade_out.or(existing_highlight.fade_out);
            } else {
                merged_highlights.push((name, highlight));
            }
        }

        Arc::new(Self {
            highlights: merged_highlights,
        })
    }
}

#[cfg(test)]
mod tests {
    use gpui::FontStyle;

    use super::*;

    #[test]
    fn test_syntax_theme_merge() {
        // Merging into an empty `SyntaxTheme` keeps all the user-defined styles.
        let syntax_theme = SyntaxTheme::merge(
            Arc::new(SyntaxTheme::new_test([])),
            vec![
                (
                    "foo".to_string(),
                    HighlightStyle {
                        color: Some(gpui::red()),
                        ..Default::default()
                    },
                ),
                (
                    "foo.bar".to_string(),
                    HighlightStyle {
                        color: Some(gpui::green()),
                        ..Default::default()
                    },
                ),
            ],
        );
        assert_eq!(
            syntax_theme,
            Arc::new(SyntaxTheme::new_test([
                ("foo", gpui::red()),
                ("foo.bar", gpui::green())
            ]))
        );

        // Merging empty user-defined styles keeps all the base styles.
        let syntax_theme = SyntaxTheme::merge(
            Arc::new(SyntaxTheme::new_test([
                ("foo", gpui::blue()),
                ("foo.bar", gpui::red()),
            ])),
            Vec::new(),
        );
        assert_eq!(
            syntax_theme,
            Arc::new(SyntaxTheme::new_test([
                ("foo", gpui::blue()),
                ("foo.bar", gpui::red())
            ]))
        );

        let syntax_theme = SyntaxTheme::merge(
            Arc::new(SyntaxTheme::new_test([
                ("foo", gpui::red()),
                ("foo.bar", gpui::green()),
            ])),
            vec![(
                "foo.bar".to_string(),
                HighlightStyle {
                    color: Some(gpui::yellow()),
                    ..Default::default()
                },
            )],
        );
        assert_eq!(
            syntax_theme,
            Arc::new(SyntaxTheme::new_test([
                ("foo", gpui::red()),
                ("foo.bar", gpui::yellow())
            ]))
        );

        let syntax_theme = SyntaxTheme::merge(
            Arc::new(SyntaxTheme::new_test([
                ("foo", gpui::red()),
                ("foo.bar", gpui::green()),
            ])),
            vec![(
                "foo.bar".to_string(),
                HighlightStyle {
                    font_style: Some(FontStyle::Italic),
                    ..Default::default()
                },
            )],
        );
        assert_eq!(
            syntax_theme,
            Arc::new(SyntaxTheme::new_test_styles([
                (
                    "foo",
                    HighlightStyle {
                        color: Some(gpui::red()),
                        ..Default::default()
                    }
                ),
                (
                    "foo.bar",
                    HighlightStyle {
                        color: Some(gpui::green()),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                )
            ]))
        );
    }
}
