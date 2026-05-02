use std::sync::Arc;

use agent::ContextServerRegistry;
use agent_settings::{AgentProfileId, AgentSettings};
use collections::HashMap;
use context_server::ContextServerId;
use fs::Fs;
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, ScrollHandle, SharedString, Window,
    prelude::*,
};
use settings::{
    AgentProfileContent, ContextServerPresetContent, Settings, SettingsStore,
    update_settings_file,
};
use ui::{
    Button, ButtonStyle, Checkbox, Divider, DividerColor, LabelSize, Modal, ModalHeader,
    WithScrollbar, prelude::*,
};
use workspace::{ModalView, Workspace};

pub struct ConfigureContextServerToolsModal {
    context_server_id: ContextServerId,
    context_server_registry: Entity<ContextServerRegistry>,
    fs: Arc<dyn Fs>,
    focus_handle: FocusHandle,
    expanded_tools: HashMap<SharedString, bool>,
    scroll_handle: ScrollHandle,
    _settings_subscription: gpui::Subscription,
}

impl ConfigureContextServerToolsModal {
    fn new(
        context_server_id: ContextServerId,
        context_server_registry: Entity<ContextServerRegistry>,
        fs: Arc<dyn Fs>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings_subscription =
            cx.observe_global::<SettingsStore>(|_this, cx| cx.notify());

        Self {
            context_server_id,
            context_server_registry,
            fs,
            focus_handle: cx.focus_handle(),
            expanded_tools: HashMap::default(),
            scroll_handle: ScrollHandle::new(),
            _settings_subscription: settings_subscription,
        }
    }

    pub fn toggle(
        context_server_id: ContextServerId,
        context_server_registry: Entity<ContextServerRegistry>,
        fs: Arc<dyn Fs>,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        workspace.toggle_modal(window, cx, |window, cx| {
            Self::new(context_server_id, context_server_registry, fs, window, cx)
        });
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }

    fn active_profile_and_settings(
        &self,
        cx: &App,
    ) -> Option<(AgentProfileId, agent_settings::AgentProfileSettings)> {
        let settings = AgentSettings::get_global(cx);
        let profile_id = settings.default_profile.clone();
        let profile = settings.profiles.get(&profile_id)?.clone();
        Some((profile_id, profile))
    }

    fn is_tool_enabled(&self, tool_name: &str, cx: &App) -> bool {
        let settings = AgentSettings::get_global(cx);
        let Some(profile) = settings.profiles.get(&settings.default_profile) else {
            return false;
        };
        profile.is_context_server_tool_enabled(&self.context_server_id.0, tool_name)
    }

    fn toggle_tool(&self, tool_name: SharedString, cx: &mut Context<Self>) {
        let Some((profile_id, profile_settings)) = self.active_profile_and_settings(cx) else {
            return;
        };

        let is_currently_enabled =
            profile_settings.is_context_server_tool_enabled(&self.context_server_id.0, &tool_name);
        let server_id: Arc<str> = self.context_server_id.0.clone();

        update_settings_file(self.fs.clone(), cx, {
            let profile_id = profile_id.clone();
            let tool_name: Arc<str> = tool_name.as_ref().into();
            move |settings, _cx| {
                let profiles = settings
                    .agent
                    .get_or_insert_default()
                    .profiles
                    .get_or_insert_default();
                let profile = profiles
                    .entry(profile_id.0)
                    .or_insert_with(|| AgentProfileContent {
                        name: profile_settings.name.into(),
                        tools: profile_settings.tools,
                        enable_all_context_servers: Some(
                            profile_settings.enable_all_context_servers,
                        ),
                        context_servers: profile_settings
                            .context_servers
                            .into_iter()
                            .map(|(id, preset)| {
                                (
                                    id,
                                    ContextServerPresetContent {
                                        tools: preset.tools,
                                    },
                                )
                            })
                            .collect(),
                        default_model: profile_settings.default_model.clone(),
                    });

                let preset = profile.context_servers.entry(server_id).or_default();
                *preset.tools.entry(tool_name).or_default() = !is_currently_enabled;
            }
        });
    }

    fn set_all_tools(&self, enabled: bool, cx: &mut Context<Self>) {
        let Some((profile_id, profile_settings)) = self.active_profile_and_settings(cx) else {
            return;
        };

        let tool_names: Vec<SharedString> = self
            .context_server_registry
            .read(cx)
            .tools_for_server(&self.context_server_id)
            .map(|tool| tool.name())
            .collect();

        let server_id: Arc<str> = self.context_server_id.0.clone();

        update_settings_file(self.fs.clone(), cx, {
            let profile_id = profile_id.clone();
            move |settings, _cx| {
                let profiles = settings
                    .agent
                    .get_or_insert_default()
                    .profiles
                    .get_or_insert_default();
                let profile = profiles
                    .entry(profile_id.0)
                    .or_insert_with(|| AgentProfileContent {
                        name: profile_settings.name.into(),
                        tools: profile_settings.tools,
                        enable_all_context_servers: Some(
                            profile_settings.enable_all_context_servers,
                        ),
                        context_servers: profile_settings
                            .context_servers
                            .into_iter()
                            .map(|(id, preset)| {
                                (
                                    id,
                                    ContextServerPresetContent {
                                        tools: preset.tools,
                                    },
                                )
                            })
                            .collect(),
                        default_model: profile_settings.default_model.clone(),
                    });

                let preset = profile.context_servers.entry(server_id).or_default();
                for tool_name in tool_names {
                    let name: Arc<str> = tool_name.as_ref().into();
                    *preset.tools.entry(name).or_default() = enabled;
                }
            }
        });
    }

