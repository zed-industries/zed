use crate::util::prelude::*;

pub(crate) trait MetaListExt {
    fn require_parens_delim(&self) -> Result<()>;
    fn require_curly_braces_delim(&self) -> Result<()>;
}

impl MetaListExt for syn::MetaList {
    fn require_parens_delim(&self) -> Result<()> {
        require_delim(self, MacroDelimKind::Paren)
    }

    fn require_curly_braces_delim(&self) -> Result<()> {
        require_delim(self, MacroDelimKind::Brace)
    }
}

fn require_delim(meta: &syn::MetaList, expected: MacroDelimKind) -> Result<()> {
    let actual = MacroDelimKind::from_syn(&meta.delimiter);
    if actual == expected {
        return Ok(());
    }

    let path = darling::util::path_to_string(&meta.path);
    bail!(
        meta,
        "wrong delimiter, expected {} e.g. `{path}{}`, but got {}: `{path}{}`",
        expected.name(),
        expected.example(),
        actual.name(),
        actual.example(),
    );
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MacroDelimKind {
    Paren,
    Brace,
    Bracket,
}

impl MacroDelimKind {
    fn from_syn(delim: &syn::MacroDelimiter) -> Self {
        match delim {
            syn::MacroDelimiter::Paren(_) => Self::Paren,
            syn::MacroDelimiter::Brace(_) => Self::Brace,
            syn::MacroDelimiter::Bracket(_) => Self::Bracket,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Paren => "parentheses",
            Self::Brace => "curly braces",
            Self::Bracket => "square brackets",
        }
    }

    fn example(self) -> &'static str {
        match self {
            Self::Paren => "(...)",
            Self::Brace => "{...}",
            Self::Bracket => "[...]",
        }
    }
}
