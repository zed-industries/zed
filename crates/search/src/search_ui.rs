use design_system::DesignSystem;
use gpui::{Action, ElementBox, RenderContext, View};
use workspace::searchable::Direction;

use crate::{SearchOption, SelectNextMatch, SelectPrevMatch};

pub fn nav_button<V: View>(direction: Direction, cx: &mut RenderContext<V>) -> ElementBox {
    let action: Box<dyn Action>;
    match direction {
        Direction::Prev => {
            action = Box::new(SelectPrevMatch);
        }
        Direction::Next => {
            action = Box::new(SelectNextMatch);
        }
    };

    enum NavButton {}
    DesignSystem::<NavButton>::label_button(direction as usize, false, action, cx, |theme| {
        match direction {
            Direction::Prev => &theme.search.previous,
            Direction::Next => &theme.search.next,
        }
    })
}

pub fn option_button<V: View>(
    option: SearchOption,
    enabled: bool,
    cx: &mut RenderContext<V>,
) -> ElementBox {
    DesignSystem::<V>::label_button(
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
