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

struct WorkingSetState {
    context_server_tools_by_id: HashMap<ToolId, Arc<dyn Tool>>,
    context_server_tools_by_name: HashMap<String, Arc<dyn Tool>>,
    disabled_tools_by_source: HashMap<ToolSource, HashSet<Arc<str>>>,
    is_scripting_tool_disabled: bool,
    next_tool_id: ToolId,
}

impl Default for WorkingSetState {
    fn default() -> Self {
        Self {
            context_server_tools_by_id: HashMap::default(),
            context_server_tools_by_name: HashMap::default(),
            disabled_tools_by_source: HashMap::default(),
            is_scripting_tool_disabled: true,
            next_tool_id: ToolId::default(),
        }
    }
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

    pub fn are_all_tools_enabled(&self) -> bool {
        let state = self.state.lock();
        state.disabled_tools_by_source.is_empty() && !state.is_scripting_tool_disabled
    }

    pub fn are_all_tools_from_source_enabled(&self, source: &ToolSource) -> bool {
        let state = self.state.lock();
        !state.disabled_tools_by_source.contains_key(source)
    }

    pub fn enabled_tools(&self, cx: &App) -> Vec<Arc<dyn Tool>> {
        self.state.lock().enabled_tools(cx)
    }

    pub fn enable_all_tools(&self) {
        let mut state = self.state.lock();
        state.disabled_tools_by_source.clear();
        state.enable_scripting_tool();
    }

    pub fn disable_all_tools(&self, cx: &App) {
        let mut state = self.state.lock();
        state.disable_all_tools(cx);
    }

    pub fn enable_source(&self, source: &ToolSource) {
        let mut state = self.state.lock();
        state.enable_source(source);
    }

    pub fn disable_source(&self, source: ToolSource, cx: &App) {
        let mut state = self.state.lock();
        state.disable_source(source, cx);
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

    pub fn is_scripting_tool_enabled(&self) -> bool {
        let state = self.state.lock();
        !state.is_scripting_tool_disabled
    }

    pub fn enable_scripting_tool(&self) {
        let mut state = self.state.lock();
        state.enable_scripting_tool();
    }

    pub fn disable_scripting_tool(&self) {
        let mut state = self.state.lock();
        state.disable_scripting_tool();
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
        !self.is_disabled(source, name)
    }

    fn is_disabled(&self, source: &ToolSource, name: &Arc<str>) -> bool {
        self.disabled_tools_by_source
            .get(source)
            .map_or(false, |disabled_tools| disabled_tools.contains(name))
    }

    fn enable(&mut self, source: ToolSource, tools_to_enable: &[Arc<str>]) {
        self.disabled_tools_by_source
            .entry(source)
            .or_default()
            .retain(|name| !tools_to_enable.contains(name));
    }

    fn disable(&mut self, source: ToolSource, tools_to_disable: &[Arc<str>]) {
        self.disabled_tools_by_source
            .entry(source)
            .or_default()
            .extend(tools_to_disable.into_iter().cloned());
    }

    fn enable_source(&mut self, source: &ToolSource) {
        self.disabled_tools_by_source.remove(source);
    }

    fn disable_source(&mut self, source: ToolSource, cx: &App) {
        let tools_by_source = self.tools_by_source(cx);
        let Some(tools) = tools_by_source.get(&source) else {
            return;
        };

        self.disabled_tools_by_source.insert(
            source,
            tools
                .into_iter()
                .map(|tool| tool.name().into())
                .collect::<HashSet<_>>(),
        );
    }

    fn disable_all_tools(&mut self, cx: &App) {
        let tools = self.tools_by_source(cx);

        for (source, tools) in tools {
            let tool_names = tools
                .into_iter()
                .map(|tool| tool.name().into())
                .collect::<Vec<_>>();

            self.disable(source, &tool_names);
        }

        self.disable_scripting_tool();
    }

    fn enable_scripting_tool(&mut self) {
        self.is_scripting_tool_disabled = false;
    }

    fn disable_scripting_tool(&mut self) {
        self.is_scripting_tool_disabled = true;
    }
}
