use std::marker::PhantomData;

use ui::prelude::*;
use ui::WorkspaceElement;

#[derive(Element)]
pub struct WorkspaceStory<S: 'static + Send + Sync + Clone> {
    state_type: PhantomData<S>,
}

impl<S: 'static + Send + Sync + Clone> WorkspaceStory<S> {
    pub fn new() -> Self {
        Self {
            state_type: PhantomData,
        }
    }

    fn render(&mut self, cx: &mut ViewContext<S>) -> impl Element<State = S> {
        // Just render the workspace without any story boilerplate.
        WorkspaceElement::new()
    }
}
