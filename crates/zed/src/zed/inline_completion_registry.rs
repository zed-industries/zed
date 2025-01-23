use std::{cell::RefCell, rc::Rc, sync::Arc};

use client::{Client, UserStore};
use collections::HashMap;
use copilot::{Copilot, CopilotCompletionProvider};
use editor::{Editor, EditorMode};
use feature_flags::{FeatureFlagAppExt, PredictEditsFeatureFlag};
use gpui::{AnyWindowHandle, AppContext, Context, Model, ViewContext, WeakView};
use language::language_settings::{all_language_settings, InlineCompletionProvider};
use settings::SettingsStore;
use supermaven::{Supermaven, SupermavenCompletionProvider};
use workspace::Workspace;
use zed_predict_tos::ZedPredictTos;

pub fn init(client: Arc<Client>, user_store: Model<UserStore>, cx: &mut AppContext) {
    let editors: Rc<RefCell<HashMap<WeakView<Editor>, AnyWindowHandle>>> = Rc::default();
    cx.observe_new_views({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
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
            assign_inline_completion_provider(editor, provider, &client, user_store.clone(), cx);
        }
    })
    .detach();

    let mut provider = all_language_settings(None, cx).inline_completions.provider;
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_inline_completion_provider(
                    editor,
                    provider,
                    &client,
                    user_store.clone(),
                    cx,
                );
            })
        });
    }

    if cx.has_flag::<PredictEditsFeatureFlag>() {
        cx.on_action(clear_zeta_edit_history);
    }

    cx.observe_flag::<PredictEditsFeatureFlag, _>({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
        move |active, cx| {
            let provider = all_language_settings(None, cx).inline_completions.provider;
            assign_inline_completion_providers(&editors, provider, &client, user_store.clone(), cx);
            if active && !cx.is_action_available(&zeta::ClearHistory) {
                cx.on_action(clear_zeta_edit_history);
            }
        }
    })
    .detach();

    cx.observe_global::<SettingsStore>({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
        move |cx| {
            let new_provider = all_language_settings(None, cx).inline_completions.provider;
            if new_provider != provider {
                provider = new_provider;
                assign_inline_completion_providers(
                    &editors,
                    provider,
                    &client,
                    user_store.clone(),
                    cx,
                );

                if !user_store
                    .read(cx)
                    .current_user_has_accepted_terms()
                    .unwrap_or(false)
                {
                    match provider {
                        InlineCompletionProvider::Zed => {
                            let Some(window) = cx.active_window() else {
                                return;
                            };

                            let Some(workspace) = window
                                .downcast::<Workspace>()
                                .and_then(|w| w.root_view(cx).ok())
                            else {
                                return;
                            };

                            window
                                .update(cx, |_, cx| {
                                    ZedPredictTos::toggle(workspace, user_store.clone(), cx);
                                })
                                .ok();
                        }
                        InlineCompletionProvider::None
                        | InlineCompletionProvider::Copilot
                        | InlineCompletionProvider::Supermaven => {}
                    }
                }
            }
        }
    })
    .detach();
}

fn clear_zeta_edit_history(_: &zeta::ClearHistory, cx: &mut AppContext) {
    if let Some(zeta) = zeta::Zeta::global(cx) {
        zeta.update(cx, |zeta, _| zeta.clear_history());
    }
}

fn assign_inline_completion_providers(
    editors: &Rc<RefCell<HashMap<WeakView<Editor>, AnyWindowHandle>>>,
    provider: InlineCompletionProvider,
    client: &Arc<Client>,
    user_store: Model<UserStore>,
    cx: &mut AppContext,
) {
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_inline_completion_provider(
                    editor,
                    provider,
                    &client,
                    user_store.clone(),
                    cx,
                );
            })
        });
    }
}

fn register_backward_compatible_actions(editor: &mut Editor, cx: &ViewContext<Editor>) {
    // We renamed some of these actions to not be copilot-specific, but that
    // would have not been backwards-compatible. So here we are re-registering
    // the actions with the old names to not break people's keymaps.
    editor
        .register_action(cx.listener(
            |editor, _: &copilot::Suggest, cx: &mut ViewContext<Editor>| {
                editor.show_inline_completion(&Default::default(), cx);
            },
        ))
        .detach();
    editor
        .register_action(cx.listener(
            |editor, _: &copilot::NextSuggestion, cx: &mut ViewContext<Editor>| {
                editor.next_inline_completion(&Default::default(), cx);
            },
        ))
        .detach();
    editor
        .register_action(cx.listener(
            |editor, _: &copilot::PreviousSuggestion, cx: &mut ViewContext<Editor>| {
                editor.previous_inline_completion(&Default::default(), cx);
            },
        ))
        .detach();
    editor
        .register_action(cx.listener(
            |editor,
             _: &editor::actions::AcceptPartialCopilotSuggestion,
             cx: &mut ViewContext<Editor>| {
                editor.accept_partial_inline_completion(&Default::default(), cx);
            },
        ))
        .detach();
}

fn assign_inline_completion_provider(
    editor: &mut Editor,
    provider: language::language_settings::InlineCompletionProvider,
    client: &Arc<Client>,
    user_store: Model<UserStore>,
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
                let provider = cx.new_model(|_| CopilotCompletionProvider::new(copilot));
                editor.set_inline_completion_provider(Some(provider), cx);
            }
        }
        language::language_settings::InlineCompletionProvider::Supermaven => {
            if let Some(supermaven) = Supermaven::global(cx) {
                let provider = cx.new_model(|_| SupermavenCompletionProvider::new(supermaven));
                editor.set_inline_completion_provider(Some(provider), cx);
            }
        }

        language::language_settings::InlineCompletionProvider::Zed => {
            if cx.has_flag::<PredictEditsFeatureFlag>()
                || (cfg!(debug_assertions) && client.status().borrow().is_connected())
            {
                let zeta = zeta::Zeta::register(client.clone(), user_store, cx);
                if let Some(buffer) = editor.buffer().read(cx).as_singleton() {
                    if buffer.read(cx).file().is_some() {
                        zeta.update(cx, |zeta, cx| {
                            zeta.register_buffer(&buffer, cx);
                        });
                    }
                }
                let provider = cx.new_model(|_| zeta::ZetaInlineCompletionProvider::new(zeta));
                editor.set_inline_completion_provider(Some(provider), cx);
            }
        }
    }
}
