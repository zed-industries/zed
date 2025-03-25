mod dap_log;
pub use dap_log::*;

use gpui::App;

pub fn init(cx: &mut App) {
    dap_log::init(cx);
}
