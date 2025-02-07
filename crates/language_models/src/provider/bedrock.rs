use std::pin::Pin;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use aws_config::Region;
use aws_credential_types::Credentials;
use aws_http_client::AwsHttpClient;
use bedrock::bedrock_client::types::{ContentBlockDelta, ContentBlockStart, ConverseStreamOutput};
use bedrock::bedrock_client::{self, Config};
use bedrock::{BedrockError, BedrockStreamingResponse, Model};
use collections::BTreeMap;
use editor::{Editor, EditorElement, EditorStyle};
use futures::{
    future::{self, BoxFuture},
    stream::BoxStream,
    FutureExt, Stream, StreamExt,
};
use gpui::{
    AnyView, App, AsyncApp, Context, Entity, FontStyle, Subscription, Task, TextStyle, WhiteSpace,
};
use gpui_tokio::Tokio;
use http_client::HttpClient;
use language_model::{
    LanguageModel, LanguageModelCacheConfiguration, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelName, LanguageModelProvider, LanguageModelProviderId, LanguageModelProviderName,
    LanguageModelProviderState, LanguageModelRequest, RateLimiter, Role, StopReason,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{Settings, SettingsStore};
use strum::IntoEnumIterator;
use theme::ThemeSettings;
use tokio::runtime::Handle;
use ui::{prelude::*, Icon, IconName, Tooltip};
use util::ResultExt;

use crate::AllLanguageModelSettings;

const PROVIDER_ID: &str = "amazon-bedrock";
const PROVIDER_NAME: &str = "Amazon Bedrock";

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AmazonBedrockSettings {
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub available_models: Vec<AvailableModel>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AvailableModel {
    pub name: String,
    pub display_name: Option<String>,
    pub max_tokens: usize,
    pub cache_configuration: Option<LanguageModelCacheConfiguration>,
    pub max_output_tokens: Option<u32>,
    pub default_temperature: Option<f32>,
}

// Different because we don't want to overwrite their AWS credentials
const ZED_BEDROCK_AAID: &str = "ZED_ACCESS_KEY_ID";
const ZED_BEDROCK_SK: &str = "ZED_SECRET_ACCESS_KEY";
const ZED_BEDROCK_REGION: &str = "ZED_AWS_REGION";

pub struct State {
    aa_id: Option<String>,
    sk: Option<String>,
    credentials_from_env: bool,
    region: Option<String>,
    _subscription: Subscription,
}

impl State {
    fn reset_credentials(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        let delete_aa_id = cx.delete_credentials(
            &AllLanguageModelSettings::get_global(cx)
                .bedrock
                .access_key_id,
        );
        let delete_sk: Task<Result<()>> = cx.delete_credentials(
            &AllLanguageModelSettings::get_global(cx)
                .bedrock
                .secret_access_key,
        );
        let delete_region =
            cx.delete_credentials(&AllLanguageModelSettings::get_global(cx).bedrock.region);
        cx.spawn(|this, mut cx| async move {
            delete_aa_id.await.ok();
            delete_sk.await.ok();
            delete_region.await.ok();
            this.update(&mut cx, |this, cx| {
                this.aa_id = None;
                this.sk = None;
                this.region = None;
                this.credentials_from_env = false;
                cx.notify();
            })
        })
    }

    fn set_credentials(
        &mut self,
        access_key_id: String,
        secret_key: String,
        region: String,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let write_aa_id = cx.write_credentials(
            ZED_BEDROCK_AAID, // TODO: GET THIS REVIEWED, MAKE SURE IT DOESN'T BREAK STUFF LONG TERM
            "Bearer",
            access_key_id.as_bytes(),
        );
        let write_sk = cx.write_credentials(
            ZED_BEDROCK_SK, // TODO: GET THIS REVIEWED, MAKE SURE IT DOESN'T BREAK STUFF LONG TERM
            "Bearer",
            secret_key.as_bytes(),
        );
        let write_region = cx.write_credentials(ZED_BEDROCK_REGION, "Bearer", region.as_bytes());
        cx.spawn(|this, mut cx| async move {
            write_aa_id.await?;
            write_sk.await?;
            write_region.await?;

            this.update(&mut cx, |this, cx| {
                this.aa_id = Some(access_key_id);
                this.sk = Some(secret_key);
                this.region = Some(region);
                cx.notify();
            })
        })
    }

    fn is_authenticated(&self) -> bool {
        self.aa_id.is_some() && self.sk.is_some()
    }

    fn authenticate(&self, cx: &mut Context<Self>) -> Task<Result<()>> {
        // just hit the sdk-bedrock list models to check if the credentials are valid
        if self.is_authenticated() {
            Task::ready(Ok(()))
        } else {
            // TODO: Figure out how to only enter the pw once,
            // perhaps serde it
            cx.spawn(|this, mut cx| async move {
                let (aa_id, sk, region, from_env) = if let (Ok(aa_id), Ok(sk), Ok(region)) = (
                    std::env::var(ZED_BEDROCK_AAID),
                    std::env::var(ZED_BEDROCK_SK),
                    std::env::var(ZED_BEDROCK_REGION),
                ) {
                    (aa_id, sk, region, true)
                } else {
                    let (_, aa_id) = cx
                        .update(|cx| cx.read_credentials(ZED_BEDROCK_AAID))?
                        .await?
                        .ok_or_else(|| anyhow!("Access key ID not found"))?;
                    let (_, sk) = cx
                        .update(|cx| cx.read_credentials(ZED_BEDROCK_SK))?
                        .await?
                        .ok_or_else(|| anyhow!("Secret access key not found"))?;
                    let (_, region) = cx
                        .update(|cx| cx.read_credentials(ZED_BEDROCK_REGION))?
                        .await?
                        .ok_or_else(|| anyhow!("Region not found"))?;

                    (
                        String::from_utf8(aa_id)?,
                        String::from_utf8(sk)?,
                        String::from_utf8(region)?,
                        false,
                    )
                };

                this.update(&mut cx, |this, cx| {
                    this.credentials_from_env = from_env;
                    this.aa_id = Some(aa_id);
                    this.sk = Some(sk);
                    this.region = Some(region);
                    cx.notify();
                })
            })
        }
    }
}

pub struct BedrockLanguageModelProvider {
    http_client: AwsHttpClient,
    handler: tokio::runtime::Handle,
    state: gpui::Entity<State>,
}

impl BedrockLanguageModelProvider {
    // This has to succeed
    pub fn new(http_client: Arc<dyn HttpClient>, cx: &mut App) -> Self {
        let state = cx.new(|cx| State {
            aa_id: None,
            sk: None,
            region: Some(String::from("us-east-1")),
            credentials_from_env: false,
            _subscription: cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            }),
        });

        let tokio_handle = Tokio::handle(cx);

        let coerced_client = AwsHttpClient::new(http_client.clone(), tokio_handle.clone());

        Self {
            http_client: coerced_client,
            handler: tokio_handle.clone(),
            state,
        }
    }
}

