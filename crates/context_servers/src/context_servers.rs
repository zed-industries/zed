use gpui::{actions, AppContext, Context, ViewContext};
use manager::ContextServerManager;
use workspace::Workspace;

pub mod client;
pub mod manager;
pub mod protocol;
pub mod types;

actions!(context_servers, [Restart]);

/// The namespace for the context servers actions.
const CONTEXT_SERVERS_NAMESPACE: &'static str = "context_servers";

pub fn init(cx: &mut AppContext) {
    manager::init(cx);
}
