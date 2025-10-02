use anyhow::Result;
use anyhow::anyhow;
use collections::HashMap;
use language::BufferSnapshot;
use language::ImportsConfig;
use language::LanguageId;
use std::rc::Rc;
use std::{borrow::Cow, ops::Range};
use text::OffsetRangeExt;
use util::RangeExt;
use util::paths::PathStyle;

use crate::Identifier;

// Future improvements:
//
// * Support for aliases?
//
// * Scoping for imports that aren't at the top level
//
// * Consider only scanning prefix of the file / other strategies for not scanning entire file. This
// could look like having query matches that indicate it reached a declaration that is not allowed
// in the import section.
//
// * When comparing namespaces to paths, drop index.ts, lib.rs, __init__.py, etc
//
// * Only use the top syntax layer?

#[derive(Debug, Clone)]
pub struct Imports {
    // TODO: this is not so meaningful when imports come from anywhere in the file
    pub all_imports_range: Range<usize>,
    pub identifier_namespaces: HashMap<Identifier, Vec<Namespace>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Namespace(pub Vec<Rc<str>>);

impl Imports {
    pub fn collect(snapshot: &BufferSnapshot) -> Result<Self> {
        // Query to match different import patterns
        let mut matches = snapshot
            .syntax
            .matches(0..snapshot.len(), &snapshot.text, |grammar| {
                grammar.imports_config().map(|imports| &imports.query)
            });

        let mut detached_nodes: Vec<DetachedNode> = Vec::new();
        let mut identifier_namespaces = HashMap::default();
        let mut all_imports_range: Option<Range<usize>> = None;
        let mut import_range = None;

        while let Some(query_match) = matches.peek() {
            let language_id = query_match.language.id();
            let ImportsConfig {
                import_statement_ix,
                name_ix,
                path_ix,
                list_ix,
                ..
            } = matches.grammars()[query_match.grammar_index]
                .imports_config()
                .unwrap();

            let mut name_range = None;
            let mut path_range = None;
            let mut list_range = None;
            for capture in query_match.captures {
                let capture_range = capture.node.byte_range();
                if capture.index == *import_statement_ix {
                    import_range = Some(capture_range.clone());
                    all_imports_range
                        .get_or_insert_with(|| capture_range.clone())
                        .end = capture_range.end;
                    Self::collect_from_import_statement(
                        &detached_nodes,
                        &snapshot,
                        &mut identifier_namespaces,
                    );
                    detached_nodes.clear();
                } else if import_range
                    .as_ref()
                    .is_some_and(|import_range| import_range.contains_inclusive(&capture_range))
                {
                    if capture.index == *name_ix {
                        name_range = Some(capture_range);
                    } else if capture.index == *path_ix {
                        path_range = Some(capture_range);
                    } else if capture.index == *list_ix {
                        list_range = Some(capture_range);
                    }
                }
            }

            if let Some(path_range) = path_range {
                if let Some(name_range) = name_range {
                    if let Some(list_range) = list_range {
                        let point = list_range.to_point(snapshot);
                        log::warn!(
                            "bug in {} imports query: unexpected capture of both list and name ({}:{}:{})",
                            query_match.language.name(),
                            snapshot
                                .file()
                                .map(|p| p.path().display(PathStyle::Posix))
                                .unwrap_or_default(),
                            point.start.row + 1,
                            point.start.column + 1
                        );
                    }
                    detached_nodes.push(DetachedNode {
                        path: path_range,
                        suffix: name_range,
                        language_id,
                    });
                } else if let Some(list_range) = list_range {
                    detached_nodes.push(DetachedNode {
                        path: path_range,
                        suffix: list_range.clone(),
                        language_id,
                    });
                }
            } else if let Some(name_range) = name_range {
                detached_nodes.push(DetachedNode {
                    path: 0..0,
                    suffix: name_range.clone(),
                    language_id,
                });
            }

            matches.advance();
        }

        let Some(import_range) = all_imports_range else {
            return Err(anyhow!("No imports"));
        };

        Self::collect_from_import_statement(&detached_nodes, &snapshot, &mut identifier_namespaces);

        Ok(Imports {
            all_imports_range: import_range,
            identifier_namespaces,
        })
    }

