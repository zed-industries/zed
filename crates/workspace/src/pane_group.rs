use crate::{FollowerStatesByLeader, JoinProject, Pane, Workspace};
use anyhow::{anyhow, Result};
use call::{ActiveCall, ParticipantLocation};
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::Vector2F},
    Axis, Border, CursorStyle, ModelHandle, MouseButton, RenderContext, ViewHandle,
};
use project::Project;
use serde::Deserialize;
use settings::Settings;
use theme::Theme;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaneGroup {
    pub(crate) root: Member,
}

impl PaneGroup {
    pub(crate) fn with_root(root: Member) -> Self {
        Self { root }
    }

    pub fn new(pane: ViewHandle<Pane>) -> Self {
        Self {
            root: Member::Pane(pane),
        }
    }

    pub fn split(
        &mut self,
        old_pane: &ViewHandle<Pane>,
        new_pane: &ViewHandle<Pane>,
        direction: SplitDirection,
    ) -> Result<()> {
        match &mut self.root {
            Member::Pane(pane) => {
                if pane == old_pane {
                    self.root = Member::new_axis(old_pane.clone(), new_pane.clone(), direction);
                    Ok(())
                } else {
                    Err(anyhow!("Pane not found"))
                }
            }
            Member::Axis(axis) => axis.split(old_pane, new_pane, direction),
        }
    }

    /// Returns:
    /// - Ok(true) if it found and removed a pane
    /// - Ok(false) if it found but did not remove the pane
    /// - Err(_) if it did not find the pane
    pub fn remove(&mut self, pane: &ViewHandle<Pane>) -> Result<bool> {
        match &mut self.root {
            Member::Pane(_) => Ok(false),
            Member::Axis(axis) => {
                if let Some(last_pane) = axis.remove(pane)? {
                    self.root = last_pane;
                }
                Ok(true)
            }
        }
    }

    pub(crate) fn render(
        &self,
        project: &ModelHandle<Project>,
        theme: &Theme,
        follower_states: &FollowerStatesByLeader,
        active_call: Option<&ModelHandle<ActiveCall>>,
        active_pane: &ViewHandle<Pane>,
        cx: &mut RenderContext<Workspace>,
    ) -> ElementBox {
        self.root.render(
            project,
            theme,
            follower_states,
            active_call,
            active_pane,
            cx,
        )
    }

