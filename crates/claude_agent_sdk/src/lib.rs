//! Claude Agent SDK for Zed
//!
//! This crate provides an agentic Claude integration for Zed IDE,
//! enabling Claude to have native access to editor tools, buffers,
//! LSP diagnostics, and more.

mod agent;
mod provider;
mod tools;

pub use agent::*;
pub use provider::*;
pub use tools::*;

use language_model::{LanguageModelProviderId, LanguageModelProviderName};

/// Provider ID for Claude Agent SDK
pub const CLAUDE_AGENT_PROVIDER_ID: LanguageModelProviderId =
    LanguageModelProviderId::new("claude_agent");

/// Provider name for Claude Agent SDK
pub const CLAUDE_AGENT_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Claude Agent");

/// Agent SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
