use crate::face_pile::FacePile;
use call::{ActiveCall, ParticipantLocation, Room};
use client::{proto::PeerId, Client, ParticipantIndex, User, UserStore};
use gpui::{
    actions, canvas, div, overlay, point, px, rems, AppContext, DismissEvent, Div, Element,
    FocusableView, Hsla, InteractiveElement, IntoElement, Model, ParentElement, Path, Render,
    Stateful, StatefulInteractiveElement, Styled, Subscription, ViewContext, VisualContext,
    WeakView, WindowBounds,
};
use project::{Project, RepositoryEntry};
use recent_projects::RecentProjects;
use std::sync::Arc;
use theme::{ActiveTheme, PlayerColors};
use ui::{
    h_stack, popover_menu, prelude::*, Avatar, Button, ButtonLike, ButtonStyle, ContextMenu, Icon,
    IconButton, IconElement, KeyBinding, Tooltip,
};
use util::ResultExt;
use workspace::{notifications::NotifyResultExt, Workspace, WORKSPACE_DB};

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
        let titlebar_item = cx.build_view(|cx| CollabTitlebarItem::new(workspace, cx));
        workspace.set_titlebar_item(titlebar_item.into(), cx)
    })
    .detach();
    // cx.add_action(CollabTitlebarItem::share_project);
    // cx.add_action(CollabTitlebarItem::unshare_project);
    // cx.add_action(CollabTitlebarItem::toggle_user_menu);
    // cx.add_action(CollabTitlebarItem::toggle_vcs_menu);
    // cx.add_action(CollabTitlebarItem::toggle_project_menu);
}

pub struct CollabTitlebarItem {
    project: Model<Project>,
    user_store: Model<UserStore>,
    client: Arc<Client>,
    workspace: WeakView<Workspace>,
    //branch_popover: Option<ViewHandle<BranchList>>,
    project_popover: Option<recent_projects::RecentProjects>,
    //user_menu: ViewHandle<ContextMenu>,
    _subscriptions: Vec<Subscription>,
}

impl Render for CollabTitlebarItem {
    type Element = Stateful<Div>;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        let room = ActiveCall::global(cx).read(cx).room().cloned();
        let current_user = self.user_store.read(cx).current_user();
        let client = self.client.clone();
        let project_id = self.project.read(cx).remote_id();

