use std::sync::Arc;

use agent_settings::{AgentProfileId, AgentProfileSettings, AgentSettings};
use assistant_tool::{Tool, ToolSource, ToolWorkingSet};
use gpui::{App, Entity};
use settings::Settings;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentProfile {
    id: AgentProfileId,
    tool_set: Entity<ToolWorkingSet>,
}

impl AgentProfile {
    pub fn new(id: AgentProfileId, tool_set: Entity<ToolWorkingSet>) -> Self {
        Self { id, tool_set }
    }

    pub fn id(&self) -> &AgentProfileId {
        &self.id
    }

    pub fn enabled_tools(&self, cx: &App) -> Vec<Arc<dyn Tool>> {
        let Some(settings) = self.settings(cx) else {
            return vec![];
        };
        self.tool_set
            .read(cx)
            .tools(cx)
            .into_iter()
            .filter(|tool| Self::is_enabled(&settings, tool.source(), tool.name()))
            .collect()
    }

    fn is_enabled(settings: &AgentProfileSettings, source: ToolSource, name: String) -> bool {
        match source {
            ToolSource::Native => *settings.tools.get(&Arc::from(name)).unwrap_or(&false),
            ToolSource::ContextServer { id } => {
                if settings.enable_all_context_servers {
                    return true;
                }

                let Some(preset) = settings.context_servers.get(&Arc::from(id)) else {
                    return false;
                };
                *preset.tools.get(&Arc::from(name)).unwrap_or(&false)
            }
        }
    }

    fn settings(&self, cx: &App) -> Option<AgentProfileSettings> {
        AgentSettings::get_global(cx)
            .profiles
            .get(&self.id)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use agent_settings::ContextServerPreset;
    use assistant_tool::ToolRegistry;
    use collections::IndexMap;
    use gpui::{AppContext, TestAppContext};
    use http_client::FakeHttpClient;
    use project::Project;
    use settings::{Settings, SettingsStore};
    use ui::SharedString;

    use super::*;

    #[gpui::test]
    async fn test_enabled_built_in_tools_for_profile(cx: &mut TestAppContext) {
        init_test_settings(cx);

        let id = AgentProfileId::default();
        let profile_settings = cx.read(|cx| {
            AgentSettings::get_global(cx)
                .profiles
                .get(&id)
                .unwrap()
                .clone()
        });
        let tool_set = default_tool_set(cx);

        let profile = AgentProfile::new(id.clone(), tool_set);

        let mut enabled_tools = cx
            .read(|cx| profile.enabled_tools(cx))
            .into_iter()
            .map(|tool| tool.name())
            .collect::<Vec<_>>();
        enabled_tools.sort();

        let mut expected_tools = profile_settings
            .tools
            .into_iter()
            .filter_map(|(tool, enabled)| enabled.then_some(tool.to_string()))
            // Provider dependent
            .filter(|tool| tool != "web_search")
            .collect::<Vec<_>>();
        // Plus all registered MCP tools
        expected_tools.extend(["enabled_mcp_tool".into(), "disabled_mcp_tool".into()]);
        expected_tools.sort();

        assert_eq!(enabled_tools, expected_tools);
    }

    #[gpui::test]
    async fn test_custom_mcp_settings(cx: &mut TestAppContext) {
        init_test_settings(cx);

        let id = AgentProfileId("custom_mcp".into());
        let profile_settings = cx.read(|cx| {
            AgentSettings::get_global(cx)
                .profiles
                .get(&id)
                .unwrap()
                .clone()
        });
        let tool_set = default_tool_set(cx);

        let profile = AgentProfile::new(id.clone(), tool_set);

        let mut enabled_tools = cx
            .read(|cx| profile.enabled_tools(cx))
            .into_iter()
            .map(|tool| tool.name())
            .collect::<Vec<_>>();
        enabled_tools.sort();

        let mut expected_tools = profile_settings.context_servers["mcp"]
            .tools
            .iter()
            .filter_map(|(key, enabled)| enabled.then(|| key.to_string()))
            .collect::<Vec<_>>();
        expected_tools.sort();

        assert_eq!(enabled_tools, expected_tools);
    }

    #[gpui::test]
    async fn test_only_built_in(cx: &mut TestAppContext) {
        init_test_settings(cx);

        let id = AgentProfileId("write_minus_mcp".into());
        let profile_settings = cx.read(|cx| {
            AgentSettings::get_global(cx)
                .profiles
                .get(&id)
                .unwrap()
                .clone()
        });
        let tool_set = default_tool_set(cx);

        let profile = AgentProfile::new(id.clone(), tool_set);

        let mut enabled_tools = cx
            .read(|cx| profile.enabled_tools(cx))
            .into_iter()
            .map(|tool| tool.name())
            .collect::<Vec<_>>();
        enabled_tools.sort();

        let mut expected_tools = profile_settings
            .tools
            .into_iter()
            .filter_map(|(tool, enabled)| enabled.then_some(tool.to_string()))
            // Provider dependent
            .filter(|tool| tool != "web_search")
            .collect::<Vec<_>>();
        expected_tools.sort();

        assert_eq!(enabled_tools, expected_tools);
    }

    fn init_test_settings(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            Project::init_settings(cx);
            AgentSettings::register(cx);
            language_model::init_settings(cx);
            ToolRegistry::default_global(cx);
            assistant_tools::init(FakeHttpClient::with_404_response(), cx);
        });

