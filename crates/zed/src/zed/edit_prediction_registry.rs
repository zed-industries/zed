use client::{Client, UserStore};
use codestral::{CodestralEditPredictionDelegate, load_codestral_api_key};
use collections::HashMap;
use copilot::CopilotEditPredictionDelegate;
use edit_prediction::{EditPredictionModel, ZedEditPredictionDelegate, Zeta2FeatureFlag};
use editor::Editor;
use feature_flags::FeatureFlagAppExt;
use gpui::{AnyWindowHandle, App, AppContext as _, Context, Entity, WeakEntity};
use language::language_settings::{EditPredictionProvider, all_language_settings};

use settings::{
    EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME, EditPredictionPromptFormat, SettingsStore,
};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use supermaven::{Supermaven, SupermavenEditPredictionDelegate};
use ui::Window;

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
            let provider_config = edit_prediction_provider_config_for_settings(cx);
            assign_edit_prediction_provider(
                editor,
                provider_config,
                &client,
                user_store.clone(),
                window,
                cx,
            );
        }
    })
    .detach();

    cx.on_action(clear_edit_prediction_store_edit_history);

    cx.subscribe(&user_store, {
        let editors = editors.clone();
        let client = client.clone();

        move |user_store, event, cx| {
            if let client::user::Event::PrivateUserInfoUpdated = event {
                let provider_config = edit_prediction_provider_config_for_settings(cx);
                assign_edit_prediction_providers(
                    &editors,
                    provider_config,
                    &client,
                    user_store,
                    cx,
                );
            }
        }
    })
    .detach();

    cx.observe_global::<SettingsStore>({
        let editors = editors.clone();
        let client = client.clone();
        let user_store = user_store.clone();
        let mut previous_config = edit_prediction_provider_config_for_settings(cx);
        move |cx| {
            let new_provider_config = edit_prediction_provider_config_for_settings(cx);

            if new_provider_config != previous_config {
                telemetry::event!(
                    "Edit Prediction Provider Changed",
                    from = previous_config.map(|config| config.name()),
                    to = new_provider_config.map(|config| config.name())
                );

                previous_config = new_provider_config;
                assign_edit_prediction_providers(
                    &editors,
                    new_provider_config,
                    &client,
                    user_store.clone(),
                    cx,
                );
            }
        }
    })
    .detach();

    cx.observe_flag::<Zeta2FeatureFlag, _>({
        let mut previous_config = edit_prediction_provider_config_for_settings(cx);
        move |_is_enabled, cx| {
            let new_provider_config = edit_prediction_provider_config_for_settings(cx);
            if new_provider_config != previous_config {
                previous_config = new_provider_config;
                assign_edit_prediction_providers(
                    &editors,
                    new_provider_config,
                    &client,
                    user_store.clone(),
                    cx,
                );
            }
        }
    })
    .detach();
}

fn edit_prediction_provider_config_for_settings(cx: &App) -> Option<EditPredictionProviderConfig> {
    let settings = &all_language_settings(None, cx).edit_predictions;
    let provider = settings.provider;
    match provider {
        EditPredictionProvider::None => None,
        EditPredictionProvider::Copilot => Some(EditPredictionProviderConfig::Copilot),
        EditPredictionProvider::Supermaven => Some(EditPredictionProviderConfig::Supermaven),
        EditPredictionProvider::Zed => Some(EditPredictionProviderConfig::Zed(
            EditPredictionModel::Zeta1,
        )),
        EditPredictionProvider::Codestral => Some(EditPredictionProviderConfig::Codestral),
        EditPredictionProvider::Ollama | EditPredictionProvider::OpenAiCompatibleApi => {
            let custom_settings = if provider == EditPredictionProvider::Ollama {
                settings.ollama.as_ref()?
            } else {
                settings.open_ai_compatible_api.as_ref()?
            };

            let mut format = custom_settings.prompt_format;
            if format == EditPredictionPromptFormat::Infer {
                if let Some(inferred_format) = infer_prompt_format(&custom_settings.model) {
                    format = inferred_format;
                } else {
                    // todo: notify user that prompt format inference failed
                    return None;
                }
            }

            if format == EditPredictionPromptFormat::Zeta {
                Some(EditPredictionProviderConfig::Zed(
                    EditPredictionModel::Zeta1,
                ))
            } else {
                Some(EditPredictionProviderConfig::Zed(
                    EditPredictionModel::Fim { format },
                ))
            }
        }
        EditPredictionProvider::Sweep => Some(EditPredictionProviderConfig::Zed(
            EditPredictionModel::Sweep,
        )),
        EditPredictionProvider::Mercury => Some(EditPredictionProviderConfig::Zed(
            EditPredictionModel::Mercury,
        )),
        EditPredictionProvider::Experimental(name) => {
            if name == EXPERIMENTAL_ZETA2_EDIT_PREDICTION_PROVIDER_NAME
                && cx.has_flag::<Zeta2FeatureFlag>()
            {
                Some(EditPredictionProviderConfig::Zed(
                    EditPredictionModel::Zeta2,
                ))
            } else {
                None
            }
        }
    }
}

