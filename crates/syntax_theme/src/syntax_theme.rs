#![allow(missing_docs)]

use std::{
    collections::{BTreeMap, btree_map::Entry},
    sync::Arc,
};

use gpui::HighlightStyle;
#[cfg(any(test, feature = "test-support"))]
use gpui::Hsla;
use parking_lot::RwLock;
use syntax_token::SyntaxTokenId;

#[derive(Debug, Default)]
pub struct SyntaxTheme {
    highlights: Vec<HighlightStyle>,
    capture_name_map: BTreeMap<String, usize>,
    /// Memoized `SyntaxTokenId` -> index into `highlights`, filled on first use.
    ///
    /// Resolving a token walks `capture_name_map` for the longest matching
    /// prefix, which is far too costly to repeat for every span on every frame.
    /// The outer `Option` marks a slot as not yet resolved; the inner one marks
    /// a token this theme does not style.
    resolved_indices: RwLock<Vec<Option<Option<usize>>>>,
}

impl Clone for SyntaxTheme {
    fn clone(&self) -> Self {
        Self {
            highlights: self.highlights.clone(),
            capture_name_map: self.capture_name_map.clone(),
            resolved_indices: RwLock::new(self.resolved_indices.read().clone()),
        }
    }
}

/// Two themes are equal when they style capture names identically; the memo is
/// derived state and never distinguishes them.
impl PartialEq for SyntaxTheme {
    fn eq(&self, other: &Self) -> bool {
        self.highlights == other.highlights && self.capture_name_map == other.capture_name_map
    }
}

impl Eq for SyntaxTheme {}

impl SyntaxTheme {
    pub fn new(highlights: impl IntoIterator<Item = (String, HighlightStyle)>) -> Self {
        let (capture_names, highlights) = highlights.into_iter().unzip();

        Self {
            capture_name_map: Self::create_capture_name_map(capture_names),
            highlights,
            resolved_indices: RwLock::default(),
        }
    }

    fn create_capture_name_map(highlights: Vec<String>) -> BTreeMap<String, usize> {
        highlights
            .into_iter()
            .enumerate()
            .map(|(i, key)| (key, i))
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
                .map(|(key, style)| (key.to_owned(), style)),
        )
    }

    /// Returns the style this theme gives `token`.
    ///
    /// Resolution goes through the token's capture name rather than a stored
    /// slot index, so the same token yields each theme's own styling. Two themes
    /// that define the same capture names in different orders assign them
    /// different slots, which is why an index minted against one theme must
    /// never be read against another.
    pub fn get(&self, token: SyntaxTokenId) -> Option<&HighlightStyle> {
        self.highlights.get(self.resolved_index(token)?)
    }

    /// Returns the style for the innermost capture this theme defines.
    ///
    /// `enclosing` holds the captures around `innermost`, outermost first. A
    /// grammar may give one node several captures so that a theme which does
    /// not define the most specific one still styles the node through a more
    /// general one.
    pub fn style_for_captures(
        &self,
        innermost: SyntaxTokenId,
        enclosing: &[SyntaxTokenId],
    ) -> Option<&HighlightStyle> {
        self.highlights
            .get(self.styled_index(innermost, enclosing)?)
    }

    /// Returns the name this theme styles the innermost matching capture under.
    ///
    /// The name may be shorter than the capture's own, since resolution falls
    /// back to the longest prefix the theme defines.
    pub fn capture_name_for_captures(
        &self,
        innermost: SyntaxTokenId,
        enclosing: &[SyntaxTokenId],
    ) -> Option<&str> {
        self.get_capture_name(self.styled_index(innermost, enclosing)?)
    }

    fn styled_index(&self, innermost: SyntaxTokenId, enclosing: &[SyntaxTokenId]) -> Option<usize> {
        self.resolved_index(innermost).or_else(|| {
            enclosing
                .iter()
                .rev()
                .find_map(|token| self.resolved_index(*token))
        })
    }

    fn resolved_index(&self, token: SyntaxTokenId) -> Option<usize> {
        let slot = token.index();
        {
            let memo = self.resolved_indices.read();
            if let Some(Some(resolved)) = memo.get(slot) {
                return *resolved;
            }
        }

        let resolved = syntax_token::name_for(token)
            .and_then(|capture_name| self.highlight_id(&capture_name))
            .map(|highlight_index| highlight_index as usize);

        let mut memo = self.resolved_indices.write();
        if memo.len() <= slot {
            memo.resize(slot + 1, None);
        }
        memo[slot] = Some(resolved);
        resolved
    }

    pub fn style_for_name(&self, name: &str) -> Option<HighlightStyle> {
        self.capture_name_map
            .get(name)
            .map(|highlight_idx| self.highlights[*highlight_idx])
    }

    pub fn get_capture_name(&self, idx: impl Into<usize>) -> Option<&str> {
        let idx = idx.into();
        self.capture_name_map
            .iter()
            .find(|(_, value)| **value == idx)
            .map(|(key, _)| key.as_ref())
    }

    pub fn highlight_id(&self, capture_name: &str) -> Option<u32> {
        self.capture_name_map
            .range::<str, _>((
                capture_name.split(".").next().map_or(
                    std::ops::Bound::Included(capture_name),
                    std::ops::Bound::Included,
                ),
                std::ops::Bound::Included(capture_name),
            ))
            .rfind(|(prefix, _)| {
                capture_name
                    .strip_prefix(*prefix)
                    .is_some_and(|remainder| remainder.is_empty() || remainder.starts_with('.'))
            })
            .map(|(_, index)| *index as u32)
    }

    /// Returns a new [`Arc<SyntaxTheme>`] with the given syntax styles merged in.
    pub fn merge(base: Arc<Self>, user_syntax_styles: Vec<(String, HighlightStyle)>) -> Arc<Self> {
        if user_syntax_styles.is_empty() {
            return base;
        }

        let mut base = Arc::try_unwrap(base).unwrap_or_else(|base| (*base).clone());

        for (name, highlight) in user_syntax_styles {
            match base.capture_name_map.entry(name) {
                Entry::Occupied(entry) => {
                    if let Some(existing_highlight) = base.highlights.get_mut(*entry.get()) {
                        existing_highlight.color = highlight.color.or(existing_highlight.color);
                        existing_highlight.font_weight =
                            highlight.font_weight.or(existing_highlight.font_weight);
                        existing_highlight.font_style =
                            highlight.font_style.or(existing_highlight.font_style);
                        existing_highlight.background_color = highlight
                            .background_color
                            .or(existing_highlight.background_color);
                        existing_highlight.underline =
                            highlight.underline.or(existing_highlight.underline);
                        existing_highlight.strikethrough =
                            highlight.strikethrough.or(existing_highlight.strikethrough);
                        existing_highlight.fade_out =
                            highlight.fade_out.or(existing_highlight.fade_out);
                    }
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(base.highlights.len());
                    base.highlights.push(highlight);
                }
            }
        }

        Arc::new(base)
    }
}

