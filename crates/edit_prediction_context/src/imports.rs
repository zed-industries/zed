use collections::HashMap;
use language::BufferSnapshot;
use language::ImportsConfig;
use language::LanguageId;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use std::{borrow::Cow, ops::Range};
use text::OffsetRangeExt;
use util::RangeExt;
use util::paths::PathStyle;
use util::rel_path::RelPath;

use crate::Identifier;
use crate::text_similarity::Occurrences;

// TODO:
//
// * Distinguish different types of paths? (whether they are always relative)
//
// * Sort out how to get trace logs to automatically appear in test failures

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
//
// * Consider deferring path normalization

// Things to document (for extension authors)
//
// * the @import capture must match before all others it contains

#[derive(Debug, Clone)]
pub struct Imports {
    // TODO: this is not so meaningful when imports come from anywhere in the file
    pub all_imports_range: Option<Range<usize>>,
    pub identifier_to_imports: HashMap<Identifier, Vec<Import>>,
    // todo!
    pub wildcard_modules: Vec<Module>,
}

#[derive(Debug, Clone)]
pub enum Import {
    Direct {
        module: Module,
    },
    Alias {
        module: Module,
        external_identifier: Identifier,
    },
}

#[derive(Debug, Clone)]
pub enum Module {
    Source(Arc<Path>),
    Namespace(Namespace),
}

impl Module {
    fn empty() -> Self {
        Module::Namespace(Namespace::default())
    }

    fn push_range(
        &mut self,
        range: &ModuleRange,
        snapshot: &BufferSnapshot,
        parent_abs_path: Option<&Path>,
    ) -> usize {
        if range.is_empty() {
            return 0;
        }

        match range {
            ModuleRange::Source(range) => {
                if let Self::Namespace(namespace) = self
                    && namespace.0.is_empty()
                {
                    let path = snapshot.text_for_range(range.clone()).collect::<Cow<str>>();
                    let path = Path::new(path.as_ref());
                    if (path.starts_with(".") || path.starts_with(".."))
                        && let Some(parent_abs_path) = parent_abs_path
                        && let Ok(abs_path) =
                            util::paths::normalize_lexically(&parent_abs_path.join(path))
                    {
                        *self = Self::Source(abs_path.into());
                    } else {
                        *self = Self::Source(path.into());
                    };
                } else {
                    // todo: warn!
                }
            }
            ModuleRange::Namespace(range) => {
                if let Self::Namespace(namespace) = self {
                    namespace.0.push(range_text(snapshot, range));
                    return 1;
                } else {
                    // todo: warn!
                }
            }
        }
        0
    }
}

#[derive(Debug, Clone)]
enum ModuleRange {
    Source(Range<usize>),
    Namespace(Range<usize>),
}

impl Deref for ModuleRange {
    type Target = Range<usize>;

    fn deref(&self) -> &Self::Target {
        match self {
            ModuleRange::Source(range) => range,
            ModuleRange::Namespace(range) => range,
        }
    }
}

