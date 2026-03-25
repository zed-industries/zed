pub use language_core::highlight_map::{HighlightId, HighlightMap};

use gpui::HighlightStyle;
use theme::SyntaxTheme;

pub fn build_highlight_map(capture_names: &[&str], theme: &SyntaxTheme) -> HighlightMap {
    HighlightMap::from_ids(
        capture_names
            .iter()
            .map(|capture_name| {
                theme
                    .highlight_id(capture_name)
                    .map_or_else(HighlightId::default, HighlightId)
            })
            .collect::<Vec<_>>(),
    )
}

pub fn highlight_style(id: HighlightId, theme: &SyntaxTheme) -> Option<HighlightStyle> {
    theme.get(id.index() as usize).cloned()
}

pub fn highlight_name(id: HighlightId, theme: &SyntaxTheme) -> Option<&str> {
    theme.get_capture_name(id.index() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::rgba;

    #[test]
    fn test_highlight_map() {
        let theme = SyntaxTheme::new(
            [
                ("function", rgba(0x100000ff)),
                ("function.method", rgba(0x200000ff)),
                ("function.async", rgba(0x300000ff)),
                ("variable.builtin.self.rust", rgba(0x400000ff)),
                ("variable.builtin", rgba(0x500000ff)),
                ("variable", rgba(0x600000ff)),
            ]
            .iter()
            .map(|(name, color)| (name.to_string(), (*color).into())),
        );

        let capture_names = &[
            "function.special",
            "function.async.rust",
            "variable.builtin.self",
        ];

        let map = build_highlight_map(capture_names, &theme);
        assert_eq!(highlight_name(map.get(0), &theme), Some("function"));
        assert_eq!(highlight_name(map.get(1), &theme), Some("function.async"));
        assert_eq!(highlight_name(map.get(2), &theme), Some("variable.builtin"));
    }
}
