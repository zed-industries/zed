use gpui::{Div, ParentComponent, Render, SharedString, Styled, ViewContext};
use theme2::ActiveTheme;

use crate::prelude::*;
use crate::{h_stack, v_stack, KeyBinding, Label, LabelSize, StyledExt, TextColor};

pub struct TextTooltip {
    title: SharedString,
    meta: Option<SharedString>,
    key_binding: Option<KeyBinding>,
}

impl TextTooltip {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            meta: None,
            key_binding: None,
        }
    }

    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn key_binding(mut self, key_binding: impl Into<Option<KeyBinding>>) -> Self {
        self.key_binding = key_binding.into();
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
            .child(
                h_stack()
                    .child(self.title.clone())
                    .when_some(self.key_binding.clone(), |this, key_binding| {
                        this.justify_between().child(key_binding)
                    }),
            )
            .when_some(self.meta.clone(), |this, meta| {
                this.child(
                    Label::new(meta)
                        .size(LabelSize::Small)
                        .color(TextColor::Muted),
                )
            })
    }
}
