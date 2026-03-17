use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use call::{ActiveCall, Room};
use channel::ChannelStore;
use client::{User, proto::PeerId};
use gpui::{
    AnyElement, Hsla, IntoElement, MouseButton, Path, ScreenCaptureSource, Styled, WeakEntity,
    canvas, point,
};
use gpui::{App, Task, Window};
use icons::IconName;
use livekit_client::ConnectionQuality;
use project::WorktreeSettings;
use rpc::proto::{self};
use settings::{Settings as _, SettingsLocation};
use theme::ActiveTheme;
use ui::{
    Avatar, AvatarAudioStatusIndicator, ContextMenu, ContextMenuItem, Divider, DividerColor,
    Facepile, PopoverMenu, SplitButton, SplitButtonStyle, TintColor, Tooltip, prelude::*,
};
use util::rel_path::RelPath;
use workspace::{ParticipantLocation, notifications::DetachAndPromptErr};
use zed_actions::ShowCallStats;

use crate::TitleBar;

#[derive(Clone, Default)]
pub struct CallStats {
    pub latency_ms: Option<f64>,
    pub jitter_ms: Option<f64>,
    pub packet_loss_pct: Option<f64>,
    pub input_lag: Option<Duration>,
}

impl TitleBar {
    pub(crate) fn start_call_stats_polling(&mut self, cx: &mut gpui::Context<Self>) {
        self.call_stats_poll_task = Some(cx.spawn(async move |this, cx| {
            loop {
                if this
                    .update(cx, |this, cx| this.poll_call_stats(cx))
                    .is_err()
                {
                    break;
                }
                cx.background_executor().timer(Duration::from_secs(1)).await;
            }
        }));
    }

    pub(crate) fn stop_call_stats_polling(&mut self) {
        self.call_stats_poll_task.take();
        self.call_stats = CallStats::default();
    }

    fn poll_call_stats(&mut self, cx: &mut gpui::Context<Self>) {
        let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() else {
            return;
        };

        self.call_stats.input_lag = room.read(cx).input_lag();
        let stats_future = room.read(cx).get_stats();

        let background_task = cx.background_executor().spawn(async move {
            let session_stats = stats_future.await;
            session_stats.map(|stats| compute_network_stats(&stats))
        });

        cx.spawn(async move |this, cx| {
            let result = background_task.await;
            this.update(cx, |this, cx| {
                if let Some(computed) = result {
                    this.call_stats.latency_ms = computed.latency_ms;
                    this.call_stats.jitter_ms = computed.jitter_ms;
                    this.call_stats.packet_loss_pct = computed.packet_loss_pct;
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }
}

struct ComputedNetworkStats {
    latency_ms: Option<f64>,
    jitter_ms: Option<f64>,
    packet_loss_pct: Option<f64>,
}

fn compute_network_stats(stats: &livekit_client::SessionStats) -> ComputedNetworkStats {
    let mut best_rtt: Option<f64> = None;
    let mut best_jitter: Option<f64> = None;
    let mut total_packets_received: u64 = 0;
    let mut total_packets_lost: i64 = 0;

    let all_stats = stats
        .publisher_stats
        .iter()
        .chain(stats.subscriber_stats.iter());

    for stat in all_stats {
        extract_metrics(
            stat,
            &mut best_rtt,
            &mut best_jitter,
            &mut total_packets_received,
            &mut total_packets_lost,
        );
    }

    let total_expected = total_packets_received as i64 + total_packets_lost;
    let packet_loss_pct = if total_expected > 0 {
        Some((total_packets_lost as f64 / total_expected as f64) * 100.0)
    } else {
        None
    };

    ComputedNetworkStats {
        latency_ms: best_rtt.map(|rtt| rtt * 1000.0),
        jitter_ms: best_jitter.map(|j| j * 1000.0),
        packet_loss_pct,
    }
}

#[cfg(all(
    not(rust_analyzer),
    any(
        test,
        feature = "test-support",
        all(target_os = "windows", target_env = "gnu"),
        target_os = "freebsd"
    )
))]
fn extract_metrics(
    _stat: &livekit_client::RtcStats,
    _best_rtt: &mut Option<f64>,
    _best_jitter: &mut Option<f64>,
    _total_packets_received: &mut u64,
    _total_packets_lost: &mut i64,
) {
}

#[cfg(any(
    rust_analyzer,
    not(any(
        test,
        feature = "test-support",
        all(target_os = "windows", target_env = "gnu"),
        target_os = "freebsd"
    ))
))]
fn extract_metrics(
    stat: &livekit_client::RtcStats,
    best_rtt: &mut Option<f64>,
    best_jitter: &mut Option<f64>,
    total_packets_received: &mut u64,
    total_packets_lost: &mut i64,
) {
    use livekit_client::RtcStats;

    match stat {
        RtcStats::CandidatePair(pair) => {
            let rtt = pair.candidate_pair.current_round_trip_time;
            if rtt > 0.0 {
                *best_rtt = Some(match *best_rtt {
                    Some(current) => current.min(rtt),
                    None => rtt,
                });
            }
        }
        RtcStats::InboundRtp(inbound) => {
            let jitter = inbound.received.jitter;
            if jitter > 0.0 {
                *best_jitter = Some(match *best_jitter {
                    Some(current) => current.max(jitter),
                    None => jitter,
                });
            }
            *total_packets_received += inbound.received.packets_received;
            *total_packets_lost += inbound.received.packets_lost;
        }
        RtcStats::RemoteInboundRtp(remote_inbound) => {
            let rtt = remote_inbound.remote_inbound.round_trip_time;
            if rtt > 0.0 {
                *best_rtt = Some(match *best_rtt {
                    Some(current) => current.min(rtt),
                    None => rtt,
                });
            }
        }
        _ => {}
    }
}

