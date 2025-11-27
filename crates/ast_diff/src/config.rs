//! Configuration for syntax-aware diffing.

use std::borrow::Cow;
use std::collections::HashSet;

/// Configuration for syntax-aware diffing of a specific language.
pub trait SyntaxDiffConfig {
    /// Returns true if the given node kind should be treated as an atom
    /// (not have its children diffed individually).
    fn is_atom_node(&self, node_kind: &str) -> bool;

    /// Returns true if the given text is a delimiter (either open or close).
    fn is_delimiter(&self, text: &str) -> bool;

    /// Returns the matching close delimiter for an open delimiter.
    ///
    /// Returns `None` if the given string is not an open delimiter.
    fn get_matching_delimiter<'a>(&'a self, open: &str) -> Option<&'a str>;

    /// Returns true if the text is an open delimiter.
    fn is_open_delimiter(&self, text: &str) -> bool;

    /// Returns true if the text is a close delimiter.
    fn is_close_delimiter(&self, text: &str) -> bool;

    fn is_comment(&self, node_kind: &str) -> bool {
        let kind_lower = node_kind.to_lowercase();
        kind_lower.contains("comment")
    }

    fn is_string(&self, node_kind: &str) -> bool {
        let kind_lower = node_kind.to_lowercase();
        kind_lower.contains("string")
    }

    fn is_keyword(&self, node_kind: &str) -> bool {
        let _ = node_kind;
        false
    }

    fn is_type(&self, node_kind: &str) -> bool {
        let kind_lower = node_kind.to_lowercase();
        kind_lower.contains("type") && !kind_lower.contains("identifier")
    }
}

/// Default configuration that works reasonably well for most languages.
#[derive(Debug, Clone)]
pub struct DefaultConfig {
    atom_nodes: HashSet<&'static str>,
    delimiter_tokens: Vec<(&'static str, &'static str)>,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        let atom_nodes: HashSet<&'static str> = [
            "string",
            "string_literal",
            "string_content",
            "raw_string_literal",
            "interpreted_string_literal",
            "template_string",
            "comment",
            "line_comment",
            "block_comment",
            "doc_comment",
            "number",
            "integer",
            "integer_literal",
            "float",
            "float_literal",
            "char",
            "char_literal",
            "character_literal",
            "regex",
            "regex_literal",
            "ERROR",
        ]
        .into_iter()
        .collect();

        let delimiter_tokens = vec![("(", ")"), ("{", "}"), ("[", "]"), ("<", ">")];

        Self {
            atom_nodes,
            delimiter_tokens,
        }
    }
}

impl SyntaxDiffConfig for DefaultConfig {
    fn is_atom_node(&self, node_kind: &str) -> bool {
        self.atom_nodes.contains(node_kind)
    }

    fn is_delimiter(&self, text: &str) -> bool {
        self.delimiter_tokens
            .iter()
            .any(|(start, end)| *start == text || *end == text)
    }

    fn get_matching_delimiter<'a>(&'a self, open: &str) -> Option<&'a str> {
        self.delimiter_tokens
            .iter()
            .find(|(start, _)| *start == open)
            .map(|(_, end)| *end)
    }

    fn is_open_delimiter(&self, text: &str) -> bool {
        self.delimiter_tokens
            .iter()
            .any(|(start, _)| *start == text)
    }

    fn is_close_delimiter(&self, text: &str) -> bool {
        self.delimiter_tokens.iter().any(|(_, end)| *end == text)
    }
}

