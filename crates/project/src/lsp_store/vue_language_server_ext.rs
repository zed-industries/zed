use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::Context as _;
use gpui::{AppContext, WeakEntity};
use lsp::{LanguageServer, LanguageServerName};
use serde_json::Value;

use super::LspStoreEvent;
use crate::{LspStore, ProjectSettings};
use settings::Settings;

struct VueServerRequest;
struct TypescriptServerResponse;

impl lsp::notification::Notification for VueServerRequest {
    type Params = Vec<(u64, String, serde_json::Value)>;

    const METHOD: &'static str = "tsserver/request";
}

impl lsp::notification::Notification for TypescriptServerResponse {
    type Params = Vec<(u64, serde_json::Value)>;

    const METHOD: &'static str = "tsserver/response";
}

const VUE_SERVER_NAME: LanguageServerName = LanguageServerName::new_static("vue-language-server");
const VTSLS: LanguageServerName = LanguageServerName::new_static("vtsls");
const TS_LS: LanguageServerName = LanguageServerName::new_static("typescript-language-server");

const TS_SERVER_READY_TIMEOUT: Duration = Duration::from_secs(5);

struct PendingBridgeQueue {
    requests: Vec<(u64, String, serde_json::Value)>,
    vue_server: Arc<LanguageServer>,
}

fn find_ts_server_id(
    lsp_store: &WeakEntity<LspStore>,
    cx: &impl AppContext,
) -> Option<lsp::LanguageServerId> {
    lsp_store
        .read_with(cx, |this, _| {
            this.as_local().and_then(|local| {
                local
                    .language_server_ids
                    .iter()
                    .find_map(|(seed, v)| [VTSLS, TS_LS].contains(&seed.name).then_some(v.id))
            })
        })
        .ok()
        .flatten()
}

fn find_running_ts_server(
    lsp_store: &WeakEntity<LspStore>,
    cx: &impl AppContext,
) -> anyhow::Result<Arc<LanguageServer>> {
    let server_id = find_ts_server_id(lsp_store, cx)
        .context("Could not find vtsls or typescript-language-server in language_server_ids")?;

    lsp_store.read_with(cx, |this, _| {
        this.language_server_for_id(server_id)
            .context("TS server exists in language_server_ids but is not Running")
    })?
}

fn forward_requests(
    requests: Vec<(u64, String, serde_json::Value)>,
    target_server: Arc<LanguageServer>,
    vue_server: Arc<LanguageServer>,
    request_timeout: Duration,
    cx: &gpui::AsyncApp,
) {
    for (request_id, command, payload) in requests {
        let target_server = target_server.clone();
        let vue_server = vue_server.clone();
        let command_name = command.clone();
        cx.background_spawn(async move {
            let response = target_server
                .request::<lsp::request::ExecuteCommand>(
                    lsp::ExecuteCommandParams {
                        command: "typescript.tsserverRequest".to_owned(),
                        arguments: vec![Value::String(command), payload],
                        ..Default::default()
                    },
                    request_timeout,
                )
                .await;

            let response_body = match response {
                util::ConnectionResult::Result(Ok(result)) => match result {
                    Some(Value::Object(mut map)) => {
                        map.remove("body").unwrap_or(Value::Object(map))
                    }
                    Some(other) => {
                        log::debug!(
                            "[vue-bridge] tsserver/request id={request_id} cmd={command_name} \
                            returned non-object: {}",
                            other
                        );
                        other
                    }
                    None => {
                        log::warn!(
                            "[vue-bridge] tsserver/request id={request_id} cmd={command_name} \
                            returned None"
                        );
                        Value::Null
                    }
                },
                util::ConnectionResult::Result(Err(error)) => {
                    log::warn!(
                        "[vue-bridge] tsserver/request id={request_id} cmd={command_name} \
                        failed: {error:?}"
                    );
                    Value::Null
                }
                other => {
                    log::warn!(
                        "[vue-bridge] tsserver/request id={request_id} cmd={command_name} \
                        no response: {other:?}"
                    );
                    Value::Null
                }
            };

            if let Err(error) =
                vue_server.notify::<TypescriptServerResponse>(vec![(request_id, response_body)])
            {
                log::warn!(
                    "[vue-bridge] Failed to send tsserver/response id={request_id}: {error:?}"
                );
            }
        })
        .detach();
    }
}

fn send_null_responses(
    requests: Vec<(u64, String, serde_json::Value)>,
    vue_server: &Arc<LanguageServer>,
) {
    if requests.is_empty() {
        return;
    }
    let null_responses = requests
        .into_iter()
        .map(|(id, _, _)| (id, Value::Null))
        .collect::<Vec<_>>();
    if let Err(error) = vue_server.notify::<TypescriptServerResponse>(null_responses) {
        log::warn!("Failed to send null tsserver responses: {error:?}");
    }
}

