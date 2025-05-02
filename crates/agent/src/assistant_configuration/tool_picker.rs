use std::sync::Arc;

use assistant_settings::{
    AgentProfile, AgentProfileContent, AgentProfileId, AssistantSettings, AssistantSettingsContent,
    ContextServerPresetContent,
};
use assistant_tool::{ToolSource, ToolWorkingSet};
use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{App, Context, DismissEvent, Entity, EventEmitter, Focusable, Task, WeakEntity, Window};
use picker::{Picker, PickerDelegate};
use settings::{Settings as _, update_settings_file};
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt as _;

use crate::ThreadStore;

pub struct ToolPicker {
    picker: Entity<Picker<ToolPickerDelegate>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolPickerMode {
    BuiltinTools,
    McpTools,
}

impl ToolPicker {
    pub fn builtin_tools(
        delegate: ToolPickerDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));
        Self { picker }
    }

    pub fn mcp_tools(
        delegate: ToolPickerDelegate,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let picker = cx.new(|cx| Picker::list(delegate, window, cx).modal(false));
        Self { picker }
    }
}

impl EventEmitter<DismissEvent> for ToolPicker {}

impl Focusable for ToolPicker {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ToolPicker {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

#[derive(Debug, Clone)]
pub enum PickerItem {
    Tool {
        server_id: Option<Arc<str>>,
        name: Arc<str>,
    },
    ContextServer {
        server_id: Arc<str>,
    },
}

impl PickerItem {
    pub fn name(&self) -> &str {
        match self {
            PickerItem::ContextServer { server_id, .. } => server_id.as_ref(),
            PickerItem::Tool { name, .. } => name.as_ref(),
        }
    }
}

pub struct ToolPickerDelegate {
    tool_picker: WeakEntity<ToolPicker>,
    thread_store: WeakEntity<ThreadStore>,
    fs: Arc<dyn Fs>,
    items: Vec<PickerItem>,
    profile_id: AgentProfileId,
    profile: AgentProfile,
    matches: Vec<StringMatch>,
    selected_index: usize,
    mode: ToolPickerMode,
}

impl ToolPickerDelegate {
    pub fn new(
        mode: ToolPickerMode,
        fs: Arc<dyn Fs>,
        tool_set: Entity<ToolWorkingSet>,
        thread_store: WeakEntity<ThreadStore>,
        profile_id: AgentProfileId,
        profile: AgentProfile,
        cx: &mut Context<ToolPicker>,
    ) -> Self {
        let items = Self::resolve_items(mode, &tool_set, cx);

        Self {
            tool_picker: cx.entity().downgrade(),
            thread_store,
            fs,
            items,
            profile_id,
            profile,
            matches: Vec::new(),
            selected_index: 0,
            mode,
        }
    }

