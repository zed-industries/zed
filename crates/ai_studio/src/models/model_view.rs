// Model view components will be implemented here
// For now, model UI is handled in ai_studio_view.rs 

use anyhow::Result;
use gpui::{
    App, Context, EventEmitter, Focusable, FocusHandle, Render, IntoElement,
    Window, div, prelude::*, Entity, 
};
use ui::{prelude::*, Button, ButtonStyle, Label, LabelSize};
use ui_input::SingleLineInput;

use super::{ModelConfig, ModelType, ModelProvider, ModelParameters, ModelCapability};

/// Modal for creating a new model configuration
pub struct ModelCreationModal {
    focus_handle: FocusHandle,
    
    // Form fields - using Entity<Editor> for proper text input
    name_input: Entity<SingleLineInput>,
    description_input: Entity<SingleLineInput>,
    api_key_input: Entity<SingleLineInput>,
    base_url_input: Entity<SingleLineInput>,
    model_path_input: Entity<SingleLineInput>,
    model_name_input: Entity<SingleLineInput>,
    max_tokens_input: Entity<SingleLineInput>,
    temperature_input: Entity<SingleLineInput>,
    
    model_type: ModelType,
    provider_type: ProviderType,
    
    // State
    is_submitting: bool,
    validation_errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    Local,
    Ollama,
    HuggingFace,
    Custom,
}

