use bitflags::bitflags;
pub use buffer_search::BufferSearchBar;
use gpui::{actions, Action, AppContext, IntoElement};
pub use mode::SearchMode;
use project::search::SearchQuery;
use ui::prelude::*;
use ui::{ButtonStyle, Icon, IconButton};
//pub use project_search::{ProjectSearchBar, ProjectSearchView};
// use theme::components::{
//     action_button::Button, svg::Svg, ComponentExt, IconButtonStyle, ToggleIconButtonStyle,
// };

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
    CycleMode,
    ToggleWholeWord,
    ToggleCaseSensitive,
    ToggleReplace,
    SelectNextMatch,
    SelectPrevMatch,
    SelectAllMatches,
    NextHistoryQuery,
    PreviousHistoryQuery,
    ActivateTextMode,
    ActivateSemanticMode,
    ActivateRegexMode,
    ReplaceAll,
    ReplaceNext,
);

bitflags! {
    #[derive(Default)]
    pub struct SearchOptions: u8 {
        const NONE = 0b000;
        const WHOLE_WORD = 0b001;
        const CASE_SENSITIVE = 0b010;
        const INCLUDE_IGNORED = 0b100;
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

    pub fn icon(&self) -> ui::Icon {
        match *self {
            SearchOptions::WHOLE_WORD => ui::Icon::WholeWord,
            SearchOptions::CASE_SENSITIVE => ui::Icon::CaseSensitive,
            _ => panic!("{:?} is not a named SearchOption", self),
        }
    }

    pub fn to_toggle_action(&self) -> Box<dyn Action + Sync + Send + 'static> {
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

    pub fn as_button(&self, active: bool) -> impl IntoElement {
        IconButton::new(0, self.icon())
            .on_click({
                let action = self.to_toggle_action();
                move |_, cx| {
                    cx.dispatch_action(action.boxed_clone());
                }
            })
            .style(ButtonStyle::Subtle)
            .when(active, |button| button.style(ButtonStyle::Filled))
    }
}

fn toggle_replace_button(active: bool) -> impl IntoElement {
    // todo: add toggle_replace button
    IconButton::new(0, Icon::Replace)
        .on_click(|_, cx| {
            cx.dispatch_action(Box::new(ToggleReplace));
            cx.notify();
        })
        .style(ButtonStyle::Subtle)
        .when(active, |button| button.style(ButtonStyle::Filled))
}

fn render_replace_button(
    action: impl Action + 'static + Send + Sync,
    icon: Icon,
) -> impl IntoElement {
    // todo: add tooltip
    IconButton::new(0, icon).on_click(move |_, cx| {
        cx.dispatch_action(action.boxed_clone());
    })
}
