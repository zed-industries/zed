mod profile_modal_header;

use std::sync::Arc;

use agent::ContextServerRegistry;
use agent_settings::{AgentProfile, AgentProfileId, AgentSettings, builtin_profiles};
use editor::Editor;
use fs::Fs;
use gpui::{DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription, prelude::*};
use language_model::{LanguageModel, LanguageModelRegistry};
use settings::SettingsStore;
use settings::{
    LanguageModelProviderSetting, LanguageModelSelection, Settings as _, update_settings_file,
};
use ui::{
    KeyBinding, ListItem, ListItemSpacing, ListSeparator, Navigable, NavigableEntry, prelude::*,
};
use workspace::{ModalView, Workspace};

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
    delete_profile: NavigableEntry,
    cancel_item: NavigableEntry,
}

#[derive(Clone)]
pub struct NewProfileMode {
    name_editor: Entity<Editor>,
    base_profile_id: Option<AgentProfileId>,
}

pub struct ManageProfilesModal {
    fs: Arc<dyn Fs>,
    context_server_registry: Entity<ContextServerRegistry>,
    active_model: Option<Arc<dyn LanguageModel>>,
    focus_handle: FocusHandle,
    mode: Mode,
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

        // Keep this modal in sync with settings changes (including profile deletion).
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, |this, window, cx| {
                if matches!(this.mode, Mode::ChooseProfile(_)) {
                    this.mode = Mode::choose_profile(window, cx);
                    this.focus_handle(cx).focus(window, cx);
                    cx.notify();
                }
            });

        Self {
            fs,
            active_model,
            context_server_registry,
            focus_handle,
            mode: Mode::choose_profile(window, cx),
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
        }
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
        .entry(mode.delete_profile)
        .entry(mode.cancel_item)
    }
}

impl Render for ManageProfilesModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| this.cancel(window, cx)))
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
            })
    }
}
