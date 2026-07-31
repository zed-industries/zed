use fuzzy_nucleo::{Case, LengthPenalty, StringMatchCandidate};
use gpui::BackgroundExecutor;
use language_core::{Grammar, SymbolKind};
use std::collections::HashSet;
use std::ops::Range;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tree_sitter::StreamingIterator;

/// Lightweight file location for an indexed symbol.
/// Deliberately does not depend on `project` or `worktree` crates.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolLocation {
    pub worktree_id: u64,
    pub path: Arc<str>,
}

/// A symbol extracted from tree-sitter parsing, before being added to the index.
#[derive(Clone, Debug)]
pub struct ExtractedSymbol {
    /// Symbol name only (e.g., "initBookForOfficial"), from @name capture.
    pub name: String,
    /// Full display text from source (e.g., "fn initBookForOfficial()"), for display.
    pub display_text: String,
    /// Byte range of the name within `display_text`.
    pub name_range: Range<u32>,
    /// Inferred from the @item node's tree-sitter type.
    pub kind: SymbolKind,
    /// Row (0-indexed) of the @name node's start position (where the cursor lands on jump).
    pub row: u32,
    /// Column (0-indexed) of the @name node's start position.
    pub column: u32,
}

/// A symbol stored in the index.
#[derive(Clone, Debug)]
pub struct IndexedSymbol {
    /// Symbol name only — used as the fuzzy match candidate string.
    pub name: Arc<str>,
    /// Full display text from source (e.g., "fn initBookForOfficial()").
    pub display_text: Arc<str>,
    /// Byte range of the name within `display_text`.
    pub name_range: Range<u32>,
    /// File location.
    pub location: SymbolLocation,
    /// Inferred symbol kind.
    pub kind: SymbolKind,
    /// Row (0-indexed) of the @name node.
    pub row: u32,
    /// Column (0-indexed) of the @name node.
    pub column: u32,
}

/// A search result that carries its own symbol snapshot, immune to concurrent index mutations.
#[derive(Clone, Debug)]
pub struct SymbolSearchResult {
    pub symbol: IndexedSymbol,
    pub score: f64,
    pub positions: Vec<usize>,
}

/// A consistent snapshot of the index used for concurrent search.
/// Self-contained — safe to search on a background thread while the index mutates.
#[derive(Clone)]
pub struct IndexSnapshot {
    symbols: Arc<[IndexedSymbol]>,
    candidates: Arc<[StringMatchCandidate]>,
}

impl IndexSnapshot {
    /// Search the snapshot with fuzzy matching.
    pub fn search(
        &self,
        query: &str,
        max_results: usize,
        cancel_flag: Arc<AtomicBool>,
        executor: BackgroundExecutor,
    ) -> impl std::future::Future<Output = Vec<SymbolSearchResult>> {
        let candidates = self.candidates.clone();
        let symbols = self.symbols.clone();
        let query = query.to_string();
        async move {
            if query.trim().is_empty() {
                return Vec::new();
            }
            let matches = fuzzy_nucleo::match_strings_async(
                &candidates,
                &query,
                Case::Smart,
                LengthPenalty::On,
                max_results,
                &cancel_flag,
                executor,
            )
            .await;

            matches
                .into_iter()
                .filter_map(|mat| {
                    symbols.get(mat.candidate_id).map(|symbol| {
                        SymbolSearchResult {
                            symbol: symbol.clone(),
                            score: mat.score,
                            positions: mat.positions,
                        }
                    })
                })
                .collect()
        }
    }
}

/// In-memory symbol index with client-side fuzzy search.
pub struct SymbolIndex {
    symbols: Vec<IndexedSymbol>,
    candidates: Vec<StringMatchCandidate>,
    snapshot: IndexSnapshot,
    snapshot_dirty: bool,
}