impl DefaultConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_additional_atoms(mut self, atoms: impl IntoIterator<Item = &'static str>) -> Self {
        self.atom_nodes.extend(atoms);
        self
    }

    pub fn with_additional_delimiters(
        mut self,
        delimiters: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> Self {
        self.delimiter_tokens.extend(delimiters);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_common_atoms() {
        let config = DefaultConfig::default();

        assert!(config.is_atom_node("string_literal"));
        assert!(config.is_atom_node("comment"));
        assert!(config.is_atom_node("line_comment"));
        assert!(config.is_atom_node("number"));
    }

    #[test]
    fn test_default_config_has_common_delimiters() {
        let config = DefaultConfig::default();

        assert!(config.is_delimiter("("));
        assert!(config.is_delimiter(")"));
        assert!(config.is_delimiter("{"));
        assert!(config.is_delimiter("}"));
        assert!(config.is_delimiter("["));
        assert!(config.is_delimiter("]"));
    }

    #[test]
    fn test_is_comment() {
        let config = DefaultConfig::default();

        assert!(config.is_comment("comment"));
        assert!(config.is_comment("line_comment"));
        assert!(config.is_comment("block_comment"));
        assert!(!config.is_comment("identifier"));
    }

    #[test]
    fn test_is_string() {
        let config = DefaultConfig::default();

        assert!(config.is_string("string"));
        assert!(config.is_string("string_literal"));
        assert!(config.is_string("raw_string"));
        assert!(!config.is_string("identifier"));
    }

    #[test]
    fn test_with_additional_atoms() {
        let config = DefaultConfig::default().with_additional_atoms(["custom_atom"]);
        assert!(config.is_atom_node("custom_atom"));
    }

    #[test]
    fn test_with_additional_delimiters() {
        let config = DefaultConfig::default().with_additional_delimiters([("<", ">")]);
        assert!(config.is_delimiter("<"));
        assert!(config.is_delimiter(">"));
    }

    #[test]
    fn test_language_diff_config_creation() {
        let delimiters = vec![
            ("{".to_string(), "}".to_string()),
            ("(".to_string(), ")".to_string()),
        ];
        let config = LanguageDiffConfig::new(delimiters);

        assert!(config.is_open_delimiter("{"));
        assert!(config.is_close_delimiter("}"));
        assert!(config.is_open_delimiter("("));
        assert!(config.is_close_delimiter(")"));
        assert!(!config.is_delimiter("["));
    }

    #[test]
    fn test_language_diff_config_matching_delimiter() {
        let delimiters = vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
        ];
        let config = LanguageDiffConfig::new(delimiters);

        assert_eq!(config.get_matching_delimiter("{"), Some("}"));
        assert_eq!(config.get_matching_delimiter("["), Some("]"));
        assert_eq!(config.get_matching_delimiter("("), None);
    }

    #[test]
    fn test_language_diff_config_atom_nodes() {
        let config = LanguageDiffConfig::new(vec![]);

        // Default atom nodes should include common types
        assert!(config.is_atom_node("string"));
        assert!(config.is_atom_node("comment"));
        assert!(config.is_atom_node("number"));
        assert!(config.is_atom_node("line_comment"));
        assert!(config.is_atom_node("string_literal"));
        assert!(!config.is_atom_node("identifier"));
    }

    #[test]
    fn test_language_diff_config_with_additional_atoms() {
        let config =
            LanguageDiffConfig::new(vec![]).with_additional_atoms(["custom_node".to_string()]);

        assert!(config.is_atom_node("custom_node"));
        // Should still have default atoms
        assert!(config.is_atom_node("string"));
    }

    #[test]
    fn test_language_diff_config_with_custom_atoms() {
        let config = LanguageDiffConfig::new(vec![]).with_atom_nodes(["only_this".to_string()]);

        assert!(config.is_atom_node("only_this"));
        // Should NOT have default atoms
        assert!(!config.is_atom_node("string"));
    }

    #[test]
    fn test_language_diff_config_comment_detection() {
        let config = LanguageDiffConfig::new(vec![]);

        assert!(config.is_comment("comment"));
        assert!(config.is_comment("line_comment"));
        assert!(config.is_comment("block_comment"));
        assert!(!config.is_comment("identifier"));
    }

    #[test]
    fn test_language_diff_config_string_detection() {
        let config = LanguageDiffConfig::new(vec![]);

        assert!(config.is_string("string"));
        assert!(config.is_string("string_literal"));
        assert!(config.is_string("template_string"));
        assert!(!config.is_string("identifier"));
    }

    #[test]
    fn test_rust_style_delimiters() {
        let delimiters = vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
            ("(".to_string(), ")".to_string()),
            ("<".to_string(), ">".to_string()),
            ("r#\"".to_string(), "\"#".to_string()),
        ];
        let config = LanguageDiffConfig::new(delimiters);

        assert!(config.is_open_delimiter("{"));
        assert!(config.is_close_delimiter("}"));
        assert!(config.is_open_delimiter("<"));
        assert!(config.is_close_delimiter(">"));
        assert!(config.is_open_delimiter("r#\""));
        assert!(config.is_close_delimiter("\"#"));

        assert_eq!(config.get_matching_delimiter("{"), Some("}"));
        assert_eq!(config.get_matching_delimiter("r#\""), Some("\"#"));
    }

    #[test]
    fn test_typescript_style_delimiters() {
        let delimiters = vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
            ("(".to_string(), ")".to_string()),
            ("<".to_string(), ">".to_string()),
            ("\"".to_string(), "\"".to_string()),
            ("'".to_string(), "'".to_string()),
            ("`".to_string(), "`".to_string()),
        ];
        let config = LanguageDiffConfig::new(delimiters);

        assert!(config.is_open_delimiter("{"));
        assert!(config.is_close_delimiter("}"));
        assert!(config.is_delimiter("\"")); // Both open and close
        assert!(config.is_delimiter("'")); // Both open and close
        assert!(config.is_delimiter("`")); // Template string
    }

    #[test]
    fn test_python_style_delimiters() {
        let delimiters = vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
            ("(".to_string(), ")".to_string()),
            ("\"".to_string(), "\"".to_string()),
            ("'".to_string(), "'".to_string()),
            ("\"\"\"".to_string(), "\"\"\"".to_string()),
            ("'''".to_string(), "'''".to_string()),
        ];
        let config = LanguageDiffConfig::new(delimiters);

        assert!(config.is_open_delimiter("{"));
        assert!(config.is_open_delimiter("\"\"\""));
        assert!(config.is_close_delimiter("\"\"\""));
        assert_eq!(config.get_matching_delimiter("\"\"\""), Some("\"\"\""));
    }

    #[test]
    fn test_empty_delimiter_list() {
        let config = LanguageDiffConfig::new(vec![]);

        assert!(!config.is_delimiter("{"));
        assert!(!config.is_delimiter("}"));
        assert_eq!(config.get_matching_delimiter("{"), None);
    }

    #[test]
    fn test_config_preserves_delimiter_order() {
        let delimiters = vec![
            ("A".to_string(), "a".to_string()),
            ("B".to_string(), "b".to_string()),
            ("C".to_string(), "c".to_string()),
        ];
        let config = LanguageDiffConfig::new(delimiters);

        assert_eq!(config.get_matching_delimiter("A"), Some("a"));
        assert_eq!(config.get_matching_delimiter("B"), Some("b"));
        assert_eq!(config.get_matching_delimiter("C"), Some("c"));
    }
}

