use assistant_slash_command::{SlashCommand, SlashCommandRegistry};
use collections::HashMap;
use gpui::AppContext;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SlashCommandId(usize);

pub struct SlashCommandWorkingSet {
    context_server_commands_by_id: HashMap<SlashCommandId, Arc<dyn SlashCommand>>,
    context_server_commands_by_name: HashMap<String, Arc<dyn SlashCommand>>,
    next_command_id: SlashCommandId,
}

impl SlashCommandWorkingSet {
    pub fn command(&self, name: &str, cx: &AppContext) -> Option<Arc<dyn SlashCommand>> {
        self.context_server_commands_by_name
            .get(name)
            .cloned()
            .or_else(|| SlashCommandRegistry::global(cx).command(name))
    }

    pub fn insert(&mut self, command: Arc<dyn SlashCommand>) -> SlashCommandId {
        let command_id = self.next_command_id;
        self.next_command_id.0 += 1;
        self.context_server_commands_by_id
            .insert(command_id, command.clone());
        self.slash_commands_changed();
        command_id
    }

    pub fn remove(&mut self, command_id: SlashCommandId) {
        self.context_server_commands_by_id.remove(&command_id);
        self.slash_commands_changed();
    }

    fn slash_commands_changed(&mut self) {
        self.context_server_commands_by_name.clear();
        self.context_server_commands_by_name.extend(
            self.context_server_commands_by_id
                .values()
                .map(|command| (command.name(), command.clone())),
        );
    }
}
