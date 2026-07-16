//! Agent Setup page — configure which model each agent role uses.
//! Replaces the "External Agents" ACP page with a native agent role
//! configuration UI.
//!
//! Each role (Edit, Research, Terminal, General) gets a card showing
//! its assigned model and available tools. The role→model mapping is
//! the UI for the RouterConfig.

use std::ops::Range;

use gpui::{Entity, FocusHandle, ScrollHandle, prelude::*};
use settings::Settings as _;
use ui::{prelude::*, *};

use crate::SettingsWindow;
use crate::page_data::SubPageLink;

/// Render the Agent Setup page content.
pub(crate) fn render_agent_setup_page(
    _settings_window: &SettingsWindow,
    scroll_handle: &ScrollHandle,
    _window: &mut Window,
    cx: &mut Context<SettingsWindow>,
) -> AnyElement {
    // Read router config from AgentSettings (default-disabled until wired).
    // For now we show the default profiles so the user can see what's available.
    let config = agent_settings::AgentSettings::get_global(cx);
    let router_enabled = config.router.enabled;

    v_flex()
        .id("agent-setup-page")
        .size_full()
        .pt_2p5()
        .px_8()
        .pb_16()
        .track_scroll(scroll_handle)
        .overflow_y_scroll()
        .child(Label::new("Agent Setup"))
        .child(
            Label::new("Configure which model each agent role uses. When routing is enabled, the agent will dispatch tasks to the optimal model.")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(div().h_4())
        .child(render_router_toggle(router_enabled))
        .child(div().h_6())
        .child(render_role_card("Edit", "Code generation, refactoring, and file editing", "claude-sonnet-4", "anthropic", &["All tools"], cx))
        .child(div().h_3())
        .child(render_role_card("Research", "Web search, documentation lookup, and research questions", "gemini-2.0-flash", "google", &["search_web", "fetch", "read_file", "grep"], cx))
        .child(div().h_3())
        .child(render_role_card("Terminal", "Build, test, deploy, and terminal commands", "deepseek/deepseek-chat", "openrouter", &["terminal", "read_file"], cx))
        .child(div().h_3())
        .child(render_role_card("Planning", "Architecture, design, task decomposition, and orchestration", "claude-sonnet-4", "anthropic", &["All tools", "spawn_agent"], cx))
        .child(div().h_3())
        .child(render_role_card("Vision", "Image analysis, screenshots, and visual UI understanding", "gemini-2.0-flash", "google", &["read_file", "fetch"], cx))
        .child(div().h_3())
        .child(render_role_card("Review", "Code review, audit, and change verification", "claude-sonnet-4", "anthropic", &["read_file", "grep", "diagnostics"], cx))
        .child(div().h_3())
        .child(render_role_card("General", "Q&A, planning, and everything else", "(inherited)", "(default provider)", &["All tools"], cx))
        .child(div().h_6())
        .child(
            div()
                .p_3()
                .bg(cx.theme().colors().editor_background)
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .child(
                    v_flex()
                        .gap_2()
                        .child(Label::new("How routing works").size(LabelSize::Small).color(Color::Muted))
                        .child(
                            Label::new("When enabled, the agent inspects your prompt and routes it to the best model for the job. Edits go to a strong coder, research goes to a web-capable model, terminal ops go to a fast cheap model. The router is disabled by default — enable it above to activate.")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        ),
                ),
        )
        .into_any_element()
}

fn render_router_toggle(enabled: bool) -> impl IntoElement {
    let (label, description) = if enabled {
        ("Agent routing is enabled", "Tasks will be automatically dispatched to the optimal model per role.")
    } else {
        ("Agent routing is disabled", "All tasks use the default model. Enable routing below to activate role-based model selection.")
    };

    v_flex()
        .gap_2()
        .child(
            h_flex()
                .justify_between()
                .child(
                    v_flex()
                        .child(Label::new("Enable Agent Routing"))
                        .child(
                            Label::new(label)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted),
                        ),
                )
                .child(
                    // Toggle is read-only for now — wired when settings schema is connected
                    ToggleButton::new("router-toggle", enabled)
                        .size(ButtonSize::Large),
                ),
        )
        .child(
            Label::new(description)
                .size(LabelSize::XSmall)
                .color(Color::Muted),
        )
}

fn render_role_card(
    role: &str,
    description: &str,
    model: &str,
    provider: &str,
    tools: &[&str],
    cx: &mut Context<SettingsWindow>,
) -> impl IntoElement {
    v_flex()
        .p_4()
        .rounded_md()
        .border_1()
        .border_color(cx.theme().colors().border)
        .child(
            h_flex()
                .justify_between()
                .child(
                    h_flex()
                        .gap_2()
                        .child(role_icon(role))
                        .child(
                            v_flex()
                                .child(Label::new(format!("{} Agent", role)).size(LabelSize::Large))
                                .child(
                                    Label::new(description)
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                ),
                        ),
                ),
        )
        .child(div().h_3())
        .child(
            v_flex()
                .gap_1()
                .child(
                    h_flex()
                        .gap_1()
                        .child(Label::new("Model:").size(LabelSize::Small).color(Color::Muted))
                        .child(Label::new(model).size(LabelSize::Small)),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(Label::new("Provider:").size(LabelSize::Small).color(Color::Muted))
                        .child(Label::new(provider).size(LabelSize::Small)),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(Label::new("Tools:").size(LabelSize::Small).color(Color::Muted))
                        .child(Label::new(tools.join(", ")).size(LabelSize::Small)),
                ),
        )
}

fn role_icon(role: &str) -> IconName {
    match role {
        "Edit" => IconName::Pencil,
        "Research" => IconName::MagnifyingGlass,
        "Terminal" => IconName::Terminal,
        "Planning" => IconName::Sparkle,
        "Vision" => IconName::Image,
        "Review" => IconName::Check,
        "General" => IconName::Chat,
        _ => IconName::Ai,
    }
}

/// Data for the sub-page link in page_data.rs
pub(crate) fn agent_setup_sub_page_link() -> SubPageLink {
    SubPageLink {
        title: "Agent Setup".into(),
        r#type: Default::default(),
        json_path: Some("agent_setup"),
        description: Some(
            "Configure which model each agent role uses for different task types."
                .into(),
        ),
        search_aliases: &[
            "agent role",
            "agent setup",
            "edit agent",
            "research agent",
            "terminal agent",
            "model routing",
            "role based model",
            "task routing",
            "router",
            "auxiliary",
        ],
        in_json: false,
        files: settings::SettingsFile::User,
        render: |settings_window: &SettingsWindow,
                 scroll_handle: &ScrollHandle,
                 window: &mut Window,
                 cx: &mut Context<SettingsWindow>| {
            render_agent_setup_page(settings_window, scroll_handle, window, cx)
        },
    }
}
