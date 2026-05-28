use gpui::{App, Context, Entity, EventEmitter, SharedString};
use std::{cmp::Ordering, ops::Range, sync::Arc};
use text::{Anchor, BufferId, OffsetRangeExt as _};

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

impl ConflictRegion {
    pub fn resolve(
        &self,
        buffer: Entity<language::Buffer>,
        ranges: &[Range<Anchor>],
        cx: &mut App,
    ) {
        let buffer_snapshot = buffer.read(cx).snapshot();
        let mut deletions = Vec::new();
        let empty = "";
        let outer_range = self.range.to_offset(&buffer_snapshot);
        let mut offset = outer_range.start;
        for kept_range in ranges {
            let kept_range = kept_range.to_offset(&buffer_snapshot);
            if kept_range.start > offset {
                deletions.push((offset..kept_range.start, empty));
            }
            offset = kept_range.end;
        }
        if outer_range.end > offset {
            deletions.push((offset..outer_range.end, empty));
        }

        buffer.update(cx, |buffer, cx| {
            buffer.edit(deletions, None, cx);
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
