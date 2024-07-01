use crate::TitleBar;
use call::Room;
use client::{proto::PeerId, User};
use gpui::{canvas, point, Hsla, IntoElement, Path, Styled};
use rpc::proto::{self};
use std::sync::Arc;
use theme::ActiveTheme;
use ui::{prelude::*, Avatar, AvatarAudioStatusIndicator, Facepile, Tooltip};

pub(crate) fn render_color_ribbon(color: Hsla) -> impl Element {
    canvas(
        move |_, _| {},
        move |bounds, _, cx| {
            let height = bounds.size.height;
            let horizontal_offset = height;
            let vertical_offset = px(height.0 / 2.0);
            let mut path = Path::new(bounds.lower_left());
            path.curve_to(
                bounds.origin + point(horizontal_offset, vertical_offset),
                bounds.origin + point(px(0.0), vertical_offset),
            );
            path.line_to(bounds.upper_right() + point(-horizontal_offset, vertical_offset));
            path.curve_to(
                bounds.lower_right(),
                bounds.upper_right() + point(px(0.0), vertical_offset),
            );
            path.line_to(bounds.lower_left());
            cx.paint_path(path, color);
        },
    )
    .h_1()
    .w_full()
}

impl TitleBar {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn render_collaborator(
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
        cx: &ViewContext<Self>,
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
                    div.rounded_md().bg(color)
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
                                                move |cx| {
                                                    Tooltip::text(
                                                        format!("{} is muted", github_login),
                                                        cx,
                                                    )
                                                }
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
                                div()
                                    .ml_1()
                                    .child(Label::new(format!("+{extra_count}")))
                                    .into_any_element(),
                            )
                        } else {
                            None
                        }),
                ),
        )
    }
}
