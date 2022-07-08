use gpui::{ModelHandle, ViewContext, ViewHandle};
use workspace::Workspace;

use crate::{get_working_directory, DeployModal, Event, Terminal, TerminalConnection};

pub fn deploy_modal(workspace: &mut Workspace, _: &DeployModal, cx: &mut ViewContext<Workspace>) {
    // Pull the terminal connection out of the global if it has been stored
    let possible_connection = cx
        .update_default_global::<Option<ModelHandle<TerminalConnection>>, _, _>(
            |possible_connection, _| possible_connection.take(),
        );

    if let Some(stored_connection) = possible_connection {
        // Create a view from the stored connection
        workspace.toggle_modal(cx, |_, cx| {
            cx.add_view(|cx| Terminal::from_connection(stored_connection, true, cx))
        });
    } else {
        // No connection was stored, create a new terminal
        if let Some(closed_terminal_handle) = workspace.toggle_modal(cx, |workspace, cx| {
            let project = workspace.project().read(cx);
            let abs_path = project
                .active_entry()
                .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
                .and_then(|worktree_handle| worktree_handle.read(cx).as_local())
                .and_then(get_working_directory);

            let this = cx.add_view(|cx| Terminal::new(abs_path, true, cx));
            let connection_handle = this.read(cx).connection.clone();
            cx.subscribe(&connection_handle, on_event).detach();
            this
        }) {
            let connection = closed_terminal_handle.read(cx).connection.clone();
            cx.set_global(Some(connection));
        }
    }
}

pub fn on_event(
    workspace: &mut Workspace,
    _: ModelHandle<TerminalConnection>,
    event: &Event,
    cx: &mut ViewContext<Workspace>,
) {
    // Dismiss the modal if the terminal quit
    if let Event::CloseTerminal = event {
        cx.set_global::<Option<ViewHandle<Terminal>>>(None);
        if workspace
            .modal()
            .cloned()
            .and_then(|modal| modal.downcast::<Terminal>())
            .is_some()
        {
            workspace.dismiss_modal(cx)
        }
    }
}