        h_stack()
            .id("titlebar")
            .justify_between()
            .w_full()
            .h(rems(1.75))
            // Set a non-scaling min-height here to ensure the titlebar is
            // always at least the height of the traffic lights.
            .min_h(px(32.))
            .when(
                !matches!(cx.window_bounds(), WindowBounds::Fullscreen),
                // Use pixels here instead of a rem-based size because the macOS traffic
                // lights are a static size, and don't scale with the rest of the UI.
                |s| s.pl(px(68.)),
            )
            .bg(cx.theme().colors().title_bar_background)
            .on_click(|event, cx| {
                if event.up.click_count == 2 {
                    cx.zoom_window();
                }
            })
            // left side
            .child(
                h_stack()
                    .gap_1()
                    .children(self.render_project_host(cx))
                    .child(self.render_project_name(cx))
                    .children(self.render_project_branch(cx))
                    .when_some(
                        current_user.clone().zip(client.peer_id()).zip(room.clone()),
                        |this, ((current_user, peer_id), room)| {
                            let player_colors = cx.theme().players();
                            let room = room.read(cx);
                            let mut remote_participants =
                                room.remote_participants().values().collect::<Vec<_>>();
                            remote_participants.sort_by_key(|p| p.participant_index.0);

                            this.children(self.render_collaborator(
                                &current_user,
                                peer_id,
                                true,
                                room.is_speaking(),
                                room.is_muted(cx),
                                &room,
                                project_id,
                                &current_user,
                            ))
                            .children(
                                remote_participants.iter().filter_map(|collaborator| {
                                    let is_present = project_id.map_or(false, |project_id| {
                                        collaborator.location
                                            == ParticipantLocation::SharedProject { project_id }
                                    });

                                    let face_pile = self.render_collaborator(
                                        &collaborator.user,
                                        collaborator.peer_id,
                                        is_present,
                                        collaborator.speaking,
                                        collaborator.muted,
                                        &room,
                                        project_id,
                                        &current_user,
                                    )?;

                                    Some(
                                        v_stack()
                                            .id(("collaborator", collaborator.user.id))
                                            .child(face_pile)
                                            .child(render_color_ribbon(
                                                collaborator.participant_index,
                                                player_colors,
                                            ))
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
                h_stack()
                    .gap_1()
                    .when_some(room, |this, room| {
                        let room = room.read(cx);
                        let is_shared = self.project.read(cx).is_shared();
                        let is_muted = room.is_muted(cx);
                        let is_deafened = room.is_deafened().unwrap_or(false);

                        this.child(
                            Button::new(
                                "toggle_sharing",
                                if is_shared { "Unshare" } else { "Share" },
                            )
                            .style(ButtonStyle::Subtle)
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
                        .child(
                            IconButton::new("leave-call", ui::Icon::Exit)
                                .style(ButtonStyle::Subtle)
                                .on_click(move |_, cx| {
                                    ActiveCall::global(cx)
                                        .update(cx, |call, cx| call.hang_up(cx))
                                        .detach_and_log_err(cx);
                                }),
                        )
                        .child(
                            IconButton::new(
                                "mute-microphone",
                                if is_muted {
                                    ui::Icon::MicMute
                                } else {
                                    ui::Icon::Mic
                                },
                            )
                            .style(ButtonStyle::Subtle)
                            .selected(is_muted)
                            .on_click(move |_, cx| crate::toggle_mute(&Default::default(), cx)),
                        )
                        .child(
                            IconButton::new(
                                "mute-sound",
                                if is_deafened {
                                    ui::Icon::AudioOff
                                } else {
                                    ui::Icon::AudioOn
                                },
                            )
                            .style(ButtonStyle::Subtle)
                            .selected(is_deafened.clone())
                            .tooltip(move |cx| {
                                Tooltip::with_meta("Deafen Audio", None, "Mic will be muted", cx)
                            })
                            .on_click(move |_, cx| crate::toggle_mute(&Default::default(), cx)),
                        )
                        .child(
                            IconButton::new("screen-share", ui::Icon::Screen)
                                .style(ButtonStyle::Subtle)
                                .on_click(move |_, cx| {
                                    crate::toggle_screen_sharing(&Default::default(), cx)
                                }),
                        )
                    })
                    .child(h_stack().px_1p5().map(|this| {
                        if let Some(user) = current_user {
                            // TODO: Finish implementing user menu popover
                            //
                            this.child(
                                popover_menu("user-menu")
                                    .menu(|cx| {
                                        ContextMenu::build(cx, |menu, _| menu.header("ADADA"))
                                    })
                                    .trigger(
                                        ButtonLike::new("user-menu")
                                            .child(
                                                h_stack()
                                                    .gap_0p5()
                                                    .child(Avatar::new(user.avatar_uri.clone()))
                                                    .child(
                                                        IconElement::new(Icon::ChevronDown)
                                                            .color(Color::Muted),
                                                    ),
                                            )
                                            .style(ButtonStyle::Subtle)
                                            .tooltip(move |cx| {
                                                Tooltip::text("Toggle User Menu", cx)
                                            }),
                                    )
                                    .anchor(gpui::AnchorCorner::TopRight),
                            )
                            // this.child(
                            //     ButtonLike::new("user-menu")
                            //         .child(
                            //             h_stack().gap_0p5().child(Avatar::data(avatar)).child(
                            //                 IconElement::new(Icon::ChevronDown).color(Color::Muted),
                            //             ),
                            //         )
                            //         .style(ButtonStyle::Subtle)
                            //         .tooltip(move |cx| Tooltip::text("Toggle User Menu", cx)),
                            // )
                        } else {
                            this.child(Button::new("sign_in", "Sign in").on_click(move |_, cx| {
                                let client = client.clone();
                                cx.spawn(move |mut cx| async move {
                                    client
                                        .authenticate_and_connect(true, &cx)
                                        .await
                                        .notify_async_err(&mut cx);
                                })
                                .detach();
                            }))
                        }
                    })),
            )
    }
}

fn render_color_ribbon(participant_index: ParticipantIndex, colors: &PlayerColors) -> gpui::Canvas {
    let color = colors.color_for_participant(participant_index.0).cursor;
    canvas(move |bounds, cx| {
        let mut path = Path::new(bounds.lower_left());
        let height = bounds.size.height;
        path.curve_to(bounds.origin + point(height, px(0.)), bounds.origin);
        path.line_to(bounds.upper_right() - point(height, px(0.)));
        path.curve_to(bounds.lower_right(), bounds.upper_right());
        path.line_to(bounds.lower_left());
        cx.paint_path(path, color);
    })
    .h_1()
    .w_full()
}

impl CollabTitlebarItem {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
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
            workspace: workspace.weak_handle(),
            project,
            user_store,
            client,
            //         user_menu: cx.add_view(|cx| {
            //             let view_id = cx.view_id();
            //             let mut menu = ContextMenu::new(view_id, cx);
            //             menu.set_position_mode(OverlayPositionMode::Local);
            //             menu
            //         }),
            //         branch_popover: None,
            project_popover: None,
            _subscriptions: subscriptions,
        }
    }

    // resolve if you are in a room -> render_project_owner
    // render_project_owner -> resolve if you are in a room -> Option<foo>

    pub fn render_project_host(&self, cx: &mut ViewContext<Self>) -> Option<impl Element> {
        let host = self.project.read(cx).host()?;
        let host = self.user_store.read(cx).get_cached_user(host.user_id)?;
        let participant_index = self
            .user_store
            .read(cx)
            .participant_indices()
            .get(&host.id)?;
        Some(
            div().border().border_color(gpui::red()).child(
                Button::new("project_owner_trigger", host.github_login.clone())
                    .color(Color::Player(participant_index.0))
                    .style(ButtonStyle::Subtle)
                    .tooltip(move |cx| Tooltip::text("Toggle following", cx)),
            ),
        )
    }

    pub fn render_project_name(&self, cx: &mut ViewContext<Self>) -> impl Element {
        let name = {
            let mut names = self.project.read(cx).visible_worktrees(cx).map(|worktree| {
                let worktree = worktree.read(cx);
                worktree.root_name()
            });

            names.next().unwrap_or("")
        };

        let name = util::truncate_and_trailoff(name, MAX_PROJECT_NAME_LENGTH);

        div()
            .border()
            .border_color(gpui::red())
            .child(
                Button::new("project_name_trigger", name)
                    .style(ButtonStyle::Subtle)
                    .tooltip(move |cx| Tooltip::text("Recent Projects", cx))
                    .on_click(cx.listener(|this, _, cx| {
                        this.toggle_project_menu(&ToggleProjectMenu, cx);
                    })),
            )
            .children(self.project_popover.as_ref().map(|popover| {
                overlay().child(
                    div()
                        .min_w_56()
                        .on_mouse_down_out(cx.listener_for(&popover.picker, |picker, _, cx| {
                            picker.cancel(&Default::default(), cx)
                        }))
                        .child(popover.picker.clone()),
                )
            }))
    }

    pub fn render_project_branch(&self, cx: &mut ViewContext<Self>) -> Option<impl Element> {
        let entry = {
            let mut names_and_branches =
                self.project.read(cx).visible_worktrees(cx).map(|worktree| {
                    let worktree = worktree.read(cx);
                    worktree.root_git_entry()
                });

            names_and_branches.next().flatten()
        };

        let branch_name = entry
            .as_ref()
            .and_then(RepositoryEntry::branch)
            .map(|branch| util::truncate_and_trailoff(&branch, MAX_BRANCH_NAME_LENGTH))?;

        Some(
            div().border().border_color(gpui::red()).child(
                Button::new("project_branch_trigger", branch_name)
                    .style(ButtonStyle::Subtle)
                    .tooltip(move |cx| {
                        cx.build_view(|_| {
                            Tooltip::new("Recent Branches")
                                .key_binding(KeyBinding::new(gpui::KeyBinding::new(
                                    "cmd-b",
                                    // todo!() Replace with real action.
                                    gpui::NoAction,
                                    None,
                                )))
                                .meta("Local branches only")
                        })
                        .into()
                    }),
            ),
        )
    }

    fn render_collaborator(
        &self,
        user: &Arc<User>,
        peer_id: PeerId,
        is_present: bool,
        is_speaking: bool,
        is_muted: bool,
        room: &Room,
        project_id: Option<u64>,
        current_user: &Arc<User>,
    ) -> Option<FacePile> {
        let followers = project_id.map_or(&[] as &[_], |id| room.followers_for(peer_id, id));

        let pile = FacePile::default().child(
            div()
                .child(
                    Avatar::new(user.avatar_uri.clone())
                        .grayscale(!is_present)
                        .border_color(if is_speaking {
                            gpui::blue()
                        } else if is_muted {
                            gpui::red()
                        } else {
                            Hsla::default()
                        }),
                )
                .children(followers.iter().filter_map(|follower_peer_id| {
                    let follower = room
                        .remote_participants()
                        .values()
                        .find_map(|p| (p.peer_id == *follower_peer_id).then_some(&p.user))
                        .or_else(|| {
                            (self.client.peer_id() == Some(*follower_peer_id))
                                .then_some(current_user)
                        })?
                        .clone();

                    Some(div().child(Avatar::new(follower.avatar_uri.clone())))
                })),
        );

        Some(pile)
    }

    fn window_activation_changed(&mut self, cx: &mut ViewContext<Self>) {
        let project = if cx.is_window_active() {
            Some(self.project.clone())
        } else {
            None
        };
        ActiveCall::global(cx)
            .update(cx, |call, cx| call.set_location(project.as_ref(), cx))
            .detach_and_log_err(cx);
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

    // pub fn toggle_user_menu(&mut self, _: &ToggleUserMenu, cx: &mut ViewContext<Self>) {
    //     self.user_menu.update(cx, |user_menu, cx| {
    //         let items = if let Some(_) = self.user_store.read(cx).current_user() {
    //             vec![
    //                 ContextMenuItem::action("Settings", zed_actions::OpenSettings),
    //                 ContextMenuItem::action("Theme", theme_selector::Toggle),
    //                 ContextMenuItem::separator(),
    //                 ContextMenuItem::action(
    //                     "Share Feedback",
    //                     feedback::feedback_editor::GiveFeedback,
    //                 ),
    //                 ContextMenuItem::action("Sign Out", SignOut),
    //             ]
    //         } else {
    //             vec![
    //                 ContextMenuItem::action("Settings", zed_actions::OpenSettings),
    //                 ContextMenuItem::action("Theme", theme_selector::Toggle),
    //                 ContextMenuItem::separator(),
    //                 ContextMenuItem::action(
    //                     "Share Feedback",
    //                     feedback::feedback_editor::GiveFeedback,
    //                 ),
    //             ]
    //         };
    //         user_menu.toggle(Default::default(), AnchorCorner::TopRight, items, cx);
    //     });
    // }

    // fn render_branches_popover_host<'a>(
    //     &'a self,
    //     _theme: &'a theme::Titlebar,
    //     cx: &'a mut ViewContext<Self>,
    // ) -> Option<AnyElement<Self>> {
    //     self.branch_popover.as_ref().map(|child| {
    //         let theme = theme::current(cx).clone();
    //         let child = ChildView::new(child, cx);
    //         let child = MouseEventHandler::new::<BranchList, _>(0, cx, |_, _| {
    //             child
    //                 .flex(1., true)
    //                 .contained()
    //                 .constrained()
    //                 .with_width(theme.titlebar.menu.width)
    //                 .with_height(theme.titlebar.menu.height)
    //         })
    //         .on_click(MouseButton::Left, |_, _, _| {})
    //         .on_down_out(MouseButton::Left, move |_, this, cx| {
    //             this.branch_popover.take();
    //             cx.emit(());
    //             cx.notify();
    //         })
    //         .contained()
    //         .into_any();

    //         Overlay::new(child)
    //             .with_fit_mode(OverlayFitMode::SwitchAnchor)
    //             .with_anchor_corner(AnchorCorner::TopLeft)
    //             .with_z_index(999)
    //             .aligned()
    //             .bottom()
    //             .left()
    //             .into_any()
    //     })
    // }

    // fn render_project_popover_host<'a>(
    //     &'a self,
    //     _theme: &'a theme::Titlebar,
    //     cx: &'a mut ViewContext<Self>,
    // ) -> Option<AnyElement<Self>> {
    //     self.project_popover.as_ref().map(|child| {
    //         let theme = theme::current(cx).clone();
    //         let child = ChildView::new(child, cx);
    //         let child = MouseEventHandler::new::<RecentProjects, _>(0, cx, |_, _| {
    //             child
    //                 .flex(1., true)
    //                 .contained()
    //                 .constrained()
    //                 .with_width(theme.titlebar.menu.width)
    //                 .with_height(theme.titlebar.menu.height)
    //         })
    //         .on_click(MouseButton::Left, |_, _, _| {})
    //         .on_down_out(MouseButton::Left, move |_, this, cx| {
    //             this.project_popover.take();
    //             cx.emit(());
    //             cx.notify();
    //         })
    //         .into_any();

    //         Overlay::new(child)
    //             .with_fit_mode(OverlayFitMode::SwitchAnchor)
    //             .with_anchor_corner(AnchorCorner::TopLeft)
    //             .with_z_index(999)
    //             .aligned()
    //             .bottom()
    //             .left()
    //             .into_any()
    //     })
    // }

    // pub fn toggle_vcs_menu(&mut self, _: &ToggleVcsMenu, cx: &mut ViewContext<Self>) {
    //     if self.branch_popover.take().is_none() {
    //         if let Some(workspace) = self.workspace.upgrade(cx) {
    //             let Some(view) =
    //                 cx.add_option_view(|cx| build_branch_list(workspace, cx).log_err())
    //             else {
    //                 return;
    //             };
    //             cx.subscribe(&view, |this, _, event, cx| {
    //                 match event {
    //                     PickerEvent::Dismiss => {
    //                         this.branch_popover = None;
    //                     }
    //                 }

    //                 cx.notify();
    //             })
    //             .detach();
    //             self.project_popover.take();
    //             cx.focus(&view);
    //             self.branch_popover = Some(view);
    //         }
    //     }

    //     cx.notify();
    // }

    pub fn toggle_project_menu(&mut self, _: &ToggleProjectMenu, cx: &mut ViewContext<Self>) {
        let workspace = self.workspace.clone();
        if self.project_popover.take().is_none() {
            cx.spawn(|this, mut cx| async move {
                let workspaces = WORKSPACE_DB
                    .recent_workspaces_on_disk()
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(_, location)| location)
                    .collect();

                let workspace = workspace.clone();
                this.update(&mut cx, move |this, cx| {
                    let view = RecentProjects::open_popover(workspace, workspaces, cx);

                    cx.subscribe(&view.picker, |this, _, _: &DismissEvent, cx| {
                        this.project_popover = None;
                        cx.notify();
                    })
                    .detach();
                    let focus_handle = view.focus_handle(cx);
                    cx.focus(&focus_handle);
                    // todo!()
                    //this.branch_popover.take();
                    this.project_popover = Some(view);
                    cx.notify();
                })
                .log_err();
            })
            .detach();
        }
        cx.notify();
    }

    // fn render_user_menu_button(
    //     &self,
    //     theme: &Theme,
    //     avatar: Option<Arc<ImageData>>,
    //     cx: &mut ViewContext<Self>,
    // ) -> AnyElement<Self> {
    //     let tooltip = theme.tooltip.clone();
    //     let user_menu_button_style = if avatar.is_some() {
    //         &theme.titlebar.user_menu.user_menu_button_online
    //     } else {
    //         &theme.titlebar.user_menu.user_menu_button_offline
    //     };

    //     let avatar_style = &user_menu_button_style.avatar;
    //     Stack::new()
    //         .with_child(
    //             MouseEventHandler::new::<ToggleUserMenu, _>(0, cx, |state, _| {
    //                 let style = user_menu_button_style
    //                     .user_menu
    //                     .inactive_state()
    //                     .style_for(state);

    //                 let mut dropdown = Flex::row().align_children_center();

    //                 if let Some(avatar_img) = avatar {
    //                     dropdown = dropdown.with_child(Self::render_face(
    //                         avatar_img,
    //                         *avatar_style,
    //                         Color::transparent_black(),
    //                         None,
    //                     ));
    //                 };

    //                 dropdown
    //                     .with_child(
    //                         Svg::new("icons/caret_down.svg")
    //                             .with_color(user_menu_button_style.icon.color)
    //                             .constrained()
    //                             .with_width(user_menu_button_style.icon.width)
    //                             .contained()
    //                             .into_any(),
    //                     )
    //                     .aligned()
    //                     .constrained()
    //                     .with_height(style.width)
    //                     .contained()
    //                     .with_style(style.container)
    //                     .into_any()
    //             })
    //             .with_cursor_style(CursorStyle::PointingHand)
    //             .on_down(MouseButton::Left, move |_, this, cx| {
    //                 this.user_menu.update(cx, |menu, _| menu.delay_cancel());
    //             })
    //             .on_click(MouseButton::Left, move |_, this, cx| {
    //                 this.toggle_user_menu(&Default::default(), cx)
    //             })
    //             .with_tooltip::<ToggleUserMenu>(
    //                 0,
    //                 "Toggle User Menu".to_owned(),
    //                 Some(Box::new(ToggleUserMenu)),
    //                 tooltip,
    //                 cx,
    //             )
    //             .contained(),
    //         )
    //         .with_child(
    //             ChildView::new(&self.user_menu, cx)
    //                 .aligned()
    //                 .bottom()
    //                 .right(),
    //         )
    //         .into_any()
    // }

    // fn render_sign_in_button(&self, theme: &Theme, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
    //     let titlebar = &theme.titlebar;
    //     MouseEventHandler::new::<SignIn, _>(0, cx, |state, _| {
    //         let style = titlebar.sign_in_button.inactive_state().style_for(state);
    //         Label::new("Sign In", style.text.clone())
    //             .contained()
    //             .with_style(style.container)
    //     })
    //     .with_cursor_style(CursorStyle::PointingHand)
    //     .on_click(MouseButton::Left, move |_, this, cx| {
    //         let client = this.client.clone();
    //         cx.app_context()
    //             .spawn(|cx| async move { client.authenticate_and_connect(true, &cx).await })
    //             .detach_and_log_err(cx);
    //     })
    //     .into_any()
    // }

    // fn render_connection_status(
    //     &self,
    //     status: &client::Status,
    //     cx: &mut ViewContext<Self>,
    // ) -> Option<AnyElement<Self>> {
    //     enum ConnectionStatusButton {}

    //     let theme = &theme::current(cx).clone();
    //     match status {
    //         client::Status::ConnectionError
    //         | client::Status::ConnectionLost
    //         | client::Status::Reauthenticating { .. }
    //         | client::Status::Reconnecting { .. }
    //         | client::Status::ReconnectionError { .. } => Some(
    //             Svg::new("icons/disconnected.svg")
    //                 .with_color(theme.titlebar.offline_icon.color)
    //                 .constrained()
    //                 .with_width(theme.titlebar.offline_icon.width)
    //                 .aligned()
    //                 .contained()
    //                 .with_style(theme.titlebar.offline_icon.container)
    //                 .into_any(),
    //         ),
    //         client::Status::UpgradeRequired => {
    //             let auto_updater = auto_update::AutoUpdater::get(cx);
    //             let label = match auto_updater.map(|auto_update| auto_update.read(cx).status()) {
    //                 Some(AutoUpdateStatus::Updated) => "Please restart Zed to Collaborate",
    //                 Some(AutoUpdateStatus::Installing)
    //                 | Some(AutoUpdateStatus::Downloading)
    //                 | Some(AutoUpdateStatus::Checking) => "Updating...",
    //                 Some(AutoUpdateStatus::Idle) | Some(AutoUpdateStatus::Errored) | None => {
    //                     "Please update Zed to Collaborate"
    //                 }
    //             };

    //             Some(
    //                 MouseEventHandler::new::<ConnectionStatusButton, _>(0, cx, |_, _| {
    //                     Label::new(label, theme.titlebar.outdated_warning.text.clone())
    //                         .contained()
    //                         .with_style(theme.titlebar.outdated_warning.container)
    //                         .aligned()
    //                 })
    //                 .with_cursor_style(CursorStyle::PointingHand)
    //                 .on_click(MouseButton::Left, |_, _, cx| {
    //                     if let Some(auto_updater) = auto_update::AutoUpdater::get(cx) {
    //                         if auto_updater.read(cx).status() == AutoUpdateStatus::Updated {
    //                             workspace::restart(&Default::default(), cx);
    //                             return;
    //                         }
    //                     }
    //                     auto_update::check(&Default::default(), cx);
    //                 })
    //                 .into_any(),
    //             )
    //         }
    //         _ => None,
    //     }
    // }
}
