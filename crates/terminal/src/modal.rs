use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
};

use gpui::{AnyWeakModelHandle, Entity, ModelHandle, ViewContext, WeakModelHandle};
use settings::{Settings, WorkingDirectory};
use workspace::Workspace;

use crate::{
    terminal_container_view::{
        get_working_directory, DeployModal, TerminalContainer, TerminalContainerContent,
    },
    Event, Terminal,
};

// TODO: Need to put this basic structure in workspace, and make 'program handles'
// based off of the 'searchable item' pattern except with models this way, the workspace's clients
// can register their models as programs.
// Programs are:
//  - Kept alive by the program manager, they need to emit an event to get dropped from it
//  - Can be interacted with directly, (closed, activated), etc, bypassing associated view(s)
//  - Have special rendering methods that the program manager offers to fill out the status bar
//  - Can emit events for the program manager which:
//    - Add a jewel (notification, change, etc.)
//    - Drop the program
//    - ???
//  - Program Manager is kept in a global, listens for window drop so it can drop all it's program handles
//  - Start by making up the infrastructure, then just render the first item as the modal terminal in it's spot
// update),

struct ProgramManager {
    window_to_programs: HashMap<usize, HashSet<AnyWeakModelHandle>>,
}

impl ProgramManager {
    pub fn add_program<T: Entity>(&mut self, window: usize, program: WeakModelHandle<T>) {
        let mut programs = if let Some(programs) = self.window_to_programs.remove(&window) {
            programs
        } else {
            HashSet::default()
        };

        programs.insert(AnyWeakModelHandle::from(program));
        self.window_to_programs.insert(window, programs);
    }

    pub fn get_programs<T: Entity>(
        &self,
        window: &usize,
    ) -> impl Iterator<Item = WeakModelHandle<T>> + '_ {
        self.window_to_programs
            .get(window)
            .into_iter()
            .flat_map(|programs| {
                programs
                    .iter()
                    .filter(|program| program.model_type() != TypeId::of::<T>())
                    .map(|program| program.downcast().unwrap())
            })
    }
}

#[derive(Debug)]
struct StoredTerminal(ModelHandle<Terminal>);

pub fn deploy_modal(workspace: &mut Workspace, _: &DeployModal, cx: &mut ViewContext<Workspace>) {
    // cx.window_id()

    // Pull the terminal connection out of the global if it has been stored
    let possible_terminal =
        cx.update_default_global::<Option<StoredTerminal>, _, _>(|possible_connection, _| {
            possible_connection.take()
        });

    if let Some(StoredTerminal(stored_terminal)) = possible_terminal {
        workspace.toggle_modal(cx, |_, cx| {
            // Create a view from the stored connection if the terminal modal is not already shown
            cx.add_view(|cx| TerminalContainer::from_terminal(stored_terminal.clone(), true, cx))
        });
        // Toggle Modal will dismiss the terminal modal if it is currently shown, so we must
        // store the terminal back in the global
        cx.set_global::<Option<StoredTerminal>>(Some(StoredTerminal(stored_terminal.clone())));
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
                cx.set_global::<Option<StoredTerminal>>(Some(StoredTerminal(
                    terminal_handle.clone(),
                )));
            }

            this
        }) {
            // Terminal modal was dismissed. Store terminal if the terminal view is connected
            if let TerminalContainerContent::Connected(connected) =
                &closed_terminal_handle.read(cx).content
            {
                let terminal_handle = connected.read(cx).handle();
                // Set the global immediately if terminal construction was successful,
                // in case the user opens the command palette
                cx.set_global::<Option<StoredTerminal>>(Some(StoredTerminal(terminal_handle)));
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
        if workspace.modal::<TerminalContainer>().is_some() {
            workspace.dismiss_modal(cx)
        }
    }
}
