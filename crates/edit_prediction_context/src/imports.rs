use anyhow::Result;
use anyhow::anyhow;
use collections::HashMap;
use language::BufferSnapshot;
use language::ImportsConfig;
use language::LanguageId;
use std::rc::Rc;
use std::{borrow::Cow, ops::Range};
use util::RangeExt;

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
// Initial goals:
//
// * Get region that has top-of-file imports
//
// * symbol -> (namespace, count)

#[derive(Debug, Clone)]
pub struct Imports {
    pub range: Range<usize>,
    pub symbols: HashMap<Identifier, Vec<Namespace>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Namespace(pub Vec<Rc<str>>);

impl Imports {
    pub fn collect(snapshot: &BufferSnapshot) -> Result<Self> {
        // TODO: Should this only work for the outer syntax layer?

        // Query to match different import patterns
        let mut matches = snapshot
            .syntax
            .matches(0..snapshot.len(), &snapshot.text, |grammar| {
                grammar.imports_config().map(|imports| &imports.query)
            });

        let mut import_range: Option<Range<usize>> = None;
        let mut namespace_appends: Vec<NamespaceAppend> = Vec::new();
        let mut symbols: Vec<Range<usize>> = Vec::new();

        while let Some(query_match) = matches.peek() {
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
                    if let Some(import_range) = import_range.as_mut() {
                        import_range.end = capture_range.end;
                    } else {
                        import_range = Some(capture_range.clone());
                    }
                    name_range.take();
                    path_range.take();
                    list_range.take();
                } else if capture.index == *name_ix {
                    name_range = Some(capture_range);
                } else if capture.index == *path_ix {
                    path_range = Some(capture_range);
                } else if capture.index == *list_ix {
                    list_range = Some(capture_range);
                }
            }

            if let Some(path_range) = path_range {
                if let Some(name_range) = name_range {
                    if list_range.is_some() {
                        // todo! improve
                        log::error!("Unexpected match of both list and name");
                    }
                    namespace_appends.push(NamespaceAppend {
                        path: path_range,
                        suffix: name_range,
                    });
                } else if let Some(list_range) = list_range {
                    namespace_appends.push(NamespaceAppend {
                        path: path_range,
                        suffix: list_range.clone(),
                    });
                }
            } else if let Some(name_range) = name_range {
                symbols.push(name_range);
            }

            matches.advance();
        }

        let Some(import_range) = import_range else {
            return Err(anyhow!("No imports"));
        };

        let mut nodes = Vec::new();
        for append in namespace_appends {
            if !append.add_to_nodes(&mut nodes) {
                nodes.push(Node {
                    path: append.path.clone(),
                    path_children: Vec::new(),
                    suffix: append.suffix.clone(),
                    suffix_children: Vec::new(),
                });
            }
        }

        for symbol in symbols {
            if !add_symbol_to_nodes(&mut nodes, &symbol) {
                // todo!
                log::error!("bug: parent for symbol not found");
            }
        }

        let mut import_symbols = HashMap::default();
        for node in &nodes {
            let mut namespace = Namespace::default();
            node.add_to_symbols(&mut namespace, &mut import_symbols, snapshot);
        }

        Ok(Imports {
            range: import_range,
            symbols: import_symbols,
        })
    }
}

#[derive(Default, Debug)]
struct Node {
    path: Range<usize>,
    path_children: Vec<Node>,
    suffix: Range<usize>,
    suffix_children: Vec<Node>,
}

impl Node {
    fn range(&self) -> Range<usize> {
        self.path.start..self.suffix.end
    }

    #[allow(dead_code)]
    fn debug<'a>(&'a self, snapshot: &'a BufferSnapshot) -> NodeDebug<'a> {
        NodeDebug {
            node: self,
            snapshot,
        }
    }

