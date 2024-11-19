use assistant::assistant_settings::AssistantSettings;
use collections::HashMap;
use editor::{ActiveLineTrailerProvider, Editor, EditorMode};
use gpui::{AnyWindowHandle, AppContext, ViewContext, WeakView, WindowContext};
use settings::{Settings, SettingsStore};
use std::{cell::RefCell, rc::Rc};
use theme::ActiveTheme;
use ui::prelude::*;
use workspace::Workspace;

pub fn init(cx: &mut AppContext) {
    let editors: Rc<RefCell<HashMap<WeakView<Editor>, AnyWindowHandle>>> = Rc::default();

    cx.observe_new_views({
        let editors = editors.clone();
        move |_: &mut Workspace, cx: &mut ViewContext<Workspace>| {
            let workspace_handle = cx.view().clone();
            cx.subscribe(&workspace_handle, {
                let editors = editors.clone();
                move |_, _, event, cx| match event {
                    workspace::Event::ItemAdded { item } => {
                        if let Some(editor) = item.act_as::<Editor>(cx) {
                            if editor.read(cx).mode() != EditorMode::Full {
                                return;
                            }

                            cx.on_release({
                                let editor_handle = editor.downgrade();
                                let editors = editors.clone();
                                move |_, _, _| {
                                    editors.borrow_mut().remove(&editor_handle);
                                }
                            })
                            .detach();
                            editors
                                .borrow_mut()
                                .insert(editor.downgrade(), cx.window_handle());

                            let show_hints = should_show_hints(cx);
                            editor.update(cx, |editor, cx| {
                                assign_active_line_trailer_provider(editor, show_hints, cx)
                            })
                        }
                    }
                    _ => {}
                }
            })
            .detach();
        }
    })
    .detach();

    let mut show_hints = AssistantSettings::get_global(cx).show_hints;
    cx.observe_global::<SettingsStore>(move |cx| {
        let new_show_hints = should_show_hints(cx);
        if new_show_hints != show_hints {
            show_hints = new_show_hints;
            for (editor, window) in editors.borrow().iter() {
                _ = window.update(cx, |_window, cx| {
                    _ = editor.update(cx, |editor, cx| {
                        assign_active_line_trailer_provider(editor, show_hints, cx);
                    })
                });
            }
        }
    })
    .detach();
}

struct AssistantHintsProvider;

impl ActiveLineTrailerProvider for AssistantHintsProvider {
    fn render_active_line_trailer(
        &mut self,
        style: &editor::EditorStyle,
        focus_handle: &gpui::FocusHandle,
        cx: &mut WindowContext,
    ) -> Option<gpui::AnyElement> {
        if !focus_handle.is_focused(cx) {
            return None;
        }

        let chat_keybinding =
            cx.keystroke_text_for_action_in(&assistant::ToggleFocus, focus_handle);
        let generate_keybinding =
            cx.keystroke_text_for_action_in(&zed_actions::InlineAssist::default(), focus_handle);

        Some(
            h_flex()
                .id("inline-assistant-instructions")
                .w_full()
                .font_family(style.text.font().family)
                .text_color(cx.theme().status().hint)
                .line_height(style.text.line_height)
                .child(format!(
                    "{chat_keybinding} to chat, {generate_keybinding} to generate"
                ))
                .into_any(),
        )
    }
}

fn assign_active_line_trailer_provider(
    editor: &mut Editor,
    show_hints: bool,
    cx: &mut ViewContext<Editor>,
) {
    let provider = show_hints.then_some(AssistantHintsProvider);
    editor.set_active_line_trailer_provider(provider, cx);
}

fn should_show_hints(cx: &AppContext) -> bool {
    let assistant_settings = AssistantSettings::get_global(cx);
    assistant_settings.enabled && assistant_settings.show_hints
}
