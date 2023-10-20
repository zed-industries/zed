use std::marker::PhantomData;

use crate::{prelude::*, v_stack, ButtonGroup};

#[derive(Element)]
pub struct Details<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    text: &'static str,
    meta: Option<&'static str>,
    actions: Option<ButtonGroup<S>>,
}

impl<S: 'static + Send + Sync> Details<S> {
    pub fn new(text: &'static str) -> Self {
        Self {
            state_type: PhantomData,
            text,
            meta: None,
            actions: None,
        }
    }

    pub fn meta_text(mut self, meta: &'static str) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn actions(mut self, actions: ButtonGroup<S>) -> Self {
        self.actions = Some(actions);
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let color = ThemeColor::new(cx);

        v_stack()
            .p_1()
            .gap_0p5()
            .text_xs()
            .text_color(color.text)
            .size_full()
            .child(self.text)
            .children(self.meta.map(|m| m))
            .children(self.actions.take().map(|a| a))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::{Button, Story};

    use super::*;

    #[derive(Element)]
    pub struct DetailsStory<S: 'static + Send + Sync + Clone> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync + Clone> DetailsStory<S> {
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
                .child(Story::title_for::<_, Details<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Details::new("The quick brown fox jumps over the lazy dog"))
                .child(Story::label(cx, "With meta"))
                .child(
                    Details::new("The quick brown fox jumps over the lazy dog")
                        .meta_text("Sphinx of black quartz, judge my vow."),
                )
                .child(Story::label(cx, "With meta and actions"))
                .child(
                    Details::new("The quick brown fox jumps over the lazy dog")
                        .meta_text("Sphinx of black quartz, judge my vow.")
                        .actions(ButtonGroup::new(vec![
                            Button::new("Decline"),
                            Button::new("Accept").variant(crate::ButtonVariant::Filled),
                        ])),
                )
        }
    }
}
