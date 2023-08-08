use gpui::{
    elements::{Flex, Label, MouseEventHandler, ParentElement, Svg},
    platform::{CursorStyle, MouseButton},
    scene::MouseClick,
    Action, AnyElement, Element, EventContext, View, ViewContext,
};
use workspace::searchable::Direction;

use crate::{
    elements::ButtonSide,
    mode::{SearchMode, Side},
    SearchOptions, SelectNextMatch, SelectPrevMatch,
};

pub(super) fn render_close_button<V: View>(
    theme: &theme::Search,
    cx: &mut ViewContext<V>,
    on_click: impl Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    dismiss_action: Option<Box<dyn Action>>,
) -> AnyElement<V> {
    let tooltip = "Dismiss Buffer Search";
    let tooltip_style = theme::current(cx).tooltip.clone();

    enum CloseButton {}
    MouseEventHandler::<CloseButton, _>::new(0, cx, |state, _| {
        let style = theme.dismiss_button.style_for(state);
        Svg::new("icons/x_mark_8.svg")
            .with_color(style.color)
            .constrained()
            .with_width(style.icon_width)
            .aligned()
            .constrained()
            .with_width(style.button_width)
            .contained()
            .with_style(style.container)
    })
    .on_click(MouseButton::Left, on_click)
    .with_cursor_style(CursorStyle::PointingHand)
    .with_tooltip::<CloseButton>(0, tooltip.to_string(), dismiss_action, tooltip_style, cx)
    .into_any()
}

pub(super) fn render_nav_button<V: View>(
    icon: &'static str,
    direction: Direction,
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

    enum NavButton {}
    MouseEventHandler::<NavButton, _>::new(direction as usize, cx, |state, cx| {
        let theme = theme::current(cx);
        let mut style = theme.search.nav_button.style_for(state).clone();

        match direction {
            Direction::Prev => style.container.border.left = false,
            Direction::Next => style.container.border.right = false,
        };
        let label = Label::new(icon, style.label.clone())
            .contained()
            .with_style(style.container.clone());
        match direction {
            Direction::Prev => Flex::row()
                .with_child(
                    ButtonSide::left(
                        style
                            .clone()
                            .container
                            .background_color
                            .unwrap_or_else(gpui::color::Color::transparent_black),
                    )
                    .with_border(style.container.border.width, style.container.border.color)
                    .contained()
                    .constrained()
                    .with_max_width(theme.search.mode_filling_width),
                )
                .with_child(label)
                .constrained()
                .with_height(theme.workspace.toolbar.height),
            Direction::Next => Flex::row()
                .with_child(label)
                .with_child(
                    ButtonSide::right(
                        style
                            .clone()
                            .container
                            .background_color
                            .unwrap_or_else(gpui::color::Color::transparent_black),
                    )
                    .with_border(style.container.border.width, style.container.border.color)
                    .contained()
                    .constrained()
                    .with_max_width(theme.search.mode_filling_width),
                )
                .constrained()
                .with_height(theme.workspace.toolbar.height),
        }
    })
    .on_click(
        MouseButton::Left,
        on_click, /*move |_, this, cx| {
                  if let Some(search) = this.active_project_search.as_ref() {
                      search.update(cx, |search, cx| search.select_match(direction, cx));
                      }*/
    )
    .with_cursor_style(CursorStyle::PointingHand)
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
    // let is_active = if let Some(search) = self.active_project_search.as_ref() {
    //     let search = search.read(cx);
    //     search.current_mode == mode
    // } else {
    //     false
    // };

    enum SearchModeButton {}
    MouseEventHandler::<SearchModeButton, _>::new(mode.region_id(), cx, |state, cx| {
        let theme = theme::current(cx);
        let mut style = theme
            .search
            .mode_button
            .in_state(is_active)
            .style_for(state)
            .clone();

        let label = Label::new(mode.label(), style.text.clone())
            .contained()
            .with_style(style.container);

        if let Some(button_side) = mode.button_side() {
            style.container.border.left = mode.border_left();
            style.container.border.right = mode.border_right();

            if button_side == Side::Left {
                Flex::row()
                    .align_children_center()
                    .with_child(
                        ButtonSide::left(
                            style
                                .container
                                .background_color
                                .unwrap_or_else(gpui::color::Color::transparent_black),
                        )
                        .with_border(style.container.border.width, style.container.border.color)
                        .contained()
                        .constrained()
                        .with_max_width(theme.search.mode_filling_width),
                    )
                    .with_child(label)
                    .into_any()
            } else {
                Flex::row()
                    .align_children_center()
                    .with_child(label)
                    .with_child(
                        ButtonSide::right(
                            style
                                .container
                                .background_color
                                .unwrap_or_else(gpui::color::Color::transparent_black),
                        )
                        .with_border(style.container.border.width, style.container.border.color)
                        .contained()
                        .constrained()
                        .with_max_width(theme.search.mode_filling_width),
                    )
                    .into_any()
            }
        } else {
            label.into_any()
        }
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
    option: SearchOptions,
    on_click: impl Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
    cx: &mut ViewContext<V>,
) -> AnyElement<V> {
    let tooltip_style = theme::current(cx).tooltip.clone();
    MouseEventHandler::<V, _>::new(option.bits as usize, cx, |state, cx| {
        let theme = theme::current(cx);
        let style = theme
            .search
            .option_button
            .in_state(is_active)
            .style_for(state);
        Svg::new(icon)
            .with_color(style.text.color.clone())
            .contained()
            .with_style(style.container)
            .constrained()
    })
    .on_click(MouseButton::Left, on_click)
    .with_cursor_style(CursorStyle::PointingHand)
    .with_tooltip::<V>(
        option.bits as usize,
        format!("Toggle {}", option.label()),
        Some(option.to_toggle_action()),
        tooltip_style,
        cx,
    )
    .into_any()
}