    fn collect_from_import_statement(
        detached_nodes: &[DetachedNode],
        snapshot: &BufferSnapshot,
        identifier_namespaces: &mut HashMap<Identifier, Vec<Namespace>>,
    ) {
        let mut trees = Vec::new();

        for detached_node in detached_nodes {
            if !Self::attach_node(&detached_node, &mut trees) {
                trees.push(detached_node.into());
            }
        }

        for tree in &trees {
            let mut namespace = Namespace::default();
            Self::collect_from_tree(tree, snapshot, &mut namespace, identifier_namespaces);
        }
    }

    fn attach_node(detached_node: &DetachedNode, trees: &mut Vec<ImportTree>) -> bool {
        for tree in trees {
            if detached_node.path.contains_inclusive(&tree.range()) {
                let mut new_parent = detached_node.into();
                std::mem::swap(tree, &mut new_parent);
                let old_tree = new_parent;
                tree.path_children.push(old_tree);
                return true;
            } else if tree.suffix.contains_inclusive(&detached_node.suffix) {
                if Self::attach_node(detached_node, &mut tree.suffix_children) {
                    return true;
                }
                tree.suffix_children.push(detached_node.into());
                return true;
            }
        }
        false
    }

    fn collect_from_tree(
        tree: &ImportTree,
        snapshot: &BufferSnapshot,
        namespace: &mut Namespace,
        identifier_namespaces: &mut HashMap<Identifier, Vec<Namespace>>,
    ) {
        let mut pop_count = 0;

        if tree.path_children.is_empty() {
            if !tree.path.is_empty() {
                namespace.0.push(range_text(snapshot, &tree.path));
                pop_count += 1;
            }
        } else {
            for child in &tree.path_children {
                pop_count += Self::extend_namespace_from_tree(child, namespace, snapshot);
            }
        };

        if tree.suffix_children.is_empty() {
            identifier_namespaces
                .entry(Identifier {
                    language_id: tree.language_id,
                    name: range_text(snapshot, &tree.suffix).as_ref().into(),
                })
                .or_default()
                .push(namespace.clone());
        } else {
            for child in &tree.suffix_children {
                Self::collect_from_tree(child, snapshot, namespace, identifier_namespaces);
            }
        }

        namespace.0.drain(namespace.0.len() - pop_count..);
    }

    fn extend_namespace_from_tree(
        tree: &ImportTree,
        namespace: &mut Namespace,
        snapshot: &BufferSnapshot,
    ) -> usize {
        let mut pop_count = 0;
        if tree.path_children.is_empty() {
            if !tree.path.is_empty() {
                namespace.0.push(range_text(snapshot, &tree.path));
                pop_count += 1;
            }
        } else {
            for child in &tree.path_children {
                pop_count += Self::extend_namespace_from_tree(child, namespace, snapshot);
            }
        }
        if tree.suffix_children.is_empty() {
            namespace.0.push(range_text(snapshot, &tree.suffix));
            pop_count += 1;
        } else {
            for child in &tree.suffix_children {
                pop_count += Self::extend_namespace_from_tree(child, namespace, snapshot);
            }
        }
        pop_count
    }
}

fn range_text(snapshot: &BufferSnapshot, range: &Range<usize>) -> Rc<str> {
    snapshot
        .text_for_range(range.clone())
        .collect::<Cow<str>>()
        .into()
}

#[derive(Debug)]
struct DetachedNode {
    path: Range<usize>,
    suffix: Range<usize>,
    language_id: LanguageId,
}

#[derive(Debug)]
struct ImportTree {
    path: Range<usize>,
    path_children: Vec<ImportTree>,
    suffix: Range<usize>,
    suffix_children: Vec<ImportTree>,
    language_id: LanguageId,
}

impl ImportTree {
    fn range(&self) -> Range<usize> {
        self.path.start..self.suffix.end
    }

    #[allow(dead_code)]
    fn debug<'a>(&'a self, snapshot: &'a BufferSnapshot) -> NodeDebug<'a> {
        NodeDebug {
            tree: self,
            snapshot,
        }
    }
}

impl From<&DetachedNode> for ImportTree {
    fn from(value: &DetachedNode) -> Self {
        ImportTree {
            path: value.path.clone(),
            path_children: Vec::new(),
            suffix: value.suffix.clone(),
            suffix_children: Vec::new(),
            language_id: value.language_id,
        }
    }
}

struct NodeDebug<'a> {
    tree: &'a ImportTree,
    snapshot: &'a BufferSnapshot,
}

