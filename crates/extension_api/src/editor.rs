//! Editor interaction and decorations API for extensions.

pub use crate::wit::Range;
pub use crate::wit::zed::extension::editor::{
    BlockDecoration, Decoration, DecorationStyle, EditorState, GutterDecoration,
    HighlightDecoration, InlineDecoration, TextEdit,
};

/// Returns the current state of the active editor, if an editor is open and focused.
pub fn active_editor() -> Option<EditorState> {
    crate::wit::zed::extension::editor::active_editor()
}

/// Applies a list of text edits to the file at the given path.
pub fn apply_edits(path: &str, edits: Vec<TextEdit>) -> Result<(), String> {
    crate::wit::zed::extension::editor::apply_edits(path, &edits)
}

/// Updates the cursor selections in the editor for the given path.
pub fn set_selections(path: &str, selections: Vec<Range>) -> Result<(), String> {
    crate::wit::zed::extension::editor::set_selections(path, &selections)
}

/// Sets decorations (inlays, blocks, highlights, gutter marks) on the editor for the given path.
pub fn set_decorations(path: &str, key: &str, decorations: &[Decoration]) -> Result<(), String> {
    crate::wit::zed::extension::editor::set_decorations(path, key, decorations)
}

/// Clears decorations associated with a key from an editor (or all editors if path is None).
pub fn clear_decorations(path: Option<&str>, key: &str) -> Result<(), String> {
    crate::wit::zed::extension::editor::clear_decorations(path, key)
}
