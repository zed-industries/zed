mod context;
mod context_editor;
mod context_history;
mod context_store;
mod patch;
mod slash_command;
mod slash_command_picker;

use std::sync::Arc;

use client::Client;
use gpui::AppContext;

pub use crate::context::*;
pub use crate::context_editor::*;
pub use crate::context_history::*;
pub use crate::context_store::*;
pub use crate::patch::*;
pub use crate::slash_command::*;

pub fn init(client: Arc<Client>, _cx: &mut AppContext) {
    context_store::init(&client.into());
}