fn metric_quality(value: f64, warn_threshold: f64, error_threshold: f64) -> ConnectionQuality {
    if value < warn_threshold {
        ConnectionQuality::Excellent
    } else if value < error_threshold {
        ConnectionQuality::Poor
    } else {
        ConnectionQuality::Lost
    }
}

/// Computes the effective connection quality by taking the worst of the
/// LiveKit-reported quality and each individual metric rating.
fn effective_connection_quality(
    livekit_quality: ConnectionQuality,
    stats: &CallStats,
) -> ConnectionQuality {
    let mut worst = livekit_quality;

    if let Some(latency) = stats.latency_ms {
        worst = worst.max(metric_quality(latency, 100.0, 300.0));
    }
    if let Some(jitter) = stats.jitter_ms {
        worst = worst.max(metric_quality(jitter, 30.0, 75.0));
    }
    if let Some(loss) = stats.packet_loss_pct {
        worst = worst.max(metric_quality(loss, 1.0, 5.0));
    }
    if let Some(lag) = stats.input_lag {
        let lag_ms = lag.as_secs_f64() * 1000.0;
        worst = worst.max(metric_quality(lag_ms, 20.0, 50.0));
    }

    worst
}

fn format_stat(value: Option<f64>, format: impl Fn(f64) -> String) -> String {
    match value {
        Some(v) => format(v),
        None => "—".to_string(),
    }
}

