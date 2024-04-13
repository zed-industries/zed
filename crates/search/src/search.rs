use bitflags::bitflags;
pub use buffer_search::BufferSearchBar;
use gpui::{actions, Action, AppContext, IntoElement};
use project::search::SearchQuery;
pub use project_search::ProjectSearchView;
use ui::{prelude::*, Tooltip};
use ui::{ButtonStyle, IconButton};

pub mod buffer_search;
pub mod project_search;
pub(crate) mod search_bar;

pub fn init(cx: &mut AppContext) {
    menu::init();
    buffer_search::init(cx);
    project_search::init(cx);
}

actions!(
    search,
    [
        FocusSearch,
        ToggleWholeWord,
        ToggleCaseSensitive,
        ToggleIncludeIgnored,
        ToggleRegex,
        ToggleReplace,
        SelectNextMatch,
        SelectPrevMatch,
        SelectAllMatches,
        NextHistoryQuery,
        PreviousHistoryQuery,
        ReplaceAll,
        ReplaceNext,
    ]
);

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
    pub struct SearchOptions: u8 {
        const NONE = 0b000;
        const WHOLE_WORD = 0b001;
        const CASE_SENSITIVE = 0b010;
        const INCLUDE_IGNORED = 0b100;
        const REGEX = 0b1000;
    }
}

impl SearchOptions {
    pub fn label(&self) -> &'static str {
        match *self {
            SearchOptions::WHOLE_WORD => "whole word",
            SearchOptions::CASE_SENSITIVE => "match case",
            SearchOptions::INCLUDE_IGNORED => "include Ignored",
            SearchOptions::REGEX => "regular expression",
            _ => panic!("{:?} is not a named SearchOption", self),
        }
    }

    pub fn icon(&self) -> ui::IconName {
        match *self {
            SearchOptions::WHOLE_WORD => ui::IconName::WholeWord,
            SearchOptions::CASE_SENSITIVE => ui::IconName::CaseSensitive,
            SearchOptions::INCLUDE_IGNORED => ui::IconName::FileGit,
            SearchOptions::REGEX => ui::IconName::Regex,
            _ => panic!("{:?} is not a named SearchOption", self),
        }
    }

    pub fn to_toggle_action(&self) -> Box<dyn Action + Sync + Send + 'static> {
        match *self {
            SearchOptions::WHOLE_WORD => Box::new(ToggleWholeWord),
            SearchOptions::CASE_SENSITIVE => Box::new(ToggleCaseSensitive),
            SearchOptions::INCLUDE_IGNORED => Box::new(ToggleIncludeIgnored),
            SearchOptions::REGEX => Box::new(ToggleRegex),
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
        options.set(SearchOptions::INCLUDE_IGNORED, query.include_ignored());
        options.set(SearchOptions::REGEX, query.is_regex());
        options
    }

    pub fn as_button(
        &self,
        active: bool,
        action: impl Fn(&gpui::ClickEvent, &mut WindowContext) + 'static,
    ) -> impl IntoElement {
        IconButton::new(self.label(), self.icon())
            .on_click(action)
            .style(ButtonStyle::Subtle)
            .selected(active)
            .tooltip({
                let action = self.to_toggle_action();
                let label: SharedString = format!("Toggle {}", self.label()).into();
                move |cx| Tooltip::for_action(label.clone(), &*action, cx)
            })
    }
}
