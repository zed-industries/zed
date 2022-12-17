pub use buffer_search::BufferSearchBar;
use design_system::DesignSystem;
use gpui::{actions, Action, Element, ElementBox, MutableAppContext, RenderContext, View};
pub use project_search::{ProjectSearchBar, ProjectSearchView};
use settings::Settings;
use theme::buttons::ButtonStyle;
use workspace::searchable::Direction;

pub mod buffer_search;

pub mod project_search;

pub fn init(cx: &mut MutableAppContext) {
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
        SelectPrevMatch
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

    fn button_theme(&self, theme: &theme::Theme) -> theme::buttons::ButtonStyle {
        match self {
            SearchOption::WholeWord => theme.search.whole_word.to_owned(),
            SearchOption::CaseSensitive => theme.search.case_sensitive.to_owned(),
            SearchOption::Regex => theme.search.regex.to_owned(),
        }
    }
}

struct DirectionButton(Direction);

impl DirectionButton {
    fn click_action(&self) -> Box<dyn gpui::Action> {
        match self.0 {
            Direction::Prev => Box::new(SelectPrevMatch),
            Direction::Next => Box::new(SelectNextMatch),
        }
    }
    fn button_theme(&self, theme: &theme::Theme) -> ButtonStyle {
        match self.0 {
            Direction::Prev => theme.search.previous.to_owned(),
            Direction::Next => theme.search.next.to_owned(),
        }
    }
}

fn option_button<V: View>(
    option: SearchOption,
    enabled: bool,
    cx: &mut RenderContext<V>,
) -> ElementBox {
    enum OptionButton {}

    DesignSystem::<OptionButton>::toggleable_button(
        option as usize,
        enabled,
        option.to_toggle_action(),
        option.button_theme(&cx.global::<Settings>().theme),
        cx,
    )
    .boxed()
}

fn direction_button<V: View>(direction: Direction, cx: &mut RenderContext<V>) -> ElementBox {
    enum NavButton {}
    let action = DirectionButton(direction).click_action();
    let button = DirectionButton(direction).button_theme(&cx.global::<Settings>().theme);
    DesignSystem::<NavButton>::button(direction as usize, action, button, cx).boxed()
}
