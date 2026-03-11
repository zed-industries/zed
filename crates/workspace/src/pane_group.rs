pub use crate::pane_group::element::{PaneAxisElement, pane_axis};
use crate::{
    AnyActiveCall, AppState, CollaboratorId, FollowerState, Pane, ParticipantLocation, Workspace,
    WorkspaceSettings,
    workspace_settings::{PaneSplitDirectionHorizontal, PaneSplitDirectionVertical},
};
use anyhow::Result;
use collections::HashMap;
use gpui::{
    Along, AnyView, AnyWeakView, Axis, Bounds, Entity, Hsla, IntoElement, MouseButton, Pixels,
    Point, StyleRefinement, WeakEntity, Window, point, size,
};
use project::Project;
use schemars::JsonSchema;
use serde::Deserialize;
use settings::Settings;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use ui::prelude::*;

pub const HANDLE_HITBOX_SIZE: f32 = 4.0;
const HORIZONTAL_MIN_SIZE: f32 = 80.;
const VERTICAL_MIN_SIZE: f32 = 100.;

/// One or many panes, arranged in a horizontal or vertical axis due to a split.
/// Panes have all their tabs and capabilities preserved, and can be split again or resized.
/// Single-pane group is a regular pane.
#[derive(Clone)]
pub struct PaneGroup {
    pub root: Member,
    pub state: PaneGroupState,
    pub is_center: bool,
}

pub struct PaneRenderResult {
    pub element: gpui::AnyElement,
    pub contains_active_pane: bool,
}

impl PaneGroup {
    pub fn with_root(root: Member) -> Self {
        Self {
            root,
            state: PaneGroupState::default(),
            is_center: false,
        }
    }

    pub fn new(pane: Entity<Pane>) -> Self {
        Self::with_root(Member::Pane(pane))
    }

    pub fn set_is_center(&mut self, is_center: bool) {
        self.is_center = is_center;
    }

    pub fn split(
        &mut self,
        old_pane: &Entity<Pane>,
        new_pane: &Entity<Pane>,
        direction: SplitDirection,
        cx: &mut App,
    ) {
        let found = match &mut self.root {
            Member::Pane(pane) => {
                if pane == old_pane {
                    self.root = Member::new_axis(old_pane.clone(), new_pane.clone(), direction);
                    self.state.reset_flexes();
                    true
                } else {
                    false
                }
            }
            Member::Axis(axis) => axis.split(old_pane, new_pane, direction),
        };

        // If the pane wasn't found, fall back to splitting the first pane in the tree.
        if !found {
            let first_pane = self.root.first_pane();
            match &mut self.root {
                Member::Pane(_) => {
                    self.root = Member::new_axis(first_pane, new_pane.clone(), direction);
                }
                Member::Axis(axis) => {
                    let _ = axis.split(&first_pane, new_pane, direction);
                }
            }
        }

        self.mark_positions(cx);
    }

    pub fn bounding_box_for_pane(&self, pane: &Entity<Pane>) -> Option<Bounds<Pixels>> {
        match &self.root {
            Member::Pane(_) => None,
            Member::Axis(axis) => axis.bounding_box_for_pane(pane),
        }
    }

    pub fn pane_at_pixel_position(&self, coordinate: Point<Pixels>) -> Option<&Entity<Pane>> {
        match &self.root {
            Member::Pane(pane) => Some(pane),
            Member::Axis(axis) => axis.pane_at_pixel_position(coordinate),
        }
    }

    /// Moves active pane to span the entire border in the given direction,
    /// similar to Vim ctrl+w shift-[hjkl] motion.
    ///
    /// Returns:
    /// - Ok(true) if it found and moved a pane
    /// - Ok(false) if it found but did not move the pane
    /// - Err(_) if it did not find the pane
    pub fn move_to_border(
        &mut self,
        active_pane: &Entity<Pane>,
        direction: SplitDirection,
        cx: &mut App,
    ) -> Result<bool> {
        if let Some(pane) = self.find_pane_at_border(direction)
            && pane == active_pane
        {
            return Ok(false);
        }

        if !self.remove_internal(active_pane)? {
            return Ok(false);
        }

        if let Member::Axis(root) = &mut self.root
            && direction.axis() == root.axis
        {
            let idx = if direction.increasing() {
                root.members.len()
            } else {
                0
            };
            root.insert_pane(idx, active_pane);
            self.mark_positions(cx);
            return Ok(true);
        }

        let members = if direction.increasing() {
            vec![self.root.clone(), Member::Pane(active_pane.clone())]
        } else {
            vec![Member::Pane(active_pane.clone()), self.root.clone()]
        };
        self.root = Member::Axis(PaneAxis::new(direction.axis(), members));
        self.mark_positions(cx);
        Ok(true)
    }

    fn find_pane_at_border(&self, direction: SplitDirection) -> Option<&Entity<Pane>> {
        match &self.root {
            Member::Pane(pane) => Some(pane),
            Member::Axis(axis) => axis.find_pane_at_border(direction),
        }
    }

    /// Returns:
    /// - Ok(true) if it found and removed a pane
    /// - Ok(false) if it found but did not remove the pane
    /// - Err(_) if it did not find the pane
    pub fn remove(&mut self, pane: &Entity<Pane>, cx: &mut App) -> Result<bool> {
        let result = self.remove_internal(pane);
        if let Ok(true) = result {
            self.mark_positions(cx);
        }
        result
    }

    fn remove_internal(&mut self, pane: &Entity<Pane>) -> Result<bool> {
        match &mut self.root {
            Member::Pane(_) => Ok(false),
            Member::Axis(axis) => {
                if let Some(last_pane) = axis.remove(pane)? {
                    self.root = last_pane;
                    self.state.reset_flexes();
                }
                Ok(true)
            }
        }
    }

    pub fn resize(
        &mut self,
        pane: &Entity<Pane>,
        direction: Axis,
        amount: Pixels,
        bounds: &Bounds<Pixels>,
        cx: &mut App,
    ) {
        match &mut self.root {
            Member::Pane(_) => {}
            Member::Axis(axis) => {
                let _ = axis.resize(pane, direction, amount, bounds);
            }
        };
        self.mark_positions(cx);
    }

