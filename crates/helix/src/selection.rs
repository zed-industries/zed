use editor::{MultiBufferOffset, MultiBufferSnapshot};
use language::Selection;
use text::Bias;
use unicode_segmentation::GraphemeCursor;

/// A selection in Helix terms: `anchor` stays put while `head` moves, and the
/// block cursor covers the grapheme before `head` when the range is forward.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HelixRange {
    pub anchor: MultiBufferOffset,
    pub head: MultiBufferOffset,
}

impl HelixRange {
    pub fn new(anchor: MultiBufferOffset, head: MultiBufferOffset) -> Self {
        Self { anchor, head }
    }

    pub fn point(offset: MultiBufferOffset) -> Self {
        Self::new(offset, offset)
    }

    pub fn from_selection(selection: &Selection<MultiBufferOffset>) -> Self {
        if selection.reversed {
            Self::new(selection.end, selection.start)
        } else {
            Self::new(selection.start, selection.end)
        }
    }

    /// Writes the range into `selection`, keeping its id and goal.
    pub fn apply_to(self, selection: &mut Selection<MultiBufferOffset>) {
        selection.start = self.from();
        selection.end = self.to();
        selection.reversed = self.head < self.anchor;
    }

    pub fn from(&self) -> MultiBufferOffset {
        self.anchor.min(self.head)
    }

    pub fn to(&self) -> MultiBufferOffset {
        self.anchor.max(self.head)
    }

    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// The start of the grapheme the block cursor is on.
    pub fn cursor(&self, snapshot: &MultiBufferSnapshot) -> MultiBufferOffset {
        if self.head > self.anchor {
            prev_grapheme_boundary(snapshot, self.head)
        } else {
            self.head
        }
    }

    /// Puts the block cursor on the grapheme starting at `offset`, optionally
    /// extending the range while keeping the grapheme under the anchor selected.
    pub fn put_cursor(
        self,
        snapshot: &MultiBufferSnapshot,
        offset: MultiBufferOffset,
        extend: bool,
    ) -> Self {
        if !extend {
            return Self::point(offset);
        }
        let anchor = if self.head >= self.anchor && offset < self.anchor {
            next_grapheme_boundary(snapshot, self.anchor)
        } else if self.head < self.anchor && offset >= self.anchor {
            prev_grapheme_boundary(snapshot, self.anchor)
        } else {
            self.anchor
        };
        if anchor <= offset {
            Self::new(anchor, next_grapheme_boundary(snapshot, offset))
        } else {
            Self::new(anchor, offset)
        }
    }

    /// Widens an empty range to cover the next grapheme, if there is one.
    pub fn min_width_1(self, snapshot: &MultiBufferSnapshot) -> Self {
        if self.is_empty() {
            Self::new(self.anchor, next_grapheme_boundary(snapshot, self.head))
        } else {
            self
        }
    }
}

pub(crate) fn next_grapheme_boundary(
    snapshot: &MultiBufferSnapshot,
    offset: MultiBufferOffset,
) -> MultiBufferOffset {
    grapheme_boundary(snapshot, offset, true)
}

pub(crate) fn prev_grapheme_boundary(
    snapshot: &MultiBufferSnapshot,
    offset: MultiBufferOffset,
) -> MultiBufferOffset {
    grapheme_boundary(snapshot, offset, false)
}

pub(crate) fn nth_next_grapheme_boundary(
    snapshot: &MultiBufferSnapshot,
    mut offset: MultiBufferOffset,
    count: usize,
) -> MultiBufferOffset {
    for _ in 0..count {
        let next = next_grapheme_boundary(snapshot, offset);
        if next == offset {
            break;
        }
        offset = next;
    }
    offset
}

pub(crate) fn nth_prev_grapheme_boundary(
    snapshot: &MultiBufferSnapshot,
    mut offset: MultiBufferOffset,
    count: usize,
) -> MultiBufferOffset {
    for _ in 0..count {
        let previous = prev_grapheme_boundary(snapshot, offset);
        if previous == offset {
            break;
        }
        offset = previous;
    }
    offset
}

fn grapheme_boundary(
    snapshot: &MultiBufferSnapshot,
    offset: MultiBufferOffset,
    forward: bool,
) -> MultiBufferOffset {
    let len = snapshot.len();
    let offset = offset.min(len);
    if forward && offset == len {
        return len;
    }
    if !forward && offset == MultiBufferOffset(0) {
        return offset;
    }

    // Segment a window around `offset`, growing it while the boundary found
    // could be an artifact of cutting a grapheme at the window edge.
    let mut context = 64;
    loop {
        let start = snapshot.clip_offset(offset.saturating_sub_usize(context), Bias::Left);
        let end = snapshot.clip_offset((offset + context).min(len), Bias::Right);
        let text = snapshot.text_for_range(start..end).collect::<String>();
        let mut cursor = GraphemeCursor::new(offset - start, text.len(), true);
        let boundary = if forward {
            cursor.next_boundary(&text, 0)
        } else {
            cursor.prev_boundary(&text, 0)
        };
        let covers_buffer = start == MultiBufferOffset(0) && end == len;
        if let Ok(Some(boundary)) = boundary {
            let at_window_edge = if forward {
                boundary == text.len() && end < len
            } else {
                boundary == 0 && start > MultiBufferOffset(0)
            };
            if !at_window_edge || covers_buffer {
                return start + boundary;
            }
        } else if covers_buffer {
            return if forward { len } else { MultiBufferOffset(0) };
        }
        context *= 4;
    }
}
