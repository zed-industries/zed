use collections::HashMap;
use language::BufferSnapshot;
use language::ImportsConfig;
use language::Language;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use std::{borrow::Cow, ops::Range};
use text::OffsetRangeExt as _;
use util::RangeExt;
use util::paths::PathStyle;

use crate::Identifier;
use crate::text_similarity::Occurrences;

// TODO: Write documentation for extension authors. The @import capture must match before or in the
// same pattern as all all captures it contains

// Future improvements to consider:
//
// * Distinguish absolute vs relative paths in captures. `#include "maths.h"` is relative whereas
// `#include <maths.h>` is not.
//
// * Provide the name used when importing whole modules (see tests with "named_module" in the name).
// To be useful, will require parsing of identifier qualification.
//
// * Scoping for imports that aren't at the top level
//
// * Only scan a prefix of the file, when possible. This could look like having query matches that
// indicate it reached a declaration that is not allowed in the import section.
//
// * Support directly parsing to occurrences instead of storing namespaces / paths. Types should be
// generic on this, so that tests etc can still use strings. Could do similar in syntax index.
//
// * Distinguish different types of namespaces when known. E.g. "name.type" capture. Once capture
// names are more open-ended like this may make sense to build and cache a jump table (direct
// dispatch from capture index).
//
// * There are a few "Language specific:" comments on behavior that gets applied to all languages.
// Would be cleaner to be conditional on the language or otherwise configured.

#[derive(Debug, Clone, Default)]
pub struct Imports {
    pub identifier_to_imports: HashMap<Identifier, Vec<Import>>,
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
    SourceExact(Arc<Path>),
    SourceFuzzy(Arc<Path>),
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
        language: &Language,
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

                    let path = if let Some(strip_regex) =
                        language.config().import_path_strip_regex.as_ref()
                    {
                        strip_regex.replace_all(&path, "")
                    } else {
                        path
                    };

