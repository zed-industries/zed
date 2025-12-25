use text::Anchor;
use ui::SharedString;

/// Position of a cell within the source CSV buffer
#[derive(Clone, Debug)]
pub struct CellContentSpan {
    /// Start anchor of the cell content in the source buffer
    pub start: Anchor,
    /// End anchor of the cell content in the source buffer
    pub end: Anchor,
}

/// A table cell with its content and position in the source buffer
#[derive(Clone, Debug)]
pub enum TableCell {
    /// Cell existing in the CSV
    Real {
        /// Position of this cell in the source buffer
        position: CellContentSpan,
        /// Cached display value (for performance)
        cached_value: SharedString,
    },
    /// Virtual cell, created to pad malformed row
    Virtual,
}

impl TableCell {
    /// Create a TableCell with buffer position tracking
    pub fn from_buffer_position(
        content: SharedString,
        start_offset: usize,
        end_offset: usize,
        buffer_snapshot: &text::BufferSnapshot,
    ) -> Self {
        let start_anchor = buffer_snapshot.anchor_before(start_offset);
        let end_anchor = buffer_snapshot.anchor_after(end_offset);

        Self::Real {
            position: CellContentSpan {
                start: start_anchor,
                end: end_anchor,
            },
            cached_value: content,
        }
    }

    /// Get the display value for this cell
    pub fn display_value(&self) -> Option<&SharedString> {
        match self {
            TableCell::Real { cached_value, .. } => Some(cached_value),
            TableCell::Virtual => None,
        }
    }

    pub(crate) fn position(&self) -> Option<&CellContentSpan> {
        match self {
            TableCell::Real { position, .. } => Some(position),
            TableCell::Virtual => None,
        }
    }
}
