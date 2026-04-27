use std::sync::Arc;

use anyhow::Result;
use collections::HashSet;
use fs::Fs;
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, ScrollHandle, Task,
};
use language_model::LanguageModelRegistry;
use language_models::provider::open_ai_compatible::{AvailableModel, ModelCapabilities};
use settings::{OpenAiCompatibleSettingsContent, update_settings_file};
use ui::{
    Banner, Checkbox, KeyBinding, Modal, ModalFooter, ModalHeader, Section, ToggleState,
    WithScrollbar, prelude::*,
};
use ui_input::InputField;
use workspace::{ModalView, Workspace};

fn single_line_input(
    label: impl Into<SharedString>,
    placeholder: &str,
    text: Option<&str>,
    tab_index: isize,
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputField> {
    cx.new(|cx| {
        let input = InputField::new(window, cx, placeholder)
            .label(label)
            .tab_index(tab_index)
            .tab_stop(true);

        if let Some(text) = text {
            input.set_text(text, window, cx);
        }
        input
    })
}

#[derive(Clone, Copy)]
pub enum LlmCompatibleProvider {
    OpenAi,
}

impl LlmCompatibleProvider {
    fn name(&self) -> &'static str {
        match self {
            LlmCompatibleProvider::OpenAi => "OpenAI",
        }
    }

    fn api_url(&self) -> &'static str {
        match self {
            LlmCompatibleProvider::OpenAi => "https://api.openai.com/v1",
        }
    }
}

struct AddLlmProviderInput {
    provider_name: Entity<InputField>,
    api_url: Entity<InputField>,
    api_key: Entity<InputField>,
    models: Vec<ModelInput>,
}

impl AddLlmProviderInput {
    fn new(provider: LlmCompatibleProvider, window: &mut Window, cx: &mut App) -> Self {
        let provider_name =
            single_line_input("Provider Name", provider.name(), None, 1, window, cx);
        let api_url = single_line_input("API URL", provider.api_url(), None, 2, window, cx);
        let api_key = cx.new(|cx| {
            InputField::new(
                window,
                cx,
                "000000000000000000000000000000000000000000000000",
            )
            .label("API Key")
            .tab_index(3)
            .tab_stop(true)
            .masked(true)
        });

        Self {
            provider_name,
            api_url,
            api_key,
            models: vec![ModelInput::new(0, window, cx)],
        }
    }

    fn existing(
        provider_name: Arc<str>,
        settings: OpenAiCompatibleSettingsContent,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let provider_name =
            single_line_input("Provider Name", "", Some(&provider_name), 1, window, cx);
        let api_url = single_line_input("API URL", "", Some(&settings.api_url), 2, window, cx);
        let api_key = cx.new(|cx| {
            InputField::new(window, cx, "")
                .label("API Key")
                .tab_index(3)
                .tab_stop(true)
                .masked(true)
        });

        let models = settings
            .available_models
            .into_iter()
            .enumerate()
            .map(|(model_index, model)| ModelInput::existing(model_index, model, window, cx))
            .collect();

        Self {
            provider_name,
            api_url,
            api_key,
            models,
        }
    }

    fn add_model(&mut self, window: &mut Window, cx: &mut App) {
        let model_index = self.models.len();
        self.models.push(ModelInput::new(model_index, window, cx));
    }

    fn remove_model(&mut self, index: usize) {
        self.models.remove(index);
    }
}

struct ModelCapabilityToggles {
    pub supports_tools: ToggleState,
    pub supports_images: ToggleState,
    pub supports_parallel_tool_calls: ToggleState,
    pub supports_prompt_cache_key: ToggleState,
    pub supports_chat_completions: ToggleState,
}

struct ModelInput {
    name: Entity<InputField>,
    max_completion_tokens: Entity<InputField>,
    max_output_tokens: Entity<InputField>,
    max_tokens: Entity<InputField>,
    capabilities: ModelCapabilityToggles,
    original_model: Option<AvailableModel>,
}

impl ModelInput {
    fn new(model_index: usize, window: &mut Window, cx: &mut App) -> Self {
        let base_tab_index = (3 + (model_index * 4)) as isize;

        let model_name = single_line_input(
            "Model Name",
            "e.g. gpt-5, claude-opus-4, gemini-2.5-pro",
            None,
            base_tab_index + 1,
            window,
            cx,
        );
        let max_completion_tokens = single_line_input(
            "Max Completion Tokens",
            "200000",
            Some("200000"),
            base_tab_index + 2,
            window,
            cx,
        );
        let max_output_tokens = single_line_input(
            "Max Output Tokens",
            "Max Output Tokens",
            Some("32000"),
            base_tab_index + 3,
            window,
            cx,
        );
        let max_tokens = single_line_input(
            "Max Tokens",
            "Max Tokens",
            Some("200000"),
            base_tab_index + 4,
            window,
            cx,
        );

        let ModelCapabilities {
            tools,
            images,
            parallel_tool_calls,
            prompt_cache_key,
            chat_completions,
            ..
        } = ModelCapabilities::default();

        Self {
            name: model_name,
            max_completion_tokens,
            max_output_tokens,
            max_tokens,
            capabilities: ModelCapabilityToggles {
                supports_tools: tools.into(),
                supports_images: images.into(),
                supports_parallel_tool_calls: parallel_tool_calls.into(),
                supports_prompt_cache_key: prompt_cache_key.into(),
                supports_chat_completions: chat_completions.into(),
            },
            original_model: None,
        }
    }

