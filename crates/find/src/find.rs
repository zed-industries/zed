use gpui::MutableAppContext;

mod buffer_find;
mod project_find;

pub fn init(cx: &mut MutableAppContext) {
    buffer_find::init(cx);
    project_find::init(cx);
}

#[derive(Clone, Copy)]
pub enum SearchOption {
    WholeWord,
    CaseSensitive,
    Regex,
}
