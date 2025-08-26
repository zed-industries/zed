use bitflags::bitflags;
pub use buffer_search::BufferSearchBar;
use editor::SearchSettings;
use gpui::{Action, App, ClickEvent, FocusHandle, IntoElement, actions};
use project::search::SearchQuery;
pub use project_search::ProjectSearchView;
use ui::{ButtonStyle, IconButton, IconButtonShape};
use ui::{Tooltip, prelude::*};
use workspace::notifications::NotificationId;
use workspace::{Toast, Workspace};

pub use search_status_button::SEARCH_ICON;

use crate::project_search::ProjectSearchBar;

pub mod buffer_search;
pub mod project_search;
pub(crate) mod search_bar;
pub mod search_status_button;

pub fn init(cx: &mut App) {
    menu::init();
    buffer_search::init(cx);
    project_search::init(cx);
}

actions!(
    search,
    [
        /// Focuses on the search input field.
        FocusSearch,
        /// Toggles whole word matching.
        ToggleWholeWord,
        /// Toggles case-sensitive search.
        ToggleCaseSensitive,
        /// Toggles searching in ignored files.
        ToggleIncludeIgnored,
        /// Toggles regular expression mode.
        ToggleRegex,
        /// Toggles the replace interface.
        ToggleReplace,
        /// Toggles searching within selection only.
        ToggleSelection,
        /// Selects the next search match.
        SelectNextMatch,
        /// Selects the previous search match.
        SelectPreviousMatch,
        /// Selects all search matches.
        SelectAllMatches,
        /// Cycles through search modes.
        CycleMode,
        /// Navigates to the next query in search history.
        NextHistoryQuery,
        /// Navigates to the previous query in search history.
        PreviousHistoryQuery,
        /// Replaces all matches.
        ReplaceAll,
        /// Replaces the next match.
        ReplaceNext,
    ]
);

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
    pub struct SearchOptions: u8 {
        const NONE = 0;
        const WHOLE_WORD = 1 << SearchOption::WholeWord as u8;
        const CASE_SENSITIVE = 1 << SearchOption::CaseSensitive as u8;
        const INCLUDE_IGNORED = 1 << SearchOption::IncludeIgnored as u8;
        const REGEX = 1 << SearchOption::Regex as u8;
        const ONE_MATCH_PER_LINE = 1 << SearchOption::OneMatchPerLine as u8;
        /// If set, reverse direction when finding the active match
        const BACKWARDS = 1 << SearchOption::Backwards as u8;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SearchOption {
    WholeWord = 0,
    CaseSensitive,
    IncludeIgnored,
    Regex,
    OneMatchPerLine,
    Backwards,
}

pub(crate) enum SearchSource<'a, 'b> {
    Buffer,
    Project(&'a Context<'b, ProjectSearchBar>),
}

impl SearchOption {
    pub fn as_options(&self) -> SearchOptions {
        SearchOptions::from_bits(1 << *self as u8).unwrap()
    }

    pub fn label(&self) -> &'static str {
        match self {
            SearchOption::WholeWord => "Match Whole Words",
            SearchOption::CaseSensitive => "Match Case Sensitively",
            SearchOption::IncludeIgnored => "Also search files ignored by configuration",
            SearchOption::Regex => "Use Regular Expressions",
            SearchOption::OneMatchPerLine => "One Match Per Line",
            SearchOption::Backwards => "Search Backwards",
        }
    }

    pub fn icon(&self) -> ui::IconName {
        match self {
            SearchOption::WholeWord => ui::IconName::WholeWord,
            SearchOption::CaseSensitive => ui::IconName::CaseSensitive,
            SearchOption::IncludeIgnored => ui::IconName::Sliders,
            SearchOption::Regex => ui::IconName::Regex,
            _ => panic!("{self:?} is not a named SearchOption"),
        }
    }

    pub fn to_toggle_action(self) -> &'static dyn Action {
        match self {
            SearchOption::WholeWord => &ToggleWholeWord,
            SearchOption::CaseSensitive => &ToggleCaseSensitive,
            SearchOption::IncludeIgnored => &ToggleIncludeIgnored,
            SearchOption::Regex => &ToggleRegex,
            _ => panic!("{self:?} is not a toggle action"),
        }
    }

    pub(crate) fn as_button(
        &self,
        active: SearchOptions,
        search_source: SearchSource,
        focus_handle: FocusHandle,
    ) -> impl IntoElement {
        let action = self.to_toggle_action();
        let label = self.label();
        IconButton::new(
            (label, matches!(search_source, SearchSource::Buffer) as u32),
            self.icon(),
        )
        .map(|button| match search_source {
            SearchSource::Buffer => {
                let focus_handle = focus_handle.clone();
                button.on_click(move |_: &ClickEvent, window, cx| {
                    if !focus_handle.is_focused(window) {
                        window.focus(&focus_handle);
                    }
                    window.dispatch_action(action.boxed_clone(), cx);
                })
            }
            SearchSource::Project(cx) => {
                let options = self.as_options();
                button.on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                    this.toggle_search_option(options, window, cx);
                }))
            }
        })
        .style(ButtonStyle::Subtle)
        .shape(IconButtonShape::Square)
        .toggle_state(active.contains(self.as_options()))
        .tooltip({
            move |window, cx| Tooltip::for_action_in(label, action, &focus_handle, window, cx)
        })
    }
}

impl SearchOptions {
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

    pub fn from_settings(settings: &SearchSettings) -> SearchOptions {
        let mut options = SearchOptions::NONE;
        options.set(SearchOptions::WHOLE_WORD, settings.whole_word);
        options.set(SearchOptions::CASE_SENSITIVE, settings.case_sensitive);
        options.set(SearchOptions::INCLUDE_IGNORED, settings.include_ignored);
        options.set(SearchOptions::REGEX, settings.regex);
        options
    }
}

pub(crate) fn show_no_more_matches(window: &mut Window, cx: &mut App) {
    window.defer(cx, |window, cx| {
        struct NotifType();
        let notification_id = NotificationId::unique::<NotifType>();

        let Some(workspace) = window.root::<Workspace>().flatten() else {
            return;
        };
        workspace.update(cx, |workspace, cx| {
            workspace.show_toast(
                Toast::new(notification_id.clone(), "No more matches").autohide(),
                cx,
            );
        })
    });
}
