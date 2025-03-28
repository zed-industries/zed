mod profile_modal_header;

use std::sync::Arc;

use assistant_settings::{
    AgentProfile, AgentProfileContent, AssistantSettings, AssistantSettingsContent,
    ContextServerPresetContent, VersionedAssistantSettingsContent,
};
use assistant_tool::ToolWorkingSet;
use convert_case::{Case, Casing as _};
use editor::Editor;
use fs::Fs;
use gpui::{
    prelude::*, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription,
    WeakEntity,
};
use settings::{update_settings_file, Settings as _};
use ui::{
    prelude::*, KeyBinding, ListItem, ListItemSpacing, ListSeparator, Navigable, NavigableEntry,
};
use workspace::{ModalView, Workspace};

use crate::assistant_configuration::manage_profiles_modal::profile_modal_header::ProfileModalHeader;
use crate::assistant_configuration::tool_picker::{ToolPicker, ToolPickerDelegate};
use crate::{AssistantPanel, ManageProfiles, ThreadStore};

enum Mode {
    ChooseProfile(ChooseProfileMode),
    NewProfile(NewProfileMode),
    ViewProfile(ViewProfileMode),
    ConfigureTools {
        profile_id: Arc<str>,
        tool_picker: Entity<ToolPicker>,
        _subscription: Subscription,
    },
}

impl Mode {
    pub fn choose_profile(_window: &mut Window, cx: &mut Context<ManageProfilesModal>) -> Self {
        let settings = AssistantSettings::get_global(cx);

        let mut profiles = settings.profiles.clone();
        profiles.sort_unstable_by(|_, a, _, b| a.name.cmp(&b.name));

        let profiles = profiles
            .into_iter()
            .map(|(id, profile)| ProfileEntry {
                id,
                name: profile.name,
                navigation: NavigableEntry::focusable(cx),
            })
            .collect::<Vec<_>>();

        Self::ChooseProfile(ChooseProfileMode {
            profiles,
            add_new_profile: NavigableEntry::focusable(cx),
        })
    }
}

#[derive(Clone)]
struct ProfileEntry {
    pub id: Arc<str>,
    pub name: SharedString,
    pub navigation: NavigableEntry,
}

#[derive(Clone)]
pub struct ChooseProfileMode {
    profiles: Vec<ProfileEntry>,
    add_new_profile: NavigableEntry,
}

#[derive(Clone)]
pub struct ViewProfileMode {
    profile_id: Arc<str>,
    fork_profile: NavigableEntry,
    configure_tools: NavigableEntry,
}

#[derive(Clone)]
pub struct NewProfileMode {
    name_editor: Entity<Editor>,
    base_profile_id: Option<Arc<str>>,
}

pub struct ManageProfilesModal {
    fs: Arc<dyn Fs>,
    tools: Arc<ToolWorkingSet>,
    thread_store: WeakEntity<ThreadStore>,
    focus_handle: FocusHandle,
    mode: Mode,
}

impl ManageProfilesModal {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &ManageProfiles, window, cx| {
            if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                let fs = workspace.app_state().fs.clone();
                let thread_store = panel.read(cx).thread_store();
                let tools = thread_store.read(cx).tools();
                let thread_store = thread_store.downgrade();
                workspace.toggle_modal(window, cx, |window, cx| {
                    Self::new(fs, tools, thread_store, window, cx)
                })
            }
        });
    }

    pub fn new(
        fs: Arc<dyn Fs>,
        tools: Arc<ToolWorkingSet>,
        thread_store: WeakEntity<ThreadStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            fs,
            tools,
            thread_store,
            focus_handle,
            mode: Mode::choose_profile(window, cx),
        }
    }

    fn choose_profile(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.mode = Mode::choose_profile(window, cx);
        self.focus_handle(cx).focus(window);
    }

    fn new_profile(
        &mut self,
        base_profile_id: Option<Arc<str>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name_editor = cx.new(|cx| Editor::single_line(window, cx));
        name_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text("Profile name", cx);
        });

        self.mode = Mode::NewProfile(NewProfileMode {
            name_editor,
            base_profile_id,
        });
        self.focus_handle(cx).focus(window);
    }

    pub fn view_profile(
        &mut self,
        profile_id: Arc<str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = Mode::ViewProfile(ViewProfileMode {
            profile_id,
            fork_profile: NavigableEntry::focusable(cx),
            configure_tools: NavigableEntry::focusable(cx),
        });
        self.focus_handle(cx).focus(window);
    }

    fn configure_tools(
        &mut self,
        profile_id: Arc<str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        let Some(profile) = settings.profiles.get(&profile_id).cloned() else {
            return;
        };

        let tool_picker = cx.new(|cx| {
            let delegate = ToolPickerDelegate::new(
                self.fs.clone(),
                self.tools.clone(),
                self.thread_store.clone(),
                profile_id.clone(),
                profile,
                cx,
            );
            ToolPicker::new(delegate, window, cx)
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
        self.focus_handle(cx).focus(window);
    }

    fn confirm(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match &self.mode {
            Mode::ChooseProfile { .. } => {}
            Mode::NewProfile(mode) => {
                let settings = AssistantSettings::get_global(cx);

                let base_profile = mode
                    .base_profile_id
                    .as_ref()
                    .and_then(|profile_id| settings.profiles.get(profile_id).cloned());

                let name = mode.name_editor.read(cx).text(cx);
                let profile_id: Arc<str> = name.to_case(Case::Kebab).into();

                let profile = AgentProfile {
                    name: name.into(),
                    tools: base_profile
                        .as_ref()
                        .map(|profile| profile.tools.clone())
                        .unwrap_or_default(),
                    context_servers: base_profile
                        .map(|profile| profile.context_servers)
                        .unwrap_or_default(),
                };

                self.create_profile(profile_id.clone(), profile, cx);
                self.view_profile(profile_id, window, cx);
            }
            Mode::ViewProfile(_) => {}
            Mode::ConfigureTools { .. } => {}
        }
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
            Mode::ConfigureTools { .. } => {}
        }
    }

    fn create_profile(&self, profile_id: Arc<str>, profile: AgentProfile, cx: &mut Context<Self>) {
        update_settings_file::<AssistantSettings>(self.fs.clone(), cx, {
            move |settings, _cx| match settings {
                AssistantSettingsContent::Versioned(VersionedAssistantSettingsContent::V2(
                    settings,
                )) => {
                    let profiles = settings.profiles.get_or_insert_default();
                    if profiles.contains_key(&profile_id) {
                        log::error!("profile with ID '{profile_id}' already exists");
                        return;
                    }

                    profiles.insert(
                        profile_id,
                        AgentProfileContent {
                            name: profile.name.into(),
                            tools: profile.tools,
                            context_servers: profile
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
                        },
                    );
                }
                _ => {}
            }
        });
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
        }
    }
}