impl LanguageModelProvider for BedrockLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn provided_models(&self, cx: &App) -> Vec<Arc<dyn LanguageModel>> {
        let mut models = BTreeMap::default();

        for model in bedrock::Model::iter() {
            if !matches!(model, bedrock::Model::Custom { .. }) {
                models.insert(model.id().to_string(), model);
            }
        }

        // Override with available models from settings
        for model in AllLanguageModelSettings::get_global(cx)
            .bedrock
            .available_models
            .iter()
        {
            models.insert(
                model.name.clone(),
                bedrock::Model::Custom {
                    name: model.name.clone(),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    default_temperature: model.default_temperature,
                },
            );
        }

        models
            .into_values()
            .map(|model| {
                Arc::new(BedrockModel {
                    id: LanguageModelId::from(model.id().to_string()),
                    model,
                    http_client: self.http_client.clone(),
                    handler: self.handler.clone(), // internally reference counted, can be freely cloned
                    state: self.state.clone(),
                    request_limiter: RateLimiter::new(4),
                }) as Arc<dyn LanguageModel>
            })
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<()>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn configuration_view(&self, window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| ConfigurationView::new(self.state.clone(), window, cx))
            .into()
    }

    fn reset_credentials(&self, cx: &mut App) -> Task<Result<()>> {
        self.state
            .update(cx, |state, cx| state.reset_credentials(cx))
    }
}

