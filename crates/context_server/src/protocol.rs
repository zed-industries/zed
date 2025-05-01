//! This module implements parts of the Model Context Protocol.
//!
//! It handles the lifecycle messages, and provides a general interface to
//! interacting with an MCP server. It uses the generic JSON-RPC client to
//! read/write messages and the types from types.rs for serialization/deserialization
//! of messages.

use anyhow::Result;
use collections::HashMap;

use crate::client::Client;
use crate::types;

pub struct ModelContextProtocol {
    inner: Client,
}

impl ModelContextProtocol {
    pub fn new(inner: Client) -> Self {
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

        println!("Sending initialize request");
        let response: types::InitializeResponse = self
            .inner
            .request(types::RequestType::Initialize.as_str(), params)
            .await?;
        println!("Got initialize request");

        if !Self::supported_protocols().contains(&response.protocol_version) {
            return Err(anyhow::anyhow!(
                "Unsupported protocol version: {:?}",
                response.protocol_version
            ));
        }

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

    fn check_capability(&self, capability: ServerCapability) -> Result<()> {
        if self.capable(capability) {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Server does not support {:?} capability",
                capability
            ))
        }
    }

    /// List the MCP prompts.
    pub async fn list_prompts(&self) -> Result<Vec<types::Prompt>> {
        self.check_capability(ServerCapability::Prompts)?;

        let response: types::PromptsListResponse = self
            .inner
            .request(
                types::RequestType::PromptsList.as_str(),
                serde_json::json!({}),
            )
            .await?;

        Ok(response.prompts)
    }

    /// List the MCP resources.
    pub async fn list_resources(&self) -> Result<types::ResourcesListResponse> {
        self.check_capability(ServerCapability::Resources)?;

        let response: types::ResourcesListResponse = self
            .inner
            .request(
                types::RequestType::ResourcesList.as_str(),
                serde_json::json!({}),
            )
            .await?;

        Ok(response)
    }

    /// Executes a prompt with the given arguments and returns the result.
    pub async fn run_prompt<P: AsRef<str>>(
        &self,
        prompt: P,
        arguments: HashMap<String, String>,
    ) -> Result<types::PromptsGetResponse> {
        self.check_capability(ServerCapability::Prompts)?;

        let params = types::PromptsGetParams {
            name: prompt.as_ref().to_string(),
            arguments: Some(arguments),
            meta: None,
        };

        let response: types::PromptsGetResponse = self
            .inner
            .request(types::RequestType::PromptsGet.as_str(), params)
            .await?;

        Ok(response)
    }

    pub async fn completion<P: Into<String>>(
        &self,
        reference: types::CompletionReference,
        argument: P,
        value: P,
    ) -> Result<types::Completion> {
        let params = types::CompletionCompleteParams {
            r#ref: reference,
            argument: types::CompletionArgument {
                name: argument.into(),
                value: value.into(),
            },
            meta: None,
        };
        let result: types::CompletionCompleteResponse = self
            .inner
            .request(types::RequestType::CompletionComplete.as_str(), params)
            .await?;

        let completion = types::Completion {
            values: result.completion.values,
            total: types::CompletionTotal::from_options(
                result.completion.has_more,
                result.completion.total,
            ),
        };

        Ok(completion)
    }

    /// List MCP tools.
    pub async fn list_tools(&self) -> Result<types::ListToolsResponse> {
        self.check_capability(ServerCapability::Tools)?;

        let response = self
            .inner
            .request::<types::ListToolsResponse>(types::RequestType::ListTools.as_str(), ())
            .await?;

        Ok(response)
    }

    /// Executes a tool with the given arguments
    pub async fn run_tool<P: AsRef<str>>(
        &self,
        tool: P,
        arguments: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<types::CallToolResponse> {
        self.check_capability(ServerCapability::Tools)?;

        let params = types::CallToolParams {
            name: tool.as_ref().to_string(),
            arguments,
            meta: None,
        };

        let response: types::CallToolResponse = self
            .inner
            .request(types::RequestType::CallTool.as_str(), params)
            .await?;

        Ok(response)
    }
}

impl InitializedContextServerProtocol {
    pub async fn request<R: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: impl serde::Serialize,
    ) -> Result<R> {
        self.inner.request(method, params).await
    }
}
