//! This module implements parts of the Model Context Protocol.
//!
//! It handles the lifecycle messages, and provides a general interface to
//! interacting with an MCP server. It uses the generic JSON-RPC client to
//! read/write messages and the types from types.rs for serialization/deserialization
//! of messages.

use anyhow::Result;

use crate::client::Client;
use crate::types::{self, Request};

pub struct ModelContextProtocol {
    inner: Client,
}

impl ModelContextProtocol {
    pub(crate) fn new(inner: Client) -> Self {
        Self { inner }
    }

    fn supported_protocols() -> Vec<types::ProtocolVersion> {
        vec![types::ProtocolVersion(
            types::LATEST_PROTOCOL_VERSION.to_string(),
        )]
    }

    pub async fn initialize(
        self,
        client_info: types::Implementation,
    ) -> Result<InitializedContextServerProtocol> {
        let params = types::InitializeParams {
            protocol_version: types::ProtocolVersion(types::LATEST_PROTOCOL_VERSION.to_string()),
            capabilities: types::ClientCapabilities {
                experimental: None,
                sampling: None,
                roots: None,
            },
            meta: None,
            client_info,
        };

        let response: types::InitializeResponse = self
            .inner
            .request(types::request::Initialize::METHOD, params)
            .await?;

        anyhow::ensure!(
            Self::supported_protocols().contains(&response.protocol_version),
            "Unsupported protocol version: {:?}",
            response.protocol_version
        );

        log::trace!("mcp server info {:?}", response.server_info);

        self.inner.notify(
            types::NotificationType::Initialized.as_str(),
            serde_json::json!({}),
        )?;

        let initialized_protocol = InitializedContextServerProtocol {
            inner: self.inner,
            initialize: response,
        };

        Ok(initialized_protocol)
    }
}

pub struct InitializedContextServerProtocol {
    inner: Client,
    pub initialize: types::InitializeResponse,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ServerCapability {
    Experimental,
    Logging,
    Prompts,
    Resources,
    Tools,
}

impl InitializedContextServerProtocol {
    /// Check if the server supports a specific capability
    pub fn capable(&self, capability: ServerCapability) -> bool {
        match capability {
            ServerCapability::Experimental => self.initialize.capabilities.experimental.is_some(),
            ServerCapability::Logging => self.initialize.capabilities.logging.is_some(),
            ServerCapability::Prompts => self.initialize.capabilities.prompts.is_some(),
            ServerCapability::Resources => self.initialize.capabilities.resources.is_some(),
            ServerCapability::Tools => self.initialize.capabilities.tools.is_some(),
        }
    }

    pub async fn request<T: Request>(&self, params: T::Params) -> Result<T::Response> {
        self.inner.request(T::METHOD, params).await
    }
}
