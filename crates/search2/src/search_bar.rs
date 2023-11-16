use std::{borrow::Cow, sync::Arc};

use gpui::{div, Action, Component, ViewContext};
use ui::{Button, ButtonVariant, IconButton};
use workspace::searchable::Direction;

use crate::mode::SearchMode;

pub(super) fn render_nav_button<V: 'static>(
    icon: ui::Icon,
    active: bool,
    on_click: impl Fn(&mut V, &mut ViewContext<V>) + 'static + Send + Sync,
) -> impl Component<V> {
    // let tooltip_style = cx.theme().tooltip.clone();
    // let cursor_style = if active {
    //     CursorStyle::PointingHand
    // } else {
    //     CursorStyle::default()
    // };
    // enum NavButton {}
    IconButton::new("search-nav-button", icon).on_click(on_click)
}

pub(crate) fn render_search_mode_button<V: 'static>(
    mode: SearchMode,
    is_active: bool,
    on_click: impl Fn(&mut V, &mut ViewContext<V>) + 'static + Send + Sync,
) -> Button<V> {
    let button_variant = if is_active {
        ButtonVariant::Filled
    } else {
        ButtonVariant::Ghost
    };

    Button::new(mode.label())
        .on_click(Arc::new(on_click))
        .variant(button_variant)
}

pub(crate) fn render_option_button_icon<V: 'static>(
    is_active: bool,
    icon: &'static str,
    id: usize,
    label: impl Into<Cow<'static, str>>,
    action: Box<dyn Action>,
) -> impl Component<V> {
    //let tooltip_style = cx.theme().tooltip.clone();
    div()
    // MouseEventHandler::new::<V, _>(id, cx, |state, cx| {
    //     let theme = cx.theme();
    //     let style = theme
    //         .search
    //         .option_button
    //         .in_state(is_active)
    //         .style_for(state);
    //     Svg::new(icon)
    //         .with_color(style.color.clone())
    //         .constrained()
    //         .with_width(style.icon_width)
    //         .contained()
    //         .with_style(style.container)
    //         .constrained()
    //         .with_height(theme.search.option_button_height)
    //         .with_width(style.button_width)
    // })
    // .on_click(MouseButton::Left, on_click)
    // .with_cursor_style(CursorStyle::PointingHand)
    // .with_tooltip::<V>(id, label, Some(action), tooltip_style, cx)
    // .into_any()
}
