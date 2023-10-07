use std::marker::PhantomData;

use ui::prelude::*;
use ui::TrafficLights;

use crate::story::Story;

#[derive(Element)]
pub struct TrafficLightsStory<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync> TrafficLightsStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        Story::container(cx)
            .child(Story::title_for::<_, TrafficLights<S>>(cx))
            .child(Story::label(cx, "Default"))
            .child(TrafficLights::new())
            .child(Story::label(cx, "Unfocused"))
            .child(TrafficLights::new().window_has_focus(false))
    }
}
