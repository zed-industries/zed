use gpui::{ModelHandle, ViewContext};
use workspace::Workspace;

use crate::{
    terminal_tab::{get_working_directory, DeployModal, TerminalContent, TerminalView},
    Event, Terminal,
};

#[derive(Debug)]
struct StoredTerminal(ModelHandle<Terminal>);

pub fn deploy_modal(workspace: &mut Workspace, _: &DeployModal, cx: &mut ViewContext<Workspace>) {
    // Pull the terminal connection out of the global if it has been stored
    let possible_terminal =
        cx.update_default_global::<Option<StoredTerminal>, _, _>(|possible_connection, _| {
            possible_connection.take()
        });

    if let Some(StoredTerminal(stored_terminal)) = possible_terminal {
        workspace.toggle_modal(cx, |_, cx| {
            // Create a view from the stored connection if the terminal modal is not already shown
            cx.add_view(|cx| TerminalView::from_terminal(stored_terminal.clone(), true, cx))
        });
        // Toggle Modal will dismiss the terminal modal if it is currently shown, so we must
        // store the terminal back in the global
        cx.set_global::<Option<StoredTerminal>>(Some(StoredTerminal(stored_terminal.clone())));
    } else {
        // No connection was stored, create a new terminal
        if let Some(closed_terminal_handle) = workspace.toggle_modal(cx, |workspace, cx| {
            // No terminal modal visible, construct a new one.
            let working_directory = get_working_directory(workspace, cx);

            let this = cx.add_view(|cx| TerminalView::new(working_directory, true, cx));

            if let TerminalContent::Connected(connected) = &this.read(cx).content {
                let terminal_handle = connected.read(cx).handle();
                cx.subscribe(&terminal_handle, on_event).detach();
                // Set the global immediately if terminal construction was successful,
                // in case the user opens the command palette
                cx.set_global::<Option<StoredTerminal>>(Some(StoredTerminal(
                    terminal_handle.clone(),
                )));
            }

            this
        }) {
            // Terminal modal was dismissed. Store terminal if the terminal view is connected
            if let TerminalContent::Connected(connected) = &closed_terminal_handle.read(cx).content
            {
                let terminal_handle = connected.read(cx).handle();
                // Set the global immediately if terminal construction was successful,
                // in case the user opens the command palette
                cx.set_global::<Option<StoredTerminal>>(Some(StoredTerminal(
                    terminal_handle.clone(),
                )));
            }
        }
    }
}

pub fn on_event(
    workspace: &mut Workspace,
    _: ModelHandle<Terminal>,
    event: &Event,
    cx: &mut ViewContext<Workspace>,
) {
    // Dismiss the modal if the terminal quit
    if let Event::CloseTerminal = event {
        cx.set_global::<Option<StoredTerminal>>(None);
        if workspace.modal::<TerminalView>().is_some() {
            workspace.dismiss_modal(cx)
        }
    }
}