    pub fn reset_pane_sizes(&mut self, cx: &mut App) {
        match &mut self.root {
            Member::Pane(_) => {}
            Member::Axis(axis) => {
                let _ = axis.reset_pane_sizes();
            }
        };
        self.mark_positions(cx);
    }

    pub fn swap(&mut self, from: &Entity<Pane>, to: &Entity<Pane>, cx: &mut App) {
        match &mut self.root {
            Member::Pane(_) => {}
            Member::Axis(axis) => axis.swap(from, to),
        };
        self.mark_positions(cx);
    }

    pub fn mark_positions(&mut self, cx: &mut App) {
        self.root.mark_positions(self.is_center, cx);
    }

    pub fn render(
        &self,
        zoomed: Option<&AnyWeakView>,
        left_content: Option<AnyElement>,
        right_content: Option<AnyElement>,
        render_cx: &dyn PaneLeaderDecorator,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        let mut state = self.state.0.borrow_mut();
        if left_content.is_some() {
            state.left_entry_is_active = true;
            state.left_entry.get_or_insert(PaneAxisStateEntry {
                flex: 1.0,
                bounding_box: None,
            });
        } else {
            state.left_entry_is_active = false;
        }

        if right_content.is_some() {
            state.right_entry_is_active = true;
            state.right_entry.get_or_insert(PaneAxisStateEntry {
                flex: 1.0,
                bounding_box: None,
            });
        } else {
            state.right_entry_is_active = false;
        }
        drop(state);

        self.root
            .render(
                0,
                zoomed,
                left_content,
                right_content,
                Some(self.state.clone()),
                render_cx,
                window,
                cx,
            )
            .element
    }

    pub fn panes(&self) -> Vec<&Entity<Pane>> {
        let mut panes = Vec::new();
        self.root.collect_panes(&mut panes);
        panes
    }

    pub fn first_pane(&self) -> Entity<Pane> {
        self.root.first_pane()
    }

    pub fn last_pane(&self) -> Entity<Pane> {
        self.root.last_pane()
    }

    pub fn find_pane_in_direction(
        &mut self,
        active_pane: &Entity<Pane>,
        direction: SplitDirection,
        cx: &App,
    ) -> Option<&Entity<Pane>> {
        let bounding_box = self.bounding_box_for_pane(active_pane)?;
        let cursor = active_pane.read(cx).pixel_position_of_cursor(cx);
        let center = match cursor {
            Some(cursor) if bounding_box.contains(&cursor) => cursor,
            _ => bounding_box.center(),
        };

        let distance_to_next = crate::HANDLE_HITBOX_SIZE;

        let target = match direction {
            SplitDirection::Left => {
                Point::new(bounding_box.left() - distance_to_next.into(), center.y)
            }
            SplitDirection::Right => {
                Point::new(bounding_box.right() + distance_to_next.into(), center.y)
            }
            SplitDirection::Up => {
                Point::new(center.x, bounding_box.top() - distance_to_next.into())
            }
            SplitDirection::Down => {
                Point::new(center.x, bounding_box.bottom() + distance_to_next.into())
            }
        };
        self.pane_at_pixel_position(target)
    }

    pub fn invert_axies(&mut self, cx: &mut App) {
        self.root.invert_pane_axies();
        self.mark_positions(cx);
    }
}

#[derive(Debug, Clone)]
pub enum Member {
    Axis(PaneAxis),
    Pane(Entity<Pane>),
}

impl Member {
    pub fn mark_positions(&mut self, in_center_group: bool, cx: &mut App) {
        match self {
            Member::Axis(pane_axis) => {
                for member in pane_axis.members.iter_mut() {
                    member.mark_positions(in_center_group, cx);
                }
            }
            Member::Pane(entity) => entity.update(cx, |pane, _| {
                pane.in_center_group = in_center_group;
            }),
        }
    }
}

#[derive(Clone, Copy)]
pub struct PaneRenderContext<'a> {
    pub project: &'a Entity<Project>,
    pub follower_states: &'a HashMap<CollaboratorId, FollowerState>,
    pub active_call: Option<&'a dyn AnyActiveCall>,
    pub active_pane: &'a Entity<Pane>,
    pub app_state: &'a Arc<AppState>,
    pub workspace: &'a WeakEntity<Workspace>,
}

#[derive(Default)]
pub struct LeaderDecoration {
    border: Option<Hsla>,
    status_box: Option<AnyElement>,
}

pub trait PaneLeaderDecorator {
    fn decorate(&self, pane: &Entity<Pane>, cx: &App) -> LeaderDecoration;
    fn active_pane(&self) -> &Entity<Pane>;
    fn workspace(&self) -> &WeakEntity<Workspace>;
}

pub struct ActivePaneDecorator<'a> {
    active_pane: &'a Entity<Pane>,
    workspace: &'a WeakEntity<Workspace>,
}

impl<'a> ActivePaneDecorator<'a> {
    pub fn new(active_pane: &'a Entity<Pane>, workspace: &'a WeakEntity<Workspace>) -> Self {
        Self {
            active_pane,
            workspace,
        }
    }
}

impl PaneLeaderDecorator for ActivePaneDecorator<'_> {
    fn decorate(&self, _: &Entity<Pane>, _: &App) -> LeaderDecoration {
        LeaderDecoration::default()
    }
    fn active_pane(&self) -> &Entity<Pane> {
        self.active_pane
    }

    fn workspace(&self) -> &WeakEntity<Workspace> {
        self.workspace
    }
}

