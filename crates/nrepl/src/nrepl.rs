mod nrepl_settings;

use gpui::App;
use settings::Settings;

pub use crate::nrepl_settings::NreplSettings;

pub fn init(cx: &mut App) {
    NreplSettings::register(cx);
}
