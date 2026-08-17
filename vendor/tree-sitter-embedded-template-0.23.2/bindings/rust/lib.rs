//! This crate provides EmbeddedTemplate language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [LANGUAGE][] constant to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! use tree_sitter::Parser;
//!
//! let code = r#"
//! <html>
//!  <body>
//!  <% if (true) { %>
//!  <p>hello</p>
//!  <% } %>
//! </body>
//! </html>
//! "#;
//! let mut parser = Parser::new();
//! let language = tree_sitter_embedded_template::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading EmbeddedTemplate parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_embedded_template() -> *const ();
}

/// The tree-sitter [`LanguageFn`][LanguageFn] for this grammar.
///
/// [LanguageFn]: https://docs.rs/tree-sitter-language/*/tree_sitter_language/struct.LanguageFn.html
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_embedded_template) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

/// The syntax highlighting query for this grammar.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The injections query for this grammar to inject HTML/JavaScript.
pub const INJECTIONS_EJS_QUERY: &str = include_str!("../../queries/injections-ejs.scm");

/// The injections query for this grammar to inject HTML/Ruby.
pub const INJECTIONS_ERB_QUERY: &str = include_str!("../../queries/injections-erb.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading EmbeddedTemplate parser");
    }
}
