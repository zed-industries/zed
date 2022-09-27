mod add_participant_popover;

use add_participant_popover::AddParticipantPopover;
use client::{Authenticate, PeerId};
use clock::ReplicaId;
use gpui::{
    actions,
    color::Color,
    elements::*,
    geometry::{rect::RectF, vector::vec2f, PathBuilder},
    json::{self, ToJson},
    Border, CursorStyle, Entity, ImageData, MouseButton, MutableAppContext, RenderContext,
    Subscription, View, ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;
use std::{ops::Range, sync::Arc};
use theme::Theme;
use workspace::{FollowNextCollaborator, ToggleFollow, Workspace};

actions!(contacts_titlebar_item, [ToggleAddParticipantPopover]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(CollabTitlebarItem::toggle_add_participant_popover);
}

pub struct CollabTitlebarItem {
    workspace: WeakViewHandle<Workspace>,
    add_participant_popover: Option<ViewHandle<AddParticipantPopover>>,
    _subscriptions: Vec<Subscription>,
}

impl Entity for CollabTitlebarItem {
    type Event = ();
}

impl View for CollabTitlebarItem {
    fn ui_name() -> &'static str {
        "CollabTitlebarItem"
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let workspace = if let Some(workspace) = self.workspace.upgrade(cx) {
            workspace
        } else {
            return Empty::new().boxed();
        };

        let theme = cx.global::<Settings>().theme.clone();
        Flex::row()
            .with_children(self.render_toggle_contacts_button(&workspace, &theme, cx))
            .with_children(self.render_collaborators(&workspace, &theme, cx))
            .with_children(self.render_current_user(&workspace, &theme, cx))
            .with_children(self.render_connection_status(&workspace, cx))
            .boxed()
    }
}

impl CollabTitlebarItem {
    pub fn new(workspace: &ViewHandle<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        let observe_workspace = cx.observe(workspace, |_, _, cx| cx.notify());
        Self {
            workspace: workspace.downgrade(),
            add_participant_popover: None,
            _subscriptions: vec![observe_workspace],
        }
    }

    fn toggle_add_participant_popover(
        &mut self,
        _: &ToggleAddParticipantPopover,
        cx: &mut ViewContext<Self>,
    ) {
        match self.add_participant_popover.take() {
            Some(_) => {}
            None => {
                let view = cx.add_view(|_| AddParticipantPopover::new());
                self.add_participant_popover = Some(view);
            }
        }
        cx.notify();
    }

    fn render_toggle_contacts_button(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        if !workspace.read(cx).client().status().borrow().is_connected() {
            return None;
        }

        let titlebar = &theme.workspace.titlebar;

        Some(
            Stack::new()
                .with_child(
                    MouseEventHandler::<ToggleAddParticipantPopover>::new(0, cx, |state, _| {
                        let style = titlebar
                            .add_participant_button
                            .style_for(state, self.add_participant_popover.is_some());
                        Svg::new("icons/plus_8.svg")
                            .with_color(style.color)
                            .constrained()
                            .with_width(style.icon_width)
                            .aligned()
                            .constrained()
                            .with_width(style.button_width)
                            .with_height(style.button_width)
                            .contained()
                            .with_style(style.container)
                            .boxed()
                    })
                    .with_cursor_style(CursorStyle::PointingHand)
                    .on_click(MouseButton::Left, |_, cx| {
                        cx.dispatch_action(ToggleAddParticipantPopover);
                    })
                    .aligned()
                    .boxed(),
                )
                .with_children(self.add_participant_popover.as_ref().map(|popover| {
                    Overlay::new(
                        ChildView::new(popover)
                            .contained()
                            .with_margin_top(titlebar.height)
                            .with_margin_right(
                                -titlebar.add_participant_button.default.button_width,
                            )
                            .boxed(),
                    )
                    .with_fit_mode(OverlayFitMode::SwitchAnchor)
                    .with_anchor_corner(AnchorCorner::BottomLeft)
                    .boxed()
                }))
                .boxed(),
        )
    }

