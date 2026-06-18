use gpui::KeyContext;
use ui::{Context, InteractiveElement, IntoElement, ParentElement, Render, Window, v_flex};

use super::TextFinder;

impl Render for TextFinder {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("TextFinder");

        v_flex()
            .key_context(key_context)
            .on_action(cx.listener(Self::to_project_search))
            .on_action(cx.listener(Self::split_left))
            .on_action(cx.listener(Self::split_right))
            .on_action(cx.listener(Self::split_up))
            .on_action(cx.listener(Self::split_down))
            .child(self.picker.clone())
    }
}
