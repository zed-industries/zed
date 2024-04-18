use crate::{
    assistant_settings::YandexGptModel, CompletionProvider, LanguageModel, LanguageModelRequest,
    Role,
};
use anyhow::{anyhow, Result};
use editor::{Editor, EditorElement, EditorStyle};
use futures::{future::BoxFuture, stream::BoxStream, FutureExt, StreamExt};
use gpui::{AnyView, AppContext, FontStyle, FontWeight, Task, TextStyle, View, WhiteSpace};
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{env, sync::Arc};
use theme::ThemeSettings;
use ui::prelude::*;
use util::{http::HttpClient, ResultExt};
use yandex_gpt::{stream_completion, Request, RequestMessage, Role as YandexGptRole};

pub struct YandexGptCompletionProvider {
    api_key: Option<String>,
    folder_id: Option<String>,
    api_url: String,
    default_model: YandexGptModel,
    http_client: Arc<dyn HttpClient>,
    settings_version: usize,
}

impl YandexGptCompletionProvider {
    pub fn new(
        default_model: YandexGptModel,
        api_url: String,
        http_client: Arc<dyn HttpClient>,
        settings_version: usize,
    ) -> Self {
        Self {
            api_key: None,
            folder_id: None,
            api_url,
            default_model,
            http_client,
            settings_version,
        }
    }

    pub fn update(
        &mut self,
        default_model: YandexGptModel,
        api_url: String,
        settings_version: usize,
    ) {
        self.default_model = default_model;
        self.api_url = api_url;
        self.settings_version = settings_version;
    }

    pub fn settings_version(&self) -> usize {
        self.settings_version
    }

    pub fn is_authenticated(&self) -> bool {
        self.api_key.is_some()
    }

    pub fn authenticate(&self, cx: &AppContext) -> Task<Result<()>> {
        if self.is_authenticated() {
            Task::ready(Ok(()))
        } else {
            let api_url = self.api_url.clone();
            cx.spawn(|mut cx| async move {
                let (api_key, folder_id) = if let Some((api_key, folder_id)) =
                    env::var("YANDEX_GPT_API_KEY")
                        .into_iter()
                        .zip(env::var("YANDEX_GPT_FOLDER_ID"))
                        .next()
                {
                    (api_key, folder_id)
                } else {
                    let (_, creds) = cx
                        .update(|cx| cx.read_credentials(&api_url))?
                        .await?
                        .ok_or_else(|| anyhow!("credentials not found"))?;
                    let YdxCredential(api_key, folder_id) =
                        YdxCredential::from_slice(creds.as_slice())?;
                    (api_key, folder_id)
                };

                cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                    if let CompletionProvider::YandexGpt(provider) = provider {
                        provider.api_key = Some(api_key);
                        provider.folder_id = Some(folder_id);
                    }
                })
            })
        }
    }

    pub fn reset_credentials(&self, cx: &AppContext) -> Task<Result<()>> {
        let delete_credentials = cx.delete_credentials(&self.api_url);
        cx.spawn(|mut cx| async move {
            delete_credentials.await.log_err();
            cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                if let CompletionProvider::YandexGpt(provider) = provider {
                    provider.api_key = None;
                }
            })
        })
    }

    pub fn authentication_prompt(&self, cx: &mut WindowContext) -> AnyView {
        cx.new_view(|cx| AuthenticationPrompt::new(self.api_url.clone(), cx))
            .into()
    }

    pub fn default_model(&self) -> YandexGptModel {
        self.default_model.clone()
    }

    pub fn count_tokens(
        &self,
        request: LanguageModelRequest,
        cx: &AppContext,
    ) -> BoxFuture<'static, Result<usize>> {
        let request = self.to_yandex_gpt_request(request);
        let http_client = self.http_client.clone();
        let api_key = self.api_key.clone().unwrap_or_default();
        let api_url = self.api_url.clone();

        count_yandex_gpt_tokens(
            http_client,
            api_key,
            api_url,
            request,
            cx.background_executor(),
        )
    }

    pub fn complete(
        &self,
        request: LanguageModelRequest,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<String>>>> {
        let request = self.to_yandex_gpt_request(request);

        let http_client = self.http_client.clone();
        let api_key = self.api_key.clone();
        let api_url = self.api_url.clone();
        async move {
            let api_key = api_key.ok_or_else(|| anyhow!("missing api key"))?;
            let request = stream_completion(http_client.as_ref(), &api_url, &api_key, request);
            let response = request.await?;
            let stream = response
                .filter_map(|response| async move {
                    match response {
                        Ok(mut response) => Some(Ok(response.alternatives.pop()?.message.text?)),
                        Err(error) => Some(Err(error)),
                    }
                })
                .boxed();
            Ok(stream)
        }
        .boxed()
    }

    fn to_yandex_gpt_request(&self, request: LanguageModelRequest) -> Request {
        let model = match request.model {
            LanguageModel::ZedDotDev(_) => self.default_model(),
            LanguageModel::OpenAi(_) => self.default_model(),
            LanguageModel::YandexGpt(model) => model,
        };

        Request {
            model_uri: model.get_uri(self.folder_id.clone().unwrap_or_default()),
            messages: request
                .messages
                .into_iter()
                .filter_map(|msg| {
                    if msg.content.is_empty() {
                        None
                    } else {
                        Some(RequestMessage {
                            role: msg.role.into(),
                            text: msg.content,
                        })
                    }
                })
                .collect(),
            completion_options: yandex_gpt::CompletionOptions {
                stream: true,
                temperature: request.temperature,
                max_tokens: model.max_token_count(),
            },
        }
    }
}