    pub(crate) fn panes(&self) -> Vec<&ViewHandle<Pane>> {
        let mut panes = Vec::new();
        self.root.collect_panes(&mut panes);
        panes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Member {
    Axis(PaneAxis),
    Pane(ViewHandle<Pane>),
}

impl Member {
    fn new_axis(
        old_pane: ViewHandle<Pane>,
        new_pane: ViewHandle<Pane>,
        direction: SplitDirection,
    ) -> Self {
        use Axis::*;
        use SplitDirection::*;

        let axis = match direction {
            Up | Down => Vertical,
            Left | Right => Horizontal,
        };

        let members = match direction {
            Up | Left => vec![Member::Pane(new_pane), Member::Pane(old_pane)],
            Down | Right => vec![Member::Pane(old_pane), Member::Pane(new_pane)],
        };

        Member::Axis(PaneAxis { axis, members })
    }

    fn contains(&self, needle: &ViewHandle<Pane>) -> bool {
        match self {
            Member::Axis(axis) => axis.members.iter().any(|member| member.contains(needle)),
            Member::Pane(pane) => pane == needle,
        }
    }

    pub fn render(
        &self,
        project: &ModelHandle<Project>,
        theme: &Theme,
        follower_states: &FollowerStatesByLeader,
        active_call: Option<&ModelHandle<ActiveCall>>,
        active_pane: &ViewHandle<Pane>,
        cx: &mut RenderContext<Workspace>,
    ) -> ElementBox {
        enum FollowIntoExternalProject {}

        match self {
            Member::Pane(pane) => {
                let leader = follower_states
                    .iter()
                    .find_map(|(leader_id, follower_states)| {
                        if follower_states.contains_key(pane) {
                            Some(leader_id)
                        } else {
                            None
                        }
                    })
                    .and_then(|leader_id| {
                        let room = active_call?.read(cx).room()?.read(cx);
                        let collaborator = project.read(cx).collaborators().get(leader_id)?;
                        let participant = room.remote_participant_for_peer_id(*leader_id)?;
                        Some((collaborator.replica_id, participant))
                    });

                let border = if let Some((replica_id, _)) = leader.as_ref() {
                    let leader_color = theme.editor.replica_selection_style(*replica_id).cursor;
                    let mut border = Border::all(theme.workspace.leader_border_width, leader_color);
                    border
                        .color
                        .fade_out(1. - theme.workspace.leader_border_opacity);
                    border.overlay = true;
                    border
                } else {
                    Border::default()
                };

                let prompt = if let Some((_, leader)) = leader {
                    match leader.location {
                        ParticipantLocation::SharedProject {
                            project_id: leader_project_id,
                        } => {
                            if Some(leader_project_id) == project.read(cx).remote_id() {
                                None
                            } else {
                                let leader_user = leader.user.clone();
                                let leader_user_id = leader.user.id;
                                Some(
                                    MouseEventHandler::<FollowIntoExternalProject>::new(
                                        pane.id(),
                                        cx,
                                        |_, _| {
                                            Label::new(
                                                format!(
                                                    "Follow {} on their active project",
                                                    leader_user.github_login,
                                                ),
                                                theme
                                                    .workspace
                                                    .external_location_message
                                                    .text
                                                    .clone(),
                                            )
                                            .contained()
                                            .with_style(
                                                theme.workspace.external_location_message.container,
                                            )
                                            .boxed()
                                        },
                                    )
                                    .with_cursor_style(CursorStyle::PointingHand)
                                    .on_click(MouseButton::Left, move |_, cx| {
                                        cx.dispatch_action(JoinProject {
                                            project_id: leader_project_id,
                                            follow_user_id: leader_user_id,
                                        })
                                    })
                                    .aligned()
                                    .bottom()
                                    .right()
                                    .boxed(),
                                )
                            }
                        }
                        ParticipantLocation::UnsharedProject => Some(
                            Label::new(
                                format!(
                                    "{} is viewing an unshared Zed project",
                                    leader.user.github_login
                                ),
                                theme.workspace.external_location_message.text.clone(),
                            )
                            .contained()
                            .with_style(theme.workspace.external_location_message.container)
                            .aligned()
                            .bottom()
                            .right()
                            .boxed(),
                        ),
                        ParticipantLocation::External => Some(
                            Label::new(
                                format!(
                                    "{} is viewing a window outside of Zed",
                                    leader.user.github_login
                                ),
                                theme.workspace.external_location_message.text.clone(),
                            )
                            .contained()
                            .with_style(theme.workspace.external_location_message.container)
                            .aligned()
                            .bottom()
                            .right()
                            .boxed(),
                        ),
                    }
                } else {
                    None
                };

                Stack::new()
                    .with_child(
                        ChildView::new(pane, cx)
                            .contained()
                            .with_border(border)
                            .boxed(),
                    )
                    .with_children(prompt)
                    .boxed()
            }
            Member::Axis(axis) => axis.render(
                project,
                theme,
                follower_states,
                active_call,
                active_pane,
                cx,
            ),
        }
    }

    fn collect_panes<'a>(&'a self, panes: &mut Vec<&'a ViewHandle<Pane>>) {
        match self {
            Member::Axis(axis) => {
                for member in &axis.members {
                    member.collect_panes(panes);
                }
            }
            Member::Pane(pane) => panes.push(pane),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PaneAxis {
    pub axis: Axis,
    pub members: Vec<Member>,
}

impl PaneAxis {
    fn split(
        &mut self,
        old_pane: &ViewHandle<Pane>,
        new_pane: &ViewHandle<Pane>,
        direction: SplitDirection,
    ) -> Result<()> {
        for (mut idx, member) in self.members.iter_mut().enumerate() {
            match member {
                Member::Axis(axis) => {
                    if axis.split(old_pane, new_pane, direction).is_ok() {
                        return Ok(());
                    }
                }
                Member::Pane(pane) => {
                    if pane == old_pane {
                        if direction.axis() == self.axis {
                            if direction.increasing() {
                                idx += 1;
                            }

                            self.members.insert(idx, Member::Pane(new_pane.clone()));
                        } else {
                            *member =
                                Member::new_axis(old_pane.clone(), new_pane.clone(), direction);
                        }
                        return Ok(());
                    }
                }
            }
        }
        Err(anyhow!("Pane not found"))
    }

    fn remove(&mut self, pane_to_remove: &ViewHandle<Pane>) -> Result<Option<Member>> {
        let mut found_pane = false;
        let mut remove_member = None;
        for (idx, member) in self.members.iter_mut().enumerate() {
            match member {
                Member::Axis(axis) => {
                    if let Ok(last_pane) = axis.remove(pane_to_remove) {
                        if let Some(last_pane) = last_pane {
                            *member = last_pane;
                        }
                        found_pane = true;
                        break;
                    }
                }
                Member::Pane(pane) => {
                    if pane == pane_to_remove {
                        found_pane = true;
                        remove_member = Some(idx);
                        break;
                    }
                }
            }
        }

        if found_pane {
            if let Some(idx) = remove_member {
                self.members.remove(idx);
            }

            if self.members.len() == 1 {
                Ok(self.members.pop())
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow!("Pane not found"))
        }
    }

    fn render(
        &self,
        project: &ModelHandle<Project>,
        theme: &Theme,
        follower_state: &FollowerStatesByLeader,
        active_call: Option<&ModelHandle<ActiveCall>>,
        active_pane: &ViewHandle<Pane>,
        cx: &mut RenderContext<Workspace>,
    ) -> ElementBox {
        let last_member_ix = self.members.len() - 1;
        Flex::new(self.axis)
            .with_children(self.members.iter().enumerate().map(|(ix, member)| {
                let mut flex = 1.0;
                if member.contains(active_pane) {
                    flex = cx.global::<Settings>().active_pane_magnification;
                }

                let mut member =
                    member.render(project, theme, follower_state, active_call, active_pane, cx);
                if ix < last_member_ix {
                    let mut border = theme.workspace.pane_divider;
                    border.left = false;
                    border.right = false;
                    border.top = false;
                    border.bottom = false;
                    match self.axis {
                        Axis::Vertical => border.bottom = true,
                        Axis::Horizontal => border.right = true,
                    }
                    member = Container::new(member).with_border(border).boxed();
                }

                FlexItem::new(member).flex(flex, true).boxed()
            }))
            .boxed()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SplitDirection {
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn edge(&self, rect: RectF) -> f32 {
        match self {
            Self::Up => rect.min_y(),
            Self::Down => rect.max_y(),
            Self::Left => rect.min_x(),
            Self::Right => rect.max_x(),
        }
    }

    // Returns a new rectangle which shares an edge in SplitDirection and has `size` along SplitDirection
    pub fn along_edge(&self, rect: RectF, size: f32) -> RectF {
        match self {
            Self::Up => RectF::new(rect.origin(), Vector2F::new(rect.width(), size)),
            Self::Down => RectF::new(
                rect.lower_left() - Vector2F::new(0., size),
                Vector2F::new(rect.width(), size),
            ),
            Self::Left => RectF::new(rect.origin(), Vector2F::new(size, rect.height())),
            Self::Right => RectF::new(
                rect.upper_right() - Vector2F::new(size, 0.),
                Vector2F::new(size, rect.height()),
            ),
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Self::Up | Self::Down => Axis::Vertical,
            Self::Left | Self::Right => Axis::Horizontal,
        }
    }

    pub fn increasing(&self) -> bool {
        match self {
            Self::Left | Self::Up => false,
            Self::Down | Self::Right => true,
        }
    }
}
