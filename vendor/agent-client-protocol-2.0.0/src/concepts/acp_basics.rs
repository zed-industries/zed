//! The roles in ACP: clients, agents, proxies, and conductors.
//!
//! The Agent-Client Protocol defines how AI agents communicate with the
//! applications that use them. Understanding the four roles helps you choose
//! the right approach for what you're building.
//!
//! # Clients
//!
//! A **client** is an application that wants to use an AI agent. Examples include:
//!
//! - IDEs like VS Code or JetBrains
//! - Command-line tools
//! - Web applications
//! - Automation scripts
//!
//! Clients send prompts to agents and receive responses. They can also handle
//! requests from the agent (like permission requests or tool approvals).
//!
//! # Agents
//!
//! An **agent** is an AI-powered service that responds to prompts. Examples include:
//!
//! - Claude Code
//! - Custom agents built with language models
//!
//! Agents receive prompts, process them (typically by calling an LLM), and stream
//! back responses. They may also request permissions, invoke tools, or ask for
//! user input during processing.
//!
//! # Proxies
//!
//! A **proxy** sits between a client and an agent, intercepting and potentially
//! modifying messages in both directions. Proxies are useful for:
//!
//! - Adding tools via MCP (Model Context Protocol) servers
//! - Injecting system prompts or context
//! - Logging and debugging
//! - Filtering or transforming messages
//!
//! Proxies can be chained - you can have multiple proxies between a client and
//! an agent, each adding its own capabilities.
//!
//! # Conductors
//!
//! A **conductor** orchestrates a chain of proxies with a final agent. It:
//!
//! - Spawns and manages proxy processes
//! - Routes messages through the chain
//! - Handles initialization and shutdown
//!
//! The [`agent-client-protocol-conductor`] crate provides a conductor implementation. Most users
//! don't need to implement conductors themselves - they just configure which
//! proxies to use.
//!
//! # Message Flow
//!
//! Messages flow through the system like this:
//!
//! ```text
//! Client <-> Proxy 1 <-> Proxy 2 <-> ... <-> Agent
//!        ^                                ^
//!        |                                |
//!        +------ Conductor manages -------+
//! ```
//!
//! Each arrow represents a bidirectional connection. Requests flow toward the
//! agent, responses flow back toward the client, and notifications can flow
//! in either direction.
//!
//! # Next Steps
//!
//! Now that you understand the roles, see [Connections and Links](super::connections)
//! to learn how to establish connections in code.
//!
//! [`agent-client-protocol-conductor`]: https://crates.io/crates/agent-client-protocol-conductor