pub fn count_yandex_gpt_tokens(
    client: Arc<dyn HttpClient>,
    api_key: String,
    api_url: String,
    request: Request,
    background_executor: &gpui::BackgroundExecutor,
) -> BoxFuture<'static, Result<usize>> {
    background_executor
        .spawn(async move {
            yandex_gpt::tokenize_completion(
                client.as_ref(),
                api_url.as_ref(),
                api_key.as_ref(),
                request,
            )
            .await
            .map(|r| r.tokens.len())
        })
        .boxed()
}

impl From<Role> for yandex_gpt::Role {
    fn from(val: Role) -> Self {
        match val {
            Role::User => YandexGptRole::User,
            Role::Assistant => YandexGptRole::Assistant,
            Role::System => YandexGptRole::System,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct YdxCredential(String, String);

impl YdxCredential {
    pub fn as_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serialize credentials")
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let creds = serde_json::from_slice(bytes)?;
        Ok(creds)
    }
}

struct AuthenticationPrompt {
    api_key: View<Editor>,
    folder_id: View<Editor>,
    api_url: String,
}

impl AuthenticationPrompt {
    fn new(api_url: String, cx: &mut WindowContext) -> Self {
        Self {
            api_key: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("XXXX0_00000000000000000000000000000000000", cx);
                editor
            }),
            folder_id: cx.new_view(|cx| {
                let mut editor = Editor::single_line(cx);
                editor.set_placeholder_text("000000000000000000", cx);
                editor
            }),
            api_url,
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let api_key = self.api_key.read(cx).text(cx);
        if api_key.is_empty() {
            return;
        }

        let folder_id = self.folder_id.read(cx).text(cx);
        if folder_id.is_empty() {
            return;
        }

        let cred = YdxCredential(api_key.clone(), folder_id.clone());

        let write_credentials =
            cx.write_credentials(&self.api_url, "Api-Key", cred.as_bytes().as_slice());
        cx.spawn(|_, mut cx| async move {
            write_credentials.await?;
            cx.update_global::<CompletionProvider, _>(|provider, _cx| {
                if let CompletionProvider::YandexGpt(provider) = provider {
                    provider.api_key = Some(api_key);
                    provider.folder_id = Some(folder_id);
                }
            })
        })
        .detach_and_log_err(cx);
    }

    fn render_api_key_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features,
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };
        EditorElement::new(
            &self.api_key,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }

    fn render_folder_id_editor(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let settings = ThemeSettings::get_global(cx);
        let text_style = TextStyle {
            color: cx.theme().colors().text,
            font_family: settings.ui_font.family.clone(),
            font_features: settings.ui_font.features,
            font_size: rems(0.875).into(),
            font_weight: FontWeight::NORMAL,
            font_style: FontStyle::Normal,
            line_height: relative(1.3),
            background_color: None,
            underline: None,
            strikethrough: None,
            white_space: WhiteSpace::Normal,
        };
        EditorElement::new(
            &self.folder_id,
            EditorStyle {
                background: cx.theme().colors().editor_background,
                local_player: cx.theme().players().local(),
                text: text_style,
                ..Default::default()
            },
        )
    }
}

impl Render for AuthenticationPrompt {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        const INSTRUCTIONS: [&str; 5] = [
            "To use the assistant panel or inline assistant, you need to add your YandexGPT API key and Folder ID.",
            " - You can create an API key. See: yandex.cloud/docs/foundation-models/api-ref/authentication",
            " - You can get an existing folder ID or create a new one. See: yandex.cloud/docs/resource-manager/operations/folder/get-id",
            " - Make sure your Yandex Cloud account has a billing account",
            "",
        ];

        v_flex()
            .p_4()
            .size_full()
            .on_action(cx.listener(Self::save_api_key))
            .children(
                INSTRUCTIONS.map(|instruction| Label::new(instruction).size(LabelSize::Small)),
            )
            .child(Label::new("Paste your Yandex GPT API key:").size(LabelSize::Small))
            .child(
                h_flex()
                    .w_full()
                    .my_2()
                    .px_2()
                    .py_1()
                    .bg(cx.theme().colors().editor_background)
                    .rounded_md()
                    .child(self.render_api_key_editor(cx)),
            )
            .child(Label::new("Paste your Yandex Cloud Folder ID and hit enter:").size(LabelSize::Small))
            .child(
                h_flex()
                    .w_full()
                    .my_2()
                    .px_2()
                    .py_1()
                    .bg(cx.theme().colors().editor_background)
                    .rounded_md()
                    .child(self.render_folder_id_editor(cx)),
            )
            .child(
                Label::new(
                    "You can also assign the YANDEX_GPT_API_KEY and YANDEX_GPT_FOLDER_ID environment variables and restart Zed.",
                )
                .size(LabelSize::Small),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new("Click on").size(LabelSize::Small))
                    .child(Icon::new(IconName::Ai).size(IconSize::XSmall))
                    .child(
                        Label::new("in the status bar to close this panel.").size(LabelSize::Small),
                    ),
            )
            .into_any()
    }
}
