use crate::syntax_theme::SyntaxTheme;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HighlightMap(Arc<[HighlightId]>);

#[derive(Clone, Copy, Debug)]
pub struct HighlightId(pub u32);

const DEFAULT_HIGHLIGHT_ID: HighlightId = HighlightId(u32::MAX);

impl HighlightMap {
    pub fn new(capture_names: &[String], theme: &SyntaxTheme) -> Self {
        // For each capture name in the highlight query, find the longest
        // key in the theme's syntax styles that matches all of the
        // dot-separated components of the capture name.
        HighlightMap(
            capture_names
                .iter()
                .map(|capture_name| {
                    theme
                        .highlights
                        .iter()
                        .enumerate()
                        .filter_map(|(i, (key, _))| {
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
                        .map_or(DEFAULT_HIGHLIGHT_ID, |(i, _)| HighlightId(i as u32))
                })
                .collect(),
        )
    }

    pub fn get(&self, capture_id: u32) -> HighlightId {
        self.0
            .get(capture_id as usize)
            .copied()
            .unwrap_or(DEFAULT_HIGHLIGHT_ID)
    }
}

impl Default for HighlightMap {
    fn default() -> Self {
        Self(Arc::new([]))
    }
}

impl Default for HighlightId {
    fn default() -> Self {
        DEFAULT_HIGHLIGHT_ID
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::color::Color;

    #[test]
    fn test_highlight_map() {
        let theme = SyntaxTheme::new(
            [
                ("function", Color::from_u32(0x100000ff)),
                ("function.method", Color::from_u32(0x200000ff)),
                ("function.async", Color::from_u32(0x300000ff)),
                ("variable.builtin.self.rust", Color::from_u32(0x400000ff)),
                ("variable.builtin", Color::from_u32(0x500000ff)),
                ("variable", Color::from_u32(0x600000ff)),
            ]
            .iter()
            .map(|(name, color)| (name.to_string(), (*color).into()))
            .collect(),
        );

        let capture_names = &[
            "function.special".to_string(),
            "function.async.rust".to_string(),
            "variable.builtin.self".to_string(),
        ];

        let map = HighlightMap::new(capture_names, &theme);
        assert_eq!(theme.highlight_name(map.get(0)), Some("function"));
        assert_eq!(theme.highlight_name(map.get(1)), Some("function.async"));
        assert_eq!(theme.highlight_name(map.get(2)), Some("variable.builtin"));
    }
}
