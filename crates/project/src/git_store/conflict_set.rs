use gpui::{Context, EventEmitter};
use std::{cmp::Ordering, ops::Range, sync::Arc};
use text::{Anchor, BufferId};

pub struct ConflictSet {
    pub has_conflict: bool,
    pub snapshot: ConflictSetSnapshot,
}

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
            (Some(first), Some(second)) => Some(first.range.start.min(&second.range.start, buffer)),
        };
        let end = match (old_conflicts.last(), new_conflicts.last()) {
            (None, None) => None,
            (None, Some(conflict)) => Some(conflict.range.end),
            (Some(first), None) => Some(first.range.end),
            (Some(first), Some(second)) => Some(first.range.end.max(&second.range.end, buffer)),
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
    pub range: Range<Anchor>,
    pub ours: Range<Anchor>,
    pub theirs: Range<Anchor>,
    pub base: Option<Range<Anchor>>,
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

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.snapshot.conflicts = Default::default();
        cx.notify();
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
        let mut lines = buffer.text_for_range(0..buffer.len()).lines();

        let mut conflict_start: Option<usize> = None;
        let mut ours_start: Option<usize> = None;
        let mut ours_end: Option<usize> = None;
        let mut base_start: Option<usize> = None;
        let mut base_end: Option<usize> = None;
        let mut theirs_start: Option<usize> = None;

        while let Some(line) = lines.next() {
            let line_end = line_pos + line.len();

            if line.starts_with("<<<<<<< ") {
                // If we see a new conflict marker while already parsing one,
                // abandon the previous one and start a new one
                conflict_start = Some(line_pos);
                ours_start = Some(line_end + 1);
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
            } else if line.starts_with(">>>>>>> ")
                && conflict_start.is_some()
                && ours_start.is_some()
                && ours_end.is_some()
                && theirs_start.is_some()
            {
                let theirs_end = line_pos;
                let conflict_end = line_end + 1;

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

#[cfg(test)]
mod tests {
    use super::*;
    use text::{Buffer, BufferId, ToOffset as _};
    use unindent::Unindent as _;

    #[test]
    fn test_parse_conflicts_in_buffer() {
        // Create a buffer with conflict markers
        let test_content = r#"
            This is some text before the conflict.
            <<<<<<< HEAD
            This is our version
            =======
            This is their version
            >>>>>>> branch-name

            Another conflict:
            <<<<<<< HEAD
            Our second change
            ||||||| merged common ancestors
            Original content
            =======
            Their second change
            >>>>>>> branch-name
        "#
        .unindent();

        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(0, buffer_id, test_content);
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);
        assert_eq!(conflict_snapshot.conflicts.len(), 2);

        let first = &conflict_snapshot.conflicts[0];
        assert!(first.base.is_none());
        let our_text = snapshot
            .text_for_range(first.ours.clone())
            .collect::<String>();
        let their_text = snapshot
            .text_for_range(first.theirs.clone())
            .collect::<String>();
        assert_eq!(our_text, "This is our version\n");
        assert_eq!(their_text, "This is their version\n");

        let second = &conflict_snapshot.conflicts[1];
        assert!(second.base.is_some());
        let our_text = snapshot
            .text_for_range(second.ours.clone())
            .collect::<String>();
        let their_text = snapshot
            .text_for_range(second.theirs.clone())
            .collect::<String>();
        let base_text = snapshot
            .text_for_range(second.base.as_ref().unwrap().clone())
            .collect::<String>();
        assert_eq!(our_text, "Our second change\n");
        assert_eq!(their_text, "Their second change\n");
        assert_eq!(base_text, "Original content\n");

        // Test conflicts_in_range
        let range = snapshot.anchor_before(0)..snapshot.anchor_before(snapshot.len());
        let conflicts_in_range = conflict_snapshot.conflicts_in_range(range, &snapshot);
        assert_eq!(conflicts_in_range.len(), 2);

        // Test with a range that includes only the first conflict
        let first_conflict_end = conflict_snapshot.conflicts[0].range.end;
        let range = snapshot.anchor_before(0)..first_conflict_end;
        let conflicts_in_range = conflict_snapshot.conflicts_in_range(range, &snapshot);
        assert_eq!(conflicts_in_range.len(), 1);

        // Test with a range that includes only the second conflict
        let second_conflict_start = conflict_snapshot.conflicts[1].range.start;
        let range = second_conflict_start..snapshot.anchor_before(snapshot.len());
        let conflicts_in_range = conflict_snapshot.conflicts_in_range(range, &snapshot);
        assert_eq!(conflicts_in_range.len(), 1);

        // Test with a range that doesn't include any conflicts
        let range = buffer.anchor_after(first_conflict_end.to_offset(&buffer) + 1)
            ..buffer.anchor_before(second_conflict_start.to_offset(&buffer) - 1);
        let conflicts_in_range = conflict_snapshot.conflicts_in_range(range, &snapshot);
        assert_eq!(conflicts_in_range.len(), 0);
    }

    #[test]
    fn test_nested_conflict_markers() {
        // Create a buffer with nested conflict markers
        let test_content = r#"
            This is some text before the conflict.
            <<<<<<< HEAD
            This is our version
            <<<<<<< HEAD
            This is a nested conflict marker
            =======
            This is their version in a nested conflict
            >>>>>>> branch-nested
            =======
            This is their version
            >>>>>>> branch-name
        "#
        .unindent();

        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(0, buffer_id, test_content.to_string());
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);

        assert_eq!(conflict_snapshot.conflicts.len(), 1);

        // The conflict should have our version, their version, but no base
        let conflict = &conflict_snapshot.conflicts[0];
        assert!(conflict.base.is_none());

        // Check that the nested conflict was detected correctly
        let our_text = snapshot
            .text_for_range(conflict.ours.clone())
            .collect::<String>();
        assert_eq!(our_text, "This is a nested conflict marker\n");
        let their_text = snapshot
            .text_for_range(conflict.theirs.clone())
            .collect::<String>();
        assert_eq!(their_text, "This is their version in a nested conflict\n");
    }

    #[test]
    fn test_conflicts_in_range() {
        // Create a buffer with conflict markers
        let test_content = r#"
            one
            <<<<<<< HEAD1
            two
            =======
            three
            >>>>>>> branch1
            four
            five
            <<<<<<< HEAD2
            six
            =======
            seven
            >>>>>>> branch2
            eight
            nine
            <<<<<<< HEAD3
            ten
            =======
            eleven
            >>>>>>> branch3
            twelve
            <<<<<<< HEAD4
            thirteen
            =======
            fourteen
            >>>>>>> branch4
            fifteen
        "#
        .unindent();

        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(0, buffer_id, test_content.clone());
        let snapshot = buffer.snapshot();

        let conflict_snapshot = ConflictSet::parse(&snapshot);
        assert_eq!(conflict_snapshot.conflicts.len(), 4);

        let range = test_content.find("seven").unwrap()..test_content.find("eleven").unwrap();
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        assert_eq!(
            conflict_snapshot.conflicts_in_range(range, &snapshot),
            &conflict_snapshot.conflicts[1..=2]
        );

        let range = test_content.find("one").unwrap()..test_content.find("<<<<<<< HEAD2").unwrap();
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        assert_eq!(
            conflict_snapshot.conflicts_in_range(range, &snapshot),
            &conflict_snapshot.conflicts[0..=1]
        );

        let range =
            test_content.find("eight").unwrap() - 1..test_content.find(">>>>>>> branch3").unwrap();
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        assert_eq!(
            conflict_snapshot.conflicts_in_range(range, &snapshot),
            &conflict_snapshot.conflicts[1..=2]
        );

        let range = test_content.find("thirteen").unwrap() - 1..test_content.len();
        let range = buffer.anchor_before(range.start)..buffer.anchor_after(range.end);
        assert_eq!(
            conflict_snapshot.conflicts_in_range(range, &snapshot),
            &conflict_snapshot.conflicts[3..=3]
        );
    }
}
