use gpui::{
    elements::{MouseEventHandler, Svg},
    platform::{CursorStyle, MouseButton},
    scene::MouseClick,
    Action, AnyElement, Element, EventContext, View, ViewContext,
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
