use gpui::{div, prelude::*, px, rems, Div, SharedString, Stateful, WindowContext};
use theme::ActiveTheme;
use ui::Shape;

#[derive(IntoElement)]
pub struct Titlebar {
    full_screen: bool,
}

// pub struct TitlebarCall {
//     current_user: TitlebarUser,
// }

// pub struct TitlebarUser {}

impl RenderOnce for Titlebar {
    type Rendered = Stateful<Div>;

    fn render(self, cx: &mut ui::prelude::WindowContext) -> Self::Rendered {
        div()
            .flex()
            .flex_col()
            .id("titlebar")
            .justify_between()
            .w_full()
            .h(rems(1.75))
            // Set a non-scaling min-height here to ensure the titlebar is
            // always at least the height of the traffic lights.
            .min_h(px(32.))
            .pl_2()
            .when(self.full_screen, |this| {
                // Use pixels here instead of a rem-based size because the macOS traffic
                // lights are a static size, and don't scale with the rest of the UI.
                this.pl(px(80.))
            })
            .bg(cx.theme().colors().title_bar_background)
            .on_click(|event, cx| {
                if event.up.click_count == 2 {
                    cx.zoom_window();
                }
            })
            // left side
            .child(
                div()
                    .flex()
                    .flex_row()
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
                div()
                    .flex()
                    .flex_row()
                    .gap_1()
                    .pr_1()
                    .when_some(room, |this, room| {
                        let room = room.read(cx);
                        let project = self.project.read(cx);
                        let is_local = project.is_local();
                        let is_shared = is_local && project.is_shared();
                        let is_muted = room.is_muted(cx);
                        let is_deafened = room.is_deafened().unwrap_or(false);
                        let is_screen_sharing = room.is_screen_sharing();

                        this.when(is_local, |this| {
                            this.child(
                                Button::new(
                                    "toggle_sharing",
                                    if is_shared { "Unshare" } else { "Share" },
                                )
                                .style(ButtonStyle::Subtle)
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
                        })
                        .child(
                            IconButton::new("leave-call", ui::Icon::Exit)
                                .style(ButtonStyle::Subtle)
                                .icon_size(IconSize::Small)
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
                            .icon_size(IconSize::Small)
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
                            .icon_size(IconSize::Small)
                            .selected(is_deafened)
                            .tooltip(move |cx| {
                                Tooltip::with_meta("Deafen Audio", None, "Mic will be muted", cx)
                            })
                            .on_click(move |_, cx| crate::toggle_mute(&Default::default(), cx)),
                        )
                        .child(
                            IconButton::new("screen-share", ui::Icon::Screen)
                                .style(ButtonStyle::Subtle)
                                .icon_size(IconSize::Small)
                                .selected(is_screen_sharing)
                                .on_click(move |_, cx| {
                                    crate::toggle_screen_sharing(&Default::default(), cx)
                                }),
                        )
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
    }
}

impl Titlebar {
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
                    .label_size(LabelSize::Small)
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
        let workspace = self.workspace.clone();
        popover_menu("project_name_trigger")
            .trigger(
                Button::new("project_name_trigger", name)
                    .style(ButtonStyle::Subtle)
                    .label_size(LabelSize::Small)
                    .tooltip(move |cx| Tooltip::text("Recent Projects", cx)),
            )
            .menu(move |cx| Some(Self::render_project_popover(workspace.clone(), cx)))
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
        let workspace = self.workspace.upgrade()?;
        let branch_name = entry
            .as_ref()
            .and_then(RepositoryEntry::branch)
            .map(|branch| util::truncate_and_trailoff(&branch, MAX_BRANCH_NAME_LENGTH))?;
        Some(
            popover_menu("project_branch_trigger")
                .trigger(
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
                        }),
                )
                .menu(move |cx| Self::render_vcs_popover(workspace.clone(), cx)),
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

        let pile = FacePile::default()
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
                        (self.client.peer_id() == Some(*follower_peer_id)).then_some(current_user)
                    })?
                    .clone();

                Some(Avatar::new(follower.avatar_uri.clone()))
            }));

        Some(pile)
    }
}

pub struct TitlebarParticipant {
    avatar_uri: SharedString,
    follower_avatar_uris: Vec<SharedString>,
}

pub struct Facepile {}

#[derive(Default, IntoElement)]
pub struct FacePile {
    pub faces: Vec<Avatar>,
}

impl RenderOnce for FacePile {
    type Rendered = Div;

    fn render(self, _: &mut WindowContext) -> Self::Rendered {
        let face_count = self.faces.len();
        div()
            .p_1()
            .flex()
            .items_center()
            .children(self.faces.into_iter().enumerate().map(|(ix, avatar)| {
                let last_child = ix == face_count - 1;
                div()
                    .z_index((face_count - ix) as u8)
                    .when(!last_child, |div| div.neg_mr_1())
                    .child(avatar)
            }))
    }
}

#[derive(IntoElement)]
pub struct Avatar {
    pub image_uri: SharedString,
    pub audio_status: AudioStatus,
    pub shape: AvatarShape,
}

pub enum AvatarShape {
    Square,
    Circle,
}

impl RenderOnce for Avatar {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        div()
            .map(|this| match self.shape {
                AvatarShape::Square => this.rounded_md(),
                AvatarShape::Circle => this.rounded_full(),
            })
            .map(|this| match self.audio_status {
                AudioStatus::None => this,
                AudioStatus::Muted => todo!(),
                AudioStatus::Speaking => todo!(),
            })
            .size(cx.rem_size() + px(2.));

        let size = cx.rem_size();

        div()
            .when_some(self.border_color, |this, color| {
                this.border().border_color(color)
            })
            .child(
                self.image
                    .size(size)
                    // todo!(Pull the avatar fallback background from the theme.)
                    .bg(gpui::red()),
            )
            .children(self.is_available.map(|is_free| {
                // HACK: non-integer sizes result in oval indicators.
                let indicator_size = (size * 0.4).round();

                div()
                    .absolute()
                    .z_index(1)
                    .bg(if is_free {
                        cx.theme().status().created
                    } else {
                        cx.theme().status().deleted
                    })
                    .size(indicator_size)
                    .rounded(indicator_size)
                    .bottom_0()
                    .right_0()
            }))
    }
}

pub enum AudioStatus {
    None,
    Muted,
    Speaking,
}

impl RenderOnce for Avatar {
    type Rendered = Div;

    fn render(self, cx: &mut WindowContext) -> Self::Rendered {
        todo!()
    }
}
