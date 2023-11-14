use bitflags::bitflags;
pub use buffer_search::BufferSearchBar;
use gpui::{actions, Action, AnyElement, AppContext, Component, Element, Svg, View};
pub use mode::SearchMode;
use project::search::SearchQuery;
use ui::ButtonVariant;
//pub use project_search::{ProjectSearchBar, ProjectSearchView};
// use theme::components::{
//     action_button::Button, svg::Svg, ComponentExt, IconButtonStyle, ToggleIconButtonStyle,
// };

pub mod buffer_search;
mod history;
mod mode;
//pub mod project_search;
pub(crate) mod search_bar;

pub fn init(cx: &mut AppContext) {
    buffer_search::init(cx);
    //project_search::init(cx);
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

    pub fn as_button<V: 'static>(&self, active: bool) -> impl Component<V> {
        ui::IconButton::new(0, self.icon())
            .on_click({
                let action = self.to_toggle_action();
                move |_: &mut V, cx| {
                    cx.dispatch_action(action.boxed_clone());
                }
            })
            .variant(ui::ButtonVariant::Ghost)
            .when(active, |button| button.variant(ButtonVariant::Filled))
    }
}

fn toggle_replace_button<V: 'static>(active: bool) -> impl Component<V> {
    // todo: add toggle_replace button
    ui::IconButton::new(0, ui::Icon::Replace)
        .on_click(|_: &mut V, cx| {
            cx.dispatch_action(Box::new(ToggleReplace));
            cx.notify();
        })
        .variant(ui::ButtonVariant::Ghost)
        .when(active, |button| button.variant(ButtonVariant::Filled))
}

fn replace_action<V: 'static>(
    action: impl Action + 'static + Send + Sync,
    name: &'static str,
) -> impl Component<V> {
    ui::IconButton::new(0, ui::Icon::Replace).on_click(move |_: &mut V, cx| {
        cx.dispatch_action(action.boxed_clone());
    })
}