impl std::fmt::Debug for NodeDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("path_range", &self.tree.path)
            .field(
                "path_text",
                &self
                    .snapshot
                    .text_for_range(self.tree.path.clone())
                    .collect::<String>(),
            )
            .field(
                "path_children",
                &self
                    .tree
                    .path_children
                    .iter()
                    .map(|child| child.debug(&self.snapshot))
                    .collect::<Vec<Self>>(),
            )
            .field("suffix_range", &self.tree.suffix)
            .field(
                "suffix_text",
                &self
                    .snapshot
                    .text_for_range(self.tree.suffix.clone())
                    .collect::<String>(),
            )
            .field(
                "suffix_children",
                &self
                    .tree
                    .suffix_children
                    .iter()
                    .map(|child| child.debug(&self.snapshot))
                    .collect::<Vec<Self>>(),
            )
            .finish()
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use indoc::indoc;
    use itertools::Itertools;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};

    #[gpui::test]
    fn test_collect_rust_imports(cx: &mut TestAppContext) {
        let examples = vec![
            (
                "use std::collections::HashMap;",
                vec!["std::collections::HashMap"],
            ),
            (
                "use custom::std::collections::HashMap;",
                vec!["custom::std::collections::HashMap"],
            ),
            (
                "pub use std::collections::HashMap;",
                vec!["std::collections::HashMap"],
            ),
            (
                "use std::collections::{HashMap, HashSet};",
                vec!["std::collections::HashMap", "std::collections::HashSet"],
            ),
            (
                "use std::{any::TypeId, collections::{HashMap, HashSet}};",
                vec![
                    "std::any::TypeId",
                    "std::collections::HashMap",
                    "std::collections::HashSet",
                ],
            ),
            ("use std::{a::b::HashMap};", vec!["std::a::b::HashMap"]),
            (
                "use {std::any::TypeId, std::collections::{HashMap, HashSet}};",
                vec![
                    "std::any::TypeId",
                    "std::collections::HashMap",
                    "std::collections::HashSet",
                ],
            ),
            (
                indoc! {"
                    use std::collections::HashMap;
                    use std::any::{TypeId, Any};
                "},
                vec![
                    "std::collections::HashMap",
                    "std::any::TypeId",
                    "std::any::Any",
                ],
            ),
            (
                indoc! {"
                    use std::collections::HashSet;

                    fn main() {
                        let unqualified = HashSet::new();
                        let qualified = std::collections::HashMap::new();
                    }

                    use std::any::TypeId;
                "},
                vec!["std::collections::HashSet", "std::any::TypeId"],
            ),
        ];
        let language = Arc::new(rust_lang());
        let mut failures = Vec::new();
        for (source, expected) in examples {
            let buffer = cx.new(|cx| {
                let mut buffer = Buffer::local(source, cx);
                buffer.set_language(Some(language.clone()), cx);
                buffer
            });
            cx.run_until_parked();

            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

            let imports = Imports::collect(&snapshot);
            let imports = imports.expect(&format!(
                "Failed to collect imports for source:\n{}",
                source
            ));
            let mut actual_symbols = imports
                .identifier_namespaces
                .iter()
                .flat_map(|(identifier, namespaces)| {
                    namespaces.iter().map(|namespace| {
                        namespace
                            .0
                            .iter()
                            .map(|chunk| chunk.to_string())
                            .chain(std::iter::once(identifier.name.to_string()))
                            .collect::<Vec<_>>()
                    })
                })
                .collect::<Vec<_>>();
            let mut expected_symbols = expected
                .iter()
                .map(|expected| {
                    expected
                        .split("::")
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            actual_symbols.sort();
            expected_symbols.sort();
            if actual_symbols != expected_symbols {
                let top_layer = snapshot.syntax_layers().next().unwrap();
                failures.push(ImportsFailure {
                    expected_symbols,
                    actual_symbols,
                    tree: tree_to_string(&top_layer.node()),
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

    fn tree_to_string(node: &tree_sitter::Node) -> String {
        let mut cursor = node.walk();
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

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
        .with_imports_query(include_str!("../../languages/src/rust/imports.scm"))
        .unwrap()
    }

    struct ImportsFailure {
        expected_symbols: Vec<Vec<String>>,
        actual_symbols: Vec<Vec<String>>,
        tree: String,
    }

    impl std::fmt::Display for ImportsFailure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Expected imports: {:?}\n\
                Actual imports: {:?}\n\
                Tree:\n{}",
                self.expected_symbols, self.actual_symbols, self.tree,
            )
        }
    }
}
