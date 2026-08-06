//! Bracket pair discovery for [`BufferSnapshot`]s.
//!
//! Bracket queries run per [`RowChunk`](super::row_chunk::RowChunk) and the results are
//! cached in [`TreeSitterData`](super::TreeSitterData) until the buffer is reparsed.
//! Tree-sitter error recovery can produce bogus or incomplete query matches, which are
//! repaired chunk-locally before caching, see [`ChunkBrackets`].

use std::{iter, ops::Range};

use collections::{HashMap, HashSet};
use text::{Bias, OffsetRangeExt, ToOffset};
use util::RangeExt;

use crate::{
    BracketsConfig, BufferRow, BufferSnapshot, Grammar, GrammarId, TreeSitterOptions,
    syntax_map::{MAX_BYTES_TO_QUERY, SyntaxLayer},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BracketMatch {
    pub open_range: Range<usize>,
    pub close_range: Range<usize>,
    pub newline_only: bool,
    pub syntax_layer_depth: usize,
    pub color_index: Option<usize>,
}

impl BracketMatch {
    pub fn bracket_ranges(self) -> (Range<usize>, Range<usize>) {
        (self.open_range, self.close_range)
    }
}

/// A single bracket pair candidate produced by a brackets query, before
/// bogus tree-sitter matches are repaired and color indices are assigned.
#[derive(Clone, Debug)]
struct BracketMatchCandidate {
    bracket_match: BracketMatch,
    pattern: BracketPatternKey,
    rainbow_exclude: bool,
}

impl BracketMatchCandidate {
    fn open_delimiter(&self) -> BracketDelimiter {
        BracketDelimiter {
            start: self.bracket_match.open_range.start,
            end: self.bracket_match.open_range.end,
            pattern: self.pattern,
        }
    }

    fn close_delimiter(&self) -> BracketDelimiter {
        BracketDelimiter {
            start: self.bracket_match.close_range.start,
            end: self.bracket_match.close_range.end,
            pattern: self.pattern,
        }
    }
}

/// Identifies a brackets query pattern, unique across all grammars of a syntax map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BracketPatternKey {
    grammar_index: usize,
    pattern_index: usize,
}

/// One delimiter (open or close) of a bracket pair candidate.
/// Ordered by buffer position first, so sorting yields document order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct BracketDelimiter {
    start: usize,
    end: usize,
    pattern: BracketPatternKey,
}

type BracketDelimiterPair = (BracketDelimiter, BracketDelimiter);

impl BufferSnapshot {
    /// Finds all [`super::row_chunk::RowChunks`] applicable to the given range, then returns all bracket pairs that intersect with those chunks.
    /// Hence, may return more bracket pairs than the range contains.
    ///
    /// Will omit known chunks.
    /// The resulting bracket match collections are not ordered.
    pub fn fetch_bracket_ranges(
        &self,
        range: Range<usize>,
        known_chunks: Option<&HashSet<Range<BufferRow>>>,
    ) -> HashMap<Range<BufferRow>, Vec<BracketMatch>> {
        let mut all_bracket_matches = HashMap::default();
        if self
            .language
            .as_ref()
            .is_none_or(|language| language.grammar().is_none())
        {
            return all_bracket_matches;
        }

        for chunk in self
            .tree_sitter_data
            .chunks
            .applicable_chunks(&[range.to_point(self)])
        {
            if known_chunks.is_some_and(|chunks| chunks.contains(&chunk.row_range())) {
                continue;
            }
            if let Some(cached_brackets) = self
                .tree_sitter_data
                .brackets_by_chunks
                .lock()
                .get(&chunk.id)
            {
                all_bracket_matches.insert(chunk.row_range(), cached_brackets.clone());
                continue;
            }

            let chunk_range = chunk.anchor_range().to_offset(self);
            let chunk_brackets = self.chunk_bracket_matches(chunk_range);
            self.tree_sitter_data
                .brackets_by_chunks
                .lock()
                .entry(chunk.id)
                .or_insert_with(|| chunk_brackets.clone());
            all_bracket_matches.insert(chunk.row_range(), chunk_brackets);
        }

        all_bracket_matches
    }

