//! Interactive console pane API for extensions.

pub use crate::wit::zed::extension::console::ConsoleOutputKind;

/// Opens an interactive console item in the workspace (split to the right).
pub fn open_console(id: &str, title: &str) -> Result<(), String> {
    crate::wit::zed::extension::console::open_console(id, title)
}

/// Appends text of a specific kind (output, error, result, input) to the console transcript.
pub fn append(id: &str, text: &str, kind: ConsoleOutputKind) -> Result<(), String> {
    crate::wit::zed::extension::console::append(id, text, kind)
}

/// Clears the transcript of the specified console.
pub fn clear(id: &str) -> Result<(), String> {
    crate::wit::zed::extension::console::clear(id)
}

/// Sets the prompt string displayed before user input in the specified console.
pub fn set_prompt(id: &str, text: &str) -> Result<(), String> {
    crate::wit::zed::extension::console::set_prompt(id, text)
}
