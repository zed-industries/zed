//! Jinja2 grammar for tree-sitter, vendored from
//! <https://github.com/geigerzaehler/tree-sitter-jinja2> (MIT, per its Cargo.toml)
//! at commit 7af726f7ac42db3fe798d45ee375078bbef28a41, with bindings rewritten to
//! the `tree_sitter_language::LanguageFn` pattern so the grammar constant is
//! type-compatible with the workspace's patched `tree-sitter` crate.

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_jinja2() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for the Jinja2 grammar.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_jinja2) };

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let language = tree_sitter::Language::new(super::LANGUAGE);
        assert!(language.abi_version() > 0);
    }
}
