use std::{marker::PhantomData, sync::Arc};

use gpui::{
    elements::{Label, MouseEventHandler},
    Action, CursorStyle, Element, ElementBox, MouseButton, View,
};
use settings::Settings;
use theme::{design_system::LabelButton, Theme};

pub enum DesignSystem<Tag> {
    _PD(PhantomData<*const Tag>),
}
impl<Tag: 'static> DesignSystem<Tag> {
    pub fn label_button<V, F>(
        region_id: usize,
        active: bool,
        click_action: Box<dyn Action>,
        cx: &mut gpui::RenderContext<V>,
        style_for: F,
    ) -> ElementBox
    where
        V: View,
        F: Fn(&Arc<Theme>) -> &LabelButton,
    {
        enum LabelButton<T> {
            _PD(PhantomData<*const T>),
        }

        let (tooltip_style, style) = {
            let theme = &cx.global::<Settings>().theme;
            let tooltip_style = theme.tooltip.to_owned();
            let style = style_for(theme).to_owned();
            (tooltip_style, style)
        };

        MouseEventHandler::<LabelButton<Tag>>::new(region_id as usize, cx, |state, _cx| {
            style.interactions.style_for(state, active);
            Label::new(style.label, style.text)
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .on_click(MouseButton::Left, {
            let action = click_action.boxed_clone();
            move |_, cx| cx.dispatch_any_action(action.boxed_clone())
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .with_tooltip::<LabelButton<Tag>, _>(
            region_id,
            style.tooltip_text,
            Some(click_action),
            tooltip_style,
            cx,
        )
        .boxed()
    }
}
