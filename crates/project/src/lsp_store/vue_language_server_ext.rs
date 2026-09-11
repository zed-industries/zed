use std::sync::Arc;

use gpui::{AppContext, AsyncApp, WeakEntity};
use lsp::{LanguageServer, LanguageServerName};
use serde_json::Value;

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

pub fn register_requests(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    let language_server_name = language_server.name();
    if language_server_name != VUE_SERVER_NAME {
        return;
    }

    let vue_server_id = language_server.server_id();
    language_server
        .on_notification::<VueServerRequest, _>({
            move |params, cx| {
                let lsp_store = lsp_store.clone();

                cx.spawn(async move |cx| {
                    let Ok(Some(vue_server)) = lsp_store.update(cx, |this, _| {
                        this.language_server_for_id(vue_server_id)
                    }) else {
                        return;
                    };

                    // The TypeScript server may not be running yet (Zed starts
                    // vtsls lazily). Replying null immediately would poison
                    // vue-language-server's per-file project cache
                    // (file2ProjectInfo), so wait for it to appear instead.
                    let Some(target_server) =
                        wait_for_typescript_server(&lsp_store, cx).await
                    else {
                        log::warn!(
                            "vue-language-server forwarding skipped: no TypeScript server \
                             appeared in time; returning null tsserver responses"
                        );
                        if !params.is_empty() {
                            let null_responses = params
                                .into_iter()
                                .map(|(id, _, _)| (id, Value::Null))
                                .collect::<Vec<_>>();
                            let _ = vue_server.notify::<TypescriptServerResponse>(null_responses);
                        }
                        return;
                    };

                    let request_timeout = cx.update(|app| {
                        ProjectSettings::get_global(app)
                            .global_lsp_settings
                            .get_request_timeout()
                    });

                    for (request_id, command, payload) in params.into_iter() {
                        let target_server = target_server.clone();
                        let vue_server = vue_server.clone();
                        cx.background_spawn(async move {
                            // tsserver may still be starting up when the first
                            // `_vue:` requests arrive (project not loaded yet,
                            // plugin not activated). Forwarding such a failure
                            // would poison vue-language-server's per-file
                            // project cache (file2ProjectInfo), so retry until
                            // tsserver is ready before giving up.
                            const MAX_RETRIES: usize = 60;
                            const RETRY_DELAY: std::time::Duration =
                                std::time::Duration::from_millis(500);

                            let mut response = None;
                            for _ in 0..MAX_RETRIES {
                                let attempt = target_server
                                    .request::<lsp::request::ExecuteCommand>(
                                        lsp::ExecuteCommandParams {
                                            command: "typescript.tsserverRequest".to_owned(),
                                            arguments: vec![
                                                Value::String(command.clone()),
                                                payload.clone(),
                                            ],
                                            ..Default::default()
                                        },
                                        request_timeout,
                                    )
                                    .await;

                                let retryable = matches!(
                                    &attempt,
                                    util::ConnectionResult::Result(Err(_))
                                );
                                response = Some(attempt);
                                if !retryable {
                                    break;
                                }
                                smol::Timer::after(RETRY_DELAY).await;
                            }

                            let Some(response) = response else {
                                return;
                            };
                            let response_body = match response {
                                util::ConnectionResult::Result(Ok(result)) => match result {
                                    Some(Value::Object(mut map)) => {
                                        map.remove("body").unwrap_or(Value::Null)
                                    }
                                    Some(_) => Value::Null,
                                    None => Value::Null,
                                },
                                util::ConnectionResult::Result(Err(error)) => {
                                    log::warn!(
                                        "typescript.tsserverRequest failed: {error:?} for request {request_id}"
                                    );
                                    Value::Null
                                }
                                other => {
                                    log::warn!(
                                        "typescript.tsserverRequest did not return a response: {other:?} for request {request_id}"
                                    );
                                    Value::Null
                                }
                            };

                            if let Err(err) = vue_server.notify::<TypescriptServerResponse>(vec![(
                                request_id,
                                response_body,
                            )]) {
                                log::warn!(
                                    "Failed to notify vue-language-server of tsserver response: {err:?}"
                                );
                            }
                        })
                        .detach();
                    }
                })
                .detach();
            }
        })
        .detach();
}

async fn wait_for_typescript_server(
    lsp_store: &WeakEntity<LspStore>,
    cx: &mut AsyncApp,
) -> Option<Arc<LanguageServer>> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
    loop {
        let typescript_server_id = lsp_store
            .update(cx, |this, _| {
                this.as_local().and_then(|local| {
                    local
                        .language_server_ids
                        .iter()
                        .find_map(|(seed, v)| [VTSLS, TS_LS].contains(&seed.name).then_some(v.id))
                })
            })
            .ok()
            .flatten();

        if let Some(typescript_server_id) = typescript_server_id {
            if let Some(server) = lsp_store
                .update(cx, |this, _| {
                    this.language_server_for_id(typescript_server_id)
                })
                .ok()
                .flatten()
            {
                return Some(server);
            }
        }

        if std::time::Instant::now() > deadline {
            return None;
        }
        cx.background_executor()
            .timer(std::time::Duration::from_millis(250))
            .await;
    }
}
