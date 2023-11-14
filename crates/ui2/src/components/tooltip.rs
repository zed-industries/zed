use gpui::{div, Component, Div, ParentElement, Render, SharedString, Styled, ViewContext};
use theme2::ActiveTheme;

use crate::{h_stack, v_stack, Label, LabelColor, StyledExt};

use super::keybinding;

#[derive(Clone, Debug)]
pub struct TextTooltip {
    title: SharedString,
    meta: Option<SharedString>,
    keybinding: Option<SharedString>,
}

impl TextTooltip {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            meta: None,
            keybinding: None,
        }
    }

    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn keybinding(mut self, keybinding: impl Into<SharedString>) -> Self {
        self.keybinding = Some(keybinding.into());
        self
    }
}

impl Render for TextTooltip {
    type Element = Div<Self>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        v_stack()
            .elevation_2(cx)
            .font("Zed Sans")
            .text_ui_sm()
            .text_color(cx.theme().colors().text)
            .py_1()
            .px_2()
            .child(h_stack().child(self.title.clone()).when_some(
                self.keybinding.clone(),
                |this, keybinding| {
                    this.justify_between()
                        .child(Label::new(keybinding).color(LabelColor::Muted))
                },
            ))
            .when_some(self.meta.clone(), |this, meta| {
                this.child(Label::new(meta).color(LabelColor::Muted))
            })
    }
}
