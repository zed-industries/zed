use gpui::{App, actions};
use workspace::Workspace;

pub mod markdown_elements;
mod markdown_minifier;
pub mod markdown_parser;
pub mod markdown_preview_view;
pub mod markdown_renderer;

actions!(
    markdown,
    [
        /// Scrolls up by one page in the markdown preview.
        MovePageUp,
        /// Scrolls down by one page in the markdown preview.
        MovePageDown,
        /// Opens a markdown preview for the current file.
        OpenPreview,
        /// Opens a markdown preview in a split pane.
        OpenPreviewToTheSide,
        /// Opens a following markdown preview that syncs with the editor.
        OpenFollowingPreview,
        /// Copies all markdown content as plain text to clipboard.
        CopyAll,
    ]
);

/// A position within the markdown preview content.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarkdownPosition {
    /// The index of the block containing this position.
    pub block_index: usize,
    /// The character offset within the block's plain text content.
    pub char_offset: usize,
}

impl MarkdownPosition {
    pub fn new(block_index: usize, char_offset: usize) -> Self {
        Self {
            block_index,
            char_offset,
        }
    }
}

/// The current text selection state in the markdown preview.
#[derive(Clone, Debug)]
pub struct SelectionState {
    /// Where the selection started (the anchor point).
    pub anchor: MarkdownPosition,
    /// The current end of the selection (moves during drag).
    pub head: MarkdownPosition,
}

impl SelectionState {
    pub fn new(anchor: MarkdownPosition) -> Self {
        Self {
            anchor,
            head: anchor,
        }
    }

    /// Returns the start and end of the selection in document order.
    pub fn ordered(&self) -> (MarkdownPosition, MarkdownPosition) {
        if self.anchor <= self.head {
            (self.anchor, self.head)
        } else {
            (self.head, self.anchor)
        }
    }

    /// Returns true if the selection is empty (anchor equals head).
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }
}

/// The phase of the selection process.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SelectionPhase {
    /// No selection is in progress.
    #[default]
    None,
    /// User is actively selecting (mouse is down and dragging).
    Selecting,
    /// Selection is complete (mouse was released).
    Ended,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
    })
    .detach();
}
