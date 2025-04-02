use anyhow::{Result, bail};
use gpui::{Context, EventEmitter};
use std::ops::Range;
use text::{Anchor, BufferId};

pub struct ConflictSet {
    buffer_id: BufferId,
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
    pub fn new(buffer_id: BufferId, _: &mut Context<Self>) -> Self {
        Self {
            buffer_id,
            conflicts: Vec::new(),
        }
    }

    pub fn conflicts(&self) -> &[Conflict] {
        &self.conflicts
    }

    pub fn parse(buffer: &text::BufferSnapshot) -> Result<Vec<Conflict>> {
        let mut conflicts = Vec::new();
        let mut current_conflict: Option<(Range<usize>, Option<Range<usize>>)> = None;
        let mut ours: Option<Range<usize>> = None;

        let mut line_start = 0;
        let mut lines = buffer.text_for_range(0..buffer.len()).lines();

        while let Some(line) = lines.next() {
            let line_end = line_start + line.len();

            if line.starts_with("<<<<<<< ") {
                if current_conflict.is_some() {
                    bail!("Nested conflict markers not supported");
                }
                current_conflict = Some((line_start..line_end + 1, None));
                ours = Some(line_end + 1..0);
            } else if line.starts_with("=======") && current_conflict.is_some() {
                if let Some(our_range) = ours.as_mut() {
                    our_range.end = line_start;
                }
                if let Some((range, _)) = current_conflict.as_mut() {
                    range.end = line_end + 1;
                }
            } else if line.starts_with("||||||| ") && current_conflict.is_some() {
                if let Some(our_range) = ours.as_mut() {
                    our_range.end = line_start;
                }
                if let Some((_, base)) = current_conflict.as_mut() {
                    *base = Some(line_end + 1..0);
                }
            } else if line.starts_with(">>>>>>> ") && current_conflict.is_some() {
                if let Some((mut conflict_range, base_range)) = current_conflict.take() {
                    conflict_range.end = line_end + 1;

                    let (theirs, base) = if let Some(base) = base_range {
                        let mut base_range = base;
                        base_range.end = line_start;
                        (base_range.clone(), Some(base_range))
                    } else {
                        let mut our_range = ours.take().unwrap();
                        our_range.end = line_start;
                        (our_range.end..line_start, None)
                    };

                    let range = buffer.anchor_after(conflict_range.start)
                        ..buffer.anchor_before(conflict_range.end);
                    let ours = ours.take().unwrap();
                    let ours = buffer.anchor_after(ours.start)..buffer.anchor_before(ours.end);
                    let theirs =
                        buffer.anchor_after(theirs.start)..buffer.anchor_before(theirs.end);
                    let base = base.map(|base| {
                        buffer.anchor_after(base.start)..buffer.anchor_before(base.end)
                    });

                    conflicts.push(Conflict {
                        range,
                        ours,
                        theirs,
                        base,
                    });
                }
            }

            line_start = line_end + 1;
        }

        Ok(conflicts)
    }
}

impl EventEmitter<()> for ConflictSet {}

#[cfg(test)]
mod tests {
    use super::*;
    use text::{Buffer, BufferId};

    #[test]
    fn test_parse_conflicts_in_buffer() -> Result<()> {
        // Create a buffer with conflict markers
        let test_content = r#"This is some text before the conflict.
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
"#;

        let buffer_id = BufferId::new(1).unwrap();
        let buffer = Buffer::new(0, buffer_id, test_content.to_string());
        let snapshot = buffer.snapshot();

        // Parse conflicts
        let conflicts = ConflictSet::parse(&snapshot)?;

        // Verify there are 2 conflicts
        assert_eq!(conflicts.len(), 2);

        // First conflict should have our version, their version, but no base
        let first = &conflicts[0];
        assert!(first.base.is_none());
        let our_text = snapshot
            .text_for_range(first.ours.clone())
            .collect::<String>();
        let their_text = snapshot
            .text_for_range(first.theirs.clone())
            .collect::<String>();
        assert_eq!(our_text, "This is our version\n");
        assert_eq!(their_text, "This is their version\n");

        // Second conflict should have our version, their version, and a base
        let second = &conflicts[1];
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

        Ok(())
    }
}
