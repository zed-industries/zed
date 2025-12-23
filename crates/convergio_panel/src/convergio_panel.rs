//! Convergio Panel - Multi-agent panel for Convergio AI assistants
//!
//! This panel displays all available Convergio agents and allows
//! starting conversations with each one.
//!
//! ## Architecture
//!
//! This crate uses a custom chat implementation that reads directly from
//! Convergio's SQLite database, ensuring full synchronization between
//! the Zed panel and the Convergio CLI.

mod agent_invoke;
mod agent_tools;
mod chat_view;
mod convergio_db;
mod panel;
mod settings;

pub use chat_view::ConvergioChatView;
pub use convergio_db::{ChatMessage, ConvergioDb, MessageType, Session, SessionMetadata};
pub use panel::ConvergioPanel;
pub use settings::ConvergioSettings;

use gpui::App;

pub fn init(cx: &mut App) {
    settings::init(cx);
    panel::init(cx);
    chat_view::init(cx);
}
