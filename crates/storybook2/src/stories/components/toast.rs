use std::marker::PhantomData;

use ui::prelude::*;
use ui::{Label, Toast, ToastOrigin};

use crate::story::Story;

#[derive(Element)]
pub struct ToastStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> ToastStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, Toast<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(Toast::new(
                ToastOrigin::Bottom,
                |_, _| vec![Label::new("label").into_any()],
                Box::new(()),
            ))
    }
}
