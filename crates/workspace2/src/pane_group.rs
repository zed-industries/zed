use crate::{AppState, FollowerState, Pane, Workspace};
use anyhow::{anyhow, bail, Result};
use collections::HashMap;
use db::sqlez::{
    bindable::{Bind, Column, StaticColumnCount},
    statement::Statement,
};
use gpui::{
    point, size, AnyWeakView, Bounds, Div, IntoElement, Model, Pixels, Point, View, ViewContext,
};
use parking_lot::Mutex;
use project::Project;
use serde::Deserialize;
use std::sync::Arc;
use ui::prelude::*;

const HANDLE_HITBOX_SIZE: f32 = 4.0;
const HORIZONTAL_MIN_SIZE: f32 = 80.;
const VERTICAL_MIN_SIZE: f32 = 100.;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Axis {
    Vertical,
    Horizontal,
}

impl StaticColumnCount for Axis {}
impl Bind for Axis {
    fn bind(&self, statement: &Statement, start_index: i32) -> anyhow::Result<i32> {
        match self {
            Axis::Horizontal => "Horizontal",
            Axis::Vertical => "Vertical",
        }
        .bind(statement, start_index)
    }
}

impl Column for Axis {
    fn column(statement: &mut Statement, start_index: i32) -> anyhow::Result<(Self, i32)> {
        String::column(statement, start_index).and_then(|(axis_text, next_index)| {
            Ok((
                match axis_text.as_str() {
                    "Horizontal" => Axis::Horizontal,
                    "Vertical" => Axis::Vertical,
                    _ => bail!("Stored serialized item kind is incorrect"),
                },
                next_index,
            ))
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct PaneGroup {
    pub(crate) root: Member,
}

impl PaneGroup {
    pub(crate) fn with_root(root: Member) -> Self {
        Self { root }
    }

    pub fn new(pane: View<Pane>) -> Self {
        Self {
            root: Member::Pane(pane),
        }
    }

    pub fn split(
        &mut self,
        old_pane: &View<Pane>,
        new_pane: &View<Pane>,
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

    pub fn bounding_box_for_pane(&self, pane: &View<Pane>) -> Option<Bounds<Pixels>> {
        match &self.root {
            Member::Pane(_) => None,
            Member::Axis(axis) => axis.bounding_box_for_pane(pane),
        }
    }

    pub fn pane_at_pixel_position(&self, coordinate: Point<Pixels>) -> Option<&View<Pane>> {
        match &self.root {
            Member::Pane(pane) => Some(pane),
            Member::Axis(axis) => axis.pane_at_pixel_position(coordinate),
        }
    }

    /// Returns:
    /// - Ok(true) if it found and removed a pane
    /// - Ok(false) if it found but did not remove the pane
    /// - Err(_) if it did not find the pane
    pub fn remove(&mut self, pane: &View<Pane>) -> Result<bool> {
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

    pub fn swap(&mut self, from: &View<Pane>, to: &View<Pane>) {
        match &mut self.root {
            Member::Pane(_) => {}
            Member::Axis(axis) => axis.swap(from, to),
        };
    }

    pub(crate) fn render(
        &self,
        project: &Model<Project>,
        follower_states: &HashMap<View<Pane>, FollowerState>,
        active_pane: &View<Pane>,
        zoomed: Option<&AnyWeakView>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> impl IntoElement {
        self.root.render(
            project,
            0,
            follower_states,
            active_pane,
            zoomed,
            app_state,
            cx,
        )
    }

    pub(crate) fn panes(&self) -> Vec<&View<Pane>> {
        let mut panes = Vec::new();
        self.root.collect_panes(&mut panes);
        panes
    }

    pub(crate) fn first_pane(&self) -> View<Pane> {
        self.root.first_pane()
    }
}

#[derive(Clone, PartialEq)]
pub(crate) enum Member {
    Axis(PaneAxis),
    Pane(View<Pane>),
}

impl Member {
    fn new_axis(old_pane: View<Pane>, new_pane: View<Pane>, direction: SplitDirection) -> Self {
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

    fn contains(&self, needle: &View<Pane>) -> bool {
        match self {
            Member::Axis(axis) => axis.members.iter().any(|member| member.contains(needle)),
            Member::Pane(pane) => pane == needle,
        }
    }

    fn first_pane(&self) -> View<Pane> {
        match self {
            Member::Axis(axis) => axis.members[0].first_pane(),
            Member::Pane(pane) => pane.clone(),
        }
    }

    pub fn render(
        &self,
        project: &Model<Project>,
        basis: usize,
        follower_states: &HashMap<View<Pane>, FollowerState>,
        active_pane: &View<Pane>,
        zoomed: Option<&AnyWeakView>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> impl IntoElement {
        match self {
            Member::Pane(pane) => {
                // todo!()
                // let pane_element = if Some(pane.into()) == zoomed {
                //     None
                // } else {
                //     Some(pane)
                // };

                div().size_full().child(pane.clone()).into_any()

                //         Stack::new()
                //             .with_child(pane_element.contained().with_border(leader_border))
                //             .with_children(leader_status_box)
                //             .into_any()

                // let el = div()
                //     .flex()
                //     .flex_1()
                //     .gap_px()
                //     .w_full()
                //     .h_full()
                //     .bg(cx.theme().colors().editor)
                //     .children();
            }
            Member::Axis(axis) => axis
                .render(
                    project,
                    basis + 1,
                    follower_states,
                    active_pane,
                    zoomed,
                    app_state,
                    cx,
                )
                .into_any(),
        }

        // enum FollowIntoExternalProject {}

        // match self {
        //     Member::Pane(pane) => {
        //         let pane_element = if Some(&**pane) == zoomed {
        //             Empty::new().into_any()
        //         } else {
        //             ChildView::new(pane, cx).into_any()
        //         };

        //         let leader = follower_states.get(pane).and_then(|state| {
        //             let room = active_call?.read(cx).room()?.read(cx);
        //             room.remote_participant_for_peer_id(state.leader_id)
        //         });

        //         let mut leader_border = Border::default();
        //         let mut leader_status_box = None;
        //         if let Some(leader) = &leader {
        //             let leader_color = theme
        //                 .editor
        //                 .selection_style_for_room_participant(leader.participant_index.0)
        //                 .cursor;
        //             leader_border = Border::all(theme.workspace.leader_border_width, leader_color);
        //             leader_border
        //                 .color
        //                 .fade_out(1. - theme.workspace.leader_border_opacity);
        //             leader_border.overlay = true;

        //             leader_status_box = match leader.location {
        //                 ParticipantLocation::SharedProject {
        //                     project_id: leader_project_id,
        //                 } => {
        //                     if Some(leader_project_id) == project.read(cx).remote_id() {
        //                         None
        //                     } else {
        //                         let leader_user = leader.user.clone();
        //                         let leader_user_id = leader.user.id;
        //                         Some(
        //                             MouseEventHandler::new::<FollowIntoExternalProject, _>(
        //                                 pane.id(),
        //                                 cx,
        //                                 |_, _| {
        //                                     Label::new(
        //                                         format!(
        //                                             "Follow {} to their active project",
        //                                             leader_user.github_login,
        //                                         ),
        //                                         theme
        //                                             .workspace
        //                                             .external_location_message
        //                                             .text
        //                                             .clone(),
        //                                     )
        //                                     .contained()
        //                                     .with_style(
        //                                         theme.workspace.external_location_message.container,
        //                                     )
        //                                 },
        //                             )
        //                             .with_cursor_style(CursorStyle::PointingHand)
        //                             .on_click(MouseButton::Left, move |_, this, cx| {
        //                                 crate::join_remote_project(
        //                                     leader_project_id,
        //                                     leader_user_id,
        //                                     this.app_state().clone(),
        //                                     cx,
        //                                 )
        //                                 .detach_and_log_err(cx);
        //                             })
        //                             .aligned()
        //                             .bottom()
        //                             .right()
        //                             .into_any(),
        //                         )
        //                     }
        //                 }
        //                 ParticipantLocation::UnsharedProject => Some(
        //                     Label::new(
        //                         format!(
        //                             "{} is viewing an unshared Zed project",
        //                             leader.user.github_login
        //                         ),
        //                         theme.workspace.external_location_message.text.clone(),
        //                     )
        //                     .contained()
        //                     .with_style(theme.workspace.external_location_message.container)
        //                     .aligned()
        //                     .bottom()
        //                     .right()
        //                     .into_any(),
        //                 ),
        //                 ParticipantLocation::External => Some(
        //                     Label::new(
        //                         format!(
        //                             "{} is viewing a window outside of Zed",
        //                             leader.user.github_login
        //                         ),
        //                         theme.workspace.external_location_message.text.clone(),
        //                     )
        //                     .contained()
        //                     .with_style(theme.workspace.external_location_message.container)
        //                     .aligned()
        //                     .bottom()
        //                     .right()
        //                     .into_any(),
        //                 ),
        //             };
        //         }

        //         Stack::new()
        //             .with_child(pane_element.contained().with_border(leader_border))
        //             .with_children(leader_status_box)
        //             .into_any()
        //     }
        //     Member::Axis(axis) => axis.render(
        //         project,
        //         basis + 1,
        //         theme,
        //         follower_states,
        //         active_call,
        //         active_pane,
        //         zoomed,
        //         app_state,
        //         cx,
        //     ),
        // }
    }

    fn collect_panes<'a>(&'a self, panes: &mut Vec<&'a View<Pane>>) {
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

#[derive(Clone)]
pub(crate) struct PaneAxis {
    pub axis: Axis,
    pub members: Vec<Member>,
    pub flexes: Arc<Mutex<Vec<f32>>>,
    pub bounding_boxes: Arc<Mutex<Vec<Option<Bounds<Pixels>>>>>,
}

impl PartialEq for PaneAxis {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PaneAxis {
    pub fn new(axis: Axis, members: Vec<Member>) -> Self {
        let flexes = Arc::new(Mutex::new(vec![1.; members.len()]));
        let bounding_boxes = Arc::new(Mutex::new(vec![None; members.len()]));
        Self {
            axis,
            members,
            flexes,
            bounding_boxes,
        }
    }

    pub fn load(axis: Axis, members: Vec<Member>, flexes: Option<Vec<f32>>) -> Self {
        let flexes = flexes.unwrap_or_else(|| vec![1.; members.len()]);
        debug_assert!(members.len() == flexes.len());

        let flexes = Arc::new(Mutex::new(flexes));
        let bounding_boxes = Arc::new(Mutex::new(vec![None; members.len()]));
        Self {
            axis,
            members,
            flexes,
            bounding_boxes,
        }
    }

    fn split(
        &mut self,
        old_pane: &View<Pane>,
        new_pane: &View<Pane>,
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
                            *self.flexes.lock() = vec![1.; self.members.len()];
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

    fn remove(&mut self, pane_to_remove: &View<Pane>) -> Result<Option<Member>> {
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
                *self.flexes.lock() = vec![1.; self.members.len()];
            }

            if self.members.len() == 1 {
                let result = self.members.pop();
                *self.flexes.lock() = vec![1.; self.members.len()];
                Ok(result)
            } else {
                Ok(None)
            }
        } else {
            Err(anyhow!("Pane not found"))
        }
    }

    fn swap(&mut self, from: &View<Pane>, to: &View<Pane>) {
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

    fn bounding_box_for_pane(&self, pane: &View<Pane>) -> Option<Bounds<Pixels>> {
        debug_assert!(self.members.len() == self.bounding_boxes.lock().len());

        for (idx, member) in self.members.iter().enumerate() {
            match member {
                Member::Pane(found) => {
                    if pane == found {
                        return self.bounding_boxes.lock()[idx];
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

    fn pane_at_pixel_position(&self, coordinate: Point<Pixels>) -> Option<&View<Pane>> {
        debug_assert!(self.members.len() == self.bounding_boxes.lock().len());

        let bounding_boxes = self.bounding_boxes.lock();

        for (idx, member) in self.members.iter().enumerate() {
            if let Some(coordinates) = bounding_boxes[idx] {
                if coordinates.contains_point(&coordinate) {
                    return match member {
                        Member::Pane(found) => Some(found),
                        Member::Axis(axis) => axis.pane_at_pixel_position(coordinate),
                    };
                }
            }
        }
        None
    }

    fn render(
        &self,
        project: &Model<Project>,
        basis: usize,
        follower_states: &HashMap<View<Pane>, FollowerState>,
        active_pane: &View<Pane>,
        zoomed: Option<&AnyWeakView>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> Div {
        debug_assert!(self.members.len() == self.flexes.lock().len());

        div()
            .flex()
            .flex_auto()
            .map(|s| match self.axis {
                Axis::Vertical => s.flex_col(),
                Axis::Horizontal => s.flex_row(),
            })
            .children(self.members.iter().enumerate().map(|(ix, member)| {
                match member {
                    Member::Axis(axis) => axis
                        .render(
                            project,
                            basis,
                            follower_states,
                            active_pane,
                            zoomed,
                            app_state,
                            cx,
                        )
                        .into_any_element(),
                    Member::Pane(pane) => pane.clone().into_any_element(),
                }
            }))

        // let mut pane_axis = PaneAxisElement::new(
        //     self.axis,
        //     basis,
        //     self.flexes.clone(),
        //     self.bounding_boxes.clone(),
        // );
        // let mut active_pane_ix = None;

        // let mut members = self.members.iter().enumerate().peekable();
        // while let Some((ix, member)) = members.next() {
        //     let last = members.peek().is_none();

        //     if member.contains(active_pane) {
        //         active_pane_ix = Some(ix);
        //     }

        //     let mut member = member.render(
        //         project,
        //         (basis + ix) * 10,
        //         theme,
        //         follower_states,
        //         active_call,
        //         active_pane,
        //         zoomed,
        //         app_state,
        //         cx,
        //     );

        //     if !last {
        //         let mut border = theme.workspace.pane_divider;
        //         border.left = false;
        //         border.right = false;
        //         border.top = false;
        //         border.bottom = false;

        //         match self.axis {
        //             Axis::Vertical => border.bottom = true,
        //             Axis::Horizontal => border.right = true,
        //         }

        //         member = member.contained().with_border(border).into_any();
        //     }

        //     pane_axis = pane_axis.with_child(member.into_any());
        // }
        // pane_axis.set_active_pane(active_pane_ix);
        // pane_axis.into_any()
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

    pub fn edge(&self, rect: Bounds<Pixels>) -> Pixels {
        match self {
            Self::Up => rect.origin.y,
            Self::Down => rect.lower_left().y,
            Self::Left => rect.lower_left().x,
            Self::Right => rect.lower_right().x,
        }
    }

    pub fn along_edge(&self, bounds: Bounds<Pixels>, length: Pixels) -> Bounds<Pixels> {
        match self {
            Self::Up => Bounds {
                origin: bounds.origin,
                size: size(bounds.size.width, length),
            },
            Self::Down => Bounds {
                origin: point(bounds.lower_left().x, bounds.lower_left().y - length),
                size: size(bounds.size.width, length),
            },
            Self::Left => Bounds {
                origin: bounds.origin,
                size: size(length, bounds.size.height),
            },
            Self::Right => Bounds {
                origin: point(bounds.lower_right().x - length, bounds.lower_left().y),
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
}

// mod element {
//     // use std::{cell::RefCell, iter::from_fn, ops::Range, rc::Rc};

//     // use gpui::{
//     //     geometry::{
//     //         rect::Bounds<Pixels>,
//     //         vector::{vec2f, Vector2F},
//     //     },
//     //     json::{self, ToJson},
//     //     platform::{CursorStyle, MouseButton},
//     //     scene::MouseDrag,
//     //     AnyElement, Axis, CursorRegion, Element, EventContext, MouseRegion, Bounds<Pixels>Ext,
//     //     SizeConstraint, Vector2FExt, ViewContext,
//     // };

//     use crate::{
//         pane_group::{HANDLE_HITBOX_SIZE, HORIZONTAL_MIN_SIZE, VERTICAL_MIN_SIZE},
//         Workspace, WorkspaceSettings,
//     };

//     pub struct PaneAxisElement {
//         axis: Axis,
//         basis: usize,
//         active_pane_ix: Option<usize>,
//         flexes: Rc<RefCell<Vec<f32>>>,
//         children: Vec<AnyElement<Workspace>>,
//         bounding_boxes: Rc<RefCell<Vec<Option<Bounds<Pixels>>>>>,
//     }

//     impl PaneAxisElement {
//         pub fn new(
//             axis: Axis,
//             basis: usize,
//             flexes: Rc<RefCell<Vec<f32>>>,
//             bounding_boxes: Rc<RefCell<Vec<Option<Bounds<Pixels>>>>>,
//         ) -> Self {
//             Self {
//                 axis,
//                 basis,
//                 flexes,
//                 bounding_boxes,
//                 active_pane_ix: None,
//                 children: Default::default(),
//             }
//         }

//         pub fn set_active_pane(&mut self, active_pane_ix: Option<usize>) {
//             self.active_pane_ix = active_pane_ix;
//         }

//         fn layout_children(
//             &mut self,
//             active_pane_magnification: f32,
//             constraint: SizeConstraint,
//             remaining_space: &mut f32,
//             remaining_flex: &mut f32,
//             cross_axis_max: &mut f32,
//             view: &mut Workspace,
//             cx: &mut ViewContext<Workspace>,
//         ) {
//             let flexes = self.flexes.borrow();
//             let cross_axis = self.axis.invert();
//             for (ix, child) in self.children.iter_mut().enumerate() {
//                 let flex = if active_pane_magnification != 1. {
//                     if let Some(active_pane_ix) = self.active_pane_ix {
//                         if ix == active_pane_ix {
//                             active_pane_magnification
//                         } else {
//                             1.
//                         }
//                     } else {
//                         1.
//                     }
//                 } else {
//                     flexes[ix]
//                 };

//                 let child_size = if *remaining_flex == 0.0 {
//                     *remaining_space
//                 } else {
//                     let space_per_flex = *remaining_space / *remaining_flex;
//                     space_per_flex * flex
//                 };

//                 let child_constraint = match self.axis {
//                     Axis::Horizontal => SizeConstraint::new(
//                         vec2f(child_size, constraint.min.y()),
//                         vec2f(child_size, constraint.max.y()),
//                     ),
//                     Axis::Vertical => SizeConstraint::new(
//                         vec2f(constraint.min.x(), child_size),
//                         vec2f(constraint.max.x(), child_size),
//                     ),
//                 };
//                 let child_size = child.layout(child_constraint, view, cx);
//                 *remaining_space -= child_size.along(self.axis);
//                 *remaining_flex -= flex;
//                 *cross_axis_max = cross_axis_max.max(child_size.along(cross_axis));
//             }
//         }

//         fn handle_resize(
//             flexes: Rc<RefCell<Vec<f32>>>,
//             axis: Axis,
//             preceding_ix: usize,
//             child_start: Vector2F,
//             drag_bounds: Bounds<Pixels>,
//         ) -> impl Fn(MouseDrag, &mut Workspace, &mut EventContext<Workspace>) {
//             let size = move |ix, flexes: &[f32]| {
//                 drag_bounds.length_along(axis) * (flexes[ix] / flexes.len() as f32)
//             };

//             move |drag, workspace: &mut Workspace, cx| {
//                 if drag.end {
//                     // TODO: Clear cascading resize state
//                     return;
//                 }
//                 let min_size = match axis {
//                     Axis::Horizontal => HORIZONTAL_MIN_SIZE,
//                     Axis::Vertical => VERTICAL_MIN_SIZE,
//                 };
//                 let mut flexes = flexes.borrow_mut();

//                 // Don't allow resizing to less than the minimum size, if elements are already too small
//                 if min_size - 1. > size(preceding_ix, flexes.as_slice()) {
//                     return;
//                 }

//                 let mut proposed_current_pixel_change = (drag.position - child_start).along(axis)
//                     - size(preceding_ix, flexes.as_slice());

//                 let flex_changes = |pixel_dx, target_ix, next: isize, flexes: &[f32]| {
//                     let flex_change = pixel_dx / drag_bounds.length_along(axis);
//                     let current_target_flex = flexes[target_ix] + flex_change;
//                     let next_target_flex =
//                         flexes[(target_ix as isize + next) as usize] - flex_change;
//                     (current_target_flex, next_target_flex)
//                 };

//                 let mut successors = from_fn({
//                     let forward = proposed_current_pixel_change > 0.;
//                     let mut ix_offset = 0;
//                     let len = flexes.len();
//                     move || {
//                         let result = if forward {
//                             (preceding_ix + 1 + ix_offset < len).then(|| preceding_ix + ix_offset)
//                         } else {
//                             (preceding_ix as isize - ix_offset as isize >= 0)
//                                 .then(|| preceding_ix - ix_offset)
//                         };

//                         ix_offset += 1;

//                         result
//                     }
//                 });

//                 while proposed_current_pixel_change.abs() > 0. {
//                     let Some(current_ix) = successors.next() else {
//                         break;
//                     };

//                     let next_target_size = f32::max(
//                         size(current_ix + 1, flexes.as_slice()) - proposed_current_pixel_change,
//                         min_size,
//                     );

//                     let current_target_size = f32::max(
//                         size(current_ix, flexes.as_slice())
//                             + size(current_ix + 1, flexes.as_slice())
//                             - next_target_size,
//                         min_size,
//                     );

//                     let current_pixel_change =
//                         current_target_size - size(current_ix, flexes.as_slice());

//                     let (current_target_flex, next_target_flex) =
//                         flex_changes(current_pixel_change, current_ix, 1, flexes.as_slice());

//                     flexes[current_ix] = current_target_flex;
//                     flexes[current_ix + 1] = next_target_flex;

//                     proposed_current_pixel_change -= current_pixel_change;
//                 }

//                 workspace.schedule_serialize(cx);
//                 cx.notify();
//             }
//         }
//     }

//     impl Extend<AnyElement<Workspace>> for PaneAxisElement {
//         fn extend<T: IntoIterator<Item = AnyElement<Workspace>>>(&mut self, children: T) {
//             self.children.extend(children);
//         }
//     }

//     impl Element<Workspace> for PaneAxisElement {
//         type LayoutState = f32;
//         type PaintState = ();

//         fn layout(
//             &mut self,
//             constraint: SizeConstraint,
//             view: &mut Workspace,
//             cx: &mut ViewContext<Workspace>,
//         ) -> (Vector2F, Self::LayoutState) {
//             debug_assert!(self.children.len() == self.flexes.borrow().len());

//             let active_pane_magnification =
//                 settings::get::<WorkspaceSettings>(cx).active_pane_magnification;

//             let mut remaining_flex = 0.;

//             if active_pane_magnification != 1. {
//                 let active_pane_flex = self
//                     .active_pane_ix
//                     .map(|_| active_pane_magnification)
//                     .unwrap_or(1.);
//                 remaining_flex += self.children.len() as f32 - 1. + active_pane_flex;
//             } else {
//                 for flex in self.flexes.borrow().iter() {
//                     remaining_flex += flex;
//                 }
//             }

//             let mut cross_axis_max: f32 = 0.0;
//             let mut remaining_space = constraint.max_along(self.axis);

//             if remaining_space.is_infinite() {
//                 panic!("flex contains flexible children but has an infinite constraint along the flex axis");
//             }

//             self.layout_children(
//                 active_pane_magnification,
//                 constraint,
//                 &mut remaining_space,
//                 &mut remaining_flex,
//                 &mut cross_axis_max,
//                 view,
//                 cx,
//             );

//             let mut size = match self.axis {
//                 Axis::Horizontal => vec2f(constraint.max.x() - remaining_space, cross_axis_max),
//                 Axis::Vertical => vec2f(cross_axis_max, constraint.max.y() - remaining_space),
//             };

//             if constraint.min.x().is_finite() {
//                 size.set_x(size.x().max(constraint.min.x()));
//             }
//             if constraint.min.y().is_finite() {
//                 size.set_y(size.y().max(constraint.min.y()));
//             }

//             if size.x() > constraint.max.x() {
//                 size.set_x(constraint.max.x());
//             }
//             if size.y() > constraint.max.y() {
//                 size.set_y(constraint.max.y());
//             }

//             (size, remaining_space)
//         }

//         fn paint(
//             &mut self,
//             bounds: Bounds<Pixels>,
//             visible_bounds: Bounds<Pixels>,
//             remaining_space: &mut Self::LayoutState,
//             view: &mut Workspace,
//             cx: &mut ViewContext<Workspace>,
//         ) -> Self::PaintState {
//             let can_resize = settings::get::<WorkspaceSettings>(cx).active_pane_magnification == 1.;
//             let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

//             let overflowing = *remaining_space < 0.;
//             if overflowing {
//                 cx.scene().push_layer(Some(visible_bounds));
//             }

//             let mut child_origin = bounds.origin();

//             let mut bounding_boxes = self.bounding_boxes.borrow_mut();
//             bounding_boxes.clear();

//             let mut children_iter = self.children.iter_mut().enumerate().peekable();
//             while let Some((ix, child)) = children_iter.next() {
//                 let child_start = child_origin.clone();
//                 child.paint(child_origin, visible_bounds, view, cx);

//                 bounding_boxes.push(Some(Bounds<Pixels>::new(child_origin, child.size())));

//                 match self.axis {
//                     Axis::Horizontal => child_origin += vec2f(child.size().x(), 0.0),
//                     Axis::Vertical => child_origin += vec2f(0.0, child.size().y()),
//                 }

//                 if can_resize && children_iter.peek().is_some() {
//                     cx.scene().push_stacking_context(None, None);

//                     let handle_origin = match self.axis {
//                         Axis::Horizontal => child_origin - vec2f(HANDLE_HITBOX_SIZE / 2., 0.0),
//                         Axis::Vertical => child_origin - vec2f(0.0, HANDLE_HITBOX_SIZE / 2.),
//                     };

//                     let handle_bounds = match self.axis {
//                         Axis::Horizontal => Bounds<Pixels>::new(
//                             handle_origin,
//                             vec2f(HANDLE_HITBOX_SIZE, visible_bounds.height()),
//                         ),
//                         Axis::Vertical => Bounds<Pixels>::new(
//                             handle_origin,
//                             vec2f(visible_bounds.width(), HANDLE_HITBOX_SIZE),
//                         ),
//                     };

//                     let style = match self.axis {
//                         Axis::Horizontal => CursorStyle::ResizeLeftRight,
//                         Axis::Vertical => CursorStyle::ResizeUpDown,
//                     };

//                     cx.scene().push_cursor_region(CursorRegion {
//                         bounds: handle_bounds,
//                         style,
//                     });

//                     enum ResizeHandle {}
//                     let mut mouse_region = MouseRegion::new::<ResizeHandle>(
//                         cx.view_id(),
//                         self.basis + ix,
//                         handle_bounds,
//                     );
//                     mouse_region = mouse_region
//                         .on_drag(
//                             MouseButton::Left,
//                             Self::handle_resize(
//                                 self.flexes.clone(),
//                                 self.axis,
//                                 ix,
//                                 child_start,
//                                 visible_bounds.clone(),
//                             ),
//                         )
//                         .on_click(MouseButton::Left, {
//                             let flexes = self.flexes.clone();
//                             move |e, v: &mut Workspace, cx| {
//                                 if e.click_count >= 2 {
//                                     let mut borrow = flexes.borrow_mut();
//                                     *borrow = vec![1.; borrow.len()];
//                                     v.schedule_serialize(cx);
//                                     cx.notify();
//                                 }
//                             }
//                         });
//                     cx.scene().push_mouse_region(mouse_region);

//                     cx.scene().pop_stacking_context();
//                 }
//             }

//             if overflowing {
//                 cx.scene().pop_layer();
//             }
//         }

//         fn rect_for_text_range(
//             &self,
//             range_utf16: Range<usize>,
//             _: Bounds<Pixels>,
//             _: Bounds<Pixels>,
//             _: &Self::LayoutState,
//             _: &Self::PaintState,
//             view: &Workspace,
//             cx: &ViewContext<Workspace>,
//         ) -> Option<Bounds<Pixels>> {
//             self.children
//                 .iter()
//                 .find_map(|child| child.rect_for_text_range(range_utf16.clone(), view, cx))
//         }

//         fn debug(
//             &self,
//             bounds: Bounds<Pixels>,
//             _: &Self::LayoutState,
//             _: &Self::PaintState,
//             view: &Workspace,
//             cx: &ViewContext<Workspace>,
//         ) -> json::Value {
//             serde_json::json!({
//                 "type": "PaneAxis",
//                 "bounds": bounds.to_json(),
//                 "axis": self.axis.to_json(),
//                 "flexes": *self.flexes.borrow(),
//                 "children": self.children.iter().map(|child| child.debug(view, cx)).collect::<Vec<json::Value>>()
//             })
//         }
//     }
// }
