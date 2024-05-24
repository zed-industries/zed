use std::sync::Arc;

use collections::HashMap;
use derive_more::{Deref, DerefMut};
use gpui::Global;
use gpui::{AppContext, ReadGlobal};
use parking_lot::RwLock;

use crate::SlashCommand;

#[derive(Default, Deref, DerefMut)]
struct GlobalSlashCommandRegistry(Arc<SlashCommandRegistry>);

impl Global for GlobalSlashCommandRegistry {}

#[derive(Default)]
struct SlashCommandRegistryState {
    commands: HashMap<Arc<str>, Arc<dyn SlashCommand>>,
}

#[derive(Default)]
pub struct SlashCommandRegistry {
    state: RwLock<SlashCommandRegistryState>,
}

impl SlashCommandRegistry {
    /// Returns the global [`SlashCommandRegistry`].
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalSlashCommandRegistry::global(cx).0.clone()
    }

    /// Returns the global [`SlashCommandRegistry`].
    ///
    /// Inserts a default [`SlashCommandRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut AppContext) -> Arc<Self> {
        cx.default_global::<GlobalSlashCommandRegistry>().0.clone()
    }

    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            state: RwLock::new(SlashCommandRegistryState {
                commands: HashMap::default(),
            }),
        })
    }

    /// Registers the provided [`SlashCommand`].
    pub fn register_command(&self, command: impl SlashCommand) {
        self.state
            .write()
            .commands
            .insert(command.name().into(), Arc::new(command));
    }

    /// Returns the names of registered [`SlashCommand`]s.
    pub fn command_names(&self) -> Vec<Arc<str>> {
        self.state.read().commands.keys().cloned().collect()
    }

    /// Returns the [`SlashCommand`] with the given name.
    pub fn command(&self, name: &str) -> Option<Arc<dyn SlashCommand>> {
        self.state.read().commands.get(name).cloned()
    }
}
