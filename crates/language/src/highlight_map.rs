use gpui::HighlightStyle;
use smallvec::SmallVec;
use std::{
    collections::{BTreeMap, btree_map::Range},
    sync::Arc,
};
use theme::SyntaxTheme;

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[HighlightId]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightId(pub u32);

const DEFAULT_SYNTAX_HIGHLIGHT_ID: HighlightId = HighlightId(u32::MAX);

impl HighlightMap {
    pub fn new(capture_names: &[&str], theme: &SyntaxTheme) -> Self {
        // For each capture name in the highlight query, find the longest
        // key in the theme's syntax styles that matches all of the
        // dot-separated components of the capture name.
        let highlights: BTreeMap<_, _> = theme
            .highlights
            .iter()
            .enumerate()
            .map(|(i, (key, _))| (key.split('.').collect::<SmallVec<[&str; 2]>>(), i))
            .collect();
        HighlightMap(
            capture_names
                .iter()
                .map(|capture_name| {
                    let capture_name = capture_name.split('.').collect::<SmallVec<[&str; 2]>>();
                    let index = highlights
                        .range::<SmallVec<[&str; 2]>, _>((
                            std::ops::Bound::Unbounded,
                            std::ops::Bound::Included(&capture_name),
                        ))
                        .rfind(|(prefix, _)| capture_name.starts_with(prefix))
                        .map(|(_, index)| *index);
                    index.map_or(DEFAULT_SYNTAX_HIGHLIGHT_ID, |index| {
                        HighlightId(index as u32)
                    })

                })
                .collect(),
        )
    }

    pub fn get(&self, capture_id: u32) -> HighlightId {
        self.0
            .get(capture_id as usize)
            .copied()
            .unwrap_or(DEFAULT_SYNTAX_HIGHLIGHT_ID)
    }
}

impl HighlightId {
    pub const TABSTOP_INSERT_ID: HighlightId = HighlightId(u32::MAX - 1);
    pub const TABSTOP_REPLACE_ID: HighlightId = HighlightId(u32::MAX - 2);

    pub(crate) fn is_default(&self) -> bool {
        *self == DEFAULT_SYNTAX_HIGHLIGHT_ID
    }

    pub fn style(&self, theme: &SyntaxTheme) -> Option<HighlightStyle> {
        theme.highlights.get(self.0 as usize).map(|entry| entry.1)
    }

    pub fn name<'a>(&self, theme: &'a SyntaxTheme) -> Option<&'a str> {
        theme.highlights.get(self.0 as usize).map(|e| e.0.as_str())
    }
}

impl Default for HighlightMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}

impl Default for HighlightId {
    fn default() -> Self {
        DEFAULT_SYNTAX_HIGHLIGHT_ID
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::rgba;

    #[test]
    fn test_highlight_map() {
        let theme = SyntaxTheme {
            highlights: [
                ("function", rgba(0x100000ff)),
                ("function.method", rgba(0x200000ff)),
                ("function.async", rgba(0x300000ff)),
                ("variable.builtin.self.rust", rgba(0x400000ff)),
                ("variable.builtin", rgba(0x500000ff)),
                ("variable", rgba(0x600000ff)),
            ]
            .iter()
            .map(|(name, color)| (name.to_string(), (*color).into()))
            .collect(),
        };

        let capture_names = &[
            "function.special",
            "function.async.rust",
            "variable.builtin.self",
        ];

        let map = HighlightMap::new(capture_names, &theme);
        assert_eq!(map.get(0).name(&theme), Some("function"));
        assert_eq!(map.get(1).name(&theme), Some("function.async"));
        assert_eq!(map.get(2).name(&theme), Some("variable.builtin"));
    }
}