    fn chunk_bracket_matches(&self, chunk_range: Range<usize>) -> Vec<BracketMatch> {
        let mut chunk_brackets = ChunkBrackets::new(self, chunk_range);
        chunk_brackets.collect_queried_candidates(self);
        chunk_brackets.recover_error_node_delimiters();
        chunk_brackets.repair_bogus_patterns();
        chunk_brackets.deduplicate_overlapping_patterns();
        chunk_brackets.into_colorized_matches()
    }

    pub fn all_bracket_ranges(&self, range: Range<usize>) -> impl Iterator<Item = BracketMatch> {
        self.fetch_bracket_ranges(range.clone(), None)
            .into_values()
            .flatten()
            .filter(move |bracket_match| {
                let bracket_range = bracket_match.open_range.start..bracket_match.close_range.end;
                bracket_range.overlaps(&range)
            })
    }

    /// Returns bracket range pairs overlapping or adjacent to `range`
    pub fn bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = BracketMatch> + '_ {
        // Find bracket pairs that *inclusively* contain the given range.
        let range = range.start.to_previous_offset(self)..range.end.to_next_offset(self);
        self.all_bracket_ranges(range)
            .filter(|pair| !pair.newline_only)
    }

    /// Returns enclosing bracket ranges containing the given range
    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> impl Iterator<Item = BracketMatch> + '_ {
        let range = range.start.to_offset(self)..range.end.to_offset(self);

        let result: Vec<_> = self.bracket_ranges(range.clone()).collect();
        let max_depth = result
            .iter()
            .map(|mat| mat.syntax_layer_depth)
            .max()
            .unwrap_or(0);
        result.into_iter().filter(move |pair| {
            pair.open_range.start <= range.start
                && pair.close_range.end >= range.end
                && pair.syntax_layer_depth == max_depth
        })
    }

    /// Returns the smallest enclosing bracket ranges containing the given range or None if no brackets contain range
    ///
    /// Can optionally pass a range_filter to filter the ranges of brackets to consider
    pub fn innermost_enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
        range_filter: Option<&dyn Fn(Range<usize>, Range<usize>) -> bool>,
    ) -> Option<(Range<usize>, Range<usize>)> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);

        // Get the ranges of the innermost pair of brackets.
        let mut result: Option<(Range<usize>, Range<usize>)> = None;

        for pair in self.enclosing_bracket_ranges(range) {
            if let Some(range_filter) = range_filter
                && !range_filter(pair.open_range.clone(), pair.close_range.clone())
            {
                continue;
            }

            let len = pair.close_range.end - pair.open_range.start;

            if let Some((existing_open, existing_close)) = &result {
                let existing_len = existing_close.end - existing_open.start;
                if len > existing_len {
                    continue;
                }
            }

            result = Some((pair.open_range, pair.close_range));
        }

        result
    }
}

/// Bracket pair candidates of a single row chunk, in the middle of being repaired.
///
/// Tree-sitter error recovery makes bracket queries unreliable in two ways: a query
/// can match one open delimiter against several closes (bogus patterns), and it can
/// omit delimiters that only exist as concrete leaves inside ERROR nodes (incomplete
/// patterns). Affected patterns are re-paired chunk-locally, while trustworthy query
/// output is kept as is.
struct ChunkBrackets<'a> {
    chunk_range: Range<usize>,
    error_layers: Vec<SyntaxLayer<'a>>,
    grammar_ids: Vec<GrammarId>,
    configs: Vec<&'a BracketsConfig>,
    candidates: Vec<BracketMatchCandidate>,
    delimiter_kinds_by_pattern: HashMap<BracketPatternKey, (HashSet<u16>, HashSet<u16>)>,
    syntax_layer_depths_by_delimiter: HashMap<BracketDelimiter, usize>,
    bogus_patterns: HashSet<BracketPatternKey>,
    incomplete_patterns: HashSet<BracketPatternKey>,
    syntax_opens_by_pattern: HashMap<BracketPatternKey, HashSet<BracketDelimiter>>,
    syntax_closes_by_pattern: HashMap<BracketPatternKey, HashSet<BracketDelimiter>>,
}

