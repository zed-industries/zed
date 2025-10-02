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

#[derive(Debug, Clone, PartialEq, Eq)]
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

        #[derive(Debug)]
        struct NamespaceAppend {
            path: Range<usize>,
            suffix: Range<usize>,
            singleton_suffix: bool,
        }

        impl NamespaceAppend {
            fn range(&self) -> Range<usize> {
                self.path.start..self.suffix.end
            }
        }

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

            let name_text = name_range
                .clone()
                .map(|range| snapshot.text_for_range(range).collect::<String>());
            let path_text = path_range
                .clone()
                .map(|range| snapshot.text_for_range(range).collect::<String>());
            let list_text = list_range
                .clone()
                .map(|range| snapshot.text_for_range(range).collect::<String>());

            println!("MATCH");
            if let Some(path_text) = &path_text {
                println!("Path: {}", path_text);
            }
            if let Some(name_text) = &name_text {
                println!("Name: {}", name_text);
            }
            if let Some(list_text) = &list_text {
                println!("List: {}", list_text);
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
                        singleton_suffix: true,
                    });
                } else if let Some(list_range) = list_range {
                    namespace_appends.push(NamespaceAppend {
                        path: path_range,
                        suffix: list_range.clone(),
                        singleton_suffix: false,
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

        dbg!(&namespace_appends);
        dbg!(&symbols);

        #[derive(Debug)]
        struct Node {
            path: Range<usize>,
            suffix: Range<usize>,
            suffix_children: Vec<Node>,
            parent_is_path: bool,
        }

        impl Node {
            fn range(&self) -> Range<usize> {
                self.path.start..self.suffix.end
            }
        }

        let mut appends_iter = namespace_appends.into_iter();

        // todo! take Vec<Node>?
        fn add_to_node(node: &mut Node, append: &NamespaceAppend) -> bool {
            let parent_is_path = append.path.contains_inclusive(&node.range());
            if parent_is_path || node.suffix.contains_inclusive(&append.range()) {
                for node in &mut node.suffix_children {
                    if add_to_node(node, &append) {
                        return true;
                    }
                }
                node.suffix_children.push(Node {
                    path: append.path.clone(),
                    suffix: append.suffix.clone(),
                    suffix_children: Vec::new(),
                    parent_is_path,
                });
                true
            } else {
                false
            }
        }

        let mut nodes = vec![];

        if let Some(initial_append) = appends_iter.next() {
            nodes.push(Node {
                path: initial_append.path,
                suffix: initial_append.suffix,
                suffix_children: Vec::new(),
                parent_is_path: false,
            });
            'outer: for append in appends_iter {
                for node in &mut nodes {
                    if add_to_node(node, &append) {
                        continue 'outer;
                    }
                }
                nodes.push(Node {
                    path: append.path.clone(),
                    suffix: append.suffix.clone(),
                    suffix_children: Vec::new(),
                    parent_is_path: false,
                });
            }
            dbg!(&nodes);
        }

        let mut import_symbols = HashMap::default();

        fn range_text(snapshot: &BufferSnapshot, range: &Range<usize>) -> Rc<str> {
            snapshot
                .text_for_range(range.clone())
                .collect::<Cow<str>>()
                .into()
        }

        fn add_children_to_namespace(
            node: &Node,
            namespace: &mut Namespace,
            symbols: &Vec<Range<usize>>,
            import_symbols: &mut HashMap<Identifier, Vec<Namespace>>,
            snapshot: &BufferSnapshot,
        ) {
            let child_symbols = if !node.suffix_children.is_empty() {
                vec![]
            } else {
                symbols
                    .iter()
                    .filter(|symbol| node.suffix.contains_inclusive(&symbol))
                    .collect()
            };

            let mut pop_count = 0;
            if !node.parent_is_path {
                let text = range_text(snapshot, &node.path);
                namespace.0.push(text);
                pop_count += 1;
            }

            if child_symbols.is_empty() && node.parent_is_path {
                let text = range_text(snapshot, &node.suffix);
                namespace.0.push(text);
                pop_count += 1;
            }

            for child in &node.suffix_children {
                add_children_to_namespace(&child, namespace, symbols, import_symbols, snapshot);
            }

            if node.suffix_children.is_empty() {
                for symbol in &child_symbols {
                    let symbol_text = range_text(&snapshot, &symbol);
                    import_symbols
                        .entry(Identifier {
                            // todo!
                            language_id: LanguageId::new(),
                            name: symbol_text.as_ref().into(),
                        })
                        .or_default()
                        .push(namespace.clone());
                }
                if child_symbols.is_empty() {
                    if node.parent_is_path {
                        import_symbols
                            .entry(Identifier {
                                // todo!
                                language_id: LanguageId::new(),
                                name: dbg!(namespace.0.last().unwrap().as_ref().into()),
                            })
                            .or_default()
                            .push(Namespace(namespace.0[..namespace.0.len() - 1].to_vec()));
                    } else {
                        import_symbols
                            .entry(Identifier {
                                // todo!
                                language_id: LanguageId::new(),
                                name: dbg!(range_text(snapshot, &node.suffix).as_ref().into()),
                            })
                            .or_default()
                            .push(namespace.clone());
                    }
                }
            }

            for _ in 0..pop_count {
                namespace.0.pop();
            }
        }

        for node in nodes {
            let mut namespace = Namespace(vec![]);

            namespace.0.push(range_text(snapshot, &node.path));
            if node
                .suffix_children
                .first()
                .is_none_or(|child| child.parent_is_path)
            {
                namespace.0.push(range_text(snapshot, &node.suffix));
            }

            for child in node.suffix_children {
                add_children_to_namespace(
                    &child,
                    &mut namespace,
                    &symbols,
                    &mut import_symbols,
                    snapshot,
                );
            }

            namespace.0.pop();
        }

        Ok(Imports {
            range: import_range,
            symbols: import_symbols,
        })
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
            // todo!
            // ("use std::{a::b::HashMap};", vec!["std::a::b::HashMap"]),
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
                /*
                .flat_map(|(identifier, namespaces)| {
                    namespaces
                        .iter()
                        .map(|namespace| to_rust_qualified_identifier(namespace, identifier))
                })
                */
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
