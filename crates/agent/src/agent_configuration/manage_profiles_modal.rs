mod profile_modal_header;

use std::sync::Arc;

use assistant_settings::{AgentProfile, AgentProfileId, AssistantSettings, builtin_profiles};
use assistant_tool::ToolWorkingSet;
use convert_case::{Case, Casing as _};
use editor::Editor;
use fs::Fs;
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Subscription, WeakEntity,
    prelude::*,
};
use settings::{Settings as _, update_settings_file};
use ui::{
    KeyBinding, ListItem, ListItemSpacing, ListSeparator, Navigable, NavigableEntry, prelude::*,
};
use util::ResultExt as _;
use workspace::{ModalView, Workspace};

use crate::agent_configuration::manage_profiles_modal::profile_modal_header::ProfileModalHeader;
use crate::agent_configuration::tool_picker::{ToolPicker, ToolPickerDelegate};
use crate::{AssistantPanel, ManageProfiles, ThreadStore};

use super::tool_picker::ToolPickerMode;

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
}

impl Mode {
    pub fn choose_profile(_window: &mut Window, cx: &mut Context<ManageProfilesModal>) -> Self {
        let settings = AssistantSettings::get_global(cx);

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
    configure_tools: NavigableEntry,
    configure_mcps: NavigableEntry,
    cancel_item: NavigableEntry,
}

#[derive(Clone)]
pub struct NewProfileMode {
    name_editor: Entity<Editor>,
    base_profile_id: Option<AgentProfileId>,
}

pub struct ManageProfilesModal {
    fs: Arc<dyn Fs>,
    tools: Entity<ToolWorkingSet>,
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
        workspace.register_action(|workspace, action: &ManageProfiles, window, cx| {
            if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                let fs = workspace.app_state().fs.clone();
                let thread_store = panel.read(cx).thread_store();
                let tools = thread_store.read(cx).tools();
                let thread_store = thread_store.downgrade();
                workspace.toggle_modal(window, cx, |window, cx| {
                    let mut this = Self::new(fs, tools, thread_store, window, cx);

                    if let Some(profile_id) = action.customize_tools.clone() {
                        this.configure_tools(profile_id, window, cx);
                    }

                    this
                })
            }
        });
    }

    pub fn new(
        fs: Arc<dyn Fs>,
        tools: Entity<ToolWorkingSet>,
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
        base_profile_id: Option<AgentProfileId>,
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
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = Mode::ViewProfile(ViewProfileMode {
            profile_id,
            fork_profile: NavigableEntry::focusable(cx),
            configure_tools: NavigableEntry::focusable(cx),
            configure_mcps: NavigableEntry::focusable(cx),
            cancel_item: NavigableEntry::focusable(cx),
        });
        self.focus_handle(cx).focus(window);
    }

    fn configure_mcps(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        let Some(profile) = settings.profiles.get(&profile_id).cloned() else {
            return;
        };

        let tool_picker = cx.new(|cx| {
            let delegate = ToolPickerDelegate::new(
                ToolPickerMode::McpTools,
                self.fs.clone(),
                self.tools.clone(),
                self.thread_store.clone(),
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
        self.focus_handle(cx).focus(window);
    }

    fn configure_tools(
        &mut self,
        profile_id: AgentProfileId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        let Some(profile) = settings.profiles.get(&profile_id).cloned() else {
            return;
        };

        let tool_picker = cx.new(|cx| {
            let delegate = ToolPickerDelegate::new(
                ToolPickerMode::BuiltinTools,
                self.fs.clone(),
                self.tools.clone(),
                self.thread_store.clone(),
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
                let profile_id = AgentProfileId(name.to_case(Case::Kebab).into());

                let profile = AgentProfile {
                    name: name.into(),
                    tools: base_profile
                        .as_ref()
                        .map(|profile| profile.tools.clone())
                        .unwrap_or_default(),
                    enable_all_context_servers: base_profile
                        .as_ref()
                        .map(|profile| profile.enable_all_context_servers)
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
            Mode::ConfigureMcps { .. } => {}
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
            Mode::ConfigureTools { profile_id, .. } => {
                self.view_profile(profile_id.clone(), window, cx)
            }
            Mode::ConfigureMcps { profile_id, .. } => {
                self.view_profile(profile_id.clone(), window, cx)
            }
        }
    }

    fn create_profile(
        &self,
        profile_id: AgentProfileId,
        profile: AgentProfile,
        cx: &mut Context<Self>,
    ) {
        update_settings_file::<AssistantSettings>(self.fs.clone(), cx, {
            move |settings, _cx| {
                settings.create_profile(profile_id, profile).log_err();
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
            Mode::ConfigureMcps { tool_picker, .. } => tool_picker.focus_handle(cx),
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
                ListItem::new(SharedString::from(format!("profile-{}", profile.id)))
                    .toggle_state(profile.navigation.focus_handle.contains_focused(window, cx))
                    .inset(true)
                    .spacing(ListItemSpacing::Sparse)
                    .child(Label::new(profile.name.clone()))
                    .end_slot(
                        h_flex()
                            .gap_1()
                            .child(
                                Label::new("Customize")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
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
            .child(h_flex().p_2().child(mode.name_editor.clone()))
    }

    fn render_view_profile(
        &mut self,
        mode: ViewProfileMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let settings = AssistantSettings::get_global(cx);

        let profile_id = &settings.default_profile;
        let profile_name = settings
            .profiles
            .get(&mode.profile_id)
            .map(|profile| profile.name.clone())
            .unwrap_or_else(|| "Unknown".into());

        let icon = match profile_id.as_str() {
            "write" => IconName::Pencil,
            "ask" => IconName::MessageBubbles,
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
                                        .start_slot(
                                            Icon::new(IconName::Settings)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
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
                        )
                        .child(
                            div()
                                .id("configure-mcps")
                                .track_focus(&mode.configure_mcps.focus_handle)
                                .on_action({
                                    let profile_id = mode.profile_id.clone();
                                    cx.listener(move |this, _: &menu::Confirm, window, cx| {
                                        this.configure_mcps(profile_id.clone(), window, cx);
                                    })
                                })
                                .child(
                                    ListItem::new("configure-mcps")
                                        .toggle_state(
                                            mode.configure_mcps
                                                .focus_handle
                                                .contains_focused(window, cx),
                                        )
                                        .inset(true)
                                        .spacing(ListItemSpacing::Sparse)
                                        .start_slot(
                                            Icon::new(IconName::Hammer)
                                                .size(IconSize::Small)
                                                .color(Color::Muted),
                                        )
                                        .child(Label::new("Configure MCP Servers"))
                                        .on_click({
                                            let profile_id = mode.profile_id.clone();
                                            cx.listener(move |this, _, window, cx| {
                                                this.configure_mcps(profile_id.clone(), window, cx);
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
                                            div().children(
                                                KeyBinding::for_action_in(
                                                    &menu::Cancel,
                                                    &self.focus_handle,
                                                    window,
                                                    cx,
                                                )
                                                .map(|kb| kb.size(rems_from_px(12.))),
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
        .entry(mode.configure_tools)
        .entry(mode.configure_mcps)
        .entry(mode.cancel_item)
    }
}

impl Render for ManageProfilesModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = AssistantSettings::get_global(cx);

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
                        div().children(
                            KeyBinding::for_action_in(
                                &menu::Cancel,
                                &self.focus_handle,
                                window,
                                cx,
                            )
                            .map(|kb| kb.size(rems_from_px(12.))),
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

                    v_flex()
                        .pb_1()
                        .child(ProfileModalHeader::new(
                            format!("{profile_name} — Configure Tools"),
                            Some(IconName::Cog),
                        ))
                        .child(ListSeparator)
                        .child(tool_picker.clone())
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
                            format!("{profile_name} — Configure MCP Servers"),
                            Some(IconName::Hammer),
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