impl<'a> ChunkBrackets<'a> {
    fn new(snapshot: &'a BufferSnapshot, chunk_range: Range<usize>) -> Self {
        let error_layers = snapshot
            .syntax
            .layers_for_range(chunk_range.clone(), &snapshot.text, true)
            .filter(|layer| layer.node().has_error())
            .collect::<Vec<_>>();
        Self {
            chunk_range,
            error_layers,
            grammar_ids: Vec::new(),
            configs: Vec::new(),
            candidates: Vec::new(),
            delimiter_kinds_by_pattern: HashMap::default(),
            syntax_layer_depths_by_delimiter: HashMap::default(),
            bogus_patterns: HashSet::default(),
            incomplete_patterns: HashSet::default(),
            syntax_opens_by_pattern: HashMap::default(),
            syntax_closes_by_pattern: HashMap::default(),
        }
    }

    /// Runs the bracket queries and collects pair candidates overlapping the chunk,
    /// marking patterns that pair one open delimiter with several closes as bogus.
    fn collect_queried_candidates(&mut self, snapshot: &'a BufferSnapshot) {
        let bounded_query = (
            self.chunk_range.clone(),
            TreeSitterOptions {
                max_bytes_to_query: Some(MAX_BYTES_TO_QUERY),
                max_start_depth: None,
            },
        );
        // The bounded query drops any pair spanning more than `MAX_BYTES_TO_QUERY`
        // (tree-sitter's containing byte range requires full containment), which
        // resets bracket depth at chunk boundaries. Every such pair encloses a
        // chunk edge, so recover them with two point-sized unbounded queries that
        // only traverse the syntax tree spine around each edge.
        let boundary_queries = [self.chunk_range.start, self.chunk_range.end].map(|offset| {
            (
                snapshot.clip_offset(offset.saturating_sub(1), Bias::Left)
                    ..snapshot.clip_offset(offset.saturating_add(1), Bias::Right),
                TreeSitterOptions::default(),
            )
        });

        let mut seen_pairs = HashSet::default();
        // Track the close seen for each open delimiter: a second, different close
        // for the same open means the pattern's matches cannot be trusted.
        let mut close_by_open = HashMap::default();
        for (query_range, options) in iter::once(bounded_query).chain(boundary_queries) {
            let mut matches = snapshot.syntax.matches_with_options(
                query_range,
                &snapshot.text,
                options,
                |grammar| grammar.brackets_config.as_ref().map(|c| &c.query),
            );
            let grammar_indices = matches
                .grammars()
                .iter()
                .map(|grammar| self.grammar_index(grammar))
                .collect::<Vec<_>>();

            while let Some(mat) = matches.peek() {
                let mut open = None;
                let mut close = None;
                let syntax_layer_depth = mat.depth;
                let grammar_index = grammar_indices[mat.grammar_index];
                let pattern_key = BracketPatternKey {
                    grammar_index,
                    pattern_index: mat.pattern_index,
                };
                let config = self.configs[grammar_index];
                let pattern = &config.patterns[mat.pattern_index];
                for capture in mat.captures {
                    if capture.index == config.open_capture_ix {
                        open = Some((capture.node.byte_range(), capture.node.kind_id()));
                    } else if capture.index == config.close_capture_ix {
                        close = Some((capture.node.byte_range(), capture.node.kind_id()));
                    }
                }

                matches.advance();

                let Some(((open_range, open_kind), (close_range, close_kind))) = open.zip(close)
                else {
                    continue;
                };

                let bracket_range = open_range.start..=close_range.end;
                if !bracket_range.overlaps(&self.chunk_range) {
                    continue;
                }

                let candidate = BracketMatchCandidate {
                    bracket_match: BracketMatch {
                        open_range,
                        close_range,
                        syntax_layer_depth,
                        newline_only: pattern.newline_only,
                        color_index: None,
                    },
                    pattern: pattern_key,
                    rainbow_exclude: pattern.rainbow_exclude,
                };

                if !self.error_layers.is_empty() {
                    let (open_kinds, close_kinds) = self
                        .delimiter_kinds_by_pattern
                        .entry(pattern_key)
                        .or_insert_with(|| (HashSet::default(), HashSet::default()));
                    open_kinds.insert(open_kind);
                    close_kinds.insert(close_kind);
                }

                if !seen_pairs.insert((candidate.open_delimiter(), candidate.close_delimiter())) {
                    continue;
                }

                if close_by_open
                    .insert(candidate.open_delimiter(), candidate.close_delimiter())
                    .is_some()
                {
                    self.bogus_patterns.insert(pattern_key);
                }

                self.candidates.push(candidate);
            }
        }
    }

