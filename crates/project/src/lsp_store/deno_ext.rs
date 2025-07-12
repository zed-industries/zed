//! Deno-specific LSP extensions for handling virtual documents.
//!
//! Deno uses virtual documents for external dependencies (from jsr.io, deno.land, npm, etc).
//! These are accessed via the `deno:/` URI scheme and require a custom LSP request
//! to fetch their contents.

use crate::{
    lsp_command::LspCommand,
    lsp_store::{LanguageServerState, LspStore},
};
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use gpui::{App, AsyncApp, Entity, WeakEntity};
use language::Buffer;
use lsp::{AdapterServerCapabilities, LanguageServer, LanguageServerId};
use rpc::proto::{self, PeerId};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use text::BufferId;
use util::ConnectionResult;

/// Custom LSP request for fetching virtual text documents in Deno.
pub enum DenoVirtualTextDocument {}

impl lsp::request::Request for DenoVirtualTextDocument {
    type Params = VirtualTextDocumentParams;
    type Result = Option<String>;
    const METHOD: &'static str = "deno/virtualTextDocument";
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VirtualTextDocumentParams {
    #[serde(rename = "textDocument")]
    pub text_document: lsp::TextDocumentIdentifier,
}

/// Command for fetching Deno virtual text documents.
#[derive(Debug, Clone)]
pub struct FetchVirtualTextDocument {
    pub uri: lsp::Url,
}

#[async_trait(?Send)]
impl LspCommand for FetchVirtualTextDocument {
    type Response = Option<String>;
    type LspRequest = DenoVirtualTextDocument;
    type ProtoRequest = proto::DenoVirtualTextDocument;

    fn display_name(&self) -> &str {
        "Fetch Deno Virtual Document"
    }

    fn check_capabilities(&self, _capabilities: AdapterServerCapabilities) -> bool {
        true
    }

    fn to_lsp(
        &self,
        _path: &Path,
        _buffer: &Buffer,
        _language_server: &Arc<LanguageServer>,
        _cx: &App,
    ) -> Result<VirtualTextDocumentParams> {
        let params = VirtualTextDocumentParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: self.uri.clone(),
            },
        };

        Ok(params)
    }

    async fn response_from_lsp(
        self,
        message: Option<String>,
        _project: Entity<LspStore>,
        _buffer: Entity<Buffer>,
        _server_id: LanguageServerId,
        _cx: AsyncApp,
    ) -> Result<Option<String>> {
        Ok(message)
    }

    fn to_proto(&self, project_id: u64, buffer: &Buffer) -> proto::DenoVirtualTextDocument {
        proto::DenoVirtualTextDocument {
            project_id,
            buffer_id: buffer.remote_id().into(),
            uri: self.uri.to_string(),
        }
    }

    async fn from_proto(
        message: Self::ProtoRequest,
        _project: Entity<LspStore>,
        _buffer: Entity<Buffer>,
        _cx: AsyncApp,
    ) -> Result<Self> {
        let uri = lsp::Url::parse(&message.uri)
            .context("Invalid URI in DenoVirtualTextDocument request")?;
        Ok(Self { uri })
    }

    fn response_to_proto(
        response: Option<String>,
        _: &mut LspStore,
        _: PeerId,
        _: &clock::Global,
        _: &mut App,
    ) -> proto::DenoVirtualTextDocumentResponse {
        proto::DenoVirtualTextDocumentResponse {
            content: response.unwrap_or_default(),
        }
    }

    async fn response_from_proto(
        self,
        message: proto::DenoVirtualTextDocumentResponse,
        _project: Entity<LspStore>,
        _buffer: Entity<Buffer>,
        _cx: AsyncApp,
    ) -> Result<Option<String>> {
        Ok(if message.content.is_empty() {
            None
        } else {
            Some(message.content)
        })
    }

    fn buffer_id_from_proto(message: &proto::DenoVirtualTextDocument) -> Result<BufferId> {
        BufferId::new(message.buffer_id)
    }
}

