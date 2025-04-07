use std::sync::Arc;

use call::{ActiveCall, ParticipantLocation, Room};
use client::{User, proto::PeerId};
use gpui::{AnyElement, Hsla, IntoElement, MouseButton, Path, Styled, canvas, point};
use gpui::{App, Task, Window, actions};
use rpc::proto::{self};
use theme::ActiveTheme;
use ui::{Avatar, AvatarAudioStatusIndicator, Facepile, TintColor, Tooltip, prelude::*};
use workspace::notifications::DetachAndPromptErr;

use crate::TitleBar;

actions!(
    collab,
    [ToggleScreenSharing, ToggleMute, ToggleDeafen, LeaveCall]
);

fn toggle_screen_sharing(_: &ToggleScreenSharing, window: &mut Window, cx: &mut App) {
    let call = ActiveCall::global(cx).read(cx);
    if let Some(room) = call.room().cloned() {
        let toggle_screen_sharing = room.update(cx, |room, cx| {
            if room.is_screen_sharing() {
                telemetry::event!(
                    "Screen Share Disabled",
                    room_id = room.id(),
                    channel_id = room.channel_id(),
                );
                Task::ready(room.unshare_screen(cx))
            } else {
                telemetry::event!(
                    "Screen Share Enabled",
                    room_id = room.id(),
                    channel_id = room.channel_id(),
                );
                room.share_screen(cx)
            }
        });
        toggle_screen_sharing.detach_and_prompt_err("Sharing Screen Failed", window, cx, |e, _, _| Some(format!("{:?}\n\nPlease check that you have given Zed permissions to record your screen in Settings.", e)));
    }
}

fn toggle_mute(_: &ToggleMute, cx: &mut App) {
    let call = ActiveCall::global(cx).read(cx);
    if let Some(room) = call.room().cloned() {
        room.update(cx, |room, cx| {
            let operation = if room.is_muted() {
                "Microphone Enabled"
            } else {
                "Microphone Disabled"
            };
            telemetry::event!(
                operation,
                room_id = room.id(),
                channel_id = room.channel_id(),
            );

            room.toggle_mute(cx)
        });
    }
}

fn toggle_deafen(_: &ToggleDeafen, cx: &mut App) {
    if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
        room.update(cx, |room, cx| room.toggle_deafen(cx));
    }
}

fn render_color_ribbon(color: Hsla) -> impl Element {
    canvas(
        move |_, _, _| {},
        move |bounds, _, window, _| {
            let height = bounds.size.height;
            let horizontal_offset = height;
            let vertical_offset = px(height.0 / 2.0);
            let mut path = Path::new(bounds.bottom_left());
            path.curve_to(
                bounds.origin + point(horizontal_offset, vertical_offset),
                bounds.origin + point(px(0.0), vertical_offset),
            );
            path.line_to(bounds.top_right() + point(-horizontal_offset, vertical_offset));
            path.curve_to(
                bounds.bottom_right(),
                bounds.top_right() + point(px(0.0), vertical_offset),
            );
            path.line_to(bounds.bottom_left());
            window.paint_path(path, color);
        },
    )
    .h_1()
    .w_full()
}

impl TitleBar {
    pub(crate) fn render_collaborator_list(
        &self,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let room = ActiveCall::global(cx).read(cx).room().cloned();
        let current_user = self.user_store.read(cx).current_user();
        let client = self.client.clone();
        let project_id = self.project.read(cx).remote_id();
        let workspace = self.workspace.upgrade();

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
                        room,
                        project_id,
                        &current_user,
                        cx,
                    );

