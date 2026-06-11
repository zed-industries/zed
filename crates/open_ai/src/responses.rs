use anyhow::{Result, anyhow};
use futures::{
    AsyncBufReadExt, AsyncReadExt, FutureExt, Stream, StreamExt, future::BoxFuture, io::BufReader,
    stream::BoxStream,
};
use http_client::{
    AsyncBody, CustomHeaders, HttpClient, Method, Request as HttpRequest, RequestBuilderExt,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use crate::{ReasoningEffort, RequestError, Role, ServiceTier, ToolChoice};

/// Activity-based timeout for `stream_response_with_idle_timeout`, mirroring
/// the `stream_idle_timeout` behavior of OpenAI's Codex CLI: the timer covers
/// the window from sending the request until response headers arrive, and is
/// then reset every time a stream event arrives. If no activity occurs within
/// `duration`, the request fails with [`RequestError::StreamIdleTimeout`].
pub struct StreamIdleTimeout {
    pub duration: Duration,
    /// Creates a future that resolves after the given duration. Callers
    /// provide this so that this crate doesn't depend on a specific executor
    /// (e.g. GPUI's `BackgroundExecutor::timer`, which is also controllable
    /// from tests).
    pub make_timer: Arc<dyn Fn(Duration) -> BoxFuture<'static, ()> + Send + Sync>,
}

impl StreamIdleTimeout {
    fn timer(&self) -> BoxFuture<'static, ()> {
        (self.make_timer)(self.duration)
    }

    fn error(&self, provider_name: &str) -> RequestError {
        RequestError::StreamIdleTimeout {
            provider: provider_name.to_owned(),
            timeout: self.duration,
        }
    }
}

/// Wraps a stream of SSE events, failing it with
/// [`RequestError::StreamIdleTimeout`] if no event arrives within the
/// configured duration. The timer is reset every time an event arrives.
struct IdleTimeoutStream {
    provider_name: String,
    inner: BoxStream<'static, Result<StreamEvent>>,
    idle_timeout: StreamIdleTimeout,
    timer: BoxFuture<'static, ()>,
    timed_out: bool,
}

impl IdleTimeoutStream {
    fn new(
        provider_name: String,
        inner: BoxStream<'static, Result<StreamEvent>>,
        idle_timeout: StreamIdleTimeout,
    ) -> Self {
        let timer = idle_timeout.timer();
        Self {
            provider_name,
            inner,
            idle_timeout,
            timer,
            timed_out: false,
        }
    }
}