    fn enabled_count(&self, cx: &App) -> usize {
        self.context_server_registry
            .read(cx)
            .tools_for_server(&self.context_server_id)
            .filter(|tool| self.is_tool_enabled(&tool.name(), cx))
            .count()
    }

    fn render_modal_content(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let tools: Vec<_> = self
            .context_server_registry
            .read(cx)
            .tools_for_server(&self.context_server_id)
            .collect();

        let total_count = tools.len();
        let enabled_count = self.enabled_count(cx);
        let all_enabled = enabled_count == total_count;

        div()
            .size_full()
            .pb_2()
            .child(
                h_flex()
                    .px_3()
                    .py_1p5()
                    .justify_between()
                    .child(
                        Label::new(format!("{}/{} enabled", enabled_count, total_count))
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    )
                    .child(
                        Button::new(
                            "toggle-all",
                            if all_enabled {
                                "Disable All"
                            } else {
                                "Enable All"
                            },
                        )
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small)
                        .on_click(cx.listener(move |this, _event, _window, cx| {
                            this.set_all_tools(!all_enabled, cx);
                        })),
                    ),
            )
            .child(Divider::horizontal().color(DividerColor::Border))
            .child(
                v_flex()
                    .id("modal_content")
                    .px_2()
                    .gap_0p5()
                    .max_h_96()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .children(tools.iter().enumerate().flat_map(|(index, tool)| {
                        let tool_name = tool.name();
                        let is_enabled = self.is_tool_enabled(&tool_name, cx);
                        let is_expanded = self
                            .expanded_tools
                            .get(tool_name.as_ref())
                            .copied()
                            .unwrap_or(false);

                        let icon = if is_expanded {
                            IconName::ChevronUp
                        } else {
                            IconName::ChevronDown
                        };

                        let selection: ToggleState = if is_enabled {
                            ToggleState::Selected
                        } else {
                            ToggleState::Unselected
                        };

                        let mut items = vec![
                            v_flex()
                                .child(
                                    h_flex()
                                        .id(format!("tool-header-{}", index))
                                        .py_1()
                                        .pl_1()
                                        .pr_2()
                                        .w_full()
                                        .items_center()
                                        .gap_1p5()
                                        .rounded_sm()
                                        .hover(|s| s.bg(cx.theme().colors().element_hover))
                                        .child(
                                            Checkbox::new(
                                                format!("tool-toggle-{}", index),
                                                selection,
                                            )
                                            .on_click(cx.listener({
                                                let tool_name = tool_name.clone();
                                                move |this, _state, _window, cx| {
                                                    this.toggle_tool(tool_name.clone(), cx);
                                                }
                                            })),
                                        )
                                        .child(
                                            h_flex()
                                                .id(format!("tool-expand-{}", index))
                                                .flex_1()
                                                .min_w_0()
                                                .justify_between()
                                                .child(
                                                    Label::new(tool_name.clone())
                                                        .buffer_font(cx)
                                                        .size(LabelSize::Small),
                                                )
                                                .child(
                                                    Icon::new(icon)
                                                        .size(IconSize::Small)
                                                        .color(Color::Muted),
                                                )
                                                .on_click(cx.listener({
                                                    let tool_name = tool_name.clone();
                                                    move |this, _event, _window, cx| {
                                                        let current = this
                                                            .expanded_tools
                                                            .get(tool_name.as_ref())
                                                            .copied()
                                                            .unwrap_or(false);
                                                        this.expanded_tools
                                                            .insert(tool_name.clone(), !current);
                                                        cx.notify();
                                                    }
                                                })),
                                        ),
                                )
                                .when(is_expanded, |this| {
                                    this.child(
                                        Label::new(tool.description())
                                            .color(Color::Muted)
                                            .size(LabelSize::Small)
                                            .mx_1()
                                            .ml_6(),
                                    )
                                })
                                .into_any_element(),
                        ];

                        if index < tools.len() - 1 {
                            items.push(
                                h_flex()
                                    .w_full()
                                    .child(Divider::horizontal().color(DividerColor::BorderVariant))
                                    .into_any_element(),
                            );
                        }

                        items
                    })),
            )
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
            .into_any_element()
    }
}

impl ModalView for ConfigureContextServerToolsModal {}

impl Focusable for ConfigureContextServerToolsModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ConfigureContextServerToolsModal {}

impl Render for ConfigureContextServerToolsModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let enabled_count = self.enabled_count(cx);
        let total_count = self
            .context_server_registry
            .read(cx)
            .tools_for_server(&self.context_server_id)
            .count();

        let headline = format!(
            "{} — {}/{} tools enabled",
            self.context_server_id.0, enabled_count, total_count
        );

        div()
            .key_context("ContextServerToolsModal")
            .occlude()
            .elevation_3(cx)
            .w(rems(34.))
            .on_action(cx.listener(Self::cancel))
            .track_focus(&self.focus_handle)
            .child(
                Modal::new("configure-context-server-tools", None::<ScrollHandle>)
                    .header(
                        ModalHeader::new()
                            .headline(headline)
                            .show_dismiss_button(true),
                    )
                    .child(self.render_modal_content(window, cx)),
            )
    }
}