impl PaneLeaderDecorator for PaneRenderContext<'_> {
    fn decorate(&self, pane: &Entity<Pane>, cx: &App) -> LeaderDecoration {
        let follower_state = self.follower_states.iter().find_map(|(leader_id, state)| {
            if state.center_pane == *pane {
                Some((*leader_id, state))
            } else {
                None
            }
        });
        let Some((leader_id, follower_state)) = follower_state else {
            return LeaderDecoration::default();
        };

        let mut leader_color;
        let status_box;
        match leader_id {
            CollaboratorId::PeerId(peer_id) => {
                let Some(leader) = self
                    .active_call
                    .as_ref()
                    .and_then(|call| call.remote_participant_for_peer_id(peer_id, cx))
                else {
                    return LeaderDecoration::default();
                };

                let is_in_unshared_view = follower_state.active_view_id.is_some_and(|view_id| {
                    !follower_state
                        .items_by_leader_view_id
                        .contains_key(&view_id)
                });

                let mut leader_join_data = None;
                let leader_status_box = match leader.location {
                    ParticipantLocation::SharedProject {
                        project_id: leader_project_id,
                    } => {
                        if Some(leader_project_id) == self.project.read(cx).remote_id() {
                            is_in_unshared_view.then(|| {
                                Label::new(format!(
                                    "{} is in an unshared pane",
                                    leader.user.github_login
                                ))
                            })
                        } else {
                            leader_join_data = Some((leader_project_id, leader.user.id));
                            Some(Label::new(format!(
                                "Follow {} to their active project",
                                leader.user.github_login,
                            )))
                        }
                    }
                    ParticipantLocation::UnsharedProject => Some(Label::new(format!(
                        "{} is viewing an unshared Zed project",
                        leader.user.github_login
                    ))),
                    ParticipantLocation::External => Some(Label::new(format!(
                        "{} is viewing a window outside of Zed",
                        leader.user.github_login
                    ))),
                };
                status_box = leader_status_box.map(|status| {
                    div()
                        .absolute()
                        .w_96()
                        .bottom_3()
                        .right_3()
                        .elevation_2(cx)
                        .p_1()
                        .child(status)
                        .when_some(
                            leader_join_data,
                            |this, (leader_project_id, leader_user_id)| {
                                let app_state = self.app_state.clone();
                                this.cursor_pointer().on_mouse_down(
                                    MouseButton::Left,
                                    move |_, _, cx| {
                                        crate::join_in_room_project(
                                            leader_project_id,
                                            leader_user_id,
                                            app_state.clone(),
                                            cx,
                                        )
                                        .detach_and_log_err(cx);
                                    },
                                )
                            },
                        )
                        .into_any_element()
                });
                leader_color = cx
                    .theme()
                    .players()
                    .color_for_participant(leader.participant_index.0)
                    .cursor;
            }
            CollaboratorId::Agent => {
                status_box = None;
                leader_color = cx.theme().players().agent().cursor;
            }
        }

        let is_in_panel = follower_state.dock_pane.is_some();
        if is_in_panel {
            leader_color.fade_out(0.75);
        } else {
            leader_color.fade_out(0.3);
        }

        LeaderDecoration {
            status_box,
            border: Some(leader_color),
        }
    }

    fn active_pane(&self) -> &Entity<Pane> {
        self.active_pane
    }

    fn workspace(&self) -> &WeakEntity<Workspace> {
        self.workspace
    }
}

impl Member {
    fn new_axis(old_pane: Entity<Pane>, new_pane: Entity<Pane>, direction: SplitDirection) -> Self {
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

        Member::Axis(PaneAxis::new(axis, members))
    }

    fn first_pane(&self) -> Entity<Pane> {
        match self {
            Member::Axis(axis) => axis.members[0].first_pane(),
            Member::Pane(pane) => pane.clone(),
        }
    }

    fn last_pane(&self) -> Entity<Pane> {
        match self {
            Member::Axis(axis) => axis.members.last().unwrap().last_pane(),
            Member::Pane(pane) => pane.clone(),
        }
    }

    pub fn render(
        &self,
        basis: usize,
        zoomed: Option<&AnyWeakView>,
        left_content: Option<AnyElement>,
        right_content: Option<AnyElement>,
        pane_group_state: Option<PaneGroupState>,
        render_cx: &dyn PaneLeaderDecorator,
        window: &mut Window,
        cx: &mut App,
    ) -> PaneRenderResult {
        if let Some(pane_group_state) = pane_group_state
            && (left_content.is_some() || right_content.is_some())
        {
            return match self {
                Member::Axis(axis) if axis.axis == Axis::Horizontal => axis.render(
                    basis + 1,
                    zoomed,
                    Some(pane_group_state),
                    left_content,
                    right_content,
                    render_cx,
                    window,
                    cx,
                ),
                _ => {
                    let mut active_pane_ix = 0;
                    if left_content.is_some() {
                        active_pane_ix += 1;
                    }
                    if right_content.is_some() {
                        active_pane_ix += 1;
                    }

                    let inner = self.render(0, zoomed, None, None, None, render_cx, window, cx);

                    let mut children = Vec::new();
                    children.extend(left_content.map(|content| content.into_any_element()));
                    children.push(inner.element);
                    children.extend(right_content.map(|content| content.into_any_element()));

                    let element = pane_axis(
                        Axis::Horizontal,
                        0,
                        PaneAxisState::with_flexes(vec![
                            children.len() as f32
                                - pane_group_state.total_flex();
                            1
                        ]),
                        Some(pane_group_state),
                        render_cx.workspace().clone(),
                    )
                    .with_active_pane(Some(active_pane_ix))
                    .with_is_leaf_pane_mask(vec![true, matches!(self, Member::Pane(_)), true])
                    .children(children)
                    .into_any_element();
                    PaneRenderResult {
                        element,
                        contains_active_pane: inner.contains_active_pane,
                    }
                }
            };
        }

        match self {
            Member::Pane(pane) => {
                if zoomed == Some(&pane.downgrade().into()) {
                    return PaneRenderResult {
                        element: div().into_any(),
                        contains_active_pane: false,
                    };
                }

                let decoration = render_cx.decorate(pane, cx);
                let is_active = pane == render_cx.active_pane();

                PaneRenderResult {
                    element: div()
                        .relative()
                        .flex_1()
                        .size_full()
                        .child(
                            AnyView::from(pane.clone())
                                .cached(StyleRefinement::default().v_flex().size_full()),
                        )
                        .when_some(decoration.border, |this, color| {
                            this.child(
                                div()
                                    .absolute()
                                    .size_full()
                                    .left_0()
                                    .top_0()
                                    .border_2()
                                    .border_color(color),
                            )
                        })
                        .children(decoration.status_box)
                        .into_any(),
                    contains_active_pane: is_active,
                }
            }
            Member::Axis(axis) => {
                axis.render(basis + 1, zoomed, None, None, None, render_cx, window, cx)
            }
        }
    }

