mod profile_modal_header;

use std::sync::Arc;

use agent::{ContextServerRegistry, SectionGraph, TemplateSectionGraph, Templates};
use agent_settings::{
    AgentProfile, AgentProfileId, AgentSettings, PromptSectionOverride, builtin_profiles,
};
use collections::{BTreeSet, HashMap};
use editor::{Editor, EditorMode};
use fs::Fs;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription, prelude::*};
use language::Buffer;
use language_model::{LanguageModel, LanguageModelRegistry};
use multi_buffer::MultiBuffer;
use settings::SettingsStore;
use settings::{
    AgentProfileContent, ContextServerPresetContent, LanguageModelProviderSetting,
    LanguageModelSelection, PromptSectionOverrideContent, Settings as _, SoftWrap,
    update_settings_file,
};
use ui::Tooltip;
use ui::{
    IconButton, IconButtonShape, KeyBinding, ListItem, ListItemSpacing, ListSeparator, Navigable,
    NavigableEntry, prelude::*,
};
use util::ResultExt as _;
use workspace::{ModalView, Workspace};
use zed_actions::editor::{MoveDown, MoveUp};

use crate::agent_configuration::manage_profiles_modal::profile_modal_header::ProfileModalHeader;
use crate::agent_configuration::tool_picker::{ToolPicker, ToolPickerDelegate};
use crate::language_model_selector::{LanguageModelSelector, language_model_selector};
use crate::{AgentPanel, ManageProfiles};

enum Mode {
    ChooseProfile(ChooseProfileMode),
    NewProfile(NewProfileMode),
    ViewProfile(ViewProfileMode),
    ConfigureTools {
        profile_id: AgentProfileId,
        tool_picker: Entity<ToolPicker>,
        _subscription: Subscription,
    },
    ConfigureMcps {
        profile_id: AgentProfileId,
        tool_picker: Entity<ToolPicker>,
        _subscription: Subscription,
    },
    ConfigureDefaultModel {
        profile_id: AgentProfileId,
        model_picker: Entity<LanguageModelSelector>,
        _subscription: Subscription,
    },
    EditSectionOverride(EditSectionOverrideMode),
    ConfigurePrompts {
        profile_id: AgentProfileId,
        create_section_override: NavigableEntry,
    },
}

impl Mode {
    pub fn choose_profile(_window: &mut Window, cx: &mut Context<ManageProfilesModal>) -> Self {
        let settings = AgentSettings::get_global(cx);

        let mut builtin_profiles = Vec::new();
        let mut custom_profiles = Vec::new();

        for (profile_id, profile) in settings.profiles.iter() {
            let entry = ProfileEntry {
                id: profile_id.clone(),
                name: profile.name.clone(),
                navigation: NavigableEntry::focusable(cx),
            };
            if builtin_profiles::is_builtin(profile_id) {
                builtin_profiles.push(entry);
            } else {
                custom_profiles.push(entry);
            }
        }

        builtin_profiles.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        custom_profiles.sort_unstable_by(|a, b| a.name.cmp(&b.name));

        Self::ChooseProfile(ChooseProfileMode {
            builtin_profiles,
            custom_profiles,
            add_new_profile: NavigableEntry::focusable(cx),
        })
    }
}

#[derive(Clone)]
struct ProfileEntry {
    pub id: AgentProfileId,
    pub name: SharedString,
    pub navigation: NavigableEntry,
}

#[derive(Clone)]
pub struct ChooseProfileMode {
    builtin_profiles: Vec<ProfileEntry>,
    custom_profiles: Vec<ProfileEntry>,
    add_new_profile: NavigableEntry,
}

#[derive(Clone)]
pub struct ViewProfileMode {
    profile_id: AgentProfileId,
    fork_profile: NavigableEntry,
    configure_default_model: NavigableEntry,
    configure_tools: NavigableEntry,
    configure_mcps: NavigableEntry,
    configure_prompts: NavigableEntry,
    delete_profile: NavigableEntry,
    cancel_item: NavigableEntry,
}

#[derive(Clone)]
pub struct NewProfileMode {
    name_editor: Entity<Editor>,
    base_profile_id: Option<AgentProfileId>,
}

pub struct EditSectionOverrideMode {
    profile_id: AgentProfileId,
    original_section: Option<Arc<str>>,
    section_editor: Entity<Editor>,
    replacement_editor: Entity<Editor>,
    save_override: NavigableEntry,
    cancel_item: NavigableEntry,
    suggestion_index: Option<usize>,
    last_section_name: String,
}

pub struct ManageProfilesModal {
    fs: Arc<dyn Fs>,
    context_server_registry: Entity<ContextServerRegistry>,
    active_model: Option<Arc<dyn LanguageModel>>,
    focus_handle: FocusHandle,
    mode: Mode,
    section_graph: Option<SectionGraph>,
    section_defaults: Option<HashMap<String, String>>,
    _settings_subscription: Subscription,
}

impl ManageProfilesModal {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, action: &ManageProfiles, window, cx| {
            if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                let fs = workspace.app_state().fs.clone();
                let active_model = panel
                    .read(cx)
                    .active_native_agent_thread(cx)
                    .and_then(|thread| thread.read(cx).model().cloned());

                let context_server_registry = panel.read(cx).context_server_registry().clone();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let mut this = Self::new(fs, active_model, context_server_registry, window, cx);

                    if let Some(profile_id) = action.customize_tools.clone() {
                        this.configure_builtin_tools(profile_id, window, cx);
                    }

