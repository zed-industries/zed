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
        let context_server_command = self
            .state
            .lock()
            .context_server_commands_by_name
            .get(name)
            .cloned();

        if let Some(command) = &context_server_command {
            log::debug!("Found context server slash command: '{}'", name);
            return Some(command.clone());
        }

        let registry_command = SlashCommandRegistry::global(cx).command(name);
        if registry_command.is_some() {
            log::debug!("Found registry slash command: '{}'", name);
        } else {
            log::debug!(
                "Slash command '{}' not found in either context server or registry",
                name
            );
        }

        registry_command
    }

    pub fn command_names(&self, cx: &App) -> Vec<Arc<str>> {
        let mut command_names = SlashCommandRegistry::global(cx).command_names();

        let context_server_commands: Vec<Arc<str>> = self
            .state
            .lock()
            .context_server_commands_by_name
            .keys()
            .cloned()
            .collect();

        log::debug!(
            "Available slash commands - Registry: {:?}, Context Server: {:?}",
            command_names,
            context_server_commands
        );

        command_names.extend(context_server_commands);
        command_names
    }

    pub fn featured_command_names(&self, cx: &App) -> Vec<Arc<str>> {
        SlashCommandRegistry::global(cx).featured_command_names()
    }

    pub fn insert(&self, command: Arc<dyn SlashCommand>) -> SlashCommandId {
        let command_name = command.name();
        log::info!("Inserting slash command: '{}'", command_name);

        let mut state = self.state.lock();
        let command_id = state.next_command_id;
        state.next_command_id.0 += 1;
        state
            .context_server_commands_by_id
            .insert(command_id, command.clone());
        state.slash_commands_changed();

        log::info!(
            "Successfully inserted slash command '{}' with ID {:?}",
            command_name,
            command_id
        );
        command_id
    }

    pub fn remove(&self, command_ids_to_remove: &[SlashCommandId]) {
        log::info!(
            "Removing slash commands with IDs: {:?}",
            command_ids_to_remove
        );

        let mut state = self.state.lock();
        let before_count = state.context_server_commands_by_id.len();
        state
            .context_server_commands_by_id
            .retain(|id, _| !command_ids_to_remove.contains(id));
        state.slash_commands_changed();

        let after_count = state.context_server_commands_by_id.len();
        log::info!(
            "Removed {} slash commands, {} remaining",
            before_count - after_count,
            after_count
        );
    }
}

impl WorkingSetState {
    fn slash_commands_changed(&mut self) {
        self.context_server_commands_by_name.clear();
        let commands: Vec<(Arc<str>, Arc<dyn SlashCommand>)> = self
            .context_server_commands_by_id
            .values()
            .map(|command| (command.name().into(), command.clone()))
            .collect();

        log::debug!(
            "Rebuilding slash command name mapping with {} commands: {}",
            commands.len(),
            commands
                .iter()
                .map(|(name, _)| name.as_ref())
                .collect::<Vec<_>>()
                .join(", ")
        );

        self.context_server_commands_by_name.extend(commands);
    }
}
