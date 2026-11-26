use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Context as _, Result, anyhow};
use aws_config::stalled_stream_protection::StalledStreamProtectionConfig;
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::provider::ProvideCredentials;
use aws_credential_types::{Credentials, Token};
use aws_http_client::AwsHttpClient;
use aws_sigv4::http_request::{SignableBody, SignableRequest, SigningSettings, sign};
use aws_sigv4::sign::v4;
use bedrock::BedrockSystemContentBlock;
use bedrock::bedrock_client::Client as BedrockClient;
use bedrock::bedrock_client::config::timeout::TimeoutConfig;
use bedrock::bedrock_client::types::{
    CachePointBlock, CachePointType, ContentBlockDelta, ContentBlockStart, ConverseStreamOutput,
    ReasoningContentBlockDelta, StopReason,
};
use bedrock::bedrock_control_plane_client::Client as BedrockControlPlaneClient;
use bedrock::{
    BedrockAdaptiveThinkingEffort, BedrockAnyToolChoice, BedrockAutoToolChoice, BedrockBlob,
    BedrockError, BedrockImageBlock, BedrockImageFormat, BedrockImageSource, BedrockInnerContent,
    BedrockMessage, BedrockStreamingResponse, BedrockThinkingBlock, BedrockThinkingTextBlock,
    BedrockTool, BedrockToolChoice, BedrockToolConfig, BedrockToolInputSchema,
    BedrockToolResultBlock, BedrockToolResultContentBlock, BedrockToolResultStatus,
    BedrockToolSpec, BedrockToolUseBlock, CAPABILITY_CATALOG_URL, CapabilityCatalog, ConverseModel,
    ConverseModelRoute, ConverseThinking, MantleModel, MantleProtocol, ModelCatalog,
    value_to_aws_document,
};
use collections::{BTreeMap, HashMap, IndexMap};
use credentials_provider::CredentialsProvider;
use fs::Fs;
use futures::{
    AsyncBufReadExt, AsyncReadExt, FutureExt, Stream, StreamExt, future::BoxFuture, io::BufReader,
    stream::BoxStream,
};
use gpui::{
    App, AsyncApp, BackgroundExecutor, Context, Entity, FocusHandle, Subscription, Task, TaskExt,
    Window, actions,
};
use gpui_tokio::Tokio;
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
    http::{HeaderValue, header::AUTHORIZATION},
};
use language_model::{
    AuthenticateError, EnvVar, IconOrSvg, InlineDescription, LanguageModel, LanguageModelClient,
    LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelCompletionStream,
    LanguageModelEffortLevel, LanguageModelId, LanguageModelName, LanguageModelProvider,
    LanguageModelProviderId, LanguageModelProviderName, LanguageModelProviderState,
    LanguageModelRequest, LanguageModelToolChoice, LanguageModelToolChoiceSupport,
    LanguageModelToolResultContent, LanguageModelToolUse, MessageContent, ModelRateLimiters,
    ProviderErrorCategory, ProviderSettingsView, RateLimiter, Role, SubPageProviderSettings,
    TokenUsage, env_var, unavailable_error,
};
use open_ai::responses::Request as OpenAiResponseRequest;
use open_ai::responses::{ResponseOutputItem, StreamEvent as OpenAiResponseStreamEvent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use settings::{
    BedrockAvailableModel as AvailableModel, BedrockMantleAvailableModel as MantleAvailableModel,
    Settings, SettingsStore,
};
use std::path::PathBuf;
use std::sync::LazyLock;
use std::time::{Duration, SystemTime};
use strum::{EnumIter, IntoStaticStr};
use ui::{ButtonLink, ConfiguredApiCard, Divider, List, ListBulletItem, prelude::*};
use ui_input::InputField;
use util::ResultExt;

use crate::AllLanguageModelSettings;
use crate::provider::model_catalog_cache;
use crate::provider::open_ai::{
    ChatCompletionMaxTokensParameter, OpenAiResponseEventMapper, into_open_ai,
    into_open_ai_response,
};
use language_model::chat_completion::{
    ChatCompletionEventMapper, ResponseStreamEvent, ResponseStreamResult,
};
use language_model::util::{fix_streamed_json, parse_tool_arguments};
use open_ai::{ReasoningEffort, RequestError};

actions!(bedrock, [Tab, TabPrev]);

const PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("amazon-bedrock");
const PROVIDER_NAME: LanguageModelProviderName = LanguageModelProviderName::new("Amazon Bedrock");
pub(crate) const RESERVED_HEADER_NAMES: &[&str] = &[
    "host",
    "x-amz-date",
    "x-amz-security-token",
    "x-amz-content-sha256",
    "amz-sdk-invocation-id",
    "amz-sdk-request",
];

/// Credentials stored in the keychain for static authentication.
/// Region is handled separately since it's orthogonal to auth method.
#[derive(Default, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct BedrockCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: Option<String>,
    pub bearer_token: Option<String>,
}

/// Resolved authentication configuration for Bedrock.
/// Settings take priority over UX-provided credentials.
#[derive(Clone, Debug, PartialEq)]
pub enum BedrockAuth {
    /// Use default AWS credential provider chain (IMDSv2, PodIdentity, env vars, etc.)
    Automatic,
    /// Use AWS named profile from ~/.aws/credentials or ~/.aws/config
    NamedProfile { profile_name: String },
    /// Use AWS SSO profile
    SingleSignOn { profile_name: String },
    /// Use IAM credentials (access key + secret + optional session token)
    IamCredentials {
        access_key_id: String,
        secret_access_key: String,
        session_token: Option<String>,
    },
    /// Use Bedrock API Key (bearer token authentication)
    ApiKey { api_key: String },
}

impl BedrockCredentials {
    /// Convert stored credentials to the appropriate auth variant.
    /// Prefers API key if present, otherwise uses IAM credentials.
    fn into_auth(self) -> Option<BedrockAuth> {
        if let Some(api_key) = self.bearer_token.filter(|t| !t.is_empty()) {
            Some(BedrockAuth::ApiKey { api_key })
        } else if !self.access_key_id.is_empty() && !self.secret_access_key.is_empty() {
            Some(BedrockAuth::IamCredentials {
                access_key_id: self.access_key_id,
                secret_access_key: self.secret_access_key,
                session_token: self.session_token.filter(|t| !t.is_empty()),
            })
        } else {
            None
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct AmazonBedrockSettings {
    pub available_models: Vec<AvailableModel>,
    pub mantle_available_models: Vec<MantleAvailableModel>,
    pub custom_headers: CustomHeaders,
    pub region: Option<String>,
    pub endpoint: Option<String>,
    pub profile_name: Option<String>,
    pub role_arn: Option<String>,
    pub authentication_method: Option<BedrockAuthMethod>,
    pub allow_global: Option<bool>,
    pub guardrail_identifier: Option<String>,
    pub guardrail_version: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, EnumIter, IntoStaticStr, JsonSchema)]
pub enum BedrockAuthMethod {
    #[serde(rename = "named_profile")]
    NamedProfile,
    #[serde(rename = "sso")]
    SingleSignOn,
    #[serde(rename = "api_key")]
    ApiKey,
    /// IMDSv2, PodIdentity, env vars, etc.
    #[serde(rename = "default")]
    Automatic,
}

impl From<settings::BedrockAuthMethodContent> for BedrockAuthMethod {
    fn from(value: settings::BedrockAuthMethodContent) -> Self {
        match value {
            settings::BedrockAuthMethodContent::SingleSignOn => BedrockAuthMethod::SingleSignOn,
            settings::BedrockAuthMethodContent::Automatic => BedrockAuthMethod::Automatic,
            settings::BedrockAuthMethodContent::NamedProfile => BedrockAuthMethod::NamedProfile,
            settings::BedrockAuthMethodContent::ApiKey => BedrockAuthMethod::ApiKey,
        }
    }
}

fn thinking_from_settings(
    thinking: Option<&settings::BedrockThinkingSettings>,
) -> Option<ConverseThinking> {
    match thinking {
        None | Some(settings::BedrockThinkingSettings::Enabled(false)) => None,
        Some(settings::BedrockThinkingSettings::Enabled(true)) => {
            Some(ConverseThinking::budget(None))
        }
        Some(settings::BedrockThinkingSettings::Config(thinking)) => {
            Some(if thinking.adaptive.unwrap_or(false) {
                ConverseThinking::adaptive(thinking.has_xhigh.unwrap_or(false), false)
            } else {
                ConverseThinking::budget(thinking.budget_tokens)
            })
        }
    }
}

fn mantle_protocol_from_settings(value: settings::BedrockMantleProtocolContent) -> MantleProtocol {
    match value {
        settings::BedrockMantleProtocolContent::ChatCompletions => MantleProtocol::ChatCompletions,
        settings::BedrockMantleProtocolContent::Responses => MantleProtocol::Responses,
    }
}

/// The URL of the base AWS service.
///
/// Right now we're just using this as the key to store the AWS credentials
/// under in the keychain.
const AMAZON_AWS_URL: &str = "https://amazonaws.com";

// These environment variables all use a `ZED_` prefix because we don't want to overwrite the user's AWS credentials.
static ZED_BEDROCK_ACCESS_KEY_ID_VAR: LazyLock<EnvVar> = env_var!("ZED_ACCESS_KEY_ID");
static ZED_BEDROCK_SECRET_ACCESS_KEY_VAR: LazyLock<EnvVar> = env_var!("ZED_SECRET_ACCESS_KEY");
static ZED_BEDROCK_SESSION_TOKEN_VAR: LazyLock<EnvVar> = env_var!("ZED_SESSION_TOKEN");
static ZED_BEDROCK_REGION_VAR: LazyLock<EnvVar> = env_var!("ZED_AWS_REGION");
static ZED_BEDROCK_BEARER_TOKEN_VAR: LazyLock<EnvVar> = env_var!("ZED_BEDROCK_BEARER_TOKEN");

/// AWS Regions where the `bedrock-mantle` endpoint is available.
/// See <https://docs.aws.amazon.com/bedrock/latest/userguide/bedrock-mantle.html#regions>.
const MANTLE_SUPPORTED_REGIONS: &[&str] = &[
    "us-east-2",
    "us-east-1",
    "us-west-2",
    "ap-southeast-3",
    "ap-south-1",
    "ap-southeast-2",
    "ap-northeast-1",
    "eu-central-1",
    "eu-west-1",
    "eu-west-2",
    "eu-south-1",
    "eu-north-1",
    "sa-east-1",
    "us-gov-west-1",
];

fn mantle_endpoint_url(region: &str) -> String {
    format!("https://bedrock-mantle.{region}.api.aws/openai/v1")
}

/// Auth resolved for one request, shared by the Converse and Mantle APIs.
#[derive(Clone)]
enum BedrockRequestAuth {
    ApiKey { api_key: String },
    SigV4 { credentials: Credentials },
}

impl BedrockRequestAuth {
    fn apply(&self, request: &mut HttpRequest<AsyncBody>, body: &[u8], region: &str) -> Result<()> {
        match self {
            BedrockRequestAuth::ApiKey { api_key } => {
                let value = HeaderValue::from_str(&format!("Bearer {}", api_key.trim()))
                    .context("building Mantle bearer token authorization header")?;
                request.headers_mut().insert(AUTHORIZATION, value);
            }
            BedrockRequestAuth::SigV4 { credentials } => {
                sign_mantle_request_sigv4(request, body, credentials, region)?;
            }
        }

        Ok(())
    }
}

fn sign_mantle_request_sigv4(
    request: &mut HttpRequest<AsyncBody>,
    body: &[u8],
    credentials: &Credentials,
    region: &str,
) -> Result<()> {
    sign_mantle_request_sigv4_at(request, body, credentials, region, SystemTime::now())
}

fn sign_mantle_request_sigv4_at(
    request: &mut HttpRequest<AsyncBody>,
    body: &[u8],
    credentials: &Credentials,
    region: &str,
    time: SystemTime,
) -> Result<()> {
    if !request
        .headers()
        .contains_key(http_client::http::header::HOST)
        && let Some(authority) = request.uri().authority()
    {
        let host = HeaderValue::from_str(authority.as_str())
            .context("invalid host header derived from Mantle request URI")?;
        request
            .headers_mut()
            .insert(http_client::http::header::HOST, host);
    }

    let identity = credentials.clone().into();
    let signing_params: aws_sigv4::http_request::SigningParams = v4::SigningParams::builder()
        .identity(&identity)
        .region(region)
        .name("bedrock-mantle")
        .time(time)
        .settings(SigningSettings::default())
        .build()
        .context("building Mantle SigV4 signing params")?
        .into();

    let method = request.method().as_str();
    let uri = request.uri().to_string();
    let headers = request
        .headers()
        .iter()
        .map(|(name, value)| {
            value
                .to_str()
                .map(|value| (name.as_str(), value))
                .with_context(|| format!("header {name} is not valid UTF-8 and cannot be signed"))
        })
        .collect::<Result<Vec<_>>>()?;

    let signable_request =
        SignableRequest::new(method, uri, headers.into_iter(), SignableBody::Bytes(body))
            .context("constructing Mantle SigV4 request")?;

    let (instructions, _signature) = sign(signable_request, &signing_params)
        .context("signing Mantle request with SigV4")?
        .into_parts();
    instructions.apply_to_request_http1x(request);

    Ok(())
}

pub struct State {
    /// The resolved authentication method. Settings take priority over UX credentials.
    auth: Option<BedrockAuth>,
    /// A copy of the provider settings, replaced by the settings observer
    /// together with the auth and listing that depend on them.
    settings: AmazonBedrockSettings,
    /// Whether credentials came from environment variables (only relevant for static credentials)
    credentials_from_env: bool,
    credentials_provider: Arc<dyn CredentialsProvider>,
    credential_store_task: Option<Task<()>>,
    credential_delete_error: Option<String>,
    /// AWS credentials that expire, such as those from SSO, STS or a credential
    /// process, reused until shortly before they expire. Credentials without an
    /// expiry are resolved for every request instead, because the AWS SDK never
    /// rereads credential files and external tools rewrite them.
    expiring_credentials: Option<ExpiringCredentials>,
    /// Incremented on every auth change, so credentials resolved for earlier
    /// auth are not cached once they arrive.
    auth_generation: u64,
    http_client: AwsHttpClient,
    plain_http_client: Arc<dyn HttpClient>,
    fs: Arc<dyn Fs>,
    /// The last successful listing for the current auth, Region and endpoint.
    model_catalog: Option<ModelCatalog>,
    /// Why the latest listing attempt failed, if it did.
    list_models_error: Option<String>,
    fetch_models_task: Option<Task<()>>,
    /// What the public capability catalog says about Bedrock's models. It is
    /// empty until the catalog has loaded.
    capability_catalog: CapabilityCatalog,
    capability_catalog_cache_path: PathBuf,
    refresh_capability_catalog_task: Option<Task<()>>,
    last_capability_catalog_error: Option<String>,
    _subscription: Subscription,
}

struct ExpiringCredentials {
    auth: Option<BedrockAuth>,
    region: String,
    credentials: Credentials,
}

/// How long before expiry cached AWS credentials are replaced, so a request
/// that streams for a while does not outlive them.
const CREDENTIALS_EXPIRY_MARGIN: Duration = Duration::from_secs(5 * 60);

/// How long resolving AWS credentials may take before the request fails,
/// matching the AWS SDK's default identity cache load timeout. Without it a
/// hung credential process or metadata endpoint would stall the request.
const CREDENTIALS_LOAD_TIMEOUT: Duration = Duration::from_secs(5);

/// The Bedrock bearer token the AWS SDK would find in the environment, looked
/// up with `environment_variable`.
///
/// Unless a token is configured explicitly, the SDK prefers this token over any
/// AWS credentials when it builds a client, so it is honored before resolving
/// credentials. Like the SDK, this checks the Bedrock-specific variable, then
/// the generic one, and uses a set value even when it is empty.
fn environment_bearer_token(
    environment_variable: impl Fn(&str) -> Option<String>,
) -> Option<String> {
    environment_variable("AWS_BEARER_TOKEN_BEDROCK")
        .or_else(|| environment_variable("AWS_BEARER_TOKEN"))
}

impl State {
    fn sync_settings(&mut self, cx: &mut Context<Self>) {
        let settings = AllLanguageModelSettings::get_global(cx).bedrock.clone();
        if self.settings == settings {
            return;
        }
        let auth_changed = self.settings.authentication_method != settings.authentication_method
            || (matches!(
                settings.authentication_method,
                Some(BedrockAuthMethod::NamedProfile | BedrockAuthMethod::SingleSignOn)
            ) && self.settings.profile_name != settings.profile_name);
        let region_changed = self.get_region() != bedrock_region(&settings);
        let endpoint_changed = self.settings.endpoint != settings.endpoint;
        self.settings = settings;
        if auth_changed {
            let auth = self.settings_auth().or_else(|| match &self.auth {
                Some(BedrockAuth::IamCredentials { .. } | BedrockAuth::ApiKey { .. }) => {
                    self.auth.clone()
                }
                _ => None,
            });
            let from_env = auth == self.auth && self.credentials_from_env;
            let was_authenticated = self.is_authenticated();
            self.change_auth(auth, from_env, cx);
            if was_authenticated && !self.is_authenticated() {
                self.authenticate(cx).detach_and_log_err(cx);
            }
        } else if region_changed || endpoint_changed {
            self.expiring_credentials = None;
            self.model_catalog = None;
            self.list_models_error = None;
            self.restart_fetch_models_task(cx);
        }
        cx.notify();
    }

    fn settings_auth(&self) -> Option<BedrockAuth> {
        let profile_name = self
            .settings
            .profile_name
            .clone()
            .unwrap_or_else(|| "default".into());
        match self.settings.authentication_method.as_ref()? {
            BedrockAuthMethod::Automatic => Some(BedrockAuth::Automatic),
            BedrockAuthMethod::NamedProfile => Some(BedrockAuth::NamedProfile { profile_name }),
            BedrockAuthMethod::SingleSignOn => Some(BedrockAuth::SingleSignOn { profile_name }),
            BedrockAuthMethod::ApiKey => None,
        }
    }

    /// Sets the auth method and drops credentials resolved for the old one.
    fn set_auth(&mut self, auth: Option<BedrockAuth>) {
        self.auth = auth;
        self.expiring_credentials = None;
        self.auth_generation += 1;
    }

    /// Switches to `auth` and lists the models it can reach.
    fn change_auth(
        &mut self,
        auth: Option<BedrockAuth>,
        credentials_from_env: bool,
        cx: &mut Context<Self>,
    ) {
        self.set_auth(auth);
        self.credentials_from_env = credentials_from_env;
        self.model_catalog = None;
        self.list_models_error = None;
        self.restart_fetch_models_task(cx);
        cx.notify();
    }

    /// Whether requests go to a custom endpoint, which stands in for
    /// `bedrock-runtime` only. It does not serve the listing APIs, and
    /// credentials for it need not be valid for AWS.
    fn uses_custom_endpoint(&self) -> bool {
        self.settings
            .endpoint
            .as_deref()
            .is_some_and(|endpoint| !endpoint.is_empty())
    }

    /// Whether the models that the capability catalog names for the Region
    /// stand in for Bedrock's own listing: a custom endpoint does not serve
    /// one, and the credentials in use may not be allowed to ask for it.
    fn uses_public_catalog(&self) -> bool {
        self.is_authenticated()
            && self.model_catalog.is_none()
            && (self.uses_custom_endpoint() || self.list_models_error.is_some())
    }

    fn restart_fetch_models_task(&mut self, cx: &mut Context<Self>) {
        // Dropping the previous task cancels its requests, so a listing for
        // earlier auth or settings is never applied.
        self.fetch_models_task = None;
        if !self.is_authenticated() {
            // Dropping the catalog task as well cancels its fetch in flight, so
            // nothing fetched for the old sign-in is applied afterwards.
            self.refresh_capability_catalog_task = None;
            self.last_capability_catalog_error = None;
            return;
        }
        if self.refresh_capability_catalog_task.is_none() {
            self.refresh_capability_catalog_task = Some(self.refresh_capability_catalog(false, cx));
        }
        if self.uses_custom_endpoint() {
            return;
        }

        let http_client = self.http_client.clone();
        let profile_name = aws_profile_name(&self.auth);
        let region = self.get_region();
        // One deadline covers every listing call and page, so a stalled call
        // cannot leave the model list loading forever.
        let deadline = cx.background_executor().timer(LIST_MODELS_TIMEOUT);
        self.fetch_models_task = Some(cx.spawn(async move |this, cx| {
            let result = async {
                let request_auth = {
                    let state = this
                        .upgrade()
                        .context("Bedrock provider state was dropped")?;
                    let http_client = http_client.clone();
                    resolve_cached_request_auth(
                        &state,
                        cx,
                        |name| std::env::var(name).ok(),
                        move |auth, region| resolve_request_auth(http_client, auth, region),
                    )
                }
                .await?;
                Tokio::spawn_result(cx, async move {
                    let client =
                        build_control_plane_client(http_client, request_auth, profile_name, region)
                            .await;
                    bedrock::list_models(&client, deadline).await
                })
                .await
            }
            .await;
            this.update(cx, |this, cx| {
                match result {
                    Ok(model_catalog) => {
                        this.model_catalog = Some(model_catalog);
                        this.list_models_error = None;
                    }
                    Err(error) => {
                        log::warn!("Failed to list Bedrock models: {error:#}");
                        this.list_models_error = Some(format!("{error:#}"));
                    }
                }
                cx.notify();
            })
            .log_err();
        }));
    }

    /// Loads the capability catalog from disk, fetches it when that copy is
    /// missing or old, and then fetches it again every hour. The catalog is
    /// public, so the task does not depend on which credentials are in use.
    fn refresh_capability_catalog(&self, fetch_now: bool, cx: &mut Context<Self>) -> Task<()> {
        let http_client = self.plain_http_client.clone();
        let fs = self.fs.clone();
        let cache_path = self.capability_catalog_cache_path.clone();
        let executor = cx.background_executor().clone();
        let load_cache = self.capability_catalog.is_empty();
        cx.spawn(async move |this, cx| {
            let cached_catalog = if load_cache {
                model_catalog_cache::load_cached_with(fs.clone(), &cache_path, &executor, |text| {
                    CapabilityCatalog::parse(&text)
                })
                .await
                .log_err()
                .flatten()
            } else {
                None
            };
            let mut is_fresh = false;
            if let Some((capability_catalog, cached_is_fresh)) = cached_catalog {
                is_fresh = cached_is_fresh && !fetch_now;
                let applied = this.update(cx, |this, cx| {
                    this.capability_catalog = capability_catalog;
                    cx.notify();
                });
                if applied.is_err() {
                    return;
                }
            }

            loop {
                if !is_fresh {
                    let capability_catalog = executor
                        .spawn({
                            let http_client = http_client.clone();
                            let executor = executor.clone();
                            let fs = fs.clone();
                            let cache_path = cache_path.clone();
                            async move {
                                fetch_capability_catalog(
                                    http_client.as_ref(),
                                    &executor,
                                    fs.as_ref(),
                                    cache_path,
                                )
                                .await
                            }
                        })
                        .await;
                    let applied = this.update(cx, |this, cx| {
                        match capability_catalog {
                            Ok(capability_catalog) => {
                                this.capability_catalog = capability_catalog;
                                this.last_capability_catalog_error = None;
                            }
                            Err(error) => {
                                log::warn!("Failed to fetch Bedrock model capabilities: {error:#}");
                                this.last_capability_catalog_error = Some(format!("{error:#}"));
                            }
                        }
                        cx.notify();
                    });
                    if applied.is_err() {
                        return;
                    }
                }
                is_fresh = false;
                executor.timer(model_catalog_cache::REFRESH_INTERVAL).await;
            }
        })
    }

    // Reserve the position synchronously, even if the caller drops its result
    // or the executor polls a later save first.
    fn queue_credential_operation<E>(
        &mut self,
        cx: &mut Context<Self>,
        operation: impl AsyncFnOnce(gpui::WeakEntity<Self>, &mut AsyncApp) -> Result<(), E> + 'static,
    ) -> Task<Result<(), E>>
    where
        E: From<anyhow::Error> + 'static,
    {
        let previous = self.credential_store_task.take();
        let (sender, receiver) = futures::channel::oneshot::channel();
        self.credential_store_task = Some(cx.spawn(async move |this, cx| {
            if let Some(previous) = previous {
                previous.await;
            }
            drop(sender.send(operation(this, cx).await));
        }));
        cx.spawn(async move |_, _| {
            receiver
                .await
                .context("Bedrock credential operation cancelled")?
        })
    }

    fn reset_auth(&mut self, cx: &mut Context<Self>) -> Task<Result<()>> {
        self.change_auth(None, false, cx);
        let credentials_provider = self.credentials_provider.clone();
        self.credential_delete_error = None;
        self.queue_credential_operation(cx, async move |this, cx| {
            let result = credentials_provider
                .delete_credentials(AMAZON_AWS_URL, cx)
                .await;
            if let Err(error) = &result {
                this.update(cx, |state, cx| {
                    state.credential_delete_error = Some(format!(
                        "Could not delete stored Bedrock credentials: {error:#}"
                    ));
                    cx.notify();
                })?;
            }
            result
        })
    }

    fn set_static_credentials(
        &mut self,
        credentials: BedrockCredentials,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let auth = credentials.clone().into_auth();
        let credentials_provider = self.credentials_provider.clone();
        self.auth_generation += 1;
        let generation = self.auth_generation;
        self.queue_credential_operation(cx, async move |this, cx| {
            credentials_provider
                .write_credentials(
                    AMAZON_AWS_URL,
                    "Bearer",
                    &serde_json::to_vec(&credentials)?,
                    cx,
                )
                .await?;
            this.update(cx, |this, cx| {
                this.credential_delete_error = None;
                if this.auth_generation == generation {
                    this.change_auth(auth, false, cx);
                }
            })
        })
    }

    fn is_authenticated(&self) -> bool {
        self.auth.is_some()
    }

    /// Resolve authentication. Settings take priority over UX-provided credentials.
    fn authenticate(&mut self, cx: &mut Context<Self>) -> Task<Result<(), AuthenticateError>> {
        if self.is_authenticated() {
            return Task::ready(Ok(()));
        }

        self.auth_generation += 1;
        if let Some(auth) = self.settings_auth() {
            self.change_auth(Some(auth), false, cx);
            return Task::ready(Ok(()));
        }

        // Step 2: No settings auth method - try to load static credentials
        self.load_static_credentials(cx)
    }

    /// Load static credentials from environment variables or keychain.
    fn load_static_credentials(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<Result<(), AuthenticateError>> {
        let credentials_provider = self.credentials_provider.clone();
        let generation = self.auth_generation;
        self.queue_credential_operation(cx, async move |this, cx| {
            // Try environment variables first
            let (auth, from_env) = if let Some(bearer_token) = &ZED_BEDROCK_BEARER_TOKEN_VAR.value {
                if !bearer_token.is_empty() {
                    (
                        Some(BedrockAuth::ApiKey {
                            api_key: bearer_token.to_string(),
                        }),
                        true,
                    )
                } else {
                    (None, false)
                }
            } else if let Some(access_key_id) = &ZED_BEDROCK_ACCESS_KEY_ID_VAR.value {
                if let Some(secret_access_key) = &ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.value {
                    if !access_key_id.is_empty() && !secret_access_key.is_empty() {
                        let session_token = ZED_BEDROCK_SESSION_TOKEN_VAR
                            .value
                            .as_deref()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string());
                        (
                            Some(BedrockAuth::IamCredentials {
                                access_key_id: access_key_id.to_string(),
                                secret_access_key: secret_access_key.to_string(),
                                session_token,
                            }),
                            true,
                        )
                    } else {
                        (None, false)
                    }
                } else {
                    (None, false)
                }
            } else {
                (None, false)
            };

            // If we got auth from env vars, use it
            if let Some(auth) = auth {
                this.update(cx, |this, cx| {
                    if this.auth_generation == generation {
                        this.change_auth(Some(auth), from_env, cx);
                    }
                })?;
                return Ok(());
            }

            // Try keychain
            let credentials =
                language_model::read_credentials(credentials_provider.as_ref(), AMAZON_AWS_URL, cx)
                    .await;
            // A sign-out or newer sign-in may have happened while the keychain
            // was read.
            let is_current = this.read_with(cx, |this, _| this.auth_generation == generation)?;
            if !is_current {
                return Ok(());
            }
            let (_, credentials_bytes) =
                credentials?.ok_or(AuthenticateError::CredentialsNotFound)?;

            let credentials_str = String::from_utf8(credentials_bytes)
                .with_context(|| format!("invalid {PROVIDER_NAME} credentials"))?;

            let credentials: BedrockCredentials =
                serde_json::from_str(&credentials_str).context("failed to parse credentials")?;

            let auth = credentials
                .into_auth()
                .ok_or(AuthenticateError::CredentialsNotFound)?;

            this.update(cx, |this, cx| {
                if this.auth_generation == generation {
                    this.change_auth(Some(auth), false, cx);
                }
            })?;

            Ok(())
        })
    }

    /// Get the resolved region. Checks env var, then settings, then defaults to us-east-1.
    fn get_region(&self) -> String {
        bedrock_region(&self.settings)
    }

    fn get_allow_global(&self) -> bool {
        self.settings.allow_global.unwrap_or(false)
    }

    fn get_guardrail_config(&self) -> (Option<String>, Option<String>) {
        (
            self.settings.guardrail_identifier.clone(),
            self.settings.guardrail_version.clone(),
        )
    }

    fn listing_message(&self) -> Option<(String, bool)> {
        let error = self.list_models_error.as_deref();
        if self.model_catalog.is_some() {
            let error = error?;
            Some((
                format!("Zed keeps the earlier account list, which may be out of date. {error}"),
                true,
            ))
        } else if self.uses_public_catalog() {
            let reason = error.unwrap_or("A custom endpoint is configured.");
            Some((
                format!("Zed shows the models that models.dev lists for your Region. {reason}"),
                error.is_some(),
            ))
        } else {
            None
        }
    }
}

fn bedrock_region(settings: &AmazonBedrockSettings) -> String {
    ZED_BEDROCK_REGION_VAR
        .value
        .as_deref()
        .filter(|region| !region.is_empty())
        .map(str::to_string)
        .or_else(|| settings.region.clone())
        .unwrap_or_else(|| "us-east-1".to_string())
}

pub struct BedrockLanguageModelProvider {
    http_client: AwsHttpClient,
    plain_http_client: Arc<dyn HttpClient>,
    state: Entity<State>,
    request_limiters: ModelRateLimiters,
}

/// A model this provider offers, served by either the Converse API or Mantle.
enum BedrockModelConfig {
    Converse(ConverseModel),
    Mantle(MantleModel),
}

impl BedrockLanguageModelProvider {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        cx: &mut App,
    ) -> Self {
        Self::new_with_cache(
            http_client,
            credentials_provider,
            <dyn Fs>::global(cx),
            paths::temp_dir().join("bedrock").join("models.json"),
            cx,
        )
    }

    fn new_with_cache(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        fs: Arc<dyn Fs>,
        capability_catalog_cache_path: PathBuf,
        cx: &mut App,
    ) -> Self {
        let plain_http_client = http_client;
        let http_client = AwsHttpClient::new(plain_http_client.clone());
        let state = cx.new(|cx| State {
            auth: None,
            settings: AllLanguageModelSettings::get_global(cx).bedrock.clone(),
            credentials_from_env: false,
            credentials_provider,
            credential_store_task: None,
            credential_delete_error: None,
            expiring_credentials: None,
            auth_generation: 0,
            http_client: http_client.clone(),
            plain_http_client: plain_http_client.clone(),
            fs,
            model_catalog: None,
            list_models_error: None,
            fetch_models_task: None,
            capability_catalog: CapabilityCatalog::default(),
            capability_catalog_cache_path,
            refresh_capability_catalog_task: None,
            last_capability_catalog_error: None,
            _subscription: cx.observe_global::<SettingsStore>(|state: &mut State, cx| {
                state.sync_settings(cx);
            }),
        });

        Self {
            http_client,
            plain_http_client,
            state,
            request_limiters: ModelRateLimiters::default(),
        }
    }

    /// Every model this provider offers, keyed by id: the Converse models
    /// Bedrock listed with settings entries added or overriding them, then
    /// the Mantle models from settings. A Mantle model replaces a Converse
    /// model with the same id, so each id is offered and served exactly once.
    fn bedrock_models(&self, cx: &App) -> IndexMap<String, BedrockModelConfig> {
        let state = self.state.read(cx);
        let bedrock_settings = &state.settings;
        let model_catalog = state.model_catalog.as_ref();
        let capability_catalog = &state.capability_catalog;
        let allow_global = state.get_allow_global();
        let mut models = BTreeMap::default();

        match model_catalog {
            Some(model_catalog) => {
                for model in model_catalog.models(allow_global, capability_catalog) {
                    models.insert(model.id.clone(), model);
                }
            }
            None if state.uses_public_catalog() => {
                for model in capability_catalog.models_for_region(&state.get_region(), allow_global)
                {
                    models.insert(model.id.clone(), model);
                }
            }
            None => {}
        }

        for model in bedrock_settings.available_models.iter() {
            models.insert(
                model.name.clone(),
                converse_model_from_settings(model, model_catalog, capability_catalog),
            );
        }

        let mut mantle_models = BTreeMap::default();

        for model in bedrock_settings.mantle_available_models.iter() {
            mantle_models.insert(
                model.name.clone(),
                MantleModel {
                    name: model.name.clone(),
                    known_foundation_model_id: model_catalog
                        .and_then(|catalog| {
                            catalog.model_for_request_id(&model.name, capability_catalog)
                        })
                        .or_else(|| capability_catalog.model_for_request_id(&model.name))
                        .map(|model| model.base_model_id),
                    display_name: model.display_name.clone(),
                    max_tokens: model.max_tokens,
                    max_output_tokens: model.max_output_tokens,
                    protocol: mantle_protocol_from_settings(model.protocol),
                    supports_tools: model.supports_tools.unwrap_or(false),
                    supports_images: model.supports_images.unwrap_or(false),
                    supports_thinking: model.supports_thinking.unwrap_or(false),
                },
            );
        }

        let mut all_models = IndexMap::default();
        for (id, model) in models {
            all_models.insert(id, BedrockModelConfig::Converse(model));
        }
        for (id, model) in mantle_models {
            all_models.insert(id, BedrockModelConfig::Mantle(model));
        }
        all_models
    }

    /// The offered model with the greatest id under `prefix`, which is the
    /// newest because Bedrock ids embed the version. Models that Bedrock
    /// lists as legacy are skipped.
    fn preferred_model(&self, prefix: &str, cx: &App) -> Option<LanguageModel> {
        let models = self.bedrock_models(cx);
        let model_catalog = self.state.read(cx).model_catalog.as_ref();
        let (_, model) = models
            .iter()
            .filter(|(id, _)| {
                id.starts_with(prefix) && model_catalog.is_none_or(|catalog| catalog.is_active(id))
            })
            .max_by_key(|(id, _)| *id)?;
        Some(self.offered_language_model(model, &models))
    }

    fn offered_language_model(
        &self,
        config: &BedrockModelConfig,
        models: &IndexMap<String, BedrockModelConfig>,
    ) -> LanguageModel {
        let mut model = language_model(config);
        if let BedrockModelConfig::Converse(config) = config
            && config.route == ConverseModelRoute::Automatic
            && anthropic_model_id(config)
                .is_some_and(|id| id.starts_with(anthropic::FABLE_MODEL_ID_PREFIX))
            && matches!(
                models.get(FABLE_FALLBACK_MODEL_ID),
                Some(BedrockModelConfig::Converse(target)) if target.route == ConverseModelRoute::Automatic
            )
        {
            model.refusal_fallback_model_id = Some(FABLE_FALLBACK_MODEL_ID);
        }
        model
    }

    /// The current configuration of `model`, if this provider still offers it.
    fn config(
        &self,
        model: &LanguageModel,
        cx: &App,
    ) -> Result<BedrockModelConfig, LanguageModelCompletionError> {
        self.bedrock_models(cx)
            .swap_remove(model.id.0.as_ref())
            .ok_or_else(|| unavailable_error(model))
    }
}

const FABLE_FALLBACK_MODEL_ID: &str = "anthropic.claude-opus-4-8";

/// The Converse model a settings entry describes. The entry's name is sent as
/// the model id exactly as written. An entry whose name Bedrock listed, or
/// that the capability catalog knows, takes what it leaves unsaid from that
/// model. Any other entry has only the capabilities it states.
fn converse_model_from_settings(
    model: &AvailableModel,
    model_catalog: Option<&ModelCatalog>,
    capability_catalog: &CapabilityCatalog,
) -> ConverseModel {
    let known = model_catalog
        .and_then(|model_catalog| {
            model_catalog.model_for_request_id(&model.name, capability_catalog)
        })
        .or_else(|| capability_catalog.model_for_request_id(&model.name));
    let known = known.as_ref();
    let thinking = match known.and_then(|known| known.thinking.clone()) {
        None => thinking_from_settings(model.thinking.as_ref()),
        Some(mut thinking) => {
            if let Some(settings::BedrockThinkingSettings::Config(config)) = &model.thinking
                && let ConverseThinking::Budget { budget_tokens } = &mut thinking
            {
                *budget_tokens = config.budget_tokens.unwrap_or(*budget_tokens);
            }
            Some(thinking)
        }
    };
    let thinking_disabled = matches!(
        model.thinking,
        Some(settings::BedrockThinkingSettings::Enabled(false))
    ) && !thinking
        .as_ref()
        .is_some_and(ConverseThinking::is_always_on);
    ConverseModel {
        id: model.name.clone(),
        request_id: model.name.clone(),
        base_model_id: known.map_or_else(
            || bedrock::base_model_id(&model.name).to_string(),
            |known| known.base_model_id.clone(),
        ),
        foundation_model_known: known.is_some_and(|known| known.foundation_model_known),
        route: ConverseModelRoute::Pinned,
        display_name: model
            .display_name
            .clone()
            .or_else(|| known.map(|known| known.display_name.clone()))
            .unwrap_or_else(|| model.name.clone()),
        max_tokens: model.max_tokens,
        max_output_tokens: model.max_output_tokens.or_else(|| match known {
            Some(known) => known.max_output_tokens,
            None => Some(4_096),
        }),
        default_temperature: model.default_temperature,
        supports_temperature: known.map_or(model.default_temperature.is_some(), |known| {
            known.supports_temperature
        }) && !thinking
            .as_ref()
            .is_some_and(ConverseThinking::is_always_on),
        supports_tool_use: model
            .supports_tools
            .or_else(|| known.map(|known| known.supports_tool_use))
            .unwrap_or(false),
        supports_images: model
            .supports_images
            .or_else(|| known.map(|known| known.supports_images))
            .unwrap_or(false),
        supports_caching: model.cache_configuration.is_some()
            || known.is_some_and(|known| known.supports_caching),
        thinking,
        thinking_disabled,
    }
}

fn language_model(config: &BedrockModelConfig) -> LanguageModel {
    match config {
        BedrockModelConfig::Converse(model) => converse_language_model(model),
        BedrockModelConfig::Mantle(model) => mantle_language_model(model),
    }
}

impl LanguageModelProvider for BedrockLanguageModelProvider {
    fn id(&self) -> LanguageModelProviderId {
        PROVIDER_ID
    }

