pub mod cursor_position;
pub mod go_to_file;
pub mod go_to_line;
use crate::cursor_position::LineIndicatorFormat;
use go_to_file::GoToFile;
use go_to_line::GoToLine;
use gpui::AppContext;
use settings::Settings;

pub fn init(cx: &mut AppContext) {
    LineIndicatorFormat::register(cx);
    cx.observe_new_views(GoToLine::register).detach();
    cx.observe_new_views(GoToFile::register).detach();
}
