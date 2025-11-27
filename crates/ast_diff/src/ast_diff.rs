//! AST-aware diffing for syntax trees, extracted and adapted from
//! [difftastic](https://github.com/Wilfred/difftastic).

mod builder;
mod changes;
mod config;
mod dijkstra;
mod graph;
mod hash;
mod lcs_diff;
mod sliders;
mod stack;
mod syntax;
mod unchanged;

pub use builder::build_syntax;
pub use changes::{ChangeKind, ChangeMap};
pub use config::{DefaultConfig, LanguageDiffConfig, SyntaxDiffConfig};
pub use dijkstra::ExceededGraphLimit;
pub use sliders::SliderPreference;
pub use syntax::{AtomKind, StringKind, Syntax, SyntaxId, init_all_info};

use bumpalo::Bump;
use std::{fmt, ops::Range};
use tree_sitter::Tree;

/// Default graph limit (10 million vertices).
pub const DEFAULT_GRAPH_LIMIT: usize = 10_000_000;

/// Error types for syntax diffing.
#[derive(Debug)]
pub enum DiffError {
    /// The diff graph exceeded the configured limit.
    ExceededGraphLimit,
}

impl fmt::Display for DiffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffError::ExceededGraphLimit => {
                write!(f, "Diff graph exceeded configured vertex limit")
            }
        }
    }
}

impl std::error::Error for DiffError {}

impl From<ExceededGraphLimit> for DiffError {
    fn from(_: ExceededGraphLimit) -> Self {
        DiffError::ExceededGraphLimit
    }
}

/// Options for syntax diffing.
#[derive(Debug, Clone)]
pub struct DiffOptions {
    /// Maximum number of graph vertices before falling back to text diff.
    pub graph_limit: usize,
    /// Preference for slider fixing (inner vs outer delimiters).
    pub slider_preference: SliderPreference,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self {
            graph_limit: DEFAULT_GRAPH_LIMIT,
            slider_preference: SliderPreference::PreferInner,
        }
    }
}

/// The type of change detected for a syntax element.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxChangeKind {
    /// The node is unchanged.
    Unchanged,
    /// The node is novel (added or removed).
    Novel,
    /// A comment was replaced with another comment.
    ReplacedComment,
    /// A string was replaced with another string.
    ReplacedString,
}

/// A change detected during syntax diffing.
#[derive(Debug, Clone)]
pub struct SyntaxChange {
    /// The type of change.
    pub kind: SyntaxChangeKind,
    /// The byte range in the LHS text that changed (None if added).
    pub lhs_byte_range: Option<Range<usize>>,
    /// The byte range in the RHS text that changed (None if removed).
    pub rhs_byte_range: Option<Range<usize>>,
}

/// Compute a syntax-aware diff between two tree-sitter trees.
///
/// Returns `DiffError::ExceededGraphLimit` if the diff graph exceeds the
/// configured limit. Callers should fall back to text-based diffing in this case.
pub fn diff_syntax(
    lhs_tree: &Tree,
    lhs_text: &str,
    rhs_tree: &Tree,
    rhs_text: &str,
    config: &dyn SyntaxDiffConfig,
    options: &DiffOptions,
) -> Result<(Vec<Range<usize>>, Vec<Range<usize>>), DiffError> {
    let arena = Bump::new();

    let lhs_roots = build_syntax(&arena, lhs_tree, lhs_text, config);
    let rhs_roots = build_syntax(&arena, rhs_tree, rhs_text, config);

    diff_syntax_roots(&lhs_roots, &rhs_roots, options).map_err(Into::into)
}