impl LanguageModelProviderState for BedrockLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<gpui::Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

struct BedrockModel {
    id: LanguageModelId,
    model: Model,
    http_client: AwsHttpClient,
    handler: tokio::runtime::Handle,
    state: gpui::Entity<State>,
    request_limiter: RateLimiter,
}

impl BedrockModel {
    fn stream_completion(
        &self,
        request: bedrock::Request,
        cx: &AsyncApp,
    ) -> Result<
        BoxFuture<'static, BoxStream<'static, Result<BedrockStreamingResponse>>>,
    > {
        let Ok((aa_id, sk, region)) = cx.read_entity(&self.state, |state, cx| {
            let _settings = &AllLanguageModelSettings::get_global(cx).bedrock;
            (state.aa_id.clone(), state.sk.clone(), state.region.clone())
        }) else {
            return Err(anyhow!("App state dropped"));
        };

        // instantiate aws client here
        // we'll throw it in the model's struct after
        // Config::builder might be an expensive operation, figure out if it works or not
        let runtime_client = bedrock_client::Client::from_conf(
            Config::builder()
                .credentials_provider(Credentials::new(
                    aa_id.unwrap(),
                    sk.unwrap(),
                    None,
                    None,
                    "Keychain",
                ))
                .region(Region::new(region.unwrap()))
                .http_client(self.http_client.clone())
                .build(),
        );

        let owned_handle = self.handler.clone();

        Ok(async move {
            let request = bedrock::stream_completion(runtime_client, request, owned_handle);

            request.await.unwrap_or_else(|e| {
                futures::stream::once(async move { Err(e) }).boxed()
            })
        }
        .boxed())
    }
}

impl LanguageModel for BedrockModel {
    fn id(&self) -> LanguageModelId {
        self.id.clone()
    }

    fn name(&self) -> LanguageModelName {
        LanguageModelName::from(self.model.display_name().to_string())
    }

    fn provider_id(&self) -> LanguageModelProviderId {
        LanguageModelProviderId(PROVIDER_ID.into())
    }

    fn provider_name(&self) -> LanguageModelProviderName {
        LanguageModelProviderName(PROVIDER_NAME.into())
    }

    fn telemetry_id(&self) -> String {
        format!("bedrock/{}", self.model.id())
    }

    fn max_token_count(&self) -> usize {
        self.model.max_token_count()
    }

    fn max_output_tokens(&self) -> Option<u32> {
        Some(self.model.max_output_tokens())
    }

    fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &App,
    ) -> BoxFuture<'static, Result<usize>> {
        get_bedrock_tokens(request, cx)
    }

    fn stream_completion(
        &self,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<LanguageModelCompletionEvent>>>> {
        let request = request.into_bedrock(
            self.model.id().into(),
            self.model.default_temperature(),
            self.model.max_output_tokens(),
        );

        let owned_handle = self.handler.clone();

        let request = self.stream_completion(request, cx);
        let future = self.request_limiter.stream(async move {
            let response = request.map_err(|e| anyhow!(e)).unwrap().await;
            Ok(map_to_language_model_completion_events(
                response,
                owned_handle,
            ))
        });
        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn use_any_tool(
        &self,
        _request: LanguageModelRequest,
        _name: String,
        _description: String,
        _schema: Value,
        _cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        future::ready(Err(anyhow!("not implemented"))).boxed()
    }

    fn cache_configuration(&self) -> Option<LanguageModelCacheConfiguration> {
        None
    }
}

// TODO: just call the ConverseOutput.usage() method:
// https://docs.rs/aws-sdk-bedrockruntime/latest/aws_sdk_bedrockruntime/operation/converse/struct.ConverseOutput.html#method.output
pub fn get_bedrock_tokens(
    request: LanguageModelRequest,
    cx: &App,
) -> BoxFuture<'static, Result<usize>> {
    cx.background_executor()
        .spawn(async move {
            let messages = request.messages;
            let mut tokens_from_images = 0;
            let mut string_messages = Vec::with_capacity(messages.len());

            for message in messages {
                use language_model::MessageContent;

                let mut string_contents = String::new();

                for content in message.content {
                    match content {
                        MessageContent::Text(text) => {
                            string_contents.push_str(&text);
                        }
                        MessageContent::Image(image) => {
                            tokens_from_images += image.estimate_tokens();
                        }
                        MessageContent::ToolUse(_tool_use) => {
                            // TODO: Estimate token usage from tool uses.
                        }
                        MessageContent::ToolResult(tool_result) => {
                            string_contents.push_str(&tool_result.content);
                        }
                    }
                }

                if !string_contents.is_empty() {
                    string_messages.push(tiktoken_rs::ChatCompletionRequestMessage {
                        role: match message.role {
                            Role::User => "user".into(),
                            Role::Assistant => "assistant".into(),
                            Role::System => "system".into(),
                        },
                        content: Some(string_contents),
                        name: None,
                        function_call: None,
                    });
                }
            }

            // Tiktoken doesn't yet support these models, so we manually use the
            // same tokenizer as GPT-4.
            tiktoken_rs::num_tokens_from_messages("gpt-4", &string_messages)
                .map(|tokens| tokens + tokens_from_images)
        })
        .boxed()
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item = Result<BedrockStreamingResponse, BedrockError>>>>,
    handle: Handle,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent>> {
    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<BedrockStreamingResponse, BedrockError>>>>,
    }

    futures::stream::unfold(
        State {
            events
        },
        move |mut state: State| {
        let inner_handle = handle.clone();
        async move {
            inner_handle.spawn(async {
                while let Some(event) = state.events.next().await {
                    match event {
                        Ok(event) => match event {
                            ConverseStreamOutput::ContentBlockDelta(cb_delta) => {
                                if let Some(ContentBlockDelta::Text(text_out)) = cb_delta.delta {
                                    return Some((
                                        Some(Ok(LanguageModelCompletionEvent::Text(text_out))),
                                        state,
                                    ));
                                } else if let Some(ContentBlockDelta::ToolUse(_)) = cb_delta.delta {
                                    return Some((
                                        Some(Err(anyhow!("The Bedrock provider has not implemented tool use yet"))),
                                        state,
                                    ));
                                } else if cb_delta.delta.is_none() {
                                    return Some((None, state));
                                }
                            }
                            ConverseStreamOutput::ContentBlockStart(cb_start) => {
                                if let Some(start) = cb_start.start {
                                    match start {
                                        ContentBlockStart::ToolUse(_) => {
                                            return Some((
                                                Some(Err(anyhow!("The Bedrock provider has not implemented tool use yet"))),
                                                state,
                                            ))
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            ConverseStreamOutput::ContentBlockStop(_) => {
                                return Some((
                                    Some(Ok(LanguageModelCompletionEvent::Stop(StopReason::EndTurn))),
                                    state,
                                ));
                            }
                            ConverseStreamOutput::MessageStart(_) |
                            ConverseStreamOutput::MessageStop(_) |
                            ConverseStreamOutput::Metadata(_) => {}
                            _ => {}
                        },
                        Err(err) => return Some((Some(Err(anyhow!(err))), state)),
                    }
                }
                None
            }).await.unwrap()
        }
        },
    ).filter_map(|event| async move { event })
}

struct ConfigurationView {
    access_key_id_editor: Entity<Editor>,
    secret_access_key_editor: Entity<Editor>,
    region_editor: Entity<Editor>,
    state: gpui::Entity<State>,
    load_credentials_task: Option<Task<()>>,
}

impl ConfigurationView {
    const PLACEHOLDER_TEXT: &'static str = "XXXXXXXXXXXXXXXXXXX";
    const PLACEHOLDER_REGION: &'static str = "us-east-1";

    fn new(state: gpui::Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            |this, mut cx| async move {
                if let Some(task) = state
                    .update(&mut cx, |state, cx| state.authenticate(cx))
                    .log_err()
                {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(&mut cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            access_key_id_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_TEXT, cx);
                editor
            }),
            secret_access_key_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_TEXT, cx);
                editor
            }),
            region_editor: cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text(Self::PLACEHOLDER_REGION, cx);
                editor
            }),
            state,
            load_credentials_task,
        }
    }

    fn save_credentials(
        &mut self,
        _: &menu::Confirm,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let access_key_id = self.access_key_id_editor.read(cx).text(cx).to_string();
        let secret_access_key = self.secret_access_key_editor.read(cx).text(cx).to_string();
        let region = self.region_editor.read(cx).text(cx).to_string();

        let state = self.state.clone();
        cx.spawn(|_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| {
                    state.set_credentials(access_key_id, secret_access_key, region, cx)
                })?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn reset_credentials(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.access_key_id_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        self.secret_access_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        self.region_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        let state = self.state.clone();
        cx.spawn(|_, mut cx| async move {
            state
                .update(&mut cx, |state, cx| state.reset_credentials(cx))?
                .await
        })
        .detach_and_log_err(cx);
    }

    fn make_text_style(&self, cx: &Context<Self>) -> TextStyle {
        let settings = ThemeSettings::get_global(cx);
        TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features.clone(),
            font_fallbacks: settings.ui_font.fallbacks.clone(),
            font_size: rems(0.875).into(),
            font_weight: settings.ui_font.weight,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
            text_overflow: None,
            text_align: Default::default(),
            line_clamp: None,
        }
    }

    fn render_aa_id_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.access_key_id_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_sk_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.secret_access_key_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_region_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let text_style = self.make_text_style(cx);

        EditorElement::new(
            &self.region_editor,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn should_render_editor(&self, cx: &mut Context<Self>) -> bool {
        !self.state.read(cx).is_authenticated()
    }
}