        cx.update(|cx| {
            let mut agent_settings = AgentSettings::get_global(cx).clone();
            agent_settings.profiles.insert(
                AgentProfileId("write_minus_mcp".into()),
                AgentProfileSettings {
                    name: "write_minus_mcp".into(),
                    enable_all_context_servers: false,
                    ..agent_settings.profiles[&AgentProfileId::default()].clone()
                },
            );
            agent_settings.profiles.insert(
                AgentProfileId("custom_mcp".into()),
                AgentProfileSettings {
                    name: "mcp".into(),
                    tools: IndexMap::default(),
                    enable_all_context_servers: false,
                    context_servers: IndexMap::from_iter([("mcp".into(), context_server_preset())]),
                },
            );
            AgentSettings::override_global(agent_settings, cx);
        })
    }

    fn context_server_preset() -> ContextServerPreset {
        ContextServerPreset {
            tools: IndexMap::from_iter([
                ("enabled_mcp_tool".into(), true),
                ("disabled_mcp_tool".into(), false),
            ]),
        }
    }

    fn default_tool_set(cx: &mut TestAppContext) -> Entity<ToolWorkingSet> {
        cx.new(|_| {
            let mut tool_set = ToolWorkingSet::default();
            tool_set.insert(Arc::new(FakeTool::new("enabled_mcp_tool", "mcp")));
            tool_set.insert(Arc::new(FakeTool::new("disabled_mcp_tool", "mcp")));
            tool_set
        })
    }

    struct FakeTool {
        name: String,
        source: SharedString,
    }

    impl FakeTool {
        fn new(name: impl Into<String>, source: impl Into<SharedString>) -> Self {
            Self {
                name: name.into(),
                source: source.into(),
            }
        }
    }

    impl Tool for FakeTool {
        fn name(&self) -> String {
            self.name.clone()
        }

        fn source(&self) -> ToolSource {
            ToolSource::ContextServer {
                id: self.source.clone(),
            }
        }

        fn description(&self) -> String {
            unimplemented!()
        }

        fn icon(&self) -> ui::IconName {
            unimplemented!()
        }

        fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &App) -> bool {
            unimplemented!()
        }

        fn ui_text(&self, _input: &serde_json::Value) -> String {
            unimplemented!()
        }

        fn run(
            self: Arc<Self>,
            _input: serde_json::Value,
            _request: Arc<language_model::LanguageModelRequest>,
            _project: Entity<Project>,
            _action_log: Entity<assistant_tool::ActionLog>,
            _model: Arc<dyn language_model::LanguageModel>,
            _window: Option<gpui::AnyWindowHandle>,
            _cx: &mut App,
        ) -> assistant_tool::ToolResult {
            unimplemented!()
        }
    }
}
