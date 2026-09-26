use anyhow::{Context as _, Result};
use aws_config::{BehaviorVersion, Region};
use aws_credential_types::{
    Credentials,
    provider::{ProvideCredentials as _, SharedCredentialsProvider},
};
use aws_sigv4::{
    http_request::{SignableBody, SignableRequest, SigningSettings, sign},
    sign::v4,
};
use futures::AsyncReadExt as _;
use gpui::{App, Task, http_client};
use gpui_tokio::Tokio;
use language::language_settings::{
    AmazonBedrockEditPredictionSettings, AmazonBedrockPredictionBackend,
};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Arc, LazyLock, Mutex},
    time::SystemTime,
};

/// Neither Bedrock endpoint family exposes a raw text-completion API, so FIM
/// prompts are wrapped in a chat message and the model is instructed to reply
/// with nothing but the completion.
const SYSTEM_PROMPT: &str = concat!(
    "You are a code completion engine. The user message contains code context, ",
    "possibly using fill-in-the-middle markers (e.g. <|fim_prefix|>, <|fim_suffix|>, ",
    "<|fim_middle|>, [PREFIX], [SUFFIX], <PRE>, <SUF>, <MID>). Your entire reply is ",
    "inserted verbatim at the cursor position, between the prefix and the suffix. ",
    "Output ONLY the code that belongs exactly at that point. No explanations, no ",
    "markdown fences, no marker tokens of any kind (never emit [PREFIX], [SUFFIX], ",
    "[POSTFIX], [MIDDLE], <|fim_middle|>, or similar), and no repetition of the ",
    "surrounding code. If the code the file ",
    "needs belongs somewhere OTHER than the cursor position (e.g. missing imports ",
    "at the top of the file), do not output it and do not describe it -- it cannot ",
    "be placed there. In that case output the best completion for the cursor ",
    "position itself, or an empty reply if there is nothing to add."
);

/// The chat completions API accepts at most 4 stop sequences.
const MAX_STOP_SEQUENCES: usize = 4;

#[derive(Clone, PartialEq, Eq)]
struct CredentialsProviderCacheKey {
    profile: Option<String>,
    region: String,
}

/// Building a credentials provider reads AWS config files, and resolving
/// credentials may spawn a `credential_process`, so cache both per
/// (profile, region) instead of paying that cost on every prediction.
static CREDENTIALS_PROVIDER_CACHE: Mutex<
    Option<(CredentialsProviderCacheKey, SharedCredentialsProvider)>,
> = Mutex::new(None);

static CREDENTIALS_CACHE: Mutex<Option<(CredentialsProviderCacheKey, Credentials)>> =
    Mutex::new(None);

/// Refresh this long before the cached credentials actually expire, so an
/// in-flight prediction never gets signed with just-expired credentials.
const CREDENTIALS_EXPIRY_MARGIN: std::time::Duration = std::time::Duration::from_secs(120);

/// The caches hold plain data with no cross-panic invariants, so a poisoned
/// lock is safe to recover from rather than silently skipping the cache.
fn lock_cache<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn cached_credentials(cache_key: &CredentialsProviderCacheKey) -> Option<Credentials> {
    let cache = lock_cache(&CREDENTIALS_CACHE);
    let (key, credentials) = cache.as_ref()?;
    if key != cache_key {
        return None;
    }
    match credentials.expiry() {
        Some(expiry) if SystemTime::now() + CREDENTIALS_EXPIRY_MARGIN >= expiry => None,
        _ => Some(credentials.clone()),
    }
}

pub fn resolve_credentials(
    settings: &AmazonBedrockEditPredictionSettings,
    cx: &App,
) -> Task<Result<Credentials>> {
    let cache_key = CredentialsProviderCacheKey {
        profile: settings.profile.clone(),
        region: settings.region.clone(),
    };
    Tokio::spawn_result(cx, async move {
        if let Some(credentials) = cached_credentials(&cache_key) {
            return Ok(credentials);
        }

        let cached_provider = lock_cache(&CREDENTIALS_PROVIDER_CACHE)
            .clone()
            .and_then(|(key, provider)| (key == cache_key).then_some(provider));

        let provider = match cached_provider {
            Some(provider) => provider,
            None => {
                let mut config_builder = aws_config::defaults(BehaviorVersion::latest())
                    .region(Region::new(cache_key.region.clone()));
                if let Some(profile) = cache_key.profile.clone() {
                    config_builder = config_builder.profile_name(profile);
                }
                let config = config_builder.load().await;
                let provider = config
                    .credentials_provider()
                    .context("no AWS credentials provider is configured")?;
                *lock_cache(&CREDENTIALS_PROVIDER_CACHE) =
                    Some((cache_key.clone(), provider.clone()));
                provider
            }
        };

        let credentials = provider
            .provide_credentials()
            .await
            .context("failed to resolve AWS credentials for Amazon Bedrock edit predictions")?;
        *lock_cache(&CREDENTIALS_CACHE) = Some((cache_key, credentials.clone()));
        Ok(credentials)
    })
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stop: Vec<String>,
}

