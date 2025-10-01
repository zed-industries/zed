use anyhow::Result;
use anyhow::anyhow;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use tree_sitter::{Node, Query, QueryCursor, StreamingIterator, Tree};

use crate::{
    identifier_index::Identifier, treesitter_util::range_is_superset_of, zed_code::Language,
};

pub struct CollectImportsInput<'a> {
    pub language: &'a Language,
    pub tree: &'a Tree,
    pub source: &'a str,
}

// Initial goals:
//
// * Get region that has top-of-file imports
//
// * symbol -> (namespace, count)

#[derive(Debug, Clone)]
pub struct Imports {
    pub range: Range<usize>,
    pub symbols: Vec<(Namespace, Identifier)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace(Vec<Identifier>);

impl CollectImportsInput<'_> {
    pub fn collect_imports(&self) -> Result<Imports> {
        match self.language.name.0.as_ref() {
            "rust" => self.rust_imports_via_query(),
            _ => Err(anyhow!("Unsupported language")),
        }
    }

    fn rust_imports_via_query(&self) -> Result<Imports> {
        // Query to match different import patterns
        let query_source = r#"
            (use_declaration) @import_statement

            (scoped_use_list
              path: (_) @import_prefix
              list: (_) @prefixed_contents)

            (scoped_identifier
              path: (_) @import_prefix
              name: (_) @prefixed_contents)

            (use_list (identifier) @import)

            (use_declaration argument:(identifier) @import)
        "#;

        let query = Query::new(&self.language.ts_language, query_source)?;

        let mut query_cursor = QueryCursor::new();
        let mut matches =
            query_cursor.matches(&query, self.tree.root_node(), self.source.as_bytes());

        let import_statement_ix = query.capture_index_for_name("import_statement").unwrap();
        let import_prefix_ix = query.capture_index_for_name("import_prefix");
        let prefixed_contents_ix = query.capture_index_for_name("prefixed_contents");
        let import_ix = query.capture_index_for_name("import");

        let mut containing_namespaces = Vec::<(Identifier, Range<usize>)>::new();
        let mut import_range: Option<Range<usize>> = None;
        let mut import_statement_range = None;
        let mut import_symbols = Vec::new();

        while let Some(query_match) = matches.next() {
            let mut prefix_range = None;
            let mut prefixed_contents_range = None;
            let mut name_range = None;
            for capture in query_match.captures {
                let capture_range = capture.node.byte_range();
                if capture.index == import_statement_ix {
                    import_statement_range = Some(capture_range.clone());
                    if let Some(import_range) = import_range.as_mut() {
                        import_range.end = capture_range.end;
                    } else {
                        import_range = Some(capture_range.clone());
                    }
                } else if Some(capture.index) == import_prefix_ix {
                    prefix_range = Some(capture_range);
                } else if Some(capture.index) == prefixed_contents_ix {
                    prefixed_contents_range = Some(capture_range);
                } else if Some(capture.index) == import_ix {
                    name_range = Some(capture_range);
                }
            }

            if let Some((prefix_range, prefixed_contents_range)) =
                prefix_range.zip(prefixed_contents_range)
            {
                if import_statement_range
                    .as_ref()
                    .map_or(false, |import_statement_range| {
                        range_is_superset_of(import_statement_range, &prefixed_contents_range)
                    })
                {
                    while containing_namespaces
                        .last()
                        .is_some_and(|(_, containing_range)| {
                            !range_is_superset_of(containing_range, &prefixed_contents_range)
                        })
                    {
                        containing_namespaces.pop();
                    }
                    if let Some(prefix) = self.source.get(prefix_range) {
                        containing_namespaces
                            .push((Identifier(prefix.into()), prefixed_contents_range));
                    }
                } else {
                    import_statement_range.take();
                }
            }

            if let Some(range) = name_range {
                if import_statement_range
                    .as_ref()
                    .map_or(false, |import_statement_range| {
                        range_is_superset_of(import_statement_range, &range)
                    })
                {
                    while containing_namespaces
                        .last()
                        .is_some_and(|(_, containing_range)| {
                            !range_is_superset_of(containing_range, &range)
                        })
                    {
                        containing_namespaces.pop();
                    }
                    if let Some(import_name) = self.source.get(range) {
                        let namespace = containing_namespaces
                            .iter()
                            .map(|(identifier, _)| identifier.clone())
                            .collect::<Vec<_>>();
                        import_symbols.push((Namespace(namespace), Identifier(import_name.into())));
                    }
                } else {
                    import_statement_range.take();
                }
            }
        }

        let Some(import_range) = import_range else {
            return Err(anyhow!("No imports"));
        };

        Ok(Imports {
            range: import_range,
            symbols: import_symbols,
        })
    }

