use anyhow::Context as _;
use gpui::WeakEntity;
use lsp::{LanguageServer, LanguageServerName};
use serde::{Deserialize, Serialize};

use crate::LspStore;

struct TypescriptServerRequest;
struct TypescriptServerResponse;

#[derive(Serialize, Deserialize)]
struct ServerRequestParams {
    command: String,
    arguments: ServerRequestArguments,
}

#[derive(Serialize, Deserialize)]
struct ServerRequestArguments {}

impl lsp::request::Request for TypescriptServerRequest {
    type Params = ServerRequestParams;

    type Result = ();

    const METHOD: &'static str = "tsserver/request";
}

impl lsp::request::Request for TypescriptServerResponse {
    type Params = serde_json::Value;

    type Result = ();

    const METHOD: &'static str = "tsserver/response";
}

const VUE_SERVER_NAME: LanguageServerName = LanguageServerName::new_static("vue-language-server");
const VTSLS: LanguageServerName = LanguageServerName::new_static("vtsls");
const TS_LS: LanguageServerName = LanguageServerName::new_static("typescript-language-server");

pub fn register_requests(lsp_store: WeakEntity<LspStore>, language_server: &LanguageServer) {
    let language_server_name = language_server.name();
    if language_server_name == VUE_SERVER_NAME {
        language_server
            .on_request::<TypescriptServerRequest, _, _>({
                let this = lsp_store.clone();
                move |params, cx| {
                    let this = this.clone();
                    let cx = cx.clone();
                    async move {
                        let language_server = this
                            .read_with(&cx, |this, _| {
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
                            .flatten()?;
                        language_server
                            .request::<TypescriptServerRequest>(params)
                            .await
                            .into_response()
                    }
                }
            })
            .detach();
    } else if language_server_name == VTSLS || language_server_name == TS_LS {
        language_server
            .on_request::<TypescriptServerResponse, _, _>({
                let this = lsp_store.clone();
                move |params, cx| {
                    let this = this.clone();
                    let cx = cx.clone();
                    async move {
                        let language_server = this
                            .read_with(&cx, |this, _| {
                                let language_server_id = this
                                    .as_local()
                                    .and_then(|local| {
                                        local.language_server_ids.iter().find_map(|(seed, v)| {
                                            // todo: improve this
                                            [VUE_SERVER_NAME].contains(&seed.name).then_some(v.id)
                                        })
                                    })
                                    .context("Could not find language server")?;

                                this.language_server_for_id(language_server_id)
                                    .context("language server not found")
                            })
                            .flatten()?;
                        language_server
                            .request::<TypescriptServerResponse>(params)
                            .await
                            .into_response()
                    }
                }
            })
            .detach();
    }
}
