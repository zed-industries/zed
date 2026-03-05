#![allow(missing_docs)]

use std::{borrow::Borrow, collections::BTreeMap, sync::Arc};

use gpui::{HighlightStyle, Hsla};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SyntaxTheme {
    pub highlights: Vec<(String, HighlightStyle)>,
    capture_name_map: BTreeMap<SmallVec<[CaptureName<'static>; 2]>, usize>,
}

#[derive(Clone, Debug, Ord, Eq)]
enum CaptureName<'a> {
    Borrowed(&'a str),
    Owned(Arc<str>),
}

impl<'other> PartialEq<CaptureName<'other>> for CaptureName<'_> {
    fn eq(&self, other: &CaptureName<'other>) -> bool {
        let lhs = self.as_ref();
        let rhs = other.as_ref();
        lhs == rhs
    }
}

impl<'other> PartialOrd<CaptureName<'other>> for CaptureName<'_> {
    fn partial_cmp(&self, other: &CaptureName<'other>) -> Option<std::cmp::Ordering> {
        let lhs = self.as_ref();
        let rhs = other.as_ref();
        lhs.partial_cmp(rhs)
    }
}

impl Borrow<str> for CaptureName<'_> {
    fn borrow(&self) -> &str {
        match self {
            CaptureName::Borrowed(str) => str,
            CaptureName::Owned(str) => &str,
        }
    }
}

impl AsRef<str> for CaptureName<'_> {
    fn as_ref(&self) -> &str {
        match self {
            CaptureName::Borrowed(str) => str,
            CaptureName::Owned(str) => &str,
        }
    }
}

impl SyntaxTheme {
    pub fn new(highlights: Vec<(String, HighlightStyle)>) -> Self {
        Self {
            capture_name_map: Self::create_capture_name_map(&highlights),
            highlights,
        }
    }

    fn create_capture_name_map(
        highlights: &[(String, HighlightStyle)],
    ) -> BTreeMap<SmallVec<[CaptureName<'static>; 2]>, usize> {
        highlights
            .iter()
            .enumerate()
            .map(|(i, (key, _))| {
                (
                    key.split('.')
                        .map(|component| CaptureName::Owned(Arc::from(component)))
                        .collect(),
                    i,
                )
            })
            .collect()
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
        Self::new(
            colors
                .into_iter()
                .map(|(key, style)| (key.to_owned(), style))
                .collect(),
        )
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

    pub fn highlight_id<'a>(&self, name: &'a str) -> Option<u32> {
        let capture_name = name
            .split('.')
            .map(CaptureName::Borrowed)
            .collect::<SmallVec<[CaptureName<'a>; 2]>>();
        // SAFETY: We're extending the slice lifetime to 'static solely for the
        // range key comparison. The reference doesn't escape this call and the
        // data is valid for 'a which outlives it.
        let capture_name_static: SmallVec<[CaptureName<'static>; 2]> =
            unsafe { std::mem::transmute(capture_name) };
        self.capture_name_map
            .range::<[CaptureName<'static>], _>((
                std::ops::Bound::Unbounded,
                std::ops::Bound::Included(capture_name_static.as_slice()),
            ))
            .collect::<Vec<_>>()
            .into_iter()
            .rfind(|(prefix, _)| {
                for (lhs, rhs) in capture_name_static.iter().zip(prefix.into_iter()) {
                    if lhs != rhs {
                        return false;
                    }
                }
                true
            })
            .map(|(_, index)| *index as u32)
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

        Arc::new(Self::new(merged_highlights))
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