    fn render_collaborators(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> Vec<ElementBox> {
        let mut collaborators = workspace
            .read(cx)
            .project()
            .read(cx)
            .collaborators()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        collaborators.sort_unstable_by_key(|collaborator| collaborator.replica_id);
        collaborators
            .into_iter()
            .filter_map(|collaborator| {
                Some(self.render_avatar(
                    collaborator.user.avatar.clone()?,
                    collaborator.replica_id,
                    Some((collaborator.peer_id, &collaborator.user.github_login)),
                    workspace,
                    theme,
                    cx,
                ))
            })
            .collect()
    }

    fn render_current_user(
        &self,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        let user = workspace.read(cx).user_store().read(cx).current_user();
        let replica_id = workspace.read(cx).project().read(cx).replica_id();
        let status = *workspace.read(cx).client().status().borrow();
        if let Some(avatar) = user.and_then(|user| user.avatar.clone()) {
            Some(self.render_avatar(avatar, replica_id, None, workspace, theme, cx))
        } else if matches!(status, client::Status::UpgradeRequired) {
            None
        } else {
            Some(
                MouseEventHandler::<Authenticate>::new(0, cx, |state, _| {
                    let style = theme
                        .workspace
                        .titlebar
                        .sign_in_prompt
                        .style_for(state, false);
                    Label::new("Sign in".to_string(), style.text.clone())
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .on_click(MouseButton::Left, |_, cx| cx.dispatch_action(Authenticate))
                .with_cursor_style(CursorStyle::PointingHand)
                .aligned()
                .boxed(),
            )
        }
    }

    fn render_avatar(
        &self,
        avatar: Arc<ImageData>,
        replica_id: ReplicaId,
        peer: Option<(PeerId, &str)>,
        workspace: &ViewHandle<Workspace>,
        theme: &Theme,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let replica_color = theme.editor.replica_selection_style(replica_id).cursor;
        let is_followed = peer.map_or(false, |(peer_id, _)| {
            workspace.read(cx).is_following(peer_id)
        });
        let mut avatar_style = theme.workspace.titlebar.avatar;
        if is_followed {
            avatar_style.border = Border::all(1.0, replica_color);
        }
        let content = Stack::new()
            .with_child(
                Image::new(avatar)
                    .with_style(avatar_style)
                    .constrained()
                    .with_width(theme.workspace.titlebar.avatar_width)
                    .aligned()
                    .boxed(),
            )
            .with_child(
                AvatarRibbon::new(replica_color)
                    .constrained()
                    .with_width(theme.workspace.titlebar.avatar_ribbon.width)
                    .with_height(theme.workspace.titlebar.avatar_ribbon.height)
                    .aligned()
                    .bottom()
                    .boxed(),
            )
            .constrained()
            .with_width(theme.workspace.titlebar.avatar_width)
            .contained()
            .with_margin_left(theme.workspace.titlebar.avatar_margin)
            .boxed();

        if let Some((peer_id, peer_github_login)) = peer {
            MouseEventHandler::<ToggleFollow>::new(replica_id.into(), cx, move |_, _| content)
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(ToggleFollow(peer_id))
                })
                .with_tooltip::<ToggleFollow, _>(
                    peer_id.0 as usize,
                    if is_followed {
                        format!("Unfollow {}", peer_github_login)
                    } else {
                        format!("Follow {}", peer_github_login)
                    },
                    Some(Box::new(FollowNextCollaborator)),
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed()
        } else {
            content
        }
    }

    fn render_connection_status(
        &self,
        workspace: &ViewHandle<Workspace>,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        let theme = &cx.global::<Settings>().theme;
        match &*workspace.read(cx).client().status().borrow() {
            client::Status::ConnectionError
            | client::Status::ConnectionLost
            | client::Status::Reauthenticating { .. }
            | client::Status::Reconnecting { .. }
            | client::Status::ReconnectionError { .. } => Some(
                Container::new(
                    Align::new(
                        ConstrainedBox::new(
                            Svg::new("icons/cloud_slash_12.svg")
                                .with_color(theme.workspace.titlebar.offline_icon.color)
                                .boxed(),
                        )
                        .with_width(theme.workspace.titlebar.offline_icon.width)
                        .boxed(),
                    )
                    .boxed(),
                )
                .with_style(theme.workspace.titlebar.offline_icon.container)
                .boxed(),
            ),
            client::Status::UpgradeRequired => Some(
                Label::new(
                    "Please update Zed to collaborate".to_string(),
                    theme.workspace.titlebar.outdated_warning.text.clone(),
                )
                .contained()
                .with_style(theme.workspace.titlebar.outdated_warning.container)
                .aligned()
                .boxed(),
            ),
            _ => None,
        }
    }
}

pub struct AvatarRibbon {
    color: Color,
}

impl AvatarRibbon {
    pub fn new(color: Color) -> AvatarRibbon {
        AvatarRibbon { color }
    }
}

impl Element for AvatarRibbon {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: gpui::SizeConstraint,
        _: &mut gpui::LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        (constraint.max, ())
    }

    fn paint(
        &mut self,
        bounds: gpui::geometry::rect::RectF,
        _: gpui::geometry::rect::RectF,
        _: &mut Self::LayoutState,
        cx: &mut gpui::PaintContext,
    ) -> Self::PaintState {
        let mut path = PathBuilder::new();
        path.reset(bounds.lower_left());
        path.curve_to(
            bounds.origin() + vec2f(bounds.height(), 0.),
            bounds.origin(),
        );
        path.line_to(bounds.upper_right() - vec2f(bounds.height(), 0.));
        path.curve_to(bounds.lower_right(), bounds.upper_right());
        path.line_to(bounds.lower_left());
        cx.scene.push_path(path.build(self.color, None));
    }

    fn dispatch_event(
        &mut self,
        _: &gpui::Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        _: &mut gpui::EventContext,
    ) -> bool {
        false
    }

    fn rect_for_text_range(
        &self,
        _: Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &gpui::MeasurementContext,
    ) -> Option<RectF> {
        None
    }

    fn debug(
        &self,
        bounds: gpui::geometry::rect::RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        _: &gpui::DebugContext,
    ) -> gpui::json::Value {
        json::json!({
            "type": "AvatarRibbon",
            "bounds": bounds.to_json(),
            "color": self.color.to_json(),
        })
    }
}