    fn grammar_index(&mut self, grammar: &'a Grammar) -> usize {
        self.grammar_ids
            .iter()
            .position(|&id| id == grammar.id())
            .unwrap_or_else(|| {
                self.grammar_ids.push(grammar.id());
                self.configs.push(grammar.brackets_config.as_ref().unwrap());
                self.grammar_ids.len() - 1
            })
    }

    /// Bracket queries can omit delimiters swallowed by tree-sitter error recovery.
    /// Collects the concrete delimiter leaves inside ERROR nodes and marks the
    /// patterns whose query output misses some of them as incomplete (and bogus).
    fn recover_error_node_delimiters(&mut self) {
        let unambiguous_kinds = self.unambiguous_delimiter_kinds();
        self.collect_error_layer_delimiters(&unambiguous_kinds);
        self.mark_incomplete_patterns(&unambiguous_kinds);
    }

    /// Patterns whose open and close delimiters map to exactly one distinct
    /// syntax-node kind each, making their leaves recognizable inside ERROR nodes.
    fn unambiguous_delimiter_kinds(&self) -> Vec<(BracketPatternKey, u16, u16)> {
        self.delimiter_kinds_by_pattern
            .iter()
            .filter_map(|(pattern, (open_kinds, close_kinds))| {
                // Kinds are observed from this chunk's matches only: a pattern
                // alternating several delimiter tokens may pass this check when just
                // one alternative occurs in the chunk, and a pattern whose every
                // match was swallowed by an error is never recovered at all.
                if open_kinds.len() != 1 || close_kinds.len() != 1 {
                    return None;
                }
                let open_kind = *open_kinds.iter().next()?;
                let close_kind = *close_kinds.iter().next()?;
                // Syntax leaves cannot distinguish the two roles of delimiters such as quotes.
                (open_kind != close_kind).then_some((*pattern, open_kind, close_kind))
            })
            .collect()
    }

    fn collect_error_layer_delimiters(
        &mut self,
        unambiguous_kinds: &[(BracketPatternKey, u16, u16)],
    ) {
        if unambiguous_kinds.is_empty() {
            return;
        }
        for layer in &self.error_layers {
            let Some(grammar) = layer.language.grammar() else {
                continue;
            };
            let Some(grammar_index) = self.grammar_ids.iter().position(|&id| id == grammar.id())
            else {
                continue;
            };
            let relevant_patterns = unambiguous_kinds
                .iter()
                .filter(|(pattern, _, _)| pattern.grammar_index == grammar_index)
                .collect::<Vec<_>>();
            if relevant_patterns.is_empty() {
                continue;
            }

            let root_node = layer.node();
            let mut cursor = root_node.walk();
            let mut nodes = vec![(root_node, false)];
            while let Some((node, inside_error)) = nodes.pop() {
                let node_range = node.byte_range();
                if node_range.end <= self.chunk_range.start
                    || node_range.start >= self.chunk_range.end
                {
                    continue;
                }
                let inside_error = inside_error || node.is_error();
                if !inside_error && !node.has_error() {
                    continue;
                }

                if node.child_count() == 0 {
                    if !inside_error
                        || node_range.is_empty()
                        || !self.chunk_range.contains(&node_range.start)
                    {
                        continue;
                    }

                    for (pattern, open_kind, close_kind) in &relevant_patterns {
                        let delimiters_by_pattern = if node.kind_id() == *open_kind {
                            &mut self.syntax_opens_by_pattern
                        } else if node.kind_id() == *close_kind {
                            &mut self.syntax_closes_by_pattern
                        } else {
                            continue;
                        };
                        let delimiter = BracketDelimiter {
                            start: node_range.start,
                            end: node_range.end,
                            pattern: *pattern,
                        };
                        delimiters_by_pattern
                            .entry(*pattern)
                            .or_default()
                            .insert(delimiter);
                        self.syntax_layer_depths_by_delimiter
                            .entry(delimiter)
                            .or_insert(layer.depth);
                    }
                } else {
                    nodes.extend(
                        node.children(&mut cursor)
                            .map(|child| (child, inside_error)),
                    );
                }
            }
        }
    }

