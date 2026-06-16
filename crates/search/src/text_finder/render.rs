use gpui::KeyContext;
use ui::{Context, InteractiveElement, IntoElement, ParentElement, Render, Window, v_flex};

use super::TextFinder;

impl Render for TextFinder {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("TextPicker");

        v_flex()
            .key_context(key_context)
            .on_action(cx.listener(Self::to_project_search))
            .child(self.picker.clone())
    }
}