/// Handle opening a Deno virtual document
pub async fn open_deno_virtual_document(
    lsp_store: WeakEntity<LspStore>,
    original_url: lsp::Url,
    language_server_id: LanguageServerId,
    _language_server_name: lsp::LanguageServerName,
    cx: &mut AsyncApp,
) -> Result<Entity<Buffer>> {
    // First check if we have the document cached
    let cached_buffer = lsp_store.update(cx, |lsp_store, cx| match lsp_store.as_local() {
        Some(local_lsp_store) => local_lsp_store
            .deno_virtual_documents
            .read(cx)
            .get_cached_buffer(&original_url),
        None => None,
    })?;

    if let Some(buffer) = cached_buffer {
        return Ok(buffer);
    }

    // For the LSP request, we should use the percent-encoded URL
    // The LSP expects URLs like: deno:/https/jsr.io/%40fresh/core/2.0.0-alpha.37/src/mod.ts
    let request_url = original_url.clone();

    // Get the language server directly
    let language_server = lsp_store.read_with(cx, |lsp_store, _| {
        lsp_store
            .as_local()?
            .language_servers
            .get(&language_server_id)
            .and_then(|state| match state {
                LanguageServerState::Running { server, .. } => Some(server.clone()),
                _ => None,
            })
    })?;

    let Some(language_server) = language_server else {
        return Err(anyhow!("Language server not found"));
    };

    // Make the request directly to the language server
    let content = match language_server
        .request::<DenoVirtualTextDocument>(VirtualTextDocumentParams {
            text_document: lsp::TextDocumentIdentifier {
                uri: request_url.clone(),
            },
        })
        .await
    {
        ConnectionResult::Result(Ok(Some(doc_content))) => Some(doc_content),
        ConnectionResult::Result(Ok(None)) => None,
        ConnectionResult::Result(Err(e)) => return Err(e),
        ConnectionResult::ConnectionReset => return Err(anyhow!("LSP connection reset")),
        ConnectionResult::Timeout => return Err(anyhow!("LSP request timed out")),
    };

    let Some(content) = content else {
        return Err(anyhow!(
            "Failed to fetch Deno virtual document. The document might not be cached by Deno yet."
        ));
    };

    // Get the language registry
    let language_registry =
        lsp_store.read_with(cx, |lsp_store, _| match lsp_store.as_local() {
            Some(local) => Ok(local.languages.clone()),
            None => Err(anyhow!("Not a local LSP store")),
        })??;

    // Store the virtual document as an in-memory buffer
    let buffer = lsp_store.update(cx, |lsp_store, cx| match lsp_store.as_local_mut() {
        Some(local_lsp_store) => {
            Ok(local_lsp_store
                .deno_virtual_documents
                .update(cx, |store, cx| {
                    store.store_virtual_document(
                        original_url.clone(),
                        content,
                        language_registry,
                        cx,
                    )
                }))
        }
        None => Err(anyhow!("Not a local LSP store")),
    })??;

    // Set the language based on file extension
    let file_name = original_url
        .path()
        .split('/')
        .next_back()
        .unwrap_or("file.ts");
    let language_name = if file_name.ends_with(".ts") || file_name.ends_with(".tsx") {
        "TypeScript"
    } else if file_name.ends_with(".js") || file_name.ends_with(".jsx") {
        "JavaScript"
    } else {
        "TypeScript" // Default to TypeScript for Deno
    };

    // Language detection is async, but we'll do it after returning the buffer
    let buffer_handle = buffer.clone();
    let languages = lsp_store.read_with(cx, |lsp_store, _| lsp_store.languages.clone())?;
    cx.spawn(async move |cx| {
        if let Ok(language) = languages.language_for_name(language_name).await {
            let _ = buffer_handle.update(cx, |buffer, cx| {
                buffer.set_language(Some(language), cx);
            });
        }
        Ok::<(), anyhow::Error>(())
    })
    .detach();

    Ok(buffer)
}
