use client::{Client, UserStore};
use collections::HashMap;
use copilot::{Copilot, CopilotCompletionProvider};
use editor::{Editor, EditorMode};
use feature_flags::{FeatureFlagAppExt, PredictEditsFeatureFlag};
use gpui::{AnyWindowHandle, App, AppContext, Context, Entity, WeakEntity};
use language::language_settings::{all_language_settings, EditPredictionProvider};
use settings::SettingsStore;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use supermaven::{Supermaven, SupermavenCompletionProvider};
use ui::Window;
use zeta::ProviderDataCollection;

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
            let provider = all_language_settings(None, cx).edit_predictions.provider;
            assign_edit_prediction_provider(
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

    let mut provider = all_language_settings(None, cx).edit_predictions.provider;
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_edit_prediction_provider(
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
            let provider = all_language_settings(None, cx).edit_predictions.provider;
            assign_edit_prediction_providers(&editors, provider, &client, user_store.clone(), cx);
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
            let new_provider = all_language_settings(None, cx).edit_predictions.provider;

            if new_provider != provider {
                let tos_accepted = user_store
                    .read(cx)
                    .current_user_has_accepted_terms()
                    .unwrap_or(false);

                telemetry::event!(
                    "Edit Prediction Provider Changed",
                    from = provider,
                    to = new_provider,
                    zed_ai_tos_accepted = tos_accepted,
                );

                provider = new_provider;
                assign_edit_prediction_providers(
                    &editors,
                    provider,
                    &client,
                    user_store.clone(),
                    cx,
                );

                if !tos_accepted {
                    match provider {
                        EditPredictionProvider::Zed => {
                            let Some(window) = cx.active_window() else {
                                return;
                            };

                            window
                                .update(cx, |_, window, cx| {
                                    window.dispatch_action(
                                        Box::new(zed_actions::OpenZedPredictOnboarding),
                                        cx,
                                    );
                                })
                                .ok();
                        }
                        EditPredictionProvider::None
                        | EditPredictionProvider::Copilot
                        | EditPredictionProvider::Supermaven => {}
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

fn assign_edit_prediction_providers(
    editors: &Rc<RefCell<HashMap<WeakEntity<Editor>, AnyWindowHandle>>>,
    provider: EditPredictionProvider,
    client: &Arc<Client>,
    user_store: Entity<UserStore>,
    cx: &mut App,
) {
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_edit_prediction_provider(
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
                editor.next_edit_prediction(&Default::default(), window, cx);
            },
        ))
        .detach();
    editor
        .register_action(cx.listener(
            |editor,
             _: &copilot::PreviousSuggestion,
             window: &mut Window,
             cx: &mut Context<Editor>| {
                editor.previous_edit_prediction(&Default::default(), window, cx);
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

fn assign_edit_prediction_provider(
    editor: &mut Editor,
    provider: EditPredictionProvider,
    client: &Arc<Client>,
    user_store: Entity<UserStore>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    // TODO: Do we really want to collect data only for singleton buffers?
    let singleton_buffer = editor.buffer().read(cx).as_singleton();

    match provider {
        EditPredictionProvider::None => {}
        EditPredictionProvider::Copilot => {
            if let Some(copilot) = Copilot::global(cx) {
                if let Some(buffer) = singleton_buffer {
                    if buffer.read(cx).file().is_some() {
                        copilot.update(cx, |copilot, cx| {
                            copilot.register_buffer(&buffer, cx);
                        });
                    }
                }
                let provider = cx.new(|_| CopilotCompletionProvider::new(copilot));
                editor.set_edit_prediction_provider(Some(provider), window, cx);
            }
        }
        EditPredictionProvider::Supermaven => {
            if let Some(supermaven) = Supermaven::global(cx) {
                let provider = cx.new(|_| SupermavenCompletionProvider::new(supermaven));
                editor.set_edit_prediction_provider(Some(provider), window, cx);
            }
        }
        EditPredictionProvider::Zed => {
            if cx.has_flag::<PredictEditsFeatureFlag>()
                || (cfg!(debug_assertions) && client.status().borrow().is_connected())
            {
                let mut worktree = None;

                if let Some(buffer) = &singleton_buffer {
                    if let Some(file) = buffer.read(cx).file() {
                        let id = file.worktree_id(cx);
                        if let Some(inner_worktree) = editor
                            .project
                            .as_ref()
                            .and_then(|project| project.read(cx).worktree_for_id(id, cx))
                        {
                            worktree = Some(inner_worktree);
                        }
                    }
                }

                let zeta = zeta::Zeta::register(worktree, client.clone(), user_store, cx);

                if let Some(buffer) = &singleton_buffer {
                    if buffer.read(cx).file().is_some() {
                        zeta.update(cx, |zeta, cx| {
                            zeta.register_buffer(&buffer, cx);
                        });
                    }
                }

                let data_collection =
                    ProviderDataCollection::new(zeta.clone(), singleton_buffer, cx);

                let provider =
                    cx.new(|_| zeta::ZetaInlineCompletionProvider::new(zeta, data_collection));

                editor.set_edit_prediction_provider(Some(provider), window, cx);
            }
        }
    }
}