#[derive(Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    #[serde(default)]
    id: String,
    #[serde(default)]
    choices: Vec<ChatCompletionChoice>,
}

#[derive(Deserialize)]
struct ChatCompletionChoice {
    message: ChatCompletionMessage,
}

#[derive(Deserialize)]
struct ChatCompletionMessage {
    #[serde(default)]
    content: String,
}

/// Mantle serves disjoint model catalogs on its two OpenAI-compatible routes
/// (e.g. qwen3-coder and gemma-3 under `/v1`; gemma-4, GPT, and Grok under
/// `/openai/v1`), rejecting models from the other route with "model isn't
/// supported on this route". Since there is no single correct route, try one
/// and fall back to the other, remembering the working route per model.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum MantleRoute {
    V1,
    OpenAiV1,
}

impl MantleRoute {
    fn other(self) -> Self {
        match self {
            MantleRoute::V1 => MantleRoute::OpenAiV1,
            MantleRoute::OpenAiV1 => MantleRoute::V1,
        }
    }

    fn chat_completions_url(self, region: &str) -> String {
        let path = match self {
            MantleRoute::V1 => "v1",
            MantleRoute::OpenAiV1 => "openai/v1",
        };
        format!("https://bedrock-mantle.{region}.api.aws/{path}/chat/completions")
    }
}

static MANTLE_MODEL_ROUTES: LazyLock<Mutex<std::collections::HashMap<String, MantleRoute>>> =
    LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));

fn is_unsupported_route_error(status: http_client::http::StatusCode, body: &str) -> bool {
    status == http_client::http::StatusCode::BAD_REQUEST
        && body.contains("isn't supported on this route")
}

fn signing_name(backend: AmazonBedrockPredictionBackend) -> &'static str {
    match backend {
        AmazonBedrockPredictionBackend::Mantle => "bedrock-mantle",
        AmazonBedrockPredictionBackend::Runtime => "bedrock",
    }
}

pub(crate) async fn send_fim_request(
    settings: &AmazonBedrockEditPredictionSettings,
    credentials: Credentials,
    prompt: String,
    max_tokens: u32,
    stop_tokens: Vec<String>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<(String, String)> {
    let request = ChatCompletionRequest {
        model: settings.model.clone(),
        messages: vec![
            ChatMessage {
                role: "system",
                content: settings
                    .system_prompt
                    .as_deref()
                    .unwrap_or(SYSTEM_PROMPT)
                    .to_string(),
            },
            ChatMessage {
                role: "user",
                content: prompt,
            },
        ],
        max_tokens,
        temperature: 0.0,
        stop: stop_tokens.into_iter().take(MAX_STOP_SEQUENCES).collect(),
    };
    let body = serde_json::to_vec(&request)?;

    let mut attempts: Vec<(String, Option<MantleRoute>)> = Vec::new();
    if let Some(endpoint_url) = &settings.endpoint_url {
        attempts.push((
            format!("{}/chat/completions", endpoint_url.trim_end_matches('/')),
            None,
        ));
    } else {
        match settings.backend {
            AmazonBedrockPredictionBackend::Runtime => attempts.push((
                format!(
                    "https://bedrock-runtime.{}.amazonaws.com/openai/v1/chat/completions",
                    settings.region
                ),
                None,
            )),
            AmazonBedrockPredictionBackend::Mantle => {
                let preferred = lock_cache(&MANTLE_MODEL_ROUTES)
                    .get(&settings.model)
                    .copied()
                    .unwrap_or(MantleRoute::V1);
                attempts.push((
                    preferred.chat_completions_url(&settings.region),
                    Some(preferred),
                ));
                attempts.push((
                    preferred.other().chat_completions_url(&settings.region),
                    Some(preferred.other()),
                ));
            }
        }
    }

    let attempt_count = attempts.len();
    for (index, (url, route)) in attempts.into_iter().enumerate() {
        let (status, response_body) =
            send_chat_request(&url, &body, settings, &credentials, http_client).await?;

        if is_unsupported_route_error(status, &response_body) && index + 1 < attempt_count {
            continue;
        }
        if !status.is_success() {
            anyhow::bail!("amazon bedrock error: {} - {}", status, response_body);
        }

        if let Some(route) = route {
            lock_cache(&MANTLE_MODEL_ROUTES).insert(settings.model.clone(), route);
        }

        let parsed: ChatCompletionResponse = serde_json::from_str(&response_body)
            .context("failed to parse Amazon Bedrock chat completion response")?;
        let content = parsed
            .choices
            .into_iter()
            .next()
            .context("Amazon Bedrock returned no choices")?
            .message
            .content;

        return Ok((extract_completion(&content), parsed.id));
    }
    anyhow::bail!("amazon bedrock request was not attempted on any route");
}

async fn send_chat_request(
    url: &str,
    body: &[u8],
    settings: &AmazonBedrockEditPredictionSettings,
    credentials: &Credentials,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<(http_client::http::StatusCode, String)> {
    let mut http_request = http_client::Request::builder()
        .method(http_client::Method::POST)
        .uri(url)
        .header("Content-Type", "application/json")
        .body(http_client::AsyncBody::from(body.to_vec()))?;
    sign_request_sigv4(
        &mut http_request,
        body,
        credentials,
        &settings.region,
        signing_name(settings.backend),
    )?;

    let mut response = http_client.send(http_request).await?;
    let status = response.status();
    let mut response_body = String::new();
    response
        .body_mut()
        .read_to_string(&mut response_body)
        .await?;
    Ok((status, response_body))
}

fn sign_request_sigv4(
    request: &mut http_client::http::Request<http_client::AsyncBody>,
    body: &[u8],
    credentials: &Credentials,
    region: &str,
    signing_name: &str,
) -> Result<()> {
    if !request
        .headers()
        .contains_key(http_client::http::header::HOST)
        && let Some(authority) = request.uri().authority()
    {
        let host = http_client::http::HeaderValue::from_str(authority.as_str())
            .context("invalid host header derived from Bedrock request URI")?;
        request
            .headers_mut()
            .insert(http_client::http::header::HOST, host);
    }

    let identity = credentials.clone().into();
    let signing_params: aws_sigv4::http_request::SigningParams = v4::SigningParams::builder()
        .identity(&identity)
        .region(region)
        .name(signing_name)
        .time(SystemTime::now())
        .settings(SigningSettings::default())
        .build()
        .context("building Bedrock SigV4 signing params")?
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
            .context("constructing Bedrock SigV4 request")?;

    let (instructions, _signature) = sign(signable_request, &signing_params)
        .context("signing Bedrock request with SigV4")?
        .into_parts();
    instructions.apply_to_request_http1x(request);

    Ok(())
}

static FENCED_BLOCK: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?s)```[^\n]*\n(.*?)(?:\n?```|\z)").expect("valid fenced block regex")
});

