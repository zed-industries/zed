use std::{cell::OnceCell, collections::HashMap, ops::Range};

use anyhow::Result;
use gpui::SharedString;
use language::{Buffer, PLAIN_TEXT, ParseStatus};
use serde::{Deserialize, Serialize};
use text::{BufferSnapshot, Point};

/// Number of lines above and below the source line hashed into
/// [`ContentMarker::context_hash`].
const CONTEXT_ROW_RADIUS: u32 = 2;
/// Format version for [`SerializedSyntacticLocation`].
///
/// This must be incremented when its fields or the inputs to
/// [`SerializedContentMarker::context_hash`] change.
pub const SYNTACTIC_LOCATION_FORMAT_VERSION: u32 = 1;

/// An in-memory description of a source location, re-resolved against a buffer's
/// outline at open/reload time.
///
/// Unlike [`text::Anchor`], this does not track edits in a live buffer; it is
/// re-resolved from scratch. While a buffer is open the live anchor is the
/// source of truth, and this description is only consulted when restoring a
/// location in a freshly opened buffer.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SyntacticLocation {
    /// `None` when there is no enclosing symbol — either the language has no
    /// parser/outline (e.g. plaintext), or the position sits between symbols.
    symbol: Option<SymbolRef>,
    content_marker: ContentMarker,
    /// The final fallback when nothing else matches.
    fallback_row: u32,
}

/// Identifies the symbol a source location is bound to, plus where inside it
/// the location sits.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SymbolRef {
    /// Outline path of the enclosing symbol, innermost last.
    symbol_path: Vec<SharedString>,
    /// The nth occurrence (0-based) of an identical `symbol_path` in the file,
    /// used to disambiguate paths that collapse to the same text.
    symbol_ordinal: u32,
    /// Line offset from the symbol's start row to the source row.
    line_offset_in_symbol: u32,
}

/// A fingerprint of the source line and its surroundings, used to find the
/// exact line when the symbol-relative offset drifts.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ContentMarker {
    /// Trimmed, whitespace-normalized text of the source line. The primary
    /// matcher when snapping the offset to the exact line.
    line_text: SharedString,
    /// Hash of a normalized window of surrounding lines
    /// (±[`CONTEXT_ROW_RADIUS`]).
    /// A tiebreaker only, consulted when `line_text` is ambiguous within the
    /// search window.
    context_hash: u64,
}

/// Persistable syntactic and textual metadata used to relocate a source
/// location.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SerializedSyntacticLocation {
    /// The enclosing symbol, or `None` when no parser-backed symbol is
    /// available.
    pub symbol: Option<SerializedSymbolRef>,
    /// A textual fingerprint used to recover the source row.
    pub content_marker: SerializedContentMarker,
}

/// A source location that can be re-resolved after its buffer is reopened.
///
/// Consumers choose how to persist this storage-agnostic envelope. The
/// syntactic payload's format is versioned by
/// [`SYNTACTIC_LOCATION_FORMAT_VERSION`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DurableSourceLocation {
    /// The last known row, used when stronger metadata cannot resolve.
    pub fallback_row: u32,
    /// Optional metadata for relocating the source row after edits.
    pub syntactic_location: Option<SerializedSyntacticLocation>,
}

/// Resolves and serializes durable source locations against one coherent buffer
/// snapshot.
///
/// The resolver captures the buffer's text, syntax state, and syntactic index
/// when it is created. Callers should reuse it for a batch of locations, then
/// create a new resolver after the buffer changes or finishes parsing.
///
/// This type does not retain pending locations or subscribe to buffer events.
/// Consumers are responsible for preserving prior complete metadata and
/// retrying deferred work when syntax becomes ready.
pub(crate) struct SourceLocationResolver {
    snapshot: language::BufferSnapshot,
    index: SourceLocationIndex,
    syntax_state: SourceLocationSyntaxState,
}

/// Availability of symbol information in a resolver's captured snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceLocationSyntaxState {
    /// No parser-backed syntax is available. Content markers remain usable, but
    /// symbol references cannot be computed or resolved.
    Unavailable,
    /// A parser is running and its symbol information may be incomplete.
    Parsing,
    /// Parser-backed symbol information is ready.
    Ready,
}