    fn existing(
        model_index: usize,
        model: AvailableModel,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let mut input = Self::new(model_index, window, cx);

        input.name.update(cx, |input, cx| {
            input.set_text(&model.name, window, cx);
        });
        input.max_completion_tokens.update(cx, |input, cx| {
            let text = model
                .max_completion_tokens
                .map(|tokens| tokens.to_string())
                .unwrap_or_default();
            input.set_text(&text, window, cx);
        });
        input.max_output_tokens.update(cx, |input, cx| {
            let text = model
                .max_output_tokens
                .map(|tokens| tokens.to_string())
                .unwrap_or_default();
            input.set_text(&text, window, cx);
        });
        input.max_tokens.update(cx, |input, cx| {
            input.set_text(&model.max_tokens.to_string(), window, cx);
        });
        input.capabilities = ModelCapabilityToggles {
            supports_tools: model.capabilities.tools.into(),
            supports_images: model.capabilities.images.into(),
            supports_parallel_tool_calls: model.capabilities.parallel_tool_calls.into(),
            supports_prompt_cache_key: model.capabilities.prompt_cache_key.into(),
            supports_chat_completions: model.capabilities.chat_completions.into(),
        };
        input.original_model = Some(model);

        input
    }

    fn parse(
        &self,
        allow_empty_optional_token_fields: bool,
        cx: &App,
    ) -> Result<AvailableModel, SharedString> {
        let name = self.name.read(cx).text(cx);
        if name.is_empty() {
            return Err(SharedString::from("Model Name cannot be empty"));
        }
        let max_completion_tokens = self.max_completion_tokens.read(cx).text(cx);
        let max_output_tokens = self.max_output_tokens.read(cx).text(cx);

        let original_model = self.original_model.clone();

        Ok(AvailableModel {
            name,
            display_name: original_model
                .as_ref()
                .and_then(|model| model.display_name.clone()),
            max_completion_tokens: if allow_empty_optional_token_fields
                && max_completion_tokens.trim().is_empty()
            {
                None
            } else {
                Some(
                    max_completion_tokens.parse::<u64>().map_err(|_| {
                        SharedString::from("Max Completion Tokens must be a number")
                    })?,
                )
            },
            max_output_tokens: if allow_empty_optional_token_fields
                && max_output_tokens.trim().is_empty()
            {
                None
            } else {
                Some(
                    max_output_tokens
                        .parse::<u64>()
                        .map_err(|_| SharedString::from("Max Output Tokens must be a number"))?,
                )
            },
            max_tokens: self
                .max_tokens
                .read(cx)
                .text(cx)
                .parse::<u64>()
                .map_err(|_| SharedString::from("Max Tokens must be a number"))?,
            reasoning_effort: original_model
                .as_ref()
                .and_then(|model| model.reasoning_effort),
            capabilities: ModelCapabilities {
                tools: self.capabilities.supports_tools.selected(),
                images: self.capabilities.supports_images.selected(),
                parallel_tool_calls: self.capabilities.supports_parallel_tool_calls.selected(),
                prompt_cache_key: self.capabilities.supports_prompt_cache_key.selected(),
                chat_completions: self.capabilities.supports_chat_completions.selected(),
                interleaved_reasoning: original_model
                    .as_ref()
                    .is_some_and(|model| model.capabilities.interleaved_reasoning),
            },
        })
    }
}

#[derive(Clone)]
enum SaveMode {
    Create,
    EditModels(ModelEditContext),
}

#[derive(Clone)]
struct ModelEditContext {
    provider_name: Arc<str>,
    api_url: String,
}