fn infer_prompt_format(model: &str) -> Option<EditPredictionPromptFormat> {
    let model_base = model.split(':').next().unwrap_or(model);

    Some(match model_base {
        "codellama" | "code-llama" => EditPredictionPromptFormat::CodeLlama,
        "starcoder" | "starcoder2" | "starcoderbase" => EditPredictionPromptFormat::StarCoder,
        "deepseek-coder" | "deepseek-coder-v2" => EditPredictionPromptFormat::DeepseekCoder,
        "qwen2.5-coder" | "qwen-coder" | "qwen" => EditPredictionPromptFormat::Qwen,
        "codegemma" => EditPredictionPromptFormat::CodeGemma,
        "codestral" | "mistral" => EditPredictionPromptFormat::Codestral,
        "glm" | "glm-4" | "glm-4.5" => EditPredictionPromptFormat::Glm,
        _ => {
            return None;
        }
    })
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum EditPredictionProviderConfig {
    Copilot,
    Supermaven,
    Codestral,
    Zed(EditPredictionModel),
}

impl EditPredictionProviderConfig {
    fn name(&self) -> &'static str {
        match self {
            EditPredictionProviderConfig::Copilot => "Copilot",
            EditPredictionProviderConfig::Supermaven => "Supermaven",
            EditPredictionProviderConfig::Codestral => "Codestral",
            EditPredictionProviderConfig::Zed(model) => match model {
                EditPredictionModel::Zeta1 => "Zeta1",
                EditPredictionModel::Zeta2 => "Zeta2",
                EditPredictionModel::Fim { .. } => "FIM",
                EditPredictionModel::Sweep => "Sweep",
                EditPredictionModel::Mercury => "Mercury",
            },
        }
    }
}

fn clear_edit_prediction_store_edit_history(_: &edit_prediction::ClearHistory, cx: &mut App) {
    if let Some(ep_store) = edit_prediction::EditPredictionStore::try_global(cx) {
        ep_store.update(cx, |ep_store, _| ep_store.clear_history());
    }
}

