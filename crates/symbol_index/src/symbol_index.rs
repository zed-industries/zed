use fuzzy_nucleo::{Case, LengthPenalty, StringMatchCandidate};
use gpui::{BackgroundExecutor, SharedString};
use language::{Grammar, LanguageName, SymbolKind, with_parser};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
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
    pub name: SharedString,
    /// Full display text from source (e.g., "fn initBookForOfficial()").
    pub display_text: SharedString,
    /// Byte range of the name within `display_text`.
    pub name_range: Range<u32>,
    /// File location.
    pub location: SymbolLocation,
    /// Language the symbol's file was indexed as. Used at query time to route
    /// between the tree-sitter index and language servers.
    pub language: LanguageName,
    /// Inferred symbol kind.
    pub kind: SymbolKind,
    /// Row (0-indexed) of the @name node.
    pub row: u32,
    /// Column (0-indexed) of the @name node.
    pub column: u32,
}

/// A symbol search result.
#[derive(Clone, Debug)]
pub struct SymbolSearchResult {
    pub symbol: IndexedSymbol,
    /// Fuzzy match score.
    pub score: f64,
    /// Fuzzy match character positions.
    pub positions: Vec<usize>,
}

/// A searchable snapshot of the symbol index.
#[derive(Clone)]
pub struct IndexSnapshot {
    symbols: Arc<[IndexedSymbol]>,
    candidates: Arc<[StringMatchCandidate]>,
    /// Languages that have symbols in this snapshot.
    languages: Arc<HashSet<LanguageName>>,
}

impl IndexSnapshot {
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Languages that have symbols in this snapshot. Used by callers to
    /// decide which languages to check for tree-sitter eligibility.
    pub fn languages(&self) -> &HashSet<LanguageName> {
        &self.languages
    }

    /// Search the snapshot for symbols matching `query`, returning at most
    /// `max_results` symbols.
    ///
    /// When `eligible_languages` is given, it maps a worktree id to the
    /// languages whose symbols the caller is going to keep; symbols of other
    /// languages are filtered out before matching, so that `max_results` is
    /// only spent on symbols the caller keeps.
    pub fn search(
        &self,
        query: &str,
        max_results: usize,
        cancel_flag: Arc<AtomicBool>,
        executor: BackgroundExecutor,
        eligible_languages: Option<Arc<HashMap<u64, HashSet<LanguageName>>>>,
    ) -> impl Future<Output = Vec<SymbolSearchResult>> + use<> {
        let symbols = self.symbols.clone();
        let candidates = self.candidates.clone();
        let query = query.to_string();
        async move {
            if query.trim().is_empty() {
                return Vec::new();
            }

            let filtered_candidates = eligible_languages.as_ref().map(|eligible_languages| {
                symbols
                    .iter()
                    .zip(candidates.iter())
                    .filter(|(symbol, _)| {
                        eligible_languages
                            .get(&symbol.location.worktree_id)
                            .is_some_and(|languages| languages.contains(&symbol.language))
                    })
                    .map(|(_, candidate)| candidate.clone())
                    .collect::<Vec<_>>()
            });
            // Filtered candidates keep their original ids, so matches can be
            // mapped back to `symbols` below.
            let search_candidates: &[StringMatchCandidate] = filtered_candidates
                .as_deref()
                .unwrap_or(candidates.as_ref());

            let matches = fuzzy_nucleo::match_strings_async(
                &search_candidates,
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
                    symbols
                        .get(mat.candidate_id)
                        .map(|symbol| SymbolSearchResult {
                            symbol: symbol.clone(),
                            score: mat.score,
                            positions: mat.positions,
                        })
                })
                .collect()
        }
    }
}

/// A fuzzy-matchable index of symbols in project files.
#[derive(Default)]
pub struct SymbolIndex {
    /// All currently indexed symbols.
    symbols: Vec<IndexedSymbol>,
    /// Languages that have symbols in the index.
    languages: HashSet<LanguageName>,
    /// Lazily built search snapshot, invalidated by every mutation. Building
    /// it eagerly on every batch would make the main thread pay an O(total)
    /// candidate rebuild per flushed batch during the initial scan.
    snapshot: Option<IndexSnapshot>,
}

impl SymbolIndex {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    /// Languages that have symbols in the index. Overapproximates after
    /// files are removed.
    pub fn languages(&self) -> &HashSet<LanguageName> {
        &self.languages
    }

