use design_system::DesignSystem;
use gpui::{Action, ElementBox, RenderContext, View};
use workspace::searchable::Direction;

use crate::{SearchOption, SelectNextMatch, SelectPrevMatch};

trait SearchActionThemeExt {
    fn to_action(&self) -> Box<dyn Action>;
}

impl SearchActionThemeExt for Direction {
    fn to_action(&self) -> Box<dyn Action> {
        match self {
            Direction::Prev => Box::new(SelectPrevMatch) as Box<dyn Action>,
            Direction::Next => Box::new(SelectNextMatch) as Box<dyn Action>,
        }
    }
}

pub fn nav_button<V: View>(direction: Direction, cx: &mut RenderContext<V>) -> ElementBox {
    enum NavButton {}
    DesignSystem::<NavButton>::label_button(
        direction as usize,
        false,
        direction.to_action(),
        cx,
        |theme| match direction {
            Direction::Prev => &theme.search.previous,
            Direction::Next => &theme.search.next,
        },
    )
}

pub fn option_button<V: View>(
    option: SearchOption,
    enabled: bool,
    cx: &mut RenderContext<V>,
) -> ElementBox {
    enum OptionButton {}
    DesignSystem::<OptionButton>::label_button(
        option as usize,
        enabled,
        option.to_toggle_action(),
        cx,
        |theme| match option {
            SearchOption::WholeWord => &theme.search.whole_word,
            SearchOption::CaseSensitive => &theme.search.case_sensitive,
            SearchOption::Regex => &theme.search.regex,
        },
    )
}