    fn mark_incomplete_patterns(&mut self, unambiguous_kinds: &[(BracketPatternKey, u16, u16)]) {
        for (pattern, _, _) in unambiguous_kinds {
            // Recovery needs both delimiter roles inside this chunk's ERROR nodes;
            // a pair split between an ERROR node and a regular node is left as queried.
            let Some(syntax_opens) = self.syntax_opens_by_pattern.get(pattern) else {
                continue;
            };
            let Some(syntax_closes) = self.syntax_closes_by_pattern.get(pattern) else {
                continue;
            };
            // A count mismatch means some delimiter genuinely lacks a partner while the
            // user is editing; repairing would invent one. Balanced counts can still pair
            // unrelated stray delimiters, which is accepted as best-effort recovery.
            if syntax_opens.len() != syntax_closes.len() {
                continue;
            }

            let query_opens = self
                .candidates
                .iter()
                .filter(|candidate| candidate.pattern == *pattern)
                .map(|candidate| candidate.open_delimiter())
                .filter(|open| self.chunk_range.contains(&open.start))
                .collect::<HashSet<_>>();
            let query_closes = self
                .candidates
                .iter()
                .filter(|candidate| candidate.pattern == *pattern)
                .map(|candidate| candidate.close_delimiter())
                .filter(|close| self.chunk_range.contains(&close.start))
                .collect::<HashSet<_>>();
            let has_missing_open = !syntax_opens.is_subset(&query_opens);
            let has_missing_close = !syntax_closes.is_subset(&query_closes);
            if has_missing_open || has_missing_close {
                self.bogus_patterns.insert(*pattern);
                self.incomplete_patterns.insert(*pattern);
            }
        }
    }

    /// Certain patterns produce bogus or incomplete matches during error recovery.
    /// Repairs only those patterns, keeping trustworthy grammar output intact:
    /// clean patterns may legitimately pair an open in one chunk with a close in
    /// another, and must not be dropped by the chunk-local repair.
    fn repair_bogus_patterns(&mut self) {
        if self.bogus_patterns.is_empty() {
            return;
        }

        for candidate in bogus_candidates(&self.candidates, &self.bogus_patterns) {
            let syntax_layer_depth = candidate.bracket_match.syntax_layer_depth;
            self.syntax_layer_depths_by_delimiter
                .entry(candidate.open_delimiter())
                .or_insert(syntax_layer_depth);
            self.syntax_layer_depths_by_delimiter
                .entry(candidate.close_delimiter())
                .or_insert(syntax_layer_depth);
        }

        let pair_counts = self.bogus_pair_counts();
        let mut existing_pairs = bogus_candidates(&self.candidates, &self.bogus_patterns)
            .map(|candidate| (candidate.open_delimiter(), candidate.close_delimiter()))
            .collect::<HashSet<_>>();
        let mut valid_pairs = self.stack_matched_pairs(&pair_counts, &mut existing_pairs);
        self.trust_cross_chunk_pairs(&pair_counts, &mut valid_pairs);
        self.replace_bogus_pairs(valid_pairs, existing_pairs);
    }