    fn name(&self) -> LanguageModelProviderName {
        PROVIDER_NAME
    }

    fn icon(&self) -> IconOrSvg {
        IconOrSvg::Icon(IconName::AiBedrock)
    }

    fn default_model(&self, cx: &App) -> Option<LanguageModel> {
        self.preferred_model("anthropic.claude-sonnet-", cx)
    }

    fn default_fast_model(&self, cx: &App) -> Option<LanguageModel> {
        self.preferred_model("anthropic.claude-haiku-", cx)
    }

    fn provided_models(&self, cx: &App) -> Vec<LanguageModel> {
        let models = self.bedrock_models(cx);
        models
            .values()
            .map(|config| self.offered_language_model(config, &models))
            .collect()
    }

    fn is_authenticated(&self, cx: &App) -> bool {
        self.state.read(cx).is_authenticated()
    }

    fn authenticate(&self, cx: &mut App) -> Task<Result<(), AuthenticateError>> {
        self.state.update(cx, |state, cx| state.authenticate(cx))
    }

    fn refresh_models(&self, cx: &mut App) {
        self.state.update(cx, |state, cx| {
            // The loop is replaced rather than added to: dropping the task
            // cancels its hourly timer.
            state.refresh_capability_catalog_task = None;
            if state.is_authenticated() {
                state.refresh_capability_catalog_task =
                    Some(state.refresh_capability_catalog(true, cx));
            }
            state.restart_fetch_models_task(cx);
            cx.notify();
        });
    }

    fn settings_view(&self, _cx: &mut App) -> Option<ProviderSettingsView> {
        let state = self.state.clone();
        Some(ProviderSettingsView::SubPage(
            SubPageProviderSettings::new(move |window, cx| {
                cx.new(|cx| ConfigurationView::new(state.clone(), window, cx))
                    .into()
            })
            .description(InlineDescription::Text(
                "Configure authentication in settings or enter static credentials. Zed lists inference profiles in your Region. If AWS listing is unavailable, Zed keeps earlier results or offers models from models.dev. A custom endpoint uses the models.dev list and your settings.".into(),
            )),
        ))
    }
}

impl LanguageModelClient for BedrockLanguageModelProvider {
    fn stream_completion(
        &self,
        model: &LanguageModel,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let config = match cx.update(|cx| self.config(model, cx)) {
            Ok(config) => config,
            Err(error) => return async move { Err(error) }.boxed(),
        };
        let request_limiter = self.request_limiters.for_model(&model.id);
        match config {
            BedrockModelConfig::Converse(config) => {
                self.stream_converse_completion(&config, &request_limiter, request, cx)
            }
            BedrockModelConfig::Mantle(config) => {
                self.stream_mantle_completion(&config, &request_limiter, request, cx)
            }
        }
    }
}

impl LanguageModelProviderState for BedrockLanguageModelProvider {
    type ObservableEntity = State;

    fn observable_entity(&self) -> Option<Entity<Self::ObservableEntity>> {
        Some(self.state.clone())
    }
}

impl BedrockLanguageModelProvider {
    /// Resolves auth for one request, reusing unexpired cached credentials.
    fn resolve_request_auth(&self, cx: &AsyncApp) -> Task<Result<BedrockRequestAuth>> {
        let http_client = self.http_client.clone();
        self.resolve_request_auth_with(
            cx,
            |name| std::env::var(name).ok(),
            move |auth, region| resolve_request_auth(http_client, auth, region),
        )
    }

    /// [`Self::resolve_request_auth`], reading environment variables with
    /// `environment_variable` and resolving uncached auth with `resolve`.
    fn resolve_request_auth_with<ResolveFuture>(
        &self,
        cx: &AsyncApp,
        environment_variable: impl Fn(&str) -> Option<String>,
        resolve: impl FnOnce(BedrockAuth, String) -> ResolveFuture,
    ) -> Task<Result<BedrockRequestAuth>>
    where
        ResolveFuture: Future<Output = Result<BedrockRequestAuth>> + Send + 'static,
    {
        resolve_cached_request_auth(&self.state, cx, environment_variable, resolve)
    }