    /// Replace the symbols for the given files in a single batch.
    /// An empty `extracted` removes the file's symbols.
    pub fn update_files_batch(
        &mut self,
        updates: impl IntoIterator<Item = (SymbolLocation, LanguageName, Vec<ExtractedSymbol>)>,
    ) {
        let updates: Vec<_> = updates.into_iter().collect();
        let locations: HashSet<&SymbolLocation> = updates.iter().map(|(loc, _, _)| loc).collect();
        self.symbols
            .retain(|symbol| !locations.contains(&symbol.location));
        for (location, language, extracted) in updates {
            if !extracted.is_empty() {
                self.languages.insert(language.clone());
            }
            for symbol in extracted {
                self.symbols.push(IndexedSymbol {
                    name: symbol.name.into(),
                    display_text: symbol.display_text.into(),
                    name_range: symbol.name_range,
                    location: location.clone(),
                    language: language.clone(),
                    kind: symbol.kind,
                    row: symbol.row,
                    column: symbol.column,
                });
            }
        }
        self.snapshot = None;
    }

    /// Replace the symbols for a single file.
    pub fn update_file(
        &mut self,
        location: &SymbolLocation,
        language: &LanguageName,
        extracted: Vec<ExtractedSymbol>,
    ) {
        self.update_files_batch(std::iter::once((
            location.clone(),
            language.clone(),
            extracted,
        )));
    }

    /// Remove symbols for a batch of files.
    pub fn remove_files_batch(&mut self, locations: impl IntoIterator<Item = SymbolLocation>) {
        let locations: HashSet<SymbolLocation> = locations.into_iter().collect();
        self.symbols
            .retain(|symbol| !locations.contains(&symbol.location));
        self.snapshot = None;
    }

    /// Remove all symbols for a worktree.
    pub fn remove_worktree(&mut self, worktree_id: u64) {
        self.symbols
            .retain(|symbol| symbol.location.worktree_id != worktree_id);
        self.snapshot = None;
    }

    /// Remove a single file's symbols.
    pub fn remove_file(&mut self, location: &SymbolLocation) {
        self.symbols.retain(|symbol| symbol.location != *location);
        self.snapshot = None;
    }

    /// Build a searchable snapshot of the index, reusing a previously built
    /// one as long as the index has not changed since.
    pub fn snapshot(&mut self) -> IndexSnapshot {
        if let Some(snapshot) = &self.snapshot {
            return snapshot.clone();
        }
        let snapshot = IndexSnapshot {
            symbols: self.symbols.iter().cloned().collect(),
            candidates: self
                .symbols
                .iter()
                .enumerate()
                .map(|(symbol_id, symbol)| {
                    StringMatchCandidate::new(symbol_id, symbol.name.clone())
                })
                .collect(),
            languages: Arc::new(self.languages.clone()),
        };
        self.snapshot = Some(snapshot.clone());
        snapshot
    }
}

