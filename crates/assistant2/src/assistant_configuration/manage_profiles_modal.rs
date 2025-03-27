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
use gpui::{prelude::*, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription};
use settings::{update_settings_file, Settings as _};
use ui::{prelude::*, ListItem, ListItemSpacing, ListSeparator, Navigable, NavigableEntry};
use workspace::{ModalView, Workspace};

use crate::assistant_configuration::manage_profiles_modal::profile_modal_header::ProfileModalHeader;
use crate::assistant_configuration::profile_picker::{ProfilePicker, ProfilePickerDelegate};
use crate::assistant_configuration::tool_picker::{ToolPicker, ToolPickerDelegate};
use crate::{AssistantPanel, ManageProfiles};

enum Mode {
    ChooseProfile {
        profile_picker: Entity<ProfilePicker>,
        _subscription: Subscription,
    },
    NewProfile(NewProfileMode),
    ViewProfile(ViewProfileMode),
    ConfigureTools {
        profile_id: Arc<str>,
        tool_picker: Entity<ToolPicker>,
        _subscription: Subscription,
    },
}

impl Mode {
    pub fn choose_profile(window: &mut Window, cx: &mut Context<ManageProfilesModal>) -> Self {
        let this = cx.entity();

        let profile_picker = cx.new(|cx| {
            let delegate = ProfilePickerDelegate::new(
                move |profile_id, window, cx| {
                    this.update(cx, |this, cx| {
                        this.view_profile(profile_id.clone(), window, cx);
                    })
                },
                cx,
            );
            ProfilePicker::new(delegate, window, cx)
        });
        let dismiss_subscription = cx.subscribe_in(
            &profile_picker,
            window,
            |_this, _profile_picker, _: &DismissEvent, _window, cx| {
                cx.emit(DismissEvent);
            },
        );

        Self::ChooseProfile {
            profile_picker,
            _subscription: dismiss_subscription,
        }
    }
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
                let thread_store = panel.read(cx).thread_store().read(cx);
                let tools = thread_store.tools();
                workspace.toggle_modal(window, cx, |window, cx| Self::new(fs, tools, window, cx))
            }
        });
    }

    pub fn new(
        fs: Arc<dyn Fs>,
        tools: Arc<ToolWorkingSet>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        Self {
            fs,
            tools,
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
            Mode::ChooseProfile { .. } => {}
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
            Mode::ChooseProfile { profile_picker, .. } => profile_picker.focus_handle(cx),
            Mode::NewProfile(mode) => mode.name_editor.focus_handle(cx),
            Mode::ViewProfile(_) => self.focus_handle.clone(),
            Mode::ConfigureTools { tool_picker, .. } => tool_picker.focus_handle(cx),
        }
    }
}

impl EventEmitter<DismissEvent> for ManageProfilesModal {}

impl ManageProfilesModal {
    fn render_new_profile(
        &mut self,
        mode: NewProfileMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .id("new-profile")
            .track_focus(&self.focus_handle(cx))
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
                Mode::ChooseProfile { profile_picker, .. } => div()
                    .child(ProfileModalHeader::new("Profiles", IconName::ZedAssistant))
                    .child(ListSeparator)
                    .child(profile_picker.clone())
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
