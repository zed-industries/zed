use gpui::{
    elements::{Flex, Label, MouseEventHandler, ParentElement, Svg},
    Action, CursorStyle, Element, MouseButton, View,
};
use settings::Settings;
use theme::buttons::ButtonStyle;

use crate::DesignSystem;

impl<Tag: 'static> DesignSystem<Tag> {
    pub fn button<V>(
        region_id: usize,
        click_action: Box<dyn Action>,
        button: ButtonStyle,
        cx: &mut gpui::RenderContext<V>,
    ) -> MouseEventHandler<Tag>
    where
        V: View,
    {
        Self::toggleable_button(region_id, false, click_action, button, cx)
    }

    pub fn toggleable_button<V>(
        region_id: usize,
        active: bool,
        click_action: Box<dyn Action>,
        button: ButtonStyle,
        cx: &mut gpui::RenderContext<V>,
    ) -> MouseEventHandler<Tag>
    where
        V: View,
    {
        let tooltip_style = cx.global::<Settings>().theme.tooltip.to_owned();

        MouseEventHandler::<Tag>::new(region_id as usize, cx, |state, cx| {
            let button_style = button.container.style_for(state, active).clone();

            Flex::new(gpui::Axis::Horizontal)
                .with_children(button.icon.map(|icon| {
                    Svg::new(icon.location)
                        .with_color(icon.color)
                        .constrained()
                        .with_width(icon.size)
                        .with_height(icon.size)
                        .boxed()
                }))
                .with_children(
                    button
                        .label
                        .map(|label| Label::new(label, button_style.text).boxed()),
                )
                .contained()
                .with_style(button_style.container)
                .with_tooltip::<Tag, _>(
                    region_id,
                    button.tooltip_text,
                    Some(click_action.boxed_clone()),
                    tooltip_style,
                    cx,
                )
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .action_on_click(MouseButton::Left, click_action)
    }
}
