use bitflags::bitflags;
pub use buffer_search::BufferSearchBar;
use gpui::{
    actions,
    elements::{Component, SafeStylable, TooltipStyle},
    Action, AnyElement, AppContext, Element, View,
};
pub use mode::SearchMode;
use project::search::SearchQuery;
pub use project_search::{ProjectSearchBar, ProjectSearchView};
use theme::components::{
    action_button::ActionButton, svg::Svg, ComponentExt, ToggleIconButtonStyle,
};

pub mod buffer_search;
mod history;
mod mode;
pub mod project_search;
pub(crate) mod search_bar;

pub fn init(cx: &mut AppContext) {
    buffer_search::init(cx);
    project_search::init(cx);
}

actions!(
    search,
    [
        CycleMode,
        ToggleWholeWord,
        ToggleCaseSensitive,
        SelectNextMatch,
        SelectPrevMatch,
        SelectAllMatches,
        NextHistoryQuery,
        PreviousHistoryQuery,
        ActivateTextMode,
        ActivateSemanticMode,
        ActivateRegexMode
    ]
);

bitflags! {
    #[derive(Default)]
    pub struct SearchOptions: u8 {
        const NONE = 0b000;
        const WHOLE_WORD = 0b001;
        const CASE_SENSITIVE = 0b010;
    }
}

impl SearchOptions {
    pub fn label(&self) -> &'static str {
        match *self {
            SearchOptions::WHOLE_WORD => "Match Whole Word",
            SearchOptions::CASE_SENSITIVE => "Match Case",
            _ => panic!("{:?} is not a named SearchOption", self),
        }
    }

    pub fn icon(&self) -> &'static str {
        match *self {
            SearchOptions::WHOLE_WORD => "icons/word_search_12.svg",
            SearchOptions::CASE_SENSITIVE => "icons/case_insensitive_12.svg",
            _ => panic!("{:?} is not a named SearchOption", self),
        }
    }

    pub fn to_toggle_action(&self) -> Box<dyn Action> {
        match *self {
            SearchOptions::WHOLE_WORD => Box::new(ToggleWholeWord),
            SearchOptions::CASE_SENSITIVE => Box::new(ToggleCaseSensitive),
            _ => panic!("{:?} is not a named SearchOption", self),
        }
    }

    pub fn none() -> SearchOptions {
        SearchOptions::NONE
    }

    pub fn from_query(query: &SearchQuery) -> SearchOptions {
        let mut options = SearchOptions::NONE;
        options.set(SearchOptions::WHOLE_WORD, query.whole_word());
        options.set(SearchOptions::CASE_SENSITIVE, query.case_sensitive());
        options
    }

    pub fn as_button<V: View>(
        &self,
        active: bool,
        tooltip_style: TooltipStyle,
        button_style: ToggleIconButtonStyle,
    ) -> AnyElement<V> {
        ActionButton::dynamic_action(self.to_toggle_action())
            .with_tooltip(format!("Toggle {}", self.label()), tooltip_style)
            .with_contents(Svg::new(self.icon()))
            .toggleable(active)
            .with_style(button_style)
            .element()
            .into_any()
    }
}