    /// A cross-product of bogus matches multiply pairs its delimiters, while
    /// structural error-recovery pairs stay uniquely paired and can be trusted.
    fn bogus_pair_counts(&self) -> DelimiterPairCounts {
        let mut counts = DelimiterPairCounts::default();
        for candidate in bogus_candidates(&self.candidates, &self.bogus_patterns) {
            *counts
                .opens
                .entry(candidate.open_delimiter())
                .or_insert(0_usize) += 1;
            *counts
                .closes
                .entry(candidate.close_delimiter())
                .or_insert(0_usize) += 1;
        }
        counts
    }

    // Tree-sitter represents missing error-recovery tokens as empty nodes.
    fn is_concrete(&self, delimiter: &BracketDelimiter) -> bool {
        delimiter.start < delimiter.end || !self.incomplete_patterns.contains(&delimiter.pattern)
    }

    /// The concrete delimiters of bogus patterns that lie within the chunk, augmented
    /// with the ones recovered from ERROR nodes, sorted in document order.
    fn repairable_delimiters(
        &self,
        mut delimiters: Vec<BracketDelimiter>,
        syntax_delimiters_by_pattern: &HashMap<BracketPatternKey, HashSet<BracketDelimiter>>,
    ) -> Vec<BracketDelimiter> {
        delimiters.retain(|delimiter| {
            self.chunk_range.contains(&delimiter.start) && self.is_concrete(delimiter)
        });
        for pattern in &self.incomplete_patterns {
            if let Some(syntax_delimiters) = syntax_delimiters_by_pattern.get(pattern) {
                delimiters.extend(syntax_delimiters.iter().copied());
            }
        }
        delimiters.sort_unstable();
        delimiters.dedup();
        delimiters
    }

    /// Re-pairs the chunk-local delimiters of bogus patterns with a stack walk in
    /// document order. A close without any open on its stack gets one inferred right
    /// before it, unless the grammar paired it with an open beyond the chunk.
    fn stack_matched_pairs(
        &mut self,
        pair_counts: &DelimiterPairCounts,
        existing_pairs: &mut HashSet<BracketDelimiterPair>,
    ) -> HashSet<BracketDelimiterPair> {
        // Map each close to its expected open length (for inferring opens)
        let close_to_open_len = bogus_candidates(&self.candidates, &self.bogus_patterns)
            .map(|candidate| {
                (
                    candidate.close_delimiter(),
                    candidate.bracket_match.open_range.len(),
                )
            })
            .collect::<HashMap<_, _>>();
        let sorted_opens = self.repairable_delimiters(
            bogus_candidates(&self.candidates, &self.bogus_patterns)
                .map(|candidate| candidate.open_delimiter())
                .collect(),
            &self.syntax_opens_by_pattern,
        );
        let unique_closes = self.repairable_delimiters(
            bogus_candidates(&self.candidates, &self.bogus_patterns)
                .map(|candidate| candidate.close_delimiter())
                .collect(),
            &self.syntax_closes_by_pattern,
        );
        let closes_with_cross_chunk_opens =
            bogus_candidates(&self.candidates, &self.bogus_patterns)
                .filter(|candidate| {
                    !self
                        .chunk_range
                        .contains(&candidate.bracket_match.open_range.start)
                        && pair_counts.uniquely_paired(
                            &candidate.open_delimiter(),
                            &candidate.close_delimiter(),
                        )
                })
                .map(|candidate| candidate.close_delimiter())
                .collect::<HashSet<_>>();

        let mut valid_pairs = HashSet::default();
        let mut open_stacks = HashMap::default();
        let mut open_idx = 0;
        for close in &unique_closes {
            // Push all opens before this close onto stack
            while open_idx < sorted_opens.len() && sorted_opens[open_idx].start < close.start {
                let open = sorted_opens[open_idx];
                open_stacks
                    .entry(open.pattern)
                    .or_insert_with(Vec::new)
                    .push(open);
                open_idx += 1;
            }

            // Try to match with most recent open
            if let Some(open) = open_stacks
                .get_mut(&close.pattern)
                .and_then(|open_stack| open_stack.pop())
            {
                valid_pairs.insert((open, *close));
            } else if !closes_with_cross_chunk_opens.contains(close)
                && let Some(&open_len) = close_to_open_len.get(close)
            {
                // No open on stack and the grammar offers no open beyond the
                // chunk either - infer one based on expected open_len
                if close.start >= open_len {
                    let inferred = BracketDelimiter {
                        start: close.start - open_len,
                        end: close.start,
                        pattern: close.pattern,
                    };
                    valid_pairs.insert((inferred, *close));
                    existing_pairs.insert((inferred, *close));
                    let syntax_layer_depth = self
                        .syntax_layer_depths_by_delimiter
                        .get(close)
                        .copied()
                        .unwrap_or(0);
                    self.syntax_layer_depths_by_delimiter
                        .entry(inferred)
                        .or_insert(syntax_layer_depth);
                    let pattern = &self.configs[close.pattern.grammar_index].patterns
                        [close.pattern.pattern_index];
                    self.candidates.push(BracketMatchCandidate {
                        bracket_match: BracketMatch {
                            open_range: inferred.start..inferred.end,
                            close_range: close.start..close.end,
                            newline_only: pattern.newline_only,
                            syntax_layer_depth,
                            color_index: None,
                        },
                        pattern: close.pattern,
                        rainbow_exclude: pattern.rainbow_exclude,
                    });
                }
            }
        }
        valid_pairs
    }