    fn collect_panes<'a>(&'a self, panes: &mut Vec<&'a Entity<Pane>>) {
        match self {
            Member::Axis(axis) => {
                for member in &axis.members {
                    member.collect_panes(panes);
                }
            }
            Member::Pane(pane) => panes.push(pane),
        }
    }

    fn invert_pane_axies(&mut self) {
        match self {
            Self::Axis(axis) => {
                axis.axis = axis.axis.invert();
                for member in axis.members.iter_mut() {
                    member.invert_pane_axies();
                }
            }
            Self::Pane(_) => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct PaneAxisState(Rc<RefCell<PaneAxisStateInner>>);

#[derive(Default, Debug, Clone)]
pub struct PaneGroupState(Rc<RefCell<PaneGroupStateInner>>);

#[derive(Debug)]
struct PaneAxisStateInner {
    entries: Vec<PaneAxisStateEntry>,
}

#[derive(Default, Debug)]
struct PaneGroupStateInner {
    left_entry: Option<PaneAxisStateEntry>,
    left_entry_is_active: bool,
    right_entry: Option<PaneAxisStateEntry>,
    right_entry_is_active: bool,
}

#[derive(Clone, Copy, Debug)]
struct PaneAxisStateEntry {
    flex: f32,
    bounding_box: Option<Bounds<Pixels>>,
}

impl PaneGroupState {
    fn total_flex(&self) -> f32 {
        let state = self.0.borrow();
        state.left_entry.as_ref().map_or(0., |e| e.flex)
            + state.right_entry.as_ref().map_or(0., |e| e.flex)
    }

    fn reset_flexes(&self) {
        let mut state = self.0.borrow_mut();
        if let Some(left_entry) = state.left_entry.as_mut() {
            left_entry.flex = 1.0;
            left_entry.bounding_box = None;
        }
        if let Some(right_entry) = state.right_entry.as_mut() {
            right_entry.flex = 1.0;
            right_entry.bounding_box = None;
        }
    }
}

impl PaneAxisState {
    pub fn new(member_count: usize) -> Self {
        Self(Rc::new(RefCell::new(PaneAxisStateInner {
            entries: vec![
                PaneAxisStateEntry {
                    flex: 1.,
                    bounding_box: None
                };
                member_count
            ],
        })))
    }

    fn with_flexes(flexes: Vec<f32>) -> Self {
        Self(Rc::new(RefCell::new(PaneAxisStateInner {
            entries: flexes
                .into_iter()
                .map(|flex| PaneAxisStateEntry {
                    flex,
                    bounding_box: None,
                })
                .collect(),
        })))
    }

    pub fn flexes(&self) -> Vec<f32> {
        self.0.borrow().entries.iter().map(|e| e.flex).collect()
    }

    pub fn len(&self) -> usize {
        self.0.borrow().entries.len()
    }

    fn resize(&self, len: usize) {
        let mut inner = self.0.borrow_mut();
        inner.entries.clear();
        inner.entries.resize(
            len,
            PaneAxisStateEntry {
                flex: 1.,
                bounding_box: None,
            },
        );
    }

    fn reset_flexes(&self) {
        let mut inner = self.0.borrow_mut();
        inner.entries.iter_mut().for_each(|e| {
            e.flex = 1.;
            e.bounding_box = None;
        });
    }

    fn bounding_box_at(&self, index: usize) -> Option<Bounds<Pixels>> {
        self.0
            .borrow()
            .entries
            .get(index)
            .and_then(|e| e.bounding_box)
    }

    fn bounds(&self) -> Option<Bounds<Pixels>> {
        self.0
            .borrow()
            .entries
            .iter()
            .filter_map(|e| e.bounding_box)
            .reduce(|acc, e| acc.union(&e))
    }
}

#[derive(Debug, Clone)]
pub struct PaneAxis {
    pub axis: Axis,
    pub members: Vec<Member>,
    pub state: PaneAxisState,
}

impl PaneAxis {
    pub fn new(axis: Axis, members: Vec<Member>) -> Self {
        let state = PaneAxisState::new(members.len());
        Self {
            axis,
            members,
            state,
        }
    }

    pub fn load(axis: Axis, members: Vec<Member>, flexes: Option<Vec<f32>>) -> Self {
        let mut flexes = flexes.unwrap_or_else(|| vec![1.; members.len()]);
        if flexes.len() != members.len()
            || (flexes.iter().copied().sum::<f32>() - flexes.len() as f32).abs() >= 0.001
        {
            flexes = vec![1.; members.len()];
        }

        let state = PaneAxisState::with_flexes(flexes);
        Self {
            axis,
            members,
            state,
        }
    }

    fn split(
        &mut self,
        old_pane: &Entity<Pane>,
        new_pane: &Entity<Pane>,
        direction: SplitDirection,
    ) -> bool {
        for (mut idx, member) in self.members.iter_mut().enumerate() {
            match member {
                Member::Axis(axis) => {
                    if axis.split(old_pane, new_pane, direction) {
                        return true;
                    }
                }
                Member::Pane(pane) => {
                    if pane == old_pane {
                        if direction.axis() == self.axis {
                            if direction.increasing() {
                                idx += 1;
                            }
                            self.insert_pane(idx, new_pane);
                        } else {
                            *member =
                                Member::new_axis(old_pane.clone(), new_pane.clone(), direction);
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    fn insert_pane(&mut self, idx: usize, new_pane: &Entity<Pane>) {
        self.members.insert(idx, Member::Pane(new_pane.clone()));
        self.state.resize(self.members.len());
    }

    fn find_pane_at_border(&self, direction: SplitDirection) -> Option<&Entity<Pane>> {
        if self.axis != direction.axis() {
            return None;
        }
        let member = if direction.increasing() {
            self.members.last()
        } else {
            self.members.first()
        };
        member.and_then(|e| match e {
            Member::Pane(pane) => Some(pane),
            Member::Axis(_) => None,
        })
    }

    fn remove(&mut self, pane_to_remove: &Entity<Pane>) -> Result<Option<Member>> {
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
                self.state.resize(self.members.len());
            }

            if self.members.len() == 1 {
                let result = self.members.pop();
                self.state.resize(self.members.len());
                Ok(result)
            } else {
                Ok(None)
            }
        } else {
            anyhow::bail!("Pane not found");
        }
    }

    fn reset_pane_sizes(&self) {
        self.state.resize(self.members.len());
        for member in self.members.iter() {
            if let Member::Axis(axis) = member {
                axis.reset_pane_sizes();
            }
        }
    }

    fn resize(
        &mut self,
        pane: &Entity<Pane>,
        axis: Axis,
        amount: Pixels,
        bounds: &Bounds<Pixels>,
    ) -> Option<bool> {
        let container_size = self.state.bounds().unwrap_or(*bounds).size;

        let found_pane = self
            .members
            .iter()
            .any(|member| matches!(member, Member::Pane(p) if p == pane));

        if found_pane && self.axis != axis {
            return Some(false); // pane found but this is not the correct axis direction
        }
        let mut found_axis_index: Option<usize> = None;
        if !found_pane {
            for (i, pa) in self.members.iter_mut().enumerate() {
                if let Member::Axis(pa) = pa
                    && let Some(done) = pa.resize(pane, axis, amount, bounds)
                {
                    if done {
                        return Some(true); // pane found and operations already done
                    } else if self.axis != axis {
                        return Some(false); // pane found but this is not the correct axis direction
                    } else {
                        found_axis_index = Some(i); // pane found and this is correct direction
                    }
                }
            }
            found_axis_index?; // no pane found
        }

        let min_size = match axis {
            Axis::Horizontal => px(HORIZONTAL_MIN_SIZE),
            Axis::Vertical => px(VERTICAL_MIN_SIZE),
        };
        let mut state = self.state.0.borrow_mut();

        let ix = if found_pane {
            self.members.iter().position(|m| {
                if let Member::Pane(p) = m {
                    p == pane
                } else {
                    false
                }
            })
        } else {
            found_axis_index
        };

        if ix.is_none() {
            return Some(true);
        }

        let ix = ix.unwrap_or(0);

        let size = move |ix: usize, state: &PaneAxisStateInner| {
            container_size.along(axis) * (state.entries[ix].flex / state.entries.len() as f32)
        };

        // Don't allow resizing to less than the minimum size, if elements are already too small
        if min_size - px(1.) > size(ix, &state) {
            return Some(true);
        }

        let flex_changes = |pixel_dx, target_ix: usize, next: isize, state: &PaneAxisStateInner| {
            let flex_change = state.entries.len() as f32 * pixel_dx / container_size.along(axis);
            let current_target_flex = state.entries[target_ix].flex + flex_change;
            let next_target_flex =
                state.entries[(target_ix as isize + next) as usize].flex - flex_change;
            (current_target_flex, next_target_flex)
        };

        let apply_changes = |current_ix: usize,
                             proposed_current_pixel_change: Pixels,
                             state: &mut PaneAxisStateInner| {
            let next_target_size = Pixels::max(
                size(current_ix + 1, state) - proposed_current_pixel_change,
                min_size,
            );
            let current_target_size = Pixels::max(
                size(current_ix, state) + size(current_ix + 1, state) - next_target_size,
                min_size,
            );

            let current_pixel_change = current_target_size - size(current_ix, state);

            let (current_target_flex, next_target_flex) =
                flex_changes(current_pixel_change, current_ix, 1, state);

            state.entries[current_ix].flex = current_target_flex;
            state.entries[current_ix + 1].flex = next_target_flex;
        };

        if ix + 1 == state.entries.len() {
            apply_changes(ix - 1, -1.0 * amount, &mut *state);
        } else {
            apply_changes(ix, amount, &mut *state);
        }
        Some(true)
    }

    fn swap(&mut self, from: &Entity<Pane>, to: &Entity<Pane>) {
        for member in self.members.iter_mut() {
            match member {
                Member::Axis(axis) => axis.swap(from, to),
                Member::Pane(pane) => {
                    if pane == from {
                        *member = Member::Pane(to.clone());
                    } else if pane == to {
                        *member = Member::Pane(from.clone())
                    }
                }
            }
        }
    }

    fn bounding_box_for_pane(&self, pane: &Entity<Pane>) -> Option<Bounds<Pixels>> {
        debug_assert!(self.members.len() == self.state.len());

        for (idx, member) in self.members.iter().enumerate() {
            match member {
                Member::Pane(found) => {
                    if pane == found {
                        return self.state.bounding_box_at(idx);
                    }
                }
                Member::Axis(axis) => {
                    if let Some(rect) = axis.bounding_box_for_pane(pane) {
                        return Some(rect);
                    }
                }
            }
        }
        None
    }

    fn pane_at_pixel_position(&self, coordinate: Point<Pixels>) -> Option<&Entity<Pane>> {
        debug_assert!(self.members.len() == self.state.len());

        for (idx, member) in self.members.iter().enumerate() {
            if let Some(coordinates) = self.state.bounding_box_at(idx)
                && coordinates.contains(&coordinate)
            {
                return match member {
                    Member::Pane(found) => Some(found),
                    Member::Axis(axis) => axis.pane_at_pixel_position(coordinate),
                };
            }
        }
        None
    }

    fn render(
        &self,
        basis: usize,
        zoomed: Option<&AnyWeakView>,
        pane_group_state: Option<PaneGroupState>,
        left_content: Option<AnyElement>,
        right_content: Option<AnyElement>,
        render_cx: &dyn PaneLeaderDecorator,
        window: &mut Window,
        cx: &mut App,
    ) -> PaneRenderResult {
        debug_assert!(self.members.len() == self.state.len());
        let mut active_pane_ix = None;
        let mut contains_active_pane = false;
        let mut is_leaf_pane = vec![false; self.members.len()];

        let rendered_children = left_content
            .into_iter()
            .chain(self.members.iter().enumerate().map(|(ix, member)| {
                match member {
                    Member::Pane(pane) => {
                        is_leaf_pane[ix] = true;
                        if pane == render_cx.active_pane() {
                            active_pane_ix = Some(ix);
                            contains_active_pane = true;
                        }
                    }
                    Member::Axis(_) => {
                        is_leaf_pane[ix] = false;
                    }
                }

                let result = member.render(
                    (basis + ix) * 10,
                    zoomed,
                    None,
                    None,
                    None,
                    render_cx,
                    window,
                    cx,
                );
                if result.contains_active_pane {
                    contains_active_pane = true;
                }
                result.element.into_any_element()
            }))
            .chain(right_content)
            .collect::<Vec<_>>();

        let element = pane_axis(
            self.axis,
            basis,
            self.state.clone(),
            pane_group_state,
            render_cx.workspace().clone(),
        )
        .with_is_leaf_pane_mask(is_leaf_pane)
        .children(rendered_children)
        .with_active_pane(active_pane_ix)
        .into_any_element();

        PaneRenderResult {
            element,
            contains_active_pane,
        }
    }
}

impl PaneAxisStateInner {
    pub fn entries<'a>(
        &'a mut self,
        pane_group_state: Option<&'a mut PaneGroupStateInner>,
    ) -> Vec<&'a mut PaneAxisStateEntry> {
        let mut entries = Vec::new();
        if let Some(pane_group_state) = pane_group_state {
            if let Some(left) = pane_group_state
                .left_entry
                .as_mut()
                .filter(|_| pane_group_state.left_entry_is_active)
            {
                entries.push(left);
            }
            entries.extend(self.entries.iter_mut());
            if let Some(right) = pane_group_state
                .right_entry
                .as_mut()
                .filter(|_| pane_group_state.right_entry_is_active)
            {
                entries.push(right);
            }
        } else {
            entries.extend(self.entries.iter_mut());
        }
        entries
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for SplitDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SplitDirection::Up => write!(f, "up"),
            SplitDirection::Down => write!(f, "down"),
            SplitDirection::Left => write!(f, "left"),
            SplitDirection::Right => write!(f, "right"),
        }
    }
}

impl SplitDirection {
    pub fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn vertical(cx: &mut App) -> Self {
        match WorkspaceSettings::get_global(cx).pane_split_direction_vertical {
            PaneSplitDirectionVertical::Left => SplitDirection::Left,
            PaneSplitDirectionVertical::Right => SplitDirection::Right,
        }
    }

    pub fn horizontal(cx: &mut App) -> Self {
        match WorkspaceSettings::get_global(cx).pane_split_direction_horizontal {
            PaneSplitDirectionHorizontal::Down => SplitDirection::Down,
            PaneSplitDirectionHorizontal::Up => SplitDirection::Up,
        }
    }

    pub fn edge(&self, rect: Bounds<Pixels>) -> Pixels {
        match self {
            Self::Up => rect.origin.y,
            Self::Down => rect.bottom_left().y,
            Self::Left => rect.bottom_left().x,
            Self::Right => rect.bottom_right().x,
        }
    }

    pub fn along_edge(&self, bounds: Bounds<Pixels>, length: Pixels) -> Bounds<Pixels> {
        match self {
            Self::Up => Bounds {
                origin: bounds.origin,
                size: size(bounds.size.width, length),
            },
            Self::Down => Bounds {
                origin: point(bounds.bottom_left().x, bounds.bottom_left().y - length),
                size: size(bounds.size.width, length),
            },
            Self::Left => Bounds {
                origin: bounds.origin,
                size: size(length, bounds.size.height),
            },
            Self::Right => Bounds {
                origin: point(bounds.bottom_right().x - length, bounds.bottom_left().y),
                size: size(length, bounds.size.height),
            },
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

    pub fn opposite(&self) -> SplitDirection {
        match self {
            Self::Down => Self::Up,
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

pub mod element {
    use std::mem;
    use std::{cell::RefCell, iter, rc::Rc};

    use gpui::{
        Along, AnyElement, App, Axis, BorderStyle, Bounds, Element, GlobalElementId,
        HitboxBehavior, IntoElement, MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement,
        Pixels, Point, Size, Style, WeakEntity, Window, px, relative, size,
    };
    use gpui::{CursorStyle, Hitbox};
    use settings::Settings;
    use smallvec::SmallVec;
    use ui::prelude::*;
    use util::ResultExt;

    use crate::Workspace;

    use crate::WorkspaceSettings;
    use crate::pane_group::{PaneAxisStateEntry, PaneGroupState};

    use super::{HANDLE_HITBOX_SIZE, HORIZONTAL_MIN_SIZE, PaneAxisState, VERTICAL_MIN_SIZE};

    const DIVIDER_SIZE: f32 = 1.0;

    pub fn pane_axis(
        axis: Axis,
        basis: usize,
        state: PaneAxisState,
        pane_group_state: Option<PaneGroupState>,
        workspace: WeakEntity<Workspace>,
    ) -> PaneAxisElement {
        PaneAxisElement {
            axis,
            basis,
            state,
            pane_group_state,
            children: SmallVec::new(),
            active_pane_ix: None,
            workspace,
            is_leaf_pane_mask: Vec::new(),
        }
    }

    pub struct PaneAxisElement {
        axis: Axis,
        basis: usize,
        state: PaneAxisState,
        pane_group_state: Option<PaneGroupState>,
        children: SmallVec<[AnyElement; 2]>,
        active_pane_ix: Option<usize>,
        workspace: WeakEntity<Workspace>,
        // Track which children are leaf panes (Member::Pane) vs axes (Member::Axis)
        is_leaf_pane_mask: Vec<bool>,
    }

    pub struct PaneAxisLayout {
        dragged_handle: Rc<RefCell<Option<usize>>>,
        children: Vec<PaneAxisChildLayout>,
    }

    struct PaneAxisChildLayout {
        bounds: Bounds<Pixels>,
        element: AnyElement,
        handle: Option<PaneAxisHandleLayout>,
        is_leaf_pane: bool,
    }

    struct PaneAxisHandleLayout {
        hitbox: Hitbox,
        divider_bounds: Bounds<Pixels>,
    }

    impl PaneAxisElement {
        pub fn with_active_pane(mut self, active_pane_ix: Option<usize>) -> Self {
            self.active_pane_ix = active_pane_ix;
            self
        }

        pub fn with_is_leaf_pane_mask(mut self, mask: Vec<bool>) -> Self {
            self.is_leaf_pane_mask = mask;
            self
        }

        fn compute_resize(
            pane_group_state: Option<&PaneGroupState>,
            state: &PaneAxisState,
            e: &MouseMoveEvent,
            ix: usize,
            axis: Axis,
            child_start: Point<Pixels>,
            container_size: Size<Pixels>,
            workspace: WeakEntity<Workspace>,
            window: &mut Window,
            cx: &mut App,
        ) {
            let mut state = state.0.borrow_mut();
            let mut group_state = pane_group_state.as_ref().map(|state| state.0.borrow_mut());
            let group_state = group_state.as_deref_mut();
            let mut entries = state.entries(group_state);

            let min_size = match axis {
                Axis::Horizontal => px(HORIZONTAL_MIN_SIZE),
                Axis::Vertical => px(VERTICAL_MIN_SIZE),
            };
            debug_assert!(flex_values_in_bounds(&entries));

            // Math to convert a flex value to a pixel value
            let size = move |ix: usize, state: &[&mut PaneAxisStateEntry]| {
                container_size.along(axis) * (state[ix].flex / state.len() as f32)
            };

            // Don't allow resizing to less than the minimum size, if elements are already too small
            if min_size - px(1.) > size(ix, &entries) {
                return;
            }

            // This is basically a "bucket" of pixel changes that need to be applied in response to this
            // mouse event. Probably a small, fractional number like 0.5 or 1.5 pixels
            let mut proposed_current_pixel_change =
                (e.position - child_start).along(axis) - size(ix, &entries);

            // This takes a pixel change, and computes the flex changes that correspond to this pixel change
            // as well as the next one, for some reason
            let flex_changes =
                |pixel_dx, target_ix: usize, next: isize, entries: &[&mut PaneAxisStateEntry]| {
                    let flex_change = pixel_dx / container_size.along(axis);
                    let current_target_flex = entries[target_ix].flex + flex_change;
                    let next_target_flex =
                        entries[(target_ix as isize + next) as usize].flex - flex_change;
                    (current_target_flex, next_target_flex)
                };

            // Generate the list of flex successors, from the current index.
            // If you're dragging column 3 forward, out of 6 columns, then this code will produce [4, 5, 6]
            // If you're dragging column 3 backward, out of 6 columns, then this code will produce [2, 1, 0]
            let mut successors = iter::from_fn({
                let forward = proposed_current_pixel_change > px(0.);
                let mut ix_offset = 0;
                let len = entries.len();
                move || {
                    let result = if forward {
                        (ix + 1 + ix_offset < len).then(|| ix + ix_offset)
                    } else {
                        (ix as isize - ix_offset as isize >= 0).then(|| ix - ix_offset)
                    };

                    ix_offset += 1;

                    result
                }
            });

            // Now actually loop over these, and empty our bucket of pixel changes
            while proposed_current_pixel_change.abs() > px(0.) {
                let Some(current_ix) = successors.next() else {
                    break;
                };

                let next_target_size = Pixels::max(
                    size(current_ix + 1, &entries) - proposed_current_pixel_change,
                    min_size,
                );

                let current_target_size = Pixels::max(
                    size(current_ix, &entries) + size(current_ix + 1, &entries) - next_target_size,
                    min_size,
                );

                let current_pixel_change = current_target_size - size(current_ix, &entries);

                let (current_target_flex, next_target_flex) =
                    flex_changes(current_pixel_change, current_ix, 1, &entries);

                entries[current_ix].flex = current_target_flex;
                entries[current_ix + 1].flex = next_target_flex;

                proposed_current_pixel_change -= current_pixel_change;
            }

            workspace
                .update(cx, |this, cx| this.serialize_workspace(window, cx))
                .log_err();
            cx.stop_propagation();
            window.refresh();
        }

        fn layout_handle(
            axis: Axis,
            pane_bounds: Bounds<Pixels>,
            window: &mut Window,
            _cx: &mut App,
        ) -> PaneAxisHandleLayout {
            let handle_bounds = Bounds {
                origin: pane_bounds.origin.apply_along(axis, |origin| {
                    origin + pane_bounds.size.along(axis) - px(HANDLE_HITBOX_SIZE / 2.)
                }),
                size: pane_bounds
                    .size
                    .apply_along(axis, |_| px(HANDLE_HITBOX_SIZE)),
            };
            let divider_bounds = Bounds {
                origin: pane_bounds
                    .origin
                    .apply_along(axis, |origin| origin + pane_bounds.size.along(axis)),
                size: pane_bounds.size.apply_along(axis, |_| px(DIVIDER_SIZE)),
            };

            PaneAxisHandleLayout {
                hitbox: window.insert_hitbox(handle_bounds, HitboxBehavior::BlockMouse),
                divider_bounds,
            }
        }
    }

    impl IntoElement for PaneAxisElement {
        type Element = Self;

        fn into_element(self) -> Self::Element {
            self
        }
    }

    impl Element for PaneAxisElement {
        type RequestLayoutState = ();
        type PrepaintState = PaneAxisLayout;

        fn id(&self) -> Option<ElementId> {
            Some(self.basis.into())
        }

        fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
            None
        }

        fn request_layout(
            &mut self,
            _global_id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            window: &mut Window,
            cx: &mut App,
        ) -> (gpui::LayoutId, Self::RequestLayoutState) {
            let style = Style {
                flex_grow: 1.,
                flex_shrink: 1.,
                flex_basis: relative(0.).into(),
                size: size(relative(1.).into(), relative(1.).into()),
                ..Style::default()
            };
            (window.request_layout(style, None, cx), ())
        }

        fn prepaint(
            &mut self,
            global_id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            bounds: Bounds<Pixels>,
            _state: &mut Self::RequestLayoutState,
            window: &mut Window,
            cx: &mut App,
        ) -> PaneAxisLayout {
            let dragged_handle = window.with_element_state::<Rc<RefCell<Option<usize>>>, _>(
                global_id.unwrap(),
                |state, _cx| {
                    let state = state.unwrap_or_else(|| Rc::new(RefCell::new(None)));
                    (state.clone(), state)
                },
            );
            let mut state = self.state.0.borrow_mut();
            let mut group_state = self.pane_group_state.as_ref().map(|s| s.0.borrow_mut());
            let group_state = group_state.as_deref_mut();
            let mut entries = state.entries(group_state);

            let len = self.children.len();
            debug_assert!(entries.len() == len);
            debug_assert!(flex_values_in_bounds(&entries));

            let total_flex = len as f32;

            let mut origin = bounds.origin;
            let space_per_flex = bounds.size.along(self.axis) / total_flex;

            // self.state.0.borrow_mut().bounding_boxes.clear();

            let mut layout = PaneAxisLayout {
                dragged_handle,
                children: Vec::new(),
            };
            for (ix, mut child) in mem::take(&mut self.children).into_iter().enumerate() {
                let child_flex = entries[ix].flex;

                let child_size = bounds
                    .size
                    .apply_along(self.axis, |_| space_per_flex * child_flex)
                    .map(|d| d.round());

                let child_bounds = Bounds {
                    origin,
                    size: child_size,
                };

                entries[ix].bounding_box = Some(child_bounds);
                child.layout_as_root(child_size.into(), window, cx);
                child.prepaint_at(origin, window, cx);

                origin = origin.apply_along(self.axis, |val| val + child_size.along(self.axis));

                let is_leaf_pane = self.is_leaf_pane_mask.get(ix).copied().unwrap_or(true);

                layout.children.push(PaneAxisChildLayout {
                    bounds: child_bounds,
                    element: child,
                    handle: None,
                    is_leaf_pane,
                })
            }

            for (ix, child_layout) in layout.children.iter_mut().enumerate() {
                if ix < len - 1 {
                    child_layout.handle = Some(Self::layout_handle(
                        self.axis,
                        child_layout.bounds,
                        window,
                        cx,
                    ));
                }
            }

            layout
        }

        fn paint(
            &mut self,
            _id: Option<&GlobalElementId>,
            _inspector_id: Option<&gpui::InspectorElementId>,
            bounds: gpui::Bounds<ui::prelude::Pixels>,
            _: &mut Self::RequestLayoutState,
            layout: &mut Self::PrepaintState,
            window: &mut Window,
            cx: &mut App,
        ) {
            for child in &mut layout.children {
                child.element.paint(window, cx);
            }

            let overlay_opacity = WorkspaceSettings::get(None, cx)
                .active_pane_modifiers
                .inactive_opacity
                .map(|val| val.0.clamp(0.0, 1.0))
                .and_then(|val| (val <= 1.).then_some(val));

            let mut overlay_background = cx.theme().colors().editor_background;
            if let Some(opacity) = overlay_opacity {
                overlay_background.fade_out(opacity);
            }

            let overlay_border = WorkspaceSettings::get(None, cx)
                .active_pane_modifiers
                .border_size
                .and_then(|val| (val >= 0.).then_some(val));

            for (ix, child) in &mut layout.children.iter_mut().enumerate() {
                if overlay_opacity.is_some() || overlay_border.is_some() {
                    // the overlay has to be painted in origin+1px with size width-1px
                    // in order to accommodate the divider between panels
                    let overlay_bounds = Bounds {
                        origin: child
                            .bounds
                            .origin
                            .apply_along(Axis::Horizontal, |val| val + px(1.)),
                        size: child
                            .bounds
                            .size
                            .apply_along(Axis::Horizontal, |val| val - px(1.)),
                    };

                    if overlay_opacity.is_some()
                        && child.is_leaf_pane
                        && self.active_pane_ix != Some(ix)
                    {
                        window.paint_quad(gpui::fill(overlay_bounds, overlay_background));
                    }

                    if let Some(border) = overlay_border
                        && self.active_pane_ix == Some(ix)
                        && child.is_leaf_pane
                    {
                        window.paint_quad(gpui::quad(
                            overlay_bounds,
                            0.,
                            gpui::transparent_black(),
                            border,
                            cx.theme().colors().border_selected,
                            BorderStyle::Solid,
                        ));
                    }
                }

                if let Some(handle) = child.handle.as_mut() {
                    let cursor_style = match self.axis {
                        Axis::Vertical => CursorStyle::ResizeRow,
                        Axis::Horizontal => CursorStyle::ResizeColumn,
                    };

                    if layout
                        .dragged_handle
                        .borrow()
                        .is_some_and(|dragged_ix| dragged_ix == ix)
                    {
                        window.set_window_cursor_style(cursor_style);
                    } else {
                        window.set_cursor_style(cursor_style, &handle.hitbox);
                    }

                    window.paint_quad(gpui::fill(
                        handle.divider_bounds,
                        cx.theme().colors().pane_group_border,
                    ));

                    window.on_mouse_event({
                        let dragged_handle = layout.dragged_handle.clone();
                        let state = self.state.clone();
                        let group_state = self.pane_group_state.clone();
                        let workspace = self.workspace.clone();
                        let handle_hitbox = handle.hitbox.clone();
                        move |e: &MouseDownEvent, phase, window, cx| {
                            if phase.bubble() && handle_hitbox.is_hovered(window) {
                                dragged_handle.replace(Some(ix));
                                if e.click_count >= 2 {
                                    state.reset_flexes();
                                    if let Some(group_state) = group_state.as_ref() {
                                        group_state.reset_flexes();
                                    }
                                    workspace
                                        .update(cx, |this, cx| this.serialize_workspace(window, cx))
                                        .log_err();

                                    window.refresh();
                                }
                                cx.stop_propagation();
                            }
                        }
                    });
                    window.on_mouse_event({
                        let workspace = self.workspace.clone();
                        let dragged_handle = layout.dragged_handle.clone();
                        let state = self.state.clone();
                        let group_state = self.pane_group_state.clone();
                        let child_bounds = child.bounds;
                        let axis = self.axis;
                        move |e: &MouseMoveEvent, phase, window, cx| {
                            let dragged_handle = dragged_handle.borrow();
                            if phase.bubble() && *dragged_handle == Some(ix) {
                                Self::compute_resize(
                                    group_state.as_ref(),
                                    &state,
                                    e,
                                    ix,
                                    axis,
                                    child_bounds.origin,
                                    bounds.size,
                                    workspace.clone(),
                                    window,
                                    cx,
                                )
                            }
                        }
                    });
                }
            }

            window.on_mouse_event({
                let dragged_handle = layout.dragged_handle.clone();
                move |_: &MouseUpEvent, phase, _window, _cx| {
                    if phase.bubble() {
                        dragged_handle.replace(None);
                    }
                }
            });
        }
    }

    impl ParentElement for PaneAxisElement {
        fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
            self.children.extend(elements)
        }
    }

    fn flex_values_in_bounds(inner: &[&mut PaneAxisStateEntry]) -> bool {
        (inner.iter().map(|e| e.flex).sum::<f32>() - inner.len() as f32).abs() < 0.001
    }
}
