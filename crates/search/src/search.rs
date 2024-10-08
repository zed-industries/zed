use bitflags::bitflags;
pub use buffer_search::BufferSearchBar;
use editor::SearchSettings;
use gpui::{actions, Action, AppContext, FocusHandle, IntoElement};
use project::search::SearchQuery;
pub use project_search::ProjectSearchView;
use ui::{prelude::*, Tooltip};
use ui::{ButtonStyle, IconButton, IconButtonShape};
use workspace::notifications::NotificationId;
use workspace::{Toast, Workspace};

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
        ToggleSelection,
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
            SearchOptions::WHOLE_WORD => "Match Whole Words",
            SearchOptions::CASE_SENSITIVE => "Match Case Sensitively",
            SearchOptions::INCLUDE_IGNORED => "Also search files ignored by configuration",
            SearchOptions::REGEX => "Use Regular Expressions",
            _ => panic!("{:?} is not a named SearchOption", self),
        }
    }

    pub fn icon(&self) -> ui::IconName {
        match *self {
            SearchOptions::WHOLE_WORD => ui::IconName::WholeWord,
            SearchOptions::CASE_SENSITIVE => ui::IconName::CaseSensitive,
            SearchOptions::INCLUDE_IGNORED => ui::IconName::Sliders,
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

    pub fn from_settings(settings: &SearchSettings) -> SearchOptions {
        let mut options = SearchOptions::NONE;
        options.set(SearchOptions::WHOLE_WORD, settings.whole_word);
        options.set(SearchOptions::CASE_SENSITIVE, settings.case_sensitive);
        options.set(SearchOptions::INCLUDE_IGNORED, settings.include_ignored);
        options.set(SearchOptions::REGEX, settings.regex);
        options
    }

    pub fn as_button(
        &self,
        active: bool,
        focus_handle: FocusHandle,
        action: impl Fn(&gpui::ClickEvent, &mut WindowContext) + 'static,
    ) -> impl IntoElement {
        IconButton::new(self.label(), self.icon())
            .on_click(action)
            .style(ButtonStyle::Subtle)
            .shape(IconButtonShape::Square)
            .selected(active)
            .tooltip({
                let action = self.to_toggle_action();
                let label = self.label();
                move |cx| Tooltip::for_action_in(label, &*action, &focus_handle, cx)
            })
    }
}

pub(crate) fn show_no_more_matches(cx: &mut WindowContext) {
    cx.defer(|cx| {
        struct NotifType();
        let notification_id = NotificationId::unique::<NotifType>();
        let Some(workspace) = cx.window_handle().downcast::<Workspace>() else {
            return;
        };
        workspace
            .update(cx, |workspace, cx| {
                workspace.show_toast(
                    Toast::new(notification_id.clone(), "No more matches").autohide(),
                    cx,
                );
            })
            .ok();
    });
}