    /// Pairs with a delimiter outside the chunk (boundary queries recover
    /// those to keep bracket depth stable across chunks) cannot be verified
    /// by the chunk-local stack: the delimiters beyond the chunk are unknown.
    /// Trust grammar output for whichever remained unmatched, but only
    /// uniquely paired ones: cross-product bogosity multiply pairs delimiters.
    fn trust_cross_chunk_pairs(
        &self,
        pair_counts: &DelimiterPairCounts,
        valid_pairs: &mut HashSet<BracketDelimiterPair>,
    ) {
        let matched_opens = valid_pairs
            .iter()
            .map(|(open, _)| *open)
            .collect::<HashSet<_>>();
        let matched_closes = valid_pairs
            .iter()
            .map(|(_, close)| *close)
            .collect::<HashSet<_>>();
        for candidate in bogus_candidates(&self.candidates, &self.bogus_patterns) {
            let open = candidate.open_delimiter();
            let close = candidate.close_delimiter();
            let open_inside = self.chunk_range.contains(&open.start);
            let close_inside = self.chunk_range.contains(&close.start);
            let crosses_chunk = !open_inside || !close_inside;
            let unmatched = (!open_inside || !matched_opens.contains(&open))
                && (!close_inside || !matched_closes.contains(&close));
            if crosses_chunk
                && unmatched
                && pair_counts.uniquely_paired(&open, &close)
                && self.is_concrete(&open)
                && self.is_concrete(&close)
            {
                valid_pairs.insert((open, close));
            }
        }
    }

    /// Drops the bogus candidates that did not survive the repair and adds
    /// candidates for the repaired pairs that the queries never produced.
    fn replace_bogus_pairs(
        &mut self,
        valid_pairs: HashSet<BracketDelimiterPair>,
        existing_pairs: HashSet<BracketDelimiterPair>,
    ) {
        let mut repaired_pairs = valid_pairs
            .iter()
            .filter(|pair| !existing_pairs.contains(*pair))
            .copied()
            .collect::<Vec<_>>();
        repaired_pairs.sort_unstable();
        for (open, close) in repaired_pairs {
            let pattern =
                &self.configs[open.pattern.grammar_index].patterns[open.pattern.pattern_index];
            self.candidates.push(BracketMatchCandidate {
                bracket_match: BracketMatch {
                    open_range: open.start..open.end,
                    close_range: close.start..close.end,
                    newline_only: pattern.newline_only,
                    syntax_layer_depth: self
                        .syntax_layer_depths_by_delimiter
                        .get(&open)
                        .copied()
                        .or_else(|| self.syntax_layer_depths_by_delimiter.get(&close).copied())
                        .unwrap_or(0),
                    color_index: None,
                },
                pattern: open.pattern,
                rainbow_exclude: pattern.rainbow_exclude,
            });
        }

        let bogus_patterns = &self.bogus_patterns;
        self.candidates.retain(|candidate| {
            !bogus_patterns.contains(&candidate.pattern)
                || valid_pairs.contains(&(candidate.open_delimiter(), candidate.close_delimiter()))
        });
    }

