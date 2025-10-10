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
        &[
            0xFF6B6B, // Red
            0xFFB946, // Orange  
            0xF9E076, // Yellow
            0x6BCF7F, // Green
            0x4ECDC4, // Cyan
            0x45B7D1, // Blue
            0x9B59B6, // Purple
            0xE91E63, // Magenta
            0xFF8C94, // Pink
            0xFFD93D, // Gold
            0x6BCB77, // Lime
            0x4D96FF, // Sky Blue
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

    #[test]
    fn test_rainbow_color_returns_style() {
        let theme = SyntaxTheme::default();
        let palette_size = theme.rainbow_palette_size();
        
        assert_eq!(palette_size, 12, "Default palette should have 12 colors");
        
        for i in 0..palette_size {
            let style = theme.rainbow_color(i);
            assert!(style.is_some(), "Should always return a style (theme or fallback)");
            
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
        
        assert_eq!(colors.len(), 12, "Should have 12 colors");
        let unique_count = colors.iter().collect::<std::collections::HashSet<_>>().len();
        assert_eq!(unique_count, 12, "All 12 colors should be distinct");
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
        
        assert_eq!(theme.rainbow_palette_size(), 3, "Should detect 3 theme-defined colors");
    }
    
    #[test]
    fn test_rainbow_palette_size_fallback() {
        let theme = SyntaxTheme::default();
        assert_eq!(theme.rainbow_palette_size(), 12, "Should use fallback palette size of 12");
    }
}