impl Render for ConfigurationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        const IAM_CONSOLE_URL: &str = "https://us-east-1.console.aws.amazon.com/iam/home";
        const INSTRUCTIONS: [&str; 3] = [
            "To use Zed's assistant with Bedrock, you need to add the Access Key ID, Secret Access Key and AWS Region. Follow these steps:",
            "- Create a pair at:",
            "- Paste your Access Key ID, Secret Key, and Region below and hit enter to use the assistant:",
        ];
        let env_var_set = self.state.read(cx).credentials_from_env;

        if self.load_credentials_task.is_some() {
            div().child(Label::new("Loading credentials...")).into_any()
        } else if self.should_render_editor(cx) {
            v_flex()
                .size_full()
                .on_action(cx.listener(Self::save_credentials))
                .child(Label::new(INSTRUCTIONS[0]))
                .child(h_flex().child(Label::new(INSTRUCTIONS[1])).child(
                    Button::new("iam_console", IAM_CONSOLE_URL)
                        .style(ButtonStyle::Subtle)
                        .icon(IconName::ExternalLink)
                        .icon_size(IconSize::XSmall)
                        .icon_color(Color::Muted)
                        .on_click(move |_, _window, cx| cx.open_url(IAM_CONSOLE_URL))
                )
                )
                .child(Label::new(INSTRUCTIONS[2]))
                .child(
                    h_flex()
                        .gap_1()
                        .child(self.render_aa_id_editor(cx))
                        .child(self.render_sk_editor(cx))
                        .child(self.render_region_editor(cx))
                )
                .child(
                    Label::new(
                        format!("You can also assign the {ZED_BEDROCK_AAID}, {ZED_BEDROCK_SK} and {ZED_BEDROCK_REGION} environment variable and restart Zed."),
                    )
                        .size(LabelSize::Small),
                )
                .into_any()
        } else {
            h_flex()
                .size_full()
                .justify_between()
                .child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Check).color(Color::Success))
                        .child(Label::new(if env_var_set {
                            format!("Access Key ID is set in {ZED_BEDROCK_AAID}, Secret Key is set in {ZED_BEDROCK_SK}, Region is set in {ZED_BEDROCK_REGION} environment variables.")
                        } else {
                            "Credentials configured.".to_string()
                        })),
                )
                .child(
                    Button::new("reset-key", "Reset key")
                        .icon(Some(IconName::Trash))
                        .icon_size(IconSize::Small)
                        .icon_position(IconPosition::Start)
                        .disabled(env_var_set)
                        .when(env_var_set, |this| {
                            this.tooltip(Tooltip::text(format!("To reset your credentials, unset the {ZED_BEDROCK_AAID}, {ZED_BEDROCK_SK}, and {ZED_BEDROCK_REGION} environment variables.")))
                        })
                        .on_click(cx.listener(|this, _, window, cx| this.reset_credentials(window, cx))),
                )
                .into_any()
        }
    }
}
