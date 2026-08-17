use std::{cell::OnceCell, collections::HashMap, ops::Range};

use anyhow::Result;
use gpui::SharedString;
use serde::{Deserialize, Serialize};
use text::{BufferSnapshot, Point};

/// Number of lines above and below the source line hashed into
/// [`ContentMarker::context_hash`].
const CONTEXT_WINDOW: u32 = 2;
pub const SYNTACTIC_LOCATION_VERSION: u32 = 1;

/// A durable, serializable description of a source location, re-resolved
/// against a buffer's outline at open/reload time.
///
/// Unlike [`text::Anchor`], this does not track edits in a live buffer; it is
/// re-resolved from scratch. While a buffer is open the live anchor is the
/// source of truth, and this description is only consulted when restoring a
/// location in a freshly opened buffer.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SyntacticLocation {
    /// `None` when there is no enclosing symbol — either the language has no
    /// parser/outline (e.g. plaintext), or the position sits between symbols.
    pub symbol: Option<SymbolRef>,
    pub content_marker: ContentMarker,
    /// Last known absolute row; the final fallback when nothing else matches.
    pub last_known_row: u32,
}

/// Identifies the symbol a source location is bound to, plus where inside it
/// the location sits.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolRef {
    /// Outline path of the enclosing symbol, innermost last.
    pub symbol_path: Vec<SharedString>,
    /// The nth occurrence (0-based) of an identical `symbol_path` in the file,
    /// used to disambiguate paths that collapse to the same text.
    pub symbol_ordinal: u32,
    /// Line offset from the symbol's start row to the source row.
    pub line_offset_in_symbol: u32,
}

