use std::sync::Arc;

use gpui::HighlightStyle;

use super::SyntaxTheme;

/// Semantic highlight theme which guards the original theme to hold references
/// to it
#[derive(PartialEq, Eq, Default)]
pub struct SemanticTheme {
    syntax_theme: Arc<SyntaxTheme>,
    highlights: indexmap::IndexMap<String, TokenHighlight>,
    default_modifiers: TokenHighlight,
}

impl From<Arc<SyntaxTheme>> for SemanticTheme {
    fn from(syntax_theme: Arc<SyntaxTheme>) -> Self {
        Self {
            default_modifiers: TokenHighlight::default(),
            highlights: indexmap::indexmap! {
                "keyword".into() => syntax_theme.get("keyword").into(),
                "variable".into() => syntax_theme.get("variable").into(),
                "enumMember".into() => syntax_theme.get("constructor").into(),
                "comment".into() => syntax_theme.get("comment").into(),
                "enum".into() => syntax_theme.get("enum").into(),
                "type".into() => syntax_theme.get("type").into(),
                "function".into() => syntax_theme.get("function.definition").into(),
                "method".into() => syntax_theme.get("function.method").into(),
                "macro".into() => syntax_theme.get("function.attribute").into(),
                "namespace".into() => syntax_theme.get("emphasis.strong").into(),
                "number".into() => syntax_theme.get("number").into(),
                "string".into() => syntax_theme.get("string").into(),
                "parameter".into() => syntax_theme.get("label").into(),
                "const".into() => syntax_theme.get("constant").into(),
                "punctuation".into() => syntax_theme.get("keyword").into(),
                "dot".into() => syntax_theme.get("keyword").into(),
                "colon".into() => syntax_theme.get("keyword").into(),
                "semicolon".into() => syntax_theme.get("keyword").into(),
                "label".into() => syntax_theme.get("keyword").into(),
                "lifetime".into() => syntax_theme.get("keyword").into(),
                "selfKeyword".into() => syntax_theme.get("keyword").into(),
                "selfTypeKeyword".into() => syntax_theme.get("keyword").into(),
                "operator".into() => syntax_theme.get("keyword").into(),
            },
            syntax_theme,
        }
    }
}

/// Token semantic highlight
#[derive(PartialEq, Eq, Clone, Default)]
pub struct TokenHighlight {
    /// Highlight style for the token type
    pub style: HighlightStyle,
    pub(crate) highlights: indexmap::IndexMap<String, HighlightStyle>,
}

impl From<HighlightStyle> for TokenHighlight {
    fn from(value: HighlightStyle) -> Self {
        Self {
            style: value,
            highlights: Default::default(),
        }
    }
}

impl From<Arc<SyntaxTheme>> for TokenHighlight {
    fn from(syntax_theme: Arc<SyntaxTheme>) -> Self {
        Self {
            style: HighlightStyle::default(),
            highlights: indexmap::indexmap! {
                "async".into() => syntax_theme.get("emphasis"),
                "mutable".into() => syntax_theme.get("emphasis"),
                "unsafe".into() => syntax_theme.get("emphasis"),
                "associated".into() => syntax_theme.get("property"),
                "attribute".into() => syntax_theme.get("property"),
                "documentation".into() => syntax_theme.get("comment.doc"),
                "constant".into() => syntax_theme.get("constant"),
                "intraDocLink".into() => syntax_theme.get("link_uri"),
            },
        }
    }
}

impl TokenHighlight {
    pub(crate) fn import(mut self, items: &[(String, HighlightStyle)]) -> Self {
        for (name, highlight) in items {
            self.highlights.insert(name.clone(), *highlight);
        }
        self
    }

    pub(crate) fn assemble(&self, fallback: &TokenHighlight) -> Self {
        let mut this = self.clone();
        for (name, value) in fallback.highlights.iter() {
            this.highlights.entry(name.clone()).or_insert(*value);
        }
        this
    }

    /// Get the style for a semantic token or modifier
    pub fn get(&self, name: &str) -> Option<HighlightStyle> {
        self.highlights.get(name).cloned()
    }
}

impl SemanticTheme {
    /// Creates a new theme with the fallback items and the items
    pub(crate) fn import(
        mut self,
        fallback: TokenHighlight,
        items: &[(String, TokenHighlight)],
    ) -> Self {
        self.default_modifiers = fallback;
        for (name, highlight) in items {
            self.highlights.insert(name.clone(), highlight.clone());
        }
        self
    }

    /// Get the style for a semantic token or modifier
    pub fn get(&self, name: &str) -> Option<TokenHighlight> {
        match self.highlights.get(name) {
            Some(highlight) => Some(highlight.assemble(&self.default_modifiers)),
            None => None,
        }
    }
}