impl Stream for IdleTimeoutStream {
    type Item = Result<StreamEvent>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        if this.timed_out {
            return Poll::Ready(None);
        }
        match this.inner.poll_next_unpin(cx) {
            Poll::Ready(item) => {
                this.timer = this.idle_timeout.timer();
                Poll::Ready(item)
            }
            Poll::Pending => match this.timer.poll_unpin(cx) {
                Poll::Ready(()) => {
                    this.timed_out = true;
                    log::error!(
                        "No data received from {}'s API within {:?}; treating the stream as dead",
                        this.provider_name,
                        this.idle_timeout.duration,
                    );
                    let error = this.idle_timeout.error(&this.provider_name);
                    Poll::Ready(Some(Err(anyhow::Error::new(error))))
                }
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Request {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub input: Vec<ResponseInputItem>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<ResponseIncludable>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<ToolDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseIncludable {
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseInputItem {
    Message(ResponseMessageItem),
    FunctionCall(ResponseFunctionCallItem),
    FunctionCallOutput(ResponseFunctionCallOutputItem),
    Reasoning(ResponseReasoningInputItem),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessageItem {
    pub role: Role,
    pub content: Vec<ResponseInputContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFunctionCallItem {
    pub call_id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFunctionCallOutputItem {
    pub call_id: String,
    pub output: ResponseFunctionCallOutputContent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResponseReasoningInputItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub summary: Vec<ResponseReasoningSummaryPart>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub content: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseReasoningSummaryPart {
    SummaryText { text: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseFunctionCallOutputContent {
    List(Vec<ResponseInputContent>),
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseInputContent {
    #[serde(rename = "input_text")]
    Text { text: String },
    #[serde(rename = "input_image")]
    Image { image_url: String },
    #[serde(rename = "output_text")]
    OutputText {
        text: String,
        #[serde(default)]
        annotations: Vec<serde_json::Value>,
    },
    #[serde(rename = "refusal")]
    Refusal { refusal: String },
}

#[derive(Serialize, Debug)]
pub struct ReasoningConfig {
    pub effort: ReasoningEffort,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummaryMode>,
}

#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummaryMode {
    Auto,
    Concise,
    Detailed,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToolDefinition {
    Function {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parameters: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseError {
    #[serde(default)]
    pub code: Option<String>,
    pub message: String,
    #[serde(default)]
    pub param: Option<Value>,
}

/// Payload of the top-level `error` SSE event from the Responses API.
///
/// OpenAI's spec documents the error fields as being at the top level of the
/// event, but in practice the API often nests them under an `error` object.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct GenericStreamErrorPayload {
    #[serde(flatten)]
    top_level: PartialResponseError,
    #[serde(default)]
    error: Option<PartialResponseError>,
}

#[derive(Deserialize, Debug, Clone, Default)]
struct PartialResponseError {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    param: Option<Value>,
}

impl GenericStreamErrorPayload {
    pub fn into_response_error(self) -> ResponseError {
        let nested = self.error.unwrap_or_default();
        ResponseError {
            code: self.top_level.code.or(nested.code),
            message: self
                .top_level
                .message
                .or(nested.message)
                .unwrap_or_default(),
            param: self.top_level.param.or(nested.param),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum StreamEvent {
    #[serde(rename = "response.created")]
    Created { response: ResponseSummary },
    #[serde(rename = "response.in_progress")]
    InProgress { response: ResponseSummary },
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        output_index: usize,
        #[serde(default)]
        sequence_number: Option<u64>,
        item: ResponseOutputItem,
    },
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        output_index: usize,
        #[serde(default)]
        sequence_number: Option<u64>,
        item: ResponseOutputItem,
    },
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        item_id: String,
        output_index: usize,
        content_index: usize,
        part: Value,
    },
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        item_id: String,
        output_index: usize,
        content_index: usize,
        part: Value,
    },
    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        item_id: String,
        output_index: usize,
        #[serde(default)]
        content_index: Option<usize>,
        delta: String,
    },
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        item_id: String,
        output_index: usize,
        #[serde(default)]
        content_index: Option<usize>,
        text: String,
    },
    #[serde(rename = "response.refusal.delta")]
    RefusalDelta {
        item_id: String,
        output_index: usize,
        content_index: usize,
        delta: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.refusal.done")]
    RefusalDone {
        item_id: String,
        output_index: usize,
        content_index: usize,
        refusal: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.reasoning_summary_part.added")]
    ReasoningSummaryPartAdded {
        item_id: String,
        output_index: usize,
        summary_index: usize,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ReasoningSummaryTextDelta {
        item_id: String,
        output_index: usize,
        delta: String,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ReasoningSummaryTextDone {
        item_id: String,
        output_index: usize,
        text: String,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ReasoningSummaryPartDone {
        item_id: String,
        output_index: usize,
        summary_index: usize,
    },
    #[serde(rename = "response.function_call_arguments.delta")]
    FunctionCallArgumentsDelta {
        item_id: String,
        output_index: usize,
        delta: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    FunctionCallArgumentsDone {
        item_id: String,
        output_index: usize,
        arguments: String,
        #[serde(default)]
        sequence_number: Option<u64>,
    },
    #[serde(rename = "response.completed")]
    Completed { response: ResponseSummary },
    #[serde(rename = "response.incomplete")]
    Incomplete { response: ResponseSummary },
    #[serde(rename = "response.failed")]
    Failed { response: ResponseSummary },
    #[serde(rename = "response.error")]
    Error { error: ResponseError },
    #[serde(rename = "error")]
    GenericError {
        #[serde(flatten)]
        error: GenericStreamErrorPayload,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseSummary {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub incomplete_details: Option<ResponseIncompleteDetails>,
    #[serde(default)]
    pub error: Option<ResponseError>,
    #[serde(default)]
    pub usage: Option<ResponseUsage>,
    #[serde(default)]
    pub output: Vec<ResponseOutputItem>,
    #[serde(default)]
    pub service_tier: Option<crate::ServiceTier>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseIncompleteDetails {
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseUsage {
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub input_tokens_details: ResponseInputTokensDetails,
    #[serde(default)]
    pub output_tokens: Option<u64>,
    #[serde(default)]
    pub output_tokens_details: ResponseOutputTokensDetails,
    #[serde(default)]
    pub total_tokens: Option<u64>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseInputTokensDetails {
    #[serde(default)]
    pub cached_tokens: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ResponseOutputTokensDetails {
    #[serde(default)]
    pub reasoning_tokens: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseOutputItem {
    Message(ResponseOutputMessage),
    FunctionCall(ResponseFunctionToolCall),
    Reasoning(ResponseReasoningItem),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseReasoningItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub summary: Vec<ReasoningSummaryPart>,
    #[serde(default)]
    pub content: Vec<Value>,
    #[serde(default)]
    pub encrypted_content: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReasoningSummaryPart {
    SummaryText {
        text: String,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseOutputMessage {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub content: Vec<Value>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub phase: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseFunctionToolCall {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub arguments: String,
    #[serde(default)]
    pub call_id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

pub async fn stream_response(
    client: &dyn HttpClient,
    provider_name: &str,
    api_url: &str,
    api_key: &str,
    request: Request,
    extra_headers: &CustomHeaders,
) -> Result<BoxStream<'static, Result<StreamEvent>>, RequestError> {
    stream_response_with_idle_timeout(
        client,
        provider_name,
        api_url,
        api_key,
        request,
        extra_headers,
        None,
    )
    .await
}

pub async fn stream_response_with_idle_timeout(
    client: &dyn HttpClient,
    provider_name: &str,
    api_url: &str,
    api_key: &str,
    request: Request,
    extra_headers: &CustomHeaders,
    idle_timeout: Option<StreamIdleTimeout>,
) -> Result<BoxStream<'static, Result<StreamEvent>>, RequestError> {
    let uri = format!("{api_url}/responses");
    let is_streaming = request.stream;
    let request = HttpRequest::builder()
        .method(Method::POST)
        .uri(uri)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key.trim()))
        .extra_headers(extra_headers)
        .body(AsyncBody::from(
            serde_json::to_string(&request).map_err(|e| RequestError::Other(e.into()))?,
        ))
        .map_err(|e| RequestError::Other(e.into()))?;

    // The Codex backend doesn't send response headers until the model starts
    // producing output, so this window can legitimately last as long as the
    // model thinks. The idle timeout must therefore be generous (see the
    // 10-second header timeout regression in #57891/#58035).
    let mut response = match &idle_timeout {
        Some(idle_timeout) => {
            let mut send = std::pin::pin!(client.send(request).fuse());
            let mut timer = std::pin::pin!(idle_timeout.timer().fuse());
            futures::select_biased! {
                response = send => response?,
                _ = timer => return Err(idle_timeout.error(provider_name)),
            }
        }
        None => client.send(request).await?,
    };
    if response.status().is_success() {
        if is_streaming {
            let reader = BufReader::new(response.into_body());
            let events = reader
                .lines()
                .filter_map(|line| async move {
                    match line {
                        Ok(line) => {
                            let line = line
                                .strip_prefix("data: ")
                                .or_else(|| line.strip_prefix("data:"))?;
                            if line == "[DONE]" || line.is_empty() {
                                None
                            } else {
                                match serde_json::from_str::<StreamEvent>(line) {
                                    Ok(event) => Some(Ok(event)),
                                    Err(error) => {
                                        log::error!(
                                            "Failed to parse OpenAI responses stream event: `{}`\nResponse: `{}`",
                                            error,
                                            line,
                                        );
                                        Some(Err(anyhow!(error)))
                                    }
                                }
                            }
                        }
                        Err(error) => Some(Err(anyhow!(error))),
                    }
                })
                .boxed();
            Ok(match idle_timeout {
                Some(idle_timeout) => {
                    IdleTimeoutStream::new(provider_name.to_owned(), events, idle_timeout).boxed()
                }
                None => events,
            })
        } else {
            let mut body = String::new();
            let read = response.body_mut().read_to_string(&mut body);
            match &idle_timeout {
                Some(idle_timeout) => {
                    let mut read = std::pin::pin!(read.fuse());
                    let mut timer = std::pin::pin!(idle_timeout.timer().fuse());
                    futures::select_biased! {
                        result = read => result.map_err(|e| RequestError::Other(e.into()))?,
                        _ = timer => return Err(idle_timeout.error(provider_name)),
                    }
                }
                None => read.await.map_err(|e| RequestError::Other(e.into()))?,
            };

            match serde_json::from_str::<ResponseSummary>(&body) {
                Ok(response_summary) => {
                    let events = vec![
                        StreamEvent::Created {
                            response: response_summary.clone(),
                        },
                        StreamEvent::InProgress {
                            response: response_summary.clone(),
                        },
                    ];

                    let mut all_events = events;
                    for (output_index, item) in response_summary.output.iter().enumerate() {
                        all_events.push(StreamEvent::OutputItemAdded {
                            output_index,
                            sequence_number: None,
                            item: item.clone(),
                        });

                        match item {
                            ResponseOutputItem::Message(message) => {
                                for content_item in &message.content {
                                    if let Some(text) = content_item.get("text") {
                                        if let Some(text_str) = text.as_str() {
                                            if let Some(ref item_id) = message.id {
                                                all_events.push(StreamEvent::OutputTextDelta {
                                                    item_id: item_id.clone(),
                                                    output_index,
                                                    content_index: None,
                                                    delta: text_str.to_string(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                            ResponseOutputItem::FunctionCall(function_call) => {
                                if let Some(ref item_id) = function_call.id {
                                    all_events.push(StreamEvent::FunctionCallArgumentsDone {
                                        item_id: item_id.clone(),
                                        output_index,
                                        arguments: function_call.arguments.clone(),
                                        sequence_number: None,
                                    });
                                }
                            }
                            ResponseOutputItem::Reasoning(reasoning) => {
                                if let Some(ref item_id) = reasoning.id {
                                    for part in &reasoning.summary {
                                        if let ReasoningSummaryPart::SummaryText { text } = part {
                                            all_events.push(
                                                StreamEvent::ReasoningSummaryTextDelta {
                                                    item_id: item_id.clone(),
                                                    output_index,
                                                    delta: text.clone(),
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                            ResponseOutputItem::Unknown => {}
                        }

                        all_events.push(StreamEvent::OutputItemDone {
                            output_index,
                            sequence_number: None,
                            item: item.clone(),
                        });
                    }

                    let status = response_summary.status.clone();
                    all_events.push(match status.as_deref() {
                        Some("incomplete") => StreamEvent::Incomplete {
                            response: response_summary,
                        },
                        Some("failed") => StreamEvent::Failed {
                            response: response_summary,
                        },
                        _ => StreamEvent::Completed {
                            response: response_summary,
                        },
                    });

                    Ok(futures::stream::iter(all_events.into_iter().map(Ok)).boxed())
                }
                Err(error) => {
                    log::error!(
                        "Failed to parse OpenAI non-streaming response: `{}`\nResponse: `{}`",
                        error,
                        body,
                    );
                    Err(RequestError::Other(anyhow!(error)))
                }
            }
        }
    } else {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|e| RequestError::Other(e.into()))?;

        Err(RequestError::HttpResponseError {
            provider: provider_name.to_owned(),
            status_code: response.status(),
            body,
            headers: response.headers().clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_client::FakeHttpClient;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Returns a timer factory whose timers never fire for the first
    /// `fire_from_call` calls and fire immediately afterwards. Useful for
    /// deterministically controlling when the idle timeout elapses.
    fn timer_factory(
        fire_from_call: usize,
    ) -> Arc<dyn Fn(Duration) -> BoxFuture<'static, ()> + Send + Sync> {
        let calls = Arc::new(AtomicUsize::new(0));
        Arc::new(move |_duration| {
            let call = calls.fetch_add(1, Ordering::SeqCst);
            if call >= fire_from_call {
                futures::future::ready(()).boxed()
            } else {
                futures::future::pending().boxed()
            }
        })
    }

    fn idle_timeout(
        make_timer: Arc<dyn Fn(Duration) -> BoxFuture<'static, ()> + Send + Sync>,
    ) -> StreamIdleTimeout {
        StreamIdleTimeout {
            duration: Duration::from_secs(300),
            make_timer,
        }
    }

    fn test_request() -> Request {
        Request {
            model: "gpt-test".into(),
            instructions: None,
            input: Vec::new(),
            include: Vec::new(),
            stream: true,
            temperature: None,
            top_p: None,
            max_output_tokens: None,
            parallel_tool_calls: None,
            tool_choice: None,
            tools: Vec::new(),
            prompt_cache_key: None,
            reasoning: None,
            store: None,
            service_tier: None,
        }
    }

    fn assert_idle_timeout_error(error: &anyhow::Error) {
        match error.downcast_ref::<RequestError>() {
            Some(RequestError::StreamIdleTimeout { .. }) => {}
            other => panic!("expected StreamIdleTimeout, got {other:?}"),
        }
    }

    #[test]
    fn idle_timeout_stream_fails_when_stream_stalls() {
        futures::executor::block_on(async {
            // One event arrives, then the stream stalls forever. The initial
            // timer (call 0) never fires; the reset timer (call 1) fires
            // immediately, simulating the idle window elapsing after the
            // first event.
            let inner = futures::stream::iter(vec![Ok(StreamEvent::Unknown)])
                .chain(futures::stream::pending())
                .boxed();
            let mut stream = IdleTimeoutStream::new(
                "test-provider".to_owned(),
                inner,
                idle_timeout(timer_factory(1)),
            );

            assert!(matches!(stream.next().await, Some(Ok(_))));
            let error = stream
                .next()
                .await
                .expect("expected an error item")
                .expect_err("expected the stream to fail");
            assert_idle_timeout_error(&error);
            assert!(
                stream.next().await.is_none(),
                "stream should end after timing out"
            );
        });
    }

    #[test]
    fn idle_timeout_stream_prefers_data_over_timer() {
        futures::executor::block_on(async {
            // Even with timers that fire immediately, available events are
            // always delivered first.
            let inner = futures::stream::iter(vec![
                Ok(StreamEvent::Unknown),
                Ok(StreamEvent::Unknown),
                Ok(StreamEvent::Unknown),
            ])
            .boxed();
            let mut stream = IdleTimeoutStream::new(
                "test-provider".to_owned(),
                inner,
                idle_timeout(timer_factory(0)),
            );

            for _ in 0..3 {
                assert!(matches!(stream.next().await, Some(Ok(_))));
            }
            assert!(stream.next().await.is_none());
        });
    }

    #[test]
    fn stream_response_times_out_when_no_response_headers_arrive() {
        futures::executor::block_on(async {
            // The server accepts the request but never sends response headers.
            let client = FakeHttpClient::create(move |_request| async move {
                std::future::pending::<()>().await;
                Err(anyhow!("unreachable"))
            });

            let result = stream_response_with_idle_timeout(
                &*client,
                "test-provider",
                "https://test.example",
                "test-key",
                test_request(),
                &CustomHeaders::default(),
                Some(idle_timeout(timer_factory(0))),
            )
            .await;

            match result {
                Err(RequestError::StreamIdleTimeout { .. }) => {}
                other => panic!("expected StreamIdleTimeout, got {:?}", other.map(|_| ())),
            }
        });
    }

    #[test]
    fn stream_response_succeeds_when_headers_arrive_before_timeout() {
        futures::executor::block_on(async {
            let client = FakeHttpClient::create(move |_request| async move {
                let body = concat!(
                    "data: {\"type\":\"response.completed\",\"response\":{\"id\":\"resp_1\",\"status\":\"completed\"}}\n",
                    "\n",
                    "data: [DONE]\n",
                );
                Ok(http_client::Response::builder()
                    .status(200)
                    .body(AsyncBody::from(body))?)
            });

            let stream = stream_response_with_idle_timeout(
                &*client,
                "test-provider",
                "https://test.example",
                "test-key",
                test_request(),
                &CustomHeaders::default(),
                // The initial timer never fires; timers created after events
                // arrive fire immediately, but events are always preferred.
                Some(idle_timeout(timer_factory(1))),
            )
            .await
            .expect("request should succeed");

            let events = stream.collect::<Vec<_>>().await;
            assert_eq!(events.len(), 1);
            assert!(matches!(events[0], Ok(StreamEvent::Completed { .. })));
        });
    }
}
