#![allow(missing_docs)]

use std::sync::Arc;

use gpui::{HighlightStyle, Hsla};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
}

impl SyntaxTheme {
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

    pub fn get_opt(&self, name: &str) -> Option<HighlightStyle> {
        self.highlights
            .iter()
            .find_map(|entry| if entry.0 == name { Some(entry.1) } else { None })
    }

    pub fn color(&self, name: &str) -> Hsla {
        self.get(name).color.unwrap_or_default()
    }

    pub fn highlight_id(&self, name: &str) -> Option<u32> {
        let ix = self.highlights.iter().position(|entry| entry.0 == name)?;
        Some(ix as u32)
    }

    pub fn rainbow_palette_size(&self) -> usize {
        const MAX_RAINBOW_COLORS: usize = 32;

        let mut count = 0;
        for i in 0..MAX_RAINBOW_COLORS {
            let key = format!("variable.rainbow.{}", i);
            if self.get_opt(&key).is_none() {
                break;
            }
            count = i + 1;
        }

        if count == 0 {
            Self::fallback_rainbow_palette().len()
        } else {
            count
        }
    }

    pub fn rainbow_color(&self, index: usize) -> Option<HighlightStyle> {
        let key = format!("variable.rainbow.{}", index);
        if let Some(style) = self.get_opt(&key) {
            return Some(style);
        }

        Some(Self::generate_fallback_rainbow_color(index))
    }

    const fn fallback_rainbow_palette() -> &'static [u32] {
        // Pastel palette with enhanced saturation (32 colors)
        // - More saturated but still milky/soft
        // - Excludes red/reddish (error confusion), white, black, grey
        // - Balanced brightness with stronger color presence
        // - Well-distributed across color spectrum
        // - Similar colors separated by at least 2 indices to prevent clustering
        &[
            0x90ffa0, // 0: Saturated mint green
            0xd090ff, // 1: Saturated purple
            0xffb070, // 2: Saturated peach
            0x70a8ff, // 3: Saturated blue
            0xfff070, // 4: Saturated yellow
            0x60f8d0, // 5: Saturated aqua
            0xffa0e8, // 6: Saturated pink
            0x50c8ff, // 7: Saturated sky blue
            0xffc060, // 8: Saturated golden
            0xc8a8ff, // 9: Saturated lavender
            0x70f0d8, // 10: Saturated teal
            0xe898ff, // 11: Saturated lilac
            0xff9850, // 12: Saturated apricot
            0x60d8ff, // 13: Saturated cyan
            0xa8ff70, // 14: Saturated lime
            0xc070ff, // 15: Saturated violet
            0xf0d070, // 16: Saturated gold
            0x50b8ff, // 17: Saturated ocean
            0x80f8d0, // 18: Saturated mint teal
            0xf0c0ff, // 19: Saturated pale lavender
            0xffb0d8, // 20: Saturated light pink
            0x98b0ff, // 21: Saturated periwinkle
            0x60d0b8, // 22: Saturated seafoam
            0xd8a0ff, // 23: Saturated light violet
            0xffe090, // 24: Saturated cream
            0x70d0f0, // 25: Saturated light cyan
            0x90e0f0, // 26: Saturated powder cyan
            0xff98c0, // 27: Saturated rose
            0xa0b8ff, // 28: Saturated light sky
            0xe0c8ff, // 29: Saturated very light lavender
            0x80d0ff, // 30: Saturated light blue
            0xa8b8e8, // 31: Saturated pale periwinkle
        ]
    }

    fn generate_fallback_rainbow_color(index: usize) -> HighlightStyle {
        let palette = Self::fallback_rainbow_palette();
        let color_value = palette[index % palette.len()];
        let rgba = gpui::rgb(color_value);

        HighlightStyle {
            color: Some(rgba.into()),
            ..Default::default()
        }
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
                ("foo.bar", gpui::green()),
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
                ("foo.bar", gpui::red()),
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
                ("foo.bar", gpui::yellow()),
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
                    },
                ),
                (
                    "foo.bar",
                    HighlightStyle {
                        color: Some(gpui::green()),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    },
                ),
            ]))
        );
    }

    #[test]
    fn test_rainbow_color_returns_style() {
        let theme = SyntaxTheme::default();
        let palette_size = theme.rainbow_palette_size();

        assert_eq!(palette_size, 32, "Default palette should have 32 colors");

        for i in 0..palette_size {
            let style = theme.rainbow_color(i);
            assert!(
                style.is_some(),
                "Should always return a style (theme or fallback)"
            );

            if let Some(s) = style {
                assert!(s.color.is_some(), "Style should have a color");
            }
        }
    }

    #[test]
    fn test_fallback_colors_distinct() {
        let theme = SyntaxTheme::default();
        let palette_size = theme.rainbow_palette_size();
        let mut colors = Vec::new();

        for i in 0..palette_size {
            if let Some(style) = theme.rainbow_color(i) {
                if let Some(color) = style.color {
                    colors.push(format!("{:?}", color));
                }
            }
        }

        assert_eq!(colors.len(), 32, "Should have 32 colors");
        let unique_count = colors
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(unique_count, 32, "All 32 colors should be distinct");
    }

    #[test]
    fn test_rainbow_color_deterministic() {
        let theme = SyntaxTheme::default();

        let style1 = theme.rainbow_color(3);
        let style2 = theme.rainbow_color(3);

        assert_eq!(style1, style2, "Same index should return same color");
    }

    #[test]
    fn test_rainbow_palette_size_with_theme_colors() {
        // Test with theme that defines custom rainbow colors
        let theme = SyntaxTheme::new_test([
            ("variable.rainbow.0", gpui::red()),
            ("variable.rainbow.1", gpui::green()),
            ("variable.rainbow.2", gpui::blue()),
        ]);

        assert_eq!(
            theme.rainbow_palette_size(),
            3,
            "Should detect 3 theme-defined colors"
        );
    }

    #[test]
    fn test_rainbow_palette_size_fallback() {
        let theme = SyntaxTheme::default();
        assert_eq!(
            theme.rainbow_palette_size(),
            32,
            "Should use fallback palette size of 32"
        );
    }
}
