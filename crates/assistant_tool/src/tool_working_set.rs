use std::{borrow::Borrow, sync::Arc};

use crate::{Tool, ToolRegistry, ToolSource};
use collections::{HashMap, HashSet, IndexMap};
use gpui::{App, SharedString};
use util::debug_panic;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ToolId(usize);

/// A unique identifier for a tool within a working set.
#[derive(Clone, PartialEq, Eq, Hash, Default)]
pub struct UniqueToolName(SharedString);

impl Borrow<str> for UniqueToolName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl From<String> for UniqueToolName {
    fn from(value: String) -> Self {
        UniqueToolName(SharedString::new(value))
    }
}

impl Into<String> for UniqueToolName {
    fn into(self) -> String {
        self.0.into()
    }
}

impl std::fmt::Debug for UniqueToolName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for UniqueToolName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

/// A working set of tools for use in one instance of the Assistant Panel.
#[derive(Default)]
pub struct ToolWorkingSet {
    context_server_tools_by_id: HashMap<ToolId, Arc<dyn Tool>>,
    context_server_tools_by_name: HashMap<UniqueToolName, Arc<dyn Tool>>,
    next_tool_id: ToolId,
}

impl ToolWorkingSet {
    pub fn tool(&self, name: &str, cx: &App) -> Option<Arc<dyn Tool>> {
        self.context_server_tools_by_name
            .get(name)
            .cloned()
            .or_else(|| ToolRegistry::global(cx).tool(name))
    }

    pub fn tools(&self, cx: &App) -> Vec<(UniqueToolName, Arc<dyn Tool>)> {
        let mut tools = ToolRegistry::global(cx)
            .tools()
            .into_iter()
            .map(|tool| (UniqueToolName(tool.name().into()), tool))
            .collect::<Vec<_>>();
        tools.extend(self.context_server_tools_by_name.clone());
        tools
    }