    fn extract_namespace_components(node: Node, namespace: &mut Vec<Identifier>, source: &str) {
        match node.kind() {
            "scoped_identifier" => {
                if let Some(path_node) = node.child_by_field_name("path") {
                    Self::extract_namespace_components(path_node, namespace, source);
                }
                if let Some(name_node) = node.child_by_field_name("name") {
                    if name_node.kind() == "identifier" {
                        let name_text = Self::node_text(&name_node, source);
                        namespace.push(Identifier(name_text.into()));
                    }
                }
            }
            "identifier" => {
                let name_text = Self::node_text(&node, source);
                namespace.push(Identifier(name_text.into()));
            }
            _ => {
                // For other node types, try to extract text directly
                let text = Self::node_text(&node, source);
                if !text.trim().is_empty() {
                    namespace.push(Identifier(text.into()));
                }
            }
        }
    }

    fn extract_symbols_from_node(
        node: Node,
        current_namespace: &Vec<Identifier>,
        symbols: &mut Vec<(Namespace, Identifier)>,
        scoped_identifier_id: u16,
        scoped_use_list_id: u16,
        use_list_id: u16,
        identifier_id: u16,
        source: &str,
    ) {
        match node.kind_id() {
            id if id == scoped_identifier_id => {
                // Handle scoped_identifier: path::name
                if let (Some(path_node), Some(name_node)) = (
                    node.child_by_field_name("path"),
                    node.child_by_field_name("name"),
                ) {
                    let mut namespace = current_namespace.clone();
                    Self::extract_path_to_namespace(
                        path_node,
                        &mut namespace,
                        scoped_identifier_id,
                        identifier_id,
                        source,
                    );

                    if name_node.kind_id() == identifier_id {
                        let name_text = Self::node_text(&name_node, source);
                        symbols.push((Namespace(namespace), Identifier(name_text.into())));
                    }
                }
            }
            id if id == scoped_use_list_id => {
                // Handle scoped_use_list: path::{list}
                if let (Some(path_node), Some(list_node)) = (
                    node.child_by_field_name("path"),
                    node.child_by_field_name("list"),
                ) {
                    let mut namespace = current_namespace.clone();
                    Self::extract_path_to_namespace(
                        path_node,
                        &mut namespace,
                        scoped_identifier_id,
                        identifier_id,
                        source,
                    );

                    Self::extract_symbols_from_node(
                        list_node,
                        &namespace,
                        symbols,
                        scoped_identifier_id,
                        scoped_use_list_id,
                        use_list_id,
                        identifier_id,
                        source,
                    );
                }
            }
            id if id == use_list_id => {
                // Handle use_list: {item1, item2, ...}
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        let child = cursor.node();
                        if child.is_named() {
                            Self::extract_symbols_from_node(
                                child,
                                current_namespace,
                                symbols,
                                scoped_identifier_id,
                                scoped_use_list_id,
                                use_list_id,
                                identifier_id,
                                source,
                            );
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
            }
            id if id == identifier_id => {
                // Handle simple identifier
                let name_text = Self::node_text(&node, source);
                symbols.push((
                    Namespace(current_namespace.clone()),
                    Identifier(name_text.into()),
                ));
            }
            _ => {
                // For other node types, recurse through children
                let mut cursor = node.walk();
                if cursor.goto_first_child() {
                    loop {
                        let child = cursor.node();
                        if child.is_named() {
                            Self::extract_symbols_from_node(
                                child,
                                current_namespace,
                                symbols,
                                scoped_identifier_id,
                                scoped_use_list_id,
                                use_list_id,
                                identifier_id,
                                source,
                            );
                        }
                        if !cursor.goto_next_sibling() {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn extract_path_to_namespace(
        path_node: Node,
        namespace: &mut Vec<Identifier>,
        scoped_identifier_id: u16,
        identifier_id: u16,
        source: &str,
    ) {
        match path_node.kind_id() {
            id if id == scoped_identifier_id => {
                // Recursive case: path::name
                if let (Some(inner_path), Some(name)) = (
                    path_node.child_by_field_name("path"),
                    path_node.child_by_field_name("name"),
                ) {
                    Self::extract_path_to_namespace(
                        inner_path,
                        namespace,
                        scoped_identifier_id,
                        identifier_id,
                        source,
                    );
                    if name.kind_id() == identifier_id {
                        namespace.push(Identifier(Self::node_text(&name, source).into()));
                    }
                }
            }
            id if id == identifier_id => {
                // Base case: simple identifier
                namespace.push(Identifier(Self::node_text(&path_node, source).into()));
            }
            _ => {
                // Fallback: treat as identifier
                namespace.push(Identifier(Self::node_text(&path_node, source).into()));
            }
        }
    }

    fn node_text<'a>(node: &Node, source: &'a str) -> &'a str {
        let range = node.byte_range();
        &source[range]
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, LazyLock};

    use itertools::Itertools;

    use crate::{
        treesitter_util::{language_for_name, load_languages, parse_source},
        zed_code::LanguageName,
    };

    use super::*;

    static LANGUAGES: LazyLock<Vec<Arc<Language>>> = LazyLock::new(|| load_languages());

    fn tree_to_string(tree: &tree_sitter::Tree) -> String {
        let mut cursor = tree.walk();
        let mut result = String::new();
        let mut depth = 0;
        loop {
            result.push_str(&"  ".repeat(depth));
            if let Some(field_name) = cursor.field_name() {
                result.push_str(field_name);
                result.push_str(": ");
            }
            if cursor.node().is_named() {
                result.push_str(cursor.node().kind());
            } else {
                result.push('"');
                result.push_str(cursor.node().kind());
                result.push('"');
            }
            result.push('\n');

            if cursor.goto_first_child() {
                depth += 1;
                continue;
            }
            if cursor.goto_next_sibling() {
                continue;
            }
            if cursor.goto_parent() {
                depth -= 1;
                if cursor.goto_next_sibling() {
                    continue;
                }
            }
            break;
        }
        result
    }

    fn run_collect_imports(language_name: &str, source: &str) -> (Option<Imports>, Tree) {
        let language = language_for_name(&LANGUAGES, &LanguageName(language_name.into())).unwrap();
        let tree = parse_source(&language, source);
        let input = CollectImportsInput {
            language: &language,
            tree: &tree,
            source,
        };
        (input.collect_imports().ok(), tree)
    }

    fn rust_symbol(symbol: &str) -> (Namespace, Identifier) {
        let parts = symbol.split("::").collect::<Vec<&str>>();
        let last_part = parts.len().saturating_sub(1);
        (
            Namespace(
                parts[0..last_part]
                    .iter()
                    .map(|part| Identifier((*part).into()))
                    .collect::<Vec<_>>(),
            ),
            Identifier(parts[last_part].into()),
        )
    }

    struct ImportsFailure {
        expected: Vec<(Namespace, Identifier)>,
        actual: Vec<(Namespace, Identifier)>,
        source: String,
        tree: Tree,
    }

    impl std::fmt::Display for ImportsFailure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Expected imports: {:?}\n\
                Actual imports: {:?}\n\
                Source:\n{}\n\
                Tree:\n{}",
                self.expected,
                self.actual,
                self.source,
                tree_to_string(&self.tree)
            )
        }
    }

    #[test]
    fn test_collect_rust_imports() {
        let examples = vec![
            (
                "use std::collections::HashMap;",
                vec!["std::collections::HashMap"],
            ),
            /*
            (
                "pub use std::collections::HashMap;",
                vec!["std::collections::HashMap"],
            ),
            (
                "use std::collections::{HashMap, HashSet};",
                vec!["std::collections::HashMap", "std::collections::HashSet"],
            ),
            (
                "use std::{any::TypeId, collections::{HashMap, HashSet};",
                vec![
                    "std::any::TypeId",
                    "std::collections::HashMap",
                    "std::collections::HashSet",
                ],
            ),
            */
        ];
        let mut failures = Vec::new();
        for (source, expected) in examples {
            let (imports, tree) = run_collect_imports("rust", source);
            let imports = imports.expect(&format!(
                "Failed to collect imports for source:\n{}",
                source
            ));
            let expected_symbols = expected
                .iter()
                .map(|symbol| rust_symbol(symbol))
                .collect::<Vec<_>>();
            if imports.symbols != expected_symbols {
                failures.push(ImportsFailure {
                    expected: expected_symbols,
                    actual: imports.symbols,
                    source: source.to_string(),
                    tree,
                });
            }
        }

        if !failures.is_empty() {
            panic!(
                "{} cases failed:\n\n{}",
                failures.len(),
                failures
                    .into_iter()
                    .map(|failure| failure.to_string())
                    .join("\n")
            )
        }
    }
}
