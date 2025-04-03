use gpui::{Context, EventEmitter};
use std::ops::Range;
use text::{Anchor, BufferId};

pub struct ConflictSet {
    pub has_conflict: bool,
    pub snapshot: ConflictSetSnapshot,
}

#[derive(Debug, Clone)]
pub struct ConflictSetSnapshot {
    pub buffer_id: BufferId,
    pub conflicts: Vec<Conflict>,
}

#[derive(Debug, Clone)]
pub struct Conflict {
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
                conflicts: Vec::new(),
            },
        }
    }

    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.snapshot.conflicts.clear();
        cx.notify();
    }

    pub fn conflicts(&self) -> &[Conflict] {
        &self.snapshot.conflicts
    }

    pub fn set_snapshot(&mut self, snapshot: ConflictSetSnapshot) {
        self.snapshot = snapshot;
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

                conflicts.push(Conflict {
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
            conflicts,
            buffer_id: buffer.remote_id(),
        }
    }
}

impl EventEmitter<()> for ConflictSet {}

#[cfg(test)]
mod tests {
    use super::*;
    use text::{Buffer, BufferId};
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
}