    fn resolve_items(
        mode: ToolPickerMode,
        tool_set: &Entity<ToolWorkingSet>,
        cx: &mut App,
    ) -> Vec<PickerItem> {
        let mut items = Vec::new();
        for (source, tools) in tool_set.read(cx).tools_by_source(cx) {
            match source {
                ToolSource::Native => {
                    if mode == ToolPickerMode::BuiltinTools {
                        items.extend(tools.into_iter().map(|tool| PickerItem::Tool {
                            name: tool.name().into(),
                            server_id: None,
                        }));
                    }
                }
                ToolSource::ContextServer { id } => {
                    if mode == ToolPickerMode::McpTools {
                        let server_id: Arc<str> = id.clone().into();
                        items.push(PickerItem::ContextServer {
                            server_id: server_id.clone(),
                        });
                        items.extend(tools.into_iter().map(|tool| PickerItem::Tool {
                            name: tool.name().into(),
                            server_id: Some(server_id.clone()),
                        }));
                    }
                }
            }
        }
        items
    }
}

impl PickerDelegate for ToolPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn can_select(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        let item_match = &self.matches[ix];
        let item = &self.items[item_match.candidate_id];
        match item {
            PickerItem::Tool { .. } => true,
            PickerItem::ContextServer { .. } => false,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        match self.mode {
            ToolPickerMode::BuiltinTools => "Search built-in tools…",
            ToolPickerMode::McpTools => "Search MCP servers…",
        }
        .into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .items
            .iter()
            .enumerate()
            .map(|(id, item)| StringMatchCandidate::new(id, item.name()))
            .collect::<Vec<_>>();

        cx.spawn_in(window, async move |this, cx| {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(cx, |this, _cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.matches.len().saturating_sub(1));
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.matches.is_empty() {
            self.dismissed(window, cx);
            return;
        }

        let candidate_id = self.matches[self.selected_index].candidate_id;
        let item = &self.items[candidate_id];

        let PickerItem::Tool {
            name: tool_name,
            server_id,
        } = item
        else {
            return;
        };

        let is_currently_enabled = if let Some(server_id) = server_id.clone() {
            let preset = self.profile.context_servers.entry(server_id).or_default();
            *preset.tools.entry(tool_name.clone()).or_default()
        } else {
            *self.profile.tools.entry(tool_name.clone()).or_default()
        };

        let active_profile_id = &AssistantSettings::get_global(cx).default_profile;
        if active_profile_id == &self.profile_id {
            self.thread_store
                .update(cx, |this, cx| {
                    this.load_profile(self.profile.clone(), cx);
                })
                .log_err();
        }

        update_settings_file::<AssistantSettings>(self.fs.clone(), cx, {
            let profile_id = self.profile_id.clone();
            let default_profile = self.profile.clone();
            let server_id = server_id.clone();
            let tool_name = tool_name.clone();

            move |settings: &mut AssistantSettingsContent, _cx| {
                settings
                    .v2_setting(|v2_settings| {
                        let profiles = v2_settings.profiles.get_or_insert_default();
                        let profile =
                            profiles
                                .entry(profile_id)
                                .or_insert_with(|| AgentProfileContent {
                                    name: default_profile.name.into(),
                                    tools: default_profile.tools,
                                    enable_all_context_servers: Some(
                                        default_profile.enable_all_context_servers,
                                    ),
                                    context_servers: default_profile
                                        .context_servers
                                        .into_iter()
                                        .map(|(server_id, preset)| {
                                            (
                                                server_id,
                                                ContextServerPresetContent {
                                                    tools: preset.tools,
                                                },
                                            )
                                        })
                                        .collect(),
                                });

                        if let Some(server_id) = server_id {
                            let preset = profile.context_servers.entry(server_id).or_default();
                            *preset.tools.entry(tool_name).or_default() = !is_currently_enabled;
                        } else {
                            *profile.tools.entry(tool_name).or_default() = !is_currently_enabled;
                        }

                        Ok(())
                    })
                    .ok();
            }
        });
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.tool_picker
            .update(cx, |_this, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let item_match = &self.matches[ix];
        let item = &self.items[item_match.candidate_id];

        match item {
            PickerItem::ContextServer { server_id, .. } => Some(
                div()
                    .px_2()
                    .pb_1()
                    .when(ix > 1, |this| {
                        this.mt_1()
                            .pt_2()
                            .border_t_1()
                            .border_color(cx.theme().colors().border_variant)
                    })
                    .child(
                        Label::new(server_id)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    )
                    .into_any_element(),
            ),
            PickerItem::Tool { name, server_id } => {
                let is_enabled = if let Some(server_id) = server_id {
                    self.profile
                        .context_servers
                        .get(server_id.as_ref())
                        .and_then(|preset| preset.tools.get(name))
                        .copied()
                        .unwrap_or(self.profile.enable_all_context_servers)
                } else {
                    self.profile.tools.get(name).copied().unwrap_or(false)
                };

                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(HighlightedLabel::new(
                            item_match.string.clone(),
                            item_match.positions.clone(),
                        ))
                        .end_slot::<Icon>(is_enabled.then(|| {
                            Icon::new(IconName::Check)
                                .size(IconSize::Small)
                                .color(Color::Success)
                        }))
                        .into_any_element(),
                )
            }
        }
    }
}