/// Extract symbols from source text using the grammar's outline query.
/// Returns an empty vec if the grammar has no outline config or parsing fails.
pub fn extract_symbols(text: &str, grammar: &Grammar) -> Vec<ExtractedSymbol> {
    let config = match grammar.outline_config.as_ref() {
        Some(config) => config,
        None => return Vec::new(),
    };

    let language = grammar.ts_language.clone();
    let mut parser = tree_sitter::Parser::new();
    if let Err(err) = parser.set_language(&language) {
        log::warn!("failed to set tree-sitter language: {err}");
        return Vec::new();
    }

    let tree = match parser.parse(text, None) {
        Some(tree) => tree,
        None => {
            log::warn!("tree-sitter parse returned None");
            return Vec::new();
        }
    };

    let source = text.as_bytes();
    let root_node = tree.root_node();
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(&config.query, root_node, source);

    let mut extracted_symbols = Vec::new();

    while let Some(query_match) = matches.next() {
        let mut item_node = None;
        let mut name_node = None;
        let mut name_parts = Vec::new();
        // Track the byte range spanning all relevant captures (context, name,
        // open, close) so we can extract display text from source in the
        // correct order, preserving original token positions.
        let mut first_capture_start: Option<usize> = None;
        let mut last_capture_end: Option<usize> = None;

        for capture in query_match.captures {
            let node = capture.node;
            let capture_index = capture.index;

            if capture_index == config.item_capture_ix {
                item_node = Some(node);
            } else if capture_index == config.name_capture_ix {
                if name_node.is_none() {
                    name_node = Some(node);
                }
                if let Ok(text) = node.utf8_text(source) {
                    name_parts.push(text.to_string());
                }
                if first_capture_start.map_or(true, |s| node.start_byte() < s) {
                    first_capture_start = Some(node.start_byte());
                }
                if last_capture_end.map_or(true, |e| node.end_byte() > e) {
                    last_capture_end = Some(node.end_byte());
                }
            } else if config.context_capture_ix == Some(capture_index)
                || config.extra_context_capture_ix == Some(capture_index)
                || config.open_capture_ix == Some(capture_index)
                || config.close_capture_ix == Some(capture_index)
            {
                if first_capture_start.map_or(true, |s| node.start_byte() < s) {
                    first_capture_start = Some(node.start_byte());
                }
                if last_capture_end.map_or(true, |e| node.end_byte() > e) {
                    last_capture_end = Some(node.end_byte());
                }
            }
        }

        let item_node = match item_node {
            Some(node) => node,
            None => continue,
        };

        let name_node = match name_node {
            Some(node) => node,
            None => continue,
        };

        let name = name_parts.join(" ");
        if name.is_empty() {
            continue;
        }

        // Build display_text directly from source, spanning from the first
        // relevant capture to the last. This preserves original token order
        // and spacing, including captures after @name (e.g., "()").
        let name_start_byte = name_node.start_byte();
        let name_end_byte = name_node.end_byte();
        let first_start = first_capture_start.unwrap_or(name_start_byte);
        let last_end = last_capture_end.unwrap_or(name_end_byte);

        let raw = &text[first_start..last_end];
        // Normalize whitespace (collapse runs, including newlines, into single spaces).
        let display_text: String = raw
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let name_range =
            (name_start_byte - first_start) as u32..(name_end_byte - first_start) as u32;

        let kind = infer_symbol_kind(item_node.kind());

        let pos = name_node.start_position();
        let (row, column) = (pos.row as u32, pos.column as u32);

        extracted_symbols.push(ExtractedSymbol {
            name,
            display_text,
            name_range,
            kind,
            row,
            column,
        });
    }

    extracted_symbols
}

