use crate::prelude::*;
use crate::Label;
use crate::LabelColor;

#[derive(Default, PartialEq)]
pub enum InputVariant {
    #[default]
    Ghost,
    Filled,
}

#[derive(Component)]
pub struct Input {
    placeholder: SharedString,
    value: String,
    state: InteractionState,
    variant: InputVariant,
    disabled: bool,
    is_active: bool,
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

    fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
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

    #[derive(Component)]
    pub struct InputStory;

    impl InputStory {
        pub fn new() -> Self {
            Self
        }

        fn render<V: 'static>(self, _view: &mut V, cx: &mut ViewContext<V>) -> impl Component<V> {
            Story::container(cx)
                .child(Story::title_for::<_, Input>(cx))
                .child(Story::label(cx, "Default"))
                .child(div().flex().child(Input::new("Search")))
        }
    }
}
