mod bidi;
/// Input keybinding configuration & actions that can be bound (`Backspace`, `Copy`, etc.).
///
/// Explicitly not exported using `pub use bindings::*` to avoid namespace pollution.
pub mod bindings;
mod handler;
mod state;

pub use bidi::{TextDirection, detect_base_direction};
pub use bindings::{INPUT_CONTEXT, InputBindings, bind_input_keys};
pub use handler::*;
pub use state::{InputLineLayout, InputState};
