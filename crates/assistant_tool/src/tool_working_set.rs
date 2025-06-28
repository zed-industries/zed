use std::sync::Arc;

use collections::{HashMap, IndexMap};
use gpui::App;

use crate::{Tool, ToolRegistry, ToolSource};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ToolId(usize);

/// A working set of tools for use in one instance of the Assistant Panel.
#[derive(Default)]
pub struct ToolWorkingSet {
    context_server_tools_by_id: HashMap<ToolId, Arc<dyn Tool>>,
    context_server_tools_by_name: HashMap<String, Arc<dyn Tool>>,
    next_tool_id: ToolId,
}

impl ToolWorkingSet {
    pub fn tool(&self, name: &str, cx: &App) -> Option<Arc<dyn Tool>> {
        self.context_server_tools_by_name
            .get(name)
            .cloned()
            .or_else(|| ToolRegistry::global(cx).tool(name))
    }

    pub fn tools(&self, cx: &App) -> Vec<Arc<dyn Tool>> {
        let mut tools = ToolRegistry::global(cx).tools();
        tools.extend(self.context_server_tools_by_id.values().cloned());
        tools
    }

    pub fn tools_by_source(&self, cx: &App) -> IndexMap<ToolSource, Vec<Arc<dyn Tool>>> {
        let mut tools_by_source = IndexMap::default();

        for tool in self.tools(cx) {
            tools_by_source
                .entry(tool.source())
                .or_insert_with(Vec::new)
                .push(tool);
        }

        for tools in tools_by_source.values_mut() {
            tools.sort_by_key(|tool| tool.name());
        }

        tools_by_source.sort_unstable_keys();

        tools_by_source
    }

    pub fn insert(&mut self, tool: Arc<dyn Tool>) -> ToolId {
        let tool_id = self.next_tool_id;
        self.next_tool_id.0 += 1;
        self.context_server_tools_by_id
            .insert(tool_id, tool.clone());
        self.tools_changed();
        tool_id
    }

    pub fn remove(&mut self, tool_ids_to_remove: &[ToolId]) {
        self.context_server_tools_by_id
            .retain(|id, _| !tool_ids_to_remove.contains(id));
        self.tools_changed();
    }

    fn tools_changed(&mut self) {
        self.context_server_tools_by_name.clear();
        self.context_server_tools_by_name.extend(
            self.context_server_tools_by_id
                .values()
                .map(|tool| (tool.name(), tool.clone())),
        );
    }
}
