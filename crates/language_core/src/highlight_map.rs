use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[HighlightId]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HighlightId(pub u32);

const DEFAULT_SYNTAX_HIGHLIGHT_ID: HighlightId = HighlightId(u32::MAX);

impl HighlightMap {
    pub fn new(capture_names: &[&str], highlight_names: &[&str]) -> Self {
        // For each capture name in the highlight query, find the longest
        // key in the theme's syntax styles that matches all of the
        // dot-separated components of the capture name.
        HighlightMap(
            capture_names
                .iter()
                .map(|capture_name| {
                    highlight_names
                        .iter()
                        .enumerate()
                        .filter_map(|(i, key)| {
                            let mut len = 0;
                            let capture_parts = capture_name.split('.');
                            for key_part in key.split('.') {
                                if capture_parts.clone().any(|part| part == key_part) {
                                    len += 1;
                                } else {
                                    return None;
                                }
                            }
                            Some((i, len))
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
}
