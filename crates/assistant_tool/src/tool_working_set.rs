use std::sync::Arc;

use collections::{HashMap, HashSet, IndexMap};
use gpui::{App, Context, EventEmitter};

use crate::{Tool, ToolRegistry, ToolSource};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ToolId(usize);

/// A working set of tools for use in one instance of the Assistant Panel.
#[derive(Default)]
pub struct ToolWorkingSet {
    context_server_tools_by_id: HashMap<ToolId, Arc<dyn Tool>>,
    context_server_tools_by_name: HashMap<String, Arc<dyn Tool>>,
    enabled_sources: HashSet<ToolSource>,
    enabled_tools_by_source: HashMap<ToolSource, HashSet<Arc<str>>>,
    next_tool_id: ToolId,
}

pub enum ToolWorkingSetEvent {
    EnabledToolsChanged,
}

impl EventEmitter<ToolWorkingSetEvent> for ToolWorkingSet {}

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

    pub fn enabled_tools(&self, cx: &App) -> Vec<Arc<dyn Tool>> {
        let all_tools = self.tools(cx);

        all_tools
            .into_iter()
            .filter(|tool| self.is_enabled(&tool.source(), &tool.name().into()))
            .collect()
    }

    pub fn disable_all_tools(&mut self, cx: &mut Context<Self>) {
        self.enabled_tools_by_source.clear();
        cx.emit(ToolWorkingSetEvent::EnabledToolsChanged);
    }

    pub fn enable_source(&mut self, source: ToolSource, cx: &mut Context<Self>) {
        self.enabled_sources.insert(source.clone());

        let tools_by_source = self.tools_by_source(cx);
        if let Some(tools) = tools_by_source.get(&source) {
            self.enabled_tools_by_source.insert(
                source,
                tools
                    .into_iter()
                    .map(|tool| tool.name().into())
                    .collect::<HashSet<_>>(),
            );
        }
        cx.emit(ToolWorkingSetEvent::EnabledToolsChanged);
    }

    pub fn disable_source(&mut self, source: &ToolSource, cx: &mut Context<Self>) {
        self.enabled_sources.remove(source);
        self.enabled_tools_by_source.remove(source);
        cx.emit(ToolWorkingSetEvent::EnabledToolsChanged);
    }

    pub fn insert(&mut self, tool: Arc<dyn Tool>) -> ToolId {
        let tool_id = self.next_tool_id;
        self.next_tool_id.0 += 1;
        self.context_server_tools_by_id
            .insert(tool_id, tool.clone());
        self.tools_changed();
        tool_id
    }

    pub fn is_enabled(&self, source: &ToolSource, name: &Arc<str>) -> bool {
        self.enabled_tools_by_source
            .get(source)
            .map_or(false, |enabled_tools| enabled_tools.contains(name))
    }

    pub fn is_disabled(&self, source: &ToolSource, name: &Arc<str>) -> bool {
        !self.is_enabled(source, name)
    }

    pub fn enable(
        &mut self,
        source: ToolSource,
        tools_to_enable: &[Arc<str>],
        cx: &mut Context<Self>,
    ) {
        self.enabled_tools_by_source
            .entry(source)
            .or_default()
            .extend(tools_to_enable.into_iter().cloned());
        cx.emit(ToolWorkingSetEvent::EnabledToolsChanged);
    }

    pub fn disable(
        &mut self,
        source: ToolSource,
        tools_to_disable: &[Arc<str>],
        cx: &mut Context<Self>,
    ) {
        self.enabled_tools_by_source
            .entry(source)
            .or_default()
            .retain(|name| !tools_to_disable.contains(name));
        cx.emit(ToolWorkingSetEvent::EnabledToolsChanged);
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
