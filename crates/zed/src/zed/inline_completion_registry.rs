use std::{cell::RefCell, rc::Rc, sync::Arc};

use client::telemetry::Telemetry;
use collections::HashMap;
use copilot::Copilot;
use copilot_ui::CopilotCompletionProvider;
use editor::{Editor, EditorMode};
use gpui::{AnyWindowHandle, AppContext, Context, ViewContext, WeakView};
use language::language_settings::all_language_settings;
use settings::SettingsStore;
use supermaven::{Supermaven, SupermavenCompletionProvider};

pub fn init(telemetry: Arc<Telemetry>, cx: &mut AppContext) {
    let editors: Rc<RefCell<HashMap<WeakView<Editor>, AnyWindowHandle>>> = Rc::default();
    cx.observe_new_views({
        let editors = editors.clone();
        let telemetry = telemetry.clone();
        move |editor: &mut Editor, cx: &mut ViewContext<Editor>| {
            if editor.mode() != EditorMode::Full {
                return;
            }

            register_backward_compatible_actions(editor, cx);

            let editor_handle = cx.view().downgrade();
            cx.on_release({
                let editor_handle = editor_handle.clone();
                let editors = editors.clone();
                move |_, _, _| {
                    editors.borrow_mut().remove(&editor_handle);
                }
            })
            .detach();
            editors
                .borrow_mut()
                .insert(editor_handle, cx.window_handle());
            let provider = all_language_settings(None, cx).inline_completions.provider;
            assign_inline_completion_provider(editor, provider, &telemetry, cx);
        }
    })
    .detach();

    let mut provider = all_language_settings(None, cx).inline_completions.provider;
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_inline_completion_provider(editor, provider, &telemetry, cx);
            })
        });
    }

    cx.observe_global::<SettingsStore>(move |cx| {
        let new_provider = all_language_settings(None, cx).inline_completions.provider;
        if new_provider != provider {
            provider = new_provider;
            for (editor, window) in editors.borrow().iter() {
                _ = window.update(cx, |_window, cx| {
                    _ = editor.update(cx, |editor, cx| {
                        assign_inline_completion_provider(editor, provider, &telemetry, cx);
                    })
                });
            }
        }
    })
    .detach();
}

fn register_backward_compatible_actions(editor: &mut Editor, cx: &mut ViewContext<Editor>) {
    // We renamed some of these actions to not be copilot-specific, but that
    // would have not been backwards-compatible. So here we are re-registering
    // the actions with the old names to not break people's keymaps.
    editor
        .register_action(cx.listener(
            |editor, _: &copilot::Suggest, cx: &mut ViewContext<Editor>| {
                editor.show_inline_completion(&Default::default(), cx);
            },
        ))
        .register_action(cx.listener(
            |editor, _: &copilot::NextSuggestion, cx: &mut ViewContext<Editor>| {
                editor.next_inline_completion(&Default::default(), cx);
            },
        ))
        .register_action(cx.listener(
            |editor, _: &copilot::PreviousSuggestion, cx: &mut ViewContext<Editor>| {
                editor.previous_inline_completion(&Default::default(), cx);
            },
        ))
        .register_action(cx.listener(
            |editor,
             _: &editor::actions::AcceptPartialCopilotSuggestion,
             cx: &mut ViewContext<Editor>| {
                editor.accept_partial_inline_completion(&Default::default(), cx);
            },
        ));
}

fn assign_inline_completion_provider(
    editor: &mut Editor,
    provider: language::language_settings::InlineCompletionProvider,
    telemetry: &Arc<Telemetry>,
    cx: &mut ViewContext<Editor>,
) {
    match provider {
        language::language_settings::InlineCompletionProvider::None => {}
        language::language_settings::InlineCompletionProvider::Copilot => {
            if let Some(copilot) = Copilot::global(cx) {
                if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                    if buffer.read(cx).file().is_some() {
                        copilot.update(cx, |copilot, cx| {
                            copilot.register_buffer(&buffer, cx);
                        });
                    }
                }
                let provider = cx.new_model(|_| {
                    CopilotCompletionProvider::new(copilot).with_telemetry(telemetry.clone())
                });
                editor.set_inline_completion_provider(Some(provider), cx);
            }
        }
        language::language_settings::InlineCompletionProvider::Supermaven => {
            if let Some(supermaven) = Supermaven::global(cx) {
                let provider = cx.new_model(|_| SupermavenCompletionProvider::new(supermaven));
                editor.set_inline_completion_provider(Some(provider), cx);
            }
        }
    }
}