                    this.children(current_user_face_pile.map(|face_pile| {
                        v_flex()
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .child(face_pile)
                            .child(render_color_ribbon(player_colors.local().cursor))
                    }))
                    .children(remote_participants.iter().filter_map(|collaborator| {
                        let player_color =
                            player_colors.color_for_participant(collaborator.participant_index.0);
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
                            room,
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
                                    cx.listener(move |this, _, window, cx| {
                                        this.workspace
                                            .update(cx, |workspace, cx| {
                                                if is_following {
                                                    workspace.unfollow(peer_id, window, cx);
                                                } else {
                                                    workspace.follow(peer_id, window, cx);
                                                }
                                            })
                                            .ok();
                                    })
                                })
                                .tooltip({
                                    let login = collaborator.user.github_login.clone();
                                    Tooltip::text(format!("Follow {login}"))
                                }),
                        )
                    }))
                },
            )
    }

    fn render_collaborator(
        &self,
        user: &Arc<User>,
        peer_id: PeerId,
        is_present: bool,
        is_speaking: bool,
        is_muted: bool,
        leader_selection_color: Option<Hsla>,
        room: &Room,
        project_id: Option<u64>,
        current_user: &Arc<User>,
        cx: &App,
    ) -> Option<Div> {
        if room.role_for_user(user.id) == Some(proto::ChannelRole::Guest) {
            return None;
        }

        const FACEPILE_LIMIT: usize = 3;
        let followers = project_id.map_or(&[] as &[_], |id| room.followers_for(peer_id, id));
        let extra_count = followers.len().saturating_sub(FACEPILE_LIMIT);

        Some(
            div()
                .m_0p5()
                .p_0p5()
                // When the collaborator is not followed, still draw this wrapper div, but leave
                // it transparent, so that it does not shift the layout when following.
                .when_some(leader_selection_color, |div, color| {
                    div.rounded_sm().bg(color)
                })
                .child(
                    Facepile::empty()
                        .child(
                            Avatar::new(user.avatar_uri.clone())
                                .grayscale(!is_present)
                                .border_color(if is_speaking {
                                    cx.theme().status().info
                                } else {
                                    // We draw the border in a transparent color rather to avoid
                                    // the layout shift that would come with adding/removing the border.
                                    gpui::transparent_black()
                                })
                                .when(is_muted, |avatar| {
                                    avatar.indicator(
                                        AvatarAudioStatusIndicator::new(ui::AudioStatus::Muted)
                                            .tooltip({
                                                let github_login = user.github_login.clone();
                                                Tooltip::text(format!("{} is muted", github_login))
                                            }),
                                    )
                                }),
                        )
                        .children(followers.iter().take(FACEPILE_LIMIT).filter_map(
                            |follower_peer_id| {
                                let follower = room
                                    .remote_participants()
                                    .values()
                                    .find_map(|p| {
                                        (p.peer_id == *follower_peer_id).then_some(&p.user)
                                    })
                                    .or_else(|| {
                                        (self.client.peer_id() == Some(*follower_peer_id))
                                            .then_some(current_user)
                                    })?
                                    .clone();

                                Some(div().mt(-px(4.)).child(
                                    Avatar::new(follower.avatar_uri.clone()).size(rems(0.75)),
                                ))
                            },
                        ))
                        .children(if extra_count > 0 {
                            Some(
                                Label::new(format!("+{extra_count}"))
                                    .ml_1()
                                    .into_any_element(),
                            )
                        } else {
                            None
                        }),
                ),
        )
    }

    pub(crate) fn render_call_controls(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<AnyElement> {
        let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() else {
            return Vec::new();
        };

        let is_connecting_to_project = self
            .workspace
            .update(cx, |workspace, cx| workspace.has_active_modal(window, cx))
            .unwrap_or(false);

        let room = room.read(cx);
        let project = self.project.read(cx);
        let is_local = project.is_local() || project.is_via_ssh();
        let is_shared = is_local && project.is_shared();
        let is_muted = room.is_muted();
        let muted_by_user = room.muted_by_user();
        let is_deafened = room.is_deafened().unwrap_or(false);
        let is_screen_sharing = room.is_screen_sharing();
        let can_use_microphone = room.can_use_microphone();
        let can_share_projects = room.can_share_projects();
        let screen_sharing_supported = cx.is_screen_capture_supported();

        let mut children = Vec::new();

        if is_local && can_share_projects && !is_connecting_to_project {
            children.push(
                Button::new(
                    "toggle_sharing",
                    if is_shared { "Unshare" } else { "Share" },
                )
                .tooltip(Tooltip::text(if is_shared {
                    "Stop sharing project with call participants"
                } else {
                    "Share project with call participants"
                }))
                .style(ButtonStyle::Subtle)
                .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                .toggle_state(is_shared)
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, window, cx| {
                    if is_shared {
                        this.unshare_project(window, cx);
                    } else {
                        this.share_project(cx);
                    }
                }))
                .into_any_element(),
            );
        }

        children.push(
            div()
                .pr_2()
                .child(
                    IconButton::new("leave-call", ui::IconName::Exit)
                        .style(ButtonStyle::Subtle)
                        .tooltip(Tooltip::text("Leave call"))
                        .icon_size(IconSize::Small)
                        .on_click(move |_, _window, cx| {
                            ActiveCall::global(cx)
                                .update(cx, |call, cx| call.hang_up(cx))
                                .detach_and_log_err(cx);
                        }),
                )
                .into_any_element(),
        );

        if can_use_microphone {
            children.push(
                IconButton::new(
                    "mute-microphone",
                    if is_muted {
                        ui::IconName::MicMute
                    } else {
                        ui::IconName::Mic
                    },
                )
                .tooltip(move |window, cx| {
                    if is_muted {
                        if is_deafened {
                            Tooltip::with_meta(
                                "Unmute Microphone",
                                None,
                                "Audio will be unmuted",
                                window,
                                cx,
                            )
                        } else {
                            Tooltip::simple("Unmute Microphone", cx)
                        }
                    } else {
                        Tooltip::simple("Mute Microphone", cx)
                    }
                })
                .style(ButtonStyle::Subtle)
                .icon_size(IconSize::Small)
                .toggle_state(is_muted)
                .selected_style(ButtonStyle::Tinted(TintColor::Error))
                .on_click(move |_, _window, cx| {
                    toggle_mute(&Default::default(), cx);
                })
                .into_any_element(),
            );
        }

        children.push(
            IconButton::new(
                "mute-sound",
                if is_deafened {
                    ui::IconName::AudioOff
                } else {
                    ui::IconName::AudioOn
                },
            )
            .style(ButtonStyle::Subtle)
            .selected_style(ButtonStyle::Tinted(TintColor::Error))
            .icon_size(IconSize::Small)
            .toggle_state(is_deafened)
            .tooltip(move |window, cx| {
                if is_deafened {
                    let label = "Unmute Audio";

                    if !muted_by_user {
                        Tooltip::with_meta(label, None, "Microphone will be unmuted", window, cx)
                    } else {
                        Tooltip::simple(label, cx)
                    }
                } else {
                    let label = "Mute Audio";

                    if !muted_by_user {
                        Tooltip::with_meta(label, None, "Microphone will be muted", window, cx)
                    } else {
                        Tooltip::simple(label, cx)
                    }
                }
            })
            .on_click(move |_, _, cx| toggle_deafen(&Default::default(), cx))
            .into_any_element(),
        );

        if can_use_microphone && screen_sharing_supported {
            children.push(
                IconButton::new("screen-share", ui::IconName::Screen)
                    .style(ButtonStyle::Subtle)
                    .icon_size(IconSize::Small)
                    .toggle_state(is_screen_sharing)
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                    .tooltip(Tooltip::text(if is_screen_sharing {
                        "Stop Sharing Screen"
                    } else {
                        "Share Screen"
                    }))
                    .on_click(move |_, window, cx| {
                        toggle_screen_sharing(&Default::default(), window, cx)
                    })
                    .into_any_element(),
            );
        }

        children.push(div().pr_2().into_any_element());

        children
    }
}