#[cfg(feature = "bundled-themes")]
mod bundled_themes {
    use std::collections::BTreeMap;
    use std::sync::Arc;

    use gpui::{FontStyle, FontWeight, HighlightStyle, Hsla, Rgba, rgb};
    use serde::Deserialize;

    use super::SyntaxTheme;

    #[derive(Deserialize)]
    struct ThemeFile {
        themes: Vec<ThemeEntry>,
    }

    #[derive(Deserialize)]
    struct ThemeEntry {
        name: String,
        style: ThemeStyle,
    }

    #[derive(Deserialize)]
    struct ThemeStyle {
        syntax: BTreeMap<String, SyntaxStyleEntry>,
    }

    #[derive(Deserialize)]
    struct SyntaxStyleEntry {
        color: Option<String>,
        font_weight: Option<f32>,
        font_style: Option<String>,
    }

    impl SyntaxStyleEntry {
        fn to_highlight_style(&self) -> HighlightStyle {
            HighlightStyle {
                color: self.color.as_deref().map(hex_to_hsla),
                font_weight: self.font_weight.map(FontWeight),
                font_style: self.font_style.as_deref().and_then(|s| match s {
                    "italic" => Some(FontStyle::Italic),
                    "normal" => Some(FontStyle::Normal),
                    "oblique" => Some(FontStyle::Oblique),
                    _ => None,
                }),
                ..Default::default()
            }
        }
    }

    fn hex_to_hsla(hex: &str) -> Hsla {
        let hex = hex.trim_start_matches('#');
        let rgba: Rgba = match hex.len() {
            6 => rgb(u32::from_str_radix(hex, 16).unwrap_or(0)),
            8 => {
                let value = u32::from_str_radix(hex, 16).unwrap_or(0);
                Rgba {
                    r: ((value >> 24) & 0xff) as f32 / 255.0,
                    g: ((value >> 16) & 0xff) as f32 / 255.0,
                    b: ((value >> 8) & 0xff) as f32 / 255.0,
                    a: (value & 0xff) as f32 / 255.0,
                }
            }
            _ => rgb(0),
        };
        rgba.into()
    }

    fn load_theme(json: &str, theme_name: &str) -> Arc<SyntaxTheme> {
        let theme_file: ThemeFile = serde_json::from_str(json).expect("failed to parse theme JSON");
        let theme_entry = theme_file
            .themes
            .iter()
            .find(|entry| entry.name == theme_name)
            .unwrap_or_else(|| panic!("theme {theme_name:?} not found in theme JSON"));

        let highlights = theme_entry
            .style
            .syntax
            .iter()
            .map(|(name, entry)| (name.clone(), entry.to_highlight_style()));

        Arc::new(SyntaxTheme::new(highlights))
    }

    impl SyntaxTheme {
        /// Load the "One Dark" syntax theme from the bundled theme JSON.
        pub fn one_dark() -> Arc<Self> {
            load_theme(
                include_str!("../../../assets/themes/one/one.json"),
                "One Dark",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::FontStyle;

    use super::*;

    #[test]
    fn token_resolves_to_each_themes_own_style() {
        // Real theme files list their capture names in whatever order the author
        // wrote them, so the same name lands on different slots in different
        // themes. Resolution must follow the name, never the slot.
        let dark = SyntaxTheme::new_test([("keyword", gpui::red()), ("string", gpui::green())]);
        let light = SyntaxTheme::new_test([("string", gpui::blue()), ("keyword", gpui::yellow())]);

        assert_ne!(
            dark.highlight_id("keyword"),
            light.highlight_id("keyword"),
            "themes must disagree on slots for this test to be meaningful"
        );

        let keyword = syntax_token::intern("keyword");
        assert_eq!(
            dark.get(keyword).and_then(|style| style.color),
            Some(gpui::red())
        );
        assert_eq!(
            light.get(keyword).and_then(|style| style.color),
            Some(gpui::yellow())
        );
    }

    #[test]
    fn token_falls_back_to_the_longest_prefix_the_theme_defines() {
        let theme = SyntaxTheme::new_test([("function", gpui::red())]);

        let method = syntax_token::intern("function.method");
        assert_eq!(
            theme.get(method).and_then(|style| style.color),
            Some(gpui::red())
        );

        let unrelated = syntax_token::intern("comment.doc");
        assert_eq!(theme.get(unrelated), None);
    }

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