impl EventEmitter<DismissEvent> for ManageProfilesModal {}

impl ManageProfilesModal {
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
                .child(ProfileModalHeader::new(
                    "Agent Profiles",
                    IconName::ZedAssistant,
                ))
                .child(
                    v_flex()
                        .pb_1()
                        .child(ListSeparator)
                        .children(mode.profiles.iter().map(|profile| {
                            div()
                                .id(SharedString::from(format!("profile-{}", profile.id)))
                                .track_focus(&profile.navigation.focus_handle)
                                .on_action({
                                    let profile_id = profile.id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.view_profile(profile_id.clone(), window, cx);
                                    })
                                })
                                .child(
                                    ListItem::new(SharedString::from(format!(
                                        "profile-{}",
                                        profile.id
                                    )))
                                    .toggle_state(
                                        profile
                                            .navigation
                                            .focus_handle
                                            .contains_focused(window, cx),
                                    )
                                    .inset(true)
                                    .spacing(ListItemSpacing::Sparse)
                                    .child(Label::new(profile.name.clone()))
                                    .end_slot(
                                        h_flex()
                                            .gap_1()
                                            .child(Label::new("Customize").size(LabelSize::Small))
                                            .children(KeyBinding::for_action_in(
                                                &menu::Confirm,
                                                &self.focus_handle,
                                                window,
                                                cx,
                                            )),
                                    )
                                    .on_click({
                                        let profile_id = profile.id.clone();
                                        cx.listener(move |this, _, window, cx| {
                                            this.new_profile(Some(profile_id.clone()), window, cx);
                                        })
                                    }),
                                )
                        }))
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
            for profile in mode.profiles {
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
        let settings = AssistantSettings::get_global(cx);

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
                match base_profile_name {
                    Some(base_profile) => format!("Fork {base_profile}"),
                    None => "New Profile".into(),
                },
                IconName::Plus,
            ))
            .child(ListSeparator)
            .child(h_flex().p_2().child(mode.name_editor.clone()))
    }

    fn render_view_profile(
        &mut self,
        mode: ViewProfileMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let settings = AssistantSettings::get_global(cx);

        let profile_name = settings
            .profiles
            .get(&mode.profile_id)
            .map(|profile| profile.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        Navigable::new(
            div()
                .track_focus(&self.focus_handle(cx))
                .size_full()
                .child(ProfileModalHeader::new(
                    profile_name,
                    IconName::ZedAssistant,
                ))
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
                                        .start_slot(Icon::new(IconName::GitBranch))
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
                                .id("configure-tools")
                                .track_focus(&mode.configure_tools.focus_handle)
                                .on_action({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.configure_tools(profile_id.clone(), window, cx);
                                    })
                                })
                                .child(
                                    ListItem::new("configure-tools")
                                        .toggle_state(
                                            mode.configure_tools
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(Icon::new(IconName::Cog))
                                        .child(Label::new("Configure Tools"))
                                        .on_click({
                                            let profile_id = mode.profile_id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.configure_tools(
                                                    profile_id.clone(),
                                                    window,
                                                    cx,
                                                );
                                            })
                                        }),
                                ),
                        ),
                )
                .into_any_element(),
        )
        .entry(mode.fork_profile)
        .entry(mode.configure_tools)
    }
}

impl Render for ManageProfilesModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AssistantSettings::get_global(cx);

        div()
            .elevation_3(cx)
            .w(rems(34.))
            .key_context("ManageProfilesModal")
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| this.cancel(window, cx)))
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| this.confirm(window, cx)))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle(cx).focus(window);
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

                    div()
                        .child(ProfileModalHeader::new(
                            format!("{profile_name}: Configure Tools"),
                            IconName::Cog,
                        ))
                        .child(ListSeparator)
                        .child(tool_picker.clone())
                        .into_any_element()
                }
            })
    }
}
