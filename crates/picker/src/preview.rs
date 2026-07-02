use std::ops::Range;
use std::path::PathBuf;
use std::sync::Arc;

use gpui::{AnyElement, App, Entity, IntoElement, Window};
use language::{Anchor, Buffer, HighlightedText};

/// The editor-agnostic interface a [`Picker`](crate::Picker) uses to drive its
/// preview.
pub trait PreviewBackend: 'static {
    fn update(&self, update: Update, window: &mut Window, cx: &mut App);
    fn render(&self, layout: Layout, cx: &mut App) -> AnyElement;
    /// Called after a resize to let the preview do resizing logic like scrolling.
    fn adjust_to_new_size(&self, window: &mut Window, cx: &mut App);
    /// Empty the preview and show a placeholder message.
    fn clear(&self, cx: &mut App);
}

/// The preview window of a [`Picker`](crate::Picker).
pub struct Preview {
    content: Arc<dyn PreviewBackend>,
    pub(crate) layout: Layout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Layout {
    #[default]
    Hidden,
    Below,
    Right,
}

impl Preview {
    pub fn new(content: Arc<dyn PreviewBackend>) -> Self {
        Preview {
            content,
            layout: Layout::default(),
        }
    }

    pub fn update(&mut self, update: Update, window: &mut Window, cx: &mut App) {
        self.content.update(update, window, cx);
    }

    pub fn render(&self, cx: &mut App) -> impl IntoElement {
        self.content.render(self.layout, cx)
    }

    pub fn adjust_to_new_size(&self, window: &mut Window, cx: &mut App) {
        self.content.adjust_to_new_size(window, cx);
    }

    pub(crate) fn clear(&self, cx: &mut App) {
        self.content.clear(cx);
    }
}

/// Identifies what a preview should show.
pub enum PreviewSource {
    /// The buffer is identified by its absolute path; the preview opens it.
    ///
    /// Used by pickers (like the file finder) that only know the path of the
    /// match.
    Path(PathBuf),
    /// The buffer is provided directly.
    ///
    /// Used by pickers (like the text picker) that already hold the matched
    /// buffer.
    Buffer(Entity<Buffer>),
    /// No buffer to show; display this message centered in the preview instead.
    ///
    /// Used by pickers that have a selection without a previewable buffer (like
    /// the file finder's "create new file" entry). Built as a [`HighlightedText`]
    /// so callers can emphasize parts of the message (e.g. a file path).
    Message(HighlightedText),
}

pub struct MatchLocation {
    /// The location of the match (for highlighting)
    pub anchor_range: Range<Anchor>,
    /// The location of the match as an offset (for scrolling)
    pub range: Range<usize>,
}

/// An update for the [`Preview`] window of a [`Picker`](crate::Picker).
pub struct Update {
    /// Where to source the buffer to preview.
    pub source: PreviewSource,
    /// The location to highlight and scroll to, if any.
    pub match_location: Option<MatchLocation>,
}

impl Update {
    /// Preview the buffer at `abs_path` without highlighting anything.
    pub fn from_path(abs_path: PathBuf) -> Self {
        Self {
            source: PreviewSource::Path(abs_path),
            match_location: None,
        }
    }

    /// Show `message` centered in the preview instead of a buffer.
    ///
    /// The message is a [`HighlightedText`], so parts of it (e.g. a file path)
    /// can be emphasized.
    pub fn message(message: HighlightedText) -> Self {
        Self {
            source: PreviewSource::Message(message),
            match_location: None,
        }
    }

    /// Preview `buffer`, highlighting and scrolling to `highlight`.
    pub fn from_buffer(buffer: Entity<Buffer>, highlight: MatchLocation) -> Self {
        Self {
            source: PreviewSource::Buffer(buffer),
            match_location: Some(highlight),
        }
    }
}
