use gpui::{ModelHandle, ViewContext};
use settings::{Settings, WorkingDirectory};
use workspace::{programs::ProgramManager, Workspace};

use crate::{
    terminal_container_view::{
        get_working_directory, DeployModal, TerminalContainer, TerminalContainerContent,
    },
    Event, Terminal,
};

pub fn deploy_modal(workspace: &mut Workspace, _: &DeployModal, cx: &mut ViewContext<Workspace>) {
    let window = cx.window_id();

    // Pull the terminal connection out of the global if it has been stored
    let possible_terminal = ProgramManager::remove::<Terminal, _>(window, cx);

    if let Some(terminal_handle) = possible_terminal {
        workspace.toggle_modal(cx, |_, cx| {
            // Create a view from the stored connection if the terminal modal is not already shown
            cx.add_view(|cx| TerminalContainer::from_terminal(terminal_handle.clone(), true, cx))
        });
        // Toggle Modal will dismiss the terminal modal if it is currently shown, so we must
        // store the terminal back in the global
        ProgramManager::insert_or_replace::<Terminal, _>(window, terminal_handle, cx);
    } else {
        // No connection was stored, create a new terminal
        if let Some(closed_terminal_handle) = workspace.toggle_modal(cx, |workspace, cx| {
            // No terminal modal visible, construct a new one.
            let wd_strategy = cx
                .global::<Settings>()
                .terminal_overrides
                .working_directory
                .clone()
                .unwrap_or(WorkingDirectory::CurrentProjectDirectory);

            let working_directory = get_working_directory(workspace, cx, wd_strategy);

            let this = cx.add_view(|cx| TerminalContainer::new(working_directory, true, cx));

            if let TerminalContainerContent::Connected(connected) = &this.read(cx).content {
                let terminal_handle = connected.read(cx).handle();
                cx.subscribe(&terminal_handle, on_event).detach();
                // Set the global immediately if terminal construction was successful,
                // in case the user opens the command palette
                ProgramManager::insert_or_replace::<Terminal, _>(window, terminal_handle, cx);
            }

            this
        }) {
            // Terminal modal was dismissed and the terminal view is connected, store the terminal
            if let TerminalContainerContent::Connected(connected) =
                &closed_terminal_handle.read(cx).content
            {
                let terminal_handle = connected.read(cx).handle();
                // Set the global immediately if terminal construction was successful,
                // in case the user opens the command palette
                ProgramManager::insert_or_replace::<Terminal, _>(window, terminal_handle, cx);
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
        ProgramManager::remove::<Terminal, _>(cx.window_id(), cx);

        if workspace.modal::<TerminalContainer>().is_some() {
            workspace.dismiss_modal(cx)
        }
    }
}
