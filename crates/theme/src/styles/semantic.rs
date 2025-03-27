use std::sync::Arc;

use collections::IndexMap;
use gpui::HighlightStyle;

use super::SyntaxTheme;

/// Semantic highlight theme which guards the original theme to hold references
/// to it
#[derive(PartialEq, Eq, Default)]
pub struct SemanticTheme {
    syntax_theme: Arc<SyntaxTheme>,
    highlights: IndexMap<String, Highlight>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Highlight {
    TreeSitter(&'static str),
    Semantic(HighlightStyle),
}

pub(crate) const DEFAULT_SEMANTIC_TOKENS: &[(&str, Highlight)] = &[
    ("keyword", Highlight::TreeSitter("keyword")),
    ("variable", Highlight::TreeSitter("variable")),
    ("enumMember", Highlight::TreeSitter("variant")),
    ("comment", Highlight::TreeSitter("comment")),
    ("enum", Highlight::TreeSitter("enum")),
    ("type", Highlight::TreeSitter("type")),
    ("function", Highlight::TreeSitter("function.definition")),
    ("method", Highlight::TreeSitter("function.method")),
    ("macro", Highlight::TreeSitter("function.attribute")),
    ("namespace", Highlight::TreeSitter("emphasis.strong")),
    ("number", Highlight::TreeSitter("number")),
    ("string", Highlight::TreeSitter("string")),
    ("parameter", Highlight::TreeSitter("label")),
    ("const", Highlight::TreeSitter("constant")),
    ("punctuation", Highlight::TreeSitter("punctuation")),
    ("selfKeyword", Highlight::TreeSitter("keyword")),
    ("selfTypeKeyword", Highlight::TreeSitter("keyword")),
    ("operator", Highlight::TreeSitter("punctuation.special")),
];

pub(crate) const DEFAULT_SEMANTIC_MODIFIERS: &[(&str, Highlight)] = &[
    ("async", Highlight::TreeSitter("emphasis")),
    ("mutable", Highlight::TreeSitter("emphasis")),
    ("unsafe", Highlight::TreeSitter("emphasis")),
    ("associated", Highlight::TreeSitter("property")),
    ("attribute", Highlight::TreeSitter("property")),
    ("documentation", Highlight::TreeSitter("comment.doc")),
    ("constant", Highlight::TreeSitter("constant")),
    ("intraDocLink", Highlight::TreeSitter("link_uri")),
];

impl SemanticTheme {
    /// Creates a new theme with the fallback items and the items
    pub(crate) fn new(
        fallback: &[(&str, Highlight)],
        syntax_theme: Arc<SyntaxTheme>,
        items: &[(String, HighlightStyle)],
    ) -> Self {
        let mut highlights = IndexMap::default();
        for (syntax_token, semantic_style) in fallback {
            highlights.insert(syntax_token.to_string(), *semantic_style);
        }
        for (name, highlight) in items {
            highlights.insert(name.to_string(), Highlight::Semantic(*highlight));
        }
        Self {
            syntax_theme,
            highlights,
        }
    }

    /// Get the style for a semantic token or modifier
    pub fn get(&self, name: &str) -> Option<HighlightStyle> {
        match self.highlights.get(name) {
            Some(Highlight::Semantic(style)) => Some(*style),
            Some(Highlight::TreeSitter(ts)) => Some(self.syntax_theme.get(ts)),
            None => None,
        }
    }
}
