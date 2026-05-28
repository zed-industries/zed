//! UI-related utilities

use gpui::App;
use theme::ActiveTheme;

mod apca_contrast;
mod color_contrast;
mod constants;
mod corner_solver;
mod format_distance;
mod search_input;
mod with_rem_size;

pub use apca_contrast::*;
pub use color_contrast::*;
pub use constants::*;
pub use corner_solver::{CornerSolver, inner_corner_radius};
pub use format_distance::*;
pub use search_input::*;
pub use with_rem_size::*;

/// Returns true if the current theme is light or vibrant light.
pub fn is_light(cx: &mut App) -> bool {
    cx.theme().appearance.is_light()
}

/// Returns the platform-appropriate label for the "reveal in file manager" action.
pub fn reveal_in_file_manager_label(is_remote: bool) -> &'static str {
    if cfg!(target_os = "macos") && !is_remote {
        "Reveal in Finder"
    } else if cfg!(target_os = "windows") && !is_remote {
        "Reveal in File Explorer"
    } else {
        "Reveal in File Manager"
    }
}

/// Capitalizes the first character of a string.
///
/// This function takes a string slice as input and returns a new `String` with the first character
/// capitalized.
///
/// # Examples
///
/// ```
/// use ui::utils::capitalize;
///
/// assert_eq!(capitalize("hello"), "Hello");
/// assert_eq!(capitalize("WORLD"), "WORLD");
/// assert_eq!(capitalize(""), "");
/// ```
pub fn capitalize(str: &str) -> String {
    let mut chars = str.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
