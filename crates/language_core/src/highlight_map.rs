use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[HighlightId]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightId(pub u32);

const DEFAULT_SYNTAX_HIGHLIGHT_ID: HighlightId = HighlightId(u32::MAX);

impl HighlightMap {
    pub fn new(capture_names: &[&str], highlight_names: &[&str]) -> Self {
        // For each capture name in the highlight query, find the longest
        // key in the theme's syntax styles that is a dot-delimited prefix
        // of the capture name.
        HighlightMap(
            capture_names
                .iter()
                .map(|capture_name| {
                    highlight_names
                        .iter()
                        .enumerate()
                        .filter_map(|(i, key)| {
                            capture_name.strip_prefix(key).and_then(|remainder| {
                                if remainder.is_empty() || remainder.starts_with('.') {
                                    Some((i, key.len()))
                                } else {
                                    None
                                }
                            })
                        })
                        .max_by_key(|(_, len)| *len)
                        .map_or(DEFAULT_SYNTAX_HIGHLIGHT_ID, |(i, _)| HighlightId(i as u32))
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

    pub fn is_default(&self) -> bool {
        *self == DEFAULT_SYNTAX_HIGHLIGHT_ID
    }

    /// Returns the underlying index. Useful for extension traits that need
    /// to look up theme data by index.
    pub fn index(&self) -> u32 {
        self.0
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

    #[test]
    fn test_highlight_map() {
        let highlight_names = &[
            "function",
            "function.method",
            "function.async",
            "variable.builtin.self.rust",
            "variable.builtin",
            "variable",
        ];

        let capture_names = &[
            "function.special",
            "function.async.rust",
            "variable.builtin.self",
        ];

        let map = HighlightMap::new(capture_names, highlight_names);

        // "function.special" best matches "function" (index 0)
        assert_eq!(map.get(0), HighlightId(0));
        // "function.async.rust" best matches "function.async" (index 2)
        assert_eq!(map.get(1), HighlightId(2));
        // "variable.builtin.self" best matches "variable.builtin" (index 4)
        assert_eq!(map.get(2), HighlightId(4));
    }

    #[test]
    fn test_highlight_map_requires_dot_delimited_prefix_matches() {
        let highlight_names = &["foo.baz", "foo", "foo.bar"];
        let capture_names = &["foo.bar.baz"];

        let map = HighlightMap::new(capture_names, highlight_names);

        // "foo.baz" is not a prefix match for "foo.bar.baz", so the
        // longest valid dot-delimited prefix is "foo.bar" (index 2).
        assert_eq!(map.get(0), HighlightId(2));
    }
}
