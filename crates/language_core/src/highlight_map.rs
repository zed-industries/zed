use std::sync::Arc;

use syntax_token::SyntaxTokenId;

/// Maps a grammar's tree-sitter capture indices to canonical syntax token ids.
///
/// Built once when a grammar's highlights query is installed and never mutated
/// afterwards. The mapping depends only on the query's capture names, so it is
/// independent of any theme; a theme is consulted separately, at paint time, to
/// style the resulting tokens.
#[derive(Clone, Debug, Default)]
pub struct HighlightMap(Arc<[SyntaxTokenId]>);

impl HighlightMap {
    pub fn from_capture_names<'a>(capture_names: impl IntoIterator<Item = &'a str>) -> Self {
        Self(
            capture_names
                .into_iter()
                .map(syntax_token::intern)
                .collect(),
        )
    }

    #[inline]
    pub fn get(&self, capture_id: u32) -> Option<SyntaxTokenId> {
        self.0.get(capture_id as usize).copied()
    }
}
