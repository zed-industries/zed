use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render};
use ui::{KeyBinding, Modal, ModalFooter, ModalHeader, Section, prelude::*};
use ui_input::SingleLineInput;
use workspace::{ModalView, Workspace};

pub enum LlmCompatibleProvider {
    OpenAi,
}

impl LlmCompatibleProvider {
    fn name(&self) -> &'static str {
        match self {
            LlmCompatibleProvider::OpenAi => "OpenAI",
        }
    }

    fn base_url(&self) -> &'static str {
        match self {
            LlmCompatibleProvider::OpenAi => "https://api.openai.com/v1",
        }
    }
}

struct ConfiguredModel {
    name: Entity<SingleLineInput>,
    max_completion_tokens: Entity<SingleLineInput>,
    max_output_tokens: Entity<SingleLineInput>,
    max_tokens: Entity<SingleLineInput>,
}

impl ConfiguredModel {
    fn new(window: &mut Window, cx: &mut App) -> Self {
        let model_name = single_line_input(
            "Model Name",
            "e.g. gpt-4o, claude-opus-4, gemini-2.5-pro",
            None,
            window,
            cx,
        );
        let max_completion_tokens = single_line_input(
            "Max Completion Tokens",
            "200000",
            Some("200000"),
            window,
            cx,
        );
        let max_output_tokens = single_line_input(
            "Max Output Tokens",
            "Max Output Tokens",
            Some("32000"),
            window,
            cx,
        );
        let max_tokens = single_line_input("Max Tokens", "Max Tokens", Some("200000"), window, cx);
        Self {
            name: model_name,
            max_completion_tokens,
            max_output_tokens,
            max_tokens,
        }
    }
}

fn single_line_input(
    label: impl Into<SharedString>,
    placeholder: impl Into<SharedString>,
    text: Option<&str>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<SingleLineInput> {
    cx.new(|cx| {
        let input = SingleLineInput::new(window, cx, placeholder).label(label);
        if let Some(text) = text {
            input
                .editor()
                .update(cx, |editor, cx| editor.set_text(text, window, cx));
        }
        input
    })
}

pub struct AddLlmProviderModal {
    provider: LlmCompatibleProvider,
    provider_name_input: Entity<SingleLineInput>,
    base_url_input: Entity<SingleLineInput>,
    api_key_input: Entity<SingleLineInput>,
    configured_models: Vec<ConfiguredModel>,
    focus_handle: FocusHandle,
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

    fn new(provider: LlmCompatibleProvider, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let provider_name_input = single_line_input(
            "Provider Name",
            provider.name(),
            Some(provider.name()),
            window,
            cx,
        );
        let base_url_input = single_line_input("Base URL", provider.base_url(), None, window, cx);
        let api_key_input = single_line_input("API Key", "00000000", None, window, cx);

        Self {
            provider,
            provider_name_input,
            base_url_input,
            api_key_input,
            configured_models: vec![ConfiguredModel::new(window, cx)],
            focus_handle: cx.focus_handle(),
        }
    }

    fn confirm(&mut self, _: &menu::Confirm, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn render_section(&self) -> Section {
        Section::new()
            .child(
                Label::new(match self.provider {
                    LlmCompatibleProvider::OpenAi => {
                        "This provider will use an OpenAI compatible API."
                    }
                })
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .child(self.provider_name_input.clone())
            .child(self.base_url_input.clone())
            .child(self.api_key_input.clone())
    }

    fn render_model_section(&self, cx: &mut Context<Self>) -> Section {
        Section::new().contained(true).child(
            v_flex()
                .gap_2()
                .child(
                    h_flex()
                        .justify_between()
                        .child(Label::new("Models"))
                        .child(
                            Button::new("add-model", "Add Model")
                                .icon(IconName::Plus)
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.configured_models
                                        .push(ConfiguredModel::new(window, cx));
                                    cx.notify();
                                })),
                        ),
                )
                .children(
                    self.configured_models
                        .iter()
                        .map(|model| self.render_model(model, cx)),
                ),
        )
    }

    fn render_model(
        &self,
        model: &ConfiguredModel,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        v_flex()
            .gap_2()
            .child(model.name.clone())
            .child(
                h_flex()
                    .gap_2()
                    .child(model.max_completion_tokens.clone())
                    .child(model.max_output_tokens.clone()),
            )
            .child(model.max_tokens.clone())
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

        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("AddLlmProviderModal")
            .on_action(cx.listener(Self::cancel))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
            }))
            .child(
                Modal::new("configure-context-server", None)
                    .header(ModalHeader::new().headline("Add LLM Provider"))
                    .section(self.render_section())
                    .section(self.render_model_section(cx))
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("cancel", "Cancel")
                                        .key_binding(
                                            KeyBinding::for_action_in(
                                                &menu::Cancel,
                                                &focus_handle,
                                                window,
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
                                                window,
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