                    this
                })
            }
        });
    }

    pub fn new(
        fs: Arc<dyn Fs>,
        active_model: Option<Arc<dyn LanguageModel>>,
        context_server_registry: Entity<ContextServerRegistry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let section_graph = Templates::section_graph().log_err();
        let section_defaults = Templates::section_defaults()
            .log_err()
            .map(|defaults| defaults.into_iter().collect());

        // Keep this modal in sync with settings changes (including profile deletion).
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, |this, window, cx| {
                if matches!(this.mode, Mode::ChooseProfile(_)) {
                    this.mode = Mode::choose_profile(window, cx);
                    this.focus_handle(cx).focus(window, cx);
                }
                cx.notify();
            });

        Self {
            fs,
            active_model,
            context_server_registry,
            focus_handle,
            mode: Mode::choose_profile(window, cx),
            section_graph,
            section_defaults,
            _settings_subscription: settings_subscription,
        }
    }

    fn choose_profile(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.mode = Mode::choose_profile(window, cx);
        self.focus_handle(cx).focus(window, cx);
    }

    fn new_profile(
        &mut self,
        base_profile_id: Option<AgentProfileId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name_editor = cx.new(|cx| Editor::single_line(window, cx));
        name_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Profile name", window, cx);
        });

        self.mode = Mode::NewProfile(NewProfileMode {
            name_editor,
            base_profile_id,
        });
        self.focus_handle(cx).focus(window, cx);
    }

    pub fn view_profile(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = Mode::ViewProfile(ViewProfileMode {
            profile_id,
            fork_profile: NavigableEntry::focusable(cx),
            configure_default_model: NavigableEntry::focusable(cx),
            configure_tools: NavigableEntry::focusable(cx),
            configure_mcps: NavigableEntry::focusable(cx),
            configure_prompts: NavigableEntry::focusable(cx),
            delete_profile: NavigableEntry::focusable(cx),
            cancel_item: NavigableEntry::focusable(cx),
        });
        self.focus_handle(cx).focus(window, cx);
    }

    fn configure_default_model(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let fs = self.fs.clone();
        let profile_id_for_closure = profile_id.clone();

        let model_picker = cx.new(|cx| {
            let profile_id = profile_id_for_closure.clone();

            language_model_selector(
                {
                    let profile_id = profile_id.clone();
                    move |cx| {
                        let settings = AgentSettings::get_global(cx);

                        settings
                            .profiles
                            .get(&profile_id)
                            .and_then(|profile| profile.default_model.as_ref())
                            .and_then(|selection| {
                                let registry = LanguageModelRegistry::read_global(cx);
                                let provider_id = language_model::LanguageModelProviderId(
                                    gpui::SharedString::from(selection.provider.0.clone()),
                                );
                                let provider = registry.provider(&provider_id)?;
                                let model = provider
                                    .provided_models(cx)
                                    .iter()
                                    .find(|m| m.id().0 == selection.model.as_str())?
                                    .clone();
                                Some(language_model::ConfiguredModel { provider, model })
                            })
                    }
                },
                {
                    let fs = fs.clone();
                    move |model, cx| {
                        let provider = model.provider_id().0.to_string();
                        let model_id = model.id().0.to_string();
                        let profile_id = profile_id.clone();

                        update_settings_file(fs.clone(), cx, move |settings, _cx| {
                            let agent_settings = settings.agent.get_or_insert_default();
                            if let Some(profiles) = agent_settings.profiles.as_mut() {
                                if let Some(profile) = profiles.get_mut(profile_id.0.as_ref()) {
                                    profile.default_model = Some(LanguageModelSelection {
                                        provider: LanguageModelProviderSetting(provider.clone()),
                                        model: model_id.clone(),
                                    });
                                }
                            }
                        });
                    }
                },
                {
                    let fs = fs.clone();
                    move |model, should_be_favorite, cx| {
                        crate::favorite_models::toggle_in_settings(
                            model,
                            should_be_favorite,
                            fs.clone(),
                            cx,
                        );
                    }
                },
                false, // Do not use popover styles for the model picker
                self.focus_handle.clone(),
                window,
                cx,
            )
            .modal(false)
        });

        let dismiss_subscription = cx.subscribe_in(&model_picker, window, {
            let profile_id = profile_id.clone();
            move |this, _picker, _: &DismissEvent, window, cx| {
                this.view_profile(profile_id.clone(), window, cx);
            }
        });

        self.mode = Mode::ConfigureDefaultModel {
            profile_id,
            model_picker,
            _subscription: dismiss_subscription,
        };
        self.focus_handle(cx).focus(window, cx);
    }

    fn configure_mcp_tools(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let settings = AgentSettings::get_global(cx);
        let Some(profile) = settings.profiles.get(&profile_id).cloned() else {
            return;
        };

        let tool_picker = cx.new(|cx| {
            let delegate = ToolPickerDelegate::mcp_tools(
                &self.context_server_registry,
                self.fs.clone(),
                profile_id.clone(),
                profile,
                cx,
            );
            ToolPicker::mcp_tools(delegate, window, cx)
        });
        let dismiss_subscription = cx.subscribe_in(&tool_picker, window, {
            let profile_id = profile_id.clone();
            move |this, _tool_picker, _: &DismissEvent, window, cx| {
                this.view_profile(profile_id.clone(), window, cx);
            }
        });

        self.mode = Mode::ConfigureMcps {
            profile_id,
            tool_picker,
            _subscription: dismiss_subscription,
        };
        self.focus_handle(cx).focus(window, cx);
    }

    fn configure_prompts(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = Mode::ConfigurePrompts {
            profile_id,
            create_section_override: NavigableEntry::focusable(cx),
        };
        self.focus_handle(cx).focus(window, cx);
    }

    fn create_section_override(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.edit_section_override(profile_id, None, window, cx);
    }

    fn edit_section_override(
        &mut self,
        profile_id: AgentProfileId,
        override_entry: Option<PromptSectionOverride>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let section_editor = cx.new(|cx| Editor::single_line(window, cx));
        section_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Section name (e.g. system_prompt_intro)", window, cx);
        });

        let replacement_editor = cx.new(|cx| {
            let buffer = cx.new(|cx| Buffer::local("", cx));
            let buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
            let mut editor = Editor::new(
                EditorMode::AutoHeight {
                    min_lines: 6,
                    max_lines: Some(16),
                },
                buffer,
                None,
                window,
                cx,
            );
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_placeholder_text("Replacement template", window, cx);
            editor
        });

        let original_section = override_entry
            .as_ref()
            .map(|override_entry| override_entry.section.clone());
        let mut last_section_name = override_entry
            .as_ref()
            .map(|override_entry| override_entry.section.to_string())
            .unwrap_or_default();
        if let Some(override_entry) = override_entry {
            section_editor.update(cx, |editor, cx| {
                editor.set_text(override_entry.section.as_ref(), window, cx);
            });
            replacement_editor.update(cx, |editor, cx| {
                editor.set_text(override_entry.replacement.as_ref(), window, cx);
            });
        }

        if last_section_name.is_empty() {
            last_section_name = section_editor.read(cx).text(cx).trim().to_string();
        }

        self.mode = Mode::EditSectionOverride(EditSectionOverrideMode {
            profile_id,
            original_section,
            section_editor,
            replacement_editor,
            save_override: NavigableEntry::focusable(cx),
            cancel_item: NavigableEntry::focusable(cx),
            suggestion_index: None,
            last_section_name,
        });
        self.focus_handle(cx).focus(window, cx);
    }

    fn configure_builtin_tools(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let settings = AgentSettings::get_global(cx);
        let Some(profile) = settings.profiles.get(&profile_id).cloned() else {
            return;
        };

        //todo: This causes the web search tool to show up even it only works when using zed hosted models
        let tool_names: Vec<Arc<str>> = agent::supported_built_in_tool_names(
            self.active_model.as_ref().map(|model| model.provider_id()),
            cx,
        )
        .into_iter()
        .map(|s| Arc::from(s))
        .collect();

        let tool_picker = cx.new(|cx| {
            let delegate = ToolPickerDelegate::builtin_tools(
                tool_names,
                self.fs.clone(),
                profile_id.clone(),
                profile,
                cx,
            );
            ToolPicker::builtin_tools(delegate, window, cx)
        });
        let dismiss_subscription = cx.subscribe_in(&tool_picker, window, {
            let profile_id = profile_id.clone();
            move |this, _tool_picker, _: &DismissEvent, window, cx| {
                this.view_profile(profile_id.clone(), window, cx);
            }
        });

        self.mode = Mode::ConfigureTools {
            profile_id,
            tool_picker,
            _subscription: dismiss_subscription,
        };
        self.focus_handle(cx).focus(window, cx);
    }

    fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match &self.mode {
            Mode::ChooseProfile { .. } => {}
            Mode::NewProfile(mode) => {
                let name = mode.name_editor.read(cx).text(cx);

                let profile_id =
                    AgentProfile::create(name, mode.base_profile_id.clone(), self.fs.clone(), cx);
                self.view_profile(profile_id, window, cx);
            }
            Mode::ViewProfile(_) => {}
            Mode::ConfigureTools { .. } => {}
            Mode::ConfigureMcps { .. } => {}
            Mode::ConfigureDefaultModel { .. } => {}
            Mode::EditSectionOverride(_) => {}
            Mode::ConfigurePrompts { .. } => {}
        }
    }

    fn delete_profile(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if builtin_profiles::is_builtin(&profile_id) {
            self.view_profile(profile_id, window, cx);
            return;
        }

        let fs = self.fs.clone();

        update_settings_file(fs, cx, move |settings, _cx| {
            let Some(agent_settings) = settings.agent.as_mut() else {
                return;
            };

            let Some(profiles) = agent_settings.profiles.as_mut() else {
                return;
            };

            profiles.shift_remove(profile_id.0.as_ref());

            if agent_settings
                .default_profile
                .as_deref()
                .is_some_and(|default_profile| default_profile == profile_id.0.as_ref())
            {
                agent_settings.default_profile = Some(AgentProfileId::default().0);
            }
        });

        self.choose_profile(window, cx);
    }

    fn cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match &self.mode {
            Mode::ChooseProfile { .. } => {
                cx.emit(DismissEvent);
            }
            Mode::NewProfile(mode) => {
                if let Some(profile_id) = mode.base_profile_id.clone() {
                    self.view_profile(profile_id, window, cx);
                } else {
                    self.choose_profile(window, cx);
                }
            }
            Mode::ViewProfile(_) => self.choose_profile(window, cx),
            Mode::ConfigureTools { profile_id, .. } => {
                self.view_profile(profile_id.clone(), window, cx)
            }
            Mode::ConfigureMcps { profile_id, .. } => {
                self.view_profile(profile_id.clone(), window, cx)
            }
            Mode::ConfigureDefaultModel { profile_id, .. } => {
                self.view_profile(profile_id.clone(), window, cx)
            }
            Mode::EditSectionOverride(mode) => {
                self.configure_prompts(mode.profile_id.clone(), window, cx)
            }
            Mode::ConfigurePrompts { profile_id, .. } => {
                self.view_profile(profile_id.clone(), window, cx)
            }
        }
    }

    fn save_section_override(
        &mut self,
        profile_id: AgentProfileId,
        original_section: Option<Arc<str>>,
        section: String,
        replacement: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let section = section.trim().to_string();
        if section.is_empty() {
            self.configure_prompts(profile_id, window, cx);
            return;
        }
        if let Some(section_graph) = &self.section_graph
            && !section_exists(section_graph, &section)
        {
            return;
        }

        let default_profile = AgentSettings::get_global(cx)
            .profiles
            .get(&profile_id)
            .cloned();
        let Some(default_profile) = default_profile else {
            self.configure_prompts(profile_id, window, cx);
            return;
        };

        let fs = self.fs.clone();
        update_settings_file(fs, cx, {
            let profile_id = profile_id.clone();
            let section = section.clone();
            let replacement = replacement;
            let default_profile = default_profile.clone();
            move |settings, _cx| {
                let agent_settings = settings.agent.get_or_insert_default();
                let profiles = agent_settings.profiles.get_or_insert_default();
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
                        default_model: default_profile.default_model.clone(),
                        prompt_section_overrides: default_profile
                            .prompt_section_overrides
                            .iter()
                            .cloned()
                            .map(PromptSectionOverrideContent::from)
                            .collect(),
                        system_prompt_provider: default_profile.system_prompt_provider.clone(),
                    });

                if let Some(original_section) = original_section.as_deref()
                    && original_section != section
                {
                    profile.prompt_section_overrides.retain(|override_entry| {
                        override_entry.section.as_ref() != original_section
                    });
                }

                let overrides = &mut profile.prompt_section_overrides;
                if let Some(existing) = overrides
                    .iter_mut()
                    .find(|override_entry| override_entry.section.as_ref() == section)
                {
                    existing.replacement = replacement.clone().into();
                } else {
                    overrides.push(PromptSectionOverrideContent {
                        section: section.clone().into(),
                        replacement: replacement.clone().into(),
                    });
                }
            }
        });

        self.configure_prompts(profile_id, window, cx);
    }

    fn delete_section_override(
        &mut self,
        profile_id: AgentProfileId,
        section: Arc<str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let fs = self.fs.clone();
        update_settings_file(fs, cx, {
            let profile_id = profile_id.clone();
            let section = section.clone();
            move |settings, _cx| {
                let agent_settings = settings.agent.get_or_insert_default();
                let profiles = agent_settings.profiles.get_or_insert_default();
                let Some(profile) = profiles.get_mut(profile_id.0.as_ref()) else {
                    return;
                };

                profile
                    .prompt_section_overrides
                    .retain(|override_entry| override_entry.section.as_ref() != section.as_ref());
            }
        });

        self.configure_prompts(profile_id, window, cx);
    }
}