/// Flush queued requests if vtsls is now Running. Returns true if flushed.
fn try_flush_queue(
    pending_queue: &Arc<Mutex<Option<PendingBridgeQueue>>>,
    target_server: Arc<LanguageServer>,
    request_timeout: Duration,
    cx: &gpui::AsyncApp,
) -> bool {
    let Ok(mut guard) = pending_queue.lock() else {
        return false;
    };

    let Some(queue) = guard.take() else {
        return false;
    };

    log::info!(
        "[vue-bridge] Flushing {} queued request(s) to TS server",
        queue.requests.len()
    );
    forward_requests(
        queue.requests,
        target_server,
        queue.vue_server,
        request_timeout,
        cx,
    );
    true
}

/// vtsls is Running: flush any pending queue and forward requests directly.
fn handle_requests_with_running_ts_server(
    requests: Vec<(u64, String, serde_json::Value)>,
    target_server: Arc<LanguageServer>,
    vue_server: Arc<LanguageServer>,
    pending_queue: &Arc<Mutex<Option<PendingBridgeQueue>>>,
    cx: &gpui::AsyncApp,
) {
    let request_timeout = cx.update(|app| {
        ProjectSettings::get_global(app)
            .global_lsp_settings
            .get_request_timeout()
    });

    try_flush_queue(pending_queue, target_server.clone(), request_timeout, cx);
    forward_requests(requests, target_server, vue_server, request_timeout, cx);
}

/// vtsls is not Running: buffer requests and set up a subscription + timeout
/// to flush or discard them once vtsls starts (or doesn't).
#[allow(clippy::redundant_clone)]
fn buffer_requests_until_ts_server_ready(
    requests: Vec<(u64, String, serde_json::Value)>,
    vue_server: Arc<LanguageServer>,
    lsp_store: &WeakEntity<LspStore>,
    pending_queue: &Arc<Mutex<Option<PendingBridgeQueue>>>,
    subscription_registered: &Arc<AtomicBool>,
    cx: &mut gpui::AsyncApp,
) {
    let mut guard = match pending_queue.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    if let Some(queue) = guard.as_mut() {
        queue.requests.extend(requests);
        return;
    }

    log::debug!(
        "[vue-bridge] vtsls not Running, buffering {} request(s)",
        requests.len(),
    );

    *guard = Some(PendingBridgeQueue {
        requests,
        vue_server: vue_server.clone(),
    });

    drop(guard);

    if subscription_registered.load(Ordering::Relaxed) {
        return;
    }

    if let Some(lsp_store_entity) = lsp_store.upgrade() {
        cx.subscribe(&lsp_store_entity, {
            let pending_queue = pending_queue.clone();
            let lsp_store = lsp_store.clone();
            move |_entity, event, cx| {
                let LspStoreEvent::LanguageServerAdded(_, name, _) = event else {
                    return;
                };

                if *name != VTSLS && *name != TS_LS {
                    return;
                }

                let Ok(target_server) = find_running_ts_server(&lsp_store, cx) else {
                    return;
                };

                log::info!(
                    "[vue-bridge] {} reached Running (event-driven flush)",
                    name
                );

                let request_timeout = ProjectSettings::get_global(cx)
                    .global_lsp_settings
                    .get_request_timeout();

                let async_cx = cx.to_async();
                try_flush_queue(&pending_queue, target_server, request_timeout, &async_cx);
            }
        })
        .detach();

        subscription_registered.store(true, Ordering::Relaxed);
    }

    let pending_queue = pending_queue.clone();
    let vue_server = vue_server.clone();
    let background_executor = cx.background_executor().clone();
    cx.background_spawn(async move {
        background_executor.timer(TS_SERVER_READY_TIMEOUT).await;

        let Ok(mut guard) = pending_queue.lock() else {
            return;
        };

        let Some(queue) = guard.take() else {
            return;
        };

        log::warn!(
            "[vue-bridge] Timed out waiting {}s for vtsls to reach Running. \
                Sending null responses for {} queued request(s).",
            TS_SERVER_READY_TIMEOUT.as_secs(),
            queue.requests.len()
        );

        send_null_responses(queue.requests, &vue_server);
    })
    .detach();
}

pub fn register_requests(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    let language_server_name = language_server.name();
    if language_server_name != VUE_SERVER_NAME {
        return;
    }

    let vue_server_id = language_server.server_id();

    // Shared queue state:
    // - Some(queue) = Queuing phase (vtsls not yet Running, requests are buffered)
    // - None = Direct phase (vtsls is Running, or queue was drained by timeout)
    let pending_queue: Arc<Mutex<Option<PendingBridgeQueue>>> = Arc::new(Mutex::new(None));

    let subscription_registered: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    language_server
        .on_notification::<VueServerRequest, _>({
            move |params, cx| {
                let lsp_store = lsp_store.clone();
                let Ok(Some(vue_server)) =
                    lsp_store.read_with(cx, |this, _| this.language_server_for_id(vue_server_id))
                else {
                    return;
                };

                if let Ok(target_server) = find_running_ts_server(&lsp_store, cx) {
                    handle_requests_with_running_ts_server(
                        params,
                        target_server,
                        vue_server,
                        &pending_queue,
                        cx,
                    );
                } else {
                    buffer_requests_until_ts_server_ready(
                        params,
                        vue_server,
                        &lsp_store,
                        &pending_queue,
                        &subscription_registered,
                        cx,
                    );
                }
            }
        })
        .detach();
}