fn infer_symbol_kind(node_type: &str) -> SymbolKind {
    let t = node_type.to_lowercase();
    if t.contains("function") || t.contains("method") || t.contains("macro") {
        SymbolKind::Function
    } else if t.contains("struct") {
        SymbolKind::Struct
    } else if t.contains("class") {
        SymbolKind::Class
    } else if t.contains("enum") {
        if t.contains("variant") || t.contains("member") {
            SymbolKind::EnumMember
        } else {
            SymbolKind::Enum
        }
    } else if t.contains("interface") || t.contains("trait") {
        SymbolKind::Interface
    } else if t.contains("impl") {
        SymbolKind::Class
    } else if t.contains("module") || t.contains("namespace") || t.contains("import") {
        SymbolKind::Module
    } else if t.contains("constructor") {
        SymbolKind::Constructor
    } else if t.contains("const") || t.contains("static") {
        SymbolKind::Constant
    } else if t.contains("field") {
        SymbolKind::Field
    } else if t.contains("property") {
        SymbolKind::Property
    } else if t.contains("type") {
        SymbolKind::TypeParameter
    } else if t.contains("var") {
        SymbolKind::Variable
    } else {
        SymbolKind::Null
    }
}

impl SymbolIndex {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
            candidates: Vec::new(),
            snapshot: IndexSnapshot {
                symbols: Arc::from(Vec::new()),
                candidates: Arc::from(Vec::new()),
            },
            snapshot_dirty: false,
        }
    }

    /// Set all symbols at once (for initial bulk index build).
    pub fn set_symbols(&mut self, symbols: Vec<IndexedSymbol>) {
        self.symbols = symbols;
        self.snapshot_dirty = true;
    }

    /// Add or replace symbols for a single file path.
    pub fn update_file(&mut self, location: SymbolLocation, extracted: Vec<ExtractedSymbol>) {
        self.update_files_batch([(location, extracted)]);
    }

    /// Add or replace symbols for multiple file paths in a single pass.
    /// More efficient than calling `update_file` repeatedly since candidates
    /// are rebuilt only once.
    pub fn update_files_batch(
        &mut self,
        updates: impl IntoIterator<Item = (SymbolLocation, Vec<ExtractedSymbol>)>,
    ) {
        let updates: Vec<_> = updates.into_iter().collect();
        let locations: HashSet<&SymbolLocation> = updates.iter().map(|(l, _)| l).collect();
        if !locations.is_empty() {
            self.symbols
                .retain(|symbol| !locations.contains(&symbol.location));
        }
        for (location, extracted) in updates {
            for symbol in extracted {
                self.symbols.push(IndexedSymbol {
                    name: Arc::from(symbol.name.as_str()),
                    display_text: Arc::from(symbol.display_text.as_str()),
                    name_range: symbol.name_range,
                    location: location.clone(),
                    kind: symbol.kind,
                    row: symbol.row,
                    column: symbol.column,
                });
            }
        }
        self.snapshot_dirty = true;
    }

    /// Remove all symbols for a file path.
    pub fn remove_file(&mut self, location: &SymbolLocation) {
        self.symbols.retain(|symbol| &symbol.location != location);
        self.snapshot_dirty = true;
    }

    /// Remove all symbols for multiple file paths in a single pass.
    pub fn remove_files_batch(&mut self, locations: &[SymbolLocation]) {
        if locations.is_empty() {
            return;
        }
        let location_set: HashSet<&SymbolLocation> = locations.iter().collect();
        self.symbols
            .retain(|symbol| !location_set.contains(&symbol.location));
        self.snapshot_dirty = true;
    }

    /// Remove all symbols belonging to a worktree.
    pub fn remove_worktree(&mut self, worktree_id: u64) {
        self.symbols
            .retain(|symbol| symbol.location.worktree_id != worktree_id);
        self.snapshot_dirty = true;
    }

    /// Take a snapshot of the index for concurrent search.
    /// The returned snapshot is self-contained and can be searched on a background
    /// thread without holding a borrow on the index.
    pub fn snapshot(&mut self) -> IndexSnapshot {
        if self.snapshot_dirty {
            self.rebuild_candidates();
            self.snapshot_dirty = false;
        }
        self.snapshot.clone()
    }

    /// Total number of indexed symbols.
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    fn rebuild_candidates(&mut self) {
        self.candidates = self
            .symbols
            .iter()
            .enumerate()
            .map(|(id, symbol)| StringMatchCandidate::new(id, symbol.name.clone()))
            .collect();

        self.snapshot = IndexSnapshot {
            symbols: Arc::from(&self.symbols[..]),
            candidates: Arc::from(&self.candidates[..]),
        };
    }
}

