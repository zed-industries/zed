//! UI-related utilities

use gpui::App;
use theme::ActiveTheme;

mod color_contrast;
mod corner_solver;
mod format_distance;
mod search_input;
mod with_rem_size;

pub use color_contrast::*;
pub use corner_solver::{CornerSolver, corner_solver};
pub use format_distance::*;
pub use search_input::*;
pub use with_rem_size::*;

/// Returns true if the current theme is light or vibrant light.
pub fn is_light(cx: &mut App) -> bool {
    cx.theme().appearance.is_light()
}
