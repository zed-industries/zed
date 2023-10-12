use std::marker::PhantomData;

use crate::prelude::*;
use crate::theme;

#[derive(Element, Clone)]
pub struct Details<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
    text: &'static str,
    meta: Option<&'static str>,
}

impl<S: 'static + Send + Sync + Clone> Details<S> {
    pub fn new(text: &'static str) -> Self {
        Self {
            state_type: PhantomData,
            text,
            meta: None,
        }
    }

    pub fn meta_text(mut self, meta: &'static str) -> Self {
        self.meta = Some(meta);
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);

        div()
            // .flex()
            // .w_full()
            .p_1()
            .gap_0p5()
            .text_xs()
            .text_color(theme.lowest.base.default.foreground)
            .child(self.text)
            .children(self.meta.map(|m| m))
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use crate::Story;

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

        fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
            Story::container(cx)
                .child(Story::title_for::<_, Details<S>>(cx))
                .child(Story::label(cx, "Default"))
                .child(Details::new("The quick brown fox jumps over the lazy dog"))
                .child(Story::label(cx, "With meta"))
                .child(
                    Details::new("The quick brown fox jumps over the lazy dog")
                        .meta_text("Sphinx of black quartz, judge my vow."),
                )
        }
    }
}
