use std::borrow::Cow;

use gpui::{
    elements::{Label, MouseEventHandler, Svg},
    platform::{CursorStyle, MouseButton},
    scene::{CornerRadii, MouseClick},
    Action, AnyElement, Element, EventContext, View, ViewContext,
};
use workspace::searchable::Direction;

use crate::{
    mode::{SearchMode, Side},
    SelectNextMatch, SelectPrevMatch,
};

pub(super) fn render_close_button<V: View>(
    tooltip: &'static str,
    theme: &theme::Search,
    cx: &mut ViewContext<V>,
    on_click: impl Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    dismiss_action: Option<Box<dyn Action>>,
) -> AnyElement<V> {
    let tooltip_style = theme::current(cx).tooltip.clone();

    enum CloseButton {}
    MouseEventHandler::new::<CloseButton, _>(0, cx, |state, _| {
        let style = theme.dismiss_button.style_for(state);
        Svg::new("icons/x_mark_8.svg")
            .with_color(style.color)
            .constrained()
            .with_width(style.icon_width)
            .aligned()
            .contained()
            .with_style(style.container)
            .constrained()
            .with_height(theme.search_bar_row_height)
    })
    .on_click(MouseButton::Left, on_click)
    .with_cursor_style(CursorStyle::PointingHand)
    .with_tooltip::<CloseButton>(0, tooltip.to_string(), dismiss_action, tooltip_style, cx)
    .into_any()
}

pub(super) fn render_nav_button<V: View>(
    icon: &'static str,
    direction: Direction,
    active: bool,
    on_click: impl Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    cx: &mut ViewContext<V>,
) -> AnyElement<V> {
    let action: Box<dyn Action>;
    let tooltip;

    match direction {
        Direction::Prev => {
            action = Box::new(SelectPrevMatch);
            tooltip = "Select Previous Match";
        }
        Direction::Next => {
            action = Box::new(SelectNextMatch);
            tooltip = "Select Next Match";
        }
    };
    let tooltip_style = theme::current(cx).tooltip.clone();
    let cursor_style = if active {
        CursorStyle::PointingHand
    } else {
        CursorStyle::default()
    };
    enum NavButton {}
    MouseEventHandler::new::<NavButton, _>(direction as usize, cx, |state, cx| {
        let theme = theme::current(cx);
        let style = theme
            .search
            .nav_button
            .in_state(active)
            .style_for(state)
            .clone();
        let mut container_style = style.container.clone();
        let label = Label::new(icon, style.label.clone()).aligned().contained();
        container_style.corner_radii = match direction {
            Direction::Prev => CornerRadii {
                bottom_right: 0.,
                top_right: 0.,
                ..container_style.corner_radii
            },
            Direction::Next => CornerRadii {
                bottom_left: 0.,
                top_left: 0.,
                ..container_style.corner_radii
            },
        };
        if direction == Direction::Prev {
            // Remove right border so that when both Next and Prev buttons are
            // next to one another, there's no double border between them.
            container_style.border.right = false;
        }
        label.with_style(container_style)
    })
    .on_click(MouseButton::Left, on_click)
    .with_cursor_style(cursor_style)
    .with_tooltip::<NavButton>(
        direction as usize,
        tooltip.to_string(),
        Some(action),
        tooltip_style,
        cx,
    )
    .into_any()
}

pub(crate) fn render_search_mode_button<V: View>(
    mode: SearchMode,
    is_active: bool,
    on_click: impl Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    cx: &mut ViewContext<V>,
) -> AnyElement<V> {
    let tooltip_style = theme::current(cx).tooltip.clone();
    enum SearchModeButton {}
    MouseEventHandler::new::<SearchModeButton, _>(mode.region_id(), cx, |state, cx| {
        let theme = theme::current(cx);
        let mut style = theme
            .search
            .mode_button
            .in_state(is_active)
            .style_for(state)
            .clone();
        style.container.border.left = mode.border_left();
        style.container.border.right = mode.border_right();

        let label = Label::new(mode.label(), style.text.clone())
            .aligned()
            .contained();
        let mut container_style = style.container.clone();
        if let Some(button_side) = mode.button_side() {
            if button_side == Side::Left {
                container_style.corner_radii = CornerRadii {
                    bottom_right: 0.,
                    top_right: 0.,
                    ..container_style.corner_radii
                };
                label.with_style(container_style)
            } else {
                container_style.corner_radii = CornerRadii {
                    bottom_left: 0.,
                    top_left: 0.,
                    ..container_style.corner_radii
                };
                label.with_style(container_style)
            }
        } else {
            container_style.corner_radii = CornerRadii::default();
            label.with_style(container_style)
        }
        .constrained()
        .with_height(theme.search.search_bar_row_height)
    })
    .on_click(MouseButton::Left, on_click)
    .with_cursor_style(CursorStyle::PointingHand)
    .with_tooltip::<SearchModeButton>(
        mode.region_id(),
        mode.tooltip_text().to_owned(),
        Some(mode.activate_action()),
        tooltip_style,
        cx,
    )
    .into_any()
}

pub(crate) fn render_option_button_icon<V: View>(
    is_active: bool,
    icon: &'static str,
    id: usize,
    label: impl Into<Cow<'static, str>>,
    action: Box<dyn Action>,
    on_click: impl Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    cx: &mut ViewContext<V>,
) -> AnyElement<V> {
    let tooltip_style = theme::current(cx).tooltip.clone();
    MouseEventHandler::new::<V, _>(id, cx, |state, cx| {
        let theme = theme::current(cx);
        let style = theme
            .search
            .option_button
            .in_state(is_active)
            .style_for(state);
        Svg::new(icon)
            .with_color(style.color.clone())
            .constrained()
            .with_width(style.icon_width)
            .contained()
            .with_style(style.container)
            .constrained()
            .with_height(theme.search.option_button_height)
            .with_width(style.button_width)
    })
    .on_click(MouseButton::Left, on_click)
    .with_cursor_style(CursorStyle::PointingHand)
    .with_tooltip::<V>(id, label, Some(action), tooltip_style, cx)
    .into_any()
}
