mod dap_log;
pub use dap_log::*;

use gpui::AppContext;

pub fn init(cx: &mut AppContext) {
    dap_log::init(cx);
}