    /// Overlapping query patterns can attribute one physical pair to several
    /// patterns, both when queried and when repaired. Keeps the first match,
    /// following the pattern order in the query.
    fn deduplicate_overlapping_patterns(&mut self) {
        let mut seen_ranges = HashSet::default();
        self.candidates.retain(|candidate| {
            seen_ranges.insert((
                candidate.bracket_match.open_range.start,
                candidate.bracket_match.open_range.end,
                candidate.bracket_match.close_range.start,
                candidate.bracket_match.close_range.end,
            ))
        });
    }

    /// Assigns nesting-depth color indices and returns the matches sorted by open range.
    fn into_colorized_matches(self) -> Vec<BracketMatch> {
        let mut opens = Vec::new();
        let mut color_pairs = Vec::new();
        let mut all_brackets = self
            .candidates
            .into_iter()
            .enumerate()
            .map(|(index, candidate)| {
                let bracket_match = candidate.bracket_match;
                // Certain languages have "brackets" that are not brackets, e.g. tags. and such
                // bracket will match the entire tag with all text inside.
                // For now, avoid highlighting any pair that has more than single char in each bracket.
                // We need to  colorize `<Element/>` bracket pairs, so cannot make this check stricter.
                let should_color = !candidate.rainbow_exclude
                    && (bracket_match.open_range.len() == 1
                        || bracket_match.close_range.len() == 1);
                if should_color {
                    opens.push(bracket_match.open_range.clone());
                    color_pairs.push((
                        bracket_match.open_range.clone(),
                        bracket_match.close_range.clone(),
                        index,
                    ));
                }
                bracket_match
            })
            .collect::<Vec<_>>();

        opens.sort_by_key(|r| (r.start, r.end));
        opens.dedup_by(|a, b| a.start == b.start && a.end == b.end);
        color_pairs.sort_by_key(|(_, close, _)| close.end);

        let mut open_stack = Vec::new();
        let mut open_index = 0;
        for (open, close, index) in color_pairs {
            while open_index < opens.len() && opens[open_index].start < close.start {
                open_stack.push(opens[open_index].clone());
                open_index += 1;
            }

            if open_stack.last() == Some(&open) {
                let depth_index = open_stack.len() - 1;
                all_brackets[index].color_index = Some(depth_index);
                open_stack.pop();
            }
        }

        all_brackets.sort_by_key(|bracket_match| {
            (bracket_match.open_range.start, bracket_match.open_range.end)
        });

        all_brackets
    }
}

fn bogus_candidates<'a>(
    candidates: &'a [BracketMatchCandidate],
    bogus_patterns: &'a HashSet<BracketPatternKey>,
) -> impl Iterator<Item = &'a BracketMatchCandidate> {
    candidates
        .iter()
        .filter(|candidate| bogus_patterns.contains(&candidate.pattern))
}

#[derive(Default)]
struct DelimiterPairCounts {
    opens: HashMap<BracketDelimiter, usize>,
    closes: HashMap<BracketDelimiter, usize>,
}

impl DelimiterPairCounts {
    fn uniquely_paired(&self, open: &BracketDelimiter, close: &BracketDelimiter) -> bool {
        self.opens.get(open) == Some(&1) && self.closes.get(close) == Some(&1)
    }
}