/// The result of resolving a valid serialized source location.
///
/// Invalid serialized data is returned as an error by
/// [`SourceLocationResolver::resolve`] instead of using one of these variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceLocationResolution {
    /// The location resolved to a concrete row.
    Resolved {
        /// The resolved zero-based row.
        row: u32,
        /// The strategy that produced the row.
        kind: SourceLocationResolutionKind,
    },
    /// Symbol-dependent resolution must wait for parser-backed syntax.
    Deferred {
        /// A clamped row that can be used until resolution is retried.
        provisional_row: u32,
    },
    /// The location was valid but neither its durable metadata nor fallback row
    /// could be resolved in this snapshot.
    Unresolvable,
}

/// The result of serializing a live anchor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SourceLocationSerialization {
    /// The serialized location has the highest fidelity available for the
    /// buffer's settled syntax state.
    Complete(DurableSourceLocation),
    /// Parsing is in progress, so the serialized location contains a current
    /// row and content marker but deliberately omits symbol information.
    ///
    /// Consumers with previously complete metadata should preserve that
    /// metadata and update its fallback row instead of replacing it with this
    /// provisional value.
    Provisional(DurableSourceLocation),
}

/// Persistable identity and relative position of an enclosing symbol.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SerializedSymbolRef {
    /// Outline path of the enclosing symbol, innermost last.
    pub symbol_path: Vec<String>,
    /// The zero-based occurrence of an identical symbol path in the file.
    pub symbol_ordinal: u32,
    /// The source row's offset from the symbol's start row.
    pub line_offset_in_symbol: u32,
}

/// Persistable fingerprint of a source line and its surroundings.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SerializedContentMarker {
    /// Trimmed line text with internal whitespace collapsed.
    pub line_text: String,
    /// Stable hash of the normalized context window around the source row.
    pub context_hash: u64,
}

impl SerializedSyntacticLocation {
    /// Validates invariants required by the resolver.
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

