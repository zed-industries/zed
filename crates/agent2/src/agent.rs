//! Agent implementation for the agent-client-protocol
//!
//! Implementation Status:
//! - [x] initialize: Complete - Basic protocol handshake
//! - [x] authenticate: Complete - Accepts any auth (stub)
//! - [~] new_session: Partial - Creates session ID but Thread creation needs GPUI context
//! - [~] load_session: Stub - Returns not implemented
//! - [ ] prompt: Stub - Needs GPUI context and type conversions
//! - [~] cancelled: Partial - Removes session from map but needs GPUI cleanup

use agent_client_protocol as acp;
use gpui::Entity;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::Arc;

use crate::{templates::Templates, Thread};

pub struct Agent {
    /// Session ID -> Thread entity mapping
    sessions: RefCell<HashMap<acp::SessionId, Entity<Thread>>>,
    /// Shared templates for all threads
    templates: Arc<Templates>,
    /// Current protocol version we support
    protocol_version: acp::ProtocolVersion,
    /// Authentication state
    authenticated: Cell<bool>,
}

impl Agent {
    pub fn new(templates: Arc<Templates>) -> Self {
        Self {
            sessions: RefCell::new(HashMap::new()),
            templates,
            protocol_version: acp::VERSION,
            authenticated: Cell::new(false),
        }
    }
}

impl acp::Agent for Agent {
    /// COMPLETE: Initialize handshake with client
    async fn initialize(
        &self,
        arguments: acp::InitializeRequest,
    ) -> Result<acp::InitializeResponse, acp::Error> {
        // For now, we just use the client's requested version
        let response_version = arguments.protocol_version.clone();

        Ok(acp::InitializeResponse {
            protocol_version: response_version,
            agent_capabilities: acp::AgentCapabilities::default(),
            auth_methods: vec![
                // STUB: No authentication required for now
                acp::AuthMethod {
                    id: acp::AuthMethodId("none".into()),
                    label: "No Authentication".to_string(),
                    description: Some("No authentication required".to_string()),
                },
            ],
        })
    }

    /// COMPLETE: Handle authentication (currently just accepts any auth)
    async fn authenticate(&self, _arguments: acp::AuthenticateRequest) -> Result<(), acp::Error> {
        // STUB: Accept any authentication method for now
        self.authenticated.set(true);
        Ok(())
    }

    /// PARTIAL: Create a new session
    async fn new_session(
        &self,
        arguments: acp::NewSessionRequest,
    ) -> Result<acp::NewSessionResponse, acp::Error> {
        // Check if authenticated
        if !self.authenticated.get() {
            return Ok(acp::NewSessionResponse { session_id: None });
        }

        // STUB: Generate a simple session ID
        let session_id = acp::SessionId(format!("session-{}", uuid::Uuid::new_v4()).into());

        // Create a new Thread for this session
        // TODO: This needs to be done on the main thread with proper GPUI context
        // For now, we'll return the session ID and expect the actual Thread creation
        // to happen when we have access to a GPUI context

        // STUB: MCP server support not implemented
        if !arguments.mcp_servers.is_empty() {
            log::warn!("MCP servers requested but not yet supported");
        }

        Ok(acp::NewSessionResponse {
            session_id: Some(session_id),
        })
    }

    /// STUB: Load existing session
    async fn load_session(
        &self,
        _arguments: acp::LoadSessionRequest,
    ) -> Result<acp::LoadSessionResponse, acp::Error> {
        // STUB: Session persistence not implemented
        Ok(acp::LoadSessionResponse {
            auth_required: !self.authenticated.get(),
            auth_methods: if self.authenticated.get() {
                vec![]
            } else {
                vec![acp::AuthMethod {
                    id: acp::AuthMethodId("none".into()),
                    label: "No Authentication".to_string(),
                    description: Some("No authentication required".to_string()),
                }]
            },
        })
    }

    /// STUB: Handle prompts
    async fn prompt(&self, arguments: acp::PromptRequest) -> Result<(), acp::Error> {
        // TODO: This needs to be implemented with proper GPUI context access
        // The implementation would:
        // 1. Look up the Thread for this session
        // 2. Convert acp::ContentBlock to agent2 message format
        // 3. Call thread.send() with the converted message
        // 4. Stream responses back to the client

        let _session_id = arguments.session_id;
        let _prompt = arguments.prompt;

        // STUB: Just acknowledge receipt for now
        log::info!("Received prompt for session: {}", _session_id.0);

        Err(acp::Error::internal_error().with_data("Prompt handling not yet implemented"))
    }

    /// PARTIAL: Handle cancellation
    async fn cancelled(&self, args: acp::CancelledNotification) -> Result<(), acp::Error> {
        // Remove the session from our map
        let removed = self.sessions.borrow_mut().remove(&args.session_id);

        if removed.is_some() {
            // TODO: Properly clean up the Thread entity when we have GPUI context
            log::info!("Session {} cancelled and removed", args.session_id.0);
            Ok(())
        } else {
            Err(acp::Error::invalid_request()
                .with_data(format!("Session {} not found", args.session_id.0)))
        }
    }
}

// Helper functions for type conversions between acp and agent2 types

/// Convert acp::ContentBlock to agent2 message format
/// STUB: Needs implementation
fn convert_content_block(_block: acp::ContentBlock) -> String {
    // TODO: Implement proper conversion
    // This would handle:
    // - Text content
    // - Resource links
    // - Images
    // - Audio
    // - Other content types
    "".to_string()
}

/// Convert agent2 messages to acp format for responses
/// STUB: Needs implementation
fn convert_to_acp_content(_content: &str) -> Vec<acp::ContentBlock> {
    // TODO: Implement proper conversion
    vec![]
}
