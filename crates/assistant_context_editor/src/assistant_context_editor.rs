mod context;
mod context_editor;
mod context_history;
mod context_store;
mod slash_command;
mod slash_command_picker;

use std::sync::Arc;

use client::Client;
use gpui::{App, Context};
use workspace::Workspace;

pub use crate::context::*;
pub use crate::context_editor::*;
pub use crate::context_history::*;
pub use crate::context_store::*;
pub use crate::slash_command::*;

pub fn init(client: Arc<Client>, cx: &mut App) {
    context_store::init(&client.into());
    workspace::FollowableViewRegistry::register::<ContextEditor>(cx);

    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace
                .register_action(ContextEditor::quote_selection)
                .register_action(ContextEditor::insert_selection)
                .register_action(ContextEditor::copy_code)
                .register_action(ContextEditor::handle_insert_dragged_files);
        },
    )
    .detach();
}
