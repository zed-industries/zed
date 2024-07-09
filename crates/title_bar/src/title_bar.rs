mod call_controls;
mod collab;
mod platforms;
mod window_controls;

use crate::platforms::{platform_linux, platform_mac, platform_windows};
use auto_update::AutoUpdateStatus;
use call::{ActiveCall, ParticipantLocation};
use client::{Client, UserStore};
use collab::render_color_ribbon;
use gpui::{
    actions, div, px, Action, AnyElement, AppContext, Decorations, Element, InteractiveElement,
    Interactivity, IntoElement, Model, MouseButton, ParentElement, Render, Stateful,
    StatefulInteractiveElement, Styled, Subscription, ViewContext, VisualContext, WeakView,
};
use project::{Project, RepositoryEntry};
use recent_projects::RecentProjects;
use rpc::proto::DevServerStatus;
use settings::Settings;
use smallvec::SmallVec;
use std::sync::Arc;
use theme::{ActiveTheme, ThemeSettings};
use ui::{
    h_flex, prelude::*, Avatar, Button, ButtonLike, ButtonStyle, ContextMenu, Icon, IconButton,
    IconName, Indicator, PopoverMenu, TintColor, Tooltip,
};
use util::ResultExt;
use vcs_menu::{BranchList, OpenRecent as ToggleVcsMenu};
use workspace::{notifications::NotifyResultExt, Workspace};

const MAX_PROJECT_NAME_LENGTH: usize = 40;
const MAX_BRANCH_NAME_LENGTH: usize = 40;

actions!(
    collab,
    [
        ShareProject,
        UnshareProject,
        ToggleUserMenu,
        ToggleProjectMenu,
        SwitchBranch
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, cx| {
        let item = cx.new_view(|cx| TitleBar::new("title-bar", workspace, cx));
        workspace.set_titlebar_item(item.into(), cx)
    })
    .detach();
}

pub struct TitleBar {
    platform_style: PlatformStyle,
    content: Stateful<Div>,
    children: SmallVec<[AnyElement; 2]>,
    project: Model<Project>,
    user_store: Model<UserStore>,
    client: Arc<Client>,
    workspace: WeakView<Workspace>,
    should_move: bool,
    _subscriptions: Vec<Subscription>,
}