impl ModalView for ManageProfilesModal {}

impl Focusable for ManageProfilesModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            Mode::ChooseProfile(_) => self.focus_handle.clone(),
            Mode::NewProfile(mode) => mode.name_editor.focus_handle(cx),
            Mode::ViewProfile(_) => self.focus_handle.clone(),
            Mode::ConfigureTools { tool_picker, .. } => tool_picker.focus_handle(cx),
            Mode::ConfigureMcps { tool_picker, .. } => tool_picker.focus_handle(cx),
            Mode::ConfigureDefaultModel { model_picker, .. } => model_picker.focus_handle(cx),
            Mode::EditSectionOverride(mode) => mode.section_editor.focus_handle(cx),
            Mode::ConfigurePrompts { .. } => self.focus_handle.clone(),
        }
    }
}

impl EventEmitter<DismissEvent> for ManageProfilesModal {}

impl ManageProfilesModal {
    fn render_profile(
        &self,
        profile: &ProfileEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let is_focused = profile.navigation.focus_handle.contains_focused(window, cx);

        div()
            .id(format!("profile-{}", profile.id))
            .track_focus(&profile.navigation.focus_handle)
            .on_action({
                let profile_id = profile.id.clone();
                cx.listener(move |this, _: &menu::Confirm, window, cx| {
                    this.view_profile(profile_id.clone(), window, cx);
                })
            })
            .child(
                ListItem::new(format!("profile-{}", profile.id))
                    .toggle_state(is_focused)
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .child(Label::new(profile.name.clone()))
                    .when(is_focused, |this| {
                        this.end_slot(
                            h_flex()
                                .gap_1()
                                .child(
                                    Label::new("Customize")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(KeyBinding::for_action_in(
                                    &menu::Confirm,
                                    &self.focus_handle,
                                    cx,
                                )),
                        )
                    })
                    .on_click({
                        let profile_id = profile.id.clone();
                        cx.listener(move |this, _, window, cx| {
                            this.view_profile(profile_id.clone(), window, cx);
                        })
                    }),
            )
    }

    fn render_choose_profile(
        &mut self,
        mode: ChooseProfileMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Navigable::new(
            div()
                .track_focus(&self.focus_handle(cx))
                .size_full()
                .child(ProfileModalHeader::new("Agent Profiles", None))
                .child(
                    v_flex()
                        .pb_1()
                        .child(ListSeparator)
                        .children(
                            mode.builtin_profiles
                                .iter()
                                .map(|profile| self.render_profile(profile, window, cx)),
                        )
                        .when(!mode.custom_profiles.is_empty(), |this| {
                            this.child(ListSeparator)
                                .child(
                                    div().pl_2().pb_1().child(
                                        Label::new("Custom Profiles")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                                )
                                .children(
                                    mode.custom_profiles
                                        .iter()
                                        .map(|profile| self.render_profile(profile, window, cx)),
                                )
                        })
                        .child(ListSeparator)
                        .child(
                            div()
                                .id("new-profile")
                                .track_focus(&mode.add_new_profile.focus_handle)
                                .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                                    this.new_profile(None, window, cx);
                                }))
                                .child(
                                    ListItem::new("new-profile")
                                        .toggle_state(
                                            mode.add_new_profile
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(Icon::new(IconName::Plus))
                                        .child(Label::new("Add New Profile"))
                                        .on_click({
                                            cx.listener(move |this, _, window, cx| {
                                                this.new_profile(None, window, cx);
                                            })
                                        }),
                                ),
                        ),
                )
                .into_any_element(),
        )
        .map(|mut navigable| {
            for profile in mode.builtin_profiles {
                navigable = navigable.entry(profile.navigation);
            }
            for profile in mode.custom_profiles {
                navigable = navigable.entry(profile.navigation);
            }

            navigable
        })
        .entry(mode.add_new_profile)
    }