pub fn toggle_screen_sharing(
    screen: anyhow::Result<Option<Rc<dyn ScreenCaptureSource>>>,
    window: &mut Window,
    cx: &mut App,
) {
    let call = ActiveCall::global(cx).read(cx);
    let toggle_screen_sharing = match screen {
        Ok(screen) => {
            let Some(room) = call.room().cloned() else {
                return;
            };

            room.update(cx, |room, cx| {
                let clicked_on_currently_shared_screen =
                    room.shared_screen_id().is_some_and(|screen_id| {
                        Some(screen_id)
                            == screen
                                .as_deref()
                                .and_then(|s| s.metadata().ok().map(|meta| meta.id))
                    });
                let should_unshare_current_screen = room.is_sharing_screen();
                let unshared_current_screen = should_unshare_current_screen.then(|| {
                    telemetry::event!(
                        "Screen Share Disabled",
                        room_id = room.id(),
                        channel_id = room.channel_id(),
                    );
                    room.unshare_screen(clicked_on_currently_shared_screen || screen.is_none(), cx)
                });
                if let Some(screen) = screen {
                    if !should_unshare_current_screen {
                        telemetry::event!(
                            "Screen Share Enabled",
                            room_id = room.id(),
                            channel_id = room.channel_id(),
                        );
                    }
                    cx.spawn(async move |room, cx| {
                        unshared_current_screen.transpose()?;
                        if !clicked_on_currently_shared_screen {
                            room.update(cx, |room, cx| room.share_screen(screen, cx))?
                                .await
                        } else {
                            Ok(())
                        }
                    })
                } else {
                    Task::ready(Ok(()))
                }
            })
        }
        Err(e) => Task::ready(Err(e)),
    };
    toggle_screen_sharing.detach_and_prompt_err("Sharing Screen Failed", window, cx, |e, _, _| Some(format!("{:?}\n\nPlease check that you have given Zed permissions to record your screen in Settings.", e)));
}