/// Compute a syntax-aware diff between two syntax trees.
pub fn diff_syntax_roots<'a>(
    lhs_roots: &[&'a Syntax<'a>],
    rhs_roots: &[&'a Syntax<'a>],
    options: &DiffOptions,
) -> Result<(Vec<Range<usize>>, Vec<Range<usize>>), ExceededGraphLimit> {
    init_all_info(lhs_roots, rhs_roots);

    let mut change_map = ChangeMap::default();
    let nodes_to_diff = unchanged::mark_unchanged(lhs_roots, rhs_roots, &mut change_map);

    for (lhs_nodes, rhs_nodes) in nodes_to_diff {
        dijkstra::mark_syntax(
            lhs_nodes.first().copied(),
            rhs_nodes.first().copied(),
            &mut change_map,
            options.graph_limit,
        )?;
    }

    sliders::fix_all_sliders(options.slider_preference, lhs_roots, &mut change_map);
    sliders::fix_all_sliders(options.slider_preference, rhs_roots, &mut change_map);

    let lhs_ranges = collect_novel_ranges(lhs_roots, &change_map);
    let rhs_ranges = collect_novel_ranges(rhs_roots, &change_map);

    Ok((lhs_ranges, rhs_ranges))
}

fn collect_novel_ranges<'a>(
    nodes: &[&'a Syntax<'a>],
    change_map: &ChangeMap<'a>,
) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    collect_novel_ranges_recursive(nodes, change_map, &mut ranges);

    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_by_key(|r| r.start);
    let mut merged = vec![ranges[0].clone()];

    for range in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if range.start <= last.end {
            last.end = last.end.max(range.end);
        } else {
            merged.push(range);
        }
    }

    merged
}