    fn render_new_profile(
        &mut self,
        mode: NewProfileMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let settings = AgentSettings::get_global(cx);

        let base_profile_name = mode.base_profile_id.as_ref().map(|base_profile_id| {
            settings
                .profiles
                .get(base_profile_id)
                .map(|profile| profile.name.clone())
                .unwrap_or_else(|| "Unknown".into())
        });

        v_flex()
            .id("new-profile")
            .track_focus(&self.focus_handle(cx))
            .child(ProfileModalHeader::new(
                match &base_profile_name {
                    Some(base_profile) => format!("Fork {base_profile}"),
                    None => "New Profile".into(),
                },
                match base_profile_name {
                    Some(_) => Some(IconName::Scissors),
                    None => Some(IconName::Plus),
                },
            ))
            .child(ListSeparator)
            .child(h_flex().p_2().child(mode.name_editor))
    }

    fn render_view_profile(
        &mut self,
        mode: ViewProfileMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let settings = AgentSettings::get_global(cx);

        let profile_name = settings
            .profiles
            .get(&mode.profile_id)
            .map(|profile| profile.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        let icon = match mode.profile_id.as_str() {
            "write" => IconName::Pencil,
            "ask" => IconName::Chat,
            _ => IconName::UserRoundPen,
        };

        Navigable::new(
            div()
                .track_focus(&self.focus_handle(cx))
                .size_full()
                .child(ProfileModalHeader::new(profile_name, Some(icon)))
                .child(
                    v_flex()
                        .pb_1()
                        .child(ListSeparator)
                        .child(
                            div()
                                .id("fork-profile")
                                .track_focus(&mode.fork_profile.focus_handle)
                                .on_action({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.new_profile(Some(profile_id.clone()), window, cx);
                                    })
                                })
                                .child(
                                    ListItem::new("fork-profile")
                                        .toggle_state(
                                            mode.fork_profile
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::Scissors)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(Label::new("Fork Profile"))
                                        .on_click({
                                            let profile_id = mode.profile_id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.new_profile(
                                                    Some(profile_id.clone()),
                                                    window,
                                                    cx,
                                                );
                                            })
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .id("configure-default-model")
                                .track_focus(&mode.configure_default_model.focus_handle)
                                .on_action({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.configure_default_model(
                                            profile_id.clone(),
                                            window,
                                            cx,
                                        );
                                    })
                                })
                                .child(
                                    ListItem::new("model-item")
                                        .toggle_state(
                                            mode.configure_default_model
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::ZedAssistant)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(Label::new("Configure Default Model"))
                                        .on_click({
                                            let profile_id = mode.profile_id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.configure_default_model(
                                                    profile_id.clone(),
                                                    window,
                                                    cx,
                                                );
                                            })
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .id("configure-builtin-tools")
                                .track_focus(&mode.configure_tools.focus_handle)
                                .on_action({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.configure_builtin_tools(
                                            profile_id.clone(),
                                            window,
                                            cx,
                                        );
                                    })
                                })
                                .child(
                                    ListItem::new("configure-builtin-tools-item")
                                        .toggle_state(
                                            mode.configure_tools
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::Settings)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(Label::new("Configure Built-in Tools"))
                                        .on_click({
                                            let profile_id = mode.profile_id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.configure_builtin_tools(
                                                    profile_id.clone(),
                                                    window,
                                                    cx,
                                                );
                                            })
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .id("configure-mcps")
                                .track_focus(&mode.configure_mcps.focus_handle)
                                .on_action({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.configure_mcp_tools(profile_id.clone(), window, cx);
                                    })
                                })
                                .child(
                                    ListItem::new("configure-mcp-tools")
                                        .toggle_state(
                                            mode.configure_mcps
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::ToolHammer)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(Label::new("Configure MCP Tools"))
                                        .on_click({
                                            let profile_id = mode.profile_id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.configure_mcp_tools(
                                                    profile_id.clone(),
                                                    window,
                                                    cx,
                                                );
                                            })
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .id("configure-prompts")
                                .track_focus(&mode.configure_prompts.focus_handle)
                                .on_action({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.configure_prompts(profile_id.clone(), window, cx);
                                    })
                                })
                                .child(
                                    ListItem::new("configure-prompts")
                                        .toggle_state(
                                            mode.configure_prompts
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::TextSnippet)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(Label::new("Configure Prompts"))
                                        .on_click({
                                            let profile_id = mode.profile_id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.configure_prompts(
                                                    profile_id.clone(),
                                                    window,
                                                    cx,
                                                );
                                            })
                                        }),
                                ),
                        )
                        .child(
                            div()
                                .id("delete-profile")
                                .track_focus(&mode.delete_profile.focus_handle)
                                .on_action({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.delete_profile(profile_id.clone(), window, cx);
                                    })
                                })
                                .child(
                                    ListItem::new("delete-profile")
                                        .toggle_state(
                                            mode.delete_profile
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::Trash)
                                                .size(IconSize::Small)
                                                .color(Color::Error),
                                        )
                                        .child(Label::new("Delete Profile").color(Color::Error))
                                        .disabled(builtin_profiles::is_builtin(&mode.profile_id))
                                        .on_click({
                                            let profile_id = mode.profile_id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.delete_profile(profile_id.clone(), window, cx);
                                            })
                                        }),
                                ),
                        )
                        .child(ListSeparator)
                        .child(
                            div()
                                .id("cancel-item")
                                .track_focus(&mode.cancel_item.focus_handle)
                                .on_action({
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.cancel(window, cx);
                                    })
                                })
                                .child(
                                    ListItem::new("cancel-item")
                                        .toggle_state(
                                            mode.cancel_item
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::ArrowLeft)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(Label::new("Go Back"))
                                        .end_slot(
                                            div().child(
                                                KeyBinding::for_action_in(
                                                    &menu::Cancel,
                                                    &self.focus_handle,
                                                    cx,
                                                )
                                                .size(rems_from_px(12.)),
                                            ),
                                        )
                                        .on_click({
                                            cx.listener(move |this, _, window, cx| {
                                                this.cancel(window, cx);
                                            })
                                        }),
                                ),
                        ),
                )
                .into_any_element(),
        )
        .entry(mode.fork_profile)
        .entry(mode.configure_default_model)
        .entry(mode.configure_tools)
        .entry(mode.configure_mcps)
        .entry(mode.configure_prompts)
        .entry(mode.delete_profile)
        .entry(mode.cancel_item)
    }
}

