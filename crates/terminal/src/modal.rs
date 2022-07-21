use gpui::{ModelHandle, ViewContext};
use workspace::Workspace;

use crate::{connection::Terminal, get_wd_for_workspace, DeployModal, Event, TerminalView};

#[derive(Debug)]
struct StoredConnection(ModelHandle<Terminal>);

pub fn deploy_modal(workspace: &mut Workspace, _: &DeployModal, cx: &mut ViewContext<Workspace>) {
    // Pull the terminal connection out of the global if it has been stored
    let possible_connection =
        cx.update_default_global::<Option<StoredConnection>, _, _>(|possible_connection, _| {
            possible_connection.take()
        });

    if let Some(StoredConnection(stored_connection)) = possible_connection {
        // Create a view from the stored connection
        workspace.toggle_modal(cx, |_, cx| {
            cx.add_view(|cx| {
                TerminalView::from_connection(
                    crate::TerminalConnection(Ok(stored_connection.clone())),
                    true,
                    cx,
                )
            })
        });
        cx.set_global::<Option<StoredConnection>>(Some(StoredConnection(
            stored_connection.clone(),
        )));
    } else {
        // No connection was stored, create a new terminal
        if let Some(closed_terminal_handle) = workspace.toggle_modal(cx, |workspace, cx| {
            let wd = get_wd_for_workspace(workspace, cx);

            let this = cx.add_view(|cx| TerminalView::new(wd, true, cx).unwrap());

            let connection_handle = this.read(cx).connection.0.as_ref().unwrap().clone();
            cx.subscribe(&connection_handle, on_event).detach();
            //Set the global immediately, in case the user opens the command palette
            cx.set_global::<Option<StoredConnection>>(Some(StoredConnection(
                connection_handle.clone(),
            )));
            this
        }) {
            let connection = closed_terminal_handle
                .read(cx)
                .connection
                .0
                .as_ref()
                .unwrap()
                .clone();
            cx.set_global(Some(StoredConnection(connection)));
        }
    }

    //The problem is that the terminal modal is never re-stored.
}

pub fn on_event(
    workspace: &mut Workspace,
    _: ModelHandle<Terminal>,
    event: &Event,
    cx: &mut ViewContext<Workspace>,
) {
    // Dismiss the modal if the terminal quit
    if let Event::CloseTerminal = event {
        cx.set_global::<Option<StoredConnection>>(None);
        if workspace
            .modal()
            .cloned()
            .and_then(|modal| modal.downcast::<TerminalView>())
            .is_some()
        {
            workspace.dismiss_modal(cx)
        }
    }
}