fn assign_edit_prediction_providers(
    editors: &Rc<RefCell<HashMap<WeakEntity<Editor>, AnyWindowHandle>>>,
    provider_config: Option<EditPredictionProviderConfig>,
    client: &Arc<Client>,
    user_store: Entity<UserStore>,
    cx: &mut App,
) {
    if provider_config == Some(EditPredictionProviderConfig::Codestral) {
        load_codestral_api_key(cx).detach();
    }
    for (editor, window) in editors.borrow().iter() {
        _ = window.update(cx, |_window, window, cx| {
            _ = editor.update(cx, |editor, cx| {
                assign_edit_prediction_provider(
                    editor,
                    provider_config,
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
}

fn assign_edit_prediction_provider(
    editor: &mut Editor,
    provider_config: Option<EditPredictionProviderConfig>,
    client: &Arc<Client>,
    user_store: Entity<UserStore>,
    window: &mut Window,
    cx: &mut Context<Editor>,
) {
    // TODO: Do we really want to collect data only for singleton buffers?
    let singleton_buffer = editor.buffer().read(cx).as_singleton();

    match provider_config {
        None => {
            editor.set_edit_prediction_provider::<ZedEditPredictionDelegate>(None, window, cx);
        }
        Some(EditPredictionProviderConfig::Copilot) => {
            let ep_store = edit_prediction::EditPredictionStore::global(client, &user_store, cx);
            let Some(project) = editor.project().cloned() else {
                return;
            };
            let copilot =
                ep_store.update(cx, |this, cx| this.start_copilot_for_project(&project, cx));

            if let Some(copilot) = copilot {
                if let Some(buffer) = singleton_buffer
                    && buffer.read(cx).file().is_some()
                {
                    copilot.update(cx, |copilot, cx| {
                        copilot.register_buffer(&buffer, cx);
                    });
                }
                let provider = cx.new(|_| CopilotEditPredictionDelegate::new(copilot));
                editor.set_edit_prediction_provider(Some(provider), window, cx);
            }
        }
        Some(EditPredictionProviderConfig::Supermaven) => {
            if let Some(supermaven) = Supermaven::global(cx) {
                let provider = cx.new(|_| SupermavenEditPredictionDelegate::new(supermaven));
                editor.set_edit_prediction_provider(Some(provider), window, cx);
            }
        }
        Some(EditPredictionProviderConfig::Codestral) => {
            let http_client = client.http_client();
            let provider = cx.new(|_| CodestralEditPredictionDelegate::new(http_client));
            editor.set_edit_prediction_provider(Some(provider), window, cx);
        }
        Some(EditPredictionProviderConfig::Zed(model)) => {
            let ep_store = edit_prediction::EditPredictionStore::global(client, &user_store, cx);

            if let Some(project) = editor.project() {
                let has_model = ep_store.update(cx, |ep_store, cx| {
                    ep_store.set_edit_prediction_model(model);
                    if let Some(buffer) = &singleton_buffer {
                        ep_store.register_buffer(buffer, project, cx);
                    }
                    true
                });

                if has_model {
                    let provider = cx.new(|cx| {
                        ZedEditPredictionDelegate::new(
                            project.clone(),
                            singleton_buffer,
                            &client,
                            &user_store,
                            cx,
                        )
                    });
                    editor.set_edit_prediction_provider(Some(provider), window, cx);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use editor::MultiBuffer;
    use gpui::{BorrowAppContext, TestAppContext};
    use settings::{EditPredictionProvider, SettingsStore};
    use workspace::AppState;

    #[gpui::test]
    async fn test_subscribe_uses_stale_provider_config_after_settings_change(
        cx: &mut TestAppContext,
    ) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            client::init(&app_state.client, cx);
            language_model::init(app_state.client.clone(), cx);
            editor::init(cx);
            app_state
        });

        // Override the default provider to None so the subscribe closure
        // captures None at init time. (The test default is Zed/Zeta1, which
        // is a no-op on project-less editors and would mask the bug.)
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store: &mut SettingsStore, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.edit_predictions =
                        Some(settings::EditPredictionSettingsContent {
                            provider: Some(EditPredictionProvider::None),
                            ..Default::default()
                        });
                });
            });
        });

        cx.update(|cx| {
            init(app_state.client.clone(), app_state.user_store.clone(), cx);
        });

        // Create an editor in a window so observe_new registers it.
        let editor = cx.add_window(|window, cx| {
            let buffer = cx.new(|_cx| MultiBuffer::new(language::Capability::ReadWrite));
            Editor::new(editor::EditorMode::full(), buffer, None, window, cx)
        });

        editor
            .update(cx, |editor, _window, _cx| {
                assert!(
                    editor.edit_prediction_provider().is_none(),
                    "editor should start with no provider when settings = None"
                );
            })
            .unwrap();

        // Change settings to Codestral. The observe_global closure updates its
        // own copy of provider_config and assigns Codestral to all editors.
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store: &mut SettingsStore, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.edit_predictions =
                        Some(settings::EditPredictionSettingsContent {
                            provider: Some(EditPredictionProvider::Codestral),
                            ..Default::default()
                        });
                });
            });
        });

        editor
            .update(cx, |editor, _window, _cx| {
                assert!(
                    editor.edit_prediction_provider().is_some(),
                    "editor should have a provider after changing settings to Codestral"
                );
            })
            .unwrap();

        // Emit PrivateUserInfoUpdated. The subscribe closure should use the
        // CURRENT provider config (Codestral), but due to the bug it uses the
        // stale init-time value (None) and clears the provider.
        cx.update(|cx| {
            app_state.user_store.update(cx, |_, cx| {
                cx.emit(client::user::Event::PrivateUserInfoUpdated);
            });
        });
        cx.run_until_parked();

        editor
            .update(cx, |editor, _window, _cx| {
                assert!(
                    editor.edit_prediction_provider().is_some(),
                    "BUG: subscribe closure used stale provider_config (None) instead of current (Codestral)"
                );
            })
            .unwrap();
    }
}