impl ModelCreationModal {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            name_input: cx.new(|cx| SingleLineInput::new(window, cx, "Model name")),
            description_input: cx.new(|cx| SingleLineInput::new(window, cx, "Description (optional)")),
            api_key_input: cx.new(|cx| SingleLineInput::new(window, cx, "API Key")),
            base_url_input: cx.new(|cx| SingleLineInput::new(window, cx, "Base URL (optional)")),
            model_path_input: cx.new(|cx| SingleLineInput::new(window, cx, "Path to model file")),
            model_name_input: cx.new(|cx| SingleLineInput::new(window, cx, "Model name")),
            max_tokens_input: cx.new(|cx| SingleLineInput::new(window, cx, "4096")),
            temperature_input: cx.new(|cx| SingleLineInput::new(window, cx, "0.7")),
            model_type: ModelType::LanguageModel,
            provider_type: ProviderType::OpenAI,
            is_submitting: false,
            validation_errors: Vec::new(),
        }
    }

    pub fn from_existing_model(model: ModelConfig, window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Convert ModelProvider to ProviderType
        let provider_type = match &model.provider {
            ModelProvider::OpenAI { .. } => ProviderType::OpenAI,
            ModelProvider::Anthropic { .. } => ProviderType::Anthropic,
            ModelProvider::Local { .. } => ProviderType::Local,
            ModelProvider::Ollama { .. } => ProviderType::Ollama,
            ModelProvider::HuggingFace { .. } => ProviderType::HuggingFace,
            ModelProvider::Custom { .. } => ProviderType::Custom,
        };

        // Extract provider-specific fields
        let (api_key, base_url, model_path, model_name) = match &model.provider {
            ModelProvider::OpenAI { api_key, base_url, .. } => {
                (api_key.clone(), base_url.clone().unwrap_or_default(), String::new(), String::new())
            }
            ModelProvider::Anthropic { api_key } => {
                (api_key.clone(), String::new(), String::new(), String::new())
            }
            ModelProvider::Local { model_path, .. } => {
                (String::new(), String::new(), model_path.clone(), String::new())
            }
            ModelProvider::Ollama { base_url, model_name } => {
                (String::new(), base_url.clone(), String::new(), model_name.clone())
            }
            ModelProvider::HuggingFace { api_key, model_id } => {
                (api_key.clone().unwrap_or_default(), String::new(), String::new(), model_id.clone())
            }
            ModelProvider::Custom { .. } => {
                (String::new(), String::new(), String::new(), String::new())
            }
        };

        // Create inputs with pre-filled values
        let name_input = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "Model name");
            input.editor.update(cx, |editor, cx| {
                editor.set_text(model.name.clone(), window, cx);
            });
            input
        });

        let description_input = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "Description (optional)");
            input.editor.update(cx, |editor, cx| {
                editor.set_text(model.description.clone(), window, cx);
            });
            input
        });

        let api_key_input = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "API Key");
            input.editor.update(cx, |editor, cx| {
                editor.set_text(api_key, window, cx);
            });
            input
        });

        let base_url_input = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "Base URL (optional)");
            input.editor.update(cx, |editor, cx| {
                editor.set_text(base_url, window, cx);
            });
            input
        });

        let model_path_input = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "Path to model file");
            input.editor.update(cx, |editor, cx| {
                editor.set_text(model_path, window, cx);
            });
            input
        });

        let model_name_input = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "Model name");
            input.editor.update(cx, |editor, cx| {
                editor.set_text(model_name, window, cx);
            });
            input
        });

        let max_tokens_input = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "4096");
            if let Some(max_tokens) = model.parameters.max_tokens {
                input.editor.update(cx, |editor, cx| {
                    editor.set_text(max_tokens.to_string(), window, cx);
                });
            }
            input
        });

        let temperature_input = cx.new(|cx| {
            let input = SingleLineInput::new(window, cx, "0.7");
            if let Some(temperature) = model.parameters.temperature {
                input.editor.update(cx, |editor, cx| {
                    editor.set_text(temperature.to_string(), window, cx);
                });
            }
            input
        });

        Self {
            focus_handle: cx.focus_handle(),
            name_input,
            description_input,
            api_key_input,
            base_url_input,
            model_path_input,
            model_name_input,
            max_tokens_input,
            temperature_input,
            model_type: model.model_type,
            provider_type,
            is_submitting: false,
            validation_errors: Vec::new(),
        }
    }

    fn get_name(&self, cx: &App) -> String {
        self.name_input.read(cx).editor().read(cx).text(cx)
    }

    fn get_description(&self, cx: &App) -> String {
        self.description_input.read(cx).editor().read(cx).text(cx)
    }

    fn get_api_key(&self, cx: &App) -> String {
        self.api_key_input.read(cx).editor().read(cx).text(cx)
    }

    fn get_base_url(&self, cx: &App) -> String {
        self.base_url_input.read(cx).editor().read(cx).text(cx)
    }

    fn get_model_path(&self, cx: &App) -> String {
        self.model_path_input.read(cx).editor().read(cx).text(cx)
    }

    fn get_model_name(&self, cx: &App) -> String {
        self.model_name_input.read(cx).editor().read(cx).text(cx)
    }

    fn get_max_tokens(&self, cx: &App) -> String {
        self.max_tokens_input.read(cx).editor().read(cx).text(cx)
    }

    fn get_temperature(&self, cx: &App) -> String {
        self.temperature_input.read(cx).editor().read(cx).text(cx)
    }

    fn validate(&mut self, cx: &App) -> bool {
        self.validation_errors.clear();
        
        if self.get_name(cx).trim().is_empty() {
            self.validation_errors.push("Name is required".to_string());
        }
        
        match self.provider_type {
            ProviderType::OpenAI | ProviderType::Anthropic => {
                if self.get_api_key(cx).trim().is_empty() {
                    self.validation_errors.push("API key is required".to_string());
                }
            }
            ProviderType::Local => {
                if self.get_model_path(cx).trim().is_empty() {
                    self.validation_errors.push("Model path is required".to_string());
                }
            }
            ProviderType::Ollama => {
                if self.get_model_name(cx).trim().is_empty() {
                    self.validation_errors.push("Model name is required".to_string());
                }
            }
            ProviderType::HuggingFace => {
                if self.get_model_name(cx).trim().is_empty() {
                    self.validation_errors.push("Model ID is required".to_string());
                }
            }
            ProviderType::Custom => {
                if self.get_model_name(cx).trim().is_empty() {
                    self.validation_errors.push("Provider name is required".to_string());
                }
            }
        }
        
        self.validation_errors.is_empty()
    }

    fn build_model_config(&self, cx: &App) -> Result<ModelConfig> {
        let provider = match self.provider_type {
            ProviderType::OpenAI => ModelProvider::OpenAI {
                api_key: self.get_api_key(cx),
                base_url: if self.get_base_url(cx).trim().is_empty() { 
                    None 
                } else { 
                    Some(self.get_base_url(cx)) 
                },
                organization: None,
            },
            ProviderType::Anthropic => ModelProvider::Anthropic {
                api_key: self.get_api_key(cx),
            },
            ProviderType::Local => ModelProvider::Local {
                model_path: self.get_model_path(cx),
                executable_path: None,
            },
            ProviderType::Ollama => ModelProvider::Ollama {
                base_url: if self.get_base_url(cx).trim().is_empty() { 
                    "http://localhost:11434".to_string() 
                } else { 
                    self.get_base_url(cx) 
                },
                model_name: self.get_model_name(cx),
            },
            ProviderType::HuggingFace => ModelProvider::HuggingFace {
                api_key: if self.get_api_key(cx).trim().is_empty() { None } else { Some(self.get_api_key(cx)) },
                model_id: self.get_model_name(cx),
            },
            ProviderType::Custom => ModelProvider::Custom {
                name: self.get_model_name(cx),
                config: serde_json::Value::Object(serde_json::Map::new()),
            },
        };

        let mut parameters = ModelParameters::default();
        
        if let Ok(tokens) = self.get_max_tokens(cx).parse::<u32>() {
            parameters.max_tokens = Some(tokens);
        }
        if let Ok(temp) = self.get_temperature(cx).parse::<f32>() {
            parameters.temperature = Some(temp);
        }

        let capabilities = vec![
            ModelCapability::TextGeneration,
            ModelCapability::StreamingResponse,
        ];

        let config = ModelConfig::new(self.get_name(cx), self.model_type.clone(), provider)
            .with_description(self.get_description(cx))
            .with_parameters(parameters)
            .with_capabilities(capabilities);

        Ok(config)
    }

    fn render_provider_fields(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        match self.provider_type {
            ProviderType::OpenAI => {
                v_flex()
                    .gap_3()
                    .child(
                        div()
                            .child(Label::new("API Key *").size(LabelSize::Small))
                            .child(self.api_key_input.clone())
                    )
                    .child(
                        div()
                            .child(Label::new("Base URL (Optional)").size(LabelSize::Small))
                            .child(self.base_url_input.clone())
                    )
                    .into_any_element()
            }
            ProviderType::Anthropic => {
                div()
                    .child(Label::new("API Key *").size(LabelSize::Small))
                    .child(self.api_key_input.clone())
                    .into_any_element()
            }
            ProviderType::Local => {
                div()
                    .child(Label::new("Model Path *").size(LabelSize::Small))
                    .child(self.model_path_input.clone())
                    .into_any_element()
            }
            ProviderType::Ollama => {
                v_flex()
                    .gap_3()
                    .child(
                        div()
                            .child(Label::new("Base URL").size(LabelSize::Small))
                            .child(self.base_url_input.clone())
                    )
                    .child(
                        div()
                            .child(Label::new("Model Name *").size(LabelSize::Small))
                            .child(self.model_name_input.clone())
                    )
                    .into_any_element()
            }
            ProviderType::HuggingFace => {
                div()
                    .child(Label::new("Model ID *").size(LabelSize::Small))
                    .child(self.model_name_input.clone())
                    .into_any_element()
            }
            ProviderType::Custom => {
                div()
                    .child(Label::new("Provider Name *").size(LabelSize::Small))
                    .child(self.model_name_input.clone())
                    .into_any_element()
            }
        }
    }
}

