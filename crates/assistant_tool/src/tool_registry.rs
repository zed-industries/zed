use std::sync::Arc;

use collections::HashMap;
use derive_more::{Deref, DerefMut};
use gpui::Global;
use gpui::{AppContext, ReadGlobal};
use parking_lot::RwLock;

use crate::Tool;

#[derive(Default, Deref, DerefMut)]
struct GlobalToolRegistry(Arc<ToolRegistry>);

impl Global for GlobalToolRegistry {}

#[derive(Default)]
struct ToolRegistryState {
    tools: HashMap<Arc<str>, Arc<dyn Tool>>,
}

#[derive(Default)]
pub struct ToolRegistry {
    state: RwLock<ToolRegistryState>,
}

impl ToolRegistry {
    /// Returns the global [`ToolRegistry`].
    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalToolRegistry::global(cx).0.clone()
    }

    /// Returns the global [`ToolRegistry`].
    ///
    /// Inserts a default [`ToolRegistry`] if one does not yet exist.
    pub fn default_global(cx: &mut AppContext) -> Arc<Self> {
        cx.default_global::<GlobalToolRegistry>().0.clone()
    }

    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            state: RwLock::new(ToolRegistryState {
                tools: HashMap::default(),
            }),
        })
    }

    /// Registers the provided [`Tool`].
    pub fn register_tool(&self, tool: impl Tool) {
        let mut state = self.state.write();
        let tool_name: Arc<str> = tool.name().into();
        state.tools.insert(tool_name, Arc::new(tool));
    }

    /// Unregisters the provided [`Tool`].
    pub fn unregister_tool(&self, tool: impl Tool) {
        self.unregister_tool_by_name(tool.name().as_str())
    }

    /// Unregisters the tool with the given name.
    pub fn unregister_tool_by_name(&self, tool_name: &str) {
        let mut state = self.state.write();
        state.tools.remove(tool_name);
    }

    /// Returns the list of tools in the registry.
    pub fn tools(&self) -> Vec<Arc<dyn Tool>> {
        self.state.read().tools.values().cloned().collect()
    }

    /// Returns the [`Tool`] with the given name.
    pub fn tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.state.read().tools.get(name).cloned()
    }
}
