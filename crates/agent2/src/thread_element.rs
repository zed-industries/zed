use gpui::{App, Entity, SharedString, Window, div, prelude::*};

use crate::Thread;

pub struct ThreadElement {
    thread: Entity<Thread>,
}

impl ThreadElement {
    pub fn new(thread: Entity<Thread>) -> Self {
        Self { thread }
    }

    pub fn title(&self, cx: &App) -> SharedString {
        self.thread.read(cx).title()
    }

    pub fn cancel(&self, window: &mut Window, cx: &mut Context<Self>) {
        // todo!
    }
}

impl Render for ThreadElement {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("agent 2")
    }
}
