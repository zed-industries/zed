pub use buffer_search::BufferSearchBar;
use gpui::{actions, Action, AppContext};
pub use project_search::{ProjectSearchBar, ProjectSearchView};

pub mod buffer_search;
pub mod project_search;

pub fn init(cx: &mut AppContext) {
    buffer_search::init(cx);
    project_search::init(cx);
}

actions!(
    search,
    [
        ToggleWholeWord,
        ToggleCaseSensitive,
        ToggleRegex,
        SelectNextMatch,
        SelectPrevMatch,
        SelectAllMatches,
    ]
);

#[derive(Clone, Copy, PartialEq)]
pub enum SearchOption {
    WholeWord,
    CaseSensitive,
    Regex,
}

impl SearchOption {
    pub fn label(&self) -> &'static str {
        match self {
            SearchOption::WholeWord => "Match Whole Word",
            SearchOption::CaseSensitive => "Match Case",
            SearchOption::Regex => "Use Regular Expression",
        }
    }

    pub fn to_toggle_action(&self) -> Box<dyn Action> {
        match self {
            SearchOption::WholeWord => Box::new(ToggleWholeWord),
            SearchOption::CaseSensitive => Box::new(ToggleCaseSensitive),
            SearchOption::Regex => Box::new(ToggleRegex),
        }
    }
}
