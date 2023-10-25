use std::marker::PhantomData;

use crate::prelude::*;
use crate::Label;
use crate::LabelColor;

#[derive(Default, PartialEq)]
pub enum InputVariant {
    #[default]
    Ghost,
    Filled,
}

#[derive(Element)]
pub struct Input<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    placeholder: SharedString,
    value: String,
    state: InteractionState,
    variant: InputVariant,
    disabled: bool,
    is_active: bool,
}

impl<S: 'static + Send + Sync> Input<S> {
    pub fn new(placeholder: impl Into<SharedString>) -> Self {
        Self {
            state_type: PhantomData,
            placeholder: placeholder.into(),
            value: "".to_string(),
            state: InteractionState::default(),
            variant: InputVariant::default(),
            disabled: false,
            is_active: false,
        }
    }

    pub fn value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub fn state(mut self, state: InteractionState) -> Self {
        self.state = state;
        self
    }

    pub fn variant(mut self, variant: InputVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);

        let (input_bg, input_hover_bg, input_active_bg) = match self.variant {
            InputVariant::Ghost => (
                theme.ghost_element,
                theme.ghost_element_hover,
                theme.ghost_element_active,
            ),
            InputVariant::Filled => (
                theme.filled_element,
                theme.filled_element_hover,
                theme.filled_element_active,
            ),
        };

        let placeholder_label = Label::new(self.placeholder.clone()).color(if self.disabled {
            LabelColor::Disabled
        } else {
            LabelColor::Placeholder
        });

        let label = Label::new(self.value.clone()).color(if self.disabled {
            LabelColor::Disabled
        } else {
            LabelColor::Default
        });

        div()
            .id("input")
            .h_7()
            .w_full()
            .px_2()
            .border()
            .border_color(theme.transparent)
            .bg(input_bg)
            .hover(|style| style.bg(input_hover_bg))
            .active(|style| style.bg(input_active_bg))
            .flex()
            .items_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .text_sm()
                    .when(self.value.is_empty(), |this| this.child(placeholder_label))
                    .when(!self.value.is_empty(), |this| this.child(label)),
            )
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

    use super::*;

    #[derive(Element)]
    pub struct InputStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> InputStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, Input<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(div().flex().child(Input::new("Search")))
        }
    }
}