impl Render for TitleBar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let room = ActiveCall::global(cx).read(cx).room().cloned();
        let current_user = self.user_store.read(cx).current_user();
        let client = self.client.clone();
        let project_id = self.project.read(cx).remote_id();
        let workspace = self.workspace.upgrade();
        let close_action = Box::new(workspace::CloseWindow);

        let platform_supported = cfg!(target_os = "macos");

        let height = Self::height(cx);
        let supported_controls = cx.window_controls();
        let decorations = cx.window_decorations();

        h_flex()
            .id("titlebar")
            .w_full()
            .pt(Self::top_padding(cx))
            .h(height + Self::top_padding(cx))
            .map(|this| {
                if cx.is_fullscreen() {
                    this.pl_2()
                } else if self.platform_style == PlatformStyle::Mac {
                    this.pl(px(platform_mac::TRAFFIC_LIGHT_PADDING))
                } else {
                    this.pl_2()
                }
            })
            .map(|el| {
                match decorations {
                    Decorations::Server => el,
                    Decorations::Client { tiling, .. } => el
                        .when(!(tiling.top || tiling.right), |el| {
                            el.rounded_tr(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                        })
                        .when(!(tiling.top || tiling.left), |el| el.rounded_tl(theme::CLIENT_SIDE_DECORATION_ROUNDING))
                        // this border is to avoid a transparent gap in the rounded corners
                        .mt(px(-1.))
                        .border(px(1.))
                        .border_color(cx.theme().colors().title_bar_background),
                }
            })
            .bg(cx.theme().colors().title_bar_background)
            .content_stretch()
            .child(
                div()
                    .id("titlebar-content")
                    .flex()
                    .flex_row()
                    .justify_between()
                    .w_full()
                    // note: on windows titlebar behaviour is handled by the platform implementation
                    .when(cfg!(not(windows)), |this| {
                        this.on_click(|event, cx| {
                            if event.up.click_count == 2 {
                                cx.zoom_window();
                            }
                        })
                    })
                        // left side
                        .child(
                            h_flex()
                                .gap_1()
                                .children(self.render_application_menu(cx))
                                .children(self.render_project_host(cx))
                                .child(self.render_project_name(cx))
                                .children(self.render_project_branch(cx))
                                .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
                        )
                        .child(
                            h_flex()
                                .id("collaborator-list")
                                .w_full()
                                .gap_1()
                                .overflow_x_scroll()
                                .when_some(
                                    current_user.clone().zip(client.peer_id()).zip(room.clone()),
                                    |this, ((current_user, peer_id), room)| {
                                        let player_colors = cx.theme().players();
                                        let room = room.read(cx);
                                        let mut remote_participants =
                                            room.remote_participants().values().collect::<Vec<_>>();
                                        remote_participants.sort_by_key(|p| p.participant_index.0);

                                        let current_user_face_pile = self.render_collaborator(
                                            &current_user,
                                            peer_id,
                                            true,
                                            room.is_speaking(),
                                            room.is_muted(),
                                            None,
                                            &room,
                                            project_id,
                                            &current_user,
                                            cx,
                                        );

                                        this.children(current_user_face_pile.map(|face_pile| {
                                            v_flex()
                                                .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
                                                .child(face_pile)
                                                .child(render_color_ribbon(player_colors.local().cursor))
                                        }))
                                        .children(
                                            remote_participants.iter().filter_map(|collaborator| {
                                                let player_color = player_colors
                                                    .color_for_participant(collaborator.participant_index.0);
                                                let is_following = workspace
                                                    .as_ref()?
                                                    .read(cx)
                                                    .is_being_followed(collaborator.peer_id);
                                                let is_present = project_id.map_or(false, |project_id| {
                                                    collaborator.location
                                                        == ParticipantLocation::SharedProject { project_id }
                                                });

                                                let facepile = self.render_collaborator(
                                                    &collaborator.user,
                                                    collaborator.peer_id,
                                                    is_present,
                                                    collaborator.speaking,
                                                    collaborator.muted,
                                                    is_following.then_some(player_color.selection),
                                                    &room,
                                                    project_id,
                                                    &current_user,
                                                    cx,
                                                )?;

                                                Some(
                                                    v_flex()
                                                        .id(("collaborator", collaborator.user.id))
                                                        .child(facepile)
                                                        .child(render_color_ribbon(player_color.cursor))
                                                        .cursor_pointer()
                                                        .on_click({
                                                            let peer_id = collaborator.peer_id;
                                                            cx.listener(move |this, _, cx| {
                                                                this.workspace
                                                                    .update(cx, |workspace, cx| {
                                                                        workspace.follow(peer_id, cx);
                                                                    })
                                                                    .ok();
                                                            })
                                                        })
                                                        .tooltip({
                                                            let login = collaborator.user.github_login.clone();
                                                            move |cx| {
                                                                Tooltip::text(format!("Follow {login}"), cx)
                                                            }
                                                        }),
                                                )
                                            }),
                                        )
                                    },
                                ),
                        )
                        // right side
                        .child(
                            h_flex()
                                .gap_1()
                                .pr_1()
                                .on_mouse_down(MouseButton::Left, |_, cx| cx.stop_propagation())
                                .when_some(room, |this, room| {
                                    let room = room.read(cx);
                                    let project = self.project.read(cx);
                                    let is_local = project.is_local();
                                    let is_dev_server_project = project.dev_server_project_id().is_some();
                                    let is_shared = (is_local || is_dev_server_project) && project.is_shared();
                                    let is_muted = room.is_muted();
                                    let is_deafened = room.is_deafened().unwrap_or(false);
                                    let is_screen_sharing = room.is_screen_sharing();
                                    let can_use_microphone = room.can_use_microphone();
                                    let can_share_projects = room.can_share_projects();

                                    this.when(
                                        (is_local || is_dev_server_project) && can_share_projects,
                                        |this| {
                                            this.child(
                                                Button::new(
                                                    "toggle_sharing",
                                                    if is_shared { "Unshare" } else { "Share" },
                                                )
                                                .tooltip(move |cx| {
                                                    Tooltip::text(
                                                        if is_shared {
                                                            "Stop sharing project with call participants"
                                                        } else {
                                                            "Share project with call participants"
                                                        },
                                                        cx,
                                                    )
                                                })
                                                .style(ButtonStyle::Subtle)
                                                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                                .selected(is_shared)
                                                .label_size(LabelSize::Small)
                                                .on_click(cx.listener(
                                                    move |this, _, cx| {
                                                        if is_shared {
                                                            this.unshare_project(&Default::default(), cx);
                                                        } else {
                                                            this.share_project(&Default::default(), cx);
                                                        }
                                                    },
                                                )),
                                            )
                                        },
                                    )
                                    .child(
                                        div()
                                            .child(
                                                IconButton::new("leave-call", ui::IconName::Exit)
                                                    .style(ButtonStyle::Subtle)
                                                    .tooltip(|cx| Tooltip::text("Leave call", cx))
                                                    .icon_size(IconSize::Small)
                                                    .on_click(move |_, cx| {
                                                        ActiveCall::global(cx)
                                                            .update(cx, |call, cx| call.hang_up(cx))
                                                            .detach_and_log_err(cx);
                                                    }),
                                            )
                                            .pr_2(),
                                    )
                                    .when(can_use_microphone, |this| {
                                        this.child(
                                            IconButton::new(
                                                "mute-microphone",
                                                if is_muted {
                                                    ui::IconName::MicMute
                                                } else {
                                                    ui::IconName::Mic
                                                },
                                            )
                                            .tooltip(move |cx| {
                                                Tooltip::text(
                                                    if !platform_supported {
                                                        "Cannot share microphone"
                                                    } else if is_muted {
                                                        "Unmute microphone"
                                                    } else {
                                                        "Mute microphone"
                                                    },
                                                    cx,
                                                )
                                            })
                                            .style(ButtonStyle::Subtle)
                                            .icon_size(IconSize::Small)
                                            .selected(platform_supported && is_muted)
                                            .disabled(!platform_supported)
                                            .selected_style(ButtonStyle::Tinted(TintColor::Negative))
                                            .on_click(move |_, cx| {
                                                call_controls::toggle_mute(&Default::default(), cx);
                                            }),
                                        )
                                    })
                                    .child(
                                        IconButton::new(
                                            "mute-sound",
                                            if is_deafened {
                                                ui::IconName::AudioOff
                                            } else {
                                                ui::IconName::AudioOn
                                            },
                                        )
                                        .style(ButtonStyle::Subtle)
                                        .selected_style(ButtonStyle::Tinted(TintColor::Negative))
                                        .icon_size(IconSize::Small)
                                        .selected(is_deafened)
                                        .disabled(!platform_supported)
                                        .tooltip(move |cx| {
                                            if !platform_supported {
                                                Tooltip::text("Cannot share microphone", cx)
                                            } else if can_use_microphone {
                                                Tooltip::with_meta(
                                                    "Deafen Audio",
                                                    None,
                                                    "Mic will be muted",
                                                    cx,
                                                )
                                            } else {
                                                Tooltip::text("Deafen Audio", cx)
                                            }
                                        })
                                        .on_click(move |_, cx| {
                                            call_controls::toggle_deafen(&Default::default(), cx)
                                        }),
                                    )
                                    .when(can_share_projects, |this| {
                                        this.child(
                                            IconButton::new("screen-share", ui::IconName::Screen)
                                                .style(ButtonStyle::Subtle)
                                                .icon_size(IconSize::Small)
                                                .selected(is_screen_sharing)
                                                .disabled(!platform_supported)
                                                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                                .tooltip(move |cx| {
                                                    Tooltip::text(
                                                        if !platform_supported {
                                                            "Cannot share screen"
                                                        } else if is_screen_sharing {
                                                            "Stop Sharing Screen"
                                                        } else {
                                                            "Share Screen"
                                                        },
                                                        cx,
                                                    )
                                                })
                                                .on_click(move |_, cx| {
                                                    call_controls::toggle_screen_sharing(&Default::default(), cx)
                                                }),
                                        )
                                    })
                                    .child(div().pr_2())
                                })
                                .map(|el| {
                                    let status = self.client.status();
                                    let status = &*status.borrow();
                                    if matches!(status, client::Status::Connected { .. }) {
                                        el.child(self.render_user_menu_button(cx))
                                    } else {
                                        el.children(self.render_connection_status(status, cx))
                                            .child(self.render_sign_in_button(cx))
                                            .child(self.render_user_menu_button(cx))
                                    }
                                }),
                        )

            ).when(
            self.platform_style == PlatformStyle::Windows && !cx.is_fullscreen(),
            |title_bar| title_bar.child(platform_windows::WindowsWindowControls::new(height)),
        ).when(
            self.platform_style == PlatformStyle::Linux
                && !cx.is_fullscreen()
                && matches!(decorations, Decorations::Client { .. }),
            |title_bar| {
                title_bar
                    .child(platform_linux::LinuxWindowControls::new(close_action))
                    .when(supported_controls.window_menu, |titlebar| {
                        titlebar.on_mouse_down(gpui::MouseButton::Right, move |ev, cx| {
                            cx.show_window_menu(ev.position)
                        })
                    })

                                        .on_mouse_move(cx.listener(move |this, _ev, cx| {
                                            if this.should_move {
                                                this.should_move = false;
                                                cx.start_window_move();
                                            }
                                        }))
                                        .on_mouse_down_out(cx.listener(move |this, _ev, _cx| {
                                            this.should_move = false;
                                        }))
                                        .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |this, _ev, _cx| {
                                            this.should_move = true;
                                    }))

            },
        )
    }
}