    fn to_syntactic_location(&self, fallback_row: u32) -> Result<SyntacticLocation> {
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
            fallback_row,
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

impl SourceLocationResolver {
    /// Captures a buffer snapshot and builds the index shared by subsequent
    /// serialization and resolution operations.
    pub fn for_buffer(buffer: &Buffer) -> Self {
        let snapshot = buffer.snapshot();
        let mut parse_status = buffer.parse_status();
        let syntax_state = if *parse_status.borrow() == ParseStatus::Parsing {
            SourceLocationSyntaxState::Parsing
        } else if snapshot
            .language()
            .is_none_or(|language| language == &*PLAIN_TEXT)
        {
            SourceLocationSyntaxState::Unavailable
        } else {
            SourceLocationSyntaxState::Ready
        };

        Self {
            index: SourceLocationIndex::new(&snapshot),
            snapshot,
            syntax_state,
        }
    }

    /// Returns the syntax availability captured when this resolver was created.
    pub fn syntax_state(&self) -> SourceLocationSyntaxState {
        self.syntax_state
    }

    /// Resolves a serialized source location against the captured snapshot.
    ///
    /// Symbol-bearing locations return [`SourceLocationResolution::Deferred`]
    /// while syntax is unavailable or parsing. Symbol-less content markers can
    /// resolve without parser-backed syntax. Invalid serialized data is
    /// returned as an error.
    pub fn resolve(&self, location: &DurableSourceLocation) -> Result<SourceLocationResolution> {
        if let Some(syntactic_location) = &location.syntactic_location {
            syntactic_location.validate()?;
        }

        let should_defer = location
            .syntactic_location
            .as_ref()
            .is_some_and(|syntactic_location| {
                syntactic_location.symbol.is_some()
                    && matches!(
                        self.syntax_state,
                        SourceLocationSyntaxState::Parsing | SourceLocationSyntaxState::Unavailable
                    )
            });
        if should_defer {
            return Ok(SourceLocationResolution::Deferred {
                provisional_row: location.fallback_row.min(self.snapshot.max_point().row),
            });
        }

        let resolved = match location.syntactic_location.as_ref() {
            Some(syntactic_location) => {
                let syntactic_location =
                    syntactic_location.to_syntactic_location(location.fallback_row)?;
                resolve_syntactic_location(&self.snapshot, &self.index, &syntactic_location)
            }
            None => resolve_from_fallback_row(&self.snapshot, location.fallback_row),
        };

        Ok(resolved
            .map(SourceLocationResolution::from)
            .unwrap_or(SourceLocationResolution::Unresolvable))
    }

    /// Resolves only a fallback row.
    ///
    /// This operation does not depend on syntax and therefore never returns
    /// [`SourceLocationResolution::Deferred`].
    pub fn resolve_fallback_row(&self, fallback_row: u32) -> SourceLocationResolution {
        resolve_from_fallback_row(&self.snapshot, fallback_row)
            .map(SourceLocationResolution::from)
            .unwrap_or(SourceLocationResolution::Unresolvable)
    }

    /// Returns the current row for an anchor resolvable in the captured
    /// snapshot.
    pub fn row_for_anchor(&self, anchor: text::Anchor) -> Option<u32> {
        self.snapshot
            .can_resolve(&anchor)
            .then(|| self.snapshot.summary_for_anchor::<Point>(&anchor).row)
    }

    /// Serializes an anchor resolvable in the captured snapshot.
    ///
    /// While parsing, this returns
    /// [`SourceLocationSerialization::Provisional`] with symbol information
    /// omitted. Once parsing settles, it returns
    /// [`SourceLocationSerialization::Complete`]. Syntax-unavailable buffers
    /// produce complete content-only locations because no parser-backed symbol
    /// information is expected for that snapshot.
    pub fn serialize_anchor(&self, anchor: text::Anchor) -> Option<SourceLocationSerialization> {
        let row = self.row_for_anchor(anchor)?;
        let serialization = if self.syntax_state == SourceLocationSyntaxState::Parsing {
            let content_marker = compute_content_marker(&self.snapshot, row);
            SourceLocationSerialization::Provisional(DurableSourceLocation {
                fallback_row: row,
                syntactic_location: Some(SerializedSyntacticLocation {
                    symbol: None,
                    content_marker: SerializedContentMarker {
                        line_text: content_marker.line_text.to_string(),
                        context_hash: content_marker.context_hash,
                    },
                }),
            })
        } else {
            let syntactic_location =
                syntactic_location_for_anchor(&self.snapshot, &self.index, anchor);
            SourceLocationSerialization::Complete(DurableSourceLocation {
                fallback_row: row,
                syntactic_location: Some((&syntactic_location).into()),
            })
        };
        Some(serialization)
    }
}

impl From<ResolvedSourceLocation> for SourceLocationResolution {
    fn from(resolved: ResolvedSourceLocation) -> Self {
        SourceLocationResolution::Resolved {
            row: resolved.row,
            kind: resolved.kind,
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
    let index = SourceLocationIndex::new(snapshot);
    syntactic_location_for_anchor(snapshot, &index, anchor)
}

fn syntactic_location_for_anchor(
    snapshot: &language::BufferSnapshot,
    index: &SourceLocationIndex,
    anchor: text::Anchor,
) -> SyntacticLocation {
    let row = anchor.summary::<Point>(snapshot).row;

    let symbol = compute_symbol_ref(snapshot, index, row);
    let content_marker = compute_content_marker(snapshot, row);

    SyntacticLocation {
        symbol,
        content_marker,
        fallback_row: row,
    }
}

struct SourceLocationIndex {
    symbols: Vec<IndexedSymbol>,
    rows_by_line_text: OnceCell<HashMap<SharedString, Vec<u32>>>,
}

struct IndexedSymbol {
    symbol_path: Vec<SharedString>,
    symbol_ordinal: u32,
    range: Range<Point>,
}

impl SourceLocationIndex {
    fn new(snapshot: &language::BufferSnapshot) -> Self {
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
    index: &SourceLocationIndex,
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
    let start = row.saturating_sub(CONTEXT_ROW_RADIUS);
    let end = row.saturating_add(CONTEXT_ROW_RADIUS).min(max_row);
    // Hash the target row's offset within the window, not just the window's
    // lines: near buffer boundaries the window is clamped, so two different
    // rows (e.g. the first and last line of a three-line file with identical
    // neighbors) can otherwise produce identical windows. The hash is
    // persisted, so changing any of its inputs requires bumping
    // [`SYNTACTIC_LOCATION_FORMAT_VERSION`].
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

/// How a durable source location was resolved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceLocationResolutionKind {
    /// Resolution stayed at the expected offset in the preferred symbol.
    ExactSymbolOffset,
    /// Content within a matching symbol selected a different row or symbol
    /// occurrence than the stored offset.
    ContentWithinSymbol,
    /// Content outside the stored symbol, or without a symbol reference,
    /// selected the row.
    ContentOnly,
    /// Only the stored absolute row could be used.
    RowFallback,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ResolvedSourceLocation {
    row: u32,
    kind: SourceLocationResolutionKind,
}

fn resolve_syntactic_location(
    snapshot: &language::BufferSnapshot,
    index: &SourceLocationIndex,
    location: &SyntacticLocation,
) -> Option<ResolvedSourceLocation> {
    if let Some(symbol) = location.symbol.as_ref()
        && let Some(resolved) = resolve_with_symbol(
            snapshot,
            index,
            symbol,
            &location.content_marker,
            location.fallback_row,
        )
    {
        return Some(resolved);
    }

    if let Some(row) = resolve_content_only(
        snapshot,
        index,
        &location.content_marker,
        location.fallback_row,
    ) {
        return Some(ResolvedSourceLocation {
            row,
            kind: SourceLocationResolutionKind::ContentOnly,
        });
    }

    resolve_from_fallback_row(snapshot, location.fallback_row)
}

fn resolve_from_fallback_row(
    snapshot: &BufferSnapshot,
    fallback_row: u32,
) -> Option<ResolvedSourceLocation> {
    (fallback_row <= snapshot.max_point().row).then_some(ResolvedSourceLocation {
        row: fallback_row,
        kind: SourceLocationResolutionKind::RowFallback,
    })
}

fn resolve_with_symbol(
    snapshot: &language::BufferSnapshot,
    index: &SourceLocationIndex,
    symbol: &SymbolRef,
    content_marker: &ContentMarker,
    fallback_row: u32,
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

    if let Some(resolved) = best_context_match(&candidates) {
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
    if let Some(row) = resolve_content_only(snapshot, index, content_marker, fallback_row) {
        return Some(ResolvedSourceLocation {
            row,
            kind: SourceLocationResolutionKind::ContentOnly,
        });
    }

    Some(ResolvedSourceLocation {
        row: expected_row_in_range(&preferred_symbol.range, symbol.line_offset_in_symbol),
        kind: SourceLocationResolutionKind::ExactSymbolOffset,
    })
}

/// The line matches for one occurrence of the stored symbol path, used to
/// rank resolution candidates in [`resolve_with_symbol`].
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
fn best_context_match(candidates: &[SymbolCandidate]) -> Option<ResolvedSourceLocation> {
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
            kind: if ranked.is_preferred && ranked.distance_from_expected == 0 {
                SourceLocationResolutionKind::ExactSymbolOffset
            } else {
                SourceLocationResolutionKind::ContentWithinSymbol
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
        kind: if content_match.row == preferred.expected_row {
            SourceLocationResolutionKind::ExactSymbolOffset
        } else {
            SourceLocationResolutionKind::ContentWithinSymbol
        },
    })
}

fn resolve_content_only(
    snapshot: &language::BufferSnapshot,
    index: &SourceLocationIndex,
    content_marker: &ContentMarker,
    fallback_row: u32,
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
        .min_by_key(|row| row.abs_diff(fallback_row));

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
            let index = SourceLocationIndex::new(&snapshot);
            resolve_syntactic_location(&snapshot, &index, location)
                .expect("expected the source location to resolve")
        })
    }

    fn serialized_location(
        fallback_row: u32,
        symbol: Option<SerializedSymbolRef>,
        line_text: &str,
    ) -> DurableSourceLocation {
        DurableSourceLocation {
            fallback_row,
            syntactic_location: Some(SerializedSyntacticLocation {
                symbol,
                content_marker: SerializedContentMarker {
                    line_text: line_text.to_string(),
                    context_hash: 0,
                },
            }),
        }
    }

    #[gpui::test]
    fn test_source_location_resolver_serializes_anchor(cx: &mut TestAppContext) {
        let buffer = cx.new(|cx| {
            Buffer::local("fn bookmarked() {\n    target();\n}\n", cx)
                .with_language(rust_lang(), cx)
        });
        let other_buffer = cx.new(|cx| Buffer::local("other\n", cx));

        cx.update(|cx| {
            let buffer = buffer.read(cx);
            let snapshot = buffer.snapshot();
            let anchor = snapshot.anchor_after(Point::new(1, 0));
            let resolver = SourceLocationResolver::for_buffer(buffer);
            let serialized = resolver
                .serialize_anchor(anchor)
                .expect("resolvable anchor");
            let SourceLocationSerialization::Complete(serialized) = serialized else {
                panic!("ready syntax should produce a complete source location");
            };

            assert_eq!(resolver.syntax_state(), SourceLocationSyntaxState::Ready);
            assert_eq!(serialized.fallback_row, 1);
            assert_eq!(
                serialized
                    .syntactic_location
                    .and_then(|location| location.symbol)
                    .expect("symbol")
                    .symbol_path,
                vec!["fn bookmarked".to_string()]
            );

            let other_anchor = other_buffer.read(cx).snapshot().anchor_after(Point::zero());
            assert_eq!(resolver.serialize_anchor(other_anchor), None);
        });
    }

    #[gpui::test]
    fn test_source_location_resolver_marks_serialization_provisional_while_parsing(
        cx: &mut TestAppContext,
    ) {
        let buffer = cx.new(|cx| Buffer::local("fn bookmarked() {\n    target();\n}\n", cx));
        buffer.update(cx, |buffer, cx| {
            buffer.set_sync_parse_timeout(None);
            buffer.set_language(Some(rust_lang()), cx);
        });

        cx.update(|cx| {
            let buffer = buffer.read(cx);
            assert!(buffer.is_parsing());
            let snapshot = buffer.snapshot();
            let anchor = snapshot.anchor_after(Point::new(1, 0));
            let resolver = SourceLocationResolver::for_buffer(buffer);
            let serialized = resolver
                .serialize_anchor(anchor)
                .expect("resolvable anchor");
            let SourceLocationSerialization::Provisional(serialized) = serialized else {
                panic!("parsing syntax should produce a provisional source location");
            };

            assert_eq!(serialized.fallback_row, 1);
            let syntactic_location = serialized
                .syntactic_location
                .expect("provisional content marker");
            assert_eq!(syntactic_location.symbol, None);
            assert_eq!(syntactic_location.content_marker.line_text, "target();");
        });
    }

    #[gpui::test]
    fn test_source_location_resolver_resolves_with_ready_syntax(cx: &mut TestAppContext) {
        let original =
            syntactic_location_at_row("fn bookmarked() {\n    target();\n}\n", 1, true, cx);
        let location = DurableSourceLocation {
            fallback_row: original.fallback_row,
            syntactic_location: Some((&original).into()),
        };
        let buffer = cx.new(|cx| {
            Buffer::local(
                "fn unrelated() {}\n\nfn bookmarked() {\n    target();\n}\n",
                cx,
            )
            .with_language(rust_lang(), cx)
        });

        cx.update(|cx| {
            let resolver = SourceLocationResolver::for_buffer(buffer.read(cx));
            assert_eq!(
                resolver.resolve(&location).expect("valid location"),
                SourceLocationResolution::Resolved {
                    row: 3,
                    kind: SourceLocationResolutionKind::ExactSymbolOffset,
                }
            );
        });
    }

    #[gpui::test]
    fn test_source_location_resolver_defers_symbol_while_syntax_is_unavailable(
        cx: &mut TestAppContext,
    ) {
        let location = serialized_location(
            10,
            Some(SerializedSymbolRef {
                symbol_path: vec!["fn bookmarked".to_string()],
                symbol_ordinal: 0,
                line_offset_in_symbol: 1,
            }),
            "target();",
        );
        let buffer = cx.new(|cx| Buffer::local("one\ntwo\n", cx));

        cx.update(|cx| {
            let resolver = SourceLocationResolver::for_buffer(buffer.read(cx));
            assert_eq!(
                resolver.syntax_state(),
                SourceLocationSyntaxState::Unavailable
            );
            assert_eq!(
                resolver.resolve(&location).expect("valid location"),
                SourceLocationResolution::Deferred { provisional_row: 2 }
            );
        });
    }

    #[gpui::test]
    fn test_source_location_resolver_defers_symbol_while_parsing(cx: &mut TestAppContext) {
        let location = serialized_location(
            1,
            Some(SerializedSymbolRef {
                symbol_path: vec!["fn bookmarked".to_string()],
                symbol_ordinal: 0,
                line_offset_in_symbol: 1,
            }),
            "target();",
        );
        let buffer = cx.new(|cx| Buffer::local("fn bookmarked() {\n    target();\n}\n", cx));
        buffer.update(cx, |buffer, cx| {
            buffer.set_sync_parse_timeout(None);
            buffer.set_language(Some(rust_lang()), cx);
        });

        cx.update(|cx| {
            let buffer = buffer.read(cx);
            assert!(buffer.is_parsing());
            let resolver = SourceLocationResolver::for_buffer(buffer);
            assert_eq!(resolver.syntax_state(), SourceLocationSyntaxState::Parsing);
            assert_eq!(
                resolver.resolve(&location).expect("valid location"),
                SourceLocationResolution::Deferred { provisional_row: 1 }
            );
        });
    }

    #[gpui::test]
    fn test_source_location_resolver_resolves_content_while_parsing(cx: &mut TestAppContext) {
        let location = serialized_location(0, None, "target();");
        let buffer = cx.new(|cx| Buffer::local("before\ntarget();\nafter\n", cx));
        buffer.update(cx, |buffer, cx| {
            buffer.set_sync_parse_timeout(None);
            buffer.set_language(Some(rust_lang()), cx);
        });

        cx.update(|cx| {
            let buffer = buffer.read(cx);
            assert!(buffer.is_parsing());
            let resolver = SourceLocationResolver::for_buffer(buffer);
            assert_eq!(resolver.syntax_state(), SourceLocationSyntaxState::Parsing);
            assert_eq!(
                resolver.resolve(&location).expect("valid location"),
                SourceLocationResolution::Resolved {
                    row: 1,
                    kind: SourceLocationResolutionKind::ContentOnly,
                }
            );
        });
    }

    #[gpui::test]
    fn test_source_location_resolver_resolves_plaintext_content(cx: &mut TestAppContext) {
        let original = syntactic_location_at_row("before\ntarget\nafter\n", 1, false, cx);
        let location = DurableSourceLocation {
            fallback_row: original.fallback_row,
            syntactic_location: Some((&original).into()),
        };
        let buffer = cx.new(|cx| {
            Buffer::local("inserted\nbefore\ntarget\nafter\n", cx)
                .with_language(PLAIN_TEXT.clone(), cx)
        });

        cx.update(|cx| {
            let resolver = SourceLocationResolver::for_buffer(buffer.read(cx));
            assert_eq!(
                resolver.syntax_state(),
                SourceLocationSyntaxState::Unavailable
            );
            assert_eq!(
                resolver.resolve(&location).expect("valid location"),
                SourceLocationResolution::Resolved {
                    row: 2,
                    kind: SourceLocationResolutionKind::ContentOnly,
                }
            );
        });
    }

    #[gpui::test]
    fn test_source_location_resolver_handles_row_only_and_invalid_locations(
        cx: &mut TestAppContext,
    ) {
        let buffer = cx.new(|cx| Buffer::local("one\ntwo\n", cx));

        cx.update(|cx| {
            let resolver = SourceLocationResolver::for_buffer(buffer.read(cx));
            assert_eq!(
                resolver.resolve_fallback_row(1),
                SourceLocationResolution::Resolved {
                    row: 1,
                    kind: SourceLocationResolutionKind::RowFallback,
                }
            );
            assert_eq!(
                resolver.resolve_fallback_row(3),
                SourceLocationResolution::Unresolvable
            );

            let invalid = serialized_location(
                0,
                Some(SerializedSymbolRef {
                    symbol_path: Vec::new(),
                    symbol_ordinal: 0,
                    line_offset_in_symbol: 0,
                }),
                "one",
            );
            assert!(resolver.resolve(&invalid).is_err());
        });
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
        assert_eq!(location.fallback_row, 2);
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
        assert_eq!(location.fallback_row, 1);
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
                kind: SourceLocationResolutionKind::ExactSymbolOffset,
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
                kind: SourceLocationResolutionKind::ContentWithinSymbol,
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
                kind: SourceLocationResolutionKind::ContentOnly,
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
                kind: SourceLocationResolutionKind::ContentWithinSymbol,
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
                kind: SourceLocationResolutionKind::ContentOnly,
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
                kind: SourceLocationResolutionKind::ExactSymbolOffset,
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
                kind: SourceLocationResolutionKind::ContentWithinSymbol,
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
                kind: SourceLocationResolutionKind::ContentOnly,
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
                kind: SourceLocationResolutionKind::ContentOnly,
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
                kind: SourceLocationResolutionKind::RowFallback,
            }
        );
    }

    #[gpui::test]
    fn test_resolves_symbol_when_fallback_row_is_out_of_range(cx: &mut TestAppContext) {
        let mut location =
            syntactic_location_at_row("fn bookmarked() {\n    target();\n}\n", 1, true, cx);
        location.fallback_row = 100;

        let resolved = resolve_location(
            "fn bookmarked() {\n    target();\n}\n",
            Some(rust_lang()),
            &location,
            cx,
        );

        assert_eq!(resolved.row, 1);
        assert_ne!(resolved.kind, SourceLocationResolutionKind::RowFallback);
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
                kind: SourceLocationResolutionKind::RowFallback,
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
            .to_syntactic_location(location.fallback_row)
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