    fn add_to_symbols(
        &self,
        namespace: &mut Namespace,
        symbols: &mut HashMap<Identifier, Vec<Namespace>>,
        snapshot: &BufferSnapshot,
    ) {
        let mut pop_count = 0;

        if self.path_children.is_empty() {
            if !self.path.is_empty() {
                namespace.0.push(Self::range_text(snapshot, &self.path));
                pop_count += 1;
            }
        } else {
            for child in &self.path_children {
                pop_count += child.add_path_to_namespace(namespace, snapshot);
            }
        };

        if self.suffix_children.is_empty() {
            // todo! language id
            symbols
                .entry(Identifier {
                    language_id: LanguageId::new(),
                    name: Self::range_text(snapshot, &self.suffix).as_ref().into(),
                })
                .or_default()
                .push(namespace.clone());
        } else {
            for child in &self.suffix_children {
                child.add_to_symbols(namespace, symbols, snapshot);
            }
        }

        namespace.0.drain(namespace.0.len() - pop_count..);
    }

    fn add_path_to_namespace(&self, namespace: &mut Namespace, snapshot: &BufferSnapshot) -> usize {
        let mut pop_count = 0;
        if self.path_children.is_empty() {
            if !self.path.is_empty() {
                namespace.0.push(Self::range_text(snapshot, &self.path));
                pop_count += 1;
            }
        } else {
            for child in &self.path_children {
                pop_count += child.add_path_to_namespace(namespace, snapshot);
            }
        }
        if self.suffix_children.is_empty() {
            namespace.0.push(Self::range_text(snapshot, &self.suffix));
            pop_count += 1;
        } else {
            for child in &self.suffix_children {
                pop_count += child.add_path_to_namespace(namespace, snapshot);
            }
        }
        pop_count
    }

    fn range_text(snapshot: &BufferSnapshot, range: &Range<usize>) -> Rc<str> {
        snapshot
            .text_for_range(range.clone())
            .collect::<Cow<str>>()
            .into()
    }
}

#[derive(Debug)]
struct NamespaceAppend {
    path: Range<usize>,
    suffix: Range<usize>,
}

impl NamespaceAppend {
    fn range(&self) -> Range<usize> {
        self.path.start..self.suffix.end
    }

    fn add_to_nodes(&self, nodes: &mut Vec<Node>) -> bool {
        for node in nodes {
            if self.path.contains_inclusive(&node.range()) {
                // TODO: Consider whether this should recurse?
                let mut new_parent = Node {
                    path: self.path.clone(),
                    path_children: Vec::new(),
                    suffix: self.suffix.clone(),
                    suffix_children: Vec::new(),
                };
                std::mem::swap(node, &mut new_parent);
                let old_node = new_parent;
                node.path_children.push(old_node);
                return true;
            } else if node.suffix.contains_inclusive(&self.range()) {
                if self.add_to_nodes(&mut node.suffix_children) {
                    return true;
                }
                node.suffix_children.push(Node {
                    path: self.path.clone(),
                    path_children: Vec::new(),
                    suffix: self.suffix.clone(),
                    suffix_children: Vec::new(),
                });
                return true;
            }
        }
        false
    }
}

fn add_symbol_to_nodes(nodes: &mut Vec<Node>, symbol: &Range<usize>) -> bool {
    for node in nodes {
        if node.suffix.contains_inclusive(symbol) {
            if add_symbol_to_nodes(&mut node.suffix_children, symbol) {
                return true;
            }
            node.suffix_children.push(Node {
                path: 0..0,
                path_children: vec![],
                suffix: symbol.clone(),
                suffix_children: vec![],
            });
            return true;
        }
    }
    false
}

struct NodeDebug<'a> {
    node: &'a Node,
    snapshot: &'a BufferSnapshot,
}

impl std::fmt::Debug for NodeDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("path_range", &self.node.path)
            .field(
                "path_text",
                &self
                    .snapshot
                    .text_for_range(self.node.path.clone())
                    .collect::<String>(),
            )
            .field(
                "path_children",
                &self
                    .node
                    .path_children
                    .iter()
                    .map(|child| child.debug(&self.snapshot))
                    .collect::<Vec<Self>>(),
            )
            .field("suffix_range", &self.node.suffix)
            .field(
                "suffix_text",
                &self
                    .snapshot
                    .text_for_range(self.node.suffix.clone())
                    .collect::<String>(),
            )
            .field(
                "suffix_children",
                &self
                    .node
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
                .symbols
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

    fn to_rust_qualified_identifier(namespace: &Namespace, identifier: &Identifier) -> String {
        format!(
            "{}::{}",
            namespace.0.iter().map(|s| s.to_string()).join("::"),
            identifier.name.to_string()
        )
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