fn provider_settings_from_input(
    input: &AddLlmProviderInput,
    mode: SaveMode,
    cx: &mut App,
) -> Result<(Arc<str>, OpenAiCompatibleSettingsContent), SharedString> {
    let provider_name: Arc<str> = match &mode {
        SaveMode::Create => input.provider_name.read(cx).text(cx).into(),
        SaveMode::EditModels(context) => context.provider_name.clone(),
    };
    if provider_name.is_empty() {
        return Err("Provider Name cannot be empty".into());
    }

    if matches!(mode, SaveMode::Create)
        && LanguageModelRegistry::read_global(cx)
            .providers()
            .iter()
            .any(|provider| {
                provider.id().0.as_ref() == provider_name.as_ref()
                    || provider.name().0.as_ref() == provider_name.as_ref()
            })
    {
        return Err("Provider Name is already taken by another provider".into());
    }

    let api_url = match &mode {
        SaveMode::Create => input.api_url.read(cx).text(cx),
        SaveMode::EditModels(context) => context.api_url.clone(),
    };
    if api_url.is_empty() {
        return Err("API URL cannot be empty".into());
    }

    let api_key = input.api_key.read(cx).text(cx);
    if matches!(mode, SaveMode::Create) && api_key.is_empty() {
        return Err("API Key cannot be empty".into());
    }

    let allow_empty_optional_token_fields = matches!(mode, SaveMode::EditModels(_));
    let mut models = Vec::new();
    let mut model_names: HashSet<String> = HashSet::default();
    for model in &input.models {
        match model.parse(allow_empty_optional_token_fields, cx) {
            Ok(model) => {
                if !model_names.insert(model.name.clone()) {
                    return Err("Model Names must be unique".into());
                }
                models.push(model)
            }
            Err(err) => return Err(err),
        }
    }

    Ok((
        provider_name,
        OpenAiCompatibleSettingsContent {
            api_url,
            available_models: models,
        },
    ))
}

fn save_provider_to_settings(
    input: &AddLlmProviderInput,
    mode: SaveMode,
    cx: &mut App,
) -> Task<Result<(), SharedString>> {
    let is_create = matches!(mode, SaveMode::Create);
    let (provider_name, provider_settings) =
        match provider_settings_from_input(input, mode.clone(), cx) {
            Ok(provider_settings) => provider_settings,
            Err(error) => return Task::ready(Err(error)),
        };

    let api_key = input.api_key.read(cx).text(cx);
    let fs = <dyn Fs>::global(cx);
    let write_credentials = if is_create {
        Some(cx.write_credentials(&provider_settings.api_url, "Bearer", api_key.as_bytes()))
    } else {
        None
    };
    cx.spawn(async move |cx| {
        if let Some(write_credentials) = write_credentials {
            write_credentials
                .await
                .map_err(|_| SharedString::from("Failed to write API key to keychain"))?;
        }
        cx.update(|cx| {
            update_settings_file(fs, cx, |settings, _cx| {
                settings
                    .language_models
                    .get_or_insert_default()
                    .openai_compatible
                    .get_or_insert_default()
                    .insert(provider_name, provider_settings);
            });
        });
        Ok(())
    })
}

pub struct AddLlmProviderModal {
    provider: LlmCompatibleProvider,
    model_edit_context: Option<ModelEditContext>,
    input: AddLlmProviderInput,
    scroll_handle: ScrollHandle,
    focus_handle: FocusHandle,
    last_error: Option<SharedString>,
}

