use std::sync::Arc;

use assistant_settings::AssistantSettings;
use assistant_tool::ToolWorkingSet;
use gpui::{prelude::*, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, WeakEntity};
use settings::Settings as _;
use ui::{prelude::*, ListItem, ListItemSpacing, Navigable, NavigableEntry};
use workspace::{ModalView, Workspace};

use crate::assistant_configuration::profile_picker::{ProfilePicker, ProfilePickerDelegate};
use crate::{AssistantPanel, ManageProfiles};

enum Mode {
    ChooseProfile(Entity<ProfilePicker>),
    ViewProfile(ViewProfileMode),
    ConfigureTools(ConfigureToolsMode),
}

#[derive(Clone)]
pub struct ViewProfileMode {
    profile_id: Arc<str>,
    configure_tools: NavigableEntry,
}

#[derive(Clone)]
pub struct ConfigureToolsMode {
    #[allow(dead_code)]
    profile_id: Arc<str>,
}

pub struct ManageProfilesModal {
    #[allow(dead_code)]
    workspace: WeakEntity<Workspace>,
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
                let workspace_handle = cx.entity().downgrade();
                let thread_store = panel.read(cx).thread_store().read(cx);
                let tools = thread_store.tools();
                workspace.toggle_modal(window, cx, |window, cx| {
                    Self::new(workspace_handle, tools, window, cx)
                })
            }
        });
    }

    pub fn new(
        workspace: WeakEntity<Workspace>,
        tools: Arc<ToolWorkingSet>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let handle = cx.entity();

        Self {
            workspace,
            tools,
            focus_handle,
            mode: Mode::ChooseProfile(cx.new(|cx| {
                let delegate = ProfilePickerDelegate::new(
                    move |profile_id, window, cx| {
                        handle.update(cx, |this, cx| {
                            this.view_profile(profile_id.clone(), window, cx);
                        })
                    },
                    cx,
                );
                ProfilePicker::new(delegate, window, cx)
            })),
        }
    }

    pub fn view_profile(
        &mut self,
        profile_id: Arc<str>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = Mode::ViewProfile(ViewProfileMode {
            profile_id,
            configure_tools: NavigableEntry::focusable(cx),
        });
        self.focus_handle(cx).focus(window);
    }

    fn configure_tools(
        &mut self,
        profile_id: Arc<str>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.mode = Mode::ConfigureTools(ConfigureToolsMode { profile_id });
    }

    fn confirm(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}

    fn cancel(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {}
}

impl ModalView for ManageProfilesModal {}

impl Focusable for ManageProfilesModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        match &self.mode {
            Mode::ChooseProfile(profile_picker) => profile_picker.read(cx).focus_handle(cx),
            Mode::ViewProfile(_) | Mode::ConfigureTools(_) => self.focus_handle.clone(),
        }
    }
}

impl EventEmitter<DismissEvent> for ManageProfilesModal {}

impl ManageProfilesModal {
    fn render_view_profile(
        &mut self,
        mode: ViewProfileMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Navigable::new(
            div()
                .track_focus(&self.focus_handle(cx))
                .size_full()
                .child(
                    v_flex().child(
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
                                            this.configure_tools(profile_id.clone(), window, cx);
                                        })
                                    }),
                            ),
                    ),
                )
                .into_any_element(),
        )
        .entry(mode.configure_tools)
    }

    fn render_configure_tools(
        &mut self,
        mode: ConfigureToolsMode,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let settings = AssistantSettings::get_global(cx);

        let Some(profile) = settings.profiles.get(&mode.profile_id) else {
            return div().into_any_element();
        };

        v_flex()
            .children(profile.tools.iter().filter_map(|(tool_id, enabled)| {
                let tool = self.tools.tool(tool_id, cx)?;

                Some(
                    ListItem::new(SharedString::from(tool.name()))
                        .inset(true)
                        .child(Label::new(tool.name()))
                        .end_slot::<Icon>(enabled.then(|| {
                            Icon::new(IconName::Check)
                                .size(IconSize::Small)
                                .color(Color::Success)
                        })),
                )
            }))
            .children(
                profile
                    .context_servers
                    .iter()
                    .filter_map(|(context_server_id, preset)| {
                        Some(v_flex().child(Label::new(context_server_id)).children(
                            preset.tools.iter().filter_map(|(tool_id, enabled)| {
                                let tool = self.tools.tool(tool_id, cx)?;

                                Some(
                                    ListItem::new(SharedString::from(tool.name()))
                                        .inset(true)
                                        .child(Label::new(tool.name()))
                                        .end_slot::<Icon>(enabled.then(|| {
                                            Icon::new(IconName::Check)
                                                .size(IconSize::Small)
                                                .color(Color::Success)
                                        })),
                                )
                            }),
                        ))
                    }),
            )
            .into_any_element()
    }
}

impl Render for ManageProfilesModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                Mode::ChooseProfile(profile_picker) => profile_picker.clone().into_any_element(),
                Mode::ViewProfile(mode) => self
                    .render_view_profile(mode.clone(), window, cx)
                    .into_any_element(),
                Mode::ConfigureTools(mode) => self
                    .render_configure_tools(mode.clone(), window, cx)
                    .into_any_element(),
            })
    }
}