fn collect_novel_ranges_recursive<'a>(
    nodes: &[&'a Syntax<'a>],
    change_map: &ChangeMap<'a>,
    ranges: &mut Vec<Range<usize>>,
) {
    for node in nodes {
        match change_map.get(node) {
            Some(ChangeKind::Novel) => {
                ranges.push(node.byte_range());
            }
            Some(ChangeKind::ReplacedComment(_, _)) | Some(ChangeKind::ReplacedString(_, _)) => {
                ranges.push(node.byte_range());
            }
            Some(ChangeKind::Unchanged(_)) => {
                if let Syntax::List { children, .. } = node {
                    collect_novel_ranges_recursive(children, change_map, ranges);
                }
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bumpalo::Bump;

    fn make_atom<'a>(arena: &'a Bump, content: &str, start: usize) -> &'a Syntax<'a> {
        Syntax::new_atom(
            arena,
            start..start + content.len(),
            content.to_string(),
            AtomKind::Normal,
        )
    }

    #[test]
    fn test_identical_atoms() {
        let arena = Bump::new();

        let lhs = make_atom(&arena, "foo", 0);
        let rhs = make_atom(&arena, "foo", 0);

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&[lhs], &[rhs], &DiffOptions::default()).unwrap();

        assert!(lhs_ranges.is_empty(), "Expected no changes on LHS");
        assert!(rhs_ranges.is_empty(), "Expected no changes on RHS");
    }

    #[test]
    fn test_different_atoms() {
        let arena = Bump::new();

        let lhs = make_atom(&arena, "foo", 0);
        let rhs = make_atom(&arena, "bar", 0);

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&[lhs], &[rhs], &DiffOptions::default()).unwrap();

        assert_eq!(lhs_ranges.len(), 1, "Expected one change on LHS");
        assert_eq!(rhs_ranges.len(), 1, "Expected one change on RHS");
        assert_eq!(lhs_ranges[0], 0..3);
        assert_eq!(rhs_ranges[0], 0..3);
    }

    #[test]
    fn test_added_node() {
        let arena = Bump::new();

        let lhs = [make_atom(&arena, "a", 0)];
        let rhs = [make_atom(&arena, "a", 0), make_atom(&arena, "b", 2)];

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs, &rhs, &DiffOptions::default()).unwrap();

        assert!(lhs_ranges.is_empty(), "Expected no changes on LHS");
        assert_eq!(rhs_ranges.len(), 1, "Expected one addition on RHS");
        assert_eq!(rhs_ranges[0], 2..3);
    }

    #[test]
    fn test_list_with_unchanged_delimiters() {
        let arena = Bump::new();

        let lhs_child = make_atom(&arena, "old", 1);
        let lhs = Syntax::new_list(&arena, "(", 0..1, vec![lhs_child], ")", 4..5);

        let rhs_child = make_atom(&arena, "new", 1);
        let rhs = Syntax::new_list(&arena, "(", 0..1, vec![rhs_child], ")", 4..5);

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&[lhs], &[rhs], &DiffOptions::default()).unwrap();

        assert_eq!(lhs_ranges.len(), 1);
        assert_eq!(rhs_ranges.len(), 1);
        assert_eq!(lhs_ranges[0], 1..4);
        assert_eq!(rhs_ranges[0], 1..4);
    }

    fn parse_json(source: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .expect("Error loading JSON grammar");
        parser.parse(source, None).expect("Failed to parse")
    }

    #[test]
    fn test_integration_identical_json() {
        let arena = Bump::new();
        let config = DefaultConfig::default();
        let source = r#"{"key": "value"}"#;

        let tree = parse_json(source);
        let lhs_nodes = build_syntax(&arena, &tree, source, &config);
        let rhs_nodes = build_syntax(&arena, &tree, source, &config);

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs_nodes, &rhs_nodes, &DiffOptions::default()).unwrap();

        assert!(
            lhs_ranges.is_empty(),
            "Expected no changes for identical JSON"
        );
        assert!(
            rhs_ranges.is_empty(),
            "Expected no changes for identical JSON"
        );
    }

    #[test]
    fn test_integration_different_json_values() {
        let arena = Bump::new();
        let config = DefaultConfig::default();

        let lhs_source = r#"{"key": "old"}"#;
        let rhs_source = r#"{"key": "new"}"#;

        let lhs_tree = parse_json(lhs_source);
        let rhs_tree = parse_json(rhs_source);

        let lhs_nodes = build_syntax(&arena, &lhs_tree, lhs_source, &config);
        let rhs_nodes = build_syntax(&arena, &rhs_tree, rhs_source, &config);

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs_nodes, &rhs_nodes, &DiffOptions::default()).unwrap();

        // Should detect that the string value changed
        assert!(!lhs_ranges.is_empty(), "Expected changes on LHS");
        assert!(!rhs_ranges.is_empty(), "Expected changes on RHS");
    }

    #[test]
    fn test_integration_added_json_field() {
        let arena = Bump::new();
        let config = DefaultConfig::default();

        let lhs_source = r#"{"a": 1}"#;
        let rhs_source = r#"{"a": 1, "b": 2}"#;

        let lhs_tree = parse_json(lhs_source);
        let rhs_tree = parse_json(rhs_source);

        let lhs_nodes = build_syntax(&arena, &lhs_tree, lhs_source, &config);
        let rhs_nodes = build_syntax(&arena, &rhs_tree, rhs_source, &config);

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs_nodes, &rhs_nodes, &DiffOptions::default()).unwrap();

        // LHS should have no changes, RHS should have the added field
        assert!(
            lhs_ranges.is_empty(),
            "Expected no changes on LHS for added field"
        );
        assert!(!rhs_ranges.is_empty(), "Expected additions on RHS");
    }

    #[test]
    fn test_integration_removed_json_field() {
        let arena = Bump::new();
        let config = DefaultConfig::default();

        let lhs_source = r#"{"a": 1, "b": 2}"#;
        let rhs_source = r#"{"a": 1}"#;

        let lhs_tree = parse_json(lhs_source);
        let rhs_tree = parse_json(rhs_source);

        let lhs_nodes = build_syntax(&arena, &lhs_tree, lhs_source, &config);
        let rhs_nodes = build_syntax(&arena, &rhs_tree, rhs_source, &config);

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs_nodes, &rhs_nodes, &DiffOptions::default()).unwrap();

        // LHS should have the removed field, RHS should have no changes
        assert!(!lhs_ranges.is_empty(), "Expected removals on LHS");
        assert!(
            rhs_ranges.is_empty(),
            "Expected no changes on RHS for removed field"
        );
    }

    #[test]
    fn test_integration_nested_json_change() {
        let arena = Bump::new();
        let config = DefaultConfig::default();

        let lhs_source = r#"{"outer": {"inner": "old"}}"#;
        let rhs_source = r#"{"outer": {"inner": "new"}}"#;

        let lhs_tree = parse_json(lhs_source);
        let rhs_tree = parse_json(rhs_source);

        let lhs_nodes = build_syntax(&arena, &lhs_tree, lhs_source, &config);
        let rhs_nodes = build_syntax(&arena, &rhs_tree, rhs_source, &config);

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs_nodes, &rhs_nodes, &DiffOptions::default()).unwrap();

        // Should detect the nested change
        assert!(!lhs_ranges.is_empty(), "Expected changes on LHS");
        assert!(!rhs_ranges.is_empty(), "Expected changes on RHS");
    }

    #[test]
    fn test_diff_syntax_high_level_api() {
        let config = DefaultConfig::default();

        let lhs_source = r#"{"key": "old"}"#;
        let rhs_source = r#"{"key": "new"}"#;

        let lhs_tree = parse_json(lhs_source);
        let rhs_tree = parse_json(rhs_source);

        let (lhs_ranges, rhs_ranges) = diff_syntax(
            &lhs_tree,
            lhs_source,
            &rhs_tree,
            rhs_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(!lhs_ranges.is_empty(), "Expected changes on LHS");
        assert!(!rhs_ranges.is_empty(), "Expected changes on RHS");
    }

    #[test]
    fn test_diff_syntax_identical() {
        let config = DefaultConfig::default();
        let source = r#"[1, 2, 3]"#;

        let tree = parse_json(source);

        let (lhs_ranges, rhs_ranges) = diff_syntax(
            &tree,
            source,
            &tree,
            source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            lhs_ranges.is_empty(),
            "Expected no changes for identical input"
        );
        assert!(
            rhs_ranges.is_empty(),
            "Expected no changes for identical input"
        );
    }

    #[test]
    fn test_empty_vs_nonempty() {
        let arena = Bump::new();

        let lhs: Vec<&Syntax> = vec![];
        let rhs = vec![make_atom(&arena, "x", 0)];

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs, &rhs, &DiffOptions::default()).unwrap();

        assert!(lhs_ranges.is_empty(), "Expected no changes on empty LHS");
        assert_eq!(rhs_ranges.len(), 1, "Expected one addition on RHS");
    }

    #[test]
    fn test_nonempty_vs_empty() {
        let arena = Bump::new();

        let lhs = vec![make_atom(&arena, "x", 0)];
        let rhs: Vec<&Syntax> = vec![];

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs, &rhs, &DiffOptions::default()).unwrap();

        assert_eq!(lhs_ranges.len(), 1, "Expected one removal on LHS");
        assert!(rhs_ranges.is_empty(), "Expected no changes on empty RHS");
    }

    #[test]
    fn test_both_empty() {
        let lhs: Vec<&Syntax> = vec![];
        let rhs: Vec<&Syntax> = vec![];

        let (lhs_ranges, rhs_ranges) =
            diff_syntax_roots(&lhs, &rhs, &DiffOptions::default()).unwrap();

        assert!(lhs_ranges.is_empty());
        assert!(rhs_ranges.is_empty());
    }

    #[test]
    fn test_graph_limit_exceeded() {
        let arena = Bump::new();

        let lhs = make_atom(&arena, "a", 0);
        let rhs = make_atom(&arena, "b", 0);

        let options = DiffOptions {
            graph_limit: 1,
            ..Default::default()
        };

        let result = diff_syntax_roots(&[lhs], &[rhs], &options);

        assert!(
            result.is_err(),
            "Expected graph limit error with limit of 1"
        );
    }

    #[test]
    fn test_diff_error_display() {
        let err = DiffError::ExceededGraphLimit;
        let msg = format!("{}", err);
        assert!(msg.contains("limit"), "Error message should mention limit");
    }

    #[test]
    fn test_syntax_change_kind() {
        assert_eq!(SyntaxChangeKind::Novel, SyntaxChangeKind::Novel);
        assert_ne!(SyntaxChangeKind::Novel, SyntaxChangeKind::Unchanged);
    }

    #[test]
    fn test_deep_nesting() {
        let config = DefaultConfig::default();

        let lhs_source = r#"{"a": {"b": {"c": {"d": 1}}}}"#;
        let rhs_source = r#"{"a": {"b": {"c": {"d": 2}}}}"#;

        let lhs_tree = parse_json(lhs_source);
        let rhs_tree = parse_json(rhs_source);

        let (lhs_ranges, rhs_ranges) = diff_syntax(
            &lhs_tree,
            lhs_source,
            &rhs_tree,
            rhs_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            !lhs_ranges.is_empty(),
            "Expected changes detected in deeply nested structure"
        );
        assert!(
            !rhs_ranges.is_empty(),
            "Expected changes detected in deeply nested structure"
        );
    }

    #[test]
    fn test_multiple_changes() {
        let config = DefaultConfig::default();

        let lhs_source = r#"{"a": 1, "b": 2, "c": 3}"#;
        let rhs_source = r#"{"a": 9, "b": 2, "c": 8}"#;

        let lhs_tree = parse_json(lhs_source);
        let rhs_tree = parse_json(rhs_source);

        let (lhs_ranges, rhs_ranges) = diff_syntax(
            &lhs_tree,
            lhs_source,
            &rhs_tree,
            rhs_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(!lhs_ranges.is_empty(), "Expected changes on LHS");
        assert!(!rhs_ranges.is_empty(), "Expected changes on RHS");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::LanguageDiffConfig;

    fn parse_rust(source: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .expect("Error loading Rust grammar");
        parser.parse(source, None).expect("Failed to parse Rust")
    }

    fn parse_python(source: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_python::LANGUAGE.into())
            .expect("Error loading Python grammar");
        parser.parse(source, None).expect("Failed to parse Python")
    }

    fn parse_typescript(source: &str) -> tree_sitter::Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
            .expect("Error loading TypeScript grammar");
        parser
            .parse(source, None)
            .expect("Failed to parse TypeScript")
    }

    fn rust_config() -> LanguageDiffConfig {
        LanguageDiffConfig::new(vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
            ("(".to_string(), ")".to_string()),
            ("<".to_string(), ">".to_string()),
        ])
    }

    fn python_config() -> LanguageDiffConfig {
        LanguageDiffConfig::new(vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
            ("(".to_string(), ")".to_string()),
        ])
    }

    fn typescript_config() -> LanguageDiffConfig {
        LanguageDiffConfig::new(vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
            ("(".to_string(), ")".to_string()),
            ("<".to_string(), ">".to_string()),
        ])
    }

    #[test]
    fn test_rust_function_body_change() {
        let old_source = r#"fn main() {
    let x = 1;
    println!("{}", x);
}"#;
        let new_source = r#"fn main() {
    let x = 2;
    println!("{}", x);
}"#;

        let old_tree = parse_rust(old_source);
        let new_tree = parse_rust(new_source);
        let config = rust_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(!old_ranges.is_empty(), "Expected changes in old source");
        assert!(!new_ranges.is_empty(), "Expected changes in new source");

        let old_changed = &old_source[old_ranges[0].clone()];
        let new_changed = &new_source[new_ranges[0].clone()];
        assert!(
            old_changed.contains("1") || old_source.contains("1"),
            "Old change should involve '1'"
        );
        assert!(
            new_changed.contains("2") || new_source.contains("2"),
            "New change should involve '2'"
        );
    }

    #[test]
    fn test_rust_identical_code() {
        let source = r#"fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}"#;

        let tree = parse_rust(source);
        let config = rust_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &tree,
            source,
            &tree,
            source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            old_ranges.is_empty(),
            "Expected no changes for identical Rust code"
        );
        assert!(
            new_ranges.is_empty(),
            "Expected no changes for identical Rust code"
        );
    }

    #[test]
    fn test_rust_added_function() {
        let old_source = r#"fn one() {}"#;
        let new_source = r#"fn one() {}
fn two() {}"#;

        let old_tree = parse_rust(old_source);
        let new_tree = parse_rust(new_source);
        let config = rust_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            old_ranges.is_empty(),
            "Expected no changes on old side when adding function"
        );
        assert!(!new_ranges.is_empty(), "Expected additions on new side");
    }

    #[test]
    fn test_rust_struct_field_change() {
        let old_source = r#"struct Point {
    x: i32,
    y: i32,
}"#;
        let new_source = r#"struct Point {
    x: f64,
    y: f64,
}"#;

        let old_tree = parse_rust(old_source);
        let new_tree = parse_rust(new_source);
        let config = rust_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            !old_ranges.is_empty(),
            "Expected changes for type modification"
        );
        assert!(
            !new_ranges.is_empty(),
            "Expected changes for type modification"
        );
    }

    #[test]
    fn test_python_function_body_change() {
        let old_source = r#"def greet(name):
    return "Hello, " + name"#;
        let new_source = r#"def greet(name):
    return "Hi, " + name"#;

        let old_tree = parse_python(old_source);
        let new_tree = parse_python(new_source);
        let config = python_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            !old_ranges.is_empty(),
            "Expected changes in Python function"
        );
        assert!(
            !new_ranges.is_empty(),
            "Expected changes in Python function"
        );
    }

    #[test]
    fn test_python_identical_code() {
        let source = r#"def add(a, b):
    return a + b"#;

        let tree = parse_python(source);
        let config = python_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &tree,
            source,
            &tree,
            source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            old_ranges.is_empty(),
            "Expected no changes for identical Python code"
        );
        assert!(
            new_ranges.is_empty(),
            "Expected no changes for identical Python code"
        );
    }

    #[test]
    fn test_python_class_method_change() {
        let old_source = r#"class Calculator:
    def add(self, a, b):
        return a + b"#;
        let new_source = r#"class Calculator:
    def add(self, a, b):
        return a - b"#;

        let old_tree = parse_python(old_source);
        let new_tree = parse_python(new_source);
        let config = python_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(!old_ranges.is_empty(), "Expected changes in class method");
        assert!(!new_ranges.is_empty(), "Expected changes in class method");
    }

    #[test]
    fn test_typescript_interface_change() {
        let old_source = r#"interface User {
    name: string;
    age: number;
}"#;
        let new_source = r#"interface User {
    name: string;
    age: number;
    email: string;
}"#;

        let old_tree = parse_typescript(old_source);
        let new_tree = parse_typescript(new_source);
        let config = typescript_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            old_ranges.is_empty(),
            "Expected no changes on old side when adding field"
        );
        assert!(!new_ranges.is_empty(), "Expected additions for new field");
    }

    #[test]
    fn test_typescript_identical_code() {
        let source = r#"function greet(name: string): string {
    return `Hello, ${name}!`;
}"#;

        let tree = parse_typescript(source);
        let config = typescript_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &tree,
            source,
            &tree,
            source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            old_ranges.is_empty(),
            "Expected no changes for identical TypeScript code"
        );
        assert!(
            new_ranges.is_empty(),
            "Expected no changes for identical TypeScript code"
        );
    }

    #[test]
    fn test_typescript_function_signature_change() {
        let old_source = r#"function process(data: string): void {
    console.log(data);
}"#;
        let new_source = r#"function process(data: number): void {
    console.log(data);
}"#;

        let old_tree = parse_typescript(old_source);
        let new_tree = parse_typescript(new_source);
        let config = typescript_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(!old_ranges.is_empty(), "Expected changes for type change");
        assert!(!new_ranges.is_empty(), "Expected changes for type change");
    }

    #[test]
    fn test_default_config_vs_language_config() {
        let source = r#"{"key": "value"}"#;

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .expect("Error loading JSON grammar");
        let tree = parser.parse(source, None).expect("Failed to parse");

        let default_config = DefaultConfig::default();
        let language_config = LanguageDiffConfig::new(vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
        ]);

        let (default_lhs, default_rhs) = diff_syntax(
            &tree,
            source,
            &tree,
            source,
            &default_config,
            &DiffOptions::default(),
        )
        .unwrap();

        let (lang_lhs, lang_rhs) = diff_syntax(
            &tree,
            source,
            &tree,
            source,
            &language_config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert_eq!(
            default_lhs, lang_lhs,
            "Both configs should produce same result for identical input"
        );
        assert_eq!(
            default_rhs, lang_rhs,
            "Both configs should produce same result for identical input"
        );
    }

    #[test]
    fn test_language_config_delimiter_recognition() {
        let old_source = r#"fn test() {
    let arr = [1, 2, 3];
}"#;
        let new_source = r#"fn test() {
    let arr = [1, 2, 3, 4];
}"#;

        let old_tree = parse_rust(old_source);
        let new_tree = parse_rust(new_source);

        let config_with_brackets = LanguageDiffConfig::new(vec![
            ("{".to_string(), "}".to_string()),
            ("[".to_string(), "]".to_string()),
            ("(".to_string(), ")".to_string()),
        ]);

        let config_without_brackets = LanguageDiffConfig::new(vec![
            ("{".to_string(), "}".to_string()),
            ("(".to_string(), ")".to_string()),
        ]);

        let (with_lhs, with_rhs) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config_with_brackets,
            &DiffOptions::default(),
        )
        .unwrap();

        let (without_lhs, without_rhs) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config_without_brackets,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            !with_lhs.is_empty() || !with_rhs.is_empty(),
            "Both configs should detect changes"
        );
        assert!(
            !without_lhs.is_empty() || !without_rhs.is_empty(),
            "Both configs should detect changes"
        );
    }

    #[test]
    fn test_comment_handling_across_languages() {
        let rust_old = "// old comment\nfn main() {}";
        let rust_new = "// new comment\nfn main() {}";

        let old_tree = parse_rust(rust_old);
        let new_tree = parse_rust(rust_new);
        let config = rust_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            rust_old,
            &new_tree,
            rust_new,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            !old_ranges.is_empty() || !new_ranges.is_empty(),
            "Should detect comment change"
        );
    }

    #[test]
    fn test_string_literal_handling() {
        let old_source = r#"let msg = "hello";"#;
        let new_source = r#"let msg = "world";"#;

        let old_tree = parse_rust(old_source);
        let new_tree = parse_rust(new_source);
        let config = rust_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(!old_ranges.is_empty(), "Should detect string change");
        assert!(!new_ranges.is_empty(), "Should detect string change");
    }

    #[test]
    fn test_nested_structure_preservation() {
        let old_source = r#"fn outer() {
    fn inner() {
        let x = 1;
    }
}"#;
        let new_source = r#"fn outer() {
    fn inner() {
        let x = 2;
    }
}"#;

        let old_tree = parse_rust(old_source);
        let new_tree = parse_rust(new_source);
        let config = rust_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(!old_ranges.is_empty(), "Should detect nested change");
        assert!(!new_ranges.is_empty(), "Should detect nested change");
        assert!(
            old_ranges.len() <= 2,
            "Should have minimal changed ranges, not explode entire structure"
        );
    }

    #[test]
    fn test_whitespace_only_changes() {
        let old_source = "fn main() { let x = 1; }";
        let new_source = "fn main() {\n    let x = 1;\n}";

        let old_tree = parse_rust(old_source);
        let new_tree = parse_rust(new_source);
        let config = rust_config();

        let result = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        );

        assert!(
            result.is_ok(),
            "Should handle whitespace-only changes gracefully"
        );
    }

    #[test]
    fn test_large_file_graph_limit() {
        let mut large_source = String::new();
        for i in 0..100 {
            large_source.push_str(&format!("fn func_{}() {{ let x = {}; }}\n", i, i));
        }

        let tree = parse_rust(&large_source);
        let config = rust_config();

        let options = DiffOptions {
            graph_limit: 100,
            ..Default::default()
        };

        let result = diff_syntax(
            &tree,
            &large_source,
            &tree,
            &large_source,
            &config,
            &options,
        );

        assert!(
            result.is_ok(),
            "Identical large files should not exceed graph limit"
        );
    }

    #[test]
    fn test_completely_different_files() {
        let old_source = r#"fn old_function() {
    println!("old");
}"#;
        let new_source = r#"struct NewStruct {
    field: i32,
}"#;

        let old_tree = parse_rust(old_source);
        let new_tree = parse_rust(new_source);
        let config = rust_config();

        let (old_ranges, new_ranges) = diff_syntax(
            &old_tree,
            old_source,
            &new_tree,
            new_source,
            &config,
            &DiffOptions::default(),
        )
        .unwrap();

        assert!(
            !old_ranges.is_empty(),
            "Should detect all old content as changed"
        );
        assert!(
            !new_ranges.is_empty(),
            "Should detect all new content as novel"
        );
    }
}
