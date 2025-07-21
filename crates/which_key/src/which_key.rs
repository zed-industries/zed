//! Which-key support for Zed.

mod which_key_settings;

pub use which_key_settings::*;

use gpui::App;
use settings::Settings;

pub fn init(cx: &mut App) {
    WhichKeySettings::register(cx);
}
