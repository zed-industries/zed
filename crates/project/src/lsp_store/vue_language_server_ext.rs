use anyhow::Context as _;
use gpui::{AppContext, WeakEntity};
use lsp::{LanguageServer, LanguageServerName};
use serde_json::Value;

use crate::LspStore;

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
    if language_server_name == VUE_SERVER_NAME {
        let vue_server_id = language_server.server_id();
        language_server
            .on_notification::<VueServerRequest, _>({
                move |params, cx| {
                    let lsp_store = lsp_store.clone();
                    let Ok(Some(vue_server)) = lsp_store.read_with(cx, |this, _| {
                        this.language_server_for_id(vue_server_id)
                    }) else {
                        return;
                    };

                    let requests = params;
                    let target_server = match lsp_store.read_with(cx, |this, _| {
                        let language_server_id = this
                            .as_local()
                            .and_then(|local| {
                                local.language_server_ids.iter().find_map(|(seed, v)| {
                                    [VTSLS, TS_LS].contains(&seed.name).then_some(v.id)
                                })
                            })
                            .context("Could not find language server")?;

                        this.language_server_for_id(language_server_id)
                            .context("language server not found")
                    }) {
                        Ok(Ok(server)) => server,
                        other => {
                            log::warn!(
                                "vue-language-server forwarding skipped: {other:?}. \
                                 Returning null tsserver responses"
                            );
                            if !requests.is_empty() {
                                let null_responses = requests
                                    .into_iter()
                                    .map(|(id, _, _)| (id, Value::Null))
                                    .collect::<Vec<_>>();
                                let _ = vue_server
                                    .notify::<TypescriptServerResponse>(null_responses);
                            }
                            return;
                        }
                    };

                    let cx = cx.clone();
                    for (request_id, command, payload) in requests.into_iter() {
                        let target_server = target_server.clone();
                        let vue_server = vue_server.clone();
                        cx.background_spawn(async move {
                            let response = target_server
                                .request::<lsp::request::ExecuteCommand>(
                                    lsp::ExecuteCommandParams {
                                        command: "typescript.tsserverRequest".to_owned(),
                                        arguments: vec![Value::String(command), payload],
                                        ..Default::default()
                                    },
                                )
                                .await;

                            let response_body = match response {
                                util::ConnectionResult::Result(Ok(result)) => match result {
                                    Some(Value::Object(mut map)) => map
                                        .remove("body")
                                        .unwrap_or(Value::Object(map)),
                                    Some(other) => other,
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

                            if let Err(err) = vue_server
                                .notify::<TypescriptServerResponse>(vec![(request_id, response_body)])
                            {
                                log::warn!(
                                    "Failed to notify vue-language-server of tsserver response: {err:?}"
                                );
                            }
                        })
                        .detach();
                    }
                }
            })
            .detach();
    }
}