                    let path = Path::new(path.as_ref());
                    if (path.starts_with(".") || path.starts_with(".."))
                        && let Some(parent_abs_path) = parent_abs_path
                        && let Ok(abs_path) =
                            util::paths::normalize_lexically(&parent_abs_path.join(path))
                    {
                        *self = Self::SourceExact(abs_path.into());
                    } else {
                        *self = Self::SourceFuzzy(path.into());
                    };
                } else if matches!(self, Self::SourceExact(_))
                    || matches!(self, Self::SourceFuzzy(_))
                {
                    log::warn!("bug in imports query: encountered multiple @source matches");
                } else {
                    log::warn!(
                        "bug in imports query: encountered both @namespace and @source match"
                    );
                }
            }
            ModuleRange::Namespace(range) => {
                if let Self::Namespace(namespace) = self {
                    let segment = range_text(snapshot, range);
                    if language.config().ignored_import_segments.contains(&segment) {
                        return 0;
                    } else {
                        namespace.0.push(segment);
                        return 1;
                    }
                } else {
                    log::warn!(
                        "bug in imports query: encountered both @namespace and @source match"
                    );
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Namespace(pub Vec<Arc<str>>);

impl Namespace {
    pub fn occurrences(&self) -> Occurrences {
        Occurrences::from_identifiers(&self.0)
    }
}

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
        let mut import_range = None;

        while let Some(query_match) = matches.peek() {
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
            let mut modules = Vec::new();
            let mut content: Option<(Range<usize>, ContentKind)> = None;
            for capture in query_match.captures {
                let capture_range = capture.node.byte_range();

                if capture.index == *import_ix {
                    new_import_range = Some(capture_range);
                } else if Some(capture.index) == *namespace_ix {
                    modules.push(ModuleRange::Namespace(capture_range));
                } else if Some(capture.index) == *source_ix {
                    modules.push(ModuleRange::Source(capture_range));
                } else if Some(capture.index) == *alias_ix {
                    alias_range = Some(capture_range);
                } else {
                    let mut found_content = None;
                    if Some(capture.index) == *name_ix {
                        found_content = Some((capture_range, ContentKind::Name));
                    } else if Some(capture.index) == *list_ix {
                        found_content = Some((capture_range, ContentKind::List));
                    } else if Some(capture.index) == *wildcard_ix {
                        found_content = Some((capture_range, ContentKind::Wildcard));
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
                import_range = Some(new_import_range.clone());
            }

            if let Some((content, content_kind)) = content {
                if import_range
                    .as_ref()
                    .is_some_and(|import_range| import_range.contains_inclusive(&content))
                {
                    detached_nodes.push(DetachedNode {
                        modules,
                        content: content.clone(),
                        content_kind,
                        alias: alias_range.unwrap_or(0..0),
                        language: query_match.language.clone(),
                    });
                } else {
                    log::trace!(
                        "filtered out match not inside import range: {content_kind:?} at {content:?}"
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
            if !node.content.is_empty() && node.content == tree.content {
                // multiple matches can apply to the same name/list/wildcard. This keeps the queries
                // simpler by combining info from these matches.
                if tree.module.is_empty() {
                    tree.module = node.module;
                    tree.module_children = node.module_children;
                }
                if tree.alias.is_empty() {
                    tree.alias = node.alias;
                }
                return None;
            } else if !node.module.is_empty() && node.module.contains_inclusive(&tree.range()) {
                node.module_children.push(trees.remove(tree_index));
                continue;
            } else if !node.content.is_empty() && node.content.contains_inclusive(&tree.content) {
                node.content_children.push(trees.remove(tree_index));
                continue;
            } else if !tree.content.is_empty() && tree.content.contains_inclusive(&node.content) {
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
            pop_count +=
                current_module.push_range(&tree.module, snapshot, &tree.language, parent_abs_path);
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

        if tree.content_children.is_empty() && !tree.content.is_empty() {
            match tree.content_kind {
                ContentKind::Name | ContentKind::List => {
                    if tree.alias.is_empty() {
                        identifier_to_imports
                            .entry(Identifier {
                                language_id: tree.language.id(),
                                name: range_text(snapshot, &tree.content),
                            })
                            .or_default()
                            .push(Import::Direct {
                                module: current_module.clone(),
                            });
                    } else {
                        let alias_name: Arc<str> = range_text(snapshot, &tree.alias);
                        let external_name = range_text(snapshot, &tree.content);
                        // Language specific: skip "_" aliases for Rust
                        if alias_name.as_ref() != "_" {
                            identifier_to_imports
                                .entry(Identifier {
                                    language_id: tree.language.id(),
                                    name: alias_name,
                                })
                                .or_default()
                                .push(Import::Alias {
                                    module: current_module.clone(),
                                    external_identifier: Identifier {
                                        language_id: tree.language.id(),
                                        name: external_name,
                                    },
                                });
                        }
                    }
                }
                ContentKind::Wildcard => wildcard_modules.push(current_module.clone()),
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
                Module::SourceExact(_) | Module::SourceFuzzy(_) => {
                    log::warn!(
                        "bug in imports query: encountered both @namespace and @source match"
                    );
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
            pop_count += module.push_range(&tree.module, snapshot, &tree.language, parent_abs_path);
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
                &tree.language,
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
    modules: Vec<ModuleRange>,
    content: Range<usize>,
    content_kind: ContentKind,
    alias: Range<usize>,
    language: Arc<Language>,
}

#[derive(Debug, Clone, Copy)]
enum ContentKind {
    Name,
    Wildcard,
    List,
}

impl ContentKind {
    fn capture_name(&self) -> &'static str {
        match self {
            ContentKind::Name => "name",
            ContentKind::Wildcard => "wildcard",
            ContentKind::List => "list",
        }
    }
}

#[derive(Debug)]
struct ImportTree {
    module: ModuleRange,
    /// When non-empty, provides namespace / source info which should be used instead of `module`.
    module_children: Vec<ImportTree>,
    content: Range<usize>,
    /// When non-empty, provides content which should be used instead of `content`.
    content_children: Vec<ImportTree>,
    content_kind: ContentKind,
    alias: Range<usize>,
    language: Arc<Language>,
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

    fn from_module_range(module: &ModuleRange, language: Arc<Language>) -> Self {
        ImportTree {
            module: module.clone(),
            module_children: Vec::new(),
            content: 0..0,
            content_children: Vec::new(),
            content_kind: ContentKind::Name,
            alias: 0..0,
            language,
        }
    }
}

impl From<&DetachedNode> for ImportTree {
    fn from(value: &DetachedNode) -> Self {
        let module;
        let module_children;
        match value.modules.len() {
            0 => {
                module = ModuleRange::Namespace(0..0);
                module_children = Vec::new();
            }
            1 => {
                module = value.modules[0].clone();
                module_children = Vec::new();
            }
            _ => {
                module = ModuleRange::Namespace(
                    value.modules.first().unwrap().start..value.modules.last().unwrap().end,
                );
                module_children = value
                    .modules
                    .iter()
                    .map(|module| ImportTree::from_module_range(module, value.language.clone()))
                    .collect();
            }
        }

        ImportTree {
            module,
            module_children,
            content: value.content.clone(),
            content_children: Vec::new(),
            content_kind: value.content_kind,
            alias: value.alias.clone(),
            language: value.language.clone(),
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
            .field("content_kind", &self.tree.content_kind)
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
    use collections::HashSet;
    use gpui::{TestAppContext, prelude::*};
    use indoc::indoc;
    use language::{
        Buffer, Language, LanguageConfig, tree_sitter_python, tree_sitter_rust,
        tree_sitter_typescript,
    };
    use regex::Regex;

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
    fn test_rust_crate_and_super(cx: &mut TestAppContext) {
        check_imports(&RUST, "use crate::a::b::c;", &[&["a", "b", "c"]], cx);
        check_imports(&RUST, "use super::a::b::c;", &[&["a", "b", "c"]], cx);
        // TODO: Consider stripping leading "::". Not done for now because for the text similarity matching usecase this
        // is fine.
        check_imports(&RUST, "use ::a::b::c;", &[&["::a", "b", "c"]], cx);
    }

    #[gpui::test]
    fn test_typescript_imports(cx: &mut TestAppContext) {
        let parent_abs_path = PathBuf::from("/home/user/project");

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import "./maths.js";"#,
            &[&["SOURCE /home/user/project/maths", "WILDCARD"]],
            cx,
        );

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import "../maths.js";"#,
            &[&["SOURCE /home/user/maths", "WILDCARD"]],
            cx,
        );

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import RandomNumberGenerator, { pi as π } from "./maths.js";"#,
            &[
                &["SOURCE /home/user/project/maths", "RandomNumberGenerator"],
                &["SOURCE /home/user/project/maths", "pi AS π"],
            ],
            cx,
        );

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import { pi, phi, absolute } from "./maths.js";"#,
            &[
                &["SOURCE /home/user/project/maths", "pi"],
                &["SOURCE /home/user/project/maths", "phi"],
                &["SOURCE /home/user/project/maths", "absolute"],
            ],
            cx,
        );

        // index.js is removed by import_path_strip_regex
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import { pi, phi, absolute } from "./maths/index.js";"#,
            &[
                &["SOURCE /home/user/project/maths", "pi"],
                &["SOURCE /home/user/project/maths", "phi"],
                &["SOURCE /home/user/project/maths", "absolute"],
            ],
            cx,
        );

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import type { SomeThing } from "./some-module.js";"#,
            &[&["SOURCE /home/user/project/some-module", "SomeThing"]],
            cx,
        );

        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import { type SomeThing, OtherThing } from "./some-module.js";"#,
            &[
                &["SOURCE /home/user/project/some-module", "SomeThing"],
                &["SOURCE /home/user/project/some-module", "OtherThing"],
            ],
            cx,
        );

        // index.js is removed by import_path_strip_regex
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import { type SomeThing, OtherThing } from "./some-module/index.js";"#,
            &[
                &["SOURCE /home/user/project/some-module", "SomeThing"],
                &["SOURCE /home/user/project/some-module", "OtherThing"],
            ],
            cx,
        );

        // fuzzy paths
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import { type SomeThing, OtherThing } from "@my-app/some-module.js";"#,
            &[
                &["SOURCE FUZZY @my-app/some-module", "SomeThing"],
                &["SOURCE FUZZY @my-app/some-module", "OtherThing"],
            ],
            cx,
        );
    }

    #[gpui::test]
    fn test_typescript_named_module_imports(cx: &mut TestAppContext) {
        let parent_abs_path = PathBuf::from("/home/user/project");

        // TODO: These should provide the name that the module is bound to.
        // For now instead these are treated as unqualified wildcard imports.
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import * as math from "./maths.js";"#,
            // &[&["/home/user/project/maths.js", "WILDCARD AS math"]],
            &[&["SOURCE /home/user/project/maths", "WILDCARD"]],
            cx,
        );
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &TYPESCRIPT,
            r#"import math = require("./maths");"#,
            // &[&["/home/user/project/maths", "WILDCARD AS math"]],
            &[&["SOURCE /home/user/project/maths", "WILDCARD"]],
            cx,
        );
    }

    #[gpui::test]
    fn test_python_imports(cx: &mut TestAppContext) {
        check_imports(&PYTHON, "from math import pi", &[&["math", "pi"]], cx);

        check_imports(
            &PYTHON,
            "from math import pi, sin, cos",
            &[&["math", "pi"], &["math", "sin"], &["math", "cos"]],
            cx,
        );

        check_imports(&PYTHON, "from math import *", &[&["math", "WILDCARD"]], cx);

        check_imports(
            &PYTHON,
            "from math import foo.bar.baz",
            &[&["math", "foo", "bar", "baz"]],
            cx,
        );

        check_imports(
            &PYTHON,
            "from math import pi as PI",
            &[&["math", "pi AS PI"]],
            cx,
        );

        check_imports(
            &PYTHON,
            "from serializers.json import JsonSerializer",
            &[&["serializers", "json", "JsonSerializer"]],
            cx,
        );

        check_imports(
            &PYTHON,
            "from custom.serializers import json, xml, yaml",
            &[
                &["custom", "serializers", "json"],
                &["custom", "serializers", "xml"],
                &["custom", "serializers", "yaml"],
            ],
            cx,
        );
    }

    #[gpui::test]
    fn test_python_named_module_imports(cx: &mut TestAppContext) {
        // TODO: These should provide the name that the module is bound to.
        // For now instead these are treated as unqualified wildcard imports.
        //
        // check_imports(&PYTHON, "import math", &[&["math", "WILDCARD as math"]], cx);
        // check_imports(&PYTHON, "import math as maths", &[&["math", "WILDCARD AS maths"]], cx);
        //
        // Something like:
        //
        // (import_statement
        //     name: [
        //         (dotted_name
        //             (identifier)* @namespace
        //             (identifier) @name.module .)
        //         (aliased_import
        //             name: (dotted_name
        //                 ((identifier) ".")* @namespace
        //                 (identifier) @name.module .)
        //             alias: (identifier) @alias)
        //     ]) @import

        check_imports(&PYTHON, "import math", &[&["math", "WILDCARD"]], cx);

        check_imports(
            &PYTHON,
            "import math as maths",
            &[&["math", "WILDCARD"]],
            cx,
        );

        check_imports(&PYTHON, "import a.b.c", &[&["a", "b", "c", "WILDCARD"]], cx);

        check_imports(
            &PYTHON,
            "import a.b.c as d",
            &[&["a", "b", "c", "WILDCARD"]],
            cx,
        );
    }

    #[gpui::test]
    fn test_python_package_relative_imports(cx: &mut TestAppContext) {
        // TODO: These should provide info about the dir they are relative to, to provide more
        // precise resolution. Instead, fuzzy matching is used as usual.

        check_imports(&PYTHON, "from . import math", &[&["math"]], cx);

        check_imports(&PYTHON, "from .a import math", &[&["a", "math"]], cx);

        check_imports(
            &PYTHON,
            "from ..a.b import math",
            &[&["a", "b", "math"]],
            cx,
        );

        check_imports(
            &PYTHON,
            "from ..a.b import *",
            &[&["a", "b", "WILDCARD"]],
            cx,
        );
    }

    #[gpui::test]
    fn test_c_imports(cx: &mut TestAppContext) {
        let parent_abs_path = PathBuf::from("/home/user/project");

        // TODO: Distinguish that these are not relative to current path
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &C,
            r#"#include <math.h>"#,
            &[&["SOURCE FUZZY math.h", "WILDCARD"]],
            cx,
        );

        // TODO: These should be treated as relative, but don't start with ./ or ../
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &C,
            r#"#include "math.h""#,
            &[&["SOURCE FUZZY math.h", "WILDCARD"]],
            cx,
        );
    }

    #[gpui::test]
    fn test_cpp_imports(cx: &mut TestAppContext) {
        let parent_abs_path = PathBuf::from("/home/user/project");

        // TODO: Distinguish that these are not relative to current path
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &CPP,
            r#"#include <math.h>"#,
            &[&["SOURCE FUZZY math.h", "WILDCARD"]],
            cx,
        );

        // TODO: These should be treated as relative, but don't start with ./ or ../
        check_imports_with_file_abs_path(
            Some(&parent_abs_path),
            &CPP,
            r#"#include "math.h""#,
            &[&["SOURCE FUZZY math.h", "WILDCARD"]],
            cx,
        );
    }

    #[gpui::test]
    fn test_go_imports(cx: &mut TestAppContext) {
        check_imports(
            &GO,
            r#"import . "lib/math""#,
            &[&["lib/math", "WILDCARD"]],
            cx,
        );

        // not included, these are only for side-effects
        check_imports(&GO, r#"import _ "lib/math""#, &[], cx);
    }

    #[gpui::test]
    fn test_go_named_module_imports(cx: &mut TestAppContext) {
        // TODO: These should provide the name that the module is bound to.
        // For now instead these are treated as unqualified wildcard imports.

        check_imports(
            &GO,
            r#"import "lib/math""#,
            &[&["lib/math", "WILDCARD"]],
            cx,
        );
        check_imports(
            &GO,
            r#"import m "lib/math""#,
            &[&["lib/math", "WILDCARD"]],
            cx,
        );
    }

    #[track_caller]
    fn check_imports(
        language: &Arc<Language>,
        source: &str,
        expected: &[&[&str]],
        cx: &mut TestAppContext,
    ) {
        check_imports_with_file_abs_path(None, language, source, expected, cx);
    }

    #[track_caller]
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
        'outer: loop {
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
            while cursor.goto_parent() {
                depth -= 1;
                if cursor.goto_next_sibling() {
                    continue 'outer;
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
                    ignored_import_segments: HashSet::from_iter(["crate".into(), "super".into()]),
                    import_path_strip_regex: Some(Regex::new("/(lib|mod)\\.rs$").unwrap()),
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
                    import_path_strip_regex: Some(Regex::new("(?:/index)?\\.[jt]s$").unwrap()),
                    ..Default::default()
                },
                Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            )
            .with_imports_query(include_str!("../../languages/src/typescript/imports.scm"))
            .unwrap(),
        )
    });

    static PYTHON: LazyLock<Arc<Language>> = LazyLock::new(|| {
        Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Python".into(),
                    import_path_strip_regex: Some(Regex::new("/__init__\\.py$").unwrap()),
                    ..Default::default()
                },
                Some(tree_sitter_python::LANGUAGE.into()),
            )
            .with_imports_query(include_str!("../../languages/src/python/imports.scm"))
            .unwrap(),
        )
    });

    // TODO: Ideally should use actual language configurations
    static C: LazyLock<Arc<Language>> = LazyLock::new(|| {
        Arc::new(
            Language::new(
                LanguageConfig {
                    name: "C".into(),
                    import_path_strip_regex: Some(Regex::new("^<|>$").unwrap()),
                    ..Default::default()
                },
                Some(tree_sitter_c::LANGUAGE.into()),
            )
            .with_imports_query(include_str!("../../languages/src/c/imports.scm"))
            .unwrap(),
        )
    });

    static CPP: LazyLock<Arc<Language>> = LazyLock::new(|| {
        Arc::new(
            Language::new(
                LanguageConfig {
                    name: "C++".into(),
                    import_path_strip_regex: Some(Regex::new("^<|>$").unwrap()),
                    ..Default::default()
                },
                Some(tree_sitter_cpp::LANGUAGE.into()),
            )
            .with_imports_query(include_str!("../../languages/src/cpp/imports.scm"))
            .unwrap(),
        )
    });

    static GO: LazyLock<Arc<Language>> = LazyLock::new(|| {
        Arc::new(
            Language::new(
                LanguageConfig {
                    name: "Go".into(),
                    ..Default::default()
                },
                Some(tree_sitter_go::LANGUAGE.into()),
            )
            .with_imports_query(include_str!("../../languages/src/go/imports.scm"))
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
                Self::SourceExact(path) => {
                    vec![
                        format!("SOURCE {}", path.display().to_string().replace("\\", "/")),
                        identifier.to_string(),
                    ]
                }
                Self::SourceFuzzy(path) => {
                    vec![
                        format!(
                            "SOURCE FUZZY {}",
                            path.display().to_string().replace("\\", "/")
                        ),
                        identifier.to_string(),
                    ]
                }
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
