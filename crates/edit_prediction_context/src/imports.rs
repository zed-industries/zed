use anyhow::Result;
use anyhow::anyhow;
use collections::HashMap;
use language::BufferSnapshot;
use language::ImportsConfig;
use language::LanguageId;
use std::io::Result as IoResult;
use std::sync::Arc;
use std::{borrow::Cow, ops::Range};
use text::OffsetRangeExt;
use util::RangeExt;
use util::paths::PathStyle;

use crate::Identifier;

// Future improvements:
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
//
// * Support for importing namespaces (`self` in Rust, etc). Requires parsing of identifier
// qualification.

#[derive(Debug, Clone)]
pub struct Imports {
    // TODO: this is not so meaningful when imports come from anywhere in the file
    pub all_imports_range: Option<Range<usize>>,
    pub identifier_to_imports: HashMap<Identifier, Vec<Import>>,
    // todo!
    pub wildcard_namespaces: Vec<Namespace>,
}

#[derive(Debug, Clone)]
pub enum Import {
    Direct {
        namespace: Namespace,
    },
    Alias {
        namespace: Namespace,
        external_identifier: Identifier,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Namespace(pub Vec<Arc<str>>);

impl Imports {
    pub fn gather(snapshot: &BufferSnapshot) -> Self {
        // Query to match different import patterns
        let mut matches = snapshot
            .syntax
            .matches(0..snapshot.len(), &snapshot.text, |grammar| {
                grammar.imports_config().map(|imports| &imports.query)
            });

        let mut detached_nodes: Vec<DetachedNode> = Vec::new();
        let mut identifier_to_imports = HashMap::default();
        let mut wildcard_namespaces = Vec::new();
        let mut all_imports_range: Option<Range<usize>> = None;
        let mut import_range = None;

        while let Some(query_match) = matches.peek() {
            let language_id = query_match.language.id();
            let ImportsConfig {
                query: _,
                import_ix,
                name_ix,
                namespace_ix,
                file_ix,
                list_ix,
                wildcard_ix,
                alias_ix,
            } = matches.grammars()[query_match.grammar_index]
                .imports_config()
                .unwrap();

            let mut namespace_range = None;
            let mut content: Option<(Range<usize>, NodeKind)> = None;
            let mut alias_range = None;
            for capture in query_match.captures {
                let capture_range = capture.node.byte_range();

                if capture.index == *import_ix {
                    import_range = Some(capture_range.clone());
                    all_imports_range
                        .get_or_insert_with(|| capture_range.clone())
                        .end = capture_range.end;
                    Self::gather_from_import_statement(
                        &detached_nodes,
                        &snapshot,
                        &mut identifier_to_imports,
                        &mut wildcard_namespaces,
                    );
                    detached_nodes.clear();
                } else if import_range
                    .as_ref()
                    .is_some_and(|import_range| import_range.contains_inclusive(&capture_range))
                {
                    let mut found_content = None;
                    if Some(capture.index) == *name_ix {
                        found_content = Some((capture_range, NodeKind::Name));
                    } else if Some(capture.index) == *list_ix {
                        found_content = Some((capture_range, NodeKind::List));
                    } else if Some(capture.index) == *wildcard_ix {
                        found_content = Some((capture_range, NodeKind::Wildcard));
                    } else if Some(capture.index) == *namespace_ix {
                        namespace_range = Some(capture_range);
                    } else if Some(capture.index) == *alias_ix {
                        alias_range = Some(capture_range);
                    }
                    if let Some((found_content_range, found_kind)) = found_content {
                        if let Some((_, old_kind)) = content {
                            let point = found_content_range.to_point(snapshot);
                            log::warn!(
                                "bug in {} imports query: unexpected multiple captures of {} and {} ({}:{}:{})",
                                query_match.language.name(),
                                old_kind.capture_name(),
                                found_kind.capture_name(),
                                snapshot
                                    .file()
                                    .map(|p| p.path().display(PathStyle::Posix))
                                    .unwrap_or_default(),
                                point.start.row + 1,
                                point.start.column + 1
                            );
                        }
                        content = Some((found_content_range, found_kind));
                    }
                }
            }

            if let Some((content, kind)) = content {
                detached_nodes.push(DetachedNode {
                    namespace: namespace_range.unwrap_or(0..0),
                    content: content.clone(),
                    alias: alias_range.unwrap_or(0..0),
                    language_id,
                    kind,
                });
            }

            matches.advance();
        }

        Self::gather_from_import_statement(
            &detached_nodes,
            &snapshot,
            &mut identifier_to_imports,
            &mut wildcard_namespaces,
        );

        Imports {
            all_imports_range,
            identifier_to_imports,
            wildcard_namespaces,
        }
    }

    fn gather_from_import_statement(
        detached_nodes: &[DetachedNode],
        snapshot: &BufferSnapshot,
        identifier_to_imports: &mut HashMap<Identifier, Vec<Import>>,
        wildcard_namespaces: &mut Vec<Namespace>,
    ) {
        let mut trees = Vec::new();

        for detached_node in detached_nodes {
            if !Self::attach_node(&detached_node, &mut trees) {
                trees.push(detached_node.into());
            }
        }

        for tree in &trees {
            println!("{:#?}", tree.debug(snapshot));
            let mut namespace = Namespace::default();
            Self::gather_from_tree(
                tree,
                snapshot,
                &mut namespace,
                identifier_to_imports,
                wildcard_namespaces,
            );
        }
    }

    fn attach_node(detached_node: &DetachedNode, trees: &mut Vec<ImportTree>) -> bool {
        for tree in trees {
            if detached_node.namespace.contains_inclusive(&tree.range()) {
                let mut new_parent = detached_node.into();
                std::mem::swap(tree, &mut new_parent);
                let old_tree = new_parent;
                tree.namespace_children.push(old_tree);
                return true;
            } else if tree.content == detached_node.content {
                if tree.namespace.is_empty() {
                    tree.namespace = detached_node.namespace.clone();
                }
                if tree.alias.is_empty() {
                    tree.alias = detached_node.alias.clone();
                }
                return true;
            } else if tree.content.contains_inclusive(&detached_node.content) {
                if Self::attach_node(detached_node, &mut tree.content_children) {
                    return true;
                }
                tree.content_children.push(detached_node.into());
                return true;
            }
        }
        false
    }

    fn gather_from_tree(
        tree: &ImportTree,
        snapshot: &BufferSnapshot,
        namespace: &mut Namespace,
        identifier_to_imports: &mut HashMap<Identifier, Vec<Import>>,
        wildcard_namespaces: &mut Vec<Namespace>,
    ) {
        let mut pop_count = 0;

        if tree.namespace_children.is_empty() {
            if !tree.namespace.is_empty() {
                namespace.0.push(range_text(snapshot, &tree.namespace));
                pop_count += 1;
            }
        } else {
            for child in &tree.namespace_children {
                pop_count += Self::extend_namespace_from_tree(child, namespace, snapshot);
            }
        };

        if tree.content_children.is_empty() {
            match tree.kind {
                NodeKind::Name | NodeKind::List => {
                    if tree.alias.is_empty() {
                        identifier_to_imports
                            .entry(Identifier {
                                language_id: tree.language_id,
                                name: range_text(snapshot, &tree.content),
                            })
                            .or_default()
                            .push(Import::Direct {
                                namespace: namespace.clone(),
                            });
                    } else {
                        let alias_name: Arc<str> = range_text(snapshot, &tree.alias);
                        let external_name = range_text(snapshot, &tree.content);
                        // TODO: Make this special case be language-specific
                        if alias_name.as_ref() != "_" {
                            identifier_to_imports
                                .entry(Identifier {
                                    language_id: tree.language_id,
                                    name: alias_name,
                                })
                                .or_default()
                                .push(Import::Alias {
                                    namespace: namespace.clone(),
                                    external_identifier: Identifier {
                                        language_id: tree.language_id,
                                        name: external_name,
                                    },
                                });
                        }
                    }
                }
                NodeKind::Wildcard => wildcard_namespaces.push(namespace.clone()),
            }
        } else {
            for child in &tree.content_children {
                Self::gather_from_tree(
                    child,
                    snapshot,
                    namespace,
                    identifier_to_imports,
                    wildcard_namespaces,
                );
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
        if tree.namespace_children.is_empty() {
            if !tree.namespace.is_empty() {
                namespace.0.push(range_text(snapshot, &tree.namespace));
                pop_count += 1;
            }
        } else {
            for child in &tree.namespace_children {
                pop_count += Self::extend_namespace_from_tree(child, namespace, snapshot);
            }
        }
        if tree.content_children.is_empty() {
            namespace.0.push(range_text(snapshot, &tree.content));
            pop_count += 1;
        } else {
            for child in &tree.content_children {
                pop_count += Self::extend_namespace_from_tree(child, namespace, snapshot);
            }
        }
        pop_count
    }
}

fn range_text(snapshot: &BufferSnapshot, range: &Range<usize>) -> Arc<str> {
    snapshot
        .text_for_range(range.clone())
        .collect::<Cow<str>>()
        .into()
}

#[derive(Debug)]
struct DetachedNode {
    namespace: Range<usize>,
    content: Range<usize>,
    alias: Range<usize>,
    language_id: LanguageId,
    kind: NodeKind,
}

#[derive(Debug, Clone, Copy)]
enum NodeKind {
    Name,
    Wildcard,
    List,
}

impl NodeKind {
    fn capture_name(&self) -> &'static str {
        match self {
            NodeKind::Name => "name",
            NodeKind::Wildcard => "wildcard",
            NodeKind::List => "list",
        }
    }
}

#[derive(Debug)]
struct ImportTree {
    namespace: Range<usize>,
    namespace_children: Vec<ImportTree>,
    content: Range<usize>,
    content_children: Vec<ImportTree>,
    alias: Range<usize>,
    language_id: LanguageId,
    kind: NodeKind,
}

impl ImportTree {
    fn range(&self) -> Range<usize> {
        self.namespace.start..self.content.end
    }

    #[allow(dead_code)]
    fn debug<'a>(&'a self, snapshot: &'a BufferSnapshot) -> ImportTreeDebug<'a> {
        ImportTreeDebug {
            tree: self,
            snapshot,
        }
    }
}

impl From<&DetachedNode> for ImportTree {
    fn from(value: &DetachedNode) -> Self {
        ImportTree {
            namespace: value.namespace.clone(),
            namespace_children: Vec::new(),
            content: value.content.clone(),
            content_children: Vec::new(),
            alias: value.alias.clone(),
            language_id: value.language_id,
            kind: value.kind,
        }
    }
}

struct ImportTreeDebug<'a> {
    tree: &'a ImportTree,
    snapshot: &'a BufferSnapshot,
}

impl std::fmt::Debug for ImportTreeDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportTree")
            .field("namespace_range", &self.tree.namespace)
            .field(
                "namespace_text",
                &range_text(self.snapshot, &self.tree.namespace),
            )
            .field(
                "namespace_children",
                &self
                    .tree
                    .namespace_children
                    .iter()
                    .map(|child| child.debug(&self.snapshot))
                    .collect::<Vec<Self>>(),
            )
            .field("content_range", &self.tree.content)
            .field(
                "content_text",
                &range_text(self.snapshot, &self.tree.content),
            )
            .field(
                "content_children",
                &self
                    .tree
                    .content_children
                    .iter()
                    .map(|child| child.debug(&self.snapshot))
                    .collect::<Vec<Self>>(),
            )
            .field("alias_range", &self.tree.alias)
            .field("alias_text", &range_text(self.snapshot, &self.tree.alias))
            .finish()
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, LazyLock};

    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use indoc::indoc;
    use language::{Buffer, Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};

