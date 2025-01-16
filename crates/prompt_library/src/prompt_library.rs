mod prompt_store;

use gpui::AppContext;

pub use crate::prompt_store::*;

pub fn init(cx: &mut AppContext) {
    prompt_store::init(cx);
}
