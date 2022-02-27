use gpui::MutableAppContext;

mod buffer_search;
mod project_search;

pub fn init(cx: &mut MutableAppContext) {
    buffer_search::init(cx);
    project_search::init(cx);
}

#[derive(Clone, Copy)]
pub enum SearchOption {
    WholeWord,
    CaseSensitive,
    Regex,
}