    #[gpui::test]
    fn test_rust_simple(cx: &mut TestAppContext) {
        check_imports(
            &RUST,
            "use std::collections::HashMap;",
            &[&["std", "collections", "HashMap"]],
            cx,
        );

        check_imports(
            &RUST,
            "pub use std::collections::HashMap;",
            &[&["std", "collections", "HashMap"]],
            cx,
        );

        check_imports(
            &RUST,
            "use std::collections::{HashMap, HashSet};",
            &[
                &["std", "collections", "HashMap"],
                &["std", "collections", "HashSet"],
            ],
            cx,
        );
    }

    #[gpui::test]
    fn test_rust_nested(cx: &mut TestAppContext) {
        check_imports(
            &RUST,
            "use std::{any::TypeId, collections::{HashMap, HashSet}};",
            &[
                &["std", "any", "TypeId"],
                &["std", "collections", "HashMap"],
                &["std", "collections", "HashSet"],
            ],
            cx,
        );

        check_imports(
            &RUST,
            "use a::b::c::{d::e::F, g::h::I};",
            &[
                &["a", "b", "c", "d", "e", "F"],
                &["a", "b", "c", "g", "h", "I"],
            ],
            cx,
        );
    }

    #[gpui::test]
    fn test_rust_multiple_imports(cx: &mut TestAppContext) {
        check_imports(
            &RUST,
            indoc! {"
                use std::collections::HashMap;
                use std::any::{TypeId, Any};
            "},
            &[
                &["std", "collections", "HashMap"],
                &["std", "any", "TypeId"],
                &["std", "any", "Any"],
            ],
            cx,
        );

        check_imports(
            &RUST,
            indoc! {"
                use std::collections::HashSet;

                fn main() {
                    let unqualified = HashSet::new();
                    let qualified = std::collections::HashMap::new();
                }

                use std::any::TypeId;
            "},
            &[
                &["std", "collections", "HashSet"],
                &["std", "any", "TypeId"],
            ],
            cx,
        );
    }

    #[gpui::test]
    fn test_rust_wildcard(cx: &mut TestAppContext) {
        check_imports(&RUST, "use prelude::*;", &[&["prelude", "WILDCARD"]], cx);

        check_imports(
            &RUST,
            "use zed::prelude::*;",
            &[&["zed", "prelude", "WILDCARD"]],
            cx,
        );

        check_imports(&RUST, "use prelude::{*};", &[&["prelude", "WILDCARD"]], cx);

        check_imports(
            &RUST,
            "use prelude::{File, *};",
            &[&["prelude", "File"], &["prelude", "WILDCARD"]],
            cx,
        );

        check_imports(
            &RUST,
            "use zed::{App, prelude::*};",
            &[&["zed", "App"], &["zed", "prelude", "WILDCARD"]],
            cx,
        );
    }

    #[gpui::test]
    fn test_rust_alias(cx: &mut TestAppContext) {
        check_imports(
            &RUST,
            "use std::io::Result as IoResult;",
            &[&["std", "io", "Result AS IoResult"]],
            cx,
        );
    }

    fn check_imports(
        language: &Arc<Language>,
        source: &str,
        expected: &[&[&str]],
        cx: &mut TestAppContext,
    ) {
        let buffer = cx.new(|cx| {
            let mut buffer = Buffer::local(source, cx);
            buffer.set_language(Some(language.clone()), cx);
            buffer
        });
        cx.run_until_parked();

        let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot());

        let imports = Imports::gather(&snapshot);
        let mut actual_symbols = imports
            .identifier_to_imports
            .iter()
            .flat_map(|(identifier, imports)| {
                imports
                    .iter()
                    .map(|import| import.to_identifier_parts(identifier.name.as_ref()))
            })
            .chain(
                imports
                    .wildcard_namespaces
                    .iter()
                    .map(|namespace| namespace.to_identifier_parts("WILDCARD")),
            )
            .collect::<Vec<_>>();
        let mut expected_symbols = expected
            .iter()
            .map(|expected| expected.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        actual_symbols.sort();
        expected_symbols.sort();
        if actual_symbols != expected_symbols {
            let top_layer = snapshot.syntax_layers().next().unwrap();
            panic!(
                "Expected imports: {:?}\n\
                Actual imports: {:?}\n\
                Tree:\n{}",
                expected_symbols,
                actual_symbols,
                tree_to_string(&top_layer.node()),
            );
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

    static RUST: LazyLock<Arc<Language>> = LazyLock::new(|| {
        Arc::new(
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
            .unwrap(),
        )
    });

    impl Import {
        fn to_identifier_parts(&self, identifier: &str) -> Vec<String> {
            match self {
                Import::Direct { namespace } => namespace.to_identifier_parts(identifier),
                Import::Alias {
                    namespace,
                    external_identifier: external_name,
                } => namespace
                    .to_identifier_parts(&format!("{} AS {}", external_name.name, identifier)),
            }
        }
    }

    impl Namespace {
        fn to_identifier_parts(&self, identifier: &str) -> Vec<String> {
            self.0
                .iter()
                .map(|chunk| chunk.to_string())
                .chain(std::iter::once(identifier.to_string()))
                .collect::<Vec<_>>()
        }
    }
}
