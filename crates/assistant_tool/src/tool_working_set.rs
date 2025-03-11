use std::sync::Arc;

use collections::{HashMap, HashSet};
use gpui::App;
use parking_lot::Mutex;

use crate::{Tool, ToolRegistry, ToolSource};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ToolId(usize);

/// A working set of tools for use in one instance of the Assistant Panel.
#[derive(Default)]
pub struct ToolWorkingSet {
    state: Mutex<WorkingSetState>,
}

#[derive(Default)]
struct WorkingSetState {
    context_server_tools_by_id: HashMap<ToolId, Arc<dyn Tool>>,
    context_server_tools_by_name: HashMap<String, Arc<dyn Tool>>,
    next_tool_id: ToolId,
}

impl ToolWorkingSet {
    pub fn tool(&self, name: &str, cx: &App) -> Option<Arc<dyn Tool>> {
        self.state
            .lock()
            .context_server_tools_by_name
            .get(name)
            .cloned()
            .or_else(|| ToolRegistry::global(cx).tool(name))
    }

    pub fn tools(&self, cx: &App) -> Vec<Arc<dyn Tool>> {
        let mut tools = ToolRegistry::global(cx).tools();
        tools.extend(
            self.state
                .lock()
                .context_server_tools_by_id
                .values()
                .cloned(),
        );

        tools
    }

    pub fn tools_by_source(&self, cx: &App) -> HashMap<ToolSource, Vec<Arc<dyn Tool>>> {
        let mut tools_by_source = HashMap::default();

        for tool in self.tools(cx) {
            tools_by_source
                .entry(tool.source())
                .or_insert_with(Vec::new)
                .push(tool);
        }

        tools_by_source
    }

    pub fn insert(&self, tool: Arc<dyn Tool>) -> ToolId {
        let mut state = self.state.lock();
        let tool_id = state.next_tool_id;
        state.next_tool_id.0 += 1;
        state
            .context_server_tools_by_id
            .insert(tool_id, tool.clone());
        state.tools_changed();
        tool_id
    }

    pub fn remove(&self, command_ids_to_remove: &[ToolId]) {
        let mut state = self.state.lock();
        state
            .context_server_tools_by_id
            .retain(|id, _| !command_ids_to_remove.contains(id));
        state.tools_changed();
    }
}

impl WorkingSetState {
    fn tools_changed(&mut self) {
        self.context_server_tools_by_name.clear();
        self.context_server_tools_by_name.extend(
            self.context_server_tools_by_id
                .values()
                .map(|command| (command.name(), command.clone())),
        );
    }
}