/// Chat models sometimes echo FIM-style marker tokens despite the system
/// prompt, with inventive bracketing (e.g. `[[POSTFIX]`), so allow doubled
/// brackets around the marker names the shared cleanup already truncates at.
static MARKER_TOKEN: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(
        r"\[{1,2}(PREFIX|SUFFIX|POSTFIX|MIDDLE)\]{1,2}|<\|?(PRE|SUF|MID|fim_prefix|fim_suffix|fim_middle|fim_pad|endoftext|file_separator)\|?>",
    )
    .expect("valid marker token regex")
});

/// Chat models sometimes wrap the completion in a markdown fence or preface it
/// with prose despite the system prompt; keep only the code, truncated at the
/// first marker token.
fn extract_completion(content: &str) -> String {
    let mut completion = if let Some(fenced_block) = FENCED_BLOCK
        .captures(content)
        .and_then(|captures| captures.get(1))
    {
        fenced_block.as_str().to_string()
    } else {
        let trimmed = content.trim();
        if trimmed.len() > 2
            && trimmed.starts_with('`')
            && trimmed.ends_with('`')
            && !trimmed.contains('\n')
        {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            content.to_string()
        }
    };

    if let Some(marker) = MARKER_TOKEN.find(&completion) {
        completion.truncate(marker.start());
        completion.truncate(completion.trim_end().len());
    }
    completion
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_completion() {
        assert_eq!(extract_completion("plain_code()"), "plain_code()");
        assert_eq!(extract_completion("```python\ncode()\n```"), "code()");
        assert_eq!(extract_completion("```python\ncode()"), "code()");
        assert_eq!(
            extract_completion(
                "The missing imports are:\n```python\nimport logging\nimport secrets\n```"
            ),
            "import logging\nimport secrets"
        );
        assert_eq!(extract_completion("`inline()`"), "inline()");
        assert_eq!(
            extract_completion("multi\nline().starts_with('`')`"),
            "multi\nline().starts_with('`')`"
        );
        assert_eq!(
            extract_completion("return datetime.strptime(time, \"%Y-%m-%dT%H:%M:%S\")\n[[POSTFIX]"),
            "return datetime.strptime(time, \"%Y-%m-%dT%H:%M:%S\")"
        );
        assert_eq!(extract_completion("code()\n[SUFFIX]more"), "code()");
        assert_eq!(extract_completion("code()<|fim_middle|>"), "code()");
        assert_eq!(
            extract_completion("value[POSTFIX_LEN:] + suffix"),
            "value[POSTFIX_LEN:] + suffix"
        );
    }
}