impl TitleBar {
    pub fn new(
        id: impl Into<ElementId>,
        workspace: &Workspace,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let project = workspace.project().clone();
        let user_store = workspace.app_state().user_store.clone();
        let client = workspace.app_state().client.clone();
        let active_call = ActiveCall::global(cx);
        let mut subscriptions = Vec::new();
        subscriptions.push(
            cx.observe(&workspace.weak_handle().upgrade().unwrap(), |_, _, cx| {
                cx.notify()
            }),
        );
        subscriptions.push(cx.observe(&project, |_, _, cx| cx.notify()));
        subscriptions.push(cx.observe(&active_call, |this, _, cx| this.active_call_changed(cx)));
        subscriptions.push(cx.observe_window_activation(Self::window_activation_changed));
        subscriptions.push(cx.observe(&user_store, |_, _, cx| cx.notify()));

        Self {
            platform_style: PlatformStyle::platform(),
            content: div().id(id.into()),
            children: SmallVec::new(),
            workspace: workspace.weak_handle(),
            should_move: false,
            project,
            user_store,
            client,
            _subscriptions: subscriptions,
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn height(cx: &mut WindowContext) -> Pixels {
        (1.75 * cx.rem_size()).max(px(34.))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_cx: &mut WindowContext) -> Pixels {
        // todo(windows) instead of hard coded size report the actual size to the Windows platform API
        px(32.)
    }

    #[cfg(not(target_os = "windows"))]
    fn top_padding(_cx: &WindowContext) -> Pixels {
        px(0.)
    }

    #[cfg(target_os = "windows")]
    fn top_padding(cx: &WindowContext) -> Pixels {
        use windows::Win32::UI::{
            HiDpi::GetSystemMetricsForDpi,
            WindowsAndMessaging::{SM_CXPADDEDBORDER, USER_DEFAULT_SCREEN_DPI},
        };

        // This top padding is not dependent on the title bar style and is instead a quirk of maximized windows on Windows:
        // https://devblogs.microsoft.com/oldnewthing/20150304-00/?p=44543
        let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, USER_DEFAULT_SCREEN_DPI) };
        if cx.is_maximized() {
            px((padding * 2) as f32)
        } else {
            px(0.)
        }
    }

