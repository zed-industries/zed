use anyhow::Result;
use anyhow::anyhow;
use collections::HashMap;
use itertools::Itertools;
use language::BufferSnapshot;
use language::ImportsConfig;
use std::borrow::Cow;
use std::ops::Range;
use std::rc::Rc;
use util::RangeExt;

use crate::Identifier;

// Future improvements:
//
// * Support for aliases?

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
pub struct Namespace(Vec<Rc<str>>);

impl Imports {
    pub fn collect(snapshot: &BufferSnapshot) -> Result<Self> {
        // TODO: Should this only work for the outer syntax layer?

        // Query to match different import patterns
        let mut matches = snapshot
            .syntax
            .matches(0..snapshot.len(), &snapshot.text, |grammar| {
                grammar.imports_config().map(|imports| &imports.query)
            });

        let mut containing_namespaces = Vec::<(Rc<str>, Range<usize>)>::new();
        let mut import_range: Option<Range<usize>> = None;
        let mut import_statement_range = None;
        let mut import_symbols: HashMap<Identifier, Vec<Namespace>> = HashMap::default();

        while let Some(query_match) = matches.peek() {
            let ImportsConfig {
                import_statement_ix,
                import_prefix_ix,
                prefixed_contents_ix,
                import_ix,
                ..
            } = matches.grammars()[query_match.grammar_index]
                .imports_config()
                .unwrap();

            let mut prefix_range = None;
            let mut prefixed_contents_range = None;
            let mut name_range = None;
            for capture in query_match.captures {
                let capture_range = capture.node.byte_range();
                if capture.index == *import_statement_ix {
                    import_statement_range = Some(capture_range.clone());
                    if let Some(import_range) = import_range.as_mut() {
                        import_range.end = capture_range.end;
                    } else {
                        import_range = Some(capture_range.clone());
                    }
                } else if capture.index == *import_prefix_ix {
                    prefix_range = Some(capture_range);
                } else if capture.index == *prefixed_contents_ix {
                    prefixed_contents_range = Some(capture_range);
                } else if capture.index == *import_ix {
                    name_range = Some(capture_range);
                }
            }

            if let Some((prefix_range, prefixed_contents_range)) =
                prefix_range.zip(prefixed_contents_range)
            {
                if import_statement_range
                    .as_ref()
                    .map_or(false, |import_statement_range| {
                        import_statement_range.contains_inclusive(&prefixed_contents_range)
                    })
                {
                    while containing_namespaces
                        .last()
                        .is_some_and(|(_, containing_range)| {
                            !containing_range.contains_inclusive(&prefixed_contents_range)
                        })
                    {
                        containing_namespaces.pop();
                    }
                    let name_prefix = snapshot.text_for_range(prefix_range).collect::<Cow<str>>();
                    containing_namespaces.push((name_prefix.into(), prefixed_contents_range));
                } else {
                    import_statement_range.take();
                }
            }

            if let Some(range) = name_range {
                if import_statement_range
                    .as_ref()
                    .map_or(false, |import_statement_range| {
                        import_statement_range.contains_inclusive(&range)
                    })
                {
                    while containing_namespaces
                        .last()
                        .is_some_and(|(_, containing_range)| {
                            !containing_range.contains_inclusive(&range)
                        })
                    {
                        containing_namespaces.pop();
                    }
                    let import_name = snapshot.text_for_range(range).collect::<Cow<str>>();
                    let namespace = containing_namespaces
                        .iter()
                        .map(|(identifier, _)| identifier.clone())
                        .collect::<Vec<_>>();
                    let identifier = Identifier {
                        language_id: query_match.language.id(),
                        name: import_name.into(),
                    };
                    import_symbols
                        .entry(identifier)
                        .or_default()
                        .push(Namespace(namespace));
                } else {
                    import_statement_range.take();
                }
            }

            matches.advance();
        }

        let Some(import_range) = import_range else {
            return Err(anyhow!("No imports"));
        };

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
    use itertools::Itertools;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};

    #[gpui::test]
    fn test_collect_rust_imports(cx: &mut TestAppContext) {
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
        let language = Arc::new(rust_lang());
        let mut failures = Vec::new();
        for (source, expected) in examples {
            let buffer = cx.new(|cx| {
                let mut buffer = Buffer::local(source, cx);
                buffer.set_language(Some(language.clone()), cx);
                buffer
            });

            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());
            let imports = Imports::collect(&snapshot);
            let imports = imports.expect(&format!(
                "Failed to collect imports for source:\n{}",
                source
            ));
            let actual_symbols = imports
                .symbols
                .iter()
                .flat_map(|(identifier, namespaces)| {
                    namespaces
                        .iter()
                        .map(|namespace| to_rust_qualified_identifier(namespace, identifier))
                })
                .collect::<Vec<_>>();
            let expected_symbols = expected
                .iter()
                .map(|expected| expected.to_string())
                .collect::<Vec<_>>();
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
        expected_symbols: Vec<String>,
        actual_symbols: Vec<String>,
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
