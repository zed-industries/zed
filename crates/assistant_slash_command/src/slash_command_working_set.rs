use std::sync::Arc;

use collections::HashMap;
use gpui::App;
use parking_lot::Mutex;

use crate::{SlashCommand, SlashCommandRegistry};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct SlashCommandId(usize);

/// A working set of slash commands for use in one instance of the Assistant Panel.
#[derive(Default)]
pub struct SlashCommandWorkingSet {
    state: Mutex<WorkingSetState>,
}

#[derive(Default)]
struct WorkingSetState {
    context_server_commands_by_id: HashMap<SlashCommandId, Arc<dyn SlashCommand>>,
    context_server_commands_by_name: HashMap<Arc<str>, Arc<dyn SlashCommand>>,
    next_command_id: SlashCommandId,
}

impl SlashCommandWorkingSet {
    pub fn command(&self, name: &str, cx: &App) -> Option<Arc<dyn SlashCommand>> {
        self.state
            .lock()
            .context_server_commands_by_name
            .get(name)
            .cloned()
            .or_else(|| SlashCommandRegistry::global(cx).command(name))
    }

    pub fn command_names(&self, cx: &App) -> Vec<Arc<str>> {
        let mut command_names = SlashCommandRegistry::global(cx).command_names();
        command_names.extend(
            self.state
                .lock()
                .context_server_commands_by_name
                .keys()
                .cloned(),
        );

        command_names
    }

    pub fn featured_command_names(&self, cx: &App) -> Vec<Arc<str>> {
        SlashCommandRegistry::global(cx).featured_command_names()
    }

    pub fn insert(&self, command: Arc<dyn SlashCommand>) -> SlashCommandId {
        let mut state = self.state.lock();
        let command_id = state.next_command_id;
        state.next_command_id.0 += 1;
        state
            .context_server_commands_by_id
            .insert(command_id, command.clone());
        state.slash_commands_changed();
        command_id
    }

    pub fn remove(&self, command_ids_to_remove: &[SlashCommandId]) {
        let mut state = self.state.lock();
        state
            .context_server_commands_by_id
            .retain(|id, _| !command_ids_to_remove.contains(id));
        state.slash_commands_changed();
    }
}

impl WorkingSetState {
    fn slash_commands_changed(&mut self) {
        self.context_server_commands_by_name.clear();
        self.context_server_commands_by_name.extend(
            self.context_server_commands_by_id
                .values()
                .map(|command| (command.name().into(), command.clone())),
        );
    }
}
