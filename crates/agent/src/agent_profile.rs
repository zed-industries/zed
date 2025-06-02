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

    pub fn is_enabled(settings: &AgentProfileSettings, source: ToolSource, name: String) -> bool {
        match source {
            ToolSource::Native => *settings.tools.get(&Arc::from(name)).unwrap_or(&false),
            ToolSource::ContextServer { id } => false,
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
    use assistant_tool::ToolRegistry;
    use gpui::{AppContext, TestAppContext};
    use http_client::FakeHttpClient;
    use settings::{Settings, SettingsStore};

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
        let tool_set = cx.new(|_| ToolWorkingSet::default());

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
            AgentSettings::register(cx);
            language_model::init_settings(cx);
            ToolRegistry::default_global(cx);
            assistant_tools::init(FakeHttpClient::with_404_response(), cx);
        });
    }
}