    /// Sets the platform style.
    pub fn platform_style(mut self, style: PlatformStyle) -> Self {
        self.platform_style = style;
        self
    }

    pub fn render_application_menu(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        cfg!(not(target_os = "macos")).then(|| {
            let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;
            let font = cx.text_style().font();
            let font_id = cx.text_system().resolve_font(&font);
            let width = cx
                .text_system()
                .typographic_bounds(font_id, ui_font_size, 'm')
                .unwrap()
                .size
                .width
                * 3.0;

            PopoverMenu::new("application-menu")
                .menu(move |cx| {
                    let width = width;
                    ContextMenu::build(cx, move |menu, _cx| {
                        let width = width;
                        menu.header("Workspace")
                            .action("Open Command Palette", Box::new(command_palette::Toggle))
                            .custom_row(move |cx| {
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .cursor(gpui::CursorStyle::Arrow)
                                    .child(Label::new("Buffer Font Size"))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .child(div().w(px(16.0)))
                                            .child(
                                                IconButton::new(
                                                    "reset-buffer-zoom",
                                                    IconName::RotateCcw,
                                                )
                                                .on_click(|_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        zed_actions::ResetBufferFontSize,
                                                    ))
                                                }),
                                            )
                                            .child(
                                                IconButton::new("--buffer-zoom", IconName::Dash)
                                                    .on_click(|_, cx| {
                                                        cx.dispatch_action(Box::new(
                                                            zed_actions::DecreaseBufferFontSize,
                                                        ))
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .w(width)
                                                    .flex()
                                                    .flex_row()
                                                    .justify_around()
                                                    .child(Label::new(
                                                        theme::get_buffer_font_size(cx).to_string(),
                                                    )),
                                            )
                                            .child(
                                                IconButton::new("+-buffer-zoom", IconName::Plus)
                                                    .on_click(|_, cx| {
                                                        cx.dispatch_action(Box::new(
                                                            zed_actions::IncreaseBufferFontSize,
                                                        ))
                                                    }),
                                            ),
                                    )
                                    .into_any_element()
                            })
                            .custom_row(move |cx| {
                                div()
                                    .w_full()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .cursor(gpui::CursorStyle::Arrow)
                                    .child(Label::new("UI Font Size"))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .child(
                                                IconButton::new(
                                                    "reset-ui-zoom",
                                                    IconName::RotateCcw,
                                                )
                                                .on_click(|_, cx| {
                                                    cx.dispatch_action(Box::new(
                                                        zed_actions::ResetUiFontSize,
                                                    ))
                                                }),
                                            )
                                            .child(
                                                IconButton::new("--ui-zoom", IconName::Dash)
                                                    .on_click(|_, cx| {
                                                        cx.dispatch_action(Box::new(
                                                            zed_actions::DecreaseUiFontSize,
                                                        ))
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .w(width)
                                                    .flex()
                                                    .flex_row()
                                                    .justify_around()
                                                    .child(Label::new(
                                                        theme::get_ui_font_size(cx).to_string(),
                                                    )),
                                            )
                                            .child(
                                                IconButton::new("+-ui-zoom", IconName::Plus)
                                                    .on_click(|_, cx| {
                                                        cx.dispatch_action(Box::new(
                                                            zed_actions::IncreaseUiFontSize,
                                                        ))
                                                    }),
                                            ),
                                    )
                                    .into_any_element()
                            })
                            .header("Project")
                            .action(
                                "Add Folder to Project...",
                                Box::new(workspace::AddFolderToProject),
                            )
                            .action("Open a new Project...", Box::new(workspace::Open))
                            .action(
                                "Open Recent Projects...",
                                Box::new(recent_projects::OpenRecent {
                                    create_new_window: false,
                                }),
                            )
                            .header("Help")
                            .action("About Zed", Box::new(zed_actions::About))
                            .action("Welcome", Box::new(workspace::Welcome))
                            .link(
                                "Documentation",
                                Box::new(zed_actions::OpenBrowser {
                                    url: "https://zed.dev/docs".into(),
                                }),
                            )
                            .action("Give Feedback", Box::new(feedback::GiveFeedback))
                            .action("Check for Updates", Box::new(auto_update::Check))
                            .action("View Telemetry", Box::new(zed_actions::OpenTelemetryLog))
                            .action(
                                "View Dependency Licenses",
                                Box::new(zed_actions::OpenLicenses),
                            )
                            .separator()
                            .action("Quit", Box::new(zed_actions::Quit))
                    })
                    .into()
                })
                .trigger(
                    IconButton::new("application-menu", ui::IconName::Menu)
                        .style(ButtonStyle::Subtle)
                        .tooltip(|cx| Tooltip::text("Open Application Menu", cx))
                        .icon_size(IconSize::Small),
                )
                .into_any_element()
        })
    }

    pub fn render_project_host(&self, cx: &mut ViewContext<Self>) -> Option<AnyElement> {
        if let Some(dev_server) =
            self.project
                .read(cx)
                .dev_server_project_id()
                .and_then(|dev_server_project_id| {
                    dev_server_projects::Store::global(cx)
                        .read(cx)
                        .dev_server_for_project(dev_server_project_id)
                })
        {
            return Some(
                ButtonLike::new("dev_server_trigger")
                    .child(Indicator::dot().color(
                        if dev_server.status == DevServerStatus::Online {
                            Color::Created
                        } else {
                            Color::Disabled
                        },
                    ))
                    .child(
                        Label::new(dev_server.name.clone())
                            .size(LabelSize::Small)
                            .line_height_style(LineHeightStyle::UiLabel),
                    )
                    .tooltip(move |cx| Tooltip::text("Project is hosted on a dev server", cx))
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            recent_projects::DevServerProjects::open(workspace, cx)
                        }
                    }))
                    .into_any_element(),
            );
        }

        if self.project.read(cx).is_disconnected() {
            return Some(
                Button::new("disconnected", "Disconnected")
                    .disabled(true)
                    .color(Color::Disabled)
                    .style(ButtonStyle::Subtle)
                    .label_size(LabelSize::Small)
                    .into_any_element(),
            );
        }

        let host = self.project.read(cx).host()?;
        let host_user = self.user_store.read(cx).get_cached_user(host.user_id)?;
        let participant_index = self
            .user_store
            .read(cx)
            .participant_indices()
            .get(&host_user.id)?;
        Some(
            Button::new("project_owner_trigger", host_user.github_login.clone())
                .color(Color::Player(participant_index.0))
                .style(ButtonStyle::Subtle)
                .label_size(LabelSize::Small)
                .tooltip(move |cx| {
                    Tooltip::text(
                        format!(
                            "{} is sharing this project. Click to follow.",
                            host_user.github_login.clone()
                        ),
                        cx,
                    )
                })
                .on_click({
                    let host_peer_id = host.peer_id;
                    cx.listener(move |this, _, cx| {
                        this.workspace
                            .update(cx, |workspace, cx| {
                                workspace.follow(host_peer_id, cx);
                            })
                            .log_err();
                    })
                })
                .into_any_element(),
        )
    }

    pub fn render_project_name(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let name = {
            let mut names = self.project.read(cx).visible_worktrees(cx).map(|worktree| {
                let worktree = worktree.read(cx);
                worktree.root_name()
            });

            names.next()
        };
        let is_project_selected = name.is_some();
        let name = if let Some(name) = name {
            util::truncate_and_trailoff(name, MAX_PROJECT_NAME_LENGTH)
        } else {
            "Open recent project".to_string()
        };

        let workspace = self.workspace.clone();
        Button::new("project_name_trigger", name)
            .when(!is_project_selected, |b| b.color(Color::Muted))
            .style(ButtonStyle::Subtle)
            .label_size(LabelSize::Small)
            .tooltip(move |cx| {
                Tooltip::for_action(
                    "Recent Projects",
                    &recent_projects::OpenRecent {
                        create_new_window: false,
                    },
                    cx,
                )
            })
            .on_click(cx.listener(move |_, _, cx| {
                if let Some(workspace) = workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        RecentProjects::open(workspace, false, cx);
                    })
                }
            }))
    }

    pub fn render_project_branch(&self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let entry = {
            let mut names_and_branches =
                self.project.read(cx).visible_worktrees(cx).map(|worktree| {
                    let worktree = worktree.read(cx);
                    worktree.root_git_entry()
                });

            names_and_branches.next().flatten()
        };
        let workspace = self.workspace.upgrade()?;
        let branch_name = entry
            .as_ref()
            .and_then(RepositoryEntry::branch)
            .map(|branch| util::truncate_and_trailoff(&branch, MAX_BRANCH_NAME_LENGTH))?;
        Some(
            Button::new("project_branch_trigger", branch_name)
                .color(Color::Muted)
                .style(ButtonStyle::Subtle)
                .label_size(LabelSize::Small)
                .tooltip(move |cx| {
                    Tooltip::with_meta(
                        "Recent Branches",
                        Some(&ToggleVcsMenu),
                        "Local branches only",
                        cx,
                    )
                })
                .on_click(move |_, cx| {
                    let _ = workspace.update(cx, |this, cx| {
                        BranchList::open(this, &Default::default(), cx)
                    });
                }),
        )
    }

    fn window_activation_changed(&mut self, cx: &mut ViewContext<Self>) {
        if cx.is_window_active() {
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.set_location(Some(&self.project), cx))
                .detach_and_log_err(cx);
        } else if cx.active_window().is_none() {
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.set_location(None, cx))
                .detach_and_log_err(cx);
        }
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.update_active_view_for_followers(cx);
            })
            .ok();
    }

    fn active_call_changed(&mut self, cx: &mut ViewContext<Self>) {
        cx.notify();
    }

    fn share_project(&mut self, _: &ShareProject, cx: &mut ViewContext<Self>) {
        let active_call = ActiveCall::global(cx);
        let project = self.project.clone();
        active_call
            .update(cx, |call, cx| call.share_project(project, cx))
            .detach_and_log_err(cx);
    }

    fn unshare_project(&mut self, _: &UnshareProject, cx: &mut ViewContext<Self>) {
        let active_call = ActiveCall::global(cx);
        let project = self.project.clone();
        active_call
            .update(cx, |call, cx| call.unshare_project(project, cx))
            .log_err();
    }

    fn render_connection_status(
        &self,
        status: &client::Status,
        cx: &mut ViewContext<Self>,
    ) -> Option<AnyElement> {
        match status {
            client::Status::ConnectionError
            | client::Status::ConnectionLost
            | client::Status::Reauthenticating { .. }
            | client::Status::Reconnecting { .. }
            | client::Status::ReconnectionError { .. } => Some(
                div()
                    .id("disconnected")
                    .child(Icon::new(IconName::Disconnected).size(IconSize::Small))
                    .tooltip(|cx| Tooltip::text("Disconnected", cx))
                    .into_any_element(),
            ),
            client::Status::UpgradeRequired => {
                let auto_updater = auto_update::AutoUpdater::get(cx);
                let label = match auto_updater.map(|auto_update| auto_update.read(cx).status()) {
                    Some(AutoUpdateStatus::Updated { .. }) => "Please restart Zed to Collaborate",
                    Some(AutoUpdateStatus::Installing)
                    | Some(AutoUpdateStatus::Downloading)
                    | Some(AutoUpdateStatus::Checking) => "Updating...",
                    Some(AutoUpdateStatus::Idle) | Some(AutoUpdateStatus::Errored) | None => {
                        "Please update Zed to Collaborate"
                    }
                };

                Some(
                    Button::new("connection-status", label)
                        .label_size(LabelSize::Small)
                        .on_click(|_, cx| {
                            if let Some(auto_updater) = auto_update::AutoUpdater::get(cx) {
                                if auto_updater.read(cx).status().is_updated() {
                                    workspace::reload(&Default::default(), cx);
                                    return;
                                }
                            }
                            auto_update::check(&Default::default(), cx);
                        })
                        .into_any_element(),
                )
            }
            _ => None,
        }
    }

    pub fn render_sign_in_button(&mut self, _: &mut ViewContext<Self>) -> Button {
        let client = self.client.clone();
        Button::new("sign_in", "Sign in")
            .label_size(LabelSize::Small)
            .on_click(move |_, cx| {
                let client = client.clone();
                cx.spawn(move |mut cx| async move {
                    client
                        .authenticate_and_connect(true, &cx)
                        .await
                        .notify_async_err(&mut cx);
                })
                .detach();
            })
    }

    pub fn render_user_menu_button(&mut self, cx: &mut ViewContext<Self>) -> impl Element {
        if let Some(user) = self.user_store.read(cx).current_user() {
            PopoverMenu::new("user-menu")
                .menu(|cx| {
                    ContextMenu::build(cx, |menu, _| {
                        menu.action("Settings", zed_actions::OpenSettings.boxed_clone())
                            .action("Key Bindings", Box::new(zed_actions::OpenKeymap))
                            .action("Themes…", theme_selector::Toggle::default().boxed_clone())
                            .action("Extensions", extensions_ui::Extensions.boxed_clone())
                            .separator()
                            .action("Sign Out", client::SignOut.boxed_clone())
                    })
                    .into()
                })
                .trigger(
                    ButtonLike::new("user-menu")
                        .child(
                            h_flex()
                                .gap_0p5()
                                .child(Avatar::new(user.avatar_uri.clone()))
                                .child(
                                    Icon::new(IconName::ChevronDown)
                                        .size(IconSize::Small)
                                        .color(Color::Muted),
                                ),
                        )
                        .style(ButtonStyle::Subtle)
                        .tooltip(move |cx| Tooltip::text("Toggle User Menu", cx)),
                )
                .anchor(gpui::AnchorCorner::TopRight)
        } else {
            PopoverMenu::new("user-menu")
                .menu(|cx| {
                    ContextMenu::build(cx, |menu, _| {
                        menu.action("Settings", zed_actions::OpenSettings.boxed_clone())
                            .action("Key Bindings", Box::new(zed_actions::OpenKeymap))
                            .action("Themes…", theme_selector::Toggle::default().boxed_clone())
                            .action("Extensions", extensions_ui::Extensions.boxed_clone())
                    })
                    .into()
                })
                .trigger(
                    ButtonLike::new("user-menu")
                        .child(
                            h_flex().gap_0p5().child(
                                Icon::new(IconName::ChevronDown)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            ),
                        )
                        .style(ButtonStyle::Subtle)
                        .tooltip(move |cx| Tooltip::text("Toggle User Menu", cx)),
                )
        }
    }
}

impl InteractiveElement for TitleBar {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.content.interactivity()
    }
}

impl StatefulInteractiveElement for TitleBar {}

impl ParentElement for TitleBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