impl Default for SymbolIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::BackgroundExecutor;
    use language_core::LanguageName;

    const RUST_OUTLINE_QUERY: &str = r#"
(function_item
  (visibility_modifier)? @context
  (function_modifiers)? @context
  "fn" @context
  name: (_) @name
  body: (_
    .
    "{" @open
    "}" @close .)) @item

(struct_item
  name: (_) @name) @item

(enum_item
  name: (_) @name) @item

(trait_item
  name: (_) @name) @item

(impl_item
  type: (_) @name) @item

(type_item
  name: (_) @name) @item

(mod_item
  name: (_) @name) @item

(const_item
  name: (_) @name) @item

(static_item
  name: (_) @name) @item

(macro_definition
  name: (_) @name) @item
"#;

    fn rust_grammar() -> Grammar {
        Grammar::new(tree_sitter_rust::LANGUAGE.into())
            .with_outline_query(RUST_OUTLINE_QUERY, &LanguageName::new("Rust"))
            .unwrap()
    }

    #[test]
    fn test_extract_symbols_basic() {
        let source = r#"
fn initBookForOfficial() {}

struct Book { title: String }

enum Status { Active, Inactive }
"#;

        let grammar = rust_grammar();
        let symbols = extract_symbols(source, &grammar);

        assert_eq!(symbols.len(), 3, "expected function, struct, enum");

        let function_symbol = symbols
            .iter()
            .find(|symbol| symbol.name == "initBookForOfficial")
            .expect("function symbol should exist");
        assert_eq!(function_symbol.kind, SymbolKind::Function);
        assert!(function_symbol.display_text.contains("fn"));
        // Name position should point at the function name, not "fn"
        assert_eq!(function_symbol.row, 1);
        assert_eq!(function_symbol.column, 3);

        let struct_symbol = symbols
            .iter()
            .find(|symbol| symbol.name == "Book")
            .expect("struct symbol should exist");
        assert_eq!(struct_symbol.kind, SymbolKind::Struct);
        assert_eq!(struct_symbol.row, 3);
        assert_eq!(struct_symbol.column, 7);

        let enum_symbol = symbols
            .iter()
            .find(|symbol| symbol.name == "Status")
            .expect("enum symbol should exist");
        assert_eq!(enum_symbol.kind, SymbolKind::Enum);
        assert_eq!(enum_symbol.row, 5);
        assert_eq!(enum_symbol.column, 5);
    }

    #[gpui::test]
    async fn test_initialism_matching(executor: BackgroundExecutor) {
        let mut index = SymbolIndex::new();
        let location = SymbolLocation {
            worktree_id: 0,
            path: Arc::from("src/lib.rs"),
        };
        let symbol = ExtractedSymbol {
            name: "initBookForOfficial".to_string(),
            display_text: "fn initBookForOfficial".to_string(),
            name_range: 3..21,
            kind: SymbolKind::Function,
            row: 0,
            column: 0,
        };
        index.update_file(location, vec![symbol]);

        let cancel_flag = Arc::new(AtomicBool::new(false));

        let results = index.snapshot().search("ibfo", 10, cancel_flag.clone(), executor.clone()).await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].symbol.name.as_ref(), "initBookForOfficial");

        let results = index.snapshot().search("oib", 10, cancel_flag, executor).await;
        assert!(results.is_empty());
    }

    #[test]
    fn test_add_remove_file() {
        let mut index = SymbolIndex::new();
        let location = SymbolLocation {
            worktree_id: 1,
            path: Arc::from("src/main.rs"),
        };
        let symbol = ExtractedSymbol {
            name: "main".to_string(),
            display_text: "fn main".to_string(),
            name_range: 3..7,
            kind: SymbolKind::Function,
            row: 0,
            column: 0,
        };

        assert!(index.is_empty());

        index.update_file(location.clone(), vec![symbol.clone()]);
        assert_eq!(index.len(), 1);

        index.remove_file(&location);
        assert!(index.is_empty());

        index.update_file(location, vec![symbol]);
        assert_eq!(index.len(), 1);
    }

    #[gpui::test]
    async fn test_search_returns_sorted_results(executor: BackgroundExecutor) {
        let mut index = SymbolIndex::new();
        let location = SymbolLocation {
            worktree_id: 0,
            path: Arc::from("src/lib.rs"),
        };
        let symbols = vec![
            ExtractedSymbol {
            name: "alpha_function".to_string(),
            display_text: "fn alpha_function".to_string(),
            name_range: 3..17,
                kind: SymbolKind::Function,
                row: 0,
                column: 0,
            },
            ExtractedSymbol {
            name: "beta_function".to_string(),
            display_text: "fn beta_function".to_string(),
            name_range: 3..16,
                kind: SymbolKind::Function,
                row: 1,
                column: 0,
            },
            ExtractedSymbol {
            name: "gamma_struct".to_string(),
            display_text: "struct gamma_struct".to_string(),
            name_range: 7..19,
                kind: SymbolKind::Struct,
                row: 2,
                column: 0,
            },
        ];
        index.update_file(location, symbols);

        let cancel_flag = Arc::new(AtomicBool::new(false));
        let results = index.snapshot().search("function", 10, cancel_flag, executor).await;

        assert!(!results.is_empty());
        for window in results.windows(2) {
            assert!(
                window[0].score >= window[1].score,
                "results should be sorted by score descending"
            );
        }
    }

    #[gpui::test]
    async fn test_empty_query_returns_nothing(executor: BackgroundExecutor) {
        let mut index = SymbolIndex::new();
        let location = SymbolLocation {
            worktree_id: 0,
            path: Arc::from("src/lib.rs"),
        };
        let symbol = ExtractedSymbol {
            name: "something".to_string(),
            display_text: "fn something".to_string(),
            name_range: 3..12,
            kind: SymbolKind::Function,
            row: 0,
            column: 0,
        };
        index.update_file(location, vec![symbol]);

        let cancel_flag = Arc::new(AtomicBool::new(false));
        let results = index.snapshot().search("", 10, cancel_flag, executor).await;

        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_search_survives_concurrent_mutation(executor: BackgroundExecutor) {
        let mut index = SymbolIndex::new();
        let location_a = SymbolLocation {
            worktree_id: 0,
            path: Arc::from("src/a.rs"),
        };
        let location_b = SymbolLocation {
            worktree_id: 0,
            path: Arc::from("src/b.rs"),
        };

        index.update_file(
            location_a.clone(),
            vec![ExtractedSymbol {
            name: "alpha_func".to_string(),
            display_text: "fn alpha_func".to_string(),
            name_range: 3..13,
                kind: SymbolKind::Function,
                row: 0,
                column: 0,
            }],
        );

        let cancel_flag = Arc::new(AtomicBool::new(false));
        let snapshot = index.snapshot();
        let search_future = snapshot.search("alpha", 10, cancel_flag, executor.clone());

        // Mutate the index while the search is pending.
        index.update_file(
            location_b,
            vec![ExtractedSymbol {
            name: "beta_func".to_string(),
            display_text: "fn beta_func".to_string(),
            name_range: 3..12,
                kind: SymbolKind::Function,
                row: 0,
                column: 0,
            }],
        );

        let results = search_future.await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].symbol.name.as_ref(), "alpha_func");
    }
}
