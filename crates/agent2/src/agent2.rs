mod agent;
mod db;
pub mod history_store;
mod native_agent_server;
mod templates;
mod thread;
mod tools;

#[cfg(test)]
mod tests;

pub use agent::*;
pub use db::*;
pub use native_agent_server::NativeAgentServer;
pub use templates::*;
pub use thread::*;
pub use tools::*;

use agent_client_protocol as acp;

pub fn generate_session_id() -> acp::SessionId {
    acp::SessionId(uuid::Uuid::new_v4().to_string().into())
}
