use gpui::{App, Context, Entity, EventEmitter, SharedString};
use language::line_diff;
use regex::Regex;
use std::{cmp::Ordering, ops::Range, sync::Arc};
use text::{Anchor, BufferId, OffsetRangeExt as _};

/// A compiled Auto-Resolve regex pattern. Built by callers from user settings
/// and passed into [`ConflictSetSnapshot::auto_resolution_edits`] /
/// [`ConflictRegion::decompose`] so the decomposition algorithm can collapse
/// sub-conflicts whose both sides match the same pattern.
#[derive(Debug, Clone)]
pub struct AutoResolvePattern {
    pub regex: Regex,
    pub take: AutoResolveTakeSide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoResolveTakeSide {
    Ours,
    Theirs,
}

pub struct ConflictSet {
    pub has_conflict: bool,
    pub snapshot: ConflictSetSnapshot,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConflictSetUpdate {
    pub buffer_range: Option<Range<Anchor>>,
    pub old_range: Range<usize>,
    pub new_range: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct ConflictSetSnapshot {
    pub buffer_id: BufferId,
    pub conflicts: Arc<[ConflictRegion]>,
}

impl ConflictSetSnapshot {
    pub fn conflicts_in_range(
        &self,
        range: Range<Anchor>,
        buffer: &text::BufferSnapshot,
    ) -> &[ConflictRegion] {
        let start_ix = self
            .conflicts
            .binary_search_by(|conflict| {
                conflict
                    .range
                    .end
                    .cmp(&range.start, buffer)
                    .then(Ordering::Greater)
            })
            .unwrap_err();
        let end_ix = start_ix
            + self.conflicts[start_ix..]
                .binary_search_by(|conflict| {
                    conflict
                        .range
                        .start
                        .cmp(&range.end, buffer)
                        .then(Ordering::Less)
                })
                .unwrap_err();
        &self.conflicts[start_ix..end_ix]
    }

    pub fn auto_resolvable<'a>(
        &'a self,
        buffer: &'a text::BufferSnapshot,
    ) -> impl Iterator<Item = (&'a ConflictRegion, AutoResolution)> + 'a {
        self.conflicts
            .iter()
            .filter_map(move |conflict| conflict.auto_resolution(buffer).map(|r| (conflict, r)))
    }

    pub fn auto_resolution_edits(
        &self,
        buffer: &text::BufferSnapshot,
        patterns: &[AutoResolvePattern],
        structural: Option<&super::structural_merge::LanguageMergeContext>,
    ) -> Vec<(Range<usize>, String)> {
        let mut edits = Vec::new();
        for conflict in self.conflicts.iter() {
            if let Some(structural) = structural {
                if let Some(replacement) =
                    structural.try_merge_region(conflict).resolved_text()
                {
                    let outer = conflict.range.to_offset(buffer);
                    edits.push((outer, replacement.to_string()));
                    continue;
                }
            }
            let Some(segments) = conflict.decompose(buffer, patterns) else {
                continue;
            };
            if !segments
                .iter()
                .any(|segment| matches!(segment, DecompositionSegment::Resolved(_)))
            {
                continue;
            }
            let outer = conflict.range.to_offset(buffer);
            let replacement = render_decomposed_region(
                &segments,
                &conflict.ours_branch_name,
                &conflict.theirs_branch_name,
            );
            edits.push((outer, replacement));
        }
        edits
    }

    /// Iterator over every conflict region that Auto-Resolve will change,
    /// alongside whether the change leaves any markers in place.
    pub fn decomposition_summary<'a>(
        &'a self,
        buffer: &'a text::BufferSnapshot,
        patterns: &'a [AutoResolvePattern],
        structural: Option<&'a super::structural_merge::LanguageMergeContext>,
    ) -> impl Iterator<Item = (&'a ConflictRegion, RegionSummary)> + 'a {
        self.conflicts.iter().filter_map(move |conflict| {
            if let Some(structural) = structural
                && structural.try_merge_region(conflict).is_resolved()
            {
                return Some((
                    conflict,
                    RegionSummary {
                        is_improvement: true,
                        fully_resolved: true,
                    },
                ));
            }
            let segments = conflict.decompose(buffer, patterns)?;
            let summary = RegionSummary::from_segments(&segments);
            summary.is_improvement.then_some((conflict, summary))
        })
    }

    pub fn compare(&self, other: &Self, buffer: &text::BufferSnapshot) -> ConflictSetUpdate {
        let common_prefix_len = self
            .conflicts
            .iter()
            .zip(other.conflicts.iter())
            .take_while(|(old, new)| old == new)
            .count();
        let common_suffix_len = self.conflicts[common_prefix_len..]
            .iter()
            .rev()
            .zip(other.conflicts[common_prefix_len..].iter().rev())
            .take_while(|(old, new)| old == new)
            .count();
        let old_conflicts =
            &self.conflicts[common_prefix_len..(self.conflicts.len() - common_suffix_len)];
        let new_conflicts =
            &other.conflicts[common_prefix_len..(other.conflicts.len() - common_suffix_len)];
        let old_range = common_prefix_len..(common_prefix_len + old_conflicts.len());
        let new_range = common_prefix_len..(common_prefix_len + new_conflicts.len());
        let start = match (old_conflicts.first(), new_conflicts.first()) {
            (None, None) => None,
            (None, Some(conflict)) => Some(conflict.range.start),
            (Some(conflict), None) => Some(conflict.range.start),
            (Some(first), Some(second)) => {
                Some(*first.range.start.min(&second.range.start, buffer))
            }
        };
        let end = match (old_conflicts.last(), new_conflicts.last()) {
            (None, None) => None,
            (None, Some(conflict)) => Some(conflict.range.end),
            (Some(first), None) => Some(first.range.end),
            (Some(first), Some(second)) => Some(*first.range.end.max(&second.range.end, buffer)),
        };
        ConflictSetUpdate {
            buffer_range: start.zip(end).map(|(start, end)| start..end),
            old_range,
            new_range,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConflictRegion {
    pub ours_branch_name: SharedString,
    pub theirs_branch_name: SharedString,
    pub range: Range<Anchor>,
    pub ours: Range<Anchor>,
    pub theirs: Range<Anchor>,
    pub base: Option<Range<Anchor>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoResolution {
    TakeOurs,
    TakeTheirs,
    Identical,
}

/// A single piece of a conflict region after line-level three-way
/// decomposition: either a span that has been auto-merged, or a smaller
/// sub-conflict that still needs manual resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecompositionSegment {
    Resolved(String),
    Conflict {
        base: String,
        ours: String,
        theirs: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionSummary {
    pub is_improvement: bool,
    pub fully_resolved: bool,
}

impl RegionSummary {
    pub fn from_segments(segments: &[DecompositionSegment]) -> Self {
        let any_resolved = segments
            .iter()
            .any(|s| matches!(s, DecompositionSegment::Resolved(_)));
        let any_conflict = segments
            .iter()
            .any(|s| matches!(s, DecompositionSegment::Conflict { .. }));
        Self {
            is_improvement: any_resolved,
            fully_resolved: !any_conflict,
        }
    }
}

impl ConflictRegion {
    pub fn auto_resolution(&self, buffer: &text::BufferSnapshot) -> Option<AutoResolution> {
        let base_range = self.base.as_ref()?;
        let base_text = buffer.text_for_range(base_range.clone()).collect::<String>();
        let ours_text = buffer.text_for_range(self.ours.clone()).collect::<String>();
        let theirs_text = buffer.text_for_range(self.theirs.clone()).collect::<String>();

        if ours_text == theirs_text {
            Some(AutoResolution::Identical)
        } else if ours_text == base_text {
            Some(AutoResolution::TakeTheirs)
        } else if theirs_text == base_text {
            Some(AutoResolution::TakeOurs)
        } else {
            None
        }
    }

    /// Decompose this region into a sequence of resolved and unresolved
    /// segments by re-diffing `ours` and `theirs` against `base` at line
    /// granularity. `patterns` are applied to remaining single-line
    /// sub-conflicts so user-configured regex rules (e.g. version strings)
    /// can collapse them too. Returns `None` if no diff3 base is present.
    pub fn decompose(
        &self,
        buffer: &text::BufferSnapshot,
        patterns: &[AutoResolvePattern],
    ) -> Option<Vec<DecompositionSegment>> {
        let base_range = self.base.as_ref()?;
        let base_text = buffer.text_for_range(base_range.clone()).collect::<String>();
        let ours_text = buffer.text_for_range(self.ours.clone()).collect::<String>();
        let theirs_text = buffer.text_for_range(self.theirs.clone()).collect::<String>();
        Some(decompose_three_way(
            &base_text,
            &ours_text,
            &theirs_text,
            patterns,
        ))
    }

    pub fn resolution_edits(
        &self,
        kept_ranges: &[Range<Anchor>],
        buffer: &text::BufferSnapshot,
    ) -> Vec<(Range<usize>, &'static str)> {
        let mut deletions = Vec::new();
        let outer_range = self.range.to_offset(buffer);
        let mut offset = outer_range.start;
        for kept_range in kept_ranges {
            let kept_range = kept_range.to_offset(buffer);
            if kept_range.start > offset {
                deletions.push((offset..kept_range.start, ""));
            }
            offset = kept_range.end;
        }
        if outer_range.end > offset {
            deletions.push((offset..outer_range.end, ""));
        }
        deletions
    }

    pub fn resolve(
        &self,
        buffer: Entity<language::Buffer>,
        ranges: &[Range<Anchor>],
        cx: &mut App,
    ) {
        let edits = {
            let buffer_snapshot = buffer.read(cx).snapshot();
            self.resolution_edits(ranges, &buffer_snapshot)
        };
        buffer.update(cx, |buffer, cx| {
            buffer.edit(edits, None, cx);
        });
    }
}

impl ConflictSet {
    pub fn new(buffer_id: BufferId, has_conflict: bool, _: &mut Context<Self>) -> Self {
        Self {
            has_conflict,
            snapshot: ConflictSetSnapshot {
                buffer_id,
                conflicts: Default::default(),
            },
        }
    }

    pub fn set_has_conflict(&mut self, has_conflict: bool, cx: &mut Context<Self>) -> bool {
        if has_conflict != self.has_conflict {
            self.has_conflict = has_conflict;
            if !self.has_conflict {
                cx.emit(ConflictSetUpdate {
                    buffer_range: None,
                    old_range: 0..self.snapshot.conflicts.len(),
                    new_range: 0..0,
                });
                self.snapshot.conflicts = Default::default();
            }
            true
        } else {
            false
        }
    }

    pub fn snapshot(&self) -> ConflictSetSnapshot {
        self.snapshot.clone()
    }

    pub fn set_snapshot(
        &mut self,
        snapshot: ConflictSetSnapshot,
        update: ConflictSetUpdate,
        cx: &mut Context<Self>,
    ) {
        self.snapshot = snapshot;
        cx.emit(update);
    }

    pub fn parse(buffer: &text::BufferSnapshot) -> ConflictSetSnapshot {
        let mut conflicts = Vec::new();

        let mut line_pos = 0;
        let buffer_len = buffer.len();
        let mut lines = buffer.text_for_range(0..buffer_len).lines();

        let mut conflict_start: Option<usize> = None;
        let mut ours_start: Option<usize> = None;
        let mut ours_end: Option<usize> = None;
        let mut ours_branch_name: Option<SharedString> = None;
        let mut base_start: Option<usize> = None;
        let mut base_end: Option<usize> = None;
        let mut theirs_start: Option<usize> = None;
        let mut theirs_branch_name: Option<SharedString> = None;

        while let Some(line) = lines.next() {
            let line_end = line_pos + line.len();

            if let Some(branch_name) = line.strip_prefix("<<<<<<< ") {
                // If we see a new conflict marker while already parsing one,
                // abandon the previous one and start a new one
                conflict_start = Some(line_pos);
                ours_start = Some(line_end + 1);

                let branch_name = branch_name.trim();
                if !branch_name.is_empty() {
                    ours_branch_name = Some(SharedString::new(branch_name));
                }
            } else if line.starts_with("||||||| ")
                && conflict_start.is_some()
                && ours_start.is_some()
            {
                ours_end = Some(line_pos);
                base_start = Some(line_end + 1);
            } else if line.starts_with("=======")
                && conflict_start.is_some()
                && ours_start.is_some()
            {
                // Set ours_end if not already set (would be set if we have base markers)
                if ours_end.is_none() {
                    ours_end = Some(line_pos);
                } else if base_start.is_some() {
                    base_end = Some(line_pos);
                }
                theirs_start = Some(line_end + 1);
            } else if let Some(branch_name) = line.strip_prefix(">>>>>>> ")
                && conflict_start.is_some()
                && ours_start.is_some()
                && ours_end.is_some()
                && theirs_start.is_some()
            {
                let branch_name = branch_name.trim();
                if !branch_name.is_empty() {
                    theirs_branch_name = Some(SharedString::new(branch_name));
                }

                let theirs_end = line_pos;
                let conflict_end = (line_end + 1).min(buffer_len);

                let range = buffer.anchor_after(conflict_start.unwrap())
                    ..buffer.anchor_before(conflict_end);
                let ours = buffer.anchor_after(ours_start.unwrap())
                    ..buffer.anchor_before(ours_end.unwrap());
                let theirs =
                    buffer.anchor_after(theirs_start.unwrap())..buffer.anchor_before(theirs_end);

                let base = base_start
                    .zip(base_end)
                    .map(|(start, end)| buffer.anchor_after(start)..buffer.anchor_before(end));

                conflicts.push(ConflictRegion {
                    ours_branch_name: ours_branch_name
                        .take()
                        .unwrap_or_else(|| SharedString::new_static("HEAD")),
                    theirs_branch_name: theirs_branch_name
                        .take()
                        .unwrap_or_else(|| SharedString::new_static("Origin")),
                    range,
                    ours,
                    theirs,
                    base,
                });

                conflict_start = None;
                ours_start = None;
                ours_end = None;
                base_start = None;
                base_end = None;
                theirs_start = None;
            }

            line_pos = line_end + 1;
        }

        ConflictSetSnapshot {
            conflicts: conflicts.into(),
            buffer_id: buffer.remote_id(),
        }
    }
}

impl EventEmitter<ConflictSetUpdate> for ConflictSet {}

fn split_lines(text: &str) -> Vec<&str> {
    text.split_inclusive('\n').collect()
}

/// Walk the line-level diffs of `ours` and `theirs` against `base` to produce
/// a sequence of decomposition segments. Adjacent or overlapping hunks from
/// both sides are clustered together: a cluster touched by only one side is
/// auto-resolved to that side's text, clusters where both sides produce
/// identical text are auto-resolved, and clusters where the sides diverge are
/// emitted as smaller sub-conflicts.
fn decompose_three_way(
    base: &str,
    ours: &str,
    theirs: &str,
    patterns: &[AutoResolvePattern],
) -> Vec<DecompositionSegment> {
    let base_lines = split_lines(base);
    let ours_lines = split_lines(ours);
    let theirs_lines = split_lines(theirs);

    let ours_hunks = line_diff(base, ours);
    let theirs_hunks = line_diff(base, theirs);

    let mut segments = Vec::new();
    let mut o_idx = 0;
    let mut t_idx = 0;
    let mut base_cursor: usize = 0;

    loop {
        let next_o = ours_hunks.get(o_idx).map(|(b, _)| b.start as usize);
        let next_t = theirs_hunks.get(t_idx).map(|(b, _)| b.start as usize);

        let cluster_start = match (next_o, next_t) {
            (None, None) => {
                if base_cursor < base_lines.len() {
                    let text: String = base_lines[base_cursor..].concat();
                    if !text.is_empty() {
                        push_resolved(&mut segments, text);
                    }
                }
                return segments;
            }
            (Some(o), None) => o,
            (None, Some(t)) => t,
            (Some(o), Some(t)) => o.min(t),
        };

        if cluster_start > base_cursor {
            let text: String = base_lines[base_cursor..cluster_start].concat();
            if !text.is_empty() {
                push_resolved(&mut segments, text);
            }
            base_cursor = cluster_start;
        }

        let mut cluster_end = base_cursor;
        let (cluster_o, cluster_t) =
            gather_hunk_cluster(&ours_hunks, &theirs_hunks, &mut o_idx, &mut t_idx, &mut cluster_end);

        let base_segment: String = base_lines[base_cursor..cluster_end].concat();
        let ours_segment = compose_replacement(
            &base_lines,
            &ours_lines,
            &ours_hunks,
            &cluster_o,
            base_cursor,
            cluster_end,
        );
        let theirs_segment = compose_replacement(
            &base_lines,
            &theirs_lines,
            &theirs_hunks,
            &cluster_t,
            base_cursor,
            cluster_end,
        );

        let segment = if cluster_o.is_empty() {
            DecompositionSegment::Resolved(theirs_segment)
        } else if cluster_t.is_empty() {
            DecompositionSegment::Resolved(ours_segment)
        } else if ours_segment == theirs_segment {
            DecompositionSegment::Resolved(ours_segment)
        } else if let Some(side) = pattern_match_resolution(&ours_segment, &theirs_segment, patterns)
        {
            match side {
                AutoResolveTakeSide::Ours => DecompositionSegment::Resolved(ours_segment),
                AutoResolveTakeSide::Theirs => DecompositionSegment::Resolved(theirs_segment),
            }
        } else {
            DecompositionSegment::Conflict {
                base: base_segment,
                ours: ours_segment,
                theirs: theirs_segment,
            }
        };
        match segment {
            DecompositionSegment::Resolved(text) => push_resolved(&mut segments, text),
            other => segments.push(other),
        }
        base_cursor = cluster_end;
    }
}

fn push_resolved(segments: &mut Vec<DecompositionSegment>, text: String) {
    if let Some(DecompositionSegment::Resolved(prev)) = segments.last_mut() {
        prev.push_str(&text);
    } else {
        segments.push(DecompositionSegment::Resolved(text));
    }
}

fn compose_replacement(
    base_lines: &[&str],
    side_lines: &[&str],
    hunks: &[(Range<u32>, Range<u32>)],
    cluster_hunk_indices: &[usize],
    cluster_start: usize,
    cluster_end: usize,
) -> String {
    let mut result = String::new();
    let mut cursor = cluster_start;
    for &i in cluster_hunk_indices {
        let (base_range, side_range) = &hunks[i];
        let base_start = base_range.start as usize;
        let base_end = base_range.end as usize;
        if base_start > cursor {
            for line in &base_lines[cursor..base_start] {
                result.push_str(line);
            }
        }
        for line in &side_lines[side_range.start as usize..side_range.end as usize] {
            result.push_str(line);
        }
        cursor = base_end;
    }
    if cursor < cluster_end {
        for line in &base_lines[cursor..cluster_end] {
            result.push_str(line);
        }
    }
    result
}

/// Render the decomposed region back into source: resolved segments are
/// emitted verbatim, sub-conflicts are wrapped in `<<<<<<< / ||||||| /
/// ======= / >>>>>>> ` markers so the result is still a parseable diff3
/// conflict region (with smaller markers) that Auto-Resolve can be re-run
/// against later if needed.
pub fn render_decomposed_region(
    segments: &[DecompositionSegment],
    ours_branch: &str,
    theirs_branch: &str,
) -> String {
    let mut result = String::new();
    for segment in segments {
        match segment {
            DecompositionSegment::Resolved(text) => {
                result.push_str(text);
                ensure_trailing_newline(&mut result);
            }
            DecompositionSegment::Conflict {
                base,
                ours,
                theirs,
            } => {
                result.push_str("<<<<<<< ");
                result.push_str(ours_branch);
                result.push('\n');
                result.push_str(ours);
                ensure_trailing_newline(&mut result);
                result.push_str("||||||| base\n");
                result.push_str(base);
                ensure_trailing_newline(&mut result);
                result.push_str("=======\n");
                result.push_str(theirs);
                ensure_trailing_newline(&mut result);
                result.push_str(">>>>>>> ");
                result.push_str(theirs_branch);
                result.push('\n');
            }
        }
    }
    result
}

fn ensure_trailing_newline(text: &mut String) {
    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
}

/// Apply Auto-Resolve regex patterns to a single sub-conflict. The rule fires
/// only when both sides are exactly one line and both lines match the same
/// pattern, so multi-line edits are never silently picked one way or another.
fn pattern_match_resolution(
    ours: &str,
    theirs: &str,
    patterns: &[AutoResolvePattern],
) -> Option<AutoResolveTakeSide> {
    if patterns.is_empty() {
        return None;
    }
    let ours_line = single_line(ours)?;
    let theirs_line = single_line(theirs)?;
    for pattern in patterns {
        if pattern.regex.is_match(ours_line) && pattern.regex.is_match(theirs_line) {
            return Some(pattern.take);
        }
    }
    None
}

fn single_line(text: &str) -> Option<&str> {
    let trimmed = text.strip_suffix('\n').unwrap_or(text);
    if trimmed.is_empty() || trimmed.contains('\n') {
        None
    } else {
        Some(trimmed)
    }
}

/// Grow a cluster by absorbing all hunks from `ours_hunks` and `theirs_hunks`
/// that start at or before `cluster_end`. Advances `o_idx`/`t_idx` as hunks
/// are consumed and extends `cluster_end` to cover the absorbed hunks.
pub(crate) fn gather_hunk_cluster(
    ours_hunks: &[(std::ops::Range<u32>, std::ops::Range<u32>)],
    theirs_hunks: &[(std::ops::Range<u32>, std::ops::Range<u32>)],
    o_idx: &mut usize,
    t_idx: &mut usize,
    cluster_end: &mut usize,
) -> (Vec<usize>, Vec<usize>) {
    let mut cluster_o = Vec::new();
    let mut cluster_t = Vec::new();
    loop {
        let mut grew = false;
        if let Some((b, _)) = ours_hunks.get(*o_idx) {
            if (b.start as usize) <= *cluster_end {
                cluster_o.push(*o_idx);
                *cluster_end = (*cluster_end).max(b.end as usize);
                *o_idx += 1;
                grew = true;
            }
        }
        if let Some((b, _)) = theirs_hunks.get(*t_idx) {
            if (b.start as usize) <= *cluster_end {
                cluster_t.push(*t_idx);
                *cluster_end = (*cluster_end).max(b.end as usize);
                *t_idx += 1;
                grew = true;
            }
        }
        if !grew {
            break;
        }
    }
    (cluster_o, cluster_t)
}