impl Module {
    // todo! rename
    pub fn into_occurrences(&self) -> Occurrences {
        // todo! compare paths directly
        match self {
            Module::Source(path) => Occurrences::from_worktree_path(
                // todo! figure out which worktree it belongs to
                None,
                &RelPath::new(&path, PathStyle::Posix).unwrap(),
            ),
            Module::Namespace(namespace) => Occurrences::from_identifiers(&namespace.0),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Namespace(pub Vec<Arc<str>>);

impl Imports {
    pub fn gather(snapshot: &BufferSnapshot, parent_abs_path: Option<&Path>) -> Self {
        // Query to match different import patterns
        let mut matches = snapshot
            .syntax
            .matches(0..snapshot.len(), &snapshot.text, |grammar| {
                grammar.imports_config().map(|imports| &imports.query)
            });

        let mut detached_nodes: Vec<DetachedNode> = Vec::new();
        let mut identifier_to_imports = HashMap::default();
        let mut wildcard_modules = Vec::new();
        let mut all_imports_range: Option<Range<usize>> = None;
        let mut import_range = None;

        while let Some(query_match) = matches.peek() {
            let language_id = query_match.language.id();
            let ImportsConfig {
                query: _,
                import_ix,
                name_ix,
                namespace_ix,
                source_ix,
                list_ix,
                wildcard_ix,
                alias_ix,
            } = matches.grammars()[query_match.grammar_index]
                .imports_config()
                .unwrap();

            let mut new_import_range = None;
            let mut alias_range = None;
            let mut module = None;
            let mut content: Option<(Range<usize>, NodeKind)> = None;
            for capture in query_match.captures {
                let capture_range = capture.node.byte_range();

                if capture.index == *import_ix {
                    new_import_range = Some(capture_range);
                } else if Some(capture.index) == *namespace_ix {
                    module = Some(ModuleRange::Namespace(capture_range));
                } else if Some(capture.index) == *source_ix {
                    // todo! check we didn't alreay have this and warn
                    module = Some(ModuleRange::Source(capture_range));
                } else if Some(capture.index) == *alias_ix {
                    alias_range = Some(capture_range);
                } else {
                    let mut found_content = None;
                    if Some(capture.index) == *name_ix {
                        found_content = Some((capture_range, NodeKind::Name));
                    } else if Some(capture.index) == *list_ix {
                        found_content = Some((capture_range, NodeKind::List));
                    } else if Some(capture.index) == *wildcard_ix {
                        found_content = Some((capture_range, NodeKind::Wildcard));
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

            if let Some(new_import_range) = new_import_range {
                log::trace!("starting new import {:?}", new_import_range);
                Self::gather_from_import_statement(
                    &detached_nodes,
                    &snapshot,
                    parent_abs_path,
                    &mut identifier_to_imports,
                    &mut wildcard_modules,
                );
                detached_nodes.clear();
                all_imports_range
                    .get_or_insert_with(|| new_import_range.clone())
                    .end = new_import_range.end;
                import_range = Some(new_import_range.clone());
            }

            if let Some((content, kind)) = content {
                if import_range
                    .as_ref()
                    .is_some_and(|import_range| import_range.contains_inclusive(&content))
                {
                    detached_nodes.push(DetachedNode {
                        // todo! have an empty module variant
                        module: module.unwrap_or(ModuleRange::Namespace(0..0)),
                        content: content.clone(),
                        alias: alias_range.unwrap_or(0..0),
                        language_id,
                        kind,
                    });
                } else {
                    log::trace!(
                        "filtered out match not inside import range: {kind:?} at {content:?}"
                    );
                }
            }

            matches.advance();
        }

        Self::gather_from_import_statement(
            &detached_nodes,
            &snapshot,
            parent_abs_path,
            &mut identifier_to_imports,
            &mut wildcard_modules,
        );

        Imports {
            all_imports_range,
            identifier_to_imports,
            wildcard_modules,
        }
    }

    fn gather_from_import_statement(
        detached_nodes: &[DetachedNode],
        snapshot: &BufferSnapshot,
        parent_abs_path: Option<&Path>,
        identifier_to_imports: &mut HashMap<Identifier, Vec<Import>>,
        wildcard_modules: &mut Vec<Module>,
    ) {
        let mut trees = Vec::new();

        for detached_node in detached_nodes {
            if let Some(node) = Self::attach_node(detached_node.into(), &mut trees) {
                trees.push(node);
            }
            log::trace!(
                "Attached node to tree\n{:#?}\nAttach result:\n{:#?}",
                detached_node,
                trees
                    .iter()
                    .map(|tree| tree.debug(snapshot))
                    .collect::<Vec<_>>()
            );
        }

        for tree in &trees {
            let mut module = Module::empty();
            Self::gather_from_tree(
                tree,
                snapshot,
                parent_abs_path,
                &mut module,
                identifier_to_imports,
                wildcard_modules,
            );
        }
    }

    fn attach_node(mut node: ImportTree, trees: &mut Vec<ImportTree>) -> Option<ImportTree> {
        let mut tree_index = 0;
        while tree_index < trees.len() {
            let tree = &mut trees[tree_index];
            if tree.content == node.content {
                // multiple matches can apply to the same name/list/wildcard. This keeps the queries
                // simpler by combining info from these matches.
                //
                // TODO: Log warnings when both have some information and there is a mismatch.
                if tree.module.is_empty() {
                    tree.module = node.module.clone();
                }
                if tree.alias.is_empty() {
                    tree.alias = node.alias.clone();
                }
                return None;
            } else if node.module.contains_inclusive(&tree.range()) {
                node.module_children.push(trees.remove(tree_index));
                continue;
            } else if node.content.contains_inclusive(&tree.content) {
                node.content_children.push(trees.remove(tree_index));
                continue;
            } else if tree.content.contains_inclusive(&node.content) {
                if let Some(node) = Self::attach_node(node, &mut tree.content_children) {
                    tree.content_children.push(node);
                }
                return None;
            }
            tree_index += 1;
        }
        Some(node)
    }

    fn gather_from_tree(
        tree: &ImportTree,
        snapshot: &BufferSnapshot,
        parent_abs_path: Option<&Path>,
        current_module: &mut Module,
        identifier_to_imports: &mut HashMap<Identifier, Vec<Import>>,
        wildcard_modules: &mut Vec<Module>,
    ) {
        let mut pop_count = 0;

        if tree.module_children.is_empty() {
            pop_count += current_module.push_range(&tree.module, snapshot, parent_abs_path);
        } else {
            for child in &tree.module_children {
                pop_count += Self::extend_namespace_from_tree(
                    child,
                    snapshot,
                    parent_abs_path,
                    current_module,
                );
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
                                module: current_module.clone(),
                            });
                    } else {
                        let alias_name: Arc<str> = range_text(snapshot, &tree.alias);
                        let external_name = range_text(snapshot, &tree.content);
                        // TODO: Make this special case be language-specific / configured?
                        if alias_name.as_ref() != "_" {
                            identifier_to_imports
                                .entry(Identifier {
                                    language_id: tree.language_id,
                                    name: alias_name,
                                })
                                .or_default()
                                .push(Import::Alias {
                                    module: current_module.clone(),
                                    external_identifier: Identifier {
                                        language_id: tree.language_id,
                                        name: external_name,
                                    },
                                });
                        }
                    }
                }
                NodeKind::Wildcard => wildcard_modules.push(current_module.clone()),
            }
        } else {
            for child in &tree.content_children {
                Self::gather_from_tree(
                    child,
                    snapshot,
                    parent_abs_path,
                    current_module,
                    identifier_to_imports,
                    wildcard_modules,
                );
            }
        }

        if pop_count > 0 {
            match current_module {
                Module::Source(_path) => {
                    // todo! warn
                }
                Module::Namespace(namespace) => {
                    namespace.0.drain(namespace.0.len() - pop_count..);
                }
            }
        }
    }

    fn extend_namespace_from_tree(
        tree: &ImportTree,
        snapshot: &BufferSnapshot,
        parent_abs_path: Option<&Path>,
        module: &mut Module,
    ) -> usize {
        let mut pop_count = 0;
        if tree.module_children.is_empty() {
            pop_count += module.push_range(&tree.module, snapshot, parent_abs_path);
        } else {
            for child in &tree.module_children {
                pop_count +=
                    Self::extend_namespace_from_tree(child, snapshot, parent_abs_path, module);
            }
        }
        if tree.content_children.is_empty() {
            pop_count += module.push_range(
                &ModuleRange::Namespace(tree.content.clone()),
                snapshot,
                parent_abs_path,
            );
        } else {
            for child in &tree.content_children {
                pop_count +=
                    Self::extend_namespace_from_tree(child, snapshot, parent_abs_path, module);
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
    module: ModuleRange,
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
    module: ModuleRange,
    module_children: Vec<ImportTree>,
    content: Range<usize>,
    content_children: Vec<ImportTree>,
    alias: Range<usize>,
    language_id: LanguageId,
    kind: NodeKind,
}

impl ImportTree {
    fn range(&self) -> Range<usize> {
        self.module.start.min(self.content.start)..self.module.end.max(self.content.end)
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
            module: value.module.clone(),
            module_children: Vec::new(),
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
            .field("module_range", &self.tree.module)
            .field("module_text", &range_text(self.snapshot, &self.tree.module))
            .field(
                "module_children",
                &self
                    .tree
                    .module_children
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
    use std::path::PathBuf;
    use std::sync::{Arc, LazyLock};

    use super::*;
    use gpui::{TestAppContext, prelude::*};
    use indoc::indoc;
    use language::{
        Buffer, Language, LanguageConfig, LanguageMatcher, tree_sitter_rust, tree_sitter_typescript,
    };

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

    #[gpui::test]
    fn test_typescript_imports(cx: &mut TestAppContext) {
        let parent_abs_path = PathBuf::from("/home/user/project");

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import RandomNumberGenerator, { pi as π } from "./maths.js";"#,
            &[
                &["/home/user/project/maths.js", "RandomNumberGenerator"],
                &["/home/user/project/maths.js", "pi AS π"],
            ],
            cx,
        );

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import { pi, phi, absolute } from "./maths.js";"#,
            &[
                &["/home/user/project/maths.js", "pi"],
                &["/home/user/project/maths.js", "phi"],
                &["/home/user/project/maths.js", "absolute"],
            ],
            cx,
        );

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import { pi, phi, absolute } from "./maths/index.js";"#,
            &[
                &["/home/user/project/maths/index.js", "pi"],
                &["/home/user/project/maths/index.js", "phi"],
                &["/home/user/project/maths/index.js", "absolute"],
            ],
            cx,
        );

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import "./maths.js";"#,
            &[&["/home/user/project/maths.js", "WILDCARD"]],
            cx,
        );

        // TODO: Consider supporting binding a module import to a name
        //
        // ``scm
        // (import_statement
        //     import_clause: (import_clause
        //         (namespace_import (identifier) @namespace_alias)
        //     source: (_) @namespace))
        // ```
        //
        // check_imports_with_file_abs_path(
        //     Some(&parent_abs_path),
        //     &TYPESCRIPT,
        //     r#"import * as math from "./maths.js";"#,
        //     &[&["/home/user/project/maths.js", "WILDCARD AS math"]],
        //     cx,
        // );
        //
        // ```scm
        // (import_statement
        //     import_clause: (import_require_clause
        //         (identifier) @namespace_alias
        //         source: (_) @namespace))
        // ```
        //
        // check_imports_with_file_abs_path(
        //     Some(&parent_abs_path),
        //     &TYPESCRIPT,
        //     r#"import math = require("./maths");"#,
        //     &[&["/home/user/project/maths", "WILDCARD AS math"]],
        //     cx,
        // );
    }

    fn check_imports(
        language: &Arc<Language>,
        source: &str,
        expected: &[&[&str]],
        cx: &mut TestAppContext,
    ) {
        check_imports_with_file_abs_path(None, language, source, expected, cx);
    }

    fn check_imports_with_file_abs_path(
        parent_abs_path: Option<&Path>,
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

        let imports = Imports::gather(&snapshot, parent_abs_path);
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
                    .wildcard_modules
                    .iter()
                    .map(|module| module.to_identifier_parts("WILDCARD")),
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

    static TYPESCRIPT: LazyLock<Arc<Language>> = LazyLock::new(|| {
        Arc::new(
            Language::new(
                LanguageConfig {
                    name: "TypeScript".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["ts".to_string()],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            )
            .with_imports_query(include_str!("../../languages/src/typescript/imports.scm"))
            .unwrap(),
        )
    });

    impl Import {
        fn to_identifier_parts(&self, identifier: &str) -> Vec<String> {
            match self {
                Import::Direct { module } => module.to_identifier_parts(identifier),
                Import::Alias {
                    module,
                    external_identifier: external_name,
                } => {
                    module.to_identifier_parts(&format!("{} AS {}", external_name.name, identifier))
                }
            }
        }
    }

    impl Module {
        fn to_identifier_parts(&self, identifier: &str) -> Vec<String> {
            match self {
                Self::Namespace(namespace) => namespace.to_identifier_parts(identifier),
                Self::Source(path) => vec![path.display().to_string(), identifier.to_string()],
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
