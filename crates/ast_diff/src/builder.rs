//! Build `Syntax` trees from tree-sitter parse trees.

use bumpalo::Bump;
use tree_sitter::{Tree, TreeCursor};

use crate::config::SyntaxDiffConfig;
use crate::syntax::{AtomKind, StringKind, Syntax};

pub fn build_syntax<'a>(
    arena: &'a Bump,
    tree: &Tree,
    source: &str,
    config: &dyn SyntaxDiffConfig,
) -> Vec<&'a Syntax<'a>> {
    let root = tree.root_node();

    log::trace!(
        "Building syntax tree from root node: kind={}, children={}, range={:?}",
        root.kind(),
        root.child_count(),
        root.byte_range()
    );

    let mut cursor = root.walk();
    cursor.goto_first_child();

    let result = all_syntaxes_from_cursor(arena, source, &mut cursor, config);

    log::trace!("Built {} root-level syntax nodes", result.len());

    result
}

fn all_syntaxes_from_cursor<'a>(
    arena: &'a Bump,
    source: &str,
    cursor: &mut TreeCursor,
    config: &dyn SyntaxDiffConfig,
) -> Vec<&'a Syntax<'a>> {
    let mut nodes = Vec::new();

    loop {
        if let Some(syntax) = syntax_from_cursor(arena, source, cursor, config) {
            nodes.push(syntax);
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    nodes
}

fn syntax_from_cursor<'a>(
    arena: &'a Bump,
    source: &str,
    cursor: &mut TreeCursor,
    config: &dyn SyntaxDiffConfig,
) -> Option<&'a Syntax<'a>> {
    let node = cursor.node();

    if node.byte_range().is_empty() {
        return None;
    }

    let node_kind = node.kind();
    let child_count = node.child_count();

    if config.is_atom_node(node_kind) {
        log::trace!(
            "Node '{}' forced to atom by config (children={})",
            node_kind,
            child_count
        );
        return Some(atom_from_cursor(arena, source, cursor, config));
    }

    if config.is_comment(node_kind) || config.is_string(node_kind) {
        log::trace!(
            "Node '{}' treated as atom (comment/string heuristic, children={})",
            node_kind,
            child_count
        );
        return Some(atom_from_cursor(arena, source, cursor, config));
    }

    if child_count > 0 {
        return Some(list_from_cursor(arena, source, cursor, config));
    }

    Some(atom_from_cursor(arena, source, cursor, config))
}

fn atom_from_cursor<'a>(
    arena: &'a Bump,
    source: &str,
    cursor: &mut TreeCursor,
    config: &dyn SyntaxDiffConfig,
) -> &'a Syntax<'a> {
    let node = cursor.node();
    let byte_range = node.byte_range();
    let content = source.get(byte_range.clone()).unwrap_or("").to_string();
    let kind = determine_atom_kind(node.kind(), config);

    Syntax::new_atom(arena, byte_range, content, kind)
}

fn determine_atom_kind(node_kind: &str, config: &dyn SyntaxDiffConfig) -> AtomKind {
    if node_kind == "ERROR" {
        return AtomKind::TreeSitterError;
    }

    if config.is_comment(node_kind) {
        return AtomKind::Comment;
    }

    if config.is_string(node_kind) {
        return AtomKind::String(StringKind::StringLiteral);
    }

    if config.is_keyword(node_kind) {
        return AtomKind::Keyword;
    }

    if config.is_type(node_kind) {
        return AtomKind::Type;
    }

    AtomKind::Normal
}

fn child_tokens<'a>(source: &'a str, cursor: &mut TreeCursor) -> Vec<Option<&'a str>> {
    let mut tokens = Vec::new();
    let node = cursor.node();

    let mut child_cursor = node.walk();
    if child_cursor.goto_first_child() {
        loop {
            let child = child_cursor.node();
            let text = source.get(child.byte_range());
            tokens.push(text);

            if !child_cursor.goto_next_sibling() {
                break;
            }
        }
    }

    tokens
}

fn find_delim_positions(
    source: &str,
    cursor: &mut TreeCursor,
    config: &dyn SyntaxDiffConfig,
) -> Option<(usize, usize)> {
    let tokens = child_tokens(source, cursor);

    for (i, token) in tokens.iter().enumerate() {
        if let Some(token_text) = token {
            if let Some(close_delim) = config.get_matching_delimiter(token_text) {
                for (j, close_token) in tokens.iter().enumerate().skip(i + 1) {
                    if let Some(close_text) = close_token {
                        if *close_text == close_delim {
                            return Some((i, j));
                        }
                    }
                }
            }
        }
    }

    None
}