pub fn toggle_mute(cx: &mut App) {
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

pub fn toggle_deafen(cx: &mut App) {
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
            let vertical_offset = height / 2.0;
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
                current_user.zip(client.peer_id()).zip(room),
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
                            .on_mouse_down(MouseButton::Left, |_, window, _| {
                                window.prevent_default()
                            })
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
                        let is_present = project_id.is_some_and(|project_id| {
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
                                .on_mouse_down(MouseButton::Left, |_, window, _| {
                                    window.prevent_default()
                                })
                                .on_click({
                                    let peer_id = collaborator.peer_id;
                                    cx.listener(move |this, _, window, cx| {
                                        cx.stop_propagation();

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
                                .occlude()
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
        let is_local = project.is_local() || project.is_via_remote_server();
        let is_shared = is_local && project.is_shared();
        let is_muted = room.is_muted();
        let muted_by_user = room.muted_by_user();
        let is_deafened = room.is_deafened().unwrap_or(false);
        let is_screen_sharing = room.is_sharing_screen();
        let can_use_microphone = room.can_use_microphone();
        let can_share_projects = room.can_share_projects();
        let screen_sharing_supported = cx.is_screen_capture_supported();
        let connection_quality = room.connection_quality();

        let channel_store = ChannelStore::global(cx);
        let channel = room
            .channel_id()
            .and_then(|channel_id| channel_store.read(cx).channel_for_id(channel_id).cloned());

        let mut children = Vec::new();

        let effective_quality = effective_connection_quality(connection_quality, &self.call_stats);
        let (signal_icon, signal_color, quality_label) = match effective_quality {
            ConnectionQuality::Excellent => {
                (IconName::SignalHigh, Some(Color::Success), "Excellent")
            }
            ConnectionQuality::Good => (IconName::SignalHigh, None, "Good"),
            ConnectionQuality::Poor => (IconName::SignalMedium, Some(Color::Warning), "Poor"),
            ConnectionQuality::Lost => (IconName::SignalLow, Some(Color::Error), "Lost"),
        };

        let stats = self.call_stats.clone();
        let quality_label: SharedString = quality_label.into();
        children.push(
            IconButton::new("call-quality", signal_icon)
                .style(ButtonStyle::Subtle)
                .icon_size(IconSize::Small)
                .when_some(signal_color, |button, color| button.icon_color(color))
                .tooltip(move |_window, cx| {
                    let quality_label = quality_label.clone();
                    let latency = format_stat(stats.latency_ms, |v| format!("{:.0}ms", v));
                    let jitter = format_stat(stats.jitter_ms, |v| format!("{:.0}ms", v));
                    let packet_loss = format_stat(stats.packet_loss_pct, |v| format!("{:.1}%", v));
                    let input_lag =
                        format_stat(stats.input_lag.map(|d| d.as_secs_f64() * 1000.0), |v| {
                            format!("{:.1}ms", v)
                        });

                    Tooltip::with_meta(
                        format!("Connection: {quality_label}"),
                        Some(&ShowCallStats),
                        format!(
                            "Latency: {latency} · Jitter: {jitter} · Loss: {packet_loss} · Input lag: {input_lag}",
                        ),
                        cx,
                    )
                })
                .on_click(move |_, window, cx| {
                    window.dispatch_action(Box::new(ShowCallStats), cx);
                })
                .into_any_element(),
        );
        children.push(
            h_flex()
                .gap_1()
                .child(
                    IconButton::new("leave-call", IconName::Exit)
                        .style(ButtonStyle::Subtle)
                        .tooltip(Tooltip::text("Leave Call"))
                        .icon_size(IconSize::Small)
                        .on_click(move |_, _window, cx| {
                            ActiveCall::global(cx)
                                .update(cx, |call, cx| call.hang_up(cx))
                                .detach_and_log_err(cx);
                        }),
                )
                .child(Divider::vertical().color(DividerColor::Border))
                .into_any_element(),
        );

        if is_local && can_share_projects && !is_connecting_to_project {
            let is_sharing_disabled = channel.is_some_and(|channel| match channel.visibility {
                proto::ChannelVisibility::Public => project.visible_worktrees(cx).any(|worktree| {
                    let worktree_id = worktree.read(cx).id();

                    let settings_location = Some(SettingsLocation {
                        worktree_id,
                        path: RelPath::empty(),
                    });

                    WorktreeSettings::get(settings_location, cx).prevent_sharing_in_public_channels
                }),
                proto::ChannelVisibility::Members => false,
            });

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
                .when(is_sharing_disabled, |parent| {
                    parent.disabled(true).tooltip(Tooltip::text(
                        "This project may not be shared in a public channel.",
                    ))
                })
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

        if can_use_microphone {
            children.push(
                IconButton::new(
                    "mute-microphone",
                    if is_muted {
                        IconName::MicMute
                    } else {
                        IconName::Mic
                    },
                )
                .tooltip(move |_window, cx| {
                    if is_muted {
                        if is_deafened {
                            Tooltip::with_meta(
                                "Unmute Microphone",
                                None,
                                "Audio will be unmuted",
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
                .on_click(move |_, _window, cx| toggle_mute(cx))
                .into_any_element(),
            );
        }

        children.push(
            IconButton::new(
                "mute-sound",
                if is_deafened {
                    IconName::AudioOff
                } else {
                    IconName::AudioOn
                },
            )
            .style(ButtonStyle::Subtle)
            .selected_style(ButtonStyle::Tinted(TintColor::Error))
            .icon_size(IconSize::Small)
            .toggle_state(is_deafened)
            .tooltip(move |_window, cx| {
                if is_deafened {
                    let label = "Unmute Audio";

                    if !muted_by_user {
                        Tooltip::with_meta(label, None, "Microphone will be unmuted", cx)
                    } else {
                        Tooltip::simple(label, cx)
                    }
                } else {
                    let label = "Mute Audio";

                    if !muted_by_user {
                        Tooltip::with_meta(label, None, "Microphone will be muted", cx)
                    } else {
                        Tooltip::simple(label, cx)
                    }
                }
            })
            .on_click(move |_, _, cx| toggle_deafen(cx))
            .into_any_element(),
        );

        if can_use_microphone && screen_sharing_supported {
            let trigger = IconButton::new("screen-share", IconName::Screen)
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
                    let should_share = ActiveCall::global(cx)
                        .read(cx)
                        .room()
                        .is_some_and(|room| !room.read(cx).is_sharing_screen());

                    window
                        .spawn(cx, async move |cx| {
                            let screen = if should_share {
                                cx.update(|_, cx| pick_default_screen(cx))?.await
                            } else {
                                Ok(None)
                            };
                            cx.update(|window, cx| toggle_screen_sharing(screen, window, cx))?;

                            Result::<_, anyhow::Error>::Ok(())
                        })
                        .detach();
                });

            children.push(
                SplitButton::new(
                    trigger.render(window, cx),
                    self.render_screen_list().into_any_element(),
                )
                .style(SplitButtonStyle::Transparent)
                .into_any_element(),
            );
        }

        children.push(div().pr_2().into_any_element());

        children
    }

    fn render_screen_list(&self) -> impl IntoElement {
        PopoverMenu::new("screen-share-screen-list")
            .with_handle(self.screen_share_popover_handle.clone())
            .trigger(
                ui::ButtonLike::new_rounded_right("screen-share-screen-list-trigger")
                    .child(
                        h_flex()
                            .mx_neg_0p5()
                            .h_full()
                            .justify_center()
                            .child(Icon::new(IconName::ChevronDown).size(IconSize::XSmall)),
                    )
                    .toggle_state(self.screen_share_popover_handle.is_deployed()),
            )
            .menu(|window, cx| {
                let screens = cx.screen_capture_sources();
                Some(ContextMenu::build(window, cx, |context_menu, _, cx| {
                    cx.spawn(async move |this: WeakEntity<ContextMenu>, cx| {
                        let screens = screens.await??;
                        this.update(cx, |this, cx| {
                            let active_screenshare_id = ActiveCall::global(cx)
                                .read(cx)
                                .room()
                                .and_then(|room| room.read(cx).shared_screen_id());
                            for screen in screens {
                                let Ok(meta) = screen.metadata() else {
                                    continue;
                                };

                                let label = meta
                                    .label
                                    .clone()
                                    .unwrap_or_else(|| SharedString::from("Unknown screen"));
                                let resolution = SharedString::from(format!(
                                    "{} × {}",
                                    meta.resolution.width.0, meta.resolution.height.0
                                ));
                                this.push_item(ContextMenuItem::CustomEntry {
                                    entry_render: Box::new(move |_, _| {
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                Icon::new(IconName::Screen)
                                                    .size(IconSize::XSmall)
                                                    .map(|this| {
                                                        if active_screenshare_id == Some(meta.id) {
                                                            this.color(Color::Accent)
                                                        } else {
                                                            this.color(Color::Muted)
                                                        }
                                                    }),
                                            )
                                            .child(Label::new(label.clone()))
                                            .child(
                                                Label::new(resolution.clone())
                                                    .color(Color::Muted)
                                                    .size(LabelSize::Small),
                                            )
                                            .into_any()
                                    }),
                                    selectable: true,
                                    documentation_aside: None,
                                    handler: Rc::new(move |_, window, cx| {
                                        toggle_screen_sharing(Ok(Some(screen.clone())), window, cx);
                                    }),
                                });
                            }
                        })
                    })
                    .detach_and_log_err(cx);
                    context_menu
                }))
            })
    }
}

/// Picks the screen to share when clicking on the main screen sharing button.
fn pick_default_screen(cx: &App) -> Task<anyhow::Result<Option<Rc<dyn ScreenCaptureSource>>>> {
    let source = cx.screen_capture_sources();
    cx.spawn(async move |_| {
        let available_sources = source.await??;
        Ok(available_sources
            .iter()
            .find(|it| {
                it.as_ref()
                    .metadata()
                    .is_ok_and(|meta| meta.is_main.unwrap_or_default())
            })
            .or_else(|| available_sources.first())
            .cloned())
    })
}
