use std::{collections::BTreeMap, sync::Arc};

use agent_settings::{AgentProfileId, AgentProfileSettings};
use assistant_tool::{ToolSource, ToolWorkingSet};
use fs::Fs;
use gpui::{App, Context, DismissEvent, Entity, EventEmitter, Focusable, Task, WeakEntity, Window};
use picker::{Picker, PickerDelegate};
use settings::{AgentProfileContent, ContextServerPresetContent, update_settings_file};
use ui::{ListItem, ListItemSpacing, prelude::*};
use util::ResultExt as _;

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

pub struct ToolPickerDelegate {
    tool_picker: WeakEntity<ToolPicker>,
    fs: Arc<dyn Fs>,
    items: Arc<Vec<PickerItem>>,
    profile_id: AgentProfileId,
    profile_settings: AgentProfileSettings,
    filtered_items: Vec<PickerItem>,
    selected_index: usize,
    mode: ToolPickerMode,
}

impl ToolPickerDelegate {
    pub fn new(
        mode: ToolPickerMode,
        fs: Arc<dyn Fs>,
        tool_set: Entity<ToolWorkingSet>,
        profile_id: AgentProfileId,
        profile_settings: AgentProfileSettings,
        cx: &mut Context<ToolPicker>,
    ) -> Self {
        let items = Arc::new(Self::resolve_items(mode, &tool_set, cx));

        Self {
            tool_picker: cx.entity().downgrade(),
            fs,
            items,
            profile_id,
            profile_settings,
            filtered_items: Vec::new(),
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
                    if mode == ToolPickerMode::McpTools && !tools.is_empty() {
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
        self.filtered_items.len()
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
        let item = &self.filtered_items[ix];
        match item {
            PickerItem::Tool { .. } => true,
            PickerItem::ContextServer { .. } => false,
        }
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        match self.mode {
            ToolPickerMode::BuiltinTools => "Search built-in tools…",
            ToolPickerMode::McpTools => "Search MCP tools…",
        }
        .into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let all_items = self.items.clone();

        cx.spawn_in(window, async move |this, cx| {
            let filtered_items = cx
                .background_spawn(async move {
                    let mut tools_by_provider: BTreeMap<Option<Arc<str>>, Vec<Arc<str>>> =
                        BTreeMap::default();

                    for item in all_items.iter() {
                        if let PickerItem::Tool { server_id, name } = item.clone()
                            && name.contains(&query)
                        {
                            tools_by_provider.entry(server_id).or_default().push(name);
                        }
                    }

                    let mut items = Vec::new();

                    for (server_id, names) in tools_by_provider {
                        if let Some(server_id) = server_id.clone() {
                            items.push(PickerItem::ContextServer { server_id });
                        }
                        for name in names {
                            items.push(PickerItem::Tool {
                                server_id: server_id.clone(),
                                name,
                            });
                        }
                    }

                    items
                })
                .await;

            this.update(cx, |this, _cx| {
                this.delegate.filtered_items = filtered_items;
                this.delegate.selected_index = this
                    .delegate
                    .selected_index
                    .min(this.delegate.filtered_items.len().saturating_sub(1));
            })
            .log_err();
        })
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.filtered_items.is_empty() {
            self.dismissed(window, cx);
            return;
        }

        let item = &self.filtered_items[self.selected_index];

        let PickerItem::Tool {
            name: tool_name,
            server_id,
        } = item
        else {
            return;
        };

        let is_currently_enabled = if let Some(server_id) = server_id.clone() {
            let preset = self
                .profile_settings
                .context_servers
                .entry(server_id)
                .or_default();
            let is_enabled = *preset.tools.entry(tool_name.clone()).or_default();
            *preset.tools.entry(tool_name.clone()).or_default() = !is_enabled;
            is_enabled
        } else {
            let is_enabled = *self
                .profile_settings
                .tools
                .entry(tool_name.clone())
                .or_default();
            *self
                .profile_settings
                .tools
                .entry(tool_name.clone())
                .or_default() = !is_enabled;
            is_enabled
        };

        update_settings_file(self.fs.clone(), cx, {
            let profile_id = self.profile_id.clone();
            let default_profile = self.profile_settings.clone();
            let server_id = server_id.clone();
            let tool_name = tool_name.clone();
            move |settings, _cx| {
                let profiles = settings
                    .agent
                    .get_or_insert_default()
                    .profiles
                    .get_or_insert_default();
                let profile = profiles
                    .entry(profile_id.0)
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
        let item = &self.filtered_items.get(ix)?;
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
                    self.profile_settings
                        .context_servers
                        .get(server_id.as_ref())
                        .and_then(|preset| preset.tools.get(name))
                        .copied()
                        .unwrap_or(self.profile_settings.enable_all_context_servers)
                } else {
                    self.profile_settings
                        .tools
                        .get(name)
                        .copied()
                        .unwrap_or(false)
                };

                Some(
                    ListItem::new(ix)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .toggle_state(selected)
                        .child(Label::new(name.clone()))
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
