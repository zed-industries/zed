use crate::face_pile::FacePile;
use call::{ActiveCall, Room};
use client::{proto::PeerId, Client, ParticipantIndex, User, UserStore};
use gpui::{
    actions, canvas, div, point, px, rems, AppContext, Div, Element, InteractiveElement,
    IntoElement, Model, ParentElement, Path, Render, RenderOnce, Stateful,
    StatefulInteractiveElement, Styled, Subscription, ViewContext, VisualContext, WeakView,
    WindowBounds,
};
use project::{Project, RepositoryEntry};
use std::sync::Arc;
use theme::ActiveTheme;
use ui::{
    h_stack, popover_menu, prelude::*, Avatar, Button, ButtonLike, ButtonStyle, ContextMenu, Icon,
    IconButton, IconElement, KeyBinding, Tooltip,
};
use util::ResultExt;
use workspace::{notifications::NotifyResultExt, Workspace};

const MAX_PROJECT_NAME_LENGTH: usize = 40;
const MAX_BRANCH_NAME_LENGTH: usize = 40;

actions!(
    ShareProject,
    UnshareProject,
    ToggleUserMenu,
    ToggleProjectMenu,
    SwitchBranch
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
    #[allow(unused)] // todo!()
    user_store: Model<UserStore>,
    #[allow(unused)] // todo!()
    client: Arc<Client>,
    #[allow(unused)] // todo!()
    workspace: WeakView<Workspace>,
    //branch_popover: Option<ViewHandle<BranchList>>,
    //project_popover: Option<ViewHandle<recent_projects::RecentProjects>>,
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
                        current_user.clone().zip(room.clone()).zip(project_id),
                        |this, ((current_user, room), project_id)| {
                            let remote_participants = room
                                .read(cx)
                                .remote_participants()
                                .values()
                                .map(|participant| {
                                    (
                                        participant.user.clone(),
                                        participant.participant_index,
                                        participant.peer_id,
                                    )
                                })
                                .collect::<Vec<_>>();

                            this.children(
                                self.render_collaborator(
                                    &current_user,
                                    client.peer_id().expect("todo!()"),
                                    &room,
                                    project_id,
                                    &remote_participants,
                                    cx,
                                )
                                .map(|pile| pile.render(cx)),
                            )
                            .children(
                                remote_participants.iter().filter_map(
                                    |(user, participant_index, peer_id)| {
                                        let peer_id = *peer_id;
                                        let face_pile = self
                                            .render_collaborator(
                                                user,
                                                peer_id,
                                                &room,
                                                project_id,
                                                &remote_participants,
                                                cx,
                                            )?
                                            .render(cx);
                                        Some(
                                            v_stack()
                                                .id(("collaborator", user.id))
                                                .child(face_pile)
                                                .child(render_color_ribbon(*participant_index, cx))
                                                .cursor_pointer()
                                                .on_click(cx.listener(move |this, _, cx| {
                                                    this.workspace
                                                        .update(cx, |workspace, cx| {
                                                            workspace.follow(peer_id, cx);
                                                        })
                                                        .ok();
                                                })),
                                        )
                                    },
                                ),
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
                            this.when_some(user.avatar.clone(), |this, avatar| {
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
                                                        .child(Avatar::data(avatar))
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
                            })
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

fn render_color_ribbon(
    participant_index: ParticipantIndex,
    cx: &mut WindowContext,
) -> gpui::Canvas {
    let color = cx
        .theme()
        .players()
        .color_for_participant(participant_index.0)
        .cursor;
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
            //         project_popover: None,
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

        div().border().border_color(gpui::red()).child(
            Button::new("project_name_trigger", name)
                .style(ButtonStyle::Subtle)
                .tooltip(move |cx| Tooltip::text("Recent Projects", cx)),
        )
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
        room: &Model<Room>,
        project_id: u64,
        collaborators: &[(Arc<User>, ParticipantIndex, PeerId)],
        cx: &mut WindowContext,
    ) -> Option<FacePile> {
        let room = room.read(cx);
        let followers = room.followers_for(peer_id, project_id);

        let mut pile = FacePile::default();
        pile.extend(
            user.avatar
                .clone()
                .map(|avatar| div().child(Avatar::data(avatar.clone())).into_any_element())
                .into_iter()
                .chain(followers.iter().filter_map(|follower_peer_id| {
                    let follower = collaborators
                        .iter()
                        .find(|(_, _, peer_id)| *peer_id == *follower_peer_id)?
                        .0
                        .clone();
                    follower
                        .avatar
                        .clone()
                        .map(|avatar| div().child(Avatar::data(avatar.clone())).into_any_element())
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

    // pub fn toggle_project_menu(&mut self, _: &ToggleProjectMenu, cx: &mut ViewContext<Self>) {
    //     let workspace = self.workspace.clone();
    //     if self.project_popover.take().is_none() {
    //         cx.spawn(|this, mut cx| async move {
    //             let workspaces = WORKSPACE_DB
    //                 .recent_workspaces_on_disk()
    //                 .await
    //                 .unwrap_or_default()
    //                 .into_iter()
    //                 .map(|(_, location)| location)
    //                 .collect();

    //             let workspace = workspace.clone();
    //             this.update(&mut cx, move |this, cx| {
    //                 let view = cx.add_view(|cx| build_recent_projects(workspace, workspaces, cx));

    //                 cx.subscribe(&view, |this, _, event, cx| {
    //                     match event {
    //                         PickerEvent::Dismiss => {
    //                             this.project_popover = None;
    //                         }
    //                     }

    //                     cx.notify();
    //                 })
    //                 .detach();
    //                 cx.focus(&view);
    //                 this.branch_popover.take();
    //                 this.project_popover = Some(view);
    //                 cx.notify();
    //             })
    //             .log_err();
    //         })
    //         .detach();
    //     }
    //     cx.notify();
    // }

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

    // fn render_collaborators(
    //     &self,
    //     workspace: &ViewHandle<Workspace>,
    //     theme: &Theme,
    //     room: &ModelHandle<Room>,
    //     cx: &mut ViewContext<Self>,
    // ) -> Vec<Container<Self>> {
    //     let mut participants = room
    //         .read(cx)
    //         .remote_participants()
    //         .values()
    //         .cloned()
    //         .collect::<Vec<_>>();
    //     participants.sort_by_cached_key(|p| p.user.github_login.clone());

    //     participants
    //         .into_iter()
    //         .filter_map(|participant| {
    //             let project = workspace.read(cx).project().read(cx);
    //             let replica_id = project
    //                 .collaborators()
    //                 .get(&participant.peer_id)
    //                 .map(|collaborator| collaborator.replica_id);
    //             let user = participant.user.clone();
    //             Some(
    //                 Container::new(self.render_face_pile(
    //                     &user,
    //                     replica_id,
    //                     participant.peer_id,
    //                     Some(participant.location),
    //                     participant.muted,
    //                     participant.speaking,
    //                     workspace,
    //                     theme,
    //                     cx,
    //                 ))
    //                 .with_margin_right(theme.titlebar.face_pile_spacing),
    //             )
    //         })
    //         .collect()
    // }

    // fn render_current_user(
    //     &self,
    //     workspace: &ViewHandle<Workspace>,
    //     theme: &Theme,
    //     user: &Arc<User>,
    //     peer_id: PeerId,
    //     muted: bool,
    //     speaking: bool,
    //     cx: &mut ViewContext<Self>,
    // ) -> AnyElement<Self> {
    //     let replica_id = workspace.read(cx).project().read(cx).replica_id();

    //     Container::new(self.render_face_pile(
    //         user,
    //         Some(replica_id),
    //         peer_id,
    //         None,
    //         muted,
    //         speaking,
    //         workspace,
    //         theme,
    //         cx,
    //     ))
    //     .with_margin_right(theme.titlebar.item_spacing)
    //     .into_any()
    // }

    // fn render_face_pile(
    //     &self,
    //     user: &User,
    //     _replica_id: Option<ReplicaId>,
    //     peer_id: PeerId,
    //     location: Option<ParticipantLocation>,
    //     muted: bool,
    //     speaking: bool,
    //     workspace: &ViewHandle<Workspace>,
    //     theme: &Theme,
    //     cx: &mut ViewContext<Self>,
    // ) -> AnyElement<Self> {
    //     let user_id = user.id;
    //     let project_id = workspace.read(cx).project().read(cx).remote_id();
    //     let room = ActiveCall::global(cx).read(cx).room().cloned();
    //     let self_peer_id = workspace.read(cx).client().peer_id();
    //     let self_following = workspace.read(cx).is_being_followed(peer_id);
    //     let self_following_initialized = self_following
    //         && room.as_ref().map_or(false, |room| match project_id {
    //             None => true,
    //             Some(project_id) => room
    //                 .read(cx)
    //                 .followers_for(peer_id, project_id)
    //                 .iter()
    //                 .any(|&follower| Some(follower) == self_peer_id),
    //         });

    //     let leader_style = theme.titlebar.leader_avatar;
    //     let follower_style = theme.titlebar.follower_avatar;

    //     let microphone_state = if muted {
    //         Some(theme.titlebar.muted)
    //     } else if speaking {
    //         Some(theme.titlebar.speaking)
    //     } else {
    //         None
    //     };

    //     let mut background_color = theme
    //         .titlebar
    //         .container
    //         .background_color
    //         .unwrap_or_default();

    //     let participant_index = self
    //         .user_store
    //         .read(cx)
    //         .participant_indices()
    //         .get(&user_id)
    //         .copied();
    //     if let Some(participant_index) = participant_index {
    //         if self_following_initialized {
    //             let selection = theme
    //                 .editor
    //                 .selection_style_for_room_participant(participant_index.0)
    //                 .selection;
    //             background_color = Color::blend(selection, background_color);
    //             background_color.a = 255;
    //         }
    //     }

    //     enum TitlebarParticipant {}

    //     let content = MouseEventHandler::new::<TitlebarParticipant, _>(
    //         peer_id.as_u64() as usize,
    //         cx,
    //         move |_, cx| {
    //             Stack::new()
    //                 .with_children(user.avatar.as_ref().map(|avatar| {
    //                     let face_pile = FacePile::new(theme.titlebar.follower_avatar_overlap)
    //                         .with_child(Self::render_face(
    //                             avatar.clone(),
    //                             Self::location_style(workspace, location, leader_style, cx),
    //                             background_color,
    //                             microphone_state,
    //                         ))
    //                         .with_children(
    //                             (|| {
    //                                 let project_id = project_id?;
    //                                 let room = room?.read(cx);
    //                                 let followers = room.followers_for(peer_id, project_id);
    //                                 Some(followers.into_iter().filter_map(|&follower| {
    //                                     if Some(follower) == self_peer_id {
    //                                         return None;
    //                                     }
    //                                     let participant =
    //                                         room.remote_participant_for_peer_id(follower)?;
    //                                     Some(Self::render_face(
    //                                         participant.user.avatar.clone()?,
    //                                         follower_style,
    //                                         background_color,
    //                                         None,
    //                                     ))
    //                                 }))
    //                             })()
    //                             .into_iter()
    //                             .flatten(),
    //                         )
    //                         .with_children(
    //                             self_following_initialized
    //                                 .then(|| self.user_store.read(cx).current_user())
    //                                 .and_then(|user| {
    //                                     Some(Self::render_face(
    //                                         user?.avatar.clone()?,
    //                                         follower_style,
    //                                         background_color,
    //                                         None,
    //                                     ))
    //                                 }),
    //                         );

    //                     let mut container = face_pile
    //                         .contained()
    //                         .with_style(theme.titlebar.leader_selection);

    //                     if let Some(participant_index) = participant_index {
    //                         if self_following_initialized {
    //                             let color = theme
    //                                 .editor
    //                                 .selection_style_for_room_participant(participant_index.0)
    //                                 .selection;
    //                             container = container.with_background_color(color);
    //                         }
    //                     }

    //                     container
    //                 }))
    //                 .with_children((|| {
    //                     let participant_index = participant_index?;
    //                     let color = theme
    //                         .editor
    //                         .selection_style_for_room_participant(participant_index.0)
    //                         .cursor;
    //                     Some(
    //                         AvatarRibbon::new(color)
    //                             .constrained()
    //                             .with_width(theme.titlebar.avatar_ribbon.width)
    //                             .with_height(theme.titlebar.avatar_ribbon.height)
    //                             .aligned()
    //                             .bottom(),
    //                     )
    //                 })())
    //         },
    //     );

    //     if Some(peer_id) == self_peer_id {
    //         return content.into_any();
    //     }

    //     content
    //         .with_cursor_style(CursorStyle::PointingHand)
    //         .on_click(MouseButton::Left, move |_, this, cx| {
    //             let Some(workspace) = this.workspace.upgrade(cx) else {
    //                 return;
    //             };
    //             if let Some(task) =
    //                 workspace.update(cx, |workspace, cx| workspace.follow(peer_id, cx))
    //             {
    //                 task.detach_and_log_err(cx);
    //             }
    //         })
    //         .with_tooltip::<TitlebarParticipant>(
    //             peer_id.as_u64() as usize,
    //             format!("Follow {}", user.github_login),
    //             Some(Box::new(FollowNextCollaborator)),
    //             theme.tooltip.clone(),
    //             cx,
    //         )
    //         .into_any()
    // }

    // fn location_style(
    //     workspace: &ViewHandle<Workspace>,
    //     location: Option<ParticipantLocation>,
    //     mut style: AvatarStyle,
    //     cx: &ViewContext<Self>,
    // ) -> AvatarStyle {
    //     if let Some(location) = location {
    //         if let ParticipantLocation::SharedProject { project_id } = location {
    //             if Some(project_id) != workspace.read(cx).project().read(cx).remote_id() {
    //                 style.image.grayscale = true;
    //             }
    //         } else {
    //             style.image.grayscale = true;
    //         }
    //     }

    //     style
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