/// A fingerprint of the source line and its surroundings, used to find the
/// exact line when the symbol-relative offset drifts.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContentMarker {
    /// Trimmed, whitespace-normalized text of the source line. The primary
    /// matcher when snapping the offset to the exact line.
    pub line_text: SharedString,
    /// Hash of a normalized window of surrounding lines (±[`CONTEXT_WINDOW`]).
    /// A tiebreaker only, consulted when `line_text` is ambiguous within the
    /// search window.
    pub context_hash: u64,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SerializedSyntacticLocation {
    pub symbol: Option<SerializedSymbolRef>,
    pub content_marker: SerializedContentMarker,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SerializedSourceLocation {
    pub row: u32,
    pub syntactic_location: Option<SerializedSyntacticLocation>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SerializedSymbolRef {
    pub symbol_path: Vec<String>,
    pub symbol_ordinal: u32,
    pub line_offset_in_symbol: u32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SerializedContentMarker {
    pub line_text: String,
    pub context_hash: u64,
}

impl SerializedSyntacticLocation {
    pub fn validate(&self) -> Result<()> {
        if self
            .symbol
            .as_ref()
            .is_some_and(|symbol| symbol.symbol_path.is_empty())
        {
            anyhow::bail!("syntactic location symbol path is empty");
        }
        Ok(())
    }

    pub(crate) fn to_syntactic_location(&self, last_known_row: u32) -> Result<SyntacticLocation> {
        self.validate()?;

        Ok(SyntacticLocation {
            symbol: self.symbol.as_ref().map(|symbol| SymbolRef {
                symbol_path: symbol
                    .symbol_path
                    .iter()
                    .cloned()
                    .map(SharedString::from)
                    .collect(),
                symbol_ordinal: symbol.symbol_ordinal,
                line_offset_in_symbol: symbol.line_offset_in_symbol,
            }),
            content_marker: ContentMarker {
                line_text: self.content_marker.line_text.clone().into(),
                context_hash: self.content_marker.context_hash,
            },
            last_known_row,
        })
    }
}

impl From<&SyntacticLocation> for SerializedSyntacticLocation {
    fn from(location: &SyntacticLocation) -> Self {
        Self {
            symbol: location.symbol.as_ref().map(|symbol| SerializedSymbolRef {
                symbol_path: symbol
                    .symbol_path
                    .iter()
                    .map(|segment| segment.to_string())
                    .collect(),
                symbol_ordinal: symbol.symbol_ordinal,
                line_offset_in_symbol: symbol.line_offset_in_symbol,
            }),
            content_marker: SerializedContentMarker {
                line_text: location.content_marker.line_text.to_string(),
                context_hash: location.content_marker.context_hash,
            },
        }
    }
}

/// Trims a line and collapses internal runs of whitespace into a single space,
/// so reindentation or trailing-whitespace cleanup doesn't invalidate a marker.
fn normalize_line(line: &str) -> String {
    line.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn line_text(snapshot: &BufferSnapshot, row: u32) -> String {
    let start = Point::new(row, 0);
    let end = Point::new(row, snapshot.line_len(row));
    snapshot.text_for_range(start..end).collect::<String>()
}

fn syntax_lookup_point(snapshot: &BufferSnapshot, row: u32) -> Point {
    let indent = snapshot.line_indent_for_row(row);
    let column = if indent.is_line_blank() {
        0
    } else {
        indent.raw_len()
    };
    Point::new(row, column)
}

/// Computes the [`SyntacticLocation`] for `anchor` in `snapshot`. This is
/// derived fresh from the current buffer contents at serialization time; while
/// a buffer is open the live anchor is the source of truth.
#[cfg(test)]
fn compute_syntactic_location(
    snapshot: &language::BufferSnapshot,
    anchor: text::Anchor,
) -> SyntacticLocation {
    let index = SyntacticLocationIndex::new(snapshot);
    compute_syntactic_location_with_index(snapshot, &index, anchor)
}

pub(crate) fn compute_syntactic_location_with_index(
    snapshot: &language::BufferSnapshot,
    index: &SyntacticLocationIndex,
    anchor: text::Anchor,
) -> SyntacticLocation {
    let row = anchor.summary::<Point>(snapshot).row;

    let symbol = compute_symbol_ref(snapshot, index, row);
    let content_marker = compute_content_marker(snapshot, row);

    SyntacticLocation {
        symbol,
        content_marker,
        last_known_row: row,
    }
}

pub(crate) struct SyntacticLocationIndex {
    symbols: Vec<IndexedSymbol>,
    rows_by_line_text: OnceCell<HashMap<SharedString, Vec<u32>>>,
}

struct IndexedSymbol {
    symbol_path: Vec<SharedString>,
    symbol_ordinal: u32,
    range: Range<Point>,
}

impl SyntacticLocationIndex {
    pub(crate) fn new(snapshot: &language::BufferSnapshot) -> Self {
        let outline = snapshot.outline(None);
        let mut path_stack = Vec::new();
        let mut next_ordinal_by_path = HashMap::<Vec<SharedString>, u32>::new();
        let mut symbols = Vec::with_capacity(outline.items.len());

        for item in &outline.items {
            path_stack.truncate(item.depth);
            path_stack.push(item.text.clone());
            let symbol_path = path_stack.clone();
            let next_ordinal = next_ordinal_by_path.entry(symbol_path.clone()).or_default();
            let symbol_ordinal = *next_ordinal;
            *next_ordinal += 1;

            symbols.push(IndexedSymbol {
                symbol_path,
                symbol_ordinal,
                range: item.range.start.summary::<Point>(snapshot)
                    ..item.range.end.summary::<Point>(snapshot),
            });
        }

        Self {
            symbols,
            rows_by_line_text: OnceCell::new(),
        }
    }

    fn ordinal_for(&self, range: &Range<Point>, symbol_path: &[SharedString]) -> Option<u32> {
        self.symbols
            .iter()
            .find(|symbol| symbol.range == *range && symbol.symbol_path == symbol_path)
            .map(|symbol| symbol.symbol_ordinal)
    }

    fn rows_matching_line(
        &self,
        snapshot: &language::BufferSnapshot,
        normalized_line: &SharedString,
    ) -> &[u32] {
        self.rows_by_line_text
            .get_or_init(|| {
                let mut rows_by_line_text = HashMap::<SharedString, Vec<u32>>::new();
                for row in 0..=snapshot.max_point().row {
                    rows_by_line_text
                        .entry(normalize_line(&line_text(snapshot, row)).into())
                        .or_default()
                        .push(row);
                }
                rows_by_line_text
            })
            .get(normalized_line)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }
}

fn compute_symbol_ref(
    snapshot: &language::BufferSnapshot,
    index: &SyntacticLocationIndex,
    row: u32,
) -> Option<SymbolRef> {
    let containing = snapshot.symbols_containing(syntax_lookup_point(snapshot, row), None);
    let innermost = containing.last()?;
    let symbol_path = containing
        .iter()
        .map(|item| item.text.clone())
        .collect::<Vec<_>>();

    let range = innermost.range.start.summary::<Point>(snapshot)
        ..innermost.range.end.summary::<Point>(snapshot);
    let line_offset_in_symbol = row.saturating_sub(range.start.row);
    let symbol_ordinal = index.ordinal_for(&range, &symbol_path)?;

    Some(SymbolRef {
        symbol_path,
        symbol_ordinal,
        line_offset_in_symbol,
    })
}

/// FNV-1a, implemented inline because the hash is persisted: it must remain
/// stable across releases, which rules out `std`'s unspecified default hasher
/// and third-party hashers that don't guarantee a fixed algorithm.
struct Fnv1a(u64);

impl Fnv1a {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    fn update(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

fn compute_content_marker(snapshot: &BufferSnapshot, row: u32) -> ContentMarker {
    let normalized = normalize_line(&line_text(snapshot, row));

    let mut hasher = Fnv1a::new();
    let max_row = snapshot.max_point().row;
    let start = row.saturating_sub(CONTEXT_WINDOW);
    let end = (row + CONTEXT_WINDOW).min(max_row);
    // Hash the target row's offset within the window, not just the window's
    // lines: near buffer boundaries the window is clamped, so two different
    // rows (e.g. the first and last line of a three-line file with identical
    // neighbors) can otherwise produce identical windows. The hash is
    // persisted, so changing any of its inputs requires bumping
    // [`SYNTACTIC_LOCATION_VERSION`].
    hasher.update(&(row - start).to_le_bytes());
    for context_row in start..=end {
        let context_line = normalize_line(&line_text(snapshot, context_row));
        let line_length = u32::try_from(context_line.len()).unwrap_or(u32::MAX);
        hasher.update(&line_length.to_le_bytes());
        hasher.update(context_line.as_bytes());
    }

    ContentMarker {
        line_text: normalized.into(),
        context_hash: hasher.finish(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceLocationResolutionKind {
    ExactSymbol,
    SymbolContent,
    ContentOnly,
    RowFallback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ResolvedSourceLocation {
    pub(crate) row: u32,
    pub(crate) resolution: SourceLocationResolutionKind,
}

pub(crate) fn resolve_syntactic_location(
    snapshot: &language::BufferSnapshot,
    index: &SyntacticLocationIndex,
    location: &SyntacticLocation,
) -> Option<ResolvedSourceLocation> {
    if let Some(symbol) = location.symbol.as_ref()
        && let Some(resolved) = resolve_exact_symbol(
            snapshot,
            index,
            symbol,
            &location.content_marker,
            location.last_known_row,
        )
    {
        return Some(resolved);
    }

    if let Some(row) = resolve_content_only(
        snapshot,
        index,
        &location.content_marker,
        location.last_known_row,
    ) {
        return Some(ResolvedSourceLocation {
            row,
            resolution: SourceLocationResolutionKind::ContentOnly,
        });
    }

    row_fallback(snapshot, location.last_known_row)
}

pub(crate) fn row_fallback(snapshot: &BufferSnapshot, row: u32) -> Option<ResolvedSourceLocation> {
    (row <= snapshot.max_point().row).then_some(ResolvedSourceLocation {
        row,
        resolution: SourceLocationResolutionKind::RowFallback,
    })
}

fn resolve_exact_symbol(
    snapshot: &language::BufferSnapshot,
    index: &SyntacticLocationIndex,
    symbol: &SymbolRef,
    content_marker: &ContentMarker,
    last_known_row: u32,
) -> Option<ResolvedSourceLocation> {
    let matching_symbols = index
        .symbols
        .iter()
        .filter(|candidate| candidate.symbol_path == symbol.symbol_path)
        .collect::<Vec<_>>();
    if matching_symbols.is_empty() {
        return None;
    }

    let preferred_symbol = matching_symbols
        .iter()
        .copied()
        .find(|candidate| candidate.symbol_ordinal == symbol.symbol_ordinal);

    let candidates = if content_marker.line_text.is_empty() {
        Vec::new()
    } else {
        matching_symbols
            .iter()
            .map(|matching_symbol| SymbolCandidate {
                expected_row: expected_row_in_range(
                    &matching_symbol.range,
                    symbol.line_offset_in_symbol,
                ),
                line_matches: content_matches_in_range(
                    snapshot,
                    &matching_symbol.range,
                    content_marker,
                ),
                is_preferred: matching_symbol.symbol_ordinal == symbol.symbol_ordinal,
            })
            .collect()
    };

    if let Some(resolved) = closest_context_match(&candidates) {
        return Some(resolved);
    }

    if let Some(resolved) = closest_line_match_in_preferred_symbol(&candidates) {
        return Some(resolved);
    }

    let preferred_symbol = preferred_symbol?;

    // Reaching this point with a non-empty content marker means the source line
    // is no longer inside any symbol with the stored path, so before guessing a
    // row positionally, check whether the exact line (confirmed by context
    // hash) moved elsewhere in the file, e.g. into another symbol.
    if let Some(row) = resolve_content_only(snapshot, index, content_marker, last_known_row) {
        return Some(ResolvedSourceLocation {
            row,
            resolution: SourceLocationResolutionKind::ContentOnly,
        });
    }

    Some(ResolvedSourceLocation {
        row: expected_row_in_range(&preferred_symbol.range, symbol.line_offset_in_symbol),
        resolution: SourceLocationResolutionKind::ExactSymbol,
    })
}

/// The line matches for one occurrence of the stored symbol path, used to
/// rank resolution candidates in [`resolve_exact_symbol`].
struct SymbolCandidate {
    /// The stored symbol-relative offset projected onto this occurrence.
    expected_row: u32,
    line_matches: Vec<ContentMatch>,
    /// Whether this occurrence has the stored [`SymbolRef::symbol_ordinal`].
    is_preferred: bool,
}

/// Picks the line match whose surrounding context hash still matches,
/// preferring the symbol occurrence with the stored ordinal, then the row
/// closest to the symbol-relative offset.
fn closest_context_match(candidates: &[SymbolCandidate]) -> Option<ResolvedSourceLocation> {
    struct RankedMatch {
        row: u32,
        is_preferred: bool,
        distance_from_expected: u32,
    }

    candidates
        .iter()
        .flat_map(|candidate| {
            candidate
                .line_matches
                .iter()
                .filter(|content_match| content_match.context_matches)
                .map(|content_match| RankedMatch {
                    row: content_match.row,
                    is_preferred: candidate.is_preferred,
                    distance_from_expected: content_match.row.abs_diff(candidate.expected_row),
                })
        })
        .min_by_key(|ranked| (!ranked.is_preferred, ranked.distance_from_expected))
        .map(|ranked| ResolvedSourceLocation {
            row: ranked.row,
            resolution: if ranked.is_preferred && ranked.distance_from_expected == 0 {
                SourceLocationResolutionKind::ExactSymbol
            } else {
                SourceLocationResolutionKind::SymbolContent
            },
        })
}

/// Falls back to the line match closest to the symbol-relative offset within
/// the symbol occurrence that has the stored ordinal, even though no context
/// hash matched.
fn closest_line_match_in_preferred_symbol(
    candidates: &[SymbolCandidate],
) -> Option<ResolvedSourceLocation> {
    let preferred = candidates.iter().find(|candidate| candidate.is_preferred)?;
    let content_match = preferred
        .line_matches
        .iter()
        .min_by_key(|content_match| content_match.row.abs_diff(preferred.expected_row))?;
    Some(ResolvedSourceLocation {
        row: content_match.row,
        resolution: if content_match.row == preferred.expected_row {
            SourceLocationResolutionKind::ExactSymbol
        } else {
            SourceLocationResolutionKind::SymbolContent
        },
    })
}

fn resolve_content_only(
    snapshot: &language::BufferSnapshot,
    index: &SyntacticLocationIndex,
    content_marker: &ContentMarker,
    last_known_row: u32,
) -> Option<u32> {
    if content_marker.line_text.is_empty() {
        return None;
    }

    let matching_rows = index.rows_matching_line(snapshot, &content_marker.line_text);
    let context_match = matching_rows
        .iter()
        .copied()
        .filter(|row| {
            compute_content_marker(snapshot, *row).context_hash == content_marker.context_hash
        })
        .min_by_key(|row| row.abs_diff(last_known_row));

    context_match.or_else(|| match matching_rows {
        [row] => Some(*row),
        _ => None,
    })
}

#[derive(Clone, Copy)]
struct ContentMatch {
    row: u32,
    context_matches: bool,
}

fn content_matches_in_range(
    snapshot: &language::BufferSnapshot,
    range: &Range<Point>,
    content_marker: &ContentMarker,
) -> Vec<ContentMatch> {
    let start_row = range.start.row.min(snapshot.max_point().row);
    let end_row = range_end_row(range).min(snapshot.max_point().row);
    if start_row > end_row {
        return Vec::new();
    }

    (start_row..=end_row)
        .filter_map(|row| {
            (normalize_line(&line_text(snapshot, row)) == content_marker.line_text.as_ref()).then(
                || ContentMatch {
                    row,
                    context_matches: compute_content_marker(snapshot, row).context_hash
                        == content_marker.context_hash,
                },
            )
        })
        .collect()
}

fn expected_row_in_range(range: &Range<Point>, line_offset_in_symbol: u32) -> u32 {
    range
        .start
        .row
        .saturating_add(line_offset_in_symbol)
        .clamp(range.start.row, range_end_row(range))
}

/// A symbol range that spans whole lines ends on column 0 of the following
/// line; treat that as ending on the previous row.
fn range_end_row(range: &Range<Point>) -> u32 {
    if range.end.column == 0 && range.end.row > range.start.row {
        range.end.row - 1
    } else {
        range.end.row
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use gpui::{AppContext as _, TestAppContext};
    use language::{Buffer, Language, LanguageConfig, rust_lang};

    fn syntactic_location_at_row(
        text: &str,
        row: u32,
        with_rust_language: bool,
        cx: &mut TestAppContext,
    ) -> SyntacticLocation {
        let buffer = cx.new(|cx| {
            let buffer = Buffer::local(text, cx);
            if with_rust_language {
                buffer.with_language(rust_lang(), cx)
            } else {
                buffer
            }
        });
        cx.update(|cx| {
            let snapshot = buffer.read(cx).snapshot();
            let anchor = snapshot.anchor_after(Point::new(row, 0));
            compute_syntactic_location(&snapshot, anchor)
        })
    }

    fn typescript_lang() -> Arc<Language> {
        Arc::new(
            Language::new(
                LanguageConfig {
                    name: "TypeScript".into(),
                    ..Default::default()
                },
                Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            )
            .with_outline_query(include_str!("../../grammars/src/typescript/outline.scm"))
            .expect("valid TypeScript outline query"),
        )
    }

    fn typescript_syntactic_location_at_row(
        text: &str,
        row: u32,
        cx: &mut TestAppContext,
    ) -> SyntacticLocation {
        let language = typescript_lang();
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language, cx));
        cx.update(|cx| {
            let snapshot = buffer.read(cx).snapshot();
            let anchor = snapshot.anchor_after(Point::new(row, 0));
            compute_syntactic_location(&snapshot, anchor)
        })
    }

    fn resolve_location(
        text: &str,
        language: Option<Arc<Language>>,
        location: &SyntacticLocation,
        cx: &mut TestAppContext,
    ) -> ResolvedSourceLocation {
        let buffer = cx.new(|cx| {
            let buffer = Buffer::local(text, cx);
            if let Some(language) = language {
                buffer.with_language(language, cx)
            } else {
                buffer
            }
        });
        cx.update(|cx| {
            let snapshot = buffer.read(cx).snapshot();
            let index = SyntacticLocationIndex::new(&snapshot);
            resolve_syntactic_location(&snapshot, &index, location)
                .expect("expected the source location to resolve")
        })
    }

    #[gpui::test]
    fn test_syntactic_location_inside_rust_function(cx: &mut TestAppContext) {
        let location = syntactic_location_at_row(
            "fn bookmarked() {\n    let value = 1;\n    dbg!(value);\n}\n",
            2,
            true,
            cx,
        );

        assert_eq!(
            location.symbol,
            Some(SymbolRef {
                symbol_path: vec!["fn bookmarked".into()],
                symbol_ordinal: 0,
                line_offset_in_symbol: 2,
            })
        );
        assert_eq!(location.content_marker.line_text, "dbg!(value);");
        assert_eq!(location.last_known_row, 2);
    }

    #[gpui::test]
    fn test_syntactic_location_uses_nested_symbol_path(cx: &mut TestAppContext) {
        let location = syntactic_location_at_row(
            "struct Store;\n\nimpl Store {\n    fn bookmarked(&self) {\n        let value = 1;\n    }\n}\n",
            3,
            true,
            cx,
        );

        let symbol = location.symbol.expect("expected an enclosing Rust symbol");
        assert_eq!(
            symbol.symbol_path,
            vec![
                SharedString::from("impl Store"),
                SharedString::from("fn bookmarked")
            ]
        );
        assert_eq!(symbol.symbol_ordinal, 0);
        assert_eq!(symbol.line_offset_in_symbol, 0);
    }

    #[gpui::test]
    fn test_syntactic_location_disambiguates_duplicate_symbol_paths(cx: &mut TestAppContext) {
        let text =
            "fn duplicate() {\n    let first = 1;\n}\n\nfn duplicate() {\n    let second = 2;\n}\n";
        let first = syntactic_location_at_row(text, 1, true, cx);
        let second = syntactic_location_at_row(text, 5, true, cx);

        let first_symbol = first.symbol.expect("expected the first function");
        let second_symbol = second.symbol.expect("expected the second function");
        assert_eq!(first_symbol.symbol_path, second_symbol.symbol_path);
        assert_eq!(first_symbol.symbol_ordinal, 0);
        assert_eq!(second_symbol.symbol_ordinal, 1);
    }

    #[gpui::test]
    fn test_typescript_syntactic_location_uses_nested_class_method_path(cx: &mut TestAppContext) {
        let location = typescript_syntactic_location_at_row(
            "class Store {\n    bookmarked(): void {\n        const value = 1;\n    }\n}\n",
            1,
            cx,
        );

        let symbol = location
            .symbol
            .expect("expected an enclosing TypeScript symbol");
        assert_eq!(
            symbol.symbol_path,
            vec![
                SharedString::from("class Store"),
                SharedString::from("bookmarked()")
            ]
        );
        assert_eq!(symbol.symbol_ordinal, 0);
        assert_eq!(symbol.line_offset_in_symbol, 0);
    }

    #[gpui::test]
    fn test_typescript_syntactic_location_disambiguates_duplicate_functions(
        cx: &mut TestAppContext,
    ) {
        let text = "function duplicate() {\n    return 1;\n}\n\nfunction duplicate() {\n    return 2;\n}\n";
        let first = typescript_syntactic_location_at_row(text, 1, cx);
        let second = typescript_syntactic_location_at_row(text, 5, cx);

        let first_symbol = first.symbol.expect("expected the first function");
        let second_symbol = second.symbol.expect("expected the second function");
        assert_eq!(first_symbol.symbol_path, second_symbol.symbol_path);
        assert_eq!(first_symbol.symbol_ordinal, 0);
        assert_eq!(second_symbol.symbol_ordinal, 1);
    }

    #[gpui::test]
    fn test_syntactic_location_without_parser_has_no_symbol(cx: &mut TestAppContext) {
        let location = syntactic_location_at_row("heading\nbookmarked text\n", 1, false, cx);

        assert_eq!(location.symbol, None);
        assert_eq!(location.content_marker.line_text, "bookmarked text");
        assert_eq!(location.last_known_row, 1);
    }

    #[gpui::test]
    fn test_syntactic_location_on_whitespace_between_symbols_has_no_symbol(
        cx: &mut TestAppContext,
    ) {
        let location =
            syntactic_location_at_row("fn first() {}\n    \nfn second() {}\n", 1, true, cx);

        assert_eq!(location.symbol, None);
    }

    #[gpui::test]
    fn test_content_marker_normalizes_whitespace(cx: &mut TestAppContext) {
        let first = syntactic_location_at_row("    let   value = 1;   \n", 0, false, cx);
        let second = syntactic_location_at_row("\tlet value = 1;\n", 0, false, cx);

        assert_eq!(first.content_marker.line_text, "let value = 1;");
        assert_eq!(
            first.content_marker.line_text,
            second.content_marker.line_text
        );
        assert_eq!(
            first.content_marker.context_hash,
            second.content_marker.context_hash
        );
    }

    #[gpui::test]
    fn test_content_marker_context_disambiguates_identical_lines(cx: &mut TestAppContext) {
        let first =
            syntactic_location_at_row("before one\nreturn value;\nafter one\n", 1, false, cx);
        let second =
            syntactic_location_at_row("before two\nreturn value;\nafter two\n", 1, false, cx);

        assert_eq!(
            first.content_marker.line_text,
            second.content_marker.line_text
        );
        assert_ne!(
            first.content_marker.context_hash,
            second.content_marker.context_hash
        );
    }

    #[gpui::test]
    fn test_content_marker_clamps_context_at_buffer_boundaries(cx: &mut TestAppContext) {
        let first = syntactic_location_at_row("one\ntwo\nthree\n", 0, false, cx);
        let first_again = syntactic_location_at_row("one\ntwo\nthree\n", 0, false, cx);

        assert_eq!(
            first.content_marker.context_hash,
            first_again.content_marker.context_hash
        );
        // Pinned value: the hash is persisted, so it must stay stable.
        assert_eq!(first.content_marker.context_hash, 5739249799607469836);
    }

    #[gpui::test]
    fn test_content_marker_encodes_target_position(cx: &mut TestAppContext) {
        let first = syntactic_location_at_row("same\nmiddle\nsame", 0, false, cx);
        let last = syntactic_location_at_row("same\nmiddle\nsame", 2, false, cx);

        assert_eq!(
            first.content_marker.line_text,
            last.content_marker.line_text
        );
        assert_ne!(
            first.content_marker.context_hash,
            last.content_marker.context_hash
        );
    }

    #[gpui::test]
    fn test_resolves_source_location_after_lines_inserted_above_symbol(cx: &mut TestAppContext) {
        let location =
            syntactic_location_at_row("fn bookmarked() {\n    target();\n}\n", 1, true, cx);

        let resolved = resolve_location(
            "fn unrelated() {}\n\nfn bookmarked() {\n    target();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 3,
                resolution: SourceLocationResolutionKind::ExactSymbol,
            }
        );
    }

    #[gpui::test]
    fn test_resolves_source_location_after_line_inserted_inside_symbol(cx: &mut TestAppContext) {
        let location = syntactic_location_at_row(
            "fn bookmarked() {\n    let before = 1;\n    target();\n}\n",
            2,
            true,
            cx,
        );

        let resolved = resolve_location(
            "fn bookmarked() {\n    let inserted = 0;\n    let before = 1;\n    target();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 3,
                resolution: SourceLocationResolutionKind::SymbolContent,
            }
        );
    }

    #[gpui::test]
    fn test_resolves_source_location_after_symbol_rename(cx: &mut TestAppContext) {
        let location =
            syntactic_location_at_row("fn bookmarked() {\n    target();\n}\n", 1, true, cx);

        let resolved = resolve_location(
            "fn bookmark() {\n    target();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 1,
                resolution: SourceLocationResolutionKind::ContentOnly,
            }
        );
    }

    #[gpui::test]
    fn test_resolves_source_location_when_duplicate_symbol_changes_ordinal(
        cx: &mut TestAppContext,
    ) {
        let location = syntactic_location_at_row(
            "fn duplicate() {\n    first();\n}\n\nfn duplicate() {\n    bookmarked();\n}\n",
            5,
            true,
            cx,
        );

        let resolved = resolve_location(
            "fn duplicate() {\n    inserted();\n}\n\nfn duplicate() {\n    first();\n}\n\nfn duplicate() {\n    bookmarked();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 9,
                resolution: SourceLocationResolutionKind::SymbolContent,
            }
        );
    }

    #[gpui::test]
    fn test_resolves_source_location_when_line_moves_to_another_symbol(cx: &mut TestAppContext) {
        let location = syntactic_location_at_row(
            "fn alpha() {\n    one();\n    setup();\n    let marker = compute_unique_thing();\n    teardown();\n    two();\n}\n\nfn beta() {\n    other();\n}\n",
            3,
            true,
            cx,
        );

        let resolved = resolve_location(
            "fn alpha() {\n    filler();\n}\n\nfn beta() {\n    one();\n    setup();\n    let marker = compute_unique_thing();\n    teardown();\n    two();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 7,
                resolution: SourceLocationResolutionKind::ContentOnly,
            }
        );
    }

    #[gpui::test]
    fn test_symbol_without_matching_content_resolves_to_symbol_offset(cx: &mut TestAppContext) {
        let location = syntactic_location_at_row(
            "fn bookmarked() {\n    setup();\n    target();\n    teardown();\n}\n",
            2,
            true,
            cx,
        );

        let resolved = resolve_location(
            "fn bookmarked() {\n    setup();\n    replaced();\n    teardown();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 2,
                resolution: SourceLocationResolutionKind::ExactSymbol,
            }
        );
    }

    #[gpui::test]
    fn test_resolves_typescript_source_location_after_method_moves(cx: &mut TestAppContext) {
        let location = typescript_syntactic_location_at_row(
            "class Store {\n    bookmarked(): void {\n        target();\n    }\n}\n",
            2,
            cx,
        );

        let resolved = resolve_location(
            "function unrelated() {}\n\nclass Store {\n    bookmarked(): void {\n        const inserted = 1;\n        target();\n    }\n}\n",
            Some(typescript_lang()),
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 5,
                resolution: SourceLocationResolutionKind::SymbolContent,
            }
        );
    }

    #[gpui::test]
    fn test_resolves_plaintext_source_location_from_content_context(cx: &mut TestAppContext) {
        let location =
            syntactic_location_at_row("prefix\nbefore\nbookmarked\nafter\nsuffix\n", 2, false, cx);

        let resolved = resolve_location(
            "unrelated\nprefix\nbefore\nbookmarked\nafter\nsuffix\n",
            None,
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 3,
                resolution: SourceLocationResolutionKind::ContentOnly,
            }
        );
    }

    #[gpui::test]
    fn test_plaintext_unique_line_survives_nearby_insertion(cx: &mut TestAppContext) {
        let location = syntactic_location_at_row(
            "before one\nbefore two\nbookmarked\nafter one\nafter two\n",
            2,
            false,
            cx,
        );

        let resolved = resolve_location(
            "before one\nbefore two\ninserted\nbookmarked\nafter one\nafter two\n",
            None,
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 3,
                resolution: SourceLocationResolutionKind::ContentOnly,
            }
        );
    }

    #[gpui::test]
    fn test_plaintext_content_without_matching_context_falls_back_to_row(cx: &mut TestAppContext) {
        let location = syntactic_location_at_row("before\nbookmarked\nafter\n", 1, false, cx);

        let resolved = resolve_location(
            "replacement\nold row\nbookmarked\nunrelated\nbookmarked\ndifferent\n",
            None,
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 1,
                resolution: SourceLocationResolutionKind::RowFallback,
            }
        );
    }

    #[gpui::test]
    fn test_resolves_symbol_when_last_known_row_is_out_of_range(cx: &mut TestAppContext) {
        let mut location =
            syntactic_location_at_row("fn bookmarked() {\n    target();\n}\n", 1, true, cx);
        location.last_known_row = 100;

        let resolved = resolve_location(
            "fn bookmarked() {\n    target();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(resolved.row, 1);
        assert_ne!(
            resolved.resolution,
            SourceLocationResolutionKind::RowFallback
        );
    }

    #[gpui::test]
    fn test_deleted_symbol_without_matching_content_falls_back_to_row(cx: &mut TestAppContext) {
        let location =
            syntactic_location_at_row("fn bookmarked() {\n    target();\n}\n", 1, true, cx);

        let resolved = resolve_location(
            "fn replacement() {\n    different();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(
            resolved,
            ResolvedSourceLocation {
                row: 1,
                resolution: SourceLocationResolutionKind::RowFallback,
            }
        );
    }

    #[gpui::test]
    fn test_serialized_syntactic_location_round_trip(cx: &mut TestAppContext) {
        let location = typescript_syntactic_location_at_row(
            "class Store {\n    bookmarked(): void {\n        const value = 1;\n    }\n}\n",
            2,
            cx,
        );
        let serialized = SerializedSyntacticLocation::from(&location);
        let restored = serialized
            .to_syntactic_location(location.last_known_row)
            .expect("valid serialized syntactic location");

        assert_eq!(restored, location);
    }

    #[test]
    fn test_serialized_syntactic_location_rejects_empty_symbol_path() {
        let serialized = SerializedSyntacticLocation {
            symbol: Some(SerializedSymbolRef {
                symbol_path: Vec::new(),
                symbol_ordinal: 0,
                line_offset_in_symbol: 0,
            }),
            content_marker: SerializedContentMarker {
                line_text: "bookmarked".to_string(),
                context_hash: 0,
            },
        };

        assert!(serialized.to_syntactic_location(0).is_err());
    }
}