/// Extract symbols from `text` using the given language's tree-sitter
/// grammar and outline query.
///
/// Uses the shared parser pool from the `language` crate, whose parsers are
/// configured with a WASM store, so languages loaded from extensions work
/// too.
///
/// Line endings are normalized to `\n` before parsing, matching how editor
/// buffers normalize file contents on load, so that reported rows and columns
/// line up with the buffer the symbols are opened in.
pub fn extract_symbols(text: &str, grammar: &Grammar) -> Vec<ExtractedSymbol> {
    let text = text::LineEnding::normalize_cow(std::borrow::Cow::Borrowed(text));
    let text = text.as_ref();
    let Some(config) = grammar.outline_config.as_ref() else {
        return Vec::new();
    };

    let tree = with_parser(|parser| {
        if let Err(err) = parser.set_language(&grammar.ts_language) {
            log::warn!("symbol_index: failed to set tree-sitter language: {err}");
            None
        } else {
            parser.parse(text, None)
        }
    });
    let Some(tree) = tree else {
        return Vec::new();
    };

    let source = text.as_bytes();
    let root_node = tree.root_node();
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut matches = cursor.matches(&config.query, root_node, source);

    let mut symbols = Vec::new();

    while let Some(query_match) = matches.next() {
        let mut item_node = None;
        let mut name_node = None;
        // (byte range, is_name) of each @name/@context capture. The display
        // text concatenates these in source order, inserting a space only
        // where the captures have a gap between them in the source.
        let mut capture_ranges: Vec<(Range<usize>, bool)> = Vec::new();

        let mut add_capture_range = |node: tree_sitter::Node<'_>, is_name: bool| {
            let mut range = node.start_byte()..node.end_byte();
            let start = node.start_position();
            if node.end_position().row > start.row {
                // Same clipping as `BufferSnapshot::next_outline_item`:
                // truncate the capture to its first line. Search forward from
                // the capture's start for the line's end, so the cost is
                // proportional to the line's length rather than the file's.
                // The line length includes a trailing `\r`, which must not
                // become part of the symbol.
                let line_end = source[range.start..]
                    .iter()
                    .position(|&byte| byte == b'\n')
                    .map_or(source.len(), |position| range.start + position);
                range.end = range.end.min(line_end);
                if range.end > range.start && source[range.end - 1] == b'\r' {
                    range.end -= 1;
                }
            }
            if !range.is_empty() {
                capture_ranges.push((range, is_name));
            }
        };

        for capture in query_match.captures {
            let node = capture.node;
            let capture_index = capture.index;

            if capture_index == config.item_capture_ix {
                item_node = Some(node);
            } else if capture_index == config.name_capture_ix {
                if name_node.is_none() {
                    name_node = Some(node);
                }
                add_capture_range(node, true);
            } else if config.context_capture_ix == Some(capture_index)
                || config.extra_context_capture_ix == Some(capture_index)
            {
                add_capture_range(node, false);
            }
            // @open/@close captures are not part of the display text, like
            // in the outline panel.
        }

        let Some(item_node) = item_node else { continue };
        let Some(name_node) = name_node else { continue };

        // The symbol name concatenates all @name captures directly.
        let name: String = capture_ranges
            .iter()
            .filter(|(_, is_name)| *is_name)
            .map(|(range, _)| &text[range.clone()])
            .collect();
        if name.is_empty() {
            continue;
        }

        let mut display_text = String::new();
        let mut name_ranges: Vec<Range<usize>> = Vec::new();
        let mut last_end: Option<usize> = None;
        for (range, is_name) in &capture_ranges {
            let has_gap = last_end.is_some_and(|prev| range.start > prev);
            if !display_text.is_empty() && has_gap {
                display_text.push(' ');
            }
            let start = display_text.len();
            display_text.push_str(&text[range.clone()]);
            if *is_name {
                name_ranges.push(start..display_text.len());
            }
            last_end = Some(range.end);
        }
        let name_range = match (name_ranges.first(), name_ranges.last()) {
            (Some(first), Some(last)) => first.start as u32..last.end as u32,
            _ => 0..0,
        };

        let position = name_node.start_position();
        symbols.push(ExtractedSymbol {
            name,
            display_text,
            name_range,
            kind: infer_symbol_kind(item_node.kind()),
            row: position.row as u32,
            column: position.column as u32,
        });
    }

    symbols
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

#[cfg(test)]
mod tests {
    use super::*;

    fn rust_grammar() -> Arc<Grammar> {
        language::rust_lang().grammar().unwrap().clone()
    }

    fn test_location() -> SymbolLocation {
        SymbolLocation {
            worktree_id: 1,
            path: "test.rs".into(),
        }
    }

    #[test]
    fn test_extract_symbols_basic() {
        let grammar = rust_grammar();
        let source = r#"
fn initBookForOfficial() {}

struct Book {
    title: String,
}

enum Status {
    Active,
    Inactive,
}
"#;
        let symbols = extract_symbols(source, &grammar);
        let find = |name: &str| symbols.iter().find(|s| s.name == name);

        let function_symbol = find("initBookForOfficial").unwrap();
        assert_eq!(function_symbol.kind, SymbolKind::Function);
        assert_eq!(function_symbol.row, 1);
        assert_eq!(function_symbol.column, 3);
        assert!(function_symbol.display_text.contains("fn"));

        let struct_symbol = find("Book").unwrap();
        assert_eq!(struct_symbol.kind, SymbolKind::Struct);
        assert_eq!(struct_symbol.row, 3);
        assert_eq!(struct_symbol.column, 7);

        let enum_symbol = find("Status").unwrap();
        assert_eq!(enum_symbol.kind, SymbolKind::Enum);
        assert_eq!(enum_symbol.row, 7);
        assert_eq!(enum_symbol.column, 5);

        // The real Rust outline query also captures enum variants and struct
        // fields.
        assert_eq!(find("Active").unwrap().kind, SymbolKind::EnumMember);
        assert_eq!(find("title").unwrap().kind, SymbolKind::Field);
    }