    fn stream_bedrock_request(
        &self,
        request: bedrock::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<BoxStream<'static, Result<BedrockStreamingResponse, anyhow::Error>>, BedrockError>,
    > {
        let http_client = self.http_client.clone();
        let auth_task = self.resolve_request_auth(cx);
        let (profile_name, endpoint, region, extra_headers) =
            self.state.read_with(cx, |state, _cx| {
                (
                    aws_profile_name(&state.auth),
                    state.settings.endpoint.clone(),
                    state.get_region(),
                    state.settings.custom_headers.clone(),
                )
            });

        let task = Tokio::spawn(cx, async move {
            let request_auth = auth_task.await.map_err(BedrockError::Other)?;
            let client =
                build_converse_client(http_client, request_auth, profile_name, endpoint, region)
                    .await;
            bedrock::stream_completion(client, request, extra_headers).await
        });
        async move { task.await.map_err(|e| BedrockError::Other(e.into()))? }.boxed()
    }
}

/// Resolves auth for one request to Bedrock, reusing the unexpired credentials
/// cached in `state`. Environment variables are read with
/// `environment_variable` and uncached auth is resolved with `resolve`.
fn resolve_cached_request_auth<ResolveFuture>(
    state: &Entity<State>,
    cx: &AsyncApp,
    environment_variable: impl Fn(&str) -> Option<String>,
    resolve: impl FnOnce(BedrockAuth, String) -> ResolveFuture,
) -> Task<Result<BedrockRequestAuth>>
where
    ResolveFuture: Future<Output = Result<BedrockRequestAuth>> + Send + 'static,
{
    let (auth, region, auth_generation, cached_credentials) =
        cx.read_entity(state, |state, _cx| {
            let (auth, region) = (state.auth.clone(), state.get_region());
            let fresh_until = SystemTime::now() + CREDENTIALS_EXPIRY_MARGIN;
            let cached_credentials = state
                .expiring_credentials
                .as_ref()
                .filter(|cached| {
                    cached.auth == auth
                        && cached.region == region
                        && cached
                            .credentials
                            .expiry()
                            .is_some_and(|expiry| expiry > fresh_until)
                })
                .map(|cached| cached.credentials.clone());
            (auth, region, state.auth_generation, cached_credentials)
        });
    let Some(auth) = auth else {
        return Task::ready(Err(anyhow!("Amazon Bedrock is not authenticated")));
    };
    if !matches!(auth, BedrockAuth::ApiKey { .. })
        && let Some(api_key) = environment_bearer_token(environment_variable)
    {
        return Task::ready(Ok(BedrockRequestAuth::ApiKey { api_key }));
    }
    if let Some(credentials) = cached_credentials {
        return Task::ready(Ok(BedrockRequestAuth::SigV4 { credentials }));
    }

    let resolve_task = Tokio::spawn_result(cx, resolve(auth.clone(), region.clone()));
    let timeout = cx.background_executor().timer(CREDENTIALS_LOAD_TIMEOUT);
    let state = state.clone();
    cx.spawn(async move |cx| {
        // Dropping `resolve_task` on timeout cancels the Tokio task.
        let request_auth = futures::select_biased! {
            request_auth = resolve_task.fuse() => request_auth?,
            _ = timeout.fuse() => anyhow::bail!(
                "timed out after {CREDENTIALS_LOAD_TIMEOUT:?} resolving AWS credentials"
            ),
        };
        if let BedrockRequestAuth::SigV4 { credentials } = &request_auth
            && credentials.expiry().is_some()
        {
            let credentials = credentials.clone();
            state.update(cx, |state, _| {
                if state.auth_generation == auth_generation && state.get_region() == region {
                    state.expiring_credentials = Some(ExpiringCredentials {
                        auth: Some(auth),
                        region,
                        credentials,
                    });
                }
            });
        }
        Ok(request_auth)
    })
}

/// Fetches the capability catalog and stores the part that describes
/// Bedrock's models at `cache_path`.
async fn fetch_capability_catalog(
    http_client: &dyn HttpClient,
    executor: &BackgroundExecutor,
    fs: &dyn Fs,
    cache_path: PathBuf,
) -> Result<CapabilityCatalog> {
    let catalog = model_catalog_cache::fetch(http_client, executor, CAPABILITY_CATALOG_URL).await?;
    let (capability_catalog, catalog) = CapabilityCatalog::parse_with_cache(&catalog)?;
    if let Err(error) = model_catalog_cache::store_cached(fs, cache_path, catalog).await {
        log::warn!("Failed to cache Bedrock model capabilities: {error:#}");
    }
    Ok(capability_catalog)
}

/// How long listing models may take. Zed's HTTP client applies no timeout of
/// its own to AWS SDK requests.
const LIST_MODELS_TIMEOUT: Duration = Duration::from_secs(30);

/// The AWS SDK configuration for one request from already-resolved auth. The
/// profile still applies, since it can configure more than credentials.
fn sdk_config_loader(
    http_client: AwsHttpClient,
    request_auth: BedrockRequestAuth,
    profile_name: Option<String>,
    region: String,
) -> aws_config::ConfigLoader {
    let mut config_loader = aws_config::defaults(BehaviorVersion::latest())
        .http_client(http_client)
        .region(Region::new(region));

    if let Some(profile_name) = profile_name {
        config_loader = config_loader.profile_name(profile_name);
    }

    match request_auth {
        BedrockRequestAuth::SigV4 { credentials } => {
            config_loader.credentials_provider(credentials)
        }
        BedrockRequestAuth::ApiKey { api_key } => config_loader
            .auth_scheme_preference(["httpBearerAuth".into()]) // https://github.com/smithy-lang/smithy-rs/pull/4241
            .token_provider(Token::new(api_key, None)),
    }
}

/// Builds a client for the Bedrock control plane, which lists models. The
/// `endpoint_url` setting does not apply to it: that setting names a
/// `bedrock-runtime` endpoint, which does not serve the listing APIs.
async fn build_control_plane_client(
    http_client: AwsHttpClient,
    request_auth: BedrockRequestAuth,
    profile_name: Option<String>,
    region: String,
) -> BedrockControlPlaneClient {
    let config = sdk_config_loader(http_client, request_auth, profile_name, region)
        .timeout_config(TimeoutConfig::disabled())
        .load()
        .await;
    BedrockControlPlaneClient::new(&config)
}

/// Builds a Converse client for one request from already-resolved auth.
async fn build_converse_client(
    http_client: AwsHttpClient,
    request_auth: BedrockRequestAuth,
    profile_name: Option<String>,
    endpoint: Option<String>,
    region: String,
) -> BedrockClient {
    let mut config_loader = sdk_config_loader(http_client, request_auth, profile_name, region)
        .stalled_stream_protection(StalledStreamProtectionConfig::disabled())
        .timeout_config(TimeoutConfig::disabled());

    if let Some(endpoint_url) = endpoint
        && !endpoint_url.is_empty()
    {
        config_loader = config_loader.endpoint_url(endpoint_url);
    }

    BedrockClient::new(&config_loader.load().await)
}

/// The id that the `anthropic` crate knows the Claude model behind `model`
/// by, which is the Bedrock id without its `anthropic.` namespace.
fn anthropic_model_id(model: &ConverseModel) -> Option<&str> {
    model
        .base_model_id
        .rsplit_once("anthropic.")
        .map(|(_, model_id)| model_id)
}

/// Claude models that think when a request has no `thinking` field, so that
/// turning thinking off takes an explicit `disabled`. Neither listing source
/// records this.
const THINKS_BY_DEFAULT: &[&str] = &["claude-opus-5", "claude-sonnet-5", "claude-haiku-5-5"];

fn thinks_by_default(model: &ConverseModel) -> bool {
    anthropic_model_id(model).is_some_and(|model_id| THINKS_BY_DEFAULT.contains(&model_id))
}

/// Whether `model` is driven through OpenAI's request conventions.
fn is_openai_model(model: &ConverseModel) -> bool {
    model.base_model_id.starts_with("openai.")
}

fn reasoning_source(model: &ConverseModel) -> String {
    if model.foundation_model_known {
        return model.base_model_id.clone();
    }
    // Deliberately drop opaque history if discovery later identifies the foundation:
    // a missed replay is preferable to sending a signed block to the wrong model.
    use sha2::{Digest as _, Sha256};
    format!("sha256:{:x}", Sha256::digest(model.request_id.as_bytes()))
}

fn supports_forced_tool_use(model: &ConverseModel) -> bool {
    anthropic_model_id(model).is_none_or(anthropic::supports_forced_tool_use)
}

/// Whether thinking can be turned off. This does not govern history replay.
fn always_reasons(model: &ConverseModel) -> bool {
    model
        .thinking
        .as_ref()
        .is_some_and(ConverseThinking::is_always_on)
}

fn converse_telemetry_id(model: &ConverseModel) -> String {
    let id = if model.foundation_model_known {
        &model.base_model_id
    } else {
        "custom"
    };
    format!("bedrock/{id}")
}

fn converse_language_model(model: &ConverseModel) -> LanguageModel {
    let default_effort = model
        .thinking
        .as_ref()
        .and_then(ConverseThinking::default_effort);
    LanguageModel {
        supports_tools: model.supports_tool_use,
        supports_images: model.supports_images,
        supports_thinking: model.thinking.is_some() && !model.thinking_disabled,
        supports_disabling_thinking: !always_reasons(model),
        refusal_fallback_model_id: None,
        supported_effort_levels: model
            .thinking
            .iter()
            .filter(|_| !model.thinking_disabled)
            .flat_map(ConverseThinking::efforts)
            .map(|effort| LanguageModelEffortLevel {
                name: effort.label().into(),
                value: effort.as_str().into(),
                is_default: Some(*effort) == default_effort,
                supports_disabling_thinking: None,
            })
            .collect(),
        // Add support for None - we'll filter tool calls at response
        tool_choice_support: LanguageModelToolChoiceSupport {
            auto: model.supports_tool_use,
            any: model.supports_tool_use && supports_forced_tool_use(model),
            none: model.supports_tool_use,
        },
        supports_streaming_tools: true,
        max_output_tokens: model.max_output_tokens,
        ..LanguageModel::new(
            LanguageModelId::from(model.id.clone()),
            LanguageModelName::from(model.display_name.clone()),
            PROVIDER_ID,
            PROVIDER_NAME,
            converse_telemetry_id(model),
            model.max_tokens,
        )
    }
}

impl BedrockLanguageModelProvider {
    fn stream_converse_completion(
        &self,
        config: &ConverseModel,
        request_limiter: &RateLimiter,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let (region, guardrail_identifier, guardrail_version) =
            cx.read_entity(&self.state, |state, _cx| {
                let (gid, gv) = state.get_guardrail_config();
                (state.get_region(), gid, gv)
            });

        let model_id = config.request_id.clone();
        let deny_tool_calls = request.tool_choice == Some(LanguageModelToolChoice::None);

        let request = match into_bedrock(request, config, guardrail_identifier, guardrail_version) {
            Ok(request) => request,
            Err(err) => return futures::future::ready(Err(err.into())).boxed(),
        };

        let request = self.stream_bedrock_request(request, cx);
        let model = config.clone();
        let executor = cx.background_executor().clone();
        let future = request_limiter.stream(async move {
            let response = request.await.map_err(|err| match err {
                BedrockError::Validation(ref msg) => {
                    if msg.contains("model identifier is invalid") {
                        LanguageModelCompletionError::from_provider_response(
                            PROVIDER_NAME,
                            None,
                            Some("ValidationException".to_string()),
                            format!("Bedrock rejected model ID `{model_id}` in {region}: {msg}"),
                            None,
                            ProviderErrorCategory::InvalidRequest,
                        )
                    } else {
                        LanguageModelCompletionError::from_provider_response(
                            PROVIDER_NAME,
                            None,
                            Some("ValidationException".to_string()),
                            msg.clone(),
                            None,
                            ProviderErrorCategory::InvalidRequest,
                        )
                    }
                }
                BedrockError::RateLimited => LanguageModelCompletionError::from_provider_response(
                    PROVIDER_NAME,
                    None,
                    Some("ThrottlingException".to_string()),
                    "Bedrock request was throttled".to_string(),
                    None,
                    ProviderErrorCategory::RateLimit,
                ),
                BedrockError::ServiceUnavailable => {
                    LanguageModelCompletionError::from_provider_response(
                        PROVIDER_NAME,
                        None,
                        Some("ServiceUnavailableException".to_string()),
                        "Bedrock service is temporarily unavailable".to_string(),
                        None,
                        ProviderErrorCategory::Overloaded,
                    )
                }
                BedrockError::AccessDenied(msg) => {
                    LanguageModelCompletionError::from_provider_response(
                        PROVIDER_NAME,
                        None,
                        Some("AccessDeniedException".to_string()),
                        msg,
                        None,
                        ProviderErrorCategory::Permission,
                    )
                }
                BedrockError::InternalServer(msg) => {
                    LanguageModelCompletionError::from_provider_response(
                        PROVIDER_NAME,
                        None,
                        Some("InternalServerException".to_string()),
                        msg,
                        None,
                        ProviderErrorCategory::InternalServer,
                    )
                }
                other => LanguageModelCompletionError::Other(anyhow!(other)),
            })?;
            let events = language_model::stream_in_background(
                map_to_language_model_completion_events(response, model).boxed(),
                executor,
            );

            if deny_tool_calls {
                Ok(deny_tool_use_events(events).boxed())
            } else {
                Ok(events.boxed())
            }
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }
}

const MANTLE_SELECTABLE_REASONING_EFFORTS: &[ReasoningEffort] = &[
    ReasoningEffort::Low,
    ReasoningEffort::Medium,
    ReasoningEffort::High,
    ReasoningEffort::XHigh,
];

fn mantle_default_reasoning_effort(model: &MantleModel) -> Option<ReasoningEffort> {
    model.supports_thinking().then_some(ReasoningEffort::Medium)
}

fn mantle_selected_reasoning_effort(
    request: &LanguageModelRequest,
    model: &MantleModel,
) -> Option<ReasoningEffort> {
    if !model.supports_thinking() {
        return None;
    }

    if request.thinking_allowed {
        request
            .thinking_effort
            .as_deref()
            .and_then(|effort| effort.parse::<ReasoningEffort>().ok())
            .filter(|effort| *effort != ReasoningEffort::None)
            .or_else(|| mantle_default_reasoning_effort(model))
    } else {
        Some(ReasoningEffort::None)
    }
}

fn mantle_supported_effort_levels(model: &MantleModel) -> Vec<LanguageModelEffortLevel> {
    let Some(default_effort) = mantle_default_reasoning_effort(model) else {
        return Vec::new();
    };

    MANTLE_SELECTABLE_REASONING_EFFORTS
        .iter()
        .copied()
        .map(|effort| LanguageModelEffortLevel {
            name: effort.label().into(),
            value: effort.value().into(),
            is_default: effort == default_effort,
            supports_disabling_thinking: None,
        })
        .collect()
}

/// Special-cases Mantle authorization failures with a message that points at
/// the separate `bedrock-mantle` IAM policy namespace instead of regular
/// `bedrock-runtime` permissions.
fn map_mantle_error(model: &MantleModel, error: RequestError) -> LanguageModelCompletionError {
    if let RequestError::HttpResponseError { status_code, .. } = &error
        && *status_code == http_client::http::StatusCode::FORBIDDEN
    {
        return LanguageModelCompletionError::from_provider_response(
            PROVIDER_NAME,
            Some(http_client::http::StatusCode::FORBIDDEN),
            None,
            format!(
                "Bedrock Mantle denied this request for {}. Mantle-only models require IAM \
                 permissions for the `bedrock-mantle` endpoint (for example via the \
                 `AmazonBedrockMantleInferenceAccess` managed policy) in addition to whatever \
                 permissions your existing Bedrock credentials already have.",
                model.display_name()
            ),
            None,
            ProviderErrorCategory::Permission,
        );
    }
    error.into()
}

/// The AWS profile named by profile or SSO auth, if any.
fn aws_profile_name(auth: &Option<BedrockAuth>) -> Option<String> {
    match auth {
        Some(BedrockAuth::NamedProfile { profile_name })
        | Some(BedrockAuth::SingleSignOn { profile_name }) => {
            Some(profile_name.clone()).filter(|name| !name.is_empty())
        }
        _ => None,
    }
}

/// Resolves provider settings into concrete request auth. A configured Bedrock
/// API key is sent as bearer auth; every other method resolves AWS
/// credentials to sign the request with SigV4.
async fn resolve_request_auth(
    http_client: AwsHttpClient,
    auth: BedrockAuth,
    region: String,
) -> Result<BedrockRequestAuth> {
    let profile_name = match auth {
        BedrockAuth::ApiKey { api_key } => return Ok(BedrockRequestAuth::ApiKey { api_key }),
        BedrockAuth::IamCredentials {
            access_key_id,
            secret_access_key,
            session_token,
        } => {
            return Ok(BedrockRequestAuth::SigV4 {
                credentials: Credentials::new(
                    access_key_id,
                    secret_access_key,
                    session_token,
                    None,
                    "zed-bedrock-provider",
                ),
            });
        }
        BedrockAuth::Automatic => None,
        BedrockAuth::NamedProfile { profile_name } | BedrockAuth::SingleSignOn { profile_name } => {
            Some(profile_name).filter(|name| !name.is_empty())
        }
    };

    let mut config_builder = aws_config::defaults(BehaviorVersion::latest())
        .http_client(http_client)
        .region(Region::new(region));
    if let Some(profile_name) = profile_name {
        config_builder = config_builder.profile_name(profile_name);
    }
    let credentials = config_builder
        .load()
        .await
        .credentials_provider()
        .context("no AWS credentials provider is configured")
        .context("resolving AWS credentials for Bedrock")?
        .provide_credentials()
        .await
        .context("failed to resolve AWS credentials")?;
    Ok(BedrockRequestAuth::SigV4 { credentials })
}

fn parse_mantle_chat_stream_line(line: &str) -> Result<ResponseStreamEvent> {
    match serde_json::from_str::<ResponseStreamResult>(line) {
        Ok(ResponseStreamResult::Ok(response)) => Ok(response),
        Ok(ResponseStreamResult::Err { error }) => Err(anyhow!(error.message)),
        Err(error) => {
            log::error!(
                "Failed to parse Mantle chat completion stream event: `{}`\nResponse: `{}`",
                error,
                line,
            );
            Err(anyhow!(error))
        }
    }
}

fn parse_mantle_response_stream_line(line: &str) -> Result<open_ai::responses::StreamEvent> {
    serde_json::from_str(line).map_err(|error| {
        log::error!(
            "Failed to parse Mantle responses stream event: `{}`\nResponse: `{}`",
            error,
            line,
        );
        anyhow!(error)
    })
}

async fn stream_mantle_sse<Request, Event>(
    client: &dyn HttpClient,
    provider_name: &str,
    url: &str,
    region: &str,
    auth: &BedrockRequestAuth,
    request: Request,
    extra_headers: &CustomHeaders,
    parse_stream_line: fn(&str) -> Result<Event>,
) -> std::result::Result<BoxStream<'static, Result<Event>>, RequestError>
where
    Request: Serialize,
    Event: Send + 'static,
{
    let body = serde_json::to_vec(&request).map_err(|error| RequestError::Other(error.into()))?;
    let mut request = HttpRequest::builder()
        .method(Method::POST)
        .uri(url)
        .header("Content-Type", "application/json")
        .extra_headers(extra_headers)
        .body(AsyncBody::from(body.clone()))
        .map_err(|error| RequestError::Other(error.into()))?;

    auth.apply(&mut request, &body, region)
        .map_err(RequestError::Other)?;

    let mut response = client.send(request).await?;
    if response.status().is_success() {
        let reader = BufReader::new(response.into_body());
        Ok(reader
            .lines()
            .filter_map(move |line| async move {
                match line {
                    Ok(line) => {
                        let line = line
                            .strip_prefix("data: ")
                            .or_else(|| line.strip_prefix("data:"))?;
                        if line == "[DONE]" || line.is_empty() {
                            None
                        } else {
                            Some(parse_stream_line(line))
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            })
            .boxed())
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|error| RequestError::Other(error.into()))?;

        Err(RequestError::HttpResponseError {
            provider: provider_name.to_owned(),
            status_code: response.status(),
            body,
            headers: Box::new(response.headers().clone()),
        })
    }
}

fn strip_unsupported_mantle_response_fields(request: &mut OpenAiResponseRequest) {
    request.context_management = None;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MantleMessageSnapshotKind {
    Undetermined,
    Cumulative,
    Independent,
}

struct MantleMessageSnapshot {
    item_id: String,
    text: String,
    emitted_text_length: usize,
    kind: MantleMessageSnapshotKind,
    phase: Option<String>,
    output_index: usize,
    content_index: Option<usize>,
}

/// The most recently completed message item, used to detect Mantle's cumulative
/// snapshot replays.
#[derive(Clone)]
struct MantlePreviousMessage {
    text: String,
    phase: Option<String>,
}

/// Adapts Mantle's cumulative Responses message snapshots to incremental events.
///
/// Mantle can emit a new message item containing all text produced so far, then
/// replay the corresponding output deltas. The shared OpenAI mapper assumes each
/// delta is new text, so forwarding those events directly duplicates the reply.
///
/// Only adjacent, same-phase, strict-prefix extensions are collapsed. Equal,
/// shrinking, divergent, or different-phase messages stay visible. Any non-message
/// output item (reasoning, tool call, etc.) is a hard boundary: it is forwarded
/// unchanged and clears the snapshot used for collapsing, so a message after a
/// tool call is never merged with the one before it.
struct MantleResponseEventMapper {
    open_ai_mapper: OpenAiResponseEventMapper,
    current_message: Option<MantleMessageSnapshot>,
    previous_message: Option<MantlePreviousMessage>,
    pending_message_events: Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
}

impl MantleResponseEventMapper {
    fn new() -> Self {
        Self {
            open_ai_mapper: OpenAiResponseEventMapper::new(PROVIDER_ID),
            current_message: None,
            previous_message: None,
            pending_message_events: Vec::new(),
        }
    }

    fn map_stream(
        mut self,
        events: BoxStream<'static, Result<OpenAiResponseStreamEvent>>,
    ) -> LanguageModelCompletionStream {
        events
            .flat_map(move |event| {
                futures::stream::iter(match event {
                    Ok(event) => self.map_event(event),
                    Err(error) => {
                        let mut events = self.finish_current_message();
                        events.push(Err(LanguageModelCompletionError::from(error)));
                        events
                    }
                })
            })
            .boxed()
    }

    fn map_event(
        &mut self,
        event: OpenAiResponseStreamEvent,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        match event {
            OpenAiResponseStreamEvent::OutputItemAdded {
                output_index,
                sequence_number,
                item,
            } => match &item {
                ResponseOutputItem::Message(message) => {
                    let mut events = self.finish_current_message();
                    let item_id = message.id.clone();
                    let phase = message.phase.clone();
                    if let Some(item_id) = item_id {
                        self.current_message = Some(MantleMessageSnapshot {
                            item_id,
                            text: String::new(),
                            emitted_text_length: 0,
                            kind: MantleMessageSnapshotKind::Undetermined,
                            phase,
                            output_index,
                            content_index: None,
                        });

                        self.map_message_added(OpenAiResponseStreamEvent::OutputItemAdded {
                            output_index,
                            sequence_number,
                            item,
                        });
                    } else {
                        self.flush_pending_message(&mut events, false);
                        events.extend(self.open_ai_mapper.map_event(
                            OpenAiResponseStreamEvent::OutputItemAdded {
                                output_index,
                                sequence_number,
                                item,
                            },
                        ));
                    }
                    events
                }
                _ => {
                    let mut events = self.finish_current_message();
                    self.previous_message = None;
                    events.extend(self.open_ai_mapper.map_event(
                        OpenAiResponseStreamEvent::OutputItemAdded {
                            output_index,
                            sequence_number,
                            item,
                        },
                    ));
                    events
                }
            },
            OpenAiResponseStreamEvent::OutputTextDelta {
                item_id,
                output_index,
                content_index,
                delta,
            } => self.map_text_delta(item_id, output_index, content_index, delta),
            OpenAiResponseStreamEvent::OutputTextDone {
                item_id,
                output_index,
                content_index,
                text,
            } => {
                let suffix = self
                    .current_message
                    .as_ref()
                    .filter(|message| message.item_id == item_id)
                    .and_then(|message| text.strip_prefix(&message.text))
                    .map(str::to_string);
                if let Some(suffix) = suffix {
                    if suffix.is_empty() {
                        return Vec::new();
                    }
                    return self.map_text_delta(item_id, output_index, content_index, suffix);
                }

                let mut events = self.finish_current_message();
                events.extend(self.open_ai_mapper.map_event(
                    OpenAiResponseStreamEvent::OutputTextDone {
                        item_id,
                        output_index,
                        content_index,
                        text,
                    },
                ));
                events
            }
            OpenAiResponseStreamEvent::ContentPartAdded { ref item_id, .. }
            | OpenAiResponseStreamEvent::ContentPartDone { ref item_id, .. }
            | OpenAiResponseStreamEvent::RefusalDelta { ref item_id, .. }
            | OpenAiResponseStreamEvent::RefusalDone { ref item_id, .. }
                if self
                    .current_message
                    .as_ref()
                    .is_some_and(|message| &message.item_id == item_id) =>
            {
                self.open_ai_mapper.map_event(event)
            }
            OpenAiResponseStreamEvent::OutputItemDone {
                output_index,
                sequence_number,
                item,
            } => {
                if let ResponseOutputItem::Message(message) = &item
                    && let Some(item_id) = message.id.as_ref()
                    && let Some(current_message) = self.current_message.as_mut()
                    && &current_message.item_id == item_id
                {
                    current_message.phase = message.phase.clone();
                }

                let is_non_message = !matches!(item, ResponseOutputItem::Message(_));
                let was_merged = matches!(
                    self.current_message.as_ref(),
                    Some(current_message)
                        if current_message.kind == MantleMessageSnapshotKind::Cumulative
                );
                let mut events = self.finish_current_message();
                if is_non_message {
                    self.previous_message = None;
                }
                let done_events =
                    self.open_ai_mapper
                        .map_event(OpenAiResponseStreamEvent::OutputItemDone {
                            output_index,
                            sequence_number,
                            item,
                        });
                if was_merged {
                    // The message was merged into the previous one, so the phase/
                    // reasoning metadata this `OutputItemDone` would otherwise emit
                    // again is already reflected by the metadata emitted earlier.
                    events.extend(done_events.into_iter().filter(|event| {
                        !matches!(event, Ok(LanguageModelCompletionEvent::ReasoningDetails(_)))
                    }));
                } else {
                    events.extend(done_events);
                }
                events
            }
            event => {
                let mut events = self.finish_current_message();
                events.extend(self.open_ai_mapper.map_event(event));
                events
            }
        }
    }

    fn map_message_added(&mut self, event: OpenAiResponseStreamEvent) {
        debug_assert!(
            self.pending_message_events.is_empty(),
            "pending message events must be flushed before a new message is added"
        );
        self.pending_message_events = self.open_ai_mapper.map_event(event);
    }

    fn map_text_delta(
        &mut self,
        item_id: String,
        output_index: usize,
        content_index: Option<usize>,
        delta: String,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        if !self
            .current_message
            .as_ref()
            .is_some_and(|current_message| current_message.item_id == item_id)
        {
            let mut events = self.finish_current_message();
            events.extend(self.open_ai_mapper.map_event(
                OpenAiResponseStreamEvent::OutputTextDelta {
                    item_id,
                    output_index,
                    content_index,
                    delta,
                },
            ));
            return events;
        }

        let Some(current_message) = self.current_message.as_mut() else {
            return Vec::new();
        };
        current_message.text.push_str(&delta);
        current_message.output_index = output_index;
        current_message.content_index = content_index;
        let kind = current_message.kind;

        match kind {
            MantleMessageSnapshotKind::Undetermined => {
                let snapshot = current_message.text.clone();
                let phase = current_message.phase.clone();
                let previous = self.previous_message.clone();

                if let Some(previous) = previous
                    && previous.phase == phase
                {
                    // Strict same-phase extension: previous text is a proper prefix of
                    // the new snapshot. Collapse the replayed prefix and emit only the
                    // suffix.
                    if snapshot.starts_with(&previous.text) && snapshot.len() > previous.text.len()
                    {
                        current_message.kind = MantleMessageSnapshotKind::Cumulative;
                        current_message.emitted_text_length = previous.text.len();
                        let mut events = Vec::new();
                        self.flush_pending_message(&mut events, true);
                        events.extend(self.map_snapshot_suffix(output_index, content_index));
                        return events;
                    }

                    // The snapshot is still a prefix of (or equal to) the previous
                    // message. A genuine cumulative replay keeps growing past the
                    // previous text, so defer the decision and withhold emission
                    // until it either exceeds the previous text or diverges.
                    if previous.text.starts_with(&snapshot) {
                        return Vec::new();
                    }
                }

                // Diverged from the previous message, had a different phase, or there
                // was no previous message: this is an independent message.
                current_message.kind = MantleMessageSnapshotKind::Independent;
                let mut events = Vec::new();
                self.flush_pending_message(&mut events, false);
                events.extend(self.map_text_event(item_id, output_index, content_index, snapshot));
                events
            }
            MantleMessageSnapshotKind::Cumulative => {
                self.map_snapshot_suffix(output_index, content_index)
            }
            MantleMessageSnapshotKind::Independent => {
                self.map_text_event(item_id, output_index, content_index, delta)
            }
        }
    }

    fn map_snapshot_suffix(
        &mut self,
        output_index: usize,
        content_index: Option<usize>,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let (item_id, suffix) = {
            let Some(current_message) = self.current_message.as_mut() else {
                return Vec::new();
            };

            if current_message.text.len() <= current_message.emitted_text_length {
                return Vec::new();
            }

            let suffix = current_message.text[current_message.emitted_text_length..].to_string();
            current_message.emitted_text_length = current_message.text.len();
            (current_message.item_id.clone(), suffix)
        };

        self.map_text_event(item_id, output_index, content_index, suffix)
    }

    fn map_text_event(
        &mut self,
        item_id: String,
        output_index: usize,
        content_index: Option<usize>,
        delta: String,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        if delta.is_empty() {
            return Vec::new();
        }

        self.open_ai_mapper
            .map_event(OpenAiResponseStreamEvent::OutputTextDelta {
                item_id,
                output_index,
                content_index,
                delta,
            })
    }

    fn finish_current_message(
        &mut self,
    ) -> Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
        let mut events = Vec::new();
        self.finish_current_message_into(&mut events);
        events
    }

    fn finish_current_message_into(
        &mut self,
        events: &mut Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
    ) {
        let Some(current_message) = self.current_message.take() else {
            self.flush_pending_message(events, false);
            return;
        };

        match current_message.kind {
            MantleMessageSnapshotKind::Cumulative => {
                self.flush_pending_message(events, true);
            }
            MantleMessageSnapshotKind::Independent => {
                self.flush_pending_message(events, false);
            }
            MantleMessageSnapshotKind::Undetermined => {
                // The message never exceeded or diverged from the previous one, so
                // it could not be confirmed as a cumulative replay. Treat it as an
                // independent message: emit the withheld StartMessage and any text
                // accumulated while deferring the decision.
                self.flush_pending_message(events, false);
                if !current_message.text.is_empty() {
                    events.extend(self.map_text_event(
                        current_message.item_id.clone(),
                        current_message.output_index,
                        current_message.content_index,
                        current_message.text.clone(),
                    ));
                }
            }
        }

        if !current_message.text.is_empty() {
            self.previous_message = Some(MantlePreviousMessage {
                text: current_message.text,
                phase: current_message.phase,
            });
        }
    }

    fn flush_pending_message(
        &mut self,
        events: &mut Vec<Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
        suppress_start_message: bool,
    ) {
        for event in self.pending_message_events.drain(..) {
            // When merging into the previous message, the phase/reasoning metadata
            // can't have changed since it was last emitted (a merge only happens for
            // adjacent, same-phase messages with no reasoning item in between), so
            // suppress both the StartMessage and the now-redundant ReasoningDetails.
            if suppress_start_message
                && matches!(
                    event,
                    Ok(LanguageModelCompletionEvent::StartMessage { .. })
                        | Ok(LanguageModelCompletionEvent::ReasoningDetails(_))
                )
            {
                continue;
            }
            events.push(event);
        }
    }
}

impl BedrockLanguageModelProvider {
    fn stream_mantle_request<Request, Event>(
        &self,
        config: &MantleModel,
        request_limiter: &RateLimiter,
        request: Request,
        cx: &AsyncApp,
        endpoint: &'static str,
        parse_stream_line: fn(&str) -> Result<Event>,
    ) -> BoxFuture<'static, Result<BoxStream<'static, Result<Event>>, LanguageModelCompletionError>>
    where
        Request: Serialize + Send + 'static,
        Event: Send + 'static,
    {
        let http_client = self.plain_http_client.clone();
        let model = config.clone();
        let auth_task = self.resolve_request_auth(cx);
        let region = cx.read_entity(&self.state, |state, _cx| state.get_region());
        let url = format!("{}/{}", mantle_endpoint_url(&region), endpoint);
        let extra_headers = cx.read_entity(&self.state, |state, _cx| {
            state.settings.custom_headers.clone()
        });
        let provider_name = PROVIDER_NAME.0.to_string();
        let future = request_limiter.stream(async move {
            let auth = auth_task
                .await
                .map_err(LanguageModelCompletionError::Other)?;
            stream_mantle_sse(
                http_client.as_ref(),
                &provider_name,
                &url,
                &region,
                &auth,
                request,
                &extra_headers,
                parse_stream_line,
            )
            .await
            .map_err(|err| map_mantle_error(&model, err))
        });

        async move { Ok(future.await?.boxed()) }.boxed()
    }

    fn stream_chat_completion(
        &self,
        config: &MantleModel,
        request_limiter: &RateLimiter,
        request: open_ai::Request,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<BoxStream<'static, Result<ResponseStreamEvent>>, LanguageModelCompletionError>,
    > {
        self.stream_mantle_request(
            config,
            request_limiter,
            request,
            cx,
            "chat/completions",
            parse_mantle_chat_stream_line,
        )
    }

    fn stream_response(
        &self,
        config: &MantleModel,
        request_limiter: &RateLimiter,
        request: OpenAiResponseRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<
        'static,
        Result<
            BoxStream<'static, Result<open_ai::responses::StreamEvent>>,
            LanguageModelCompletionError,
        >,
    > {
        let mut request = request;
        strip_unsupported_mantle_response_fields(&mut request);
        self.stream_mantle_request(
            config,
            request_limiter,
            request,
            cx,
            "responses",
            parse_mantle_response_stream_line,
        )
    }
}

fn mantle_language_model(model: &MantleModel) -> LanguageModel {
    LanguageModel {
        supports_tools: model.supports_tools(),
        supports_images: model.supports_images(),
        tool_choice_support: LanguageModelToolChoiceSupport {
            auto: model.supports_tools(),
            any: model.supports_tools(),
            none: true,
        },
        supports_streaming_tools: true,
        supports_thinking: model.supports_thinking(),
        supported_effort_levels: mantle_supported_effort_levels(model).into(),
        supports_split_token_display: true,
        max_output_tokens: Some(model.max_output_tokens()),
        ..LanguageModel::new(
            LanguageModelId::from(model.id().to_string()),
            LanguageModelName::from(model.display_name().to_string()),
            PROVIDER_ID,
            PROVIDER_NAME,
            format!(
                "bedrock-mantle/{}",
                model
                    .known_foundation_model_id
                    .as_deref()
                    .unwrap_or("custom")
            ),
            model.max_token_count(),
        )
    }
}

impl BedrockLanguageModelProvider {
    fn stream_mantle_completion(
        &self,
        config: &MantleModel,
        request_limiter: &RateLimiter,
        request: LanguageModelRequest,
        cx: &AsyncApp,
    ) -> BoxFuture<'static, Result<LanguageModelCompletionStream, LanguageModelCompletionError>>
    {
        let region = cx.read_entity(&self.state, |state, _cx| state.get_region());

        if !MANTLE_SUPPORTED_REGIONS.contains(&region.as_str()) {
            let display_name = config.display_name().to_string();
            let supported = MANTLE_SUPPORTED_REGIONS.join(", ");
            return futures::future::ready(Err(LanguageModelCompletionError::Other(anyhow!(
                "{display_name} is not available in {region} because Bedrock Mantle isn't offered \
                 there. Try switching to one of the following regions: {supported}."
            ))))
            .boxed();
        }

        let model_id = config.request_id().to_string();
        let max_output_tokens = Some(config.max_output_tokens());

        match config.protocol() {
            MantleProtocol::Responses => {
                let request = match into_open_ai_response(
                    request,
                    &model_id,
                    config.supports_tools(),
                    false,
                    max_output_tokens,
                    mantle_default_reasoning_effort(config),
                    config.supports_thinking(),
                    &PROVIDER_ID,
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };
                let completions = self.stream_response(config, request_limiter, request, cx);
                let executor = cx.background_executor().clone();
                async move {
                    let mapper = MantleResponseEventMapper::new();
                    Ok(language_model::stream_in_background(
                        mapper.map_stream(completions.await?).boxed(),
                        executor,
                    ))
                }
                .boxed()
            }
            MantleProtocol::ChatCompletions => {
                let reasoning_effort = mantle_selected_reasoning_effort(&request, config);
                let request = match into_open_ai(
                    request,
                    &model_id,
                    config.supports_tools(),
                    false,
                    max_output_tokens,
                    ChatCompletionMaxTokensParameter::MaxCompletionTokens,
                    reasoning_effort,
                    false,
                ) {
                    Ok(request) => request,
                    Err(error) => return async move { Err(error.into()) }.boxed(),
                };
                let completions = self.stream_chat_completion(config, request_limiter, request, cx);
                let executor = cx.background_executor().clone();
                async move {
                    let mapper = ChatCompletionEventMapper::new();
                    Ok(language_model::stream_in_background(
                        mapper.map_stream(completions.await?).boxed(),
                        executor,
                    ))
                }
                .boxed()
            }
        }
    }
}

fn deny_tool_use_events(
    events: impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>>,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    events.map(|event| {
        match event {
            Ok(LanguageModelCompletionEvent::ToolUse(tool_use)) => {
                // Convert tool use to an error message if model decided to call it
                Ok(LanguageModelCompletionEvent::Text(format!(
                    "\n\n[Error: Tool calls are disabled in this context. Attempted to call '{}']",
                    tool_use.name
                )))
            }
            other => other,
        }
    })
}

const DEEPSEEK_R1_MODEL_ID: &str = "deepseek.r1-v1:0";

fn prepare_bedrock_content(
    content: Vec<BedrockInnerContent>,
    tool_result_images_as_siblings: bool,
) -> Vec<BedrockInnerContent> {
    let mut prepared = Vec::with_capacity(content.len());
    let mut images = Vec::new();
    for block in content {
        match block {
            BedrockInnerContent::ToolResult(mut result) if tool_result_images_as_siblings => {
                // OpenAI expects parallel tool results before their sibling images.
                // <https://docs.aws.amazon.com/bedrock/latest/APIReference/API_runtime_ToolResultContentBlock.html>
                let mut has_images = false;
                for content in std::mem::take(&mut result.content) {
                    match content {
                        BedrockToolResultContentBlock::Image(image) => {
                            has_images = true;
                            images.push(BedrockInnerContent::Image(image))
                        }
                        other => result.content.push(other),
                    }
                }
                if has_images && result.content.is_empty() {
                    result.content.push(BedrockToolResultContentBlock::Text(
                        "See the attached image output.".into(),
                    ));
                }
                prepared.push(BedrockInnerContent::ToolResult(result));
            }
            other => {
                prepared.append(&mut images);
                prepared.push(other);
            }
        }
    }
    prepared.append(&mut images);
    prepared
}

pub fn into_bedrock(
    request: LanguageModelRequest,
    model: &ConverseModel,
    guardrail_identifier: Option<String>,
    guardrail_version: Option<String>,
) -> Result<bedrock::Request> {
    if model.max_output_tokens == Some(0) {
        anyhow::bail!(
            "available_models entry `{}` sets max_output_tokens to 0; the output limit must be positive",
            model.id
        );
    }
    let max_output_tokens = request.effective_max_output_tokens(model.max_output_tokens);

    let supports_caching = model.supports_caching;
    let supports_tool_use = model.supports_tool_use;
    let is_openai_model = is_openai_model(model);
    if request.tool_choice == Some(LanguageModelToolChoice::Any) && !supports_forced_tool_use(model)
    {
        anyhow::bail!("{} does not support forced tool use", model.request_id);
    }

    let mut new_messages: Vec<BedrockMessage> = Vec::new();
    let mut system_message = String::new();

    // Track whether messages contain tool content - Bedrock requires toolConfig
    // when tool blocks are present, so we may need to add a dummy tool
    let mut messages_contain_tool_content = false;

    for message in request.messages {
        if message.contents_empty() {
            continue;
        }

        let stored_source = message
            .reasoning_details
            .as_ref()
            .and_then(|details| details.get("bedrock_model"))
            .and_then(Value::as_str);
        let replay_reasoning = match stored_source {
            Some(source) => reasoning_source(model) == source,
            None => {
                !is_openai_model
                    && !anthropic_model_id(model)
                        .is_some_and(anthropic::binds_thinking_blocks_to_prefix)
            }
        };

        match message.role {
            Role::User | Role::Assistant => {
                if message.content.iter().any(|content| {
                    matches!(
                        content,
                        MessageContent::ToolUse(LanguageModelToolUse {
                            input: language_model::LanguageModelToolUseInput::Text(_),
                            ..
                        })
                    )
                }) {
                    anyhow::bail!("Bedrock does not support custom tools");
                }
                let mut bedrock_message_content: Vec<BedrockInnerContent> = message
                    .content
                    .into_iter()
                    .filter_map(|content| match content {
                        MessageContent::Text(text) => {
                            if !text.is_empty() {
                                Some(BedrockInnerContent::Text(text))
                            } else {
                                None
                            }
                        }
                        MessageContent::Compaction(_) | MessageContent::ReasoningEffort { .. } => {
                            None
                        }
                        MessageContent::Thinking { text, signature } => {
                            if !replay_reasoning {
                                return None;
                            }
                            if model.base_model_id.contains(DEEPSEEK_R1_MODEL_ID) {
                                // DeepSeekR1 doesn't support thinking blocks
                                // And the AWS API demands that you strip them
                                return None;
                            }
                            if signature.is_none() {
                                // Thinking blocks without a signature are invalid
                                // (e.g. from cancellation mid-think) and must be
                                // stripped to avoid API errors.
                                return None;
                            }
                            let thinking = BedrockThinkingTextBlock::builder()
                                .text(text)
                                .set_signature(signature)
                                .build()
                                .context("failed to build reasoning block")
                                .log_err()?;

                            Some(BedrockInnerContent::ReasoningContent(
                                BedrockThinkingBlock::ReasoningText(thinking),
                            ))
                        }
                        MessageContent::RedactedThinking(blob) => {
                            if !replay_reasoning {
                                return None;
                            }
                            if model.base_model_id.contains(DEEPSEEK_R1_MODEL_ID) {
                                // DeepSeekR1 doesn't support thinking blocks
                                // And the AWS API demands that you strip them
                                return None;
                            }
                            use base64::Engine;

                            let bytes = base64::engine::general_purpose::STANDARD
                                .decode(blob.as_bytes())
                                .context("Invalid base64 in Bedrock reasoning history")
                                .log_err()?;
                            let redacted =
                                BedrockThinkingBlock::RedactedContent(BedrockBlob::new(bytes));

                            Some(BedrockInnerContent::ReasoningContent(redacted))
                        }
                        MessageContent::ToolUse(tool_use) => {
                            messages_contain_tool_content = true;
                            let input =
                                if let language_model::LanguageModelToolUseInput::Json(input) =
                                    &tool_use.input
                                {
                                    if input.is_null() {
                                        // Bedrock API requires valid JsonValue, not null, for tool use input
                                        value_to_aws_document(&serde_json::json!({}))
                                    } else {
                                        value_to_aws_document(input)
                                    }
                                } else {
                                    value_to_aws_document(&serde_json::json!({}))
                                };
                            BedrockToolUseBlock::builder()
                                .name(tool_use.name.to_string())
                                .tool_use_id(tool_use.id.to_string())
                                .input(input)
                                .build()
                                .context("failed to build Bedrock tool use block")
                                .log_err()
                                .map(BedrockInnerContent::ToolUse)
                        }
                        MessageContent::ToolResult(tool_result) => {
                            messages_contain_tool_content = true;
                            let mut builder = BedrockToolResultBlock::builder()
                                .tool_use_id(tool_result.tool_use_id.to_string());
                            for part in tool_result.content {
                                let block = match part {
                                    LanguageModelToolResultContent::Text(text) => {
                                        BedrockToolResultContentBlock::Text(text.to_string())
                                    }
                                    LanguageModelToolResultContent::Image(image) => {
                                        use base64::Engine;

                                        match base64::engine::general_purpose::STANDARD
                                            .decode(image.source.as_bytes())
                                        {
                                            Ok(image_bytes) => {
                                                match BedrockImageBlock::builder()
                                                    .format(BedrockImageFormat::Png)
                                                    .source(BedrockImageSource::Bytes(
                                                        BedrockBlob::new(image_bytes),
                                                    ))
                                                    .build()
                                                {
                                                    Ok(image_block) => {
                                                        BedrockToolResultContentBlock::Image(
                                                            image_block,
                                                        )
                                                    }
                                                    Err(err) => {
                                                        BedrockToolResultContentBlock::Text(
                                                            format!(
                                                                "[Failed to build image block: {}]",
                                                                err
                                                            ),
                                                        )
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                BedrockToolResultContentBlock::Text(format!(
                                                    "[Failed to decode tool result image: {}]",
                                                    err
                                                ))
                                            }
                                        }
                                    }
                                };
                                builder = builder.content(block);
                            }
                            builder
                                .status({
                                    if tool_result.is_error {
                                        BedrockToolResultStatus::Error
                                    } else {
                                        BedrockToolResultStatus::Success
                                    }
                                })
                                .build()
                                .context("failed to build Bedrock tool result block")
                                .log_err()
                                .map(BedrockInnerContent::ToolResult)
                        }
                        MessageContent::Image(image) => {
                            use base64::Engine;

                            let image_bytes = base64::engine::general_purpose::STANDARD
                                .decode(image.source.as_bytes())
                                .context("failed to decode base64 image data")
                                .log_err()?;

                            BedrockImageBlock::builder()
                                .format(BedrockImageFormat::Png)
                                .source(BedrockImageSource::Bytes(BedrockBlob::new(image_bytes)))
                                .build()
                                .context("failed to build Bedrock image block")
                                .log_err()
                                .map(BedrockInnerContent::Image)
                        }
                    })
                    .collect();
                for block in &bedrock_message_content {
                    if !supports_tool_use
                        && matches!(
                            block,
                            BedrockInnerContent::ToolUse(_) | BedrockInnerContent::ToolResult(_)
                        )
                    {
                        anyhow::bail!(
                            "{} cannot accept tool calls in this thread. Choose another model or start a new thread.",
                            model.display_name
                        );
                    }
                    if !model.supports_images
                        && (matches!(block, BedrockInnerContent::Image(_))
                            || matches!(block, BedrockInnerContent::ToolResult(result) if result.content().iter().any(|part| matches!(part, BedrockToolResultContentBlock::Image(_)))))
                    {
                        anyhow::bail!(
                            "{} cannot accept images in this thread. Choose another model or start a new thread.",
                            model.display_name
                        );
                    }
                }
                bedrock_message_content =
                    prepare_bedrock_content(bedrock_message_content, is_openai_model);
                if message.cache && supports_caching && !bedrock_message_content.is_empty() {
                    bedrock_message_content.push(BedrockInnerContent::CachePoint(
                        CachePointBlock::builder()
                            .r#type(CachePointType::Default)
                            .build()
                            .context("failed to build cache point block")?,
                    ));
                }
                let bedrock_role = match message.role {
                    Role::User => bedrock::BedrockRole::User,
                    Role::Assistant => bedrock::BedrockRole::Assistant,
                    Role::System => unreachable!("System role should never occur here"),
                };
                if bedrock_message_content.is_empty() {
                    continue;
                }

                if let Some(last_message) = new_messages.last_mut()
                    && last_message.role == bedrock_role
                {
                    last_message.content.extend(bedrock_message_content);
                    continue;
                }
                new_messages.push(
                    BedrockMessage::builder()
                        .role(bedrock_role)
                        .set_content(Some(bedrock_message_content))
                        .build()
                        .context("failed to build Bedrock message")?,
                );
            }
            Role::System => {
                if !system_message.is_empty() {
                    system_message.push_str("\n\n");
                }
                system_message.push_str(&message.string_contents());
            }
        }
    }

    let mut tool_spec: Vec<BedrockTool> = if supports_tool_use {
        request
            .tools
            .iter()
            .map(|tool| {
                let language_model::LanguageModelRequestToolInput::Function {
                    input_schema, ..
                } = &tool.input
                else {
                    anyhow::bail!("Bedrock does not support custom tools");
                };
                Ok(BedrockTool::ToolSpec(
                    BedrockToolSpec::builder()
                        .name(tool.name.clone())
                        .description(tool.description.clone())
                        .input_schema(BedrockToolInputSchema::Json(value_to_aws_document(
                            input_schema,
                        )))
                        .build()
                        .context("failed to build Bedrock tool spec")?,
                ))
            })
            .collect::<Result<_>>()?
    } else {
        Vec::new()
    };

    // Bedrock requires toolConfig when messages contain tool use/result blocks.
    // If no tools are defined but messages contain tool content (e.g., when
    // summarising a conversation that used tools), add a dummy tool to satisfy
    // the API requirement.
    if supports_tool_use && tool_spec.is_empty() && messages_contain_tool_content {
        tool_spec.push(BedrockTool::ToolSpec(
            BedrockToolSpec::builder()
                .name("_placeholder")
                .description("Placeholder tool to satisfy Bedrock API requirements when conversation history contains tool usage")
                .input_schema(BedrockToolInputSchema::Json(value_to_aws_document(
                    &serde_json::json!({"type": "object", "properties": {}}),
                )))
                .build()
                .context("failed to build placeholder tool spec")?,
        ));
    }

    if !tool_spec.is_empty() && supports_caching {
        tool_spec.push(BedrockTool::CachePoint(
            CachePointBlock::builder()
                .r#type(CachePointType::Default)
                .build()
                .context("failed to build cache point block")?,
        ));
    }

    let tool_choice = match request.tool_choice {
        Some(LanguageModelToolChoice::Auto) | None => {
            BedrockToolChoice::Auto(BedrockAutoToolChoice::builder().build())
        }
        Some(LanguageModelToolChoice::Any) => {
            BedrockToolChoice::Any(BedrockAnyToolChoice::builder().build())
        }
        Some(LanguageModelToolChoice::None) => {
            // For None, we still use Auto but will filter out tool calls in the response
            BedrockToolChoice::Auto(BedrockAutoToolChoice::builder().build())
        }
    };
    let tool_config = if tool_spec.is_empty() {
        None
    } else {
        Some(
            BedrockToolConfig::builder()
                .set_tools(Some(tool_spec))
                .tool_choice(tool_choice)
                .build()?,
        )
    };

    let mut system_blocks: Vec<BedrockSystemContentBlock> = Vec::new();
    if !system_message.is_empty() {
        system_blocks.push(BedrockSystemContentBlock::Text(system_message));
        if supports_caching {
            system_blocks.push(BedrockSystemContentBlock::CachePoint(
                CachePointBlock::builder()
                    .r#type(CachePointType::Default)
                    .build()
                    .context("failed to build system cache point block")?,
            ));
        }
    }

    let thinking_allowed = request.thinking_allowed && !model.thinking_disabled;
    let selected_effort = |efforts: &[BedrockAdaptiveThinkingEffort], default_effort| {
        if !thinking_allowed {
            return default_effort;
        }
        request
            .thinking_effort
            .as_deref()
            .and_then(BedrockAdaptiveThinkingEffort::from_value)
            .filter(|effort| efforts.contains(effort))
            .unwrap_or(default_effort)
    };
    let anthropic_model_id = anthropic_model_id(model);
    let thinking = match &model.thinking {
        // Sonnet 5.5 rejects `disabled`; this is its lowest thinking mode.
        _ if !thinking_allowed && anthropic_model_id == Some("claude-sonnet-5-5") => {
            Some(bedrock::Thinking::BetweenTools)
        }
        Some(ConverseThinking::Reasoning {
            efforts,
            default_effort,
            supports_none,
        }) => Some(bedrock::Thinking::Reasoning {
            effort: (thinking_allowed || !supports_none)
                .then(|| selected_effort(efforts, *default_effort)),
        }),
        Some(ConverseThinking::Adaptive {
            default_effort,
            always_on,
            efforts,
        }) if thinking_allowed || *always_on => Some(bedrock::Thinking::Adaptive {
            effort: selected_effort(efforts, *default_effort),
            binding_controls_beta: anthropic_model_id
                .filter(|model_id| anthropic::binds_thinking_blocks_to_prefix(model_id))
                .map(|_| anthropic::THINKING_BINDING_CONTROLS_BETA_HEADER.to_string()),
        }),
        Some(ConverseThinking::Budget { budget_tokens }) if thinking_allowed => {
            let budget_tokens = max_output_tokens
                .filter(|cap| cap <= budget_tokens)
                .map_or(*budget_tokens, |cap| cap / 2);
            // Halving a binding cap leaves room for the answer. Below the 1,024-token
            // minimum, leave budget thinking off for this request.
            // https://platform.claude.com/docs/en/build-with-claude/extended-thinking#budget-rules-and-tuning
            (budget_tokens >= 1_024).then_some(bedrock::Thinking::Enabled {
                budget_tokens: Some(budget_tokens),
            })
        }
        _ if !thinking_allowed && thinks_by_default(model) => Some(bedrock::Thinking::Disabled),
        _ => None,
    };
    // Thinking and sampling controls are incompatible on Anthropic requests.
    // https://platform.claude.com/docs/en/build-with-claude/thinking#sampling-parameters
    let sends_anthropic_thinking = matches!(
        thinking,
        Some(
            bedrock::Thinking::Enabled { .. }
                | bedrock::Thinking::Adaptive { .. }
                | bedrock::Thinking::Disabled
                | bedrock::Thinking::BetweenTools
        )
    );
    let temperature = if model.supports_temperature
        && !matches!(
            anthropic_model_id,
            Some("claude-sonnet-5-5" | "claude-haiku-5-5")
        )
        && !sends_anthropic_thinking
        && !(anthropic_model_id.is_some() && thinking.is_some())
    {
        request.temperature.or(model.default_temperature)
    } else {
        None
    };

    Ok(bedrock::Request {
        model: model.request_id.clone(),
        messages: new_messages,
        max_tokens: max_output_tokens,
        system: system_blocks,
        tools: tool_config,
        thinking,
        temperature,
        top_p: None,
        guardrail_identifier,
        guardrail_version,
    })
}

pub fn map_to_language_model_completion_events(
    events: Pin<Box<dyn Send + Stream<Item = Result<BedrockStreamingResponse, anyhow::Error>>>>,
    model: ConverseModel,
) -> impl Stream<Item = Result<LanguageModelCompletionEvent, LanguageModelCompletionError>> {
    struct RawToolUse {
        id: String,
        name: String,
        input_json: String,
    }

    struct State {
        events: Pin<Box<dyn Send + Stream<Item = Result<BedrockStreamingResponse, anyhow::Error>>>>,
        tool_uses_by_index: HashMap<i32, RawToolUse>,
        redacted_thinking_by_index: HashMap<i32, Vec<u8>>,
        reasoning_model: Option<String>,
        emitted_tool_use: bool,
    }

    let report_refusals = is_openai_model(&model)
        || anthropic_model_id(&model)
            .is_some_and(|model_id| model_id.starts_with(anthropic::FABLE_MODEL_ID_PREFIX));
    // Preserve opaque reasoning and its source independently of optional catalog
    // metadata, so a catalog refresh cannot change an existing thread's replay.
    let initial_state = State {
        events,
        tool_uses_by_index: HashMap::default(),
        redacted_thinking_by_index: HashMap::default(),
        reasoning_model: Some(reasoning_source(&model)),
        emitted_tool_use: false,
    };

    futures::stream::unfold(initial_state, move |mut state| async move {
        match state.events.next().await {
            Some(event_result) => match event_result {
                Ok(event) => {
                    let result = match event {
                        ConverseStreamOutput::ContentBlockDelta(cb_delta) => match cb_delta.delta {
                            Some(ContentBlockDelta::Text(text)) => {
                                Some(Ok(LanguageModelCompletionEvent::Text(text)))
                            }
                            Some(ContentBlockDelta::ToolUse(tool_output)) => {
                                if let Some(tool_use) = state
                                    .tool_uses_by_index
                                    .get_mut(&cb_delta.content_block_index)
                                {
                                    tool_use.input_json.push_str(tool_output.input());
                                    if let Ok(input) = serde_json::from_str::<serde_json::Value>(
                                        &fix_streamed_json(&tool_use.input_json),
                                    ) {
                                        Some(Ok(LanguageModelCompletionEvent::ToolUse(
                                            LanguageModelToolUse {
                                                id: tool_use.id.clone().into(),
                                                name: tool_use.name.clone().into(),
                                                is_input_complete: false,
                                                raw_input: tool_use.input_json.clone(),
                                                input:
                                                    language_model::LanguageModelToolUseInput::Json(
                                                        input,
                                                    ),
                                                thought_signature: None,
                                            },
                                        )))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                            Some(ContentBlockDelta::ReasoningContent(thinking)) => match thinking {
                                ReasoningContentBlockDelta::Text(thoughts) => {
                                    Some(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: thoughts,
                                        signature: None,
                                    }))
                                }
                                ReasoningContentBlockDelta::Signature(sig) => {
                                    Some(Ok(LanguageModelCompletionEvent::Thinking {
                                        text: "".into(),
                                        signature: Some(sig),
                                    }))
                                }
                                ReasoningContentBlockDelta::RedactedContent(redacted) => {
                                    // A reasoning block can span multiple binary deltas.
                                    state
                                        .redacted_thinking_by_index
                                        .entry(cb_delta.content_block_index)
                                        .or_default()
                                        .extend(redacted.into_inner());
                                    None
                                }
                                _ => None,
                            },
                            _ => None,
                        },
                        ConverseStreamOutput::ContentBlockStart(cb_start) => {
                            if let Some(ContentBlockStart::ToolUse(tool_start)) = cb_start.start {
                                state.tool_uses_by_index.insert(
                                    cb_start.content_block_index,
                                    RawToolUse {
                                        id: tool_start.tool_use_id,
                                        name: tool_start.name,
                                        input_json: String::new(),
                                    },
                                );
                            }
                            None
                        }
                        ConverseStreamOutput::ContentBlockStop(cb_stop) => state
                            .redacted_thinking_by_index
                            .remove(&cb_stop.content_block_index)
                            .map(|data| {
                                use base64::Engine;
                                Ok(LanguageModelCompletionEvent::RedactedThinking {
                                    data: base64::engine::general_purpose::STANDARD.encode(data),
                                })
                            })
                            .or_else(|| {
                                state
                                    .tool_uses_by_index
                                    .remove(&cb_stop.content_block_index)
                                    .map(|tool_use| {
                                        state.emitted_tool_use = true;

                                        let input = parse_tool_arguments(&tool_use.input_json)
                                            .unwrap_or_else(|_| Value::Object(Default::default()));

                                        Ok(LanguageModelCompletionEvent::ToolUse(
                                            LanguageModelToolUse {
                                                id: tool_use.id.into(),
                                                name: tool_use.name.into(),
                                                is_input_complete: true,
                                                raw_input: tool_use.input_json,
                                                input:
                                                    language_model::LanguageModelToolUseInput::Json(
                                                        input,
                                                    ),
                                                thought_signature: None,
                                            },
                                        ))
                                    })
                            }),
                        ConverseStreamOutput::Metadata(cb_meta) => cb_meta.usage.map(|metadata| {
                            Ok(LanguageModelCompletionEvent::UsageUpdate(TokenUsage {
                                input_tokens: metadata.input_tokens as u64,
                                output_tokens: metadata.output_tokens as u64,
                                cache_creation_input_tokens: metadata
                                    .cache_write_input_tokens
                                    .unwrap_or_default()
                                    as u64,
                                cache_read_input_tokens: metadata
                                    .cache_read_input_tokens
                                    .unwrap_or_default()
                                    as u64,
                            }))
                        }),
                        ConverseStreamOutput::MessageStop(message_stop) => {
                            let stop_reason = if report_refusals
                                && (message_stop.stop_reason == StopReason::ContentFiltered
                                    || message_stop.stop_reason.as_str() == "refusal")
                            {
                                language_model::StopReason::Refusal(None)
                            } else if state.emitted_tool_use {
                                // Some models (e.g. Kimi) send EndTurn even when
                                // they've made tool calls. Trust the content over
                                // the stop reason.
                                language_model::StopReason::ToolUse
                            } else {
                                match message_stop.stop_reason {
                                    StopReason::ToolUse => language_model::StopReason::ToolUse,
                                    _ => language_model::StopReason::EndTurn,
                                }
                            };
                            Some(Ok(LanguageModelCompletionEvent::Stop(stop_reason)))
                        }
                        _ => None,
                    };

                    let source = match &result {
                        Some(Ok(
                            LanguageModelCompletionEvent::Thinking {
                                signature: Some(_), ..
                            }
                            | LanguageModelCompletionEvent::RedactedThinking { .. },
                        )) => state.reasoning_model.take(),
                        _ => None,
                    };
                    let details = source.map(|source| {
                        Ok(LanguageModelCompletionEvent::ReasoningDetails(
                            serde_json::json!({"bedrock_model": source}),
                        ))
                    });
                    Some(([details, result], state))
                }
                Err(err) => Some((
                    [None, Some(Err(LanguageModelCompletionError::Other(err)))],
                    state,
                )),
            },
            None => None,
        }
    })
    .flat_map(|events| futures::stream::iter(events.into_iter().flatten()))
}

struct ConfigurationView {
    access_key_id_editor: Entity<InputField>,
    secret_access_key_editor: Entity<InputField>,
    session_token_editor: Entity<InputField>,
    bearer_token_editor: Entity<InputField>,
    state: Entity<State>,
    load_credentials_task: Option<Task<()>>,
    focus_handle: FocusHandle,
}

impl ConfigurationView {
    const PLACEHOLDER_ACCESS_KEY_ID_TEXT: &'static str = "XXXXXXXXXXXXXXXX";
    const PLACEHOLDER_SECRET_ACCESS_KEY_TEXT: &'static str =
        "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    const PLACEHOLDER_SESSION_TOKEN_TEXT: &'static str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    const PLACEHOLDER_BEARER_TOKEN_TEXT: &'static str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    fn new(state: Entity<State>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        cx.observe(&state, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let access_key_id_editor = cx.new(|cx| {
            InputField::new(window, cx, Self::PLACEHOLDER_ACCESS_KEY_ID_TEXT)
                .label("Access Key ID")
                .tab_index(0)
                .tab_stop(true)
        });

        let secret_access_key_editor = cx.new(|cx| {
            InputField::new(window, cx, Self::PLACEHOLDER_SECRET_ACCESS_KEY_TEXT)
                .label("Secret Access Key")
                .tab_index(1)
                .tab_stop(true)
        });

        let session_token_editor = cx.new(|cx| {
            InputField::new(window, cx, Self::PLACEHOLDER_SESSION_TOKEN_TEXT)
                .label("Session Token (Optional)")
                .tab_index(2)
                .tab_stop(true)
        });

        let bearer_token_editor = cx.new(|cx| {
            InputField::new(window, cx, Self::PLACEHOLDER_BEARER_TOKEN_TEXT)
                .label("Bedrock API Key")
                .tab_index(3)
                .tab_stop(true)
        });

        let load_credentials_task = Some(cx.spawn({
            let state = state.clone();
            async move |this, cx| {
                if let Some(task) = Some(state.update(cx, |state, cx| state.authenticate(cx))) {
                    // We don't log an error, because "not signed in" is also an error.
                    let _ = task.await;
                }
                this.update(cx, |this, cx| {
                    this.load_credentials_task = None;
                    cx.notify();
                })
                .log_err();
            }
        }));

        Self {
            access_key_id_editor,
            secret_access_key_editor,
            session_token_editor,
            bearer_token_editor,
            state,
            load_credentials_task,
            focus_handle,
        }
    }

    fn save_credentials(
        &mut self,
        _: &menu::Confirm,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let access_key_id = self
            .access_key_id_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let secret_access_key = self
            .secret_access_key_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let session_token = self
            .session_token_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let session_token = if session_token.is_empty() {
            None
        } else {
            Some(session_token)
        };
        let bearer_token = self
            .bearer_token_editor
            .read(cx)
            .text(cx)
            .trim()
            .to_string();
        let bearer_token = if bearer_token.is_empty() {
            None
        } else {
            Some(bearer_token)
        };

        self.state
            .update(cx, |state, cx| {
                let credentials = BedrockCredentials {
                    access_key_id,
                    secret_access_key,
                    session_token,
                    bearer_token,
                };

                state.set_static_credentials(credentials, cx)
            })
            .detach_and_log_err(cx);
    }

    fn reset_credentials(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.access_key_id_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        self.secret_access_key_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        self.session_token_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
        self.bearer_token_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));

        self.state
            .update(cx, |state, cx| state.reset_auth(cx))
            .detach_and_log_err(cx);
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

impl Render for ConfigurationView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.state.read(cx);
        let env_var_set = state.credentials_from_env;
        let auth = state.auth.clone();
        let listing_message = state.listing_message();
        let capability_catalog_error = state.last_capability_catalog_error.clone();
        let settings_auth_method = state.settings.authentication_method.clone();

        if self.load_credentials_task.is_some() {
            return div().child(Label::new("Loading credentials...")).into_any();
        }

        let configured_label = match &auth {
            Some(BedrockAuth::Automatic) => {
                "Using automatic credentials (AWS default chain)".into()
            }
            Some(BedrockAuth::NamedProfile { profile_name }) => {
                format!("Using AWS profile: {profile_name}")
            }
            Some(BedrockAuth::SingleSignOn { profile_name }) => {
                format!("Using AWS SSO profile: {profile_name}")
            }
            Some(BedrockAuth::IamCredentials { .. }) if env_var_set => {
                format!(
                    "Using IAM credentials from {} and {} environment variables",
                    ZED_BEDROCK_ACCESS_KEY_ID_VAR.name, ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.name
                )
            }
            Some(BedrockAuth::IamCredentials { .. }) => "Using IAM credentials".into(),
            Some(BedrockAuth::ApiKey { .. }) if env_var_set => {
                format!(
                    "Using Bedrock API Key from {} environment variable",
                    ZED_BEDROCK_BEARER_TOKEN_VAR.name
                )
            }
            Some(BedrockAuth::ApiKey { .. }) => "Using Bedrock API Key".into(),
            None => "Not authenticated".into(),
        };

        // Determine if credentials can be reset
        // Settings-derived auth (non-ApiKey) cannot be reset from UI
        let is_settings_derived = matches!(
            settings_auth_method,
            Some(BedrockAuthMethod::Automatic)
                | Some(BedrockAuthMethod::NamedProfile)
                | Some(BedrockAuthMethod::SingleSignOn)
        );

        let tooltip_label = if env_var_set {
            Some(format!(
                "To reset your credentials, unset the {}, {}, and {} or {} environment variables.",
                ZED_BEDROCK_ACCESS_KEY_ID_VAR.name,
                ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.name,
                ZED_BEDROCK_SESSION_TOKEN_VAR.name,
                ZED_BEDROCK_BEARER_TOKEN_VAR.name
            ))
        } else if is_settings_derived {
            Some(
                "Authentication method is configured in settings. Edit settings.json to change."
                    .to_string(),
            )
        } else {
            None
        };

        let credentials_control = if self.state.read(cx).is_authenticated() {
            ConfiguredApiCard::new("bedrock-reset", configured_label)
                .disabled(env_var_set || is_settings_derived)
                .on_click(cx.listener(|this, _, window, cx| this.reset_credentials(window, cx)))
                .when_some(tooltip_label, |this, label| this.tooltip_label(label))
                .into_any_element()
        } else {
            self.render_static_credentials_ui().into_any_element()
        };

        v_flex()
            .min_w_0()
            .w_full()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_tab))
            .on_action(cx.listener(Self::on_tab_prev))
            .on_action(cx.listener(ConfigurationView::save_credentials))
            .gap_1()
            .child(Headline::new("Amazon Bedrock").size(HeadlineSize::Small))
            .child(
                Label::new(
                    "To use Zed's agent with Bedrock, you can set a custom authentication strategy through your settings file or use static credentials.",
                )
                .color(Color::Muted),
            )
            .child(
                Label::new("But first, to access models on AWS, you need to:")
                    .mt_1()
                    .color(Color::Muted),
            )
            .child(
                List::new()
                    .child(
                        ListBulletItem::new("")
                            .child(
                                Label::new(
                                    "Grant permissions to the strategy you'll use according to the:",
                                )
                                .color(Color::Muted),
                            )
                            .child(ButtonLink::new(
                                "Prerequisites",
                                "https://docs.aws.amazon.com/bedrock/latest/userguide/inference-prereq.html",
                            )),
                    )
                    .child(
                        ListBulletItem::new("")
                            .child(
                                Label::new("Select the models you would like access to:")
                                    .color(Color::Muted),
                            )
                            .child(ButtonLink::new(
                                "Bedrock Model Catalog",
                                "https://us-east-1.console.aws.amazon.com/bedrock/home?region=us-east-1#/model-catalog",
                            )),
                    ),
            )
            .child(credentials_control)
            .when_some(state.credential_delete_error.clone(), |this, message| {
                this.child(Label::new(message).color(Color::Error))
            })
            .when_some(listing_message, |this, (message, warning)| {
                this.child(
                    Label::new(message)
                    .size(LabelSize::Small)
                    .color(if warning { Color::Warning } else { Color::Muted }),
                )
            })
            .when_some(capability_catalog_error, |this, error| {
                this.child(
                    Label::new(format!(
                        "Could not load model limits and capabilities from models.dev: {error}"
                    ))
                    .size(LabelSize::Small)
                    .color(Color::Warning),
                )
            })
            .into_any()
    }
}

impl ConfigurationView {
    fn render_static_credentials_ui(&self) -> impl IntoElement {
        let list_item = List::new()
            .child(
                ListBulletItem::new("")
                    .child(
                        Label::new(
                            "For access keys: Create an IAM user in the AWS console with programmatic access",
                        )
                        .color(Color::Muted),
                    )
                    .child(ButtonLink::new(
                        "IAM Console",
                        "https://us-east-1.console.aws.amazon.com/iam/home?region=us-east-1#/users",
                    )),
            )
            .child(
                ListBulletItem::new("")
                    .child(
                        Label::new("For Bedrock API Keys: Generate an API key from the")
                            .color(Color::Muted),
                    )
                    .child(ButtonLink::new(
                        "Bedrock Console",
                        "https://docs.aws.amazon.com/bedrock/latest/userguide/api-keys-use.html",
                    )),
            )
            .child(
                ListBulletItem::new("")
                    .child(
                        Label::new("Attach the necessary Bedrock permissions to")
                            .color(Color::Muted),
                    )
                    .child(ButtonLink::new(
                        "this user",
                        "https://docs.aws.amazon.com/bedrock/latest/userguide/inference-prereq.html",
                    )),
            )
            .child(
                ListBulletItem::new(
                    "Enter either access keys OR a Bedrock API Key below (not both)",
                )
                .label_color(Color::Muted),
            );

        v_flex()
            .my_2()
            .tab_group()
            .gap_1p5()
            .child(Divider::horizontal())
            .child(Label::new("Static Credentials").mt_2())
            .child(
                Label::new(
                    "This method uses your AWS access key ID and secret access key, or a Bedrock API Key.",
                )
                .color(Color::Muted),
            )
            .child(list_item)
            .child(
                v_flex()
                    .gap_1()
                    .child(self.access_key_id_editor.clone())
                    .child(self.secret_access_key_editor.clone())
                    .child(self.session_token_editor.clone()),
            )
            .child(
                Label::new(format!(
                    "You can also set the {}, {} and {} environment variables (or {} for Bedrock API Key authentication) and restart Zed.",
                    ZED_BEDROCK_ACCESS_KEY_ID_VAR.name,
                    ZED_BEDROCK_SECRET_ACCESS_KEY_VAR.name,
                    ZED_BEDROCK_REGION_VAR.name,
                    ZED_BEDROCK_BEARER_TOKEN_VAR.name
                ))
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .child(
                Label::new(format!(
                    "Set profile and endpoint_url in settings.json for an AWS profile or custom Converse endpoint. For temporary credentials, set {}.",
                    ZED_BEDROCK_SESSION_TOKEN_VAR.name
                ))
                .size(LabelSize::Small)
                .color(Color::Muted)
                .mt_1()
                .mb_2p5(),
            )
            .child(Divider::horizontal())
            .child(Label::new("Using the API key").mt_2().mb_1())
            .child(self.bearer_token_editor.clone())
            .child(
                Label::new(format!(
                    "Region is configured via {} environment variable or settings.json (defaults to us-east-1).",
                    ZED_BEDROCK_REGION_VAR.name
                ))
                .size(LabelSize::Small)
                .color(Color::Muted)
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use language_model::LanguageModelRequestMessage;
    use open_ai::responses::{ResponseFunctionToolCall, ResponseOutputMessage};
    use std::path::Path;

    use bedrock::bedrock_control_plane_client::types::{
        FoundationModelLifecycle, FoundationModelLifecycleStatus, FoundationModelSummary,
        InferenceProfileModel, InferenceProfileStatus, InferenceProfileSummary,
        InferenceProfileType, ModelModality,
    };

    /// A Converse model served through a `us.` inference profile.
    fn converse_model(base_model_id: &str, thinking: Option<ConverseThinking>) -> ConverseModel {
        ConverseModel {
            id: base_model_id.to_string(),
            request_id: format!("us.{base_model_id}"),
            base_model_id: base_model_id.to_string(),
            foundation_model_known: true,
            route: ConverseModelRoute::Automatic,
            display_name: base_model_id.to_string(),
            max_tokens: 200_000,
            max_output_tokens: Some(128_000),
            default_temperature: Some(1.0),
            supports_temperature: !thinking
                .as_ref()
                .is_some_and(ConverseThinking::is_always_on),
            supports_tool_use: true,
            supports_images: true,
            supports_caching: true,
            thinking,
            thinking_disabled: false,
        }
    }

    fn adaptive_thinking(always_on: bool) -> ConverseThinking {
        ConverseThinking::Adaptive {
            efforts: vec![
                BedrockAdaptiveThinkingEffort::Low,
                BedrockAdaptiveThinkingEffort::High,
                BedrockAdaptiveThinkingEffort::Max,
            ],
            default_effort: BedrockAdaptiveThinkingEffort::High,
            always_on,
        }
    }

    fn application_profile(base: &str) -> Result<InferenceProfileSummary> {
        Ok(InferenceProfileSummary::builder()
            .inference_profile_id("app")
            .inference_profile_name("Private profile")
            .inference_profile_arn(
                "arn:aws:bedrock:us-east-1:123456789012:application-inference-profile/app",
            )
            .models(
                InferenceProfileModel::builder()
                    .model_arn(format!(
                        "arn:aws:bedrock:us-east-1::foundation-model/{base}"
                    ))
                    .build(),
            )
            .status(InferenceProfileStatus::Active)
            .r#type(InferenceProfileType::Application)
            .build()?)
    }

    #[gpui::test]
    fn application_profile_arn_reaches_request_but_not_telemetry(cx: &mut gpui::TestAppContext) {
        init_provider_test(cx);
        let result = cx.update(|cx| -> Result<()> {
            let provider = new_test_provider(http_client::FakeHttpClient::with_404_response(), cx);
            let base = "vendor.chat";
            let application = application_profile(base)?;
            let catalog = ModelCatalog::new(
                std::slice::from_ref(&application),
                &[listed_foundation_model(base)],
            );
            provider.state.update(cx, |state, _| {
                state.set_auth(api_key_auth());
                state.model_catalog = Some(catalog);
            });
            let model = provider
                .provided_models(cx)
                .into_iter()
                .next()
                .context("application profile")?;
            assert_eq!(model.telemetry_id.as_ref(), "bedrock/vendor.chat");
            let pinned = provider.state.read_with(cx, |state, _| {
                converse_model_from_settings(
                    &settings_entry(model.id.0.as_ref(), None),
                    state.model_catalog.as_ref(),
                    &state.capability_catalog,
                )
            });
            assert_eq!(pinned.request_id, model.id.0.as_ref());
            assert_eq!(
                converse_language_model(&pinned).telemetry_id.as_ref(),
                "bedrock/vendor.chat"
            );
            let BedrockModelConfig::Converse(config) = provider.config(&model, cx)? else {
                anyhow::bail!("application profile must use Converse")
            };
            for config in [config, pinned] {
                let request = into_bedrock(LanguageModelRequest::default(), &config, None, None)?;
                assert_eq!(request.model, application.inference_profile_arn());
            }

            for unknown_id in [
                "arn:aws:bedrock:us-east-1:123456789012:application-inference-profile/private",
                "vendor.secret",
            ] {
                let unknown = provider.state.read_with(cx, |state, _| {
                    converse_model_from_settings(
                        &settings_entry(unknown_id, None),
                        state.model_catalog.as_ref(),
                        &state.capability_catalog,
                    )
                });
                assert_eq!(
                    converse_language_model(&unknown).telemetry_id.as_ref(),
                    "bedrock/custom"
                );
            }
            let mut mantle = mantle_model(false);
            assert_eq!(
                mantle_language_model(&mantle).telemetry_id.as_ref(),
                "bedrock-mantle/custom"
            );
            mantle.known_foundation_model_id = Some(base.to_string());
            assert_eq!(
                mantle_language_model(&mantle).telemetry_id.as_ref(),
                "bedrock-mantle/vendor.chat"
            );
            Ok(())
        });
        assert!(result.is_ok(), "{result:?}");
    }

    #[gpui::test]
    fn refusal_fallback_is_offered_only_for_automatic_routes(cx: &mut gpui::TestAppContext) {
        init_provider_test(cx);
        let result = cx.update(|cx| -> Result<()> {
            let provider = new_test_provider(http_client::FakeHttpClient::with_404_response(), cx);
            let fable = "anthropic.claude-fable-5";
            let fallback = FABLE_FALLBACK_MODEL_ID;
            let profiles = [
                listed_inference_profile(&format!("us.{fable}"), fable),
                listed_inference_profile(&format!("us.{fallback}"), fallback),
                application_profile(fable)?,
            ];
            let foundations = [
                listed_foundation_model(fable),
                listed_foundation_model(fallback),
            ];
            provider.state.update(cx, |state, _| {
                state.model_catalog = Some(ModelCatalog::new(&profiles, &foundations));
                state.settings.available_models =
                    vec![settings_entry(&format!("us.{fable}"), None)];
            });
            for model in provider.provided_models(cx) {
                assert_eq!(
                    model.refusal_fallback_model_id,
                    (model.id.0.as_ref() == fable).then_some(fallback),
                    "{}",
                    model.id.0
                );
            }
            provider.state.update(cx, |state, _| {
                state
                    .settings
                    .available_models
                    .push(settings_entry(fallback, None))
            });
            assert!(
                provider
                    .provided_models(cx)
                    .iter()
                    .all(|model| model.refusal_fallback_model_id.is_none()),
                "a fallback pinned by settings is not an automatic route"
            );
            Ok(())
        });
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn thinking_budget_respects_effective_output_cap_and_sampling() -> Result<()> {
        for (budget, cap, expected) in [
            (4096, 8192, Some(4096)),
            (4096, 4096, Some(2048)),
            (4096, 2048, Some(1024)),
            (4096, 2047, None),
            (1023, 8192, None),
            (4096, 1024, None),
            (8192, 16384, Some(4096)),
        ] {
            let mut model = converse_model(
                "anthropic.claude-example",
                Some(ConverseThinking::Budget {
                    budget_tokens: budget,
                }),
            );
            model.max_output_tokens = Some(8192);
            let request = into_bedrock(
                LanguageModelRequest {
                    thinking_allowed: true,
                    max_output_tokens: Some(cap),
                    temperature: Some(0.3),
                    ..Default::default()
                },
                &model,
                None,
                None,
            )?;
            assert_eq!(request.max_tokens, Some(cap.min(8192)));
            assert_eq!(
                match request.thinking {
                    Some(bedrock::Thinking::Enabled { budget_tokens }) => budget_tokens,
                    None => None,
                    _ => anyhow::bail!("unexpected thinking mode"),
                },
                expected
            );
            assert_eq!(request.temperature, expected.is_none().then_some(0.3));
        }
        let custom = converse_model_from_settings(
            &settings_entry(
                "private",
                Some(settings::BedrockThinkingSettings::Enabled(true)),
            ),
            None,
            &CapabilityCatalog::default(),
        );
        let request = into_bedrock(
            LanguageModelRequest {
                thinking_allowed: true,
                ..Default::default()
            },
            &custom,
            None,
            None,
        )?;
        assert_eq!(request.max_tokens, Some(4096));
        assert!(matches!(
            request.thinking,
            Some(bedrock::Thinking::Enabled {
                budget_tokens: Some(2048)
            })
        ));
        assert_eq!(request.temperature, None);
        let adaptive = into_bedrock(
            LanguageModelRequest {
                thinking_allowed: true,
                temperature: Some(0.3),
                ..Default::default()
            },
            &converse_model("anthropic.claude-example", Some(adaptive_thinking(false))),
            None,
            None,
        )?;
        assert!(matches!(
            adaptive.thinking,
            Some(bedrock::Thinking::Adaptive { .. })
        ));
        assert_eq!(adaptive.temperature, None);
        Ok(())
    }

    #[test]
    fn incompatible_history_fails_before_content_is_encoded() -> Result<()> {
        let mut text_only = converse_model("vendor.text", None);
        text_only.supports_tool_use = false;
        text_only.supports_images = false;
        let mut empty = user_message("");
        empty.content = vec![MessageContent::ToolResult(
            language_model::LanguageModelToolResult {
                tool_use_id: "call".into(),
                tool_name: "tool".into(),
                is_error: false,
                content: vec![],
                output: None,
            },
        )];
        let mut system = user_message("keep instructions");
        system.role = Role::System;
        system
            .content
            .push(MessageContent::Image(language_model::LanguageModelImage {
                source: "AP8=".into(),
            }));
        let encoded = into_bedrock(
            LanguageModelRequest {
                messages: vec![system, empty, user_message("hello")],
                ..Default::default()
            },
            &text_only,
            None,
            None,
        )?;
        assert!(
            matches!(encoded.system.first(), Some(BedrockSystemContentBlock::Text(text)) if text == "keep instructions")
        );
        assert_eq!(encoded.messages.len(), 1);
        let invalid_limit = converse_model_from_settings(
            &AvailableModel {
                max_output_tokens: Some(0),
                ..settings_entry("vendor.manual", None)
            },
            None,
            &CapabilityCatalog::default(),
        );
        let error = into_bedrock(LanguageModelRequest::default(), &invalid_limit, None, None)
            .expect_err("invalid settings limit");
        assert!(
            error
                .to_string()
                .contains("`vendor.manual` sets max_output_tokens to 0")
        );
        let tool_result = MessageContent::ToolResult(language_model::LanguageModelToolResult {
            tool_use_id: "call".into(),
            tool_name: "tool".into(),
            is_error: false,
            content: vec![LanguageModelToolResultContent::Image(
                language_model::LanguageModelImage {
                    source: "AP8=".into(),
                },
            )],
            output: None,
        });
        let tool_use = MessageContent::ToolUse(LanguageModelToolUse {
            id: "call".into(),
            name: "tool".into(),
            is_input_complete: true,
            raw_input: "{}".into(),
            input: language_model::LanguageModelToolUseInput::Json(serde_json::json!({})),
            thought_signature: None,
        });
        for (content, tools, images, expected) in [
            (tool_use, false, true, "tool calls"),
            (tool_result.clone(), false, true, "tool calls"),
            (tool_result, true, false, "images"),
            (
                MessageContent::Image(language_model::LanguageModelImage {
                    source: "AP8=".into(),
                }),
                true,
                false,
                "images",
            ),
        ] {
            let mut model = converse_model("vendor.chat", None);
            model.supports_tool_use = tools;
            model.supports_images = images;
            let error = into_bedrock(
                LanguageModelRequest {
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::User,
                        content: vec![content],
                        cache: false,
                        reasoning_details: None,
                    }],
                    ..Default::default()
                },
                &model,
                None,
                None,
            )
            .expect_err("incompatible history");
            let error = error.to_string();
            assert!(
                error.contains(expected)
                    && error.contains("Choose another model or start a new thread"),
                "{error}"
            );
        }
        Ok(())
    }

    fn reasoning() -> ConverseThinking {
        ConverseThinking::Reasoning {
            efforts: vec![
                BedrockAdaptiveThinkingEffort::Low,
                BedrockAdaptiveThinkingEffort::Medium,
                BedrockAdaptiveThinkingEffort::High,
            ],
            default_effort: BedrockAdaptiveThinkingEffort::Medium,
            supports_none: false,
        }
    }

    fn user_message(text: &str) -> LanguageModelRequestMessage {
        LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text(text.into())],
            cache: false,
            reasoning_details: None,
        }
    }

    fn completion_events(
        model: &ConverseModel,
        events: impl Stream<Item = Result<BedrockStreamingResponse>> + Send + 'static,
    ) -> Result<Vec<LanguageModelCompletionEvent>> {
        futures::executor::block_on(
            map_to_language_model_completion_events(Box::pin(events), model.clone())
                .collect::<Vec<_>>(),
        )
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .map_err(Into::into)
    }

    fn replay_history(
        model: &ConverseModel,
        content: Vec<MessageContent>,
        reasoning_details: Option<Arc<Value>>,
    ) -> Result<bedrock::Request> {
        into_bedrock(
            LanguageModelRequest {
                messages: vec![LanguageModelRequestMessage {
                    role: Role::Assistant,
                    content,
                    cache: false,
                    reasoning_details,
                }],
                ..Default::default()
            },
            model,
            None,
            None,
        )
    }

    fn into_bedrock_request(messages: Vec<LanguageModelRequestMessage>) -> bedrock::Request {
        into_bedrock(
            LanguageModelRequest {
                messages,
                ..Default::default()
            },
            &converse_model("vendor.chat", None),
            None,
            None,
        )
        .unwrap()
    }

    fn listed_foundation_model(model_id: &str) -> FoundationModelSummary {
        FoundationModelSummary::builder()
            .model_arn(format!(
                "arn:aws:bedrock:us-east-1::foundation-model/{model_id}"
            ))
            .model_id(model_id)
            .model_name(format!("Name of {model_id}"))
            .input_modalities(ModelModality::Text)
            .output_modalities(ModelModality::Text)
            .response_streaming_supported(true)
            .model_lifecycle(
                FoundationModelLifecycle::builder()
                    .status(FoundationModelLifecycleStatus::Active)
                    .build()
                    .expect("the lifecycle has a status"),
            )
            .build()
            .expect("the summary has every required member")
    }

    fn listed_inference_profile(profile_id: &str, model_id: &str) -> InferenceProfileSummary {
        InferenceProfileSummary::builder()
            .inference_profile_id(profile_id)
            .inference_profile_arn(format!(
                "arn:aws:bedrock:us-east-1:123456789012:inference-profile/{profile_id}"
            ))
            .inference_profile_name(profile_id)
            .models(
                InferenceProfileModel::builder()
                    .model_arn(format!(
                        "arn:aws:bedrock:us-east-1::foundation-model/{model_id}"
                    ))
                    .build(),
            )
            .status(InferenceProfileStatus::Active)
            .r#type(InferenceProfileType::SystemDefined)
            .build()
            .expect("the summary has every required member")
    }

    const TEST_CACHE_PATH: &str = "/cache/bedrock/models.json";

    fn new_test_provider_with_fs(
        http_client: Arc<dyn HttpClient>,
        fs: Arc<dyn Fs>,
        cx: &mut App,
    ) -> BedrockLanguageModelProvider {
        BedrockLanguageModelProvider::new_with_cache(
            http_client,
            Arc::new(NoCredentialsProvider),
            fs,
            PathBuf::from(TEST_CACHE_PATH),
            cx,
        )
    }

    fn new_test_provider(
        http_client: Arc<dyn HttpClient>,
        cx: &mut App,
    ) -> BedrockLanguageModelProvider {
        let fs = fs::FakeFs::new(cx.background_executor().clone());
        new_test_provider_with_fs(http_client, fs, cx)
    }

    /// A capability catalog entry for a text model with tools.
    fn capability_entry(name: &str, members: Value) -> Value {
        let mut entry = serde_json::json!({
            "name": name,
            "tool_call": true,
            "modalities": {"input": ["text"], "output": ["text"]},
            "limit": {"context": 700_000, "output": 64_000},
        });
        if let (Some(entry), Some(members)) = (entry.as_object_mut(), members.as_object()) {
            entry.extend(members.clone());
        }
        entry
    }

    /// The catalog members of a model that reasons on every request.
    fn always_reasoning_members() -> Value {
        serde_json::json!({
            "reasoning": true,
            "reasoning_options": [{"type": "effort", "values": ["low", "high"]}],
        })
    }

    fn capability_catalog_json(models: Value) -> String {
        serde_json::json!({
            "amazon-bedrock": {"models": models},
            "another-provider": {"models": {}},
        })
        .to_string()
    }

    fn capability_catalog(models: Value) -> CapabilityCatalog {
        CapabilityCatalog::parse(&capability_catalog_json(models))
            .expect("the test catalog should parse")
    }

    fn api_key_auth() -> Option<BedrockAuth> {
        Some(BedrockAuth::ApiKey {
            api_key: "test-key".into(),
        })
    }

    fn init_provider_test(cx: &mut gpui::TestAppContext) {
        cx.executor().allow_parking();
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            gpui_tokio::init(cx);
        });
    }

