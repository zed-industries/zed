use gpui::{action, MutableAppContext};

mod buffer_search;
mod project_search;

pub fn init(cx: &mut MutableAppContext) {
    buffer_search::init(cx);
    project_search::init(cx);
}

action!(ToggleSearchOption, SearchOption);
action!(SelectMatch, Direction);

#[derive(Clone, Copy)]
pub enum SearchOption {
    WholeWord,
    CaseSensitive,
    Regex,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Prev,
    Next,
}
