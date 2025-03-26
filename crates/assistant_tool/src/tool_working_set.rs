use std::sync::Arc;

use collections::{HashMap, HashSet, IndexMap};
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
    enabled_tools_by_source: HashMap<ToolSource, HashSet<Arc<str>>>,
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
        self.state.lock().tools(cx)
    }

    pub fn tools_by_source(&self, cx: &App) -> IndexMap<ToolSource, Vec<Arc<dyn Tool>>> {
        self.state.lock().tools_by_source(cx)
    }

    pub fn enabled_tools(&self, cx: &App) -> Vec<Arc<dyn Tool>> {
        self.state.lock().enabled_tools(cx)
    }

    pub fn disable_all_tools(&self) {
        let mut state = self.state.lock();
        state.disable_all_tools();
    }

    pub fn enable_source(&self, source: ToolSource, cx: &App) {
        let mut state = self.state.lock();
        state.enable_source(source, cx);
    }

    pub fn disable_source(&self, source: &ToolSource) {
        let mut state = self.state.lock();
        state.disable_source(source);
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

    pub fn is_enabled(&self, source: &ToolSource, name: &Arc<str>) -> bool {
        self.state.lock().is_enabled(source, name)
    }

    pub fn is_disabled(&self, source: &ToolSource, name: &Arc<str>) -> bool {
        self.state.lock().is_disabled(source, name)
    }

    pub fn enable(&self, source: ToolSource, tools_to_enable: &[Arc<str>]) {
        let mut state = self.state.lock();
        state.enable(source, tools_to_enable);
    }

    pub fn disable(&self, source: ToolSource, tools_to_disable: &[Arc<str>]) {
        let mut state = self.state.lock();
        state.disable(source, tools_to_disable);
    }

    pub fn remove(&self, tool_ids_to_remove: &[ToolId]) {
        let mut state = self.state.lock();
        state
            .context_server_tools_by_id
            .retain(|id, _| !tool_ids_to_remove.contains(id));
        state.tools_changed();
    }
}

impl WorkingSetState {
    fn tools_changed(&mut self) {
        self.context_server_tools_by_name.clear();
        self.context_server_tools_by_name.extend(
            self.context_server_tools_by_id
                .values()
                .map(|tool| (tool.name(), tool.clone())),
        );
    }

    fn tools(&self, cx: &App) -> Vec<Arc<dyn Tool>> {
        let mut tools = ToolRegistry::global(cx).tools();
        tools.extend(self.context_server_tools_by_id.values().cloned());

        tools
    }

    fn tools_by_source(&self, cx: &App) -> IndexMap<ToolSource, Vec<Arc<dyn Tool>>> {
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

    fn enabled_tools(&self, cx: &App) -> Vec<Arc<dyn Tool>> {
        let all_tools = self.tools(cx);

        all_tools
            .into_iter()
            .filter(|tool| self.is_enabled(&tool.source(), &tool.name().into()))
            .collect()
    }

    fn is_enabled(&self, source: &ToolSource, name: &Arc<str>) -> bool {
        self.enabled_tools_by_source
            .get(source)
            .map_or(false, |enabled_tools| enabled_tools.contains(name))
    }

    fn is_disabled(&self, source: &ToolSource, name: &Arc<str>) -> bool {
        !self.is_enabled(source, name)
    }

    fn enable(&mut self, source: ToolSource, tools_to_enable: &[Arc<str>]) {
        self.enabled_tools_by_source
            .entry(source)
            .or_default()
            .extend(tools_to_enable.into_iter().cloned());
    }

    fn disable(&mut self, source: ToolSource, tools_to_disable: &[Arc<str>]) {
        self.enabled_tools_by_source
            .entry(source)
            .or_default()
            .retain(|name| !tools_to_disable.contains(name));
    }

    fn enable_source(&mut self, source: ToolSource, cx: &App) {
        let tools_by_source = self.tools_by_source(cx);
        let Some(tools) = tools_by_source.get(&source) else {
            return;
        };

        self.enabled_tools_by_source.insert(
            source,
            tools
                .into_iter()
                .map(|tool| tool.name().into())
                .collect::<HashSet<_>>(),
        );
    }

    fn disable_source(&mut self, source: &ToolSource) {
        self.enabled_tools_by_source.remove(source);
    }

    fn disable_all_tools(&mut self) {
        self.enabled_tools_by_source.clear();
    }
}
