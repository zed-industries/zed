use gpui::KeyContext;
use ui::{Context, InteractiveElement, IntoElement, ParentElement, Render, Window, v_flex};

use crate::project_search_picker::TextPicker;

impl Render for TextPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // let key_context = self.picker.read(cx).delegate.key_context(window, cx);
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("TextPicker");

        v_flex()
            .key_context(key_context)
            // .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            // .on_action(cx.listener(Self::handle_select_prev))
            // .on_action(cx.listener(Self::handle_filter_toggle_menu))
            // .on_action(cx.listener(Self::handle_split_toggle_menu))
            // .on_action(cx.listener(Self::handle_toggle_ignored))
            // .on_action(cx.listener(Self::go_to_file_split_left))
            // .on_action(cx.listener(Self::go_to_file_split_right))
            // .on_action(cx.listener(Self::go_to_file_split_up))
            // .on_action(cx.listener(Self::go_to_file_split_down))
            .child(self.picker.clone())
    }
}
