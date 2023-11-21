use crate::{prelude::*, Label};
use gpui::{prelude::*, Div, RenderOnce, Stateful};

#[derive(Default, PartialEq)]
pub enum InputVariant {
    #[default]
    Ghost,
    Filled,
}

#[derive(RenderOnce)]
pub struct Input {
    placeholder: SharedString,
    value: String,
    state: InteractionState,
    variant: InputVariant,
    disabled: bool,
    is_active: bool,
}

impl Component for Input {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        let (input_bg, input_hover_bg, input_active_bg) = match self.variant {
            InputVariant::Ghost => (
                cx.theme().colors().ghost_element_background,
                cx.theme().colors().ghost_element_hover,
                cx.theme().colors().ghost_element_active,
            ),
            InputVariant::Filled => (
                cx.theme().colors().element_background,
                cx.theme().colors().element_hover,
                cx.theme().colors().element_active,
            ),
        };

        let placeholder_label = Label::new(self.placeholder.clone()).color(if self.disabled {
            TextColor::Disabled
        } else {
            TextColor::Placeholder
        });

        let label = Label::new(self.value.clone()).color(if self.disabled {
            TextColor::Disabled
        } else {
            TextColor::Default
        });

        div()
            .id("input")
            .h_7()
            .w_full()
            .px_2()
            .border()
            .border_color(cx.theme().styles.system.transparent)
            .bg(input_bg)
            .hover(|style| style.bg(input_hover_bg))
            .active(|style| style.bg(input_active_bg))
            .flex()
            .items_center()
            .child(div().flex().items_center().text_ui_sm().map(move |this| {
                if self.value.is_empty() {
                    this.child(placeholder_label)
                } else {
                    this.child(label)
                }
            }))
    }
}

impl Input {
    pub fn new(placeholder: impl Into<SharedString>) -> Self {
        Self {
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
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use super::*;
    use crate::Story;
    use gpui::{Div, Render};

    pub struct InputStory;

    impl Render for InputStory {
        type Element = Div;

        fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
            Story::container(cx)
                .child(Story::title_for::<Input>(cx))
                .child(Story::label(cx, "Default"))
                .child(div().flex().child(Input::new("Search")))
        }
    }
}
