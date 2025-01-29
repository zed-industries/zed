use std::{cell::RefCell, rc::Rc, sync::Arc};

use client::{Client, UserStore};
use collections::HashMap;
use copilot::{Copilot, CopilotCompletionProvider};
use editor::{Editor, EditorMode};
use feature_flags::{FeatureFlagAppExt, PredictEditsFeatureFlag};
use gpui::{AnyWindowHandle, App, AppContext as _, Context, Entity, WeakEntity, Window};
use language::language_settings::{all_language_settings, InlineCompletionProvider};
use settings::SettingsStore;
use supermaven::{Supermaven, SupermavenCompletionProvider};
use zed_predict_tos::ZedPredictTos;

pub fn init(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut App) {
    let editors: Rc<RefCell<HashMap<WeakEntity<Editor>, AnyWindowHandle>>> = Rc::default();
    cx.observe_new({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
        move |editor: &mut Editor, window, cx: &mut Context<Editor>| {
            if editor.mode() != EditorMode::Full {
                return;
            }

            register_backward_compatible_actions(editor, cx);

            let Some(window) = window else {
                return;
            };

            let editor_handle = cx.entity().downgrade();
            cx.on_release({
                let editor_handle = editor_handle.clone();
                let editors = editors.clone();
                move |_, _| {
                    editors.borrow_mut().remove(&editor_handle);
                }
            })
            .detach();
            editors
                .borrow_mut()
                .insert(editor_handle, window.window_handle());
            let provider = all_language_settings(None, cx).inline_completions.provider;
            assign_inline_completion_provider(
                editor,
                provider,
                &client,
                user_store.clone(),
                window,
                cx,
            );
        }
    })
    .detach();

    let mut provider = all_language_settings(None, cx).inline_completions.provider;
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_inline_completion_provider(
                    editor,
                    provider,
                    &client,
                    user_store.clone(),
                    window,
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

                            let Some(Some(workspace)) = window
                                .update(cx, |_, window, _| window.root().flatten())
                                .ok()
                            else {
                                return;
                            };

                            window
                                .update(cx, |_, window, cx| {
                                    ZedPredictTos::toggle(
                                        workspace,
                                        user_store.clone(),
                                        window,
                                        cx,
                                    );
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

fn clear_zeta_edit_history(_: &zeta::ClearHistory, cx: &mut App) {
    if let Some(zeta) = zeta::Zeta::global(cx) {
        zeta.update(cx, |zeta, _| zeta.clear_history());
    }
}

fn assign_inline_completion_providers(
    editors: &Rc<RefCell<HashMap<WeakEntity<Editor>, AnyWindowHandle>>>,
    provider: InlineCompletionProvider,
    client: &Arc<Client>,
    user_store: Entity<UserStore>,
    cx: &mut App,
) {
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_inline_completion_provider(
                    editor,
                    provider,
                    &client,
                    user_store.clone(),
                    window,
                    cx,
                );
            })
        });
    }
}

fn register_backward_compatible_actions(editor: &mut Editor, cx: &mut Context<Editor>) {
    // We renamed some of these actions to not be copilot-specific, but that
    // would have not been backwards-compatible. So here we are re-registering
    // the actions with the old names to not break people's keymaps.
    editor
        .register_action(cx.listener(
            |editor, _: &copilot::Suggest, window: &mut Window, cx: &mut Context<Editor>| {
                editor.show_inline_completion(&Default::default(), window, cx);
            },
        ))
        .detach();
    editor
        .register_action(cx.listener(
            |editor, _: &copilot::NextSuggestion, window: &mut Window, cx: &mut Context<Editor>| {
                editor.next_inline_completion(&Default::default(), window, cx);
            },
        ))
        .detach();
    editor
        .register_action(cx.listener(
            |editor,
             _: &copilot::PreviousSuggestion,
             window: &mut Window,
             cx: &mut Context<Editor>| {
                editor.previous_inline_completion(&Default::default(), window, cx);
            },
        ))
        .detach();
    editor
        .register_action(cx.listener(
            |editor,
             _: &editor::actions::AcceptPartialCopilotSuggestion,
             window: &mut Window,
             cx: &mut Context<Editor>| {
                editor.accept_partial_inline_completion(&Default::default(), window, cx);
            },
        ))
        .detach();
}

fn assign_inline_completion_provider(
    editor: &mut Editor,
    provider: language::language_settings::InlineCompletionProvider,
    client: &Arc<Client>,
    user_store: Entity<UserStore>,
    window: &mut Window,
    cx: &mut Context<Editor>,
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
                let provider = cx.new(|_| CopilotCompletionProvider::new(copilot));
                editor.set_inline_completion_provider(Some(provider), window, cx);
            }
        }
        language::language_settings::InlineCompletionProvider::Supermaven => {
            if let Some(supermaven) = Supermaven::global(cx) {
                let provider = cx.new(|_| SupermavenCompletionProvider::new(supermaven));
                editor.set_inline_completion_provider(Some(provider), window, cx);
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
                let provider = cx.new(|_| zeta::ZetaInlineCompletionProvider::new(zeta));
                editor.set_inline_completion_provider(Some(provider), window, cx);
            }
        }
    }
}