    #[test]
    fn test_extract_symbols_skips_no_name() {
        // The real Rust outline query only produces @item nodes together with
        // @name captures, so this can only be exercised indirectly: a source
        // with no matchable symbols yields no symbols.
        let grammar = rust_grammar();
        let source = "// just a comment\n";
        assert!(extract_symbols(source, &grammar).is_empty());
    }

    #[test]
    fn test_extract_symbols_multiline_name_truncated() {
        let grammar = rust_grammar();
        // `impl` type nodes can span lines; the name capture must be truncated
        // to its first line, like the outline panel does.
        let source = "impl Foo\nfor Bar {\n    fn baz() {}\n}\n";
        let symbols = extract_symbols(source, &grammar);
        let impl_symbol = symbols
            .iter()
            .find(|s| s.display_text.starts_with("impl"))
            .unwrap();
        assert_eq!(impl_symbol.row, 0);
        assert!(impl_symbol.display_text.contains("impl"));
        // No newline may leak into the display text.
        assert!(!impl_symbol.display_text.contains('\n'));
    }

    #[test]
    fn test_extract_symbols_multiline_crlf() {
        let grammar = rust_grammar();
        // A multiline name in a CRLF file must not keep a trailing `\r`.
        let source = "impl Foo\r\nfor Bar {\r\n    fn baz() {}\r\n}\r\n";
        let symbols = extract_symbols(source, &grammar);
        let impl_symbol = symbols
            .iter()
            .find(|s| s.display_text.starts_with("impl"))
            .unwrap();
        assert!(!impl_symbol.display_text.contains('\r'));
        assert!(!impl_symbol.display_text.contains('\n'));
    }

    #[test]
    fn test_extract_symbols_normalizes_line_endings() {
        let grammar = rust_grammar();
        // Buffers normalize lone `\r` and `\r\n` on load, so indexed
        // coordinates must be reported against the normalized text.
        let source = "fn first() {}\rfn target() {}\r";
        let symbols = extract_symbols(source, &grammar);
        let target = symbols.iter().find(|s| s.name == "target").unwrap();
        assert_eq!(target.row, 1);
        assert_eq!(target.column, 3);

        let source = "fn first() {}\r\nfn target() {}\r\n";
        let symbols = extract_symbols(source, &grammar);
        let target = symbols.iter().find(|s| s.name == "target").unwrap();
        assert_eq!(target.row, 1);
        assert_eq!(target.column, 3);
    }

    #[gpui::test]
    async fn test_initialism_matching(cx: &mut gpui::TestAppContext) {
        let mut index = SymbolIndex::default();
        let language = LanguageName::new("Rust");
        index.update_file(
            &test_location(),
            &language,
            vec![ExtractedSymbol {
                name: "initBookForOfficial".to_string(),
                display_text: "fn initBookForOfficial()".to_string(),
                name_range: 3..21,
                kind: SymbolKind::Function,
                row: 0,
                column: 3,
            }],
        );
        index.update_file(
            &SymbolLocation {
                worktree_id: 1,
                path: "other.rs".into(),
            },
            &language,
            vec![ExtractedSymbol {
                name: "some_other_fn".to_string(),
                display_text: "fn some_other_fn()".to_string(),
                name_range: 3..16,
                kind: SymbolKind::Function,
                row: 1,
                column: 3,
            }],
        );
        let snapshot = index.snapshot();

        let executor = cx.executor();
        let results = snapshot
            .search("bfo", 10, Arc::new(AtomicBool::new(false)), executor, None)
            .await;
        assert_eq!(results.len(), 1);
        assert_eq!(&*results[0].symbol.name, "initBookForOfficial");
    }

