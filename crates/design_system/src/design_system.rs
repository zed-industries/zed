use std::{marker::PhantomData, sync::Arc};

use gpui::{
    elements::{ContainerStyle, Label, MouseEventHandler, TooltipStyle},
    fonts::TextStyle,
    Action, CursorStyle, Element, ElementBox, MouseButton, View,
};
use settings::Settings;
use theme::{ContainedText, Interactive, Theme};

#[derive(Clone)]
pub struct ZDSButton {
    icon: String,
    interactions: Interactive<ContainedText>,
    text: TextStyle,
    container: ContainerStyle,
    tooltip_style: TooltipStyle, //TODO: Make optional
}

pub fn button<Tag: 'static, F, V>(
    region_id: usize,
    click_action: Box<dyn Action>,
    tooltip: &str,
    cx: &mut gpui::RenderContext<V>,
    style_for: F,
) -> ElementBox
where
    V: View,
    F: Fn(&Arc<Theme>) -> &ZDSButton,
{
    enum ZDSButton<T> {
        _PD(PhantomData<*const T>),
    }

    let style = style_for(&cx.global::<Settings>().theme).clone();

    MouseEventHandler::<ZDSButton<Tag>>::new(region_id as usize, cx, |state, _cx| {
        style.interactions.style_for(state, false);
        Label::new(style.icon.to_string(), style.text.clone())
            .contained()
            .with_style(style.container)
            .boxed()
    })
    .on_click(MouseButton::Left, {
        let action = click_action.boxed_clone();
        move |_, cx| cx.dispatch_any_action(action.boxed_clone())
    })
    .with_cursor_style(CursorStyle::PointingHand)
    .with_tooltip::<ZDSButton<Tag>, _>(
        region_id,
        tooltip.to_owned(),
        Some(click_action),
        style.tooltip_style.clone(),
        cx,
    )
    .boxed()
}