fn list_from_cursor<'a>(
    arena: &'a Bump,
    source: &str,
    cursor: &mut TreeCursor,
    config: &dyn SyntaxDiffConfig,
) -> &'a Syntax<'a> {
    let root_node = cursor.node();
    let root_range = root_node.byte_range();

    let outer_open_content = "";
    let outer_open_range = root_range.start..root_range.start;
    let outer_close_content = "";
    let outer_close_range = root_range.end..root_range.end;

    let (open_idx, close_idx) = match find_delim_positions(source, cursor, config) {
        Some((i, j)) => (i as isize, j as isize),
        None => (-1, root_node.child_count() as isize),
    };

    let mut inner_open_content = outer_open_content;
    let mut inner_open_range = outer_open_range.clone();
    let mut inner_close_content = outer_close_content;
    let mut inner_close_range = outer_close_range.clone();

    let mut before_delim = Vec::new();
    let mut between_delim = Vec::new();
    let mut after_delim = Vec::new();

    cursor.goto_first_child();
    let mut node_i: isize = 0;

    loop {
        let child_node = cursor.node();

        if node_i < open_idx {
            if let Some(syntax) = syntax_from_cursor(arena, source, cursor, config) {
                before_delim.push(syntax);
            }
        } else if node_i == open_idx {
            let range = child_node.byte_range();
            inner_open_content = source.get(range.clone()).unwrap_or("");
            inner_open_range = range;
        } else if node_i < close_idx {
            if let Some(syntax) = syntax_from_cursor(arena, source, cursor, config) {
                between_delim.push(syntax);
            }
        } else if node_i == close_idx {
            let range = child_node.byte_range();
            inner_close_content = source.get(range.clone()).unwrap_or("");
            inner_close_range = range;
        } else {
            if let Some(syntax) = syntax_from_cursor(arena, source, cursor, config) {
                after_delim.push(syntax);
            }
        }

        if !cursor.goto_next_sibling() {
            break;
        }
        node_i += 1;
    }
    cursor.goto_parent();

    let inner_list = Syntax::new_list(
        arena,
        inner_open_content,
        inner_open_range,
        between_delim,
        inner_close_content,
        inner_close_range,
    );

    if before_delim.is_empty() && after_delim.is_empty() {
        inner_list
    } else {
        let mut children = before_delim;
        children.push(inner_list);
        children.append(&mut after_delim);

        Syntax::new_list(
            arena,
            outer_open_content,
            outer_open_range,
            children,
            outer_close_content,
            outer_close_range,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DefaultConfig;

    fn parse_json(source: &str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_json::LANGUAGE.into())
            .expect("Error loading JSON grammar");
        parser.parse(source, None).expect("Failed to parse")
    }

    #[test]
    fn test_build_simple_atom() {
        let arena = Bump::new();
        let source = "42";
        let tree = parse_json(source);
        let config = DefaultConfig::default();

        let roots = build_syntax(&arena, &tree, source, &config);

        assert_eq!(roots.len(), 1);
        match roots[0] {
            Syntax::Atom { content, .. } => {
                assert_eq!(content, "42");
            }
            _ => panic!("Expected atom"),
        }
    }

    #[test]
    fn test_build_string_literal() {
        let arena = Bump::new();
        let source = r#""hello""#;
        let tree = parse_json(source);
        let config = DefaultConfig::default();

        let roots = build_syntax(&arena, &tree, source, &config);

        assert_eq!(roots.len(), 1);
        match roots[0] {
            Syntax::Atom { content, kind, .. } => {
                assert_eq!(content, r#""hello""#);
                assert!(matches!(kind, AtomKind::String(_)));
            }
            _ => panic!("Expected atom"),
        }
    }

    #[test]
    fn test_build_array() {
        let arena = Bump::new();
        let source = "[1, 2, 3]";
        let tree = parse_json(source);
        let config = DefaultConfig::default();

        let roots = build_syntax(&arena, &tree, source, &config);

        assert_eq!(roots.len(), 1);
        match roots[0] {
            Syntax::List {
                open_content,
                close_content,
                children,
                ..
            } => {
                assert_eq!(open_content, "[");
                assert_eq!(close_content, "]");
                // Should have 3 numbers (commas might be separate nodes depending on grammar)
                assert!(!children.is_empty());
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_build_object() {
        let arena = Bump::new();
        let source = r#"{"key": "value"}"#;
        let tree = parse_json(source);
        let config = DefaultConfig::default();

        let roots = build_syntax(&arena, &tree, source, &config);

        assert_eq!(roots.len(), 1);
        match roots[0] {
            Syntax::List {
                open_content,
                close_content,
                ..
            } => {
                assert_eq!(open_content, "{");
                assert_eq!(close_content, "}");
            }
            _ => panic!("Expected list"),
        }
    }

    #[test]
    fn test_build_nested_structure() {
        let arena = Bump::new();
        let source = r#"{"items": [1, 2]}"#;
        let tree = parse_json(source);
        let config = DefaultConfig::default();

        let roots = build_syntax(&arena, &tree, source, &config);

        assert_eq!(roots.len(), 1);

        fn count_nodes(node: &Syntax) -> usize {
            match node {
                Syntax::Atom { .. } => 1,
                Syntax::List { children, .. } => {
                    1 + children.iter().map(|c| count_nodes(c)).sum::<usize>()
                }
            }
        }

        // Should have multiple nodes in the tree
        let total = count_nodes(roots[0]);
        assert!(total > 1, "Expected nested structure, got {} nodes", total);
    }

    #[test]
    fn test_empty_source() {
        let arena = Bump::new();
        let source = "";
        let tree = parse_json(source);
        let config = DefaultConfig::default();

        let roots = build_syntax(&arena, &tree, source, &config);

        assert!(roots.is_empty());
    }
}
