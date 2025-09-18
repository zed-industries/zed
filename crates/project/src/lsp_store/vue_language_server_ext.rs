use anyhow::Context as _;
use gpui::{AppContext, WeakEntity};
use lsp::{LanguageServer, LanguageServerName};
use serde_json::json;

use crate::LspStore;

struct VueServerRequest;
struct TypescriptServerResponse;

impl lsp::notification::Notification for VueServerRequest {
    type Params = [(u64, String, serde_json::Value); 1];

    const METHOD: &'static str = "tsserver/request";
}

impl lsp::notification::Notification for TypescriptServerResponse {
    type Params = serde_json::Value;

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
                let this = lsp_store.clone();
                move |params, cx| {
                    let this = this.clone();
                    let Ok(target_server) = this
                        .read_with(cx, |this, _| {
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
                        })
                        .flatten()
                    else {
                        return;
                    };
                    let Some(vue_server) = this
                        .read_with(cx, |this, _| this.language_server_for_id(vue_server_id))
                        .ok()
                        .flatten()
                    else {
                        return;
                    };
                    let cx = cx.clone();
                    cx.background_spawn(async move {
                        let (request_id, command, arguments) = params[0].clone();
                        let tsserver_response = target_server
                            .request::<lsp::request::ExecuteCommand>(lsp::ExecuteCommandParams {
                                command: "typescript.tsserverRequest".to_owned(),
                                arguments: vec![serde_json::Value::String(command), arguments],
                                ..Default::default()
                            })
                            .await;
                        if let util::ConnectionResult::Result(Ok(result)) = tsserver_response {
                            _ = vue_server.notify::<TypescriptServerResponse>(
                                &json!({ "id": request_id, "response": result }),
                            );
                        }
                    })
                    .detach();
                }
            })
            .detach();
    }
}
