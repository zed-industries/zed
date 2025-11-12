use super::*;
use agent_settings::{AgentProfileId, AgentSettings, CompletionMode};
use command_palette_hooks::CommandPaletteFilter;
use gpui::{TestAppContext, px};
use project::DisableAiSettings;
use settings::{DefaultAgentView, DockPosition, NotifyWhenAgentWaiting, Settings, SettingsStore};

#[gpui::test]
fn test_agent_command_palette_visibility(cx: &mut TestAppContext) {
    // Init settings
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        command_palette_hooks::init(cx);
        AgentSettings::register(cx);
        DisableAiSettings::register(cx);
    });

    let agent_settings = AgentSettings {
        enabled: true,
        button: true,
        dock: DockPosition::Right,
        default_width: px(300.),
        default_height: px(600.),
        default_model: None,
        inline_assistant_model: None,
        commit_message_model: None,
        thread_summary_model: None,
        inline_alternatives: vec![],
        default_profile: AgentProfileId::default(),
        default_view: DefaultAgentView::Thread,
        profiles: Default::default(),
        always_allow_tool_actions: false,
        notify_when_agent_waiting: NotifyWhenAgentWaiting::default(),
        play_sound_when_agent_done: false,
        single_file_review: false,
        model_parameters: vec![],
        preferred_completion_mode: CompletionMode::Normal,
        enable_feedback: false,
        expand_edit_card: true,
        expand_terminal_card: true,
        use_modifier_to_send: true,
        message_editor_min_lines: 1,
    };

    cx.update(|cx| {
        AgentSettings::override_global(agent_settings.clone(), cx);
        DisableAiSettings::override_global(DisableAiSettings { disable_ai: false }, cx);

        // Initial update
        update_command_palette_filter(cx);
    });

    // Assert visible
    cx.update(|cx| {
        let filter = CommandPaletteFilter::try_global(cx).unwrap();
        assert!(
            !filter.is_hidden(&NewThread),
            "NewThread should be visible by default"
        );
    });

    // Disable agent
    cx.update(|cx| {
        let mut new_settings = agent_settings.clone();
        new_settings.enabled = false;
        AgentSettings::override_global(new_settings, cx);

        // Trigger update
        update_command_palette_filter(cx);
    });

    // Assert hidden
    cx.update(|cx| {
        let filter = CommandPaletteFilter::try_global(cx).unwrap();
        assert!(
            filter.is_hidden(&NewThread),
            "NewThread should be hidden when agent is disabled"
        );
    });
}
