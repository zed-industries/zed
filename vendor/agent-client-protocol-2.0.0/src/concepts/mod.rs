//! Core concepts for understanding and using agent-client-protocol.
//!
//! This module provides detailed explanations of the key concepts you need
//! to work effectively with the agent-client-protocol SDK. Read these in order for a progressive
//! introduction, or jump to specific topics as needed.
//!
//! # Table of Contents
//!
//! 1. [ACP Basics][`crate::concepts::acp_basics`] - The roles in the protocol: clients,
//!    agents, proxies, and conductors.
//!
//! 2. [Connections][`crate::concepts::connections`] - How to establish connections
//!    using link types and connection builders.
//!
//! 3. [Sessions][`crate::concepts::sessions`] - Creating and managing sessions for
//!    multi-turn conversations with agents.
//!
//! 4. [Callbacks][`crate::concepts::callbacks`] - Handling incoming messages with
//!    `on_receive_*` methods.
//!
//! 5. [Explicit Peers][`crate::concepts::peers`] - Using `_to` and `_from` variants
//!    when you need to specify which peer you're communicating with.
//!
//! 6. [Ordering][`crate::concepts::ordering`] - How the dispatch loop processes
//!    messages and what ordering guarantees you get.
//!
//! 7. [Proxies and Conductors][`crate::concepts::proxies`] - Building proxies that
//!    intercept and modify messages between clients and agents.
//!
//! 8. [Error Handling][`crate::concepts::error_handling`] - Protocol errors vs
//!    connection errors, and how to handle them.
//!
//! 9. [Request Cancellation][`crate::concepts::cancellation`] - Cancelling
//!    outstanding JSON-RPC requests cooperatively.

pub mod acp_basics;
pub mod callbacks;
pub mod cancellation;
pub mod connections;
pub mod error_handling;
pub mod ordering;
pub mod peers;
pub mod proxies;
pub mod sessions;