impl Render for ManageProfilesModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Mode::EditSectionOverride(mode) = &mut self.mode {
            let section_query = mode.section_editor.read(cx).text(cx);
            let section_query = section_query.trim();
            if mode.last_section_name != section_query {
                mode.last_section_name = section_query.to_string();
                mode.suggestion_index = None;
                if !section_query.is_empty() {
                    let replacement = self
                        .section_defaults
                        .as_ref()
                        .and_then(|defaults| defaults.get(section_query))
                        .cloned()
                        .unwrap_or_default();
                    mode.replacement_editor.update(cx, |editor, cx| {
                        editor.set_text(replacement, window, cx);
                    });
                }
            }
        }
        let settings = AgentSettings::get_global(cx);

        let go_back_item = div()
            .id("cancel-item")
            .track_focus(&self.focus_handle)
            .on_action({
                cx.listener(move |this, _: &menu::Confirm, window, cx| {
                    this.cancel(window, cx);
                })
            })
            .child(
                ListItem::new("cancel-item")
                    .toggle_state(self.focus_handle.contains_focused(window, cx))
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .start_slot(
                        Icon::new(IconName::ArrowLeft)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new("Go Back"))
                    .end_slot(
                        div().child(
                            KeyBinding::for_action_in(&menu::Cancel, &self.focus_handle, cx)
                                .size(rems_from_px(12.)),
                        ),
                    )
                    .on_click({
                        cx.listener(move |this, _, window, cx| {
                            this.cancel(window, cx);
                        })
                    }),
            );

        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ManageProfilesModal")
            .on_action(cx.listener(|this, _: &MoveDown, window, cx| {
                let Mode::EditSectionOverride(mode) = &mut this.mode else {
                    return;
                };
                if !mode
                    .section_editor
                    .focus_handle(cx)
                    .contains_focused(window, cx)
                {
                    return;
                }
                let section_query = mode.section_editor.read(cx).text(cx);
                let section_query = section_query.trim();
                let section_graph = this.section_graph.as_ref();
                let is_section_valid = section_graph
                    .map(|graph| section_exists(graph, section_query))
                    .unwrap_or(true);
                let suggestions = section_graph
                    .map(|graph| section_suggestions(graph, section_query))
                    .unwrap_or_default();
                if is_section_valid || suggestions.is_empty() {
                    return;
                }
                let next_index = match mode.suggestion_index {
                    Some(ix) => (ix + 1).min(suggestions.len() - 1),
                    None => 0,
                };
                mode.suggestion_index = Some(next_index);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &MoveUp, window, cx| {
                let Mode::EditSectionOverride(mode) = &mut this.mode else {
                    return;
                };
                if !mode
                    .section_editor
                    .focus_handle(cx)
                    .contains_focused(window, cx)
                {
                    return;
                }
                let section_query = mode.section_editor.read(cx).text(cx);
                let section_query = section_query.trim();
                let section_graph = this.section_graph.as_ref();
                let is_section_valid = section_graph
                    .map(|graph| section_exists(graph, section_query))
                    .unwrap_or(true);
                let suggestions = section_graph
                    .map(|graph| section_suggestions(graph, section_query))
                    .unwrap_or_default();
                if is_section_valid || suggestions.is_empty() {
                    return;
                }
                let next_index = match mode.suggestion_index {
                    Some(ix) => ix.saturating_sub(1),
                    None => suggestions.len() - 1,
                };
                mode.suggestion_index = Some(next_index);
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| this.cancel(window, cx)))
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
                let Mode::EditSectionOverride(mode) = &mut this.mode else {
                    return;
                };
                if !mode
                    .section_editor
                    .focus_handle(cx)
                    .contains_focused(window, cx)
                {
                    return;
                }
                let section_query = mode.section_editor.read(cx).text(cx);
                let section_query = section_query.trim();
                let section_graph = this.section_graph.as_ref();
                let is_section_valid = section_graph
                    .map(|graph| section_exists(graph, section_query))
                    .unwrap_or(true);
                let suggestions = section_graph
                    .map(|graph| section_suggestions(graph, section_query))
                    .unwrap_or_default();
                if is_section_valid || suggestions.is_empty() {
                    return;
                }
                let Some(index) = mode.suggestion_index else {
                    return;
                };
                let Some(section_name) = suggestions.get(index).cloned() else {
                    return;
                };
                mode.suggestion_index = None;
                mode.last_section_name = section_name.clone();
                mode.section_editor.update(cx, |editor, cx| {
                    editor.set_text(section_name.as_str(), window, cx);
                });
                let default_replacement = this
                    .section_defaults
                    .as_ref()
                    .and_then(|defaults| defaults.get(&section_name))
                    .cloned()
                    .unwrap_or_default();
                mode.replacement_editor.update(cx, |editor, cx| {
                    editor.set_text(default_replacement, window, cx);
                });
                cx.stop_propagation();
            }))
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| this.confirm(window, cx)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window, cx);
            }))
            .on_mouse_down_out(cx.listener(|_this, _, _, cx| cx.emit(DismissEvent)))
            .child(match &self.mode {
                Mode::ChooseProfile(mode) => self
                    .render_choose_profile(mode.clone(), window, cx)
                    .into_any_element(),
                Mode::NewProfile(mode) => self
                    .render_new_profile(mode.clone(), window, cx)
                    .into_any_element(),
                Mode::ViewProfile(mode) => self
                    .render_view_profile(mode.clone(), window, cx)
                    .into_any_element(),
                Mode::ConfigureTools {
                    profile_id,
                    tool_picker,
                    ..
                } => {
                    let profile_name = settings
                        .profiles
                        .get(profile_id)
                        .map(|profile| profile.name.clone())
                        .unwrap_or_else(|| "Unknown".into());

                    v_flex()
                        .pb_1()
                        .child(ProfileModalHeader::new(
                            format!("{profile_name} — Configure Built-in Tools"),
                            Some(IconName::Cog),
                        ))
                        .child(ListSeparator)
                        .child(tool_picker.clone())
                        .child(ListSeparator)
                        .child(go_back_item)
                        .into_any_element()
                }
                Mode::ConfigureDefaultModel {
                    profile_id,
                    model_picker,
                    ..
                } => {
                    let profile_name = settings
                        .profiles
                        .get(profile_id)
                        .map(|profile| profile.name.clone())
                        .unwrap_or_else(|| "Unknown".into());

                    v_flex()
                        .pb_1()
                        .child(ProfileModalHeader::new(
                            format!("{profile_name} — Configure Default Model"),
                            Some(IconName::Ai),
                        ))
                        .child(ListSeparator)
                        .child(v_flex().w(rems(34.)).child(model_picker.clone()))
                        .child(ListSeparator)
                        .child(go_back_item)
                        .into_any_element()
                }
                Mode::ConfigureMcps {
                    profile_id,
                    tool_picker,
                    ..
                } => {
                    let profile_name = settings
                        .profiles
                        .get(profile_id)
                        .map(|profile| profile.name.clone())
                        .unwrap_or_else(|| "Unknown".into());

                    v_flex()
                        .pb_1()
                        .child(ProfileModalHeader::new(
                            format!("{profile_name} — Configure MCP Tools"),
                            Some(IconName::ToolHammer),
                        ))
                        .child(ListSeparator)
                        .child(tool_picker.clone())
                        .child(ListSeparator)
                        .child(go_back_item)
                        .into_any_element()
                }
                Mode::EditSectionOverride(mode) => {
                    let profile_name = settings
                        .profiles
                        .get(&mode.profile_id)
                        .map(|profile| profile.name.clone())
                        .unwrap_or_else(|| "Unknown".into());
                    let original_section = mode.original_section.clone();
                    let profile_id = mode.profile_id.clone();
                    let section_editor = mode.section_editor.clone();
                    let replacement_editor = mode.replacement_editor.clone();
                    let section_query = section_editor.read(cx).text(cx);
                    let section_query = section_query.trim();
                    let section_graph = self.section_graph.as_ref();
                    let is_section_valid = section_graph
                        .map(|graph| section_exists(graph, section_query))
                        .unwrap_or(true);
                    let section_suggestions = section_graph
                        .map(|graph| section_suggestions(graph, section_query))
                        .unwrap_or_default();
                    let show_suggestions = !section_query.is_empty() && !is_section_valid;
                    let default_replacement = self
                        .section_defaults
                        .as_ref()
                        .and_then(|defaults| defaults.get(section_query))
                        .cloned();
                    let save_item = div()
                        .id("save-section-override")
                        .track_focus(&mode.save_override.focus_handle)
                        .on_action({
                            let profile_id = profile_id.clone();
                            let section_editor = section_editor.clone();
                            let replacement_editor = replacement_editor.clone();
                            let original_section = original_section.clone();
                            cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                let section = section_editor.read(cx).text(cx);
                                let replacement = replacement_editor.read(cx).text(cx);
                                this.save_section_override(
                                    profile_id.clone(),
                                    original_section.clone(),
                                    section,
                                    replacement,
                                    window,
                                    cx,
                                );
                            })
                        })
                        .child(
                            ListItem::new("save-section-override")
                                .toggle_state(
                                    mode.save_override.focus_handle.contains_focused(window, cx),
                                )
                                .inset(true)
                                .spacing(ListItemSpacing::Sparse)
                                .start_slot(
                                    Icon::new(IconName::Check)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(Label::new("Save Section Override"))
                                .disabled(!is_section_valid && !section_query.is_empty())
                                .on_click({
                                    let profile_id = profile_id.clone();
                                    let section_editor = section_editor.clone();
                                    let replacement_editor = replacement_editor.clone();
                                    let original_section = original_section.clone();
                                    cx.listener(move |this, _, window, cx| {
                                        let section = section_editor.read(cx).text(cx);
                                        let replacement = replacement_editor.read(cx).text(cx);
                                        this.save_section_override(
                                            profile_id.clone(),
                                            original_section.clone(),
                                            section,
                                            replacement,
                                            window,
                                            cx,
                                        );
                                    })
                                }),
                        );
                    let cancel_item = div()
                        .id("cancel-item")
                        .track_focus(&mode.cancel_item.focus_handle)
                        .on_action({
                            let profile_id = mode.profile_id.clone();
                            cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                this.configure_prompts(profile_id.clone(), window, cx);
                            })
                        })
                        .child(
                            ListItem::new("cancel-item")
                                .toggle_state(
                                    mode.cancel_item.focus_handle.contains_focused(window, cx),
                                )
                                .inset(true)
                                .spacing(ListItemSpacing::Sparse)
                                .start_slot(
                                    Icon::new(IconName::ArrowLeft)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(Label::new("Go Back"))
                                .on_click({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _, window, cx| {
                                        this.configure_prompts(profile_id.clone(), window, cx);
                                    })
                                }),
                        );
                    let reset_item = div()
                        .id("reset-section-override")
                        .track_focus(&self.focus_handle)
                        .on_action({
                            let default_replacement = default_replacement.clone();
                            let replacement_editor = replacement_editor.clone();
                            cx.listener(move |_, _: &menu::Confirm, window, cx| {
                                let Some(default_replacement) = default_replacement.clone() else {
                                    return;
                                };
                                replacement_editor.update(cx, |editor, cx| {
                                    editor.set_text(default_replacement, window, cx);
                                });
                            })
                        })
                        .child(
                            ListItem::new("reset-section-override")
                                .toggle_state(self.focus_handle.contains_focused(window, cx))
                                .inset(true)
                                .spacing(ListItemSpacing::Sparse)
                                .start_slot(
                                    Icon::new(IconName::Undo)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(Label::new("Reset to Default"))
                                .disabled(default_replacement.is_none())
                                .on_click({
                                    let default_replacement = default_replacement.clone();
                                    let replacement_editor = replacement_editor.clone();
                                    cx.listener(move |_, _, window, cx| {
                                        let Some(default_replacement) = default_replacement.clone()
                                        else {
                                            return;
                                        };
                                        replacement_editor.update(cx, |editor, cx| {
                                            editor.set_text(default_replacement, window, cx);
                                        });
                                    })
                                }),
                        );

                    let title = if mode.original_section.is_some() {
                        "Edit Section Override"
                    } else {
                        "New Section Override"
                    };

                    v_flex()
                        .pb_1()
                        .child(ProfileModalHeader::new(
                            format!("{profile_name} — {title}"),
                            Some(IconName::TextSnippet),
                        ))
                        .child(ListSeparator)
                        .child(
                            v_flex()
                                .gap_2()
                                .px_2()
                                .pt_2()
                                .child(
                                    Label::new("Section Name")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(mode.section_editor.clone())
                                .when(!is_section_valid && !section_query.is_empty(), |this| {
                                    this.child(
                                        Label::new("Section not found in the template graph.")
                                            .size(LabelSize::Small)
                                            .color(Color::Error),
                                    )
                                })
                                .when(
                                    show_suggestions && !section_suggestions.is_empty(),
                                    |this| {
                                        let section_editor = section_editor.clone();
                                        let replacement_editor = replacement_editor.clone();
                                        let section_defaults = self.section_defaults.clone();
                                        let selected_index = mode.suggestion_index;
                                        this.child(v_flex().gap_0p5().children(
                                            section_suggestions.into_iter().enumerate().map(
                                                move |(index, section_name)| {
                                                    let section_editor = section_editor.clone();
                                                    let replacement_editor =
                                                        replacement_editor.clone();
                                                    let section_defaults = section_defaults.clone();
                                                    ListItem::new(format!(
                                                        "section-suggestion-{}",
                                                        section_name
                                                    ))
                                                    .toggle_state(selected_index == Some(index))
                                                    .inset(true)
                                                    .spacing(ListItemSpacing::Sparse)
                                                    .child(Label::new(section_name.clone()))
                                                    .on_click({
                                                        let section_name = section_name.clone();
                                                        cx.listener(move |this, _, window, cx| {
                                                            let Mode::EditSectionOverride(mode) =
                                                                &mut this.mode
                                                            else {
                                                                return;
                                                            };
                                                            mode.suggestion_index = None;
                                                            mode.last_section_name =
                                                                section_name.clone();
                                                            section_editor.update(
                                                                cx,
                                                                |editor, cx| {
                                                                    editor.set_text(
                                                                        section_name.as_str(),
                                                                        window,
                                                                        cx,
                                                                    );
                                                                },
                                                            );
                                                            let default_replacement =
                                                                section_defaults
                                                                    .as_ref()
                                                                    .and_then(|defaults| {
                                                                        defaults.get(&section_name)
                                                                    })
                                                                    .cloned()
                                                                    .unwrap_or_default();
                                                            replacement_editor.update(
                                                                cx,
                                                                |editor, cx| {
                                                                    editor.set_text(
                                                                        default_replacement,
                                                                        window,
                                                                        cx,
                                                                    );
                                                                },
                                                            );
                                                        })
                                                    })
                                                    .into_any_element()
                                                },
                                            ),
                                        ))
                                    },
                                )
                                .child(
                                    Label::new("Replacement Template")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(mode.replacement_editor.clone()),
                        )
                        .child(ListSeparator)
                        .child(reset_item)
                        .child(save_item)
                        .child(cancel_item)
                        .into_any_element()
                }
                Mode::ConfigurePrompts {
                    profile_id,
                    create_section_override,
                } => {
                    let profile_name = settings
                        .profiles
                        .get(profile_id)
                        .map(|profile| profile.name.clone())
                        .unwrap_or_else(|| "Unknown".into());
                    let section_overrides = settings
                        .profiles
                        .get(profile_id)
                        .map(|profile| profile.prompt_section_overrides.clone())
                        .unwrap_or_default();
                    let section_issues =
                        section_override_issues(&section_overrides, self.section_graph.as_ref());

                    v_flex()
                        .pb_1()
                        .child(ProfileModalHeader::new(
                            format!("{profile_name} — Configure Prompts"),
                            Some(IconName::TextSnippet),
                        ))
                        .child(ListSeparator)
                        .child(
                            v_flex()
                                .gap_1()
                                .child(
                                    div().px_2().pt_2().child(
                                        Label::new("Section Overrides")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                                )
                                .child(
                                    div()
                                        .id("create-section-override")
                                        .track_focus(&create_section_override.focus_handle)
                                        .on_action({
                                            let profile_id = profile_id.clone();
                                            cx.listener(
                                                move |this, _: &menu::Confirm, window, cx| {
                                                    this.create_section_override(
                                                        profile_id.clone(),
                                                        window,
                                                        cx,
                                                    );
                                                },
                                            )
                                        })
                                        .child(
                                            ListItem::new("create-section-override")
                                                .toggle_state(
                                                    create_section_override
                                                        .focus_handle
                                                        .contains_focused(window, cx),
                                                )
                                                .inset(true)
                                                .spacing(ListItemSpacing::Sparse)
                                                .start_slot(
                                                    Icon::new(IconName::Plus)
                                                        .size(IconSize::Small)
                                                        .color(Color::Muted),
                                                )
                                                .child(Label::new("Create Section Override"))
                                                .on_click({
                                                    let profile_id = profile_id.clone();
                                                    cx.listener(move |this, _, window, cx| {
                                                        this.create_section_override(
                                                            profile_id.clone(),
                                                            window,
                                                            cx,
                                                        );
                                                    })
                                                }),
                                        ),
                                )
                                .child(
                                    div().px_2().pt_1().child(
                                        Label::new("Overrides")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                                )
                                .child(if section_overrides.is_empty() {
                                    div()
                                        .px_2()
                                        .child(
                                            Label::new("No overrides yet.")
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .into_any_element()
                                } else {
                                    v_flex()
                                        .gap_0p5()
                                        .children(
                                            section_overrides
                                                .iter()
                                                .map(|override_entry| {
                                                    let issue = section_issues
                                                        .get(override_entry.section.as_ref());
                                                    let override_entry = override_entry.clone();
                                                    let issue_icon = issue.map(|issue| {
                                                        let (icon, color) = match issue.kind {
                                                            SectionOverrideIssueKind::Missing => {
                                                                (IconName::XCircle, Color::Error)
                                                            }
                                                            SectionOverrideIssueKind::Masked => {
                                                                (IconName::Warning, Color::Warning)
                                                            }
                                                        };
                                                        div()
                                                            .id(format!(
                                                                "section-override-issue-{}",
                                                                override_entry.section
                                                            ))
                                                            .tooltip(Tooltip::text(
                                                                issue.message.clone(),
                                                            ))
                                                            .child(
                                                                Icon::new(icon)
                                                                    .size(IconSize::Small)
                                                                    .color(color),
                                                            )
                                                    });
                                                    let delete_button = IconButton::new(
                                                        format!(
                                                            "delete-section-override-{}",
                                                            override_entry.section
                                                        ),
                                                        IconName::Trash,
                                                    )
                                                    .shape(IconButtonShape::Square)
                                                    .icon_size(IconSize::XSmall)
                                                    .icon_color(Color::Muted)
                                                    .on_click({
                                                        let profile_id = profile_id.clone();
                                                        let section =
                                                            override_entry.section.clone();
                                                        cx.listener(move |this, _, window, cx| {
                                                            this.delete_section_override(
                                                                profile_id.clone(),
                                                                section.clone(),
                                                                window,
                                                                cx,
                                                            );
                                                            cx.stop_propagation();
                                                        })
                                                    });
                                                    ListItem::new(format!(
                                                        "section-override-{}",
                                                        override_entry.section
                                                    ))
                                                    .inset(true)
                                                    .spacing(ListItemSpacing::Sparse)
                                                    .start_slot(
                                                        Icon::new(IconName::TextSnippet)
                                                            .size(IconSize::Small)
                                                            .color(Color::Muted),
                                                    )
                                                    .child(Label::new(
                                                        override_entry.section.to_string(),
                                                    ))
                                                    .end_slot(
                                                        h_flex()
                                                            .gap_1()
                                                            .child(delete_button)
                                                            .when_some(issue_icon, |this, icon| {
                                                                this.child(icon)
                                                            }),
                                                    )
                                                    .on_click({
                                                        let profile_id = profile_id.clone();
                                                        let override_entry = override_entry.clone();
                                                        cx.listener(move |this, _, window, cx| {
                                                            this.edit_section_override(
                                                                profile_id.clone(),
                                                                Some(override_entry.clone()),
                                                                window,
                                                                cx,
                                                            );
                                                        })
                                                    })
                                                    .into_any_element()
                                                })
                                                .collect::<Vec<_>>(),
                                        )
                                        .into_any_element()
                                }),
                        )
                        .child(ListSeparator)
                        .child(go_back_item)
                        .into_any_element()
                }
            })
    }
}

#[derive(Clone)]
struct SectionOverrideIssue {
    kind: SectionOverrideIssueKind,
    message: String,
}

#[derive(Clone, Copy)]
enum SectionOverrideIssueKind {
    Missing,
    Masked,
}

fn section_exists(section_graph: &SectionGraph, section: &str) -> bool {
    if section.is_empty() {
        return false;
    }
    section_graph
        .templates
        .values()
        .any(|template| template.sections.contains_key(section))
}

fn section_suggestions(section_graph: &SectionGraph, query: &str) -> Vec<String> {
    let query = query.trim();
    let all_sections = all_sections(section_graph);
    if query.is_empty() {
        return all_sections.into_iter().take(6).collect();
    }
    let query = query.to_lowercase();
    all_sections
        .into_iter()
        .filter(|section| section.to_lowercase().contains(&query))
        .take(6)
        .collect()
}

fn all_sections(section_graph: &SectionGraph) -> BTreeSet<String> {
    let mut sections = BTreeSet::new();
    for template in section_graph.templates.values() {
        sections.extend(template.sections.keys().cloned());
    }
    sections
}

fn section_override_issues(
    overrides: &[PromptSectionOverride],
    section_graph: Option<&SectionGraph>,
) -> HashMap<String, SectionOverrideIssue> {
    let Some(section_graph) = section_graph else {
        return HashMap::default();
    };
    let all_sections = all_sections(section_graph);
    let mut issues = HashMap::default();
    for override_entry in overrides {
        let section = override_entry.section.as_ref();
        if !all_sections.contains(section) {
            issues.insert(
                override_entry.section.to_string(),
                SectionOverrideIssue {
                    kind: SectionOverrideIssueKind::Missing,
                    message: "Section no longer exists in templates.".to_string(),
                },
            );
        }
    }

    let masked_by = masked_sections_by_parent_override(overrides, section_graph);
    for (section, parents) in masked_by {
        if issues.contains_key(&section) {
            continue;
        }
        if parents.is_empty() {
            continue;
        }
        let mut parent_list: Vec<String> = parents.into_iter().collect();
        parent_list.sort();
        let message = if parent_list.len() == 1 {
            format!(
                "Hidden because {} was overridden without including it.",
                parent_list[0]
            )
        } else {
            format!(
                "Hidden because {} were overridden without including it.",
                parent_list.join(", ")
            )
        };
        issues.insert(
            section,
            SectionOverrideIssue {
                kind: SectionOverrideIssueKind::Masked,
                message,
            },
        );
    }

    issues
}

fn masked_sections_by_parent_override(
    overrides: &[PromptSectionOverride],
    section_graph: &SectionGraph,
) -> HashMap<String, BTreeSet<String>> {
    let mut masked_by = HashMap::default();
    let mut references_by_section: HashMap<&str, BTreeSet<String>> = HashMap::default();
    for override_entry in overrides {
        references_by_section.insert(
            override_entry.section.as_ref(),
            section_references(override_entry.replacement.as_ref()),
        );
    }

    for template in section_graph.templates.values() {
        for override_entry in overrides {
            let parent = override_entry.section.as_ref();
            if !template.sections.contains_key(parent) {
                continue;
            }
            let descendants = collect_descendants(template, parent);
            if descendants.is_empty() {
                continue;
            }
            let referenced = references_by_section
                .get(parent)
                .cloned()
                .unwrap_or_default();
            let mut reachable = BTreeSet::new();
            for referenced_section in referenced {
                if is_descendant_or_self(template, parent, &referenced_section) {
                    reachable.extend(collect_descendants_including(template, &referenced_section));
                }
            }
            for masked in descendants.difference(&reachable) {
                masked_by
                    .entry(masked.clone())
                    .or_insert_with(BTreeSet::new)
                    .insert(parent.to_string());
            }
        }
    }

    masked_by
}

fn collect_descendants(template: &TemplateSectionGraph, root: &str) -> BTreeSet<String> {
    let mut descendants = BTreeSet::new();
    let mut stack = vec![root.to_string()];
    while let Some(section) = stack.pop() {
        if let Some(node) = template.sections.get(&section) {
            for child in &node.children {
                if descendants.insert(child.clone()) {
                    stack.push(child.clone());
                }
            }
        }
    }
    descendants
}

fn collect_descendants_including(template: &TemplateSectionGraph, root: &str) -> BTreeSet<String> {
    let mut descendants = collect_descendants(template, root);
    descendants.insert(root.to_string());
    descendants
}

fn is_descendant_or_self(template: &TemplateSectionGraph, ancestor: &str, candidate: &str) -> bool {
    if ancestor == candidate {
        return true;
    }
    let mut stack = vec![candidate.to_string()];
    let mut visited = BTreeSet::new();
    while let Some(section) = stack.pop() {
        if !visited.insert(section.clone()) {
            continue;
        }
        let Some(node) = template.sections.get(&section) else {
            continue;
        };
        for parent in &node.parents {
            if parent == ancestor {
                return true;
            }
            stack.push(parent.clone());
        }
    }
    false
}

fn section_references(text: &str) -> BTreeSet<String> {
    let mut references = BTreeSet::new();
    let mut index = 0;
    while let Some(match_index) = text[index..].find("{{#section") {
        let mut cursor = index + match_index + "{{#section".len();
        let bytes = text.as_bytes();
        while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor >= bytes.len() {
            break;
        }
        let (section_name, next_index) = if bytes[cursor] == b'"' || bytes[cursor] == b'\'' {
            let quote = bytes[cursor];
            cursor += 1;
            let start = cursor;
            while cursor < bytes.len() && bytes[cursor] != quote {
                cursor += 1;
            }
            let section_name = text.get(start..cursor).unwrap_or_default();
            (section_name, cursor.saturating_add(1).min(bytes.len()))
        } else {
            let start = cursor;
            while cursor < bytes.len()
                && !bytes[cursor].is_ascii_whitespace()
                && bytes[cursor] != b'}'
            {
                cursor += 1;
            }
            let section_name = text.get(start..cursor).unwrap_or_default();
            (section_name, cursor.min(bytes.len()))
        };
        if !section_name.is_empty() {
            references.insert(section_name.to_string());
        }
        index = next_index;
    }
    references
}