impl EventEmitter<Option<ModelConfig>> for ModelCreationModal {}

impl Focusable for ModelCreationModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ModelCreationModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_96()
            .max_w_full()
            .bg(cx.theme().colors().background)
            .border_1()
            .border_color(cx.theme().colors().border)
            .rounded_lg()
            .shadow_lg()
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Label::new("Create New Model")
                            .size(LabelSize::Large)
                    )
                    .child(
                        Button::new("close", "×")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(None);
                            }))
                    )
            )
            .child(
                // Form content
                div()
                    .id("modal_form_content")
                    .flex()
                    .flex_col()
                    .gap_4()
                    .p_4()
                    .max_h_96()
                    .overflow_y_scroll()
                    .child(
                        // Basic info
                        v_flex()
                            .gap_3()
                            .child(
                                div()
                                    .child(Label::new("Name *").size(LabelSize::Small))
                                    .child(self.name_input.clone())
                            )
                            .child(
                                div()
                                    .child(Label::new("Description").size(LabelSize::Small))
                                    .child(self.description_input.clone())
                            )
                    )
                    .child(
                        // Model type selection
                        div()
                            .child(Label::new("Model Type").size(LabelSize::Small))
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        Button::new("language_model", "Language")
                                            .style(if matches!(self.model_type, ModelType::LanguageModel) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.model_type = ModelType::LanguageModel;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Button::new("code_model", "Code")
                                            .style(if matches!(self.model_type, ModelType::CodeModel) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.model_type = ModelType::CodeModel;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Button::new("multimodal", "MultiModal")
                                            .style(if matches!(self.model_type, ModelType::MultiModal) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.model_type = ModelType::MultiModal;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
                    .child(
                        // Provider selection
                        div()
                            .child(Label::new("Provider").size(LabelSize::Small))
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_2()
                                    .child(
                                        Button::new("openai", "OpenAI")
                                            .style(if matches!(self.provider_type, ProviderType::OpenAI) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.provider_type = ProviderType::OpenAI;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Button::new("anthropic", "Anthropic")
                                            .style(if matches!(self.provider_type, ProviderType::Anthropic) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.provider_type = ProviderType::Anthropic;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Button::new("ollama", "Ollama")
                                            .style(if matches!(self.provider_type, ProviderType::Ollama) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.provider_type = ProviderType::Ollama;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Button::new("local", "Local/llama.cpp")
                                            .style(if matches!(self.provider_type, ProviderType::Local) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.provider_type = ProviderType::Local;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Button::new("huggingface", "HuggingFace")
                                            .style(if matches!(self.provider_type, ProviderType::HuggingFace) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.provider_type = ProviderType::HuggingFace;
                                                cx.notify();
                                            }))
                                    )
                                    .child(
                                        Button::new("custom", "Custom")
                                            .style(if matches!(self.provider_type, ProviderType::Custom) { 
                                                ButtonStyle::Filled 
                                            } else { 
                                                ButtonStyle::Subtle 
                                            })
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.provider_type = ProviderType::Custom;
                                                cx.notify();
                                            }))
                                    )
                            )
                    )
                    .child(
                        // Provider-specific fields
                        div()
                            .child(Label::new("Provider Configuration").size(LabelSize::Small))
                            .child(self.render_provider_fields(cx))
                    )
                    .child(
                        // Model parameters
                        div()
                            .child(Label::new("Parameters").size(LabelSize::Small))
                            .child(
                                div()
                                    .flex()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex_1()
                                            .child(Label::new("Max Tokens").size(LabelSize::XSmall))
                                            .child(self.max_tokens_input.clone())
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .child(Label::new("Temperature").size(LabelSize::XSmall))
                                            .child(self.temperature_input.clone())
                                    )
                            )
                    )
                    .when(!self.validation_errors.is_empty(), |element| {
                        element.child(
                            div()
                                .p_3()
                                .bg(Color::Error.color(cx).alpha(0.1))
                                .border_1()
                                .border_color(Color::Error.color(cx))
                                .rounded_md()
                                .children(
                                    self.validation_errors.iter().map(|error| {
                                        div().child(
                                            Label::new(format!("• {}", error))
                                                .size(LabelSize::Small)
                                                .color(Color::Error)
                                        )
                                    })
                                )
                        )
                    })
            )
            .child(
                // Footer
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .p_4()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        Button::new("cancel", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|_, _, _, cx| {
                                cx.emit(None);
                            }))
                    )
                    .child(
                        Button::new("create", "Create Model")
                            .style(ButtonStyle::Filled)
                            .disabled(self.is_submitting)
                            .on_click(cx.listener(|this, _, _, cx| {
                                if this.validate(cx) {
                                    this.is_submitting = true;
                                    cx.notify();
                                    
                                    match this.build_model_config(cx) {
                                        Ok(config) => {
                                            cx.emit(Some(config));
                                        }
                                        Err(e) => {
                                            this.validation_errors.push(format!("Failed to create model: {}", e));
                                            this.is_submitting = false;
                                            cx.notify();
                                        }
                                    }
                                }
                            }))
                    )
            )
    }
} 