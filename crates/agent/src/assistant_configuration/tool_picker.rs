use std::sync::Arc;

use assistant_settings::{
    AgentProfile, AgentProfileContent, AgentProfileId, AssistantSettings, AssistantSettingsContent,
    ContextServerPresetContent, VersionedAssistantSettingsContent,
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

impl ToolPicker {
    pub fn new(delegate: ToolPickerDelegate, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx).modal(false));
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
pub struct ToolEntry {
    pub name: Arc<str>,
    pub source: ToolSource,
}

pub struct ToolPickerDelegate {
    tool_picker: WeakEntity<ToolPicker>,
    thread_store: WeakEntity<ThreadStore>,
    fs: Arc<dyn Fs>,
    tools: Vec<ToolEntry>,
    profile_id: AgentProfileId,
    profile: AgentProfile,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl ToolPickerDelegate {
    pub fn new(
        fs: Arc<dyn Fs>,
        tool_set: Entity<ToolWorkingSet>,
        thread_store: WeakEntity<ThreadStore>,
        profile_id: AgentProfileId,
        profile: AgentProfile,
        cx: &mut Context<ToolPicker>,
    ) -> Self {
        let mut tool_entries = Vec::new();

        for (source, tools) in tool_set.read(cx).tools_by_source(cx) {
            tool_entries.extend(tools.into_iter().map(|tool| ToolEntry {
                name: tool.name().into(),
                source: source.clone(),
            }));
        }

        Self {
            tool_picker: cx.entity().downgrade(),
            thread_store,
            fs,
            tools: tool_entries,
            profile_id,
            profile,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for ToolPickerDelegate {
    type ListItem = ListItem;

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

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search tools…".into()
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self
            .tools
            .iter()
            .enumerate()
            .map(|(id, profile)| StringMatchCandidate::new(id, profile.name.as_ref()))
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
        let tool = &self.tools[candidate_id];

        let is_enabled = match &tool.source {
            ToolSource::Native => {
                let is_enabled = self.profile.tools.entry(tool.name.clone()).or_default();
                *is_enabled = !*is_enabled;
                *is_enabled
            }
            ToolSource::ContextServer { id } => {
                let preset = self
                    .profile
                    .context_servers
                    .entry(id.clone().into())
                    .or_default();
                let is_enabled = preset.tools.entry(tool.name.clone()).or_default();
                *is_enabled = !*is_enabled;
                *is_enabled
            }
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
            let tool = tool.clone();
            move |settings, _cx| match settings {
                AssistantSettingsContent::Versioned(boxed) => {
                    if let VersionedAssistantSettingsContent::V2(ref mut settings) = **boxed {
                        let profiles = settings.profiles.get_or_insert_default();
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

                        match tool.source {
                            ToolSource::Native => {
                                *profile.tools.entry(tool.name).or_default() = is_enabled;
                            }
                            ToolSource::ContextServer { id } => {
                                let preset = profile
                                    .context_servers
                                    .entry(id.clone().into())
                                    .or_default();
                                *preset.tools.entry(tool.name.clone()).or_default() = is_enabled;
                            }
                        }
                    }
                }
                _ => {}
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
        _cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let tool_match = &self.matches[ix];
        let tool = &self.tools[tool_match.candidate_id];

        let is_enabled = match &tool.source {
            ToolSource::Native => self.profile.tools.get(&tool.name).copied().unwrap_or(false),
            ToolSource::ContextServer { id } => self
                .profile
                .context_servers
                .get(id.as_ref())
                .and_then(|preset| preset.tools.get(&tool.name))
                .copied()
                .unwrap_or(false),
        };

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .gap_2()
                        .child(HighlightedLabel::new(
                            tool_match.string.clone(),
                            tool_match.positions.clone(),
                        ))
                        .map(|parent| match &tool.source {
                            ToolSource::Native => parent,
                            ToolSource::ContextServer { id } => parent
                                .child(Label::new(id).size(LabelSize::XSmall).color(Color::Muted)),
                        }),
                )
                .end_slot::<Icon>(is_enabled.then(|| {
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Success)
                })),
        )
    }
}
