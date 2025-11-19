use client::{Client, UserStore};
use codestral::CodestralCompletionProvider;
use collections::HashMap;
use copilot::{Copilot, CopilotCompletionProvider};
use editor::Editor;
use feature_flags::FeatureFlagAppExt;
use gpui::{AnyWindowHandle, App, AppContext as _, Context, Entity, WeakEntity};
use language::language_settings::{EditPredictionProvider, all_language_settings};
use language_models::MistralLanguageModelProvider;
use settings::SettingsStore;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use supermaven::{Supermaven, SupermavenCompletionProvider};
use ui::Window;
use zeta::ZetaEditPredictionProvider;
use zeta2::Zeta2FeatureFlag;

pub fn init(client: Arc<Client>, user_store: Entity<UserStore>, cx: &mut App) {
    let editors: Rc<RefCell<HashMap<WeakEntity<Editor>, AnyWindowHandle>>> = Rc::default();
    cx.observe_new({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
        move |editor: &mut Editor, window, cx: &mut Context<Editor>| {
            if !editor.mode().is_full() {
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

    cx.on_action(clear_zeta_edit_history);

    let mut provider = all_language_settings(None, cx).edit_predictions.provider;
    cx.subscribe(&user_store, {
        let editors = editors.clone();
        let client = client.clone();

        move |user_store, event, cx| {
            if let client::user::Event::PrivateUserInfoUpdated = event {
                assign_edit_prediction_providers(&editors, provider, &client, user_store, cx);
            }
        }
    })
    .detach();

    cx.observe_global::<SettingsStore>({
        let user_store = user_store.clone();
        move |cx| {
            let new_provider = all_language_settings(None, cx).edit_predictions.provider;

            if new_provider != provider {
                telemetry::event!(
                    "Edit Prediction Provider Changed",
                    from = provider,
                    to = new_provider,
                );

                provider = new_provider;
                assign_edit_prediction_providers(
                    &editors,
                    provider,
                    &client,
                    user_store.clone(),
                    cx,
                );
            }
        }
    })
    .detach();
}

fn clear_zeta_edit_history(_: &zeta::ClearHistory, cx: &mut App) {
    if let Some(zeta) = zeta::Zeta::global(cx) {
        zeta.update(cx, |zeta, _| zeta.clear_history());
    } else if let Some(zeta) = zeta2::Zeta::try_global(cx) {
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
    if provider == EditPredictionProvider::Codestral {
        let mistral = MistralLanguageModelProvider::global(client.http_client(), cx);
        mistral.load_codestral_api_key(cx).detach();
    }
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_edit_prediction_provider(
                    editor,
                    provider,
                    client,
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
                editor.show_edit_prediction(&Default::default(), window, cx);
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
        EditPredictionProvider::None => {
            editor.set_edit_prediction_provider::<ZetaEditPredictionProvider>(None, window, cx);
        }
        EditPredictionProvider::Copilot => {
            if let Some(copilot) = Copilot::global(cx) {
                if let Some(buffer) = singleton_buffer
                    && buffer.read(cx).file().is_some()
                {
                    copilot.update(cx, |copilot, cx| {
                        copilot.register_buffer(&buffer, cx);
                    });
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
        EditPredictionProvider::Codestral => {
            let http_client = client.http_client();
            let provider = cx.new(|_| CodestralCompletionProvider::new(http_client));
            editor.set_edit_prediction_provider(Some(provider), window, cx);
        }
        EditPredictionProvider::Zed => {
            if user_store.read(cx).current_user().is_some() {
                let mut worktree = None;

                if let Some(buffer) = &singleton_buffer
                    && let Some(file) = buffer.read(cx).file()
                {
                    let id = file.worktree_id(cx);
                    if let Some(inner_worktree) = editor
                        .project()
                        .and_then(|project| project.read(cx).worktree_for_id(id, cx))
                    {
                        worktree = Some(inner_worktree);
                    }
                }

                if let Some(project) = editor.project() {
                    if cx.has_flag::<Zeta2FeatureFlag>() {
                        let zeta = zeta2::Zeta::global(client, &user_store, cx);
                        let provider = cx.new(|cx| {
                            zeta2::ZetaEditPredictionProvider::new(
                                project.clone(),
                                &client,
                                &user_store,
                                cx,
                            )
                        });

                        // TODO [zeta2] handle multibuffers
                        if let Some(buffer) = &singleton_buffer
                            && buffer.read(cx).file().is_some()
                        {
                            zeta.update(cx, |zeta, cx| {
                                zeta.register_buffer(buffer, project, cx);
                            });
                        }

                        editor.set_edit_prediction_provider(Some(provider), window, cx);
                    } else {
                        let zeta = zeta::Zeta::register(worktree, client.clone(), user_store, cx);

                        if let Some(buffer) = &singleton_buffer
                            && buffer.read(cx).file().is_some()
                        {
                            zeta.update(cx, |zeta, cx| {
                                zeta.register_buffer(buffer, project, cx);
                            });
                        }

                        let provider = cx.new(|cx| {
                            zeta::ZetaEditPredictionProvider::new(
                                zeta,
                                project.clone(),
                                singleton_buffer,
                                cx,
                            )
                        });
                        editor.set_edit_prediction_provider(Some(provider), window, cx);
                    }
                }
            }
        }
    }
}