    #[test]
    fn test_add_remove_file_symbols() {
        let mut index = SymbolIndex::default();
        let language = LanguageName::new("Rust");
        let location = test_location();

        index.update_file(
            &location,
            &language,
            vec![
                ExtractedSymbol {
                    name: "alpha_func".to_string(),
                    display_text: "fn alpha_func()".to_string(),
                    name_range: 3..13,
                    kind: SymbolKind::Function,
                    row: 0,
                    column: 3,
                },
                ExtractedSymbol {
                    name: "beta_func".to_string(),
                    display_text: "fn beta_func()".to_string(),
                    name_range: 3..12,
                    kind: SymbolKind::Function,
                    row: 1,
                    column: 3,
                },
            ],
        );
        assert_eq!(index.len(), 2);

        // Update with new symbols replaces the old ones.
        index.update_file(
            &location,
            &language,
            vec![ExtractedSymbol {
                name: "gamma_func".to_string(),
                display_text: "fn gamma_func()".to_string(),
                name_range: 3..13,
                kind: SymbolKind::Function,
                row: 2,
                column: 3,
            }],
        );
        assert_eq!(index.len(), 1);

        // Removing the file removes its symbols.
        index.remove_file(&location);
        assert_eq!(index.len(), 0);
    }

    #[gpui::test]
    async fn test_search_empty_query(cx: &mut gpui::TestAppContext) {
        let mut index = SymbolIndex::default();
        let language = LanguageName::new("Rust");
        index.update_file(
            &test_location(),
            &language,
            vec![ExtractedSymbol {
                name: "alpha_func".to_string(),
                display_text: "fn alpha_func()".to_string(),
                name_range: 3..13,
                kind: SymbolKind::Function,
                row: 0,
                column: 3,
            }],
        );
        let snapshot = index.snapshot();
        let executor = cx.executor();
        let results = snapshot
            .search("", 10, Arc::new(AtomicBool::new(false)), executor, None)
            .await;
        assert!(results.is_empty());
    }

    #[gpui::test]
    async fn test_search_language_filter(cx: &mut gpui::TestAppContext) {
        let mut index = SymbolIndex::default();
        let rust = LanguageName::new("Rust");
        let python = LanguageName::new("Python");
        let location = test_location();
        index.update_file(
            &location,
            &rust,
            vec![symbol("alpha_func", SymbolKind::Function)],
        );
        index.update_file(
            &SymbolLocation {
                worktree_id: 1,
                path: "script.py".into(),
            },
            &python,
            vec![symbol("alpha_snake", SymbolKind::Function)],
        );
        let snapshot = index.snapshot();
        assert_eq!(snapshot.len(), 2);

        let executor = cx.executor();

        // Symbols of an eligible (worktree, language) pair are returned.
        let eligible = Arc::new(HashMap::from([(1u64, HashSet::from([python.clone()]))]));
        let results = snapshot
            .search(
                "alpha",
                10,
                Arc::new(AtomicBool::new(false)),
                executor.clone(),
                Some(eligible),
            )
            .await;
        assert_eq!(results.len(), 1);
        assert_eq!(&*results[0].symbol.name, "alpha_snake");

        // A language that is eligible in one worktree does not make symbols
        // of another worktree eligible.
        let eligible = Arc::new(HashMap::from([(7u64, HashSet::from([rust.clone()]))]));
        let results = snapshot
            .search(
                "alpha",
                10,
                Arc::new(AtomicBool::new(false)),
                executor.clone(),
                Some(eligible),
            )
            .await;
        assert!(results.is_empty());

        // The limit only applies to eligible symbols: one match fills the
        // limit, and no symbol of an ineligible language is returned in its
        // place.
        let eligible = Arc::new(HashMap::from([(
            1u64,
            HashSet::from([rust.clone(), python.clone()]),
        )]));
        let results = snapshot
            .search(
                "alpha",
                1,
                Arc::new(AtomicBool::new(false)),
                executor,
                Some(eligible),
            )
            .await;
        assert_eq!(results.len(), 1);
    }

    fn symbol(name: &str, kind: SymbolKind) -> ExtractedSymbol {
        ExtractedSymbol {
            name: name.to_string(),
            display_text: name.to_string(),
            name_range: 0..name.len() as u32,
            kind,
            row: 0,
            column: 0,
        }
    }

    #[test]
    fn test_remove_worktree() {
        let mut index = SymbolIndex::default();
        let language = LanguageName::new("Rust");
        index.update_file(
            &SymbolLocation {
                worktree_id: 1,
                path: "a.rs".into(),
            },
            &language,
            vec![symbol("a_symbol", SymbolKind::Function)],
        );
        index.update_file(
            &SymbolLocation {
                worktree_id: 2,
                path: "b.rs".into(),
            },
            &language,
            vec![symbol("b_symbol", SymbolKind::Function)],
        );
        assert_eq!(index.len(), 2);

        index.remove_worktree(1);
        assert_eq!(index.len(), 1);
        assert!(index.languages().contains(&language));
    }
}
