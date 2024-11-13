use assistant_tool::{Tool, ToolRegistry};
use collections::HashMap;
use gpui::AppContext;
use parking_lot::Mutex;
use std::sync::Arc;

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
    pub fn tool(&self, name: &str, cx: &AppContext) -> Option<Arc<dyn Tool>> {
        self.state
            .lock()
            .context_server_tools_by_name
            .get(name)
            .cloned()
            .or_else(|| ToolRegistry::global(cx).tool(name))
    }

    pub fn tools(&self, cx: &AppContext) -> Vec<Arc<dyn Tool>> {
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

    pub fn insert(&self, command: Arc<dyn Tool>) -> ToolId {
        let mut state = self.state.lock();
        let command_id = state.next_tool_id;
        state.next_tool_id.0 += 1;
        state
            .context_server_tools_by_id
            .insert(command_id, command.clone());
        state.tools_changed();
        command_id
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
