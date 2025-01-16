mod prompt_store;
mod prompts;

use gpui::AppContext;

pub use crate::prompt_store::*;
pub use crate::prompts::*;

pub fn init(cx: &mut AppContext) {
    prompt_store::init(cx);
}
