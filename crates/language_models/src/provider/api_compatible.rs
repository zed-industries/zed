use std::sync::Arc;

use ::util::ResultExt;
use anyhow::Result;
use convert_case::{Case, Casing};
use credentials_provider::CredentialsProvider;
use gpui::{App, AppContext as _, Context, Entity, SharedString, Task, TaskExt, Window};
use language_model::{ApiKeyState, AuthenticateError, EnvVar};
use settings::SettingsStore;
use ui::{ElevationIndex, Tooltip, prelude::*};
use ui_input::InputField;

pub trait ApiCompatibleProviderSettings: Clone + Default + PartialEq + 'static {
    fn api_url(&self) -> &str;
}

pub struct ApiCompatibleProviderState<S: ApiCompatibleProviderSettings> {
    id: Arc<str>,
    pub api_key_state: ApiKeyState,
    pub settings: S,
    credentials_provider: Arc<dyn CredentialsProvider>,
}

impl<S: ApiCompatibleProviderSettings> ApiCompatibleProviderState<S> {
    pub fn new(
        id: Arc<str>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        resolve_settings: for<'a> fn(&'a str, &'a App) -> Option<&'a S>,
        cx: &mut App,
    ) -> Entity<Self> {
        let api_key_env_var_name: SharedString =
            format!("{}_API_KEY", id).to_case(Case::UpperSnake).into();
        cx.new(|cx| {
            cx.observe_global::<SettingsStore>(move |this: &mut Self, cx| {
                let Some(settings) = resolve_settings(&this.id, cx).cloned() else {
                    return;
                };
                this.update_settings(settings, cx);
            })
            .detach();

            let settings = resolve_settings(&id, cx).cloned().unwrap_or_default();
            Self {
                id,
                api_key_state: ApiKeyState::new(
                    SharedString::new(settings.api_url()),
                    EnvVar::new(api_key_env_var_name),
                ),
                settings,
                credentials_provider,
            }
        })
    }

    pub fn is_authenticated(&self) -> bool {
        self.api_key_state.has_key()
    }

    pub fn set_api_key(
        &mut self,
        api_key: Option<String>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let api_url = SharedString::new(self.settings.api_url());
        self.api_key_state.store(
            api_url,
            api_key,
            |this| &mut this.api_key_state,
            self.credentials_provider.clone(),
            cx,
        )
    }

    pub fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        let api_url = SharedString::new(self.settings.api_url());
        self.api_key_state.load_if_needed(
            api_url,
            |this| &mut this.api_key_state,
            self.credentials_provider.clone(),
            cx,
        )
    }

    pub fn update_settings(&mut self, settings: S, cx: &mut Context<Self>) {
        if self.settings != settings {
            let api_url = SharedString::new(settings.api_url());
            self.api_key_state.handle_url_change(
                api_url,
                |this| &mut this.api_key_state,
                self.credentials_provider.clone(),
                cx,
            );
            self.settings = settings;
            cx.notify();
        }
    }
}

pub struct ApiCompatibleProviderConfigurationView<S: ApiCompatibleProviderSettings> {
    api_key_editor: Entity<InputField>,
    state: Entity<ApiCompatibleProviderState<S>>,
    provider_name: &'static str,
    load_credentials_task: Option<Task<()>>,
}

impl<S: ApiCompatibleProviderSettings> ApiCompatibleProviderConfigurationView<S> {
    pub fn new(
        state: Entity<ApiCompatibleProviderState<S>>,
        provider_name: &'static str,
        placeholder_text: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let api_key_editor = cx.new(|cx| InputField::new(window, cx, placeholder_text));

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn_in(window, {
            let state = state.clone();
            async move |this, cx| {
                let task = state.update(cx, |state, cx| state.authenticate(cx));
                match task.await {
                    Ok(()) | Err(AuthenticateError::CredentialsNotFound) => {}
                    Err(error) => {
                        log::error!(
                            "Failed to load {provider_name}-compatible provider API credentials: {error}"
                        );
                    }
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            api_key_editor,
            state,
            provider_name,
            load_credentials_task,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let api_key = self.api_key_editor.read(cx).text(cx).trim().to_string();
        if api_key.is_empty() {
            return;
        }

        // url changes can cause the editor to be displayed again
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(Some(api_key), cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_api_key(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.api_key_editor
            .update(cx, |input, cx| input.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn_in(window, async move |_, cx| {
            state
                .update(cx, |state, cx| state.set_api_key(None, cx))
                .await
        })
        .detach_and_log_err(cx);
    }

    fn should_render_editor(&self, cx: &Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl<S: ApiCompatibleProviderSettings> Render for ApiCompatibleProviderConfigurationView<S> {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.api_key_state.is_from_env_var();
        let env_var_name = state.api_key_state.env_var_name();
        let provider_name = self.provider_name;

        let api_key_section = if self.should_render_editor(cx) {
            v_flex()
                .on_action(cx.listener(Self::save_api_key))
                .child(Label::new(format!(
                    "To use Zed's agent with an {provider_name}-compatible provider, you need to add an API key."
                )))
                .child(
                    div()
                        .pt(DynamicSpacing::Base04.rems(cx))
                        .child(self.api_key_editor.clone()),
                )
                .child(
                    Label::new(format!(
                        "You can also set the {env_var_name} environment variable and restart Zed.",
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
                .into_any()
        } else {
            h_flex()
                .mt_1()
                .p_1()
                .justify_between()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .bg(cx.theme().colors().background)
                .child(
                    h_flex()
                        .flex_1()
                        .min_w_0()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(
                            div().w_full().overflow_x_hidden().text_ellipsis().child(Label::new(
                                if env_var_set {
                                    format!("API key set in {env_var_name} environment variable")
                                } else {
                                    format!("API key configured for {}", state.settings.api_url())
                                },
                            )),
                        ),
                )
                .child(
                    h_flex().flex_shrink_0().child(
                        Button::new("reset-api-key", "Reset API Key")
                            .label_size(LabelSize::Small)
                            .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                            .layer(ElevationIndex::ModalSurface)
                            .when(env_var_set, |this| {
                                this.tooltip(Tooltip::text(format!(
                                    "To reset your API key, unset the {env_var_name} environment variable.",
                                )))
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.reset_api_key(window, cx)
                            })),
                    ),
                )
                .into_any()
        };

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials…")).into_any()
        } else {
            v_flex().size_full().child(api_key_section).into_any()
        }
    }
}