    pub fn tools_by_source(&self, cx: &App) -> IndexMap<ToolSource, Vec<Arc<dyn Tool>>> {
        let mut tools_by_source = IndexMap::default();

        for (_, tool) in self.tools(cx) {
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

    pub fn insert(&mut self, tool: Arc<dyn Tool>, cx: &App) -> ToolId {
        let tool_id = self.register_tool(tool);
        self.tools_changed(cx);
        tool_id
    }

    pub fn extend(&mut self, tools: impl Iterator<Item = Arc<dyn Tool>>, cx: &App) -> Vec<ToolId> {
        let ids = tools.map(|tool| self.register_tool(tool)).collect();
        self.tools_changed(cx);
        ids
    }

    pub fn remove(&mut self, tool_ids_to_remove: &[ToolId], cx: &App) {
        self.context_server_tools_by_id
            .retain(|id, _| !tool_ids_to_remove.contains(id));
        self.tools_changed(cx);
    }

    fn register_tool(&mut self, tool: Arc<dyn Tool>) -> ToolId {
        let tool_id = self.next_tool_id;
        self.next_tool_id.0 += 1;
        self.context_server_tools_by_id
            .insert(tool_id, tool.clone());
        tool_id
    }

    fn tools_changed(&mut self, cx: &App) {
        self.context_server_tools_by_name = resolve_context_server_tool_name_conflicts(
            &self
                .context_server_tools_by_id
                .values()
                .cloned()
                .collect::<Vec<_>>(),
            &ToolRegistry::global(cx).tools(),
        );
    }
}

fn resolve_context_server_tool_name_conflicts(
    context_server_tools: &[Arc<dyn Tool>],
    native_tools: &[Arc<dyn Tool>],
) -> HashMap<UniqueToolName, Arc<dyn Tool>> {
    fn resolve_tool_name(tool: &Arc<dyn Tool>) -> String {
        let mut tool_name = tool.name();
        tool_name.truncate(MAX_TOOL_NAME_LENGTH);
        tool_name
    }

    const MAX_TOOL_NAME_LENGTH: usize = 64;

    let mut duplicated_tool_names = HashSet::default();
    let mut seen_tool_names = HashSet::default();
    seen_tool_names.extend(native_tools.iter().map(|tool| tool.name()));
    for tool in context_server_tools {
        let tool_name = resolve_tool_name(tool);
        if seen_tool_names.contains(&tool_name) {
            debug_assert!(
                tool.source() != ToolSource::Native,
                "Expected MCP tool but got a native tool: {}",
                tool_name
            );
            duplicated_tool_names.insert(tool_name);
        } else {
            seen_tool_names.insert(tool_name);
        }
    }

    if duplicated_tool_names.is_empty() {
        return context_server_tools
            .iter()
            .map(|tool| (resolve_tool_name(tool).into(), tool.clone()))
            .collect();
    }

    context_server_tools
        .iter()
        .filter_map(|tool| {
            let mut tool_name = resolve_tool_name(tool);
            if !duplicated_tool_names.contains(&tool_name) {
                return Some((tool_name.into(), tool.clone()));
            }
            match tool.source() {
                ToolSource::Native => {
                    debug_panic!("Expected MCP tool but got a native tool: {}", tool_name);
                    // Built-in tools always keep their original name
                    Some((tool_name.into(), tool.clone()))
                }
                ToolSource::ContextServer { id } => {
                    // Context server tools are prefixed with the context server ID, and truncated if necessary
                    tool_name.insert(0, '_');
                    if tool_name.len() + id.len() > MAX_TOOL_NAME_LENGTH {
                        let len = MAX_TOOL_NAME_LENGTH - tool_name.len();
                        let mut id = id.to_string();
                        id.truncate(len);
                        tool_name.insert_str(0, &id);
                    } else {
                        tool_name.insert_str(0, &id);
                    }

                    tool_name.truncate(MAX_TOOL_NAME_LENGTH);

                    if seen_tool_names.contains(&tool_name) {
                        log::error!("Cannot resolve tool name conflict for tool {}", tool.name());
                        None
                    } else {
                        Some((tool_name.into(), tool.clone()))
                    }
                }
            }
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use gpui::{AnyWindowHandle, Entity, Task, TestAppContext};
    use language_model::{LanguageModel, LanguageModelRequest};
    use project::Project;

    use crate::{ActionLog, ToolResult};

    use super::*;

    #[gpui::test]
    fn test_unique_tool_names(cx: &mut TestAppContext) {
        fn assert_tool(
            tool_working_set: &ToolWorkingSet,
            unique_name: &str,
            expected_name: &str,
            expected_source: ToolSource,
            cx: &App,
        ) {
            let tool = tool_working_set.tool(unique_name, cx).unwrap();
            assert_eq!(tool.name(), expected_name);
            assert_eq!(tool.source(), expected_source);
        }

        let tool_registry = cx.update(ToolRegistry::default_global);
        tool_registry.register_tool(TestTool::new("tool1", ToolSource::Native));
        tool_registry.register_tool(TestTool::new("tool2", ToolSource::Native));

        let mut tool_working_set = ToolWorkingSet::default();
        cx.update(|cx| {
            tool_working_set.extend(
                vec![
                    Arc::new(TestTool::new(
                        "tool2",
                        ToolSource::ContextServer { id: "mcp-1".into() },
                    )) as Arc<dyn Tool>,
                    Arc::new(TestTool::new(
                        "tool2",
                        ToolSource::ContextServer { id: "mcp-2".into() },
                    )) as Arc<dyn Tool>,
                ]
                .into_iter(),
                cx,
            );
        });

        cx.update(|cx| {
            assert_tool(&tool_working_set, "tool1", "tool1", ToolSource::Native, cx);
            assert_tool(&tool_working_set, "tool2", "tool2", ToolSource::Native, cx);
            assert_tool(
                &tool_working_set,
                "mcp-1_tool2",
                "tool2",
                ToolSource::ContextServer { id: "mcp-1".into() },
                cx,
            );
            assert_tool(
                &tool_working_set,
                "mcp-2_tool2",
                "tool2",
                ToolSource::ContextServer { id: "mcp-2".into() },
                cx,
            );
        })
    }

    #[gpui::test]
    fn test_resolve_context_server_tool_name_conflicts() {
        assert_resolve_context_server_tool_name_conflicts(
            vec![
                TestTool::new("tool1", ToolSource::Native),
                TestTool::new("tool2", ToolSource::Native),
            ],
            vec![TestTool::new(
                "tool3",
                ToolSource::ContextServer { id: "mcp-1".into() },
            )],
            vec!["tool3"],
        );

        assert_resolve_context_server_tool_name_conflicts(
            vec![
                TestTool::new("tool1", ToolSource::Native),
                TestTool::new("tool2", ToolSource::Native),
            ],
            vec![
                TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-1".into() }),
                TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-2".into() }),
            ],
            vec!["mcp-1_tool3", "mcp-2_tool3"],
        );

        assert_resolve_context_server_tool_name_conflicts(
            vec![
                TestTool::new("tool1", ToolSource::Native),
                TestTool::new("tool2", ToolSource::Native),
                TestTool::new("tool3", ToolSource::Native),
            ],
            vec![
                TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-1".into() }),
                TestTool::new("tool3", ToolSource::ContextServer { id: "mcp-2".into() }),
            ],
            vec!["mcp-1_tool3", "mcp-2_tool3"],
        );

        // Test deduplication of tools with very long names, in this case the mcp server name should be truncated
        assert_resolve_context_server_tool_name_conflicts(
            vec![TestTool::new(
                "tool-with-very-very-very-long-name",
                ToolSource::Native,
            )],
            vec![TestTool::new(
                "tool-with-very-very-very-long-name",
                ToolSource::ContextServer {
                    id: "mcp-with-very-very-very-long-name".into(),
                },
            )],
            vec!["mcp-with-very-very-very-long-_tool-with-very-very-very-long-name"],
        );

        fn assert_resolve_context_server_tool_name_conflicts(
            builtin_tools: Vec<TestTool>,
            context_server_tools: Vec<TestTool>,
            expected: Vec<&'static str>,
        ) {
            let context_server_tools: Vec<Arc<dyn Tool>> = context_server_tools
                .into_iter()
                .map(|t| Arc::new(t) as Arc<dyn Tool>)
                .collect();
            let builtin_tools: Vec<Arc<dyn Tool>> = builtin_tools
                .into_iter()
                .map(|t| Arc::new(t) as Arc<dyn Tool>)
                .collect();
            let tools =
                resolve_context_server_tool_name_conflicts(&context_server_tools, &builtin_tools);
            assert_eq!(tools.len(), expected.len());
            for (i, (name, _)) in tools.into_iter().enumerate() {
                assert_eq!(
                    name.0.as_ref(),
                    expected[i],
                    "Expected '{}' got '{}' at index {}",
                    expected[i],
                    name,
                    i
                );
            }
        }
    }

    struct TestTool {
        name: String,
        source: ToolSource,
    }

    impl TestTool {
        fn new(name: impl Into<String>, source: ToolSource) -> Self {
            Self {
                name: name.into(),
                source,
            }
        }
    }

    impl Tool for TestTool {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn icon(&self) -> icons::IconName {
            icons::IconName::Ai
        }

        fn may_perform_edits(&self) -> bool {
            false
        }

        fn needs_confirmation(
            &self,
            _input: &serde_json::Value,
            _project: &Entity<Project>,
            _cx: &App,
        ) -> bool {
            true
        }

        fn source(&self) -> ToolSource {
            self.source.clone()
        }

        fn description(&self) -> String {
            "Test tool".to_string()
        }

        fn ui_text(&self, _input: &serde_json::Value) -> String {
            "Test tool".to_string()
        }

        fn run(
            self: Arc<Self>,
            _input: serde_json::Value,
            _request: Arc<LanguageModelRequest>,
            _project: Entity<Project>,
            _action_log: Entity<ActionLog>,
            _model: Arc<dyn LanguageModel>,
            _window: Option<AnyWindowHandle>,
            _cx: &mut App,
        ) -> ToolResult {
            ToolResult {
                output: Task::ready(Err(anyhow::anyhow!("No content"))),
                card: None,
            }
        }
    }
}