impl AddLlmProviderModal {
    pub fn toggle(
        provider: LlmCompatibleProvider,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |window, cx| Self::new(provider, window, cx));
    }

    pub fn toggle_model_edit(
        provider: LlmCompatibleProvider,
        provider_name: Arc<str>,
        settings: OpenAiCompatibleSettingsContent,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |window, cx| {
            Self::new_model_edit(provider, provider_name, settings, window, cx)
        });
    }

    fn new(provider: LlmCompatibleProvider, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            input: AddLlmProviderInput::new(provider, window, cx),
            provider,
            model_edit_context: None,
            last_error: None,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
        }
    }

    fn new_model_edit(
        provider: LlmCompatibleProvider,
        provider_name: Arc<str>,
        settings: OpenAiCompatibleSettingsContent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let api_url = settings.api_url.clone();
        Self {
            input: AddLlmProviderInput::existing(provider_name.clone(), settings, window, cx),
            provider,
            model_edit_context: Some(ModelEditContext {
                provider_name,
                api_url,
            }),
            last_error: None,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        let mode = match self.model_edit_context.clone() {
            Some(context) => SaveMode::EditModels(context),
            None => SaveMode::Create,
        };
        let task = save_provider_to_settings(&self.input, mode, cx);
        cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| match result {
                Ok(_) => {
                    cx.emit(DismissEvent);
                }
                Err(error) => {
                    this.last_error = Some(error);
                    cx.notify();
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn render_model_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .mt_1()
            .gap_2()
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Models").size(LabelSize::Small))
                    .child(
                        Button::new("add-model", "Add Model")
                            .start_icon(
                                Icon::new(IconName::Plus)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.input.add_model(window, cx);
                                cx.notify();
                            })),
                    ),
            )
            .children(
                self.input
                    .models
                    .iter()
                    .enumerate()
                    .map(|(ix, _)| self.render_model(ix, cx)),
            )
    }

    fn render_model(&self, ix: usize, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let has_more_than_one_model = self.input.models.len() > 1;
        let model = &self.input.models[ix];

        v_flex()
            .p_2()
            .gap_2()
            .rounded_sm()
            .border_1()
            .border_dashed()
            .border_color(cx.theme().colors().border.opacity(0.6))
            .bg(cx.theme().colors().element_active.opacity(0.15))
            .child(model.name.clone())
            .child(
                h_flex()
                    .gap_2()
                    .child(model.max_completion_tokens.clone())
                    .child(model.max_output_tokens.clone()),
            )
            .child(model.max_tokens.clone())
            .child(
                v_flex()
                    .gap_1()
                    .child(
                        Checkbox::new(("supports-tools", ix), model.capabilities.supports_tools)
                            .label("Supports tools")
                            .on_click(cx.listener(move |this, checked, _window, cx| {
                                this.input.models[ix].capabilities.supports_tools = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new(("supports-images", ix), model.capabilities.supports_images)
                            .label("Supports images")
                            .on_click(cx.listener(move |this, checked, _window, cx| {
                                this.input.models[ix].capabilities.supports_images = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new(
                            ("supports-parallel-tool-calls", ix),
                            model.capabilities.supports_parallel_tool_calls,
                        )
                        .label("Supports parallel_tool_calls")
                        .on_click(cx.listener(
                            move |this, checked, _window, cx| {
                                this.input.models[ix]
                                    .capabilities
                                    .supports_parallel_tool_calls = *checked;
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        Checkbox::new(
                            ("supports-prompt-cache-key", ix),
                            model.capabilities.supports_prompt_cache_key,
                        )
                        .label("Supports prompt_cache_key")
                        .on_click(cx.listener(
                            move |this, checked, _window, cx| {
                                this.input.models[ix].capabilities.supports_prompt_cache_key =
                                    *checked;
                                cx.notify();
                            },
                        )),
                    )
                    .child(
                        Checkbox::new(
                            ("supports-chat-completions", ix),
                            model.capabilities.supports_chat_completions,
                        )
                        .label("Supports /chat/completions")
                        .on_click(cx.listener(
                            move |this, checked, _window, cx| {
                                this.input.models[ix].capabilities.supports_chat_completions =
                                    *checked;
                                cx.notify();
                            },
                        )),
                    ),
            )
            .when(has_more_than_one_model, |this| {
                this.child(
                    Button::new(("remove-model", ix), "Remove Model")
                        .start_icon(
                            Icon::new(IconName::Trash)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Outlined)
                        .full_width()
                        .on_click(cx.listener(move |this, _, _window, cx| {
                            this.input.remove_model(ix);
                            cx.notify();
                        })),
                )
            })
    }

    fn on_tab(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        window.focus_next(cx);
    }

    fn on_tab_prev(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus_prev(cx);
    }
}

impl EventEmitter<DismissEvent> for AddLlmProviderModal {}

impl Focusable for AddLlmProviderModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for AddLlmProviderModal {}

impl Render for AddLlmProviderModal {
    fn render(&mut self, window: &mut ui::Window, cx: &mut ui::Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);

        let window_size = window.viewport_size();
        let rem_size = window.rem_size();
        let is_large_window = window_size.height / rem_size > rems_from_px(600.).0;

        let modal_max_height = if is_large_window {
            rems_from_px(450.)
        } else {
            rems_from_px(200.)
        };
        let is_model_editing = self.model_edit_context.is_some();

        v_flex()
            .id("add-llm-provider-modal")
            .key_context("AddLlmProviderModal")
            .w(rems(34.))
            .elevation_3(cx)
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window, cx);
            }))
            .child(
                Modal::new("configure-context-server", None)
                    .header(
                        ModalHeader::new()
                            .headline(if is_model_editing {
                                "Edit Models"
                            } else {
                                "Add LLM Provider"
                            })
                            .description(match self.provider {
                                LlmCompatibleProvider::OpenAi => {
                                    "This provider will use an OpenAI compatible API."
                                }
                            }),
                    )
                    .when_some(self.last_error.clone(), |this, error| {
                        this.section(
                            Section::new().child(
                                Banner::new()
                                    .severity(Severity::Warning)
                                    .child(div().text_xs().child(error)),
                            ),
                        )
                    })
                    .child(
                        div()
                            .size_full()
                            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
                            .child(
                                v_flex()
                                    .id("modal_content")
                                    .size_full()
                                    .tab_group()
                                    .max_h(modal_max_height)
                                    .pl_3()
                                    .pr_4()
                                    .pb_2()
                                    .gap_2()
                                    .overflow_y_scroll()
                                    .track_scroll(&self.scroll_handle)
                                    .when_else(
                                        is_model_editing,
                                        |this| {
                                            this.child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        Label::new("Provider Name")
                                                            .size(LabelSize::Small),
                                                    )
                                                    .child(
                                                        Label::new(
                                                            self.input
                                                                .provider_name
                                                                .read(cx)
                                                                .text(cx),
                                                        )
                                                        .color(Color::Muted),
                                                    ),
                                            )
                                            .child(
                                                v_flex()
                                                    .gap_1()
                                                    .child(
                                                        Label::new("API URL")
                                                            .size(LabelSize::Small),
                                                    )
                                                    .child(
                                                        Label::new(
                                                            self.input.api_url.read(cx).text(cx),
                                                        )
                                                        .color(Color::Muted),
                                                    ),
                                            )
                                        },
                                        |this| this.child(self.input.provider_name.clone()),
                                    )
                                    .when(!is_model_editing, |this| {
                                        this.child(self.input.api_url.clone())
                                            .child(self.input.api_key.clone())
                                    })
                                    .child(self.render_model_section(cx)),
                            ),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_1()
                                .child(
                                    Button::new("cancel", "Cancel")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Cancel,
                                                &focus_handle,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                        )
                                        .on_click(cx.listener(|this, _event, window, cx| {
                                            this.cancel(&menu::Cancel, window, cx)
                                        })),
                                )
                                .child(
                                    Button::new("save-server", "Save Provider")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Confirm,
                                                &focus_handle,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                        )
                                        .on_click(cx.listener(|this, _event, window, cx| {
                                            this.confirm(&menu::Confirm, window, cx)
                                        })),
                                ),
                        ),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{TestAppContext, UpdateGlobal, VisualTestContext};
    use language_model::{
        LanguageModelProviderId, LanguageModelProviderName, ReasoningEffort,
        fake_provider::FakeLanguageModelProvider,
    };
    use project::Project;
    use settings::{RootUserSettings, SettingsStore};
    use util::path;
    use workspace::MultiWorkspace;

    #[gpui::test]
    async fn test_save_provider_invalid_inputs(cx: &mut TestAppContext) {
        let cx = setup_test(cx).await;

        assert_eq!(
            save_provider_validation_errors("", "someurl", "somekey", vec![], cx,).await,
            Some("Provider Name cannot be empty".into())
        );

        assert_eq!(
            save_provider_validation_errors("someprovider", "", "somekey", vec![], cx,).await,
            Some("API URL cannot be empty".into())
        );

        assert_eq!(
            save_provider_validation_errors("someprovider", "someurl", "", vec![], cx,).await,
            Some("API Key cannot be empty".into())
        );

        assert_eq!(
            save_provider_validation_errors(
                "someprovider",
                "someurl",
                "somekey",
                vec![("", "200000", "200000", "32000")],
                cx,
            )
            .await,
            Some("Model Name cannot be empty".into())
        );

        assert_eq!(
            save_provider_validation_errors(
                "someprovider",
                "someurl",
                "somekey",
                vec![("somemodel", "abc", "200000", "32000")],
                cx,
            )
            .await,
            Some("Max Tokens must be a number".into())
        );

        assert_eq!(
            save_provider_validation_errors(
                "someprovider",
                "someurl",
                "somekey",
                vec![("somemodel", "200000", "abc", "32000")],
                cx,
            )
            .await,
            Some("Max Completion Tokens must be a number".into())
        );

        assert_eq!(
            save_provider_validation_errors(
                "someprovider",
                "someurl",
                "somekey",
                vec![("somemodel", "200000", "200000", "abc")],
                cx,
            )
            .await,
            Some("Max Output Tokens must be a number".into())
        );

        assert_eq!(
            save_provider_validation_errors(
                "someprovider",
                "someurl",
                "somekey",
                vec![("somemodel", "200000", "", "32000")],
                cx,
            )
            .await,
            Some("Max Completion Tokens must be a number".into())
        );

        assert_eq!(
            save_provider_validation_errors(
                "someprovider",
                "someurl",
                "somekey",
                vec![("somemodel", "200000", "200000", "")],
                cx,
            )
            .await,
            Some("Max Output Tokens must be a number".into())
        );

        assert_eq!(
            save_provider_validation_errors(
                "someprovider",
                "someurl",
                "somekey",
                vec![
                    ("somemodel", "200000", "200000", "32000"),
                    ("somemodel", "200000", "200000", "32000"),
                ],
                cx,
            )
            .await,
            Some("Model Names must be unique".into())
        );
    }

    #[gpui::test]
    async fn test_save_provider_name_conflict(cx: &mut TestAppContext) {
        let cx = setup_test(cx).await;

        cx.update(|_window, cx| {
            LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                registry.register_provider(
                    Arc::new(FakeLanguageModelProvider::new(
                        LanguageModelProviderId::new("someprovider"),
                        LanguageModelProviderName::new("Some Provider"),
                    )),
                    cx,
                );
            });
        });

        assert_eq!(
            save_provider_validation_errors(
                "someprovider",
                "someurl",
                "someapikey",
                vec![("somemodel", "200000", "200000", "32000")],
                cx,
            )
            .await,
            Some("Provider Name is already taken by another provider".into())
        );
    }

    #[gpui::test]
    async fn test_edit_models_allows_existing_provider_name(cx: &mut TestAppContext) {
        let cx = setup_test(cx).await;

        cx.update(|_window, cx| {
            LanguageModelRegistry::global(cx).update(cx, |registry, cx| {
                registry.register_provider(
                    Arc::new(FakeLanguageModelProvider::new(
                        LanguageModelProviderId::new("someprovider"),
                        LanguageModelProviderName::new("Some Provider"),
                    )),
                    cx,
                );
            });
        });

        let result = cx.update(|window, cx| {
            let settings = OpenAiCompatibleSettingsContent {
                api_url: "https://example.com/v1".to_string(),
                available_models: vec![AvailableModel {
                    name: "somemodel".to_string(),
                    display_name: None,
                    max_tokens: 200000,
                    max_output_tokens: None,
                    max_completion_tokens: None,
                    reasoning_effort: None,
                    capabilities: ModelCapabilities::default(),
                }],
            };
            let input =
                AddLlmProviderInput::existing(Arc::from("someprovider"), settings, window, cx);

            provider_settings_from_input(
                &input,
                SaveMode::EditModels(ModelEditContext {
                    provider_name: Arc::from("someprovider"),
                    api_url: "https://example.com/v1".to_string(),
                }),
                cx,
            )
        });

        let (provider_name, settings) = result.expect("edit validation should succeed");
        assert_eq!(provider_name.as_ref(), "someprovider");
        assert_eq!(settings.api_url, "https://example.com/v1");
        assert_eq!(settings.available_models.len(), 1);
    }

    #[gpui::test]
    async fn test_existing_provider_input_prefills_settings(cx: &mut TestAppContext) {
        let cx = setup_test(cx).await;

        cx.update(|window, cx| {
            let settings = OpenAiCompatibleSettingsContent {
                api_url: "https://example.com/v1".to_string(),
                available_models: vec![AvailableModel {
                    name: "somemodel".to_string(),
                    display_name: Some("Some Model".to_string()),
                    max_tokens: 200000,
                    max_output_tokens: Some(32000),
                    max_completion_tokens: None,
                    reasoning_effort: Some(ReasoningEffort::High),
                    capabilities: ModelCapabilities {
                        tools: false,
                        images: true,
                        parallel_tool_calls: true,
                        prompt_cache_key: true,
                        chat_completions: false,
                        interleaved_reasoning: true,
                    },
                }],
            };

            let input =
                AddLlmProviderInput::existing(Arc::from("someprovider"), settings, window, cx);

            assert_eq!(input.provider_name.read(cx).text(cx), "someprovider");
            assert_eq!(input.api_url.read(cx).text(cx), "https://example.com/v1");
            assert_eq!(input.models.len(), 1);

            let model = input
                .models
                .first()
                .expect("existing model should be present");
            assert_eq!(model.name.read(cx).text(cx), "somemodel");
            assert_eq!(model.max_completion_tokens.read(cx).text(cx), "");
            assert_eq!(model.max_output_tokens.read(cx).text(cx), "32000");
            assert_eq!(model.max_tokens.read(cx).text(cx), "200000");
            assert_eq!(model.capabilities.supports_tools, ToggleState::Unselected);
            assert_eq!(model.capabilities.supports_images, ToggleState::Selected);
            assert_eq!(
                model.capabilities.supports_parallel_tool_calls,
                ToggleState::Selected
            );
            assert_eq!(
                model.capabilities.supports_prompt_cache_key,
                ToggleState::Selected
            );
            assert_eq!(
                model.capabilities.supports_chat_completions,
                ToggleState::Unselected
            );

            let parsed_model = model.parse(true, cx).unwrap();
            assert_eq!(parsed_model.display_name, Some("Some Model".to_string()));
            assert_eq!(parsed_model.max_completion_tokens, None);
            assert_eq!(parsed_model.max_output_tokens, Some(32000));
            assert_eq!(parsed_model.reasoning_effort, Some(ReasoningEffort::High));
            assert!(parsed_model.capabilities.interleaved_reasoning);
        });
    }

    #[gpui::test]
    async fn test_create_then_edit_model_settings(cx: &mut TestAppContext) {
        let cx = setup_test(cx).await;

        let create_task = cx.update(|window, cx| {
            let mut input = AddLlmProviderInput::new(LlmCompatibleProvider::OpenAi, window, cx);
            set_input_text(&input.provider_name, "someprovider", window, cx);
            set_input_text(&input.api_url, "https://example.com/v1", window, cx);
            set_input_text(&input.api_key, "someapikey", window, cx);

            let model = input.models.first_mut().expect("default model input");
            set_input_text(&model.name, "somemodel", window, cx);
            save_provider_to_settings(&input, SaveMode::Create, cx)
        });
        create_task.await.unwrap();
        cx.run_until_parked();

        let created_settings = settings_from_user_file(cx).await;
        let created_provider = created_settings
            .language_models
            .unwrap()
            .openai_compatible
            .unwrap()
            .remove("someprovider")
            .expect("created provider should be present");

        assert_eq!(created_provider.api_url, "https://example.com/v1");
        assert_eq!(created_provider.available_models.len(), 1);
        assert_eq!(created_provider.available_models[0].name, "somemodel");

        let existing_settings_json = serde_json::json!({
            "language_models": {
                "openai_compatible": {
                    "someprovider": {
                        "api_url": created_provider.api_url,
                        "available_models": [
                            {
                                "name": created_provider.available_models[0].name,
                                "display_name": "Some Model",
                                "max_tokens": created_provider.available_models[0].max_tokens,
                                "max_output_tokens": created_provider.available_models[0].max_output_tokens,
                                "max_completion_tokens": created_provider.available_models[0].max_completion_tokens,
                                "reasoning_effort": "high",
                                "capabilities": {
                                    "tools": true,
                                    "images": true,
                                    "parallel_tool_calls": true,
                                    "prompt_cache_key": true,
                                    "chat_completions": true,
                                    "interleaved_reasoning": true
                                }
                            }
                        ]
                    }
                }
            }
        })
        .to_string();

        let existing_provider = cx.update(|_window, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store
                    .set_user_settings(&existing_settings_json, cx)
                    .unwrap();
            });
            let (settings, _) = settings::SettingsContent::parse_json(&existing_settings_json);
            settings
                .unwrap()
                .language_models
                .unwrap()
                .openai_compatible
                .unwrap()
                .remove("someprovider")
                .unwrap()
        });

        let edit_task = cx.update(|window, cx| {
            let mut input = AddLlmProviderInput::existing(
                Arc::from("someprovider"),
                existing_provider,
                window,
                cx,
            );
            input.add_model(window, cx);
            let new_model = input.models.last_mut().expect("new model input");
            set_input_text(&new_model.name, "newmodel", window, cx);
            set_input_text(
                &input.api_url,
                "https://ignored.example.test/v2",
                window,
                cx,
            );

            save_provider_to_settings(
                &input,
                SaveMode::EditModels(ModelEditContext {
                    provider_name: Arc::from("someprovider"),
                    api_url: "https://example.com/v1".to_string(),
                }),
                cx,
            )
        });
        edit_task.await.unwrap();
        cx.run_until_parked();

        let edited_settings = settings_from_user_file(cx).await;
        let openai_compatible = edited_settings
            .language_models
            .unwrap()
            .openai_compatible
            .unwrap();
        assert_eq!(openai_compatible.len(), 1);

        let edited_provider = openai_compatible
            .get("someprovider")
            .expect("edited provider should keep the same key");
        assert_eq!(edited_provider.api_url, "https://example.com/v1");
        assert_eq!(edited_provider.available_models.len(), 2);

        let edited_model = &edited_provider.available_models[0];
        assert_eq!(edited_model.name, "somemodel");
        assert_eq!(edited_model.display_name.as_deref(), Some("Some Model"));
        assert_eq!(edited_model.reasoning_effort, Some(ReasoningEffort::High));
        assert!(edited_model.capabilities.interleaved_reasoning);

        let new_model = &edited_provider.available_models[1];
        assert_eq!(new_model.name, "newmodel");
        assert_eq!(new_model.display_name, None);
        assert_eq!(new_model.reasoning_effort, None);
        assert!(!new_model.capabilities.interleaved_reasoning);
    }

    #[gpui::test]
    async fn test_model_input_default_capabilities(cx: &mut TestAppContext) {
        let cx = setup_test(cx).await;

        cx.update(|window, cx| {
            let model_input = ModelInput::new(0, window, cx);
            model_input.name.update(cx, |input, cx| {
                input.set_text("somemodel", window, cx);
            });
            assert_eq!(
                model_input.capabilities.supports_tools,
                ToggleState::Selected
            );
            assert_eq!(
                model_input.capabilities.supports_images,
                ToggleState::Unselected
            );
            assert_eq!(
                model_input.capabilities.supports_parallel_tool_calls,
                ToggleState::Unselected
            );
            assert_eq!(
                model_input.capabilities.supports_prompt_cache_key,
                ToggleState::Unselected
            );
            assert_eq!(
                model_input.capabilities.supports_chat_completions,
                ToggleState::Selected
            );

            let parsed_model = model_input.parse(false, cx).unwrap();
            assert!(parsed_model.capabilities.tools);
            assert!(!parsed_model.capabilities.images);
            assert!(!parsed_model.capabilities.parallel_tool_calls);
            assert!(!parsed_model.capabilities.prompt_cache_key);
            assert!(parsed_model.capabilities.chat_completions);
        });
    }

    #[gpui::test]
    async fn test_model_input_deselected_capabilities(cx: &mut TestAppContext) {
        let cx = setup_test(cx).await;

        cx.update(|window, cx| {
            let mut model_input = ModelInput::new(0, window, cx);
            model_input.name.update(cx, |input, cx| {
                input.set_text("somemodel", window, cx);
            });

            model_input.capabilities.supports_tools = ToggleState::Unselected;
            model_input.capabilities.supports_images = ToggleState::Unselected;
            model_input.capabilities.supports_parallel_tool_calls = ToggleState::Unselected;
            model_input.capabilities.supports_prompt_cache_key = ToggleState::Unselected;
            model_input.capabilities.supports_chat_completions = ToggleState::Unselected;

            let parsed_model = model_input.parse(false, cx).unwrap();
            assert!(!parsed_model.capabilities.tools);
            assert!(!parsed_model.capabilities.images);
            assert!(!parsed_model.capabilities.parallel_tool_calls);
            assert!(!parsed_model.capabilities.prompt_cache_key);
            assert!(!parsed_model.capabilities.chat_completions);
        });
    }

    #[gpui::test]
    async fn test_model_input_with_name_and_capabilities(cx: &mut TestAppContext) {
        let cx = setup_test(cx).await;

        cx.update(|window, cx| {
            let mut model_input = ModelInput::new(0, window, cx);
            model_input.name.update(cx, |input, cx| {
                input.set_text("somemodel", window, cx);
            });

            model_input.capabilities.supports_tools = ToggleState::Selected;
            model_input.capabilities.supports_images = ToggleState::Unselected;
            model_input.capabilities.supports_parallel_tool_calls = ToggleState::Selected;
            model_input.capabilities.supports_prompt_cache_key = ToggleState::Unselected;
            model_input.capabilities.supports_chat_completions = ToggleState::Selected;

            let parsed_model = model_input.parse(false, cx).unwrap();
            assert_eq!(parsed_model.name, "somemodel");
            assert!(parsed_model.capabilities.tools);
            assert!(!parsed_model.capabilities.images);
            assert!(parsed_model.capabilities.parallel_tool_calls);
            assert!(!parsed_model.capabilities.prompt_cache_key);
            assert!(parsed_model.capabilities.chat_completions);
        });
    }

    async fn setup_test(cx: &mut TestAppContext) -> &mut VisualTestContext {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);

            language_model::init(cx);
            editor::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| <dyn Fs>::set_global(fs.clone(), cx));
        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let _workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx
    }

    async fn save_provider_validation_errors(
        provider_name: &str,
        api_url: &str,
        api_key: &str,
        models: Vec<(&str, &str, &str, &str)>,
        cx: &mut VisualTestContext,
    ) -> Option<SharedString> {
        let task = cx.update(|window, cx| {
            let mut input = AddLlmProviderInput::new(LlmCompatibleProvider::OpenAi, window, cx);
            set_input_text(&input.provider_name, provider_name, window, cx);
            set_input_text(&input.api_url, api_url, window, cx);
            set_input_text(&input.api_key, api_key, window, cx);

            for (i, (name, max_tokens, max_completion_tokens, max_output_tokens)) in
                models.iter().enumerate()
            {
                if i >= input.models.len() {
                    input.models.push(ModelInput::new(i, window, cx));
                }
                let model = &mut input.models[i];
                set_input_text(&model.name, name, window, cx);
                set_input_text(&model.max_tokens, max_tokens, window, cx);
                set_input_text(
                    &model.max_completion_tokens,
                    max_completion_tokens,
                    window,
                    cx,
                );
                set_input_text(&model.max_output_tokens, max_output_tokens, window, cx);
            }
            save_provider_to_settings(&input, SaveMode::Create, cx)
        });

        task.await.err()
    }

    fn set_input_text(input: &Entity<InputField>, text: &str, window: &mut Window, cx: &mut App) {
        input.update(cx, |input, cx| {
            input.set_text(text, window, cx);
        });
    }

    async fn settings_from_user_file(cx: &mut VisualTestContext) -> settings::SettingsContent {
        let fs = cx.update(|_window, cx| <dyn Fs>::global(cx));
        let settings_content = fs.load(paths::settings_file().as_path()).await.unwrap();
        let (settings, _) = settings::SettingsContent::parse_json(&settings_content);
        settings.unwrap()
    }
}
