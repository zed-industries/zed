use std::sync::Arc;

use collections::HashMap;
use gpui::{AppContext, Global, ReadGlobal};
use parking_lot::RwLock;

struct GlobalContextServerRegistry(Arc<ContextServerRegistry>);

impl Global for GlobalContextServerRegistry {}

pub struct ContextServerRegistry {
    command_registry: RwLock<HashMap<String, Vec<Arc<str>>>>,
    tool_registry: RwLock<HashMap<String, Vec<Arc<str>>>>,
}

impl ContextServerRegistry {
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalContextServerRegistry::global(cx).0.clone()
    }

    pub fn register(cx: &mut AppContext) {
        cx.set_global(GlobalContextServerRegistry(Arc::new(
            ContextServerRegistry {
                command_registry: RwLock::new(HashMap::default()),
                tool_registry: RwLock::new(HashMap::default()),
            },
        )))
    }

    pub fn register_command(&self, server_id: String, command_name: &str) {
        let mut registry = self.command_registry.write();
        registry
            .entry(server_id)
            .or_default()
            .push(command_name.into());
    }

    pub fn unregister_command(&self, server_id: &str, command_name: &str) {
        let mut registry = self.command_registry.write();
        if let Some(commands) = registry.get_mut(server_id) {
            commands.retain(|name| name.as_ref() != command_name);
        }
    }

    pub fn get_commands(&self, server_id: &str) -> Option<Vec<Arc<str>>> {
        let registry = self.command_registry.read();
        registry.get(server_id).cloned()
    }

    pub fn register_tool(&self, server_id: String, tool_name: &str) {
        let mut registry = self.tool_registry.write();
        registry
            .entry(server_id)
            .or_default()
            .push(tool_name.into());
    }

    pub fn unregister_tool(&self, server_id: &str, tool_name: &str) {
        let mut registry = self.tool_registry.write();
        if let Some(tools) = registry.get_mut(server_id) {
            tools.retain(|name| name.as_ref() != tool_name);
        }
    }

    pub fn get_tools(&self, server_id: &str) -> Option<Vec<Arc<str>>> {
        let registry = self.tool_registry.read();
        registry.get(server_id).cloned()
    }
}
