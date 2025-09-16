mod file_size_indicator;

pub use file_size_indicator::{FileSizeIndicator, FileSizeSettings};
use settings::Settings;

pub fn init(cx: &mut gpui::App) {
    FileSizeSettings::register(cx);
}