    fn set_bedrock_settings(cx: &mut gpui::TestAppContext, settings: Value) -> Result<()> {
        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.set_user_settings(
                    &serde_json::json!({"language_models": {"bedrock": settings}}).to_string(),
                    cx,
                )
            })
        })
        .result()
        .map(|_| ())
    }

    fn init_provider_settings(cx: &mut gpui::TestAppContext, settings: Value) {
        init_provider_test(cx);
        set_bedrock_settings(cx, settings).expect("test settings should parse");
    }

    /// What the fake control plane answers: `None` denies `ListFoundationModels`,
    /// and `Some` lists that one model behind a `us.` inference profile.
    type ListingResponse = Arc<parking_lot::Mutex<Option<&'static str>>>;

    fn listing_client(response: &ListingResponse) -> Arc<dyn HttpClient> {
        let response = response.clone();
        http_client::FakeHttpClient::create(move |request| {
            let listed = *response.lock();
            async move {
                if request.uri() == CAPABILITY_CATALOG_URL {
                    return json_response(404, serde_json::json!({}));
                }
                let is_foundation_models = request.uri().path() == "/foundation-models";
                let Some(id) = listed else {
                    return if is_foundation_models {
                        json_response(
                            403,
                            serde_json::json!({"__type": "AccessDeniedException", "message": "test denial"}),
                        )
                    } else {
                        json_response(200, serde_json::json!({"inferenceProfileSummaries": []}))
                    };
                };
                let body = if is_foundation_models {
                    serde_json::json!({"modelSummaries": [{
                        "modelId": id,
                        "modelArn": format!("arn:aws:bedrock:us-east-1::foundation-model/{id}"),
                        "modelName": format!("Name of {id}"),
                        "inputModalities": ["TEXT"], "outputModalities": ["TEXT"],
                        "responseStreamingSupported": true,
                        "modelLifecycle": {"status": "ACTIVE"},
                    }]})
                } else if request
                    .uri()
                    .query()
                    .is_some_and(|query| query.contains("SYSTEM_DEFINED"))
                {
                    serde_json::json!({"inferenceProfileSummaries": [{
                        "inferenceProfileId": format!("us.{id}"),
                        "inferenceProfileArn": format!("arn:aws:bedrock:us-east-1:123456789012:inference-profile/us.{id}"),
                        "inferenceProfileName": id,
                        "status": "ACTIVE", "type": "SYSTEM_DEFINED",
                        "models": [{"modelArn": format!("arn:aws:bedrock:us-east-1::foundation-model/{id}")}],
                    }]})
                } else {
                    serde_json::json!({"inferenceProfileSummaries": []})
                };
                json_response(200, body)
            }
        })
    }

    fn json_response(status: u16, body: Value) -> Result<http_client::Response<AsyncBody>> {
        Ok(http_client::Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .body(AsyncBody::from(body.to_string()))?)
    }

    async fn finish_listing(
        provider: &BedrockLanguageModelProvider,
        cx: &mut gpui::TestAppContext,
    ) -> Result<()> {
        let task = provider
            .state
            .update(cx, |state, _| state.fetch_models_task.take())
            .context("listing task")?;
        task.await;
        Ok(())
    }

    fn offered_ids(
        provider: &BedrockLanguageModelProvider,
        cx: &mut gpui::TestAppContext,
    ) -> Vec<String> {
        cx.update(|cx| provider.provided_models(cx))
            .into_iter()
            .map(|model| model.id.0.to_string())
            .collect()
    }

    async fn send_test_completion(
        provider: &BedrockLanguageModelProvider,
        cx: &mut gpui::TestAppContext,
    ) -> Result<()> {
        let model = cx
            .update(|cx| provider.provided_models(cx))
            .into_iter()
            .find(|model| model.id.0.as_ref() == "vendor.manual")
            .context("manual test model")?;
        let error = provider
            .stream_completion(
                &model,
                LanguageModelRequest {
                    messages: vec![user_message("test prompt")],
                    ..Default::default()
                },
                &cx.to_async(),
            )
            .await
            .err()
            .context("controlled inference error")?;
        anyhow::ensure!(
            error.to_string().contains("test completion ended"),
            "{error}"
        );
        Ok(())
    }

    type RecordedRequests =
        Arc<parking_lot::Mutex<Vec<(String, http_client::http::HeaderMap, String)>>>;

    /// A client that records every request other than the capability catalog
    /// fetch, and answers each with the error `send_test_completion` expects.
    fn recording_client() -> (Arc<dyn HttpClient>, RecordedRequests) {
        let requests = RecordedRequests::default();
        let client = http_client::FakeHttpClient::create({
            let requests = requests.clone();
            move |mut request| {
                let requests = requests.clone();
                async move {
                    if request.uri() == CAPABILITY_CATALOG_URL {
                        return json_response(404, serde_json::json!({}));
                    }
                    let mut body = String::new();
                    request.body_mut().read_to_string(&mut body).await?;
                    requests.lock().push((
                        request.uri().to_string(),
                        request.headers().clone(),
                        body,
                    ));
                    json_response(
                        400,
                        serde_json::json!({"__type": "ValidationException", "message": "test completion ended"}),
                    )
                }
            }
        });
        (client, requests)
    }

    #[gpui::test]
    async fn region_setting_selects_the_runtime_host(cx: &mut gpui::TestAppContext) {
        let scenario = async {
            init_provider_test(cx);
            set_bedrock_settings(
                cx,
                serde_json::json!({
                    "region": "eu-west-1",
                    "available_models": [{"name": "vendor.manual", "max_tokens": 10000}],
                }),
            )?;
            let (client, requests) = recording_client();
            let provider = cx.update(|cx| new_test_provider(client, cx));
            provider
                .state
                .update(cx, |state, _| state.set_auth(api_key_auth()));
            send_test_completion(&provider, cx).await?;
            let requests = requests.lock();
            let (uri, _, _) = requests.last().context("inference request")?;
            assert!(
                uri.starts_with("https://bedrock-runtime.eu-west-1.amazonaws.com/"),
                "{uri}"
            );
            anyhow::Ok(())
        };
        scenario.await.expect("Region settings scenario");
    }

    #[gpui::test]
    async fn custom_endpoint_receives_requests_and_is_not_asked_for_a_listing(
        cx: &mut gpui::TestAppContext,
    ) {
        let scenario = async {
            init_provider_test(cx);
            let endpoint = "https://gateway.example.com/bedrock";
            set_bedrock_settings(
                cx,
                serde_json::json!({
                    "endpoint_url": endpoint, "custom_headers": {"x-test-route": "gateway"},
                    "available_models": [{"name": "vendor.manual", "max_tokens": 10000}],
                    "guardrail_identifier": "test-guardrail", "guardrail_version": "1",
                }),
            )?;
            let (client, requests) = recording_client();
            let provider = cx.update(|cx| new_test_provider(client, cx));
            provider
                .state
                .update(cx, |state, cx| state.change_auth(api_key_auth(), false, cx));
            cx.run_until_parked();
            let (message, _) = provider
                .state
                .read_with(cx, |state, _| state.listing_message())
                .context("gateway listing message")?;
            assert!(message.contains("custom endpoint"));
            send_test_completion(&provider, cx).await?;
            let requests = requests.lock();
            assert!(
                requests.iter().all(|(uri, _, _)| {
                    uri.starts_with(endpoint) && uri.contains("converse-stream")
                }),
                "only inference requests are sent, and only to the endpoint: {requests:?}"
            );
            let (_, headers, body) = requests.last().context("inference request")?;
            assert_eq!(
                headers.get("x-test-route").context("custom header")?,
                "gateway"
            );
            let body: Value = serde_json::from_str(body)?;
            assert_eq!(
                body["guardrailConfig"]["guardrailIdentifier"],
                "test-guardrail"
            );
            anyhow::Ok(())
        };
        scenario.await.expect("custom endpoint scenario");
    }

    #[gpui::test]
    async fn failed_listing_keeps_the_last_success_until_a_complete_replacement(
        cx: &mut gpui::TestAppContext,
    ) {
        let scenario = async {
            init_provider_test(cx);
            let response = ListingResponse::new(parking_lot::Mutex::new(Some("vendor.chat")));
            let provider = cx.update(|cx| new_test_provider(listing_client(&response), cx));
            provider
                .state
                .update(cx, |state, cx| state.change_auth(api_key_auth(), false, cx));
            finish_listing(&provider, cx).await?;
            assert_eq!(offered_ids(&provider, cx), ["vendor.chat"]);

            *response.lock() = None;
            cx.update(|cx| provider.refresh_models(cx));
            finish_listing(&provider, cx).await?;
            assert_eq!(offered_ids(&provider, cx), ["vendor.chat"]);
            let (message, warning) = provider
                .state
                .read_with(cx, |state, _| state.listing_message())
                .context("refresh warning")?;
            assert!(warning);
            assert!(
                message.contains("earlier account list")
                    && message.contains("bedrock:ListFoundationModels")
                    && message.contains("test denial"),
                "{message}"
            );

            *response.lock() = Some("vendor.next");
            cx.update(|cx| provider.refresh_models(cx));
            finish_listing(&provider, cx).await?;
            assert_eq!(offered_ids(&provider, cx), ["vendor.next"]);
            assert_eq!(
                provider
                    .state
                    .read_with(cx, |state, _| state.listing_message()),
                None
            );
            anyhow::Ok(())
        };
        scenario.await.expect("failed listing scenario");
    }

    #[gpui::test]
    async fn required_listing_failure_uses_public_models_only_after_the_failure(
        cx: &mut gpui::TestAppContext,
    ) {
        let scenario = async {
            init_provider_test(cx);
            let response = ListingResponse::default();
            let provider = cx.update(|cx| new_test_provider(listing_client(&response), cx));
            provider.state.update(cx, |state, _| {
                state.capability_catalog = capability_catalog(serde_json::json!({
                    "us.vendor.public": capability_entry("Public", serde_json::json!({})),
                }));
            });
            assert!(
                offered_ids(&provider, cx).is_empty(),
                "public cache is not offered before authentication"
            );
            provider
                .state
                .update(cx, |state, cx| state.change_auth(api_key_auth(), false, cx));
            assert!(offered_ids(&provider, cx).is_empty());
            finish_listing(&provider, cx).await?;
            assert_eq!(offered_ids(&provider, cx), ["vendor.public"]);
            let (message, warning) = provider
                .state
                .read_with(cx, |state, _| state.listing_message())
                .context("public list message")?;
            assert!(warning);
            assert!(
                message.contains("models.dev")
                    && message.contains("ListFoundationModels")
                    && message.contains("test denial"),
                "{message}"
            );

            *response.lock() = Some("vendor.listed");
            cx.update(|cx| provider.refresh_models(cx));
            assert_eq!(offered_ids(&provider, cx), ["vendor.public"]);
            finish_listing(&provider, cx).await?;
            assert_eq!(offered_ids(&provider, cx), ["vendor.listed"]);
            anyhow::Ok(())
        };
        scenario.await.expect("required listing failure scenario");
    }

    #[gpui::test]
    async fn signed_out_settings_models_do_not_resolve_credentials_or_send_requests(
        cx: &mut gpui::TestAppContext,
    ) {
        let scenario = async {
            init_provider_test(cx);
            set_bedrock_settings(
                cx,
                serde_json::json!({
                    "available_models": [{"name":"vendor.chat", "max_tokens":10000}],
                    "mantle_available_models": [{"name":"vendor.mantle", "max_tokens":10000, "protocol":"chat_completions"}]
                }),
            )?;
            let provider = cx.update(|cx| {
                new_test_provider(
                    http_client::FakeHttpClient::create(|_| async {
                        panic!("signed-out request reached HTTP")
                    }),
                    cx,
                )
            });
            provider
                .state
                .update(cx, |state, _| state.set_auth(api_key_auth()));
            let retained = cx.update(|cx| provider.provided_models(cx));
            provider
                .state
                .update(cx, |state, cx| state.reset_auth(cx))
                .await?;
            assert_eq!(retained.len(), 2);
            for model in retained {
                let error = provider
                    .stream_completion(
                        &model,
                        LanguageModelRequest {
                            messages: vec![user_message("hello")],
                            ..Default::default()
                        },
                        &cx.to_async(),
                    )
                    .await
                    .err()
                    .context("signed out")?;
                assert!(error.to_string().contains("not authenticated"), "{error}");
            }
            let result = provider
                .resolve_request_auth_with(
                    &cx.to_async(),
                    |_| panic!("read environment"),
                    |_, _| async { panic!("loaded AWS credentials") },
                )
                .await;
            assert!(result.is_err());
            anyhow::Ok(())
        };
        scenario.await.expect("signed-out settings models");
    }

    #[gpui::test]
    async fn failed_credential_delete_is_reported(cx: &mut gpui::TestAppContext) {
        init_provider_test(cx);
        let store =
            Arc::new(credentials_provider::test_support::FakeCredentialsProvider::default());
        let provider = cx.update(|cx| {
            BedrockLanguageModelProvider::new_with_cache(
                http_client::FakeHttpClient::with_404_response(),
                store.clone(),
                fs::FakeFs::new(cx.background_executor().clone()),
                PathBuf::from(TEST_CACHE_PATH),
                cx,
            )
        });
        provider
            .state
            .update(cx, |state, _| state.set_auth(api_key_auth()));
        let failed_delete = store.defer_next_mutation();
        let delete = provider.state.update(cx, |state, cx| state.reset_auth(cx));
        drop(failed_delete);
        let error = delete.await.expect_err("fake deletion failed");
        provider.state.read_with(cx, |state, _| {
            assert!(
                state
                    .credential_delete_error
                    .as_deref()
                    .is_some_and(|message| message.contains(&error.to_string()))
            );
            assert!(!state.is_authenticated());
        });
    }

    #[gpui::test]
    async fn live_authentication_settings_replace_auth_and_invalidate_cached_credentials(
        cx: &mut gpui::TestAppContext,
    ) {
        let scenario = async {
            init_provider_test(cx);
            set_bedrock_settings(
                cx,
                serde_json::json!({"endpoint_url": "https://gateway.example.com"}),
            )?;
            let provider = cx.update(|cx| {
                new_test_provider(http_client::FakeHttpClient::with_404_response(), cx)
            });
            provider
                .state
                .update(cx, |state, cx| state.change_auth(api_key_auth(), false, cx));
            for (method, profile, expected) in [
                (
                    "named_profile",
                    "one",
                    BedrockAuth::NamedProfile {
                        profile_name: "one".into(),
                    },
                ),
                (
                    "named_profile",
                    "two",
                    BedrockAuth::NamedProfile {
                        profile_name: "two".into(),
                    },
                ),
                (
                    "sso",
                    "three",
                    BedrockAuth::SingleSignOn {
                        profile_name: "three".into(),
                    },
                ),
                ("default", "four", BedrockAuth::Automatic),
            ] {
                provider.state.update(cx, |state, _| {
                    state.expiring_credentials = Some(ExpiringCredentials {
                        auth: state.auth.clone(),
                        region: state.get_region(),
                        credentials: Credentials::new(
                            "cached",
                            "secret",
                            None,
                            Some(SystemTime::now() + Duration::from_secs(3600)),
                            "test",
                        ),
                    });
                });
                set_bedrock_settings(
                    cx,
                    serde_json::json!({
                        "endpoint_url": "https://gateway.example.com", "authentication_method": method, "profile": profile,
                    }),
                )?;
                cx.run_until_parked();
                provider.state.read_with(cx, |state, _| {
                    assert_eq!(state.auth.as_ref(), Some(&expected));
                    assert!(state.expiring_credentials.is_none());
                });
            }
            Ok::<_, anyhow::Error>(())
        };
        scenario
            .await
            .expect("authentication settings scenario should complete");
    }

    #[gpui::test]
    fn listed_models_are_offered_and_refined_by_settings(cx: &mut gpui::TestAppContext) {
        init_provider_settings(
            cx,
            serde_json::json!({
                "allow_global": true,
                "available_models": [{"name": "us.anthropic.claude-sonnet-9", "max_tokens": 500000}],
            }),
        );
        let provider =
            cx.update(|cx| new_test_provider(http_client::FakeHttpClient::with_404_response(), cx));
        let model_catalog = ModelCatalog::new(
            &[
                listed_inference_profile(
                    "us.anthropic.claude-sonnet-9",
                    "anthropic.claude-sonnet-9",
                ),
                listed_inference_profile(
                    "global.anthropic.claude-sonnet-9",
                    "anthropic.claude-sonnet-9",
                ),
                listed_inference_profile("us.anthropic.claude-haiku-9", "anthropic.claude-haiku-9"),
            ],
            &[
                listed_foundation_model("anthropic.claude-sonnet-9"),
                listed_foundation_model("anthropic.claude-haiku-9"),
            ],
        );
        provider.state.update(cx, |state, _| {
            state.model_catalog = Some(model_catalog);
            state.capability_catalog = capability_catalog(serde_json::json!({
                "global.anthropic.claude-sonnet-9":
                    capability_entry("Sonnet 9 (Global)", always_reasoning_members()),
            }));
        });

        let models = cx.update(|cx| provider.provided_models(cx));
        let [uncataloged, listed, configured] = models.as_slice() else {
            panic!("expected three models, got {}", models.len());
        };
        assert_eq!(uncataloged.id.0.as_ref(), "anthropic.claude-haiku-9");
        assert!(
            !uncataloged.supports_thinking,
            "a model the capability catalog does not know has no reasoning controls"
        );
        assert_eq!(listed.id.0.as_ref(), "anthropic.claude-sonnet-9");
        assert_eq!(listed.max_token_count(), 700_000);
        assert!(listed.supports_thinking);
        assert!(!listed.supports_disabling_thinking);

        let converse_config = |model: &LanguageModel, cx: &mut gpui::TestAppContext| match cx
            .update(|cx| provider.config(model, cx))
            .expect("the offered model should resolve")
        {
            BedrockModelConfig::Converse(model) => model,
            BedrockModelConfig::Mantle(_) => panic!("expected a converse model"),
        };
        assert_eq!(
            converse_config(listed, cx).request_id,
            "global.anthropic.claude-sonnet-9",
            "`allow_global` routes a listed model through its global profile"
        );

        assert_eq!(configured.id.0.as_ref(), "us.anthropic.claude-sonnet-9");
        assert_eq!(configured.max_token_count(), 500_000);
        assert_eq!(
            configured.name.0.as_ref(),
            "Name of anthropic.claude-sonnet-9"
        );
        assert!(
            configured.supports_thinking,
            "a settings entry takes what it leaves unsaid from the listed model"
        );
        let configured = converse_config(configured, cx);
        assert_eq!(
            configured.request_id, "us.anthropic.claude-sonnet-9",
            "a settings entry is always invoked by its own name"
        );
        assert_eq!(configured.base_model_id, "anthropic.claude-sonnet-9");
        assert_eq!(configured.max_output_tokens, Some(64_000));
    }

    #[gpui::test]
    async fn capability_catalog_is_fetched_after_sign_in_and_cached(cx: &mut gpui::TestAppContext) {
        init_provider_settings(
            cx,
            serde_json::json!({"endpoint_url": "https://gateway.example.com/bedrock"}),
        );
        let catalog_json = capability_catalog_json(serde_json::json!({
            "us.vendor.chat": capability_entry("Chat (US)", serde_json::json!({})),
        }));
        let http_client = http_client::FakeHttpClient::create(move |request| {
            let catalog_json = catalog_json.clone();
            async move {
                assert_eq!(request.uri(), CAPABILITY_CATALOG_URL);
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(catalog_json))?)
            }
        });
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        let provider = cx.update(|cx| new_test_provider_with_fs(http_client, fs.clone(), cx));

        cx.run_until_parked();
        assert!(
            provider
                .state
                .read_with(cx, |state, _| state.capability_catalog.is_empty()),
            "nothing is fetched before authentication"
        );

        provider
            .state
            .update(cx, |state, cx| state.change_auth(api_key_auth(), false, cx));
        cx.run_until_parked();
        assert_eq!(offered_ids(&provider, cx), ["vendor.chat"]);
        provider.state.read_with(cx, |state, _| {
            assert_eq!(state.last_capability_catalog_error, None)
        });
        let cached = fs
            .load(Path::new(TEST_CACHE_PATH))
            .await
            .expect("the catalog should be cached");
        assert!(cached.contains("amazon-bedrock") && !cached.contains("another-provider"));
    }

    #[gpui::test]
    async fn stale_cached_capability_catalog_is_used_when_the_fetch_fails(
        cx: &mut gpui::TestAppContext,
    ) {
        init_provider_settings(
            cx,
            serde_json::json!({"endpoint_url": "https://gateway.example.com/bedrock"}),
        );
        let fs = fs::FakeFs::new(cx.background_executor.clone());
        fs.create_dir(Path::new("/cache/bedrock"))
            .await
            .expect("create the cache directory");
        fs.set_next_mtime(SystemTime::now() - Duration::from_secs(10 * 60));
        fs.insert_file(
            TEST_CACHE_PATH,
            capability_catalog_json(serde_json::json!({
                "us.vendor.chat": capability_entry("Chat (US)", serde_json::json!({})),
            }))
            .into_bytes(),
        )
        .await;
        let provider = cx.update(|cx| {
            new_test_provider_with_fs(
                http_client::FakeHttpClient::with_404_response(),
                fs.clone(),
                cx,
            )
        });
        provider
            .state
            .update(cx, |state, cx| state.change_auth(api_key_auth(), false, cx));
        cx.run_until_parked();
        assert_eq!(offered_ids(&provider, cx), ["vendor.chat"]);
        provider.state.read_with(cx, |state, _| {
            assert!(
                state
                    .last_capability_catalog_error
                    .as_deref()
                    .is_some_and(|error| error.contains("404"))
            );
        });
    }

    fn settings_entry(
        name: &str,
        thinking: Option<settings::BedrockThinkingSettings>,
    ) -> AvailableModel {
        AvailableModel {
            name: name.to_string(),
            display_name: None,
            max_tokens: 1_000_000,
            cache_configuration: None,
            max_output_tokens: None,
            default_temperature: Some(0.5),
            supports_tools: None,
            supports_images: None,
            thinking,
        }
    }

    #[test]
    fn unknown_settings_entry_has_only_the_capabilities_it_states() {
        let capability_catalog = CapabilityCatalog::default();
        let unstated = converse_model_from_settings(
            &settings_entry("us.vendor.chat", None),
            None,
            &capability_catalog,
        );
        assert_eq!(unstated.base_model_id, "vendor.chat");
        assert_eq!(unstated.thinking, None);
        assert!(unstated.supports_temperature);
        assert_eq!(unstated.default_temperature, Some(0.5));
        assert_eq!(unstated.max_output_tokens, Some(4_096));
        assert!(!unstated.supports_tool_use && !unstated.supports_images);
        assert!(!unstated.supports_caching);

        let without_temperature = converse_model_from_settings(
            &AvailableModel {
                default_temperature: None,
                ..settings_entry("vendor.chat", None)
            },
            None,
            &capability_catalog,
        );
        assert!(!without_temperature.supports_temperature);

        let stated = converse_model_from_settings(
            &AvailableModel {
                supports_tools: Some(true),
                max_output_tokens: Some(32_000),
                ..settings_entry(
                    "us.vendor.chat",
                    Some(settings::BedrockThinkingSettings::Config(
                        settings::BedrockThinkingConfigSettings {
                            adaptive: Some(true),
                            has_xhigh: None,
                            budget_tokens: None,
                        },
                    )),
                )
            },
            None,
            &capability_catalog,
        );
        assert_eq!(
            stated.thinking,
            Some(ConverseThinking::adaptive(false, false))
        );
        assert_eq!(stated.max_output_tokens, Some(32_000));
        assert!(stated.supports_tool_use);

        let thinking = |setting| {
            let setting = serde_json::from_value(setting).expect("thinking settings");
            converse_model_from_settings(
                &settings_entry("us.vendor.chat", Some(setting)),
                None,
                &capability_catalog,
            )
            .thinking
        };
        assert!(matches!(
            thinking(serde_json::json!({"adaptive": true, "has_xhigh": true})),
            Some(ConverseThinking::Adaptive { efforts, always_on: false, .. })
                if efforts.contains(&BedrockAdaptiveThinkingEffort::XHigh)
        ));
        assert_eq!(thinking(serde_json::json!(false)), None);
    }

    #[test]
    fn known_settings_entry_takes_what_it_leaves_unsaid_from_the_catalog() {
        let capability_catalog = capability_catalog(serde_json::json!({
            "us.vendor.chat": capability_entry("Chat (US)", serde_json::json!({
                "modalities": {"input": ["text", "image"], "output": ["text"]},
            })),
        }));
        let model = converse_model_from_settings(
            &settings_entry("vendor.chat", None),
            None,
            &capability_catalog,
        );

        assert_eq!(model.request_id, "vendor.chat");
        assert_eq!(model.display_name, "Chat (US)");
        assert_eq!(
            model.max_tokens, 1_000_000,
            "the entry states its context window"
        );
        assert_eq!(model.max_output_tokens, Some(64_000));
        assert!(model.supports_tool_use && model.supports_images);
    }

    #[gpui::test]
    fn mantle_model_replaces_converse_model_with_the_same_id(cx: &mut gpui::TestAppContext) {
        init_provider_settings(
            cx,
            serde_json::json!({
                "available_models": [{"name": "shared-model", "display_name": "Converse Shared", "max_tokens": 1000}],
                "mantle_available_models": [{"name": "shared-model", "display_name": "Mantle Shared", "max_tokens": 2000, "protocol": "chat_completions"}],
            }),
        );
        let provider =
            cx.update(|cx| new_test_provider(http_client::FakeHttpClient::with_404_response(), cx));

        let shared = cx
            .update(|cx| provider.provided_models(cx))
            .into_iter()
            .filter(|model| model.id.0.as_ref() == "shared-model")
            .collect::<Vec<_>>();
        assert_eq!(shared.len(), 1, "each id should be offered once");
        assert_eq!(shared[0].name.0.as_ref(), "Mantle Shared");

        let config = cx
            .update(|cx| provider.config(&shared[0], cx))
            .expect("the offered model should resolve");
        assert!(matches!(
            config,
            BedrockModelConfig::Mantle(model) if model.display_name() == "Mantle Shared"
        ));
    }

    #[gpui::test]
    fn custom_converse_model_capabilities_come_from_settings(cx: &mut gpui::TestAppContext) {
        init_provider_settings(
            cx,
            serde_json::json!({
                "available_models": [{
                    "name": "us.anthropic.claude-custom-v1:0", "display_name": "Custom Claude",
                    "max_tokens": 200000, "supports_tools": true, "supports_images": true,
                    "thinking": {"budget_tokens": 8192}
                }],
            }),
        );
        let provider =
            cx.update(|cx| new_test_provider(http_client::FakeHttpClient::with_404_response(), cx));

        let custom = cx
            .update(|cx| provider.provided_models(cx))
            .into_iter()
            .find(|model| model.id.0.as_ref() == "us.anthropic.claude-custom-v1:0")
            .expect("custom converse model should be offered");
        assert!(custom.supports_tools);
        assert!(custom.supports_images);
        assert!(custom.supports_thinking);

        let config = cx
            .update(|cx| provider.config(&custom, cx))
            .expect("the offered model should resolve");
        match config {
            BedrockModelConfig::Converse(model) => {
                assert!(model.supports_tool_use);
                assert!(model.supports_images);
                assert_eq!(
                    model.thinking,
                    Some(ConverseThinking::Budget {
                        budget_tokens: 8192
                    })
                );
            }
            BedrockModelConfig::Mantle(_) => panic!("expected a converse model"),
        }
    }

    #[test]
    fn environment_bearer_token_prefers_the_bedrock_variable_like_the_aws_sdk() {
        let environment = |variables: &[(&str, &str)]| {
            let variables: HashMap<String, String> = variables
                .iter()
                .map(|(name, value)| (name.to_string(), value.to_string()))
                .collect();
            move |name: &str| variables.get(name).cloned()
        };

        assert_eq!(environment_bearer_token(environment(&[])), None);
        assert_eq!(
            environment_bearer_token(environment(&[("AWS_BEARER_TOKEN", "generic")])),
            Some("generic".to_string())
        );
        assert_eq!(
            environment_bearer_token(environment(&[
                ("AWS_BEARER_TOKEN", "generic"),
                ("AWS_BEARER_TOKEN_BEDROCK", "bedrock"),
            ])),
            Some("bedrock".to_string())
        );
        assert_eq!(
            environment_bearer_token(environment(&[
                ("AWS_BEARER_TOKEN", "generic"),
                ("AWS_BEARER_TOKEN_BEDROCK", ""),
            ])),
            Some(String::new()),
            "a set but empty Bedrock variable still wins, as in the SDK"
        );
    }

    #[gpui::test]
    async fn expiring_credentials_are_cached_per_auth_until_near_expiry(
        cx: &mut gpui::TestAppContext,
    ) {
        let provider = test_provider(cx);
        let auth = Some(BedrockAuth::NamedProfile {
            profile_name: "work".into(),
        });
        let other_auth = Some(BedrockAuth::NamedProfile {
            profile_name: "other".into(),
        });
        provider
            .state
            .update(cx, |state, _| state.set_auth(auth.clone()));

        // Resolved credentials that expire are cached and reused.
        let resolved = provider
            .resolve_request_auth_with(&cx.to_async(), no_environment, |_, _| {
                resolved_credentials("first", Some(Duration::from_secs(60 * 60)))
            })
            .await;
        assert_eq!(access_key_id(resolved), "first");
        let reused = provider
            .resolve_request_auth_with(&cx.to_async(), no_environment, |_, _| {
                resolved_credentials("second", None)
            })
            .await;
        assert_eq!(
            access_key_id(reused),
            "first",
            "unexpired credentials are reused"
        );

        // Credentials close to expiry, or cached for other auth, are replaced.
        for (cached_auth, expires_in, case) in [
            (&auth, Duration::from_secs(60), "near expiry"),
            (&other_auth, Duration::from_secs(60 * 60), "different auth"),
        ] {
            provider.state.update(cx, |state, _| {
                state.expiring_credentials = Some(ExpiringCredentials {
                    auth: cached_auth.clone(),
                    region: state.get_region(),
                    credentials: Credentials::new(
                        "cached",
                        "secret",
                        None,
                        Some(SystemTime::now() + expires_in),
                        "test",
                    ),
                });
            });
            let resolved = provider
                .resolve_request_auth_with(&cx.to_async(), no_environment, |_, _| {
                    resolved_credentials("fresh", None)
                })
                .await;
            assert_eq!(access_key_id(resolved), "fresh", "{case}");
        }

        // Credentials without an expiry are never cached.
        provider
            .state
            .update(cx, |state, _| state.expiring_credentials = None);
        provider
            .resolve_request_auth_with(&cx.to_async(), no_environment, |_, _| {
                resolved_credentials("static", None)
            })
            .await
            .expect("auth should resolve");
        assert!(
            provider
                .state
                .read_with(cx, |state, _| state.expiring_credentials.is_none()),
            "credentials without an expiry should not be cached"
        );

        // A resolution that completes after an auth change, even back to the
        // same auth, is not cached.
        let (send_credentials, receive_credentials) = futures::channel::oneshot::channel();
        let pending =
            provider.resolve_request_auth_with(&cx.to_async(), no_environment, |_, _| async move {
                receive_credentials
                    .await
                    .expect("the test sends credentials")
            });
        provider
            .state
            .update(cx, |state, _| state.set_auth(auth.clone()));
        assert!(
            send_credentials
                .send(resolved_credentials("stale", Some(Duration::from_secs(60 * 60))).await)
                .is_ok(),
            "the pending resolution should still be waiting"
        );
        pending.await.expect("auth should resolve");
        assert!(
            provider
                .state
                .read_with(cx, |state, _| state.expiring_credentials.is_none()),
            "credentials resolved for earlier auth should not be cached"
        );
    }

    #[gpui::test]
    async fn resolving_request_auth_prefers_the_environment_token_and_times_out(
        cx: &mut gpui::TestAppContext,
    ) {
        let provider = test_provider(cx);
        provider.state.update(cx, |state, _| {
            state.set_auth(Some(BedrockAuth::Automatic));
        });

        let request_auth = provider
            .resolve_request_auth_with(
                &cx.to_async(),
                |name| (name == "AWS_BEARER_TOKEN").then(|| "token".to_string()),
                |_, _| async { panic!("credentials should not be resolved") },
            )
            .await
            .expect("auth should resolve");
        assert!(matches!(
            request_auth,
            BedrockRequestAuth::ApiKey { api_key } if api_key == "token"
        ));

        // An API key configured in Zed takes precedence over the environment.
        provider.state.update(cx, |state, _| {
            state.set_auth(Some(BedrockAuth::ApiKey {
                api_key: "configured".into(),
            }));
        });
        let http_client = provider.http_client.clone();
        let request_auth = provider
            .resolve_request_auth_with(
                &cx.to_async(),
                |name| (name == "AWS_BEARER_TOKEN").then(|| "token".to_string()),
                move |auth, region| resolve_request_auth(http_client, auth, region),
            )
            .await
            .expect("auth should resolve");
        assert!(matches!(
            request_auth,
            BedrockRequestAuth::ApiKey { api_key } if api_key == "configured"
        ));

        let pending = provider.resolve_request_auth_with(&cx.to_async(), no_environment, |_, _| {
            futures::future::pending()
        });
        cx.executor().advance_clock(CREDENTIALS_LOAD_TIMEOUT);
        let error = pending.await.err().expect("resolution should time out");
        assert!(error.to_string().contains("timed out"), "{error}");
    }

    fn assert_request_thinking(request: &bedrock::Request, expected: Option<&bedrock::Thinking>) {
        use bedrock::Thinking;
        let equal = match (&request.thinking, expected) {
            (None, None)
            | (Some(Thinking::Disabled), Some(Thinking::Disabled))
            | (Some(Thinking::BetweenTools), Some(Thinking::BetweenTools)) => true,
            (
                Some(Thinking::Enabled {
                    budget_tokens: actual,
                }),
                Some(Thinking::Enabled {
                    budget_tokens: expected,
                }),
            ) => actual == expected,
            (
                Some(Thinking::Reasoning { effort: actual }),
                Some(Thinking::Reasoning { effort: expected }),
            ) => actual == expected,
            (
                Some(Thinking::Adaptive {
                    effort: actual,
                    binding_controls_beta: actual_beta,
                }),
                Some(Thinking::Adaptive {
                    effort: expected,
                    binding_controls_beta: expected_beta,
                }),
            ) => actual == expected && actual_beta == expected_beta,
            _ => false,
        };
        assert!(
            equal,
            "{}: expected {expected:?}, got {:?}",
            request.model, request.thinking
        );
    }

    #[test]
    fn test_always_reasoning_models_keep_reasoning_when_thinking_is_disallowed() -> Result<()> {
        let adaptive_high = |binding_controls_beta| bedrock::Thinking::Adaptive {
            effort: BedrockAdaptiveThinkingEffort::High,
            binding_controls_beta,
        };
        for (model, expected_thinking) in [
            (
                converse_model("openai.gpt-example", Some(reasoning())),
                bedrock::Thinking::Reasoning {
                    effort: Some(BedrockAdaptiveThinkingEffort::Medium),
                },
            ),
            (
                converse_model("anthropic.claude-example", Some(adaptive_thinking(true))),
                adaptive_high(None),
            ),
            (
                converse_model("anthropic.claude-fable-5-1", Some(adaptive_thinking(true))),
                adaptive_high(
                    anthropic::binds_thinking_blocks_to_prefix("claude-fable-5-1")
                        .then(|| anthropic::THINKING_BINDING_CONTROLS_BETA_HEADER.to_string()),
                ),
            ),
        ] {
            for thinking_allowed in [true, false] {
                let request = into_bedrock(
                    LanguageModelRequest {
                        thinking_allowed,
                        ..Default::default()
                    },
                    &model,
                    None,
                    None,
                )?;
                assert_request_thinking(&request, Some(&expected_thinking));
                assert_eq!(request.temperature, None);
            }
            assert!(!converse_language_model(&model).supports_disabling_thinking);
        }
        let optional = converse_model("anthropic.claude-example", Some(adaptive_thinking(false)));
        assert!(converse_language_model(&optional).supports_disabling_thinking);
        Ok(())
    }

    #[test]
    fn catalog_thinking_controls_reach_converse_requests_from_settings() -> Result<()> {
        use BedrockAdaptiveThinkingEffort::{High, Low};
        use bedrock::Thinking;
        use serde_json::json;

        let options = json!({"reasoning": true, "reasoning_options": [
            {"type":"budget_tokens"}, {"type":"toggle"}, {"type":"effort", "values":["none", "low", "high"]}
        ]});
        let catalog = capability_catalog(json!({
            "us.openai.example": capability_entry("Optional", options.clone()),
            "us.anthropic.example": capability_entry("Adaptive", options.clone()),
            "us.anthropic.claude-opus-4-5-example": capability_entry("Budget", options),
            "us.xai.example": capability_entry("Always", always_reasoning_members()),
            "us.anthropic.mandatory": capability_entry("Mandatory", always_reasoning_members()),
        }));
        let adaptive_high = || Thinking::Adaptive {
            effort: High,
            binding_controls_beta: None,
        };
        for (id, setting, thinking_allowed, effort, expected, supports_disabling_thinking) in [
            (
                "openai.example",
                None,
                false,
                "high",
                Thinking::Reasoning { effort: None },
                true,
            ),
            (
                "openai.example",
                Some(json!(false)),
                true,
                "high",
                Thinking::Reasoning { effort: None },
                true,
            ),
            (
                "openai.example",
                Some(json!(true)),
                true,
                "max",
                Thinking::Reasoning { effort: Some(Low) },
                true,
            ),
            (
                "xai.example",
                Some(json!(false)),
                false,
                "high",
                Thinking::Reasoning { effort: Some(Low) },
                false,
            ),
            (
                "anthropic.example",
                Some(json!({"adaptive":true, "has_xhigh":true})),
                true,
                "max",
                adaptive_high(),
                true,
            ),
            (
                "anthropic.claude-opus-4-5-example",
                Some(json!({"budget_tokens":5000})),
                true,
                "low",
                Thinking::Enabled {
                    budget_tokens: Some(5000),
                },
                true,
            ),
            (
                "anthropic.mandatory",
                Some(json!(false)),
                false,
                "high",
                adaptive_high(),
                false,
            ),
        ] {
            let model = converse_model_from_settings(
                &settings_entry(id, setting.map(serde_json::from_value).transpose()?),
                None,
                &catalog,
            );
            assert_eq!(model.request_id, id);
            let language_model = converse_language_model(&model);
            assert_eq!(
                language_model.supports_disabling_thinking, supports_disabling_thinking,
                "{id}"
            );
            assert!(
                language_model
                    .supported_effort_levels
                    .iter()
                    .all(|level| level.value.as_ref() != "none")
            );
            let request = into_bedrock(
                LanguageModelRequest {
                    messages: vec![user_message("Hello")],
                    thinking_allowed,
                    thinking_effort: Some(effort.into()),
                    ..Default::default()
                },
                &model,
                None,
                None,
            )?;
            assert_request_thinking(&request, Some(&expected));
        }
        Ok(())
    }

    #[test]
    fn test_effort_levels_come_from_the_model() {
        let effort_levels = |thinking| {
            converse_language_model(&converse_model("vendor.chat", thinking))
                .supported_effort_levels
                .iter()
                .map(|level| (level.value.to_string(), level.is_default))
                .collect::<Vec<_>>()
        };

        assert_eq!(
            effort_levels(Some(adaptive_thinking(false))),
            [
                ("low".to_string(), false),
                ("high".to_string(), true),
                ("max".to_string(), false),
            ]
        );
        assert_eq!(
            effort_levels(Some(reasoning())),
            [
                ("low".to_string(), false),
                ("medium".to_string(), true),
                ("high".to_string(), false),
            ]
        );
        assert_eq!(
            effort_levels(Some(ConverseThinking::Budget {
                budget_tokens: 4_096
            })),
            []
        );
        assert_eq!(effort_levels(None), []);
    }

    #[test]
    fn reasoning_is_replayed_only_to_the_model_that_produced_it() -> Result<()> {
        use bedrock::bedrock_client::types::{ContentBlockDeltaEvent, ContentBlockStopEvent};

        let reasoning_delta = |delta| -> Result<Result<BedrockStreamingResponse>> {
            Ok(Ok(ConverseStreamOutput::ContentBlockDelta(
                ContentBlockDeltaEvent::builder()
                    .content_block_index(0)
                    .delta(ContentBlockDelta::ReasoningContent(delta))
                    .build()?,
            )))
        };
        let reasoning_details = |events: &[LanguageModelCompletionEvent]| {
            events.iter().find_map(|event| match event {
                LanguageModelCompletionEvent::ReasoningDetails(value) => {
                    Some(Arc::new(value.clone()))
                }
                _ => None,
            })
        };

        let claude = converse_model("anthropic.claude-example", Some(adaptive_thinking(true)));
        let events = completion_events(
            &claude,
            futures::stream::iter([reasoning_delta(ReasoningContentBlockDelta::Signature(
                "signature".into(),
            ))?]),
        )?;
        let details = reasoning_details(&events);
        assert!(details.is_some());
        let signature = events
            .iter()
            .find_map(|event| match event {
                LanguageModelCompletionEvent::Thinking { signature, .. } => signature.clone(),
                _ => None,
            })
            .context("streamed signature")?;
        assert_eq!(signature, "signature");

        for (model, replays_reasoning) in [
            (claude.clone(), true),
            (
                converse_model("anthropic.claude-other", Some(adaptive_thinking(true))),
                false,
            ),
            (
                converse_model("anthropic.claude-older", Some(adaptive_thinking(false))),
                false,
            ),
        ] {
            let request = replay_history(
                &model,
                vec![
                    MessageContent::Thinking {
                        text: "thinking".into(),
                        signature: Some(signature.clone()),
                    },
                    MessageContent::Text("answer".into()),
                ],
                details.clone(),
            )?;
            assert_eq!(
                matches!(
                    request.messages[0].content().first(),
                    Some(BedrockInnerContent::ReasoningContent(BedrockThinkingBlock::ReasoningText(thinking))) if thinking.text() == "thinking" && thinking.signature() == Some("signature")
                ),
                replays_reasoning,
                "{}",
                model.id
            );
        }

        let openai = converse_model("openai.chat", Some(reasoning()));
        let events = completion_events(
            &openai,
            futures::stream::iter([
                reasoning_delta(ReasoningContentBlockDelta::RedactedContent(
                    BedrockBlob::new([0]),
                ))?,
                reasoning_delta(ReasoningContentBlockDelta::RedactedContent(
                    BedrockBlob::new([255]),
                ))?,
                Ok(ConverseStreamOutput::ContentBlockStop(
                    ContentBlockStopEvent::builder()
                        .content_block_index(0)
                        .build()?,
                )),
            ]),
        )?;
        let openai_details = reasoning_details(&events).context("reasoning source")?;
        assert_eq!(
            *openai_details,
            serde_json::json!({"bedrock_model": "openai.chat"})
        );
        let data = events
            .iter()
            .find_map(|event| match event {
                LanguageModelCompletionEvent::RedactedThinking { data } => Some(data.clone()),
                _ => None,
            })
            .context("opaque reasoning")?;

        // A route whose foundation model is unknown is identified by a hash,
        // so its name is not stored in the thread.
        let unknown = |id| {
            converse_model_from_settings(
                &settings_entry(id, None),
                None,
                &CapabilityCatalog::default(),
            )
        };
        let private = unknown("team-a/chat");
        let private_source = reasoning_source(&private);
        assert!(!private_source.contains("team-a") && !private_source.contains("chat"));
        let private_details = Arc::new(serde_json::json!({"bedrock_model": private_source}));

        let malformed = replay_history(
            &openai,
            vec![
                MessageContent::RedactedThinking("not base64!".into()),
                MessageContent::Text("answer".into()),
            ],
            Some(openai_details.clone()),
        )?;
        assert!(matches!(
            malformed.messages[0].content(),
            [BedrockInnerContent::Text(text)] if text == "answer"
        ));

        for (model, details, replays_reasoning) in [
            (openai.clone(), Some(openai_details.clone()), true),
            (claude, Some(openai_details), false),
            (openai, None, false),
            (private, Some(private_details.clone()), true),
            (unknown("team-b/chat"), Some(private_details.clone()), false),
            (
                converse_model("vendor.chat", None),
                Some(private_details),
                false,
            ),
        ] {
            let request = replay_history(
                &model,
                vec![
                    MessageContent::RedactedThinking(data.clone()),
                    MessageContent::Text("answer".into()),
                ],
                details,
            )?;
            let replayed = request.messages[0]
                .content()
                .iter()
                .any(|content| match content {
                    BedrockInnerContent::ReasoningContent(
                        BedrockThinkingBlock::RedactedContent(bytes),
                    ) => bytes.as_ref() == [0, 255],
                    _ => false,
                });
            assert_eq!(replayed, replays_reasoning, "{}", model.id);
        }
        Ok(())
    }

    #[test]
    fn test_openai_parallel_tool_results_precede_their_images() -> Result<()> {
        let results = (1..=2)
            .map(|index| {
                let mut content = Vec::new();
                if index == 1 {
                    content.push(LanguageModelToolResultContent::Text("Result 1".into()));
                }
                content.push(LanguageModelToolResultContent::Image(
                    language_model::LanguageModelImage {
                        source: "iVBORw0KGgo=".into(),
                    },
                ));
                MessageContent::ToolResult(language_model::LanguageModelToolResult {
                    tool_use_id: format!("tool-{index}").into(),
                    tool_name: "view_image".into(),
                    is_error: false,
                    content,
                    output: None,
                })
            })
            .collect::<Vec<_>>();
        let request_for = |base_model_id| {
            into_bedrock(
                LanguageModelRequest {
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::User,
                        content: results.clone(),
                        cache: false,
                        reasoning_details: None,
                    }],
                    ..Default::default()
                },
                &converse_model(base_model_id, Some(reasoning())),
                None,
                None,
            )
        };
        let kinds = |request: &bedrock::Request| {
            request.messages[0]
                .content()
                .iter()
                .map(|content| match content {
                    BedrockInnerContent::ToolResult(_) => "result",
                    BedrockInnerContent::Image(_) => "image",
                    _ => "other",
                })
                .collect::<Vec<_>>()
        };
        let request = request_for("openai.gpt-example")?;
        assert_eq!(kinds(&request), ["result", "result", "image", "image"]);
        assert!(matches!(
            &request.messages[0].content()[1],
            BedrockInnerContent::ToolResult(result)
                if matches!(result.content().first(), Some(BedrockToolResultContentBlock::Text(_)))
        ));

        let request = request_for("xai.chat")?;
        assert_eq!(kinds(&request), ["result", "result"]);
        assert!(matches!(
            &request.messages[0].content()[1],
            BedrockInnerContent::ToolResult(result)
                if matches!(result.content().first(), Some(BedrockToolResultContentBlock::Image(_)))
        ));
        Ok(())
    }

    #[test]
    fn test_forced_tool_use_is_rejected_locally_where_unsupported() -> Result<()> {
        let named_after_known_model = converse_model_from_settings(
            &AvailableModel {
                supports_tools: Some(true),
                ..settings_entry("us.anthropic.claude-opus-5-5", None)
            },
            None,
            &CapabilityCatalog::default(),
        );
        // Which Claude models accept forced tool use is the `anthropic` crate's
        // rule. Ours is to ask it about the model behind each id.
        for (model, claude_id) in [
            (
                converse_model("anthropic.claude-fable-5-1", Some(adaptive_thinking(true))),
                Some("claude-fable-5-1"),
            ),
            (converse_model("vendor.chat", None), None),
            (named_after_known_model, Some("claude-opus-5-5")),
        ] {
            let expected_forced = claude_id.is_none_or(anthropic::supports_forced_tool_use);
            assert_eq!(
                converse_language_model(&model).tool_choice_support.any,
                expected_forced,
                "{}",
                model.id,
            );
            let forced = into_bedrock(
                LanguageModelRequest {
                    tool_choice: Some(LanguageModelToolChoice::Any),
                    messages: vec![LanguageModelRequestMessage {
                        role: Role::Assistant,
                        content: vec![MessageContent::ToolUse(
                            language_model::LanguageModelToolUse {
                                id: "call".into(),
                                name: "example".into(),
                                raw_input: "{}".into(),
                                is_input_complete: true,
                                thought_signature: None,
                                input: language_model::LanguageModelToolUseInput::Json(
                                    serde_json::json!({}),
                                ),
                            },
                        )],
                        cache: false,
                        reasoning_details: None,
                    }],
                    ..Default::default()
                },
                &model,
                None,
                None,
            );
            if expected_forced {
                let request = forced?;
                assert!(matches!(
                    request
                        .tools
                        .context("tool configuration for forced tool use")?
                        .tool_choice(),
                    Some(BedrockToolChoice::Any(_))
                ));
            } else {
                let error = forced
                    .err()
                    .context("forced tool use should fail locally")?;
                assert!(
                    error
                        .to_string()
                        .contains("does not support forced tool use")
                );
            }
        }
        Ok(())
    }

    #[test]
    fn test_refusal_stop_reasons_are_reported_for_models_that_refuse() -> Result<()> {
        for (model, reports_refusals) in [
            (
                converse_model("anthropic.claude-fable-5", Some(adaptive_thinking(false))),
                true,
            ),
            (
                converse_model("openai.gpt-example", Some(reasoning())),
                true,
            ),
            (converse_model("vendor.chat", None), false),
        ] {
            for reason in [StopReason::from("refusal"), StopReason::ContentFiltered] {
                let response = futures::stream::iter([Ok(ConverseStreamOutput::MessageStop(
                    bedrock::bedrock_client::types::MessageStopEvent::builder()
                        .stop_reason(reason)
                        .build()?,
                ))]);
                let expected_stop_reason = if reports_refusals {
                    language_model::StopReason::Refusal(None)
                } else {
                    language_model::StopReason::EndTurn
                };
                assert_eq!(
                    completion_events(&model, response)?,
                    [LanguageModelCompletionEvent::Stop(expected_stop_reason)],
                    "{}",
                    model.id
                );
            }
        }
        Ok(())
    }

    #[test]
    fn test_thinking_disallowed_sends_explicit_opt_out_only_where_required() -> Result<()> {
        use bedrock::Thinking::{BetweenTools, Disabled};

        for (base_model_id, output_limit, expected_output, expected_thinking) in [
            ("anthropic.claude-opus-5", None, 128_000, Some(Disabled)),
            ("anthropic.claude-opus-5", Some(8192), 8192, Some(Disabled)),
            ("anthropic.claude-haiku-5-5", None, 128_000, Some(Disabled)),
            (
                "anthropic.claude-sonnet-5-5",
                None,
                128_000,
                Some(BetweenTools),
            ),
            ("anthropic.claude-opus-4-8", Some(256_000), 128_000, None),
            ("vendor.chat", Some(8192), 8192, None),
        ] {
            let always_on = base_model_id == "anthropic.claude-sonnet-5-5";
            let model = converse_model(base_model_id, Some(adaptive_thinking(always_on)));
            let request = into_bedrock(
                LanguageModelRequest {
                    messages: vec![user_message("Hi")],
                    thinking_allowed: false,
                    max_output_tokens: output_limit,
                    ..Default::default()
                },
                &model,
                None,
                None,
            )?;

            assert_eq!(request.max_tokens, Some(expected_output));
            assert_request_thinking(&request, expected_thinking.as_ref());
        }
        Ok(())
    }

    #[test]
    fn test_output_limit_is_left_to_bedrock_when_the_model_has_none() {
        let model = ConverseModel {
            max_output_tokens: None,
            ..converse_model("vendor.chat", None)
        };
        assert_eq!(converse_language_model(&model).max_output_tokens, None);

        for (output_limit, expected_output) in [(None, None), (Some(8192), Some(8192))] {
            let request = into_bedrock(
                LanguageModelRequest {
                    messages: vec![user_message("Hi")],
                    max_output_tokens: output_limit,
                    ..Default::default()
                },
                &model,
                None,
                None,
            )
            .unwrap();
            assert_eq!(request.max_tokens, expected_output);
        }
    }

    #[test]
    fn test_temperature_is_sent_only_to_models_that_accept_it() {
        let temperature = |model: &ConverseModel, temperature: Option<f32>| {
            let request = into_bedrock(
                LanguageModelRequest {
                    messages: vec![user_message("Hi")],
                    temperature,
                    ..Default::default()
                },
                model,
                None,
                None,
            )
            .unwrap();
            assert_eq!(request.top_p, None, "{} must not send top_p", model.id);
            request.temperature
        };

        let without_default = ConverseModel {
            default_temperature: None,
            ..converse_model("vendor.chat", None)
        };
        assert_eq!(temperature(&without_default, None), None);
        assert_eq!(temperature(&without_default, Some(0.3)), Some(0.3));

        let with_default = ConverseModel {
            default_temperature: Some(0.7),
            ..converse_model("vendor.chat", None)
        };
        assert_eq!(temperature(&with_default, None), Some(0.7));
        assert_eq!(temperature(&with_default, Some(0.3)), Some(0.3));

        let unsupported = ConverseModel {
            supports_temperature: false,
            ..converse_model("vendor.chat", None)
        };
        assert_eq!(temperature(&unsupported, Some(0.3)), None);

        // Always-on adaptive thinking rejects sampling controls.
        let always_on = converse_model("anthropic.claude-example", Some(adaptive_thinking(true)));
        assert_eq!(temperature(&always_on, Some(0.3)), None);
        let optional = converse_model("anthropic.claude-example", Some(adaptive_thinking(false)));
        assert_eq!(temperature(&optional, None), Some(1.0));
        for id in ["anthropic.claude-sonnet-5-5", "anthropic.claude-haiku-5-5"] {
            let without_catalog_metadata = converse_model(id, None);
            assert_eq!(temperature(&without_catalog_metadata, Some(0.3)), None);
        }
    }

    #[test]
    fn test_cache_marked_message_that_filters_to_empty_is_dropped() {
        let request = into_bedrock_request(vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("What's the weather?".into())],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::Thinking {
                    text: "Let me think about this...".into(),
                    signature: None,
                }],
                cache: true,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::Text("Summarize this conversation.".into())],
                cache: false,
                reasoning_details: None,
            },
        ]);

        for message in &request.messages {
            assert!(
                message
                    .content()
                    .iter()
                    .any(|block| !matches!(block, BedrockInnerContent::CachePoint(_))),
                "message must not consist solely of cache points: {:?}",
                message
            );
        }
        assert!(
            request
                .messages
                .iter()
                .all(|message| *message.role() == bedrock::BedrockRole::User),
            "the assistant message stripped to empty content should be dropped entirely"
        );
    }

    #[test]
    fn test_cache_marked_message_with_content_gets_cache_point() {
        let request = into_bedrock_request(vec![LanguageModelRequestMessage {
            role: Role::User,
            content: vec![MessageContent::Text("What's the weather?".into())],
            cache: true,
            reasoning_details: None,
        }]);

        assert_eq!(request.messages.len(), 1);
        assert!(
            matches!(
                request.messages[0].content().last(),
                Some(BedrockInnerContent::CachePoint(_))
            ),
            "a cache-marked message with content should end with a cache point"
        );
    }

    #[test]
    fn test_sign_mantle_request_sigv4_uses_mantle_service() -> Result<()> {
        let credentials = Credentials::new(
            "AKIDEXAMPLE",
            "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY",
            None,
            None,
            "test",
        );
        let body = br#"{"model":"vendor.mantle-example"}"#;
        let mut request = HttpRequest::builder()
            .method(Method::POST)
            .uri("https://bedrock-mantle.us-east-1.api.aws/openai/v1/responses")
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(body.to_vec()))?;
        let time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_000);

        sign_mantle_request_sigv4_at(&mut request, body, &credentials, "us-east-1", time)?;

        assert_eq!(
            request
                .headers()
                .get(http_client::http::header::HOST)
                .and_then(|value| value.to_str().ok()),
            Some("bedrock-mantle.us-east-1.api.aws")
        );

        let authorization = request
            .headers()
            .get(AUTHORIZATION)
            .context("signed authorization header")?
            .to_str()?;
        assert!(
            authorization.contains("/us-east-1/bedrock-mantle/"),
            "{authorization}"
        );
        Ok(())
    }

    #[test]
    fn test_mantle_endpoint_url_uses_openai_path_prefix() {
        assert_eq!(
            mantle_endpoint_url("us-east-1"),
            "https://bedrock-mantle.us-east-1.api.aws/openai/v1"
        );
        assert_eq!(
            mantle_endpoint_url("us-west-2"),
            "https://bedrock-mantle.us-west-2.api.aws/openai/v1"
        );
    }

    #[test]
    fn test_mantle_protocol_from_settings() {
        assert_eq!(
            mantle_protocol_from_settings(settings::BedrockMantleProtocolContent::ChatCompletions),
            MantleProtocol::ChatCompletions
        );
        assert_eq!(
            mantle_protocol_from_settings(settings::BedrockMantleProtocolContent::Responses),
            MantleProtocol::Responses
        );
    }

    #[gpui::test]
    async fn test_mantle_region_policy_rejects_unconfigured_regions_before_http(
        cx: &mut gpui::TestAppContext,
    ) {
        let region = "test-region-not-configured";
        init_provider_settings(cx, serde_json::json!({"region": region}));
        let provider = cx.update(|cx| {
            new_test_provider(
                http_client::FakeHttpClient::create(|_| async {
                    panic!("an unconfigured Region reached HTTP")
                }),
                cx,
            )
        });
        provider
            .state
            .update(cx, |state, _| state.set_auth(api_key_auth()));
        let error = provider
            .stream_mantle_completion(
                &mantle_model(false),
                &RateLimiter::new(1),
                LanguageModelRequest::default(),
                &cx.to_async(),
            )
            .await
            .err()
            .expect("an unconfigured Region should be rejected")
            .to_string();
        assert!(
            error.contains(region) && error.contains(&MANTLE_SUPPORTED_REGIONS.join(", ")),
            "{error}"
        );
    }

    fn mantle_message_item(id: &str) -> ResponseOutputItem {
        mantle_message_item_with_phase(id, None)
    }

    fn mantle_message_item_with_phase(id: &str, phase: Option<&str>) -> ResponseOutputItem {
        ResponseOutputItem::Message(ResponseOutputMessage {
            id: Some(id.to_string()),
            content: Vec::new(),
            role: Some("assistant".to_string()),
            status: Some("in_progress".to_string()),
            phase: phase.map(str::to_string),
        })
    }

    fn mantle_function_call_item(id: &str, name: &str, call_id: &str) -> ResponseOutputItem {
        ResponseOutputItem::FunctionCall(ResponseFunctionToolCall {
            id: Some(id.to_string()),
            arguments: String::new(),
            call_id: Some(call_id.to_string()),
            name: Some(name.to_string()),
            status: Some("in_progress".to_string()),
        })
    }

    fn text_delta(item_id: &str, output_index: usize, delta: &str) -> OpenAiResponseStreamEvent {
        OpenAiResponseStreamEvent::OutputTextDelta {
            item_id: item_id.to_string(),
            output_index,
            content_index: Some(0),
            delta: delta.to_string(),
        }
    }

    fn item_added(output_index: usize, item: ResponseOutputItem) -> OpenAiResponseStreamEvent {
        OpenAiResponseStreamEvent::OutputItemAdded {
            output_index,
            sequence_number: None,
            item,
        }
    }

    fn item_done(output_index: usize, item: ResponseOutputItem) -> OpenAiResponseStreamEvent {
        OpenAiResponseStreamEvent::OutputItemDone {
            output_index,
            sequence_number: None,
            item,
        }
    }

    fn map_mantle_response_events(
        events: Vec<OpenAiResponseStreamEvent>,
    ) -> Vec<LanguageModelCompletionEvent> {
        let mut mapper = MantleResponseEventMapper::new();
        events
            .into_iter()
            .flat_map(|event| mapper.map_event(event))
            .map(|event| event.expect("Mantle response event should map successfully"))
            .collect()
    }

    // Keeps only StartMessage and Text events so phase/reasoning tests can assert on
    // message boundaries without depending on incidental ReasoningDetails metadata.
    fn start_messages_and_texts(
        events: Vec<LanguageModelCompletionEvent>,
    ) -> Vec<LanguageModelCompletionEvent> {
        events
            .into_iter()
            .filter(|event| {
                matches!(
                    event,
                    LanguageModelCompletionEvent::StartMessage { .. }
                        | LanguageModelCompletionEvent::Text(_)
                )
            })
            .collect()
    }

    #[test]
    fn mantle_response_mapper_coalesces_cumulative_message_snapshots() {
        let first_message = mantle_message_item("msg_1");
        let second_message = mantle_message_item("msg_2");
        let events = map_mantle_response_events(vec![
            OpenAiResponseStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: first_message.clone(),
            },
            OpenAiResponseStreamEvent::OutputTextDelta {
                item_id: "msg_1".to_string(),
                output_index: 0,
                content_index: Some(0),
                delta: "Plan: rename".to_string(),
            },
            OpenAiResponseStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: first_message,
            },
            OpenAiResponseStreamEvent::OutputItemAdded {
                output_index: 1,
                sequence_number: None,
                item: second_message.clone(),
            },
            OpenAiResponseStreamEvent::OutputTextDelta {
                item_id: "msg_2".to_string(),
                output_index: 1,
                content_index: Some(0),
                delta: "Plan: rename".to_string(),
            },
            OpenAiResponseStreamEvent::OutputTextDelta {
                item_id: "msg_2".to_string(),
                output_index: 1,
                content_index: Some(0),
                delta: " and regenerate".to_string(),
            },
            OpenAiResponseStreamEvent::OutputItemDone {
                output_index: 1,
                sequence_number: None,
                item: second_message,
            },
            OpenAiResponseStreamEvent::Completed {
                response: open_ai::responses::ResponseSummary::default(),
            },
        ]);

        assert_eq!(
            start_messages_and_texts(events),
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan: rename".to_string()),
                LanguageModelCompletionEvent::Text(" and regenerate".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_preserves_independent_messages() {
        let first_message = mantle_message_item("msg_1");
        let second_message = mantle_message_item("msg_2");
        let events = map_mantle_response_events(vec![
            OpenAiResponseStreamEvent::OutputItemAdded {
                output_index: 0,
                sequence_number: None,
                item: first_message.clone(),
            },
            OpenAiResponseStreamEvent::OutputTextDelta {
                item_id: "msg_1".to_string(),
                output_index: 0,
                content_index: Some(0),
                delta: "Plan".to_string(),
            },
            OpenAiResponseStreamEvent::OutputItemDone {
                output_index: 0,
                sequence_number: None,
                item: first_message,
            },
            OpenAiResponseStreamEvent::OutputItemAdded {
                output_index: 1,
                sequence_number: None,
                item: second_message.clone(),
            },
            OpenAiResponseStreamEvent::OutputTextDelta {
                item_id: "msg_2".to_string(),
                output_index: 1,
                content_index: Some(0),
                delta: "Final answer".to_string(),
            },
            OpenAiResponseStreamEvent::OutputItemDone {
                output_index: 1,
                sequence_number: None,
                item: second_message,
            },
        ]);

        assert_eq!(
            events,
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan".to_string()),
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_2".to_string(),
                },
                LanguageModelCompletionEvent::Text("Final answer".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_coalesces_chained_cumulative_snapshots() {
        // Each merge must thread `previous_message`/`emitted_text_length`
        // through to the next.
        let first_message = mantle_message_item("msg_1");
        let second_message = mantle_message_item("msg_2");
        let third_message = mantle_message_item("msg_3");
        let events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Plan"),
            item_done(0, first_message),
            item_added(1, second_message.clone()),
            text_delta("msg_2", 1, "Plan and go"),
            item_done(1, second_message),
            item_added(2, third_message.clone()),
            text_delta("msg_3", 2, "Plan and go further"),
            item_done(2, third_message),
        ]);

        assert_eq!(
            events,
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan".to_string()),
                LanguageModelCompletionEvent::Text(" and go".to_string()),
                LanguageModelCompletionEvent::Text(" further".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_keeps_message_state_across_content_events() {
        let first_message = mantle_message_item("msg_1");
        let second_message = mantle_message_item("msg_2");
        let events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Plan"),
            item_done(0, first_message),
            item_added(1, second_message.clone()),
            OpenAiResponseStreamEvent::ContentPartAdded {
                item_id: "msg_2".to_string(),
                output_index: 1,
                content_index: 0,
                part: serde_json::json!({"type": "output_text", "text": ""}),
            },
            text_delta("msg_2", 1, "Plan"),
            text_delta("msg_2", 1, " and continue"),
            OpenAiResponseStreamEvent::OutputTextDone {
                item_id: "msg_2".to_string(),
                output_index: 1,
                content_index: Some(0),
                text: "Plan and continue".to_string(),
            },
            OpenAiResponseStreamEvent::ContentPartDone {
                item_id: "msg_2".to_string(),
                output_index: 1,
                content_index: 0,
                part: serde_json::json!({"type": "output_text", "text": "Plan and continue"}),
            },
            item_done(1, second_message),
        ]);

        assert_eq!(
            events,
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan".to_string()),
                LanguageModelCompletionEvent::Text(" and continue".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_resets_on_non_message_item_done() {
        let first_message = mantle_message_item("msg_1");
        let second_message = mantle_message_item("msg_2");
        let tool_call = mantle_function_call_item("call_1", "get_weather", "server_call_1");
        let events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Plan"),
            item_done(0, first_message),
            item_done(1, tool_call),
            item_added(2, second_message.clone()),
            text_delta("msg_2", 2, "Plan continued"),
            item_done(2, second_message),
        ]);

        assert_eq!(
            start_messages_and_texts(events),
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan".to_string()),
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_2".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan continued".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_keeps_post_tool_message_separate_when_extending_prior_text() {
        // A tool call sits between two messages. Even though the second message's
        // text happens to extend the first message's text, the tool call is a hard
        // boundary and the second message must remain a separate, visible message.
        let first_message = mantle_message_item("msg_1");
        let tool_call = mantle_function_call_item("call_1", "get_weather", "server_call_1");
        let second_message = mantle_message_item("msg_2");
        let events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Plan"),
            item_done(0, first_message),
            item_added(1, tool_call.clone()),
            OpenAiResponseStreamEvent::FunctionCallArgumentsDone {
                item_id: "call_1".to_string(),
                output_index: 1,
                arguments: "{\"city\":\"Boston\"}".to_string(),
                sequence_number: None,
            },
            item_done(1, tool_call),
            item_added(2, second_message.clone()),
            text_delta("msg_2", 2, "Plan continued"),
            item_done(2, second_message),
        ]);

        assert!(
            events
                .iter()
                .any(|event| matches!(event, LanguageModelCompletionEvent::ToolUse(_))),
            "the tool call is forwarded"
        );
        assert_eq!(
            start_messages_and_texts(events),
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan".to_string()),
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_2".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan continued".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_preserves_equal_and_shrinking_messages() {
        // Equal and strict-prefix (shrinking) adjacent messages must not be dropped:
        // they are emitted as independent messages rather than collapsed away.
        let first_message = mantle_message_item("msg_1");
        let second_message = mantle_message_item("msg_2");
        let third_message = mantle_message_item("msg_3");
        let events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Hello world"),
            item_done(0, first_message),
            // Equal to the previous message.
            item_added(1, second_message.clone()),
            text_delta("msg_2", 1, "Hello world"),
            item_done(1, second_message),
            // Strict prefix of (shorter than) the previous message.
            item_added(2, third_message.clone()),
            text_delta("msg_3", 2, "Hello"),
            item_done(2, third_message),
        ]);

        assert_eq!(
            events,
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Hello world".to_string()),
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_2".to_string(),
                },
                LanguageModelCompletionEvent::Text("Hello world".to_string()),
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_3".to_string(),
                },
                LanguageModelCompletionEvent::Text("Hello".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_keeps_different_phase_messages_separate() {
        // Two messages share a text prefix but have different phases; only same-phase
        // strict extensions collapse, so the second message stays visible.
        let first_message = mantle_message_item_with_phase("msg_1", Some("commentary"));
        let second_message = mantle_message_item_with_phase("msg_2", Some("final_answer"));
        let events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Plan: rename"),
            item_done(0, first_message),
            item_added(1, second_message.clone()),
            text_delta("msg_2", 1, "Plan: rename and regenerate"),
            item_done(1, second_message),
        ]);

        assert_eq!(
            start_messages_and_texts(events),
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan: rename".to_string()),
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_2".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan: rename and regenerate".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_coalesces_same_phase_strict_extension() {
        // Same phase, strict extension: the replayed prefix is collapsed and only the
        // new suffix is emitted, so no second StartMessage appears.
        let first_message = mantle_message_item_with_phase("msg_1", Some("final_answer"));
        let second_message = mantle_message_item_with_phase("msg_2", Some("final_answer"));
        let events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Plan: rename"),
            item_done(0, first_message),
            item_added(1, second_message.clone()),
            text_delta("msg_2", 1, "Plan: rename"),
            text_delta("msg_2", 1, " and regenerate"),
            item_done(1, second_message),
        ]);

        assert_eq!(
            start_messages_and_texts(events),
            vec![
                LanguageModelCompletionEvent::StartMessage {
                    message_id: "msg_1".to_string(),
                },
                LanguageModelCompletionEvent::Text("Plan: rename".to_string()),
                LanguageModelCompletionEvent::Text(" and regenerate".to_string()),
            ]
        );
    }

    #[test]
    fn mantle_response_mapper_does_not_duplicate_reasoning_details_on_merge() {
        // Merging a second message into the first must not emit any additional
        // phase/reasoning metadata events beyond what the first message alone would
        // have produced, since the metadata can't have changed by merging.
        fn reasoning_details_count(events: &[LanguageModelCompletionEvent]) -> usize {
            events
                .iter()
                .filter(|event| matches!(event, LanguageModelCompletionEvent::ReasoningDetails(_)))
                .count()
        }

        let first_message = mantle_message_item_with_phase("msg_1", Some("final_answer"));
        let baseline_events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Plan: rename"),
            item_done(0, first_message.clone()),
        ]);

        let second_message = mantle_message_item_with_phase("msg_2", Some("final_answer"));
        let merged_events = map_mantle_response_events(vec![
            item_added(0, first_message.clone()),
            text_delta("msg_1", 0, "Plan: rename"),
            item_done(0, first_message),
            item_added(1, second_message.clone()),
            text_delta("msg_2", 1, "Plan: rename"),
            text_delta("msg_2", 1, " and regenerate"),
            item_done(1, second_message),
        ]);

        assert!(reasoning_details_count(&baseline_events) > 0);
        assert_eq!(
            reasoning_details_count(&merged_events),
            reasoning_details_count(&baseline_events),
        );
    }

    fn mantle_model(supports_thinking: bool) -> MantleModel {
        MantleModel {
            known_foundation_model_id: None,
            name: "vendor.mantle-model".to_string(),
            display_name: None,
            max_tokens: 128_000,
            max_output_tokens: None,
            protocol: MantleProtocol::Responses,
            supports_tools: true,
            supports_images: false,
            supports_thinking,
        }
    }

    #[test]
    fn test_mantle_supported_effort_levels_hide_none() {
        let effort_levels = mantle_supported_effort_levels(&mantle_model(true));
        let values = effort_levels
            .iter()
            .map(|level| level.value.as_ref())
            .collect::<Vec<_>>();

        assert_eq!(values, ["low", "medium", "high", "xhigh"]);
        assert_eq!(
            effort_levels
                .iter()
                .find(|level| level.is_default)
                .map(|level| level.value.as_ref()),
            Some("medium")
        );
    }

    #[test]
    fn test_custom_mantle_model_can_disable_thinking() {
        let model = mantle_model(false);

        assert_eq!(mantle_default_reasoning_effort(&model), None);
        assert!(mantle_supported_effort_levels(&model).is_empty());
        assert_eq!(
            mantle_selected_reasoning_effort(
                &LanguageModelRequest {
                    thinking_effort: Some("high".to_string()),
                    ..Default::default()
                },
                &model,
            ),
            None
        );
    }

    #[test]
    fn test_mantle_reasoning_passes_known_efforts_through() {
        let model = mantle_model(true);
        assert_eq!(
            mantle_selected_reasoning_effort(
                &LanguageModelRequest {
                    thinking_allowed: true,
                    thinking_effort: Some("high".to_string()),
                    ..Default::default()
                },
                &model,
            ),
            Some(ReasoningEffort::High)
        );

        assert_eq!(
            mantle_selected_reasoning_effort(
                &LanguageModelRequest {
                    thinking_allowed: true,
                    thinking_effort: Some("none".to_string()),
                    ..Default::default()
                },
                &model,
            ),
            Some(ReasoningEffort::Medium)
        );
    }

    #[test]
    fn test_strip_unsupported_mantle_response_fields_removes_context_management() -> Result<()> {
        let mut request = into_open_ai_response(
            LanguageModelRequest::default(),
            "vendor.mantle-example",
            true,
            false,
            Some(128_000),
            Some(ReasoningEffort::Medium),
            false,
            &PROVIDER_ID,
        )?;

        request.context_management =
            Some(vec![open_ai::responses::ContextManagement::Compaction {
                compact_threshold: 10_000,
            }]);
        strip_unsupported_mantle_response_fields(&mut request);
        assert!(request.context_management.is_none());

        Ok(())
    }

    struct NoCredentialsProvider;

    impl CredentialsProvider for NoCredentialsProvider {
        fn read_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<Option<(String, Vec<u8>)>>> + 'a>> {
            Box::pin(async { Ok(None) })
        }

        fn write_credentials<'a>(
            &'a self,
            _url: &'a str,
            _username: &'a str,
            _password: &'a [u8],
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }

        fn delete_credentials<'a>(
            &'a self,
            _url: &'a str,
            _cx: &'a AsyncApp,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
            Box::pin(async { Ok(()) })
        }
    }

    fn test_provider(cx: &mut gpui::TestAppContext) -> BedrockLanguageModelProvider {
        init_provider_test(cx);
        cx.update(|cx| new_test_provider(http_client::FakeHttpClient::with_404_response(), cx))
    }

    fn no_environment(_: &str) -> Option<String> {
        None
    }

    async fn resolved_credentials(
        access_key_id: &str,
        expires_in: Option<Duration>,
    ) -> Result<BedrockRequestAuth> {
        Ok(BedrockRequestAuth::SigV4 {
            credentials: Credentials::new(
                access_key_id,
                "secret",
                None,
                expires_in.map(|expires_in| SystemTime::now() + expires_in),
                "test",
            ),
        })
    }

    fn access_key_id(request_auth: Result<BedrockRequestAuth>) -> String {
        match request_auth.expect("auth should resolve") {
            BedrockRequestAuth::SigV4 { credentials } => credentials.access_key_id().to_string(),
            BedrockRequestAuth::ApiKey { .. } => panic!("expected SigV4 credentials"),
        }
    }
}