/// Language-specific diff configuration.
#[derive(Debug, Clone)]
pub struct LanguageDiffConfig {
    delimiter_tokens: Vec<(String, String)>,
    atom_nodes: HashSet<Cow<'static, str>>,
}

impl LanguageDiffConfig {
    pub fn new(delimiters: Vec<(String, String)>) -> Self {
        Self {
            delimiter_tokens: delimiters,
            atom_nodes: default_atom_nodes(),
        }
    }

    pub fn with_atom_nodes(mut self, atoms: impl IntoIterator<Item = String>) -> Self {
        self.atom_nodes = atoms.into_iter().map(Cow::Owned).collect();
        self
    }

    pub fn with_additional_atoms(mut self, atoms: impl IntoIterator<Item = String>) -> Self {
        self.atom_nodes.extend(atoms.into_iter().map(Cow::Owned));
        self
    }

    pub fn delimiter_tokens_list(&self) -> &[(String, String)] {
        &self.delimiter_tokens
    }

    pub fn forced_atom_nodes_list(&self) -> Vec<&str> {
        self.atom_nodes.iter().map(|s| s.as_ref()).collect()
    }
}

impl SyntaxDiffConfig for LanguageDiffConfig {
    fn is_atom_node(&self, node_kind: &str) -> bool {
        self.atom_nodes.iter().any(|s| s.as_ref() == node_kind)
    }

    fn is_delimiter(&self, text: &str) -> bool {
        self.delimiter_tokens
            .iter()
            .any(|(start, end)| start == text || end == text)
    }

    fn get_matching_delimiter<'a>(&'a self, open: &str) -> Option<&'a str> {
        self.delimiter_tokens
            .iter()
            .find(|(start, _)| start == open)
            .map(|(_, end)| end.as_str())
    }

    fn is_open_delimiter(&self, text: &str) -> bool {
        self.delimiter_tokens.iter().any(|(start, _)| start == text)
    }

    fn is_close_delimiter(&self, text: &str) -> bool {
        self.delimiter_tokens.iter().any(|(_, end)| end == text)
    }
}

fn default_atom_nodes() -> HashSet<Cow<'static, str>> {
    [
        "string",
        "string_literal",
        "string_content",
        "raw_string_literal",
        "interpreted_string_literal",
        "template_string",
        "string_fragment",
        "comment",
        "line_comment",
        "block_comment",
        "doc_comment",
        "number",
        "integer",
        "integer_literal",
        "float",
        "float_literal",
        "char",
        "char_literal",
        "character_literal",
        "regex",
        "regex_literal",
        "ERROR",
    ]
    .into_iter()
    .map(Cow::Borrowed)
    .collect()
}
