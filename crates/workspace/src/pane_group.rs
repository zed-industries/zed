use crate::{pane_group::element::pane_axis, AppState, FollowerState, Pane, Workspace};
use anyhow::{anyhow, Result};
use call::{ActiveCall, ParticipantLocation};
use collections::HashMap;
use gpui::{
    point, size, AnyView, AnyWeakView, Axis, Bounds, IntoElement, Model, MouseButton, Pixels,
    Point, View, ViewContext,
};
use parking_lot::Mutex;
use project::Project;
use serde::Deserialize;
use std::sync::Arc;
use ui::prelude::*;

pub const HANDLE_HITBOX_SIZE: f32 = 4.0;
const HORIZONTAL_MIN_SIZE: f32 = 80.;
const VERTICAL_MIN_SIZE: f32 = 100.;

#[derive(Clone)]
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
        active_call: Option<&Model<ActiveCall>>,
        active_pane: &View<Pane>,
        zoomed: Option<&AnyWeakView>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> impl IntoElement {
        self.root.render(
            project,
            0,
            follower_states,
            active_call,
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

#[derive(Clone)]
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
        active_call: Option<&Model<ActiveCall>>,
        active_pane: &View<Pane>,
        zoomed: Option<&AnyWeakView>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> impl IntoElement {
        match self {
            Member::Pane(pane) => {
                if zoomed == Some(&pane.downgrade().into()) {
                    return div().into_any();
                }

                let follower_state = follower_states.get(pane);

                let leader = follower_state.and_then(|state| {
                    let room = active_call?.read(cx).room()?.read(cx);
                    room.remote_participant_for_peer_id(state.leader_id)
                });

                let is_in_unshared_view = follower_state.map_or(false, |state| {
                    state.active_view_id.is_some_and(|view_id| {
                        !state.items_by_leader_view_id.contains_key(&view_id)
                    })
                });

                let mut leader_border = None;
                let mut leader_status_box = None;
                let mut leader_join_data = None;
                if let Some(leader) = &leader {
                    let mut leader_color = cx
                        .theme()
                        .players()
                        .color_for_participant(leader.participant_index.0)
                        .cursor;
                    leader_color.fade_out(0.3);
                    leader_border = Some(leader_color);

                    leader_status_box = match leader.location {
                        ParticipantLocation::SharedProject {
                            project_id: leader_project_id,
                        } => {
                            if Some(leader_project_id) == project.read(cx).remote_id() {
                                if is_in_unshared_view {
                                    Some(Label::new(format!(
                                        "{} is in an unshared pane",
                                        leader.user.github_login
                                    )))
                                } else {
                                    None
                                }
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
                }

                div()
                    .relative()
                    .flex_1()
                    .size_full()
                    .child(AnyView::from(pane.clone()).cached())
                    .when_some(leader_border, |this, color| {
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
                    .when_some(leader_status_box, |this, status_box| {
                        this.child(
                            div()
                                .absolute()
                                .w_96()
                                .bottom_3()
                                .right_3()
                                .elevation_2(cx)
                                .p_1()
                                .z_index(1)
                                .child(status_box)
                                .when_some(
                                    leader_join_data,
                                    |this, (leader_project_id, leader_user_id)| {
                                        this.cursor_pointer().on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _, cx| {
                                                crate::join_remote_project(
                                                    leader_project_id,
                                                    leader_user_id,
                                                    this.app_state().clone(),
                                                    cx,
                                                )
                                                .detach_and_log_err(cx);
                                            }),
                                        )
                                    },
                                ),
                        )
                    })
                    .into_any()
            }
            Member::Axis(axis) => axis
                .render(
                    project,
                    basis + 1,
                    follower_states,
                    active_call,
                    active_pane,
                    zoomed,
                    app_state,
                    cx,
                )
                .into_any(),
        }
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
                if coordinates.contains(&coordinate) {
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
        active_call: Option<&Model<ActiveCall>>,
        active_pane: &View<Pane>,
        zoomed: Option<&AnyWeakView>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> gpui::AnyElement {
        debug_assert!(self.members.len() == self.flexes.lock().len());
        let mut active_pane_ix = None;

        pane_axis(
            self.axis,
            basis,
            self.flexes.clone(),
            self.bounding_boxes.clone(),
            cx.view().downgrade(),
        )
        .children(self.members.iter().enumerate().map(|(ix, member)| {
            if member.contains(active_pane) {
                active_pane_ix = Some(ix);
            }
            member
                .render(
                    project,
                    (basis + ix) * 10,
                    follower_states,
                    active_call,
                    active_pane,
                    zoomed,
                    app_state,
                    cx,
                )
                .into_any_element()
        }))
        .with_active_pane(active_pane_ix)
        .into_any_element()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
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

mod element {

    use std::{cell::RefCell, iter, rc::Rc, sync::Arc};

    use gpui::{
        px, relative, Along, AnyElement, Axis, Bounds, CursorStyle, Element, IntoElement,
        MouseDownEvent, MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, Point, Size, Style,
        WeakView, WindowContext,
    };
    use parking_lot::Mutex;
    use settings::Settings;
    use smallvec::SmallVec;
    use ui::prelude::*;
    use util::ResultExt;

    use crate::Workspace;

    use crate::WorkspaceSettings;

    use super::{HANDLE_HITBOX_SIZE, HORIZONTAL_MIN_SIZE, VERTICAL_MIN_SIZE};

    const DIVIDER_SIZE: f32 = 1.0;

    pub(super) fn pane_axis(
        axis: Axis,
        basis: usize,
        flexes: Arc<Mutex<Vec<f32>>>,
        bounding_boxes: Arc<Mutex<Vec<Option<Bounds<Pixels>>>>>,
        workspace: WeakView<Workspace>,
    ) -> PaneAxisElement {
        PaneAxisElement {
            axis,
            basis,
            flexes,
            bounding_boxes,
            children: SmallVec::new(),
            active_pane_ix: None,
            workspace,
        }
    }

    pub struct PaneAxisElement {
        axis: Axis,
        basis: usize,
        flexes: Arc<Mutex<Vec<f32>>>,
        bounding_boxes: Arc<Mutex<Vec<Option<Bounds<Pixels>>>>>,
        children: SmallVec<[AnyElement; 2]>,
        active_pane_ix: Option<usize>,
        workspace: WeakView<Workspace>,
    }

    impl PaneAxisElement {
        pub fn with_active_pane(mut self, active_pane_ix: Option<usize>) -> Self {
            self.active_pane_ix = active_pane_ix;
            self
        }

        fn compute_resize(
            flexes: &Arc<Mutex<Vec<f32>>>,
            e: &MouseMoveEvent,
            ix: usize,
            axis: Axis,
            child_start: Point<Pixels>,
            container_size: Size<Pixels>,
            workspace: WeakView<Workspace>,
            cx: &mut WindowContext,
        ) {
            let min_size = match axis {
                Axis::Horizontal => px(HORIZONTAL_MIN_SIZE),
                Axis::Vertical => px(VERTICAL_MIN_SIZE),
            };
            let mut flexes = flexes.lock();
            debug_assert!(flex_values_in_bounds(flexes.as_slice()));

            let size = move |ix, flexes: &[f32]| {
                container_size.along(axis) * (flexes[ix] / flexes.len() as f32)
            };

            // Don't allow resizing to less than the minimum size, if elements are already too small
            if min_size - px(1.) > size(ix, flexes.as_slice()) {
                return;
            }

            let mut proposed_current_pixel_change =
                (e.position - child_start).along(axis) - size(ix, flexes.as_slice());

            let flex_changes = |pixel_dx, target_ix, next: isize, flexes: &[f32]| {
                let flex_change = pixel_dx / container_size.along(axis);
                let current_target_flex = flexes[target_ix] + flex_change;
                let next_target_flex = flexes[(target_ix as isize + next) as usize] - flex_change;
                (current_target_flex, next_target_flex)
            };

            let mut successors = iter::from_fn({
                let forward = proposed_current_pixel_change > px(0.);
                let mut ix_offset = 0;
                let len = flexes.len();
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

            while proposed_current_pixel_change.abs() > px(0.) {
                let Some(current_ix) = successors.next() else {
                    break;
                };

                let next_target_size = Pixels::max(
                    size(current_ix + 1, flexes.as_slice()) - proposed_current_pixel_change,
                    min_size,
                );

                let current_target_size = Pixels::max(
                    size(current_ix, flexes.as_slice()) + size(current_ix + 1, flexes.as_slice())
                        - next_target_size,
                    min_size,
                );

                let current_pixel_change =
                    current_target_size - size(current_ix, flexes.as_slice());

                let (current_target_flex, next_target_flex) =
                    flex_changes(current_pixel_change, current_ix, 1, flexes.as_slice());

                flexes[current_ix] = current_target_flex;
                flexes[current_ix + 1] = next_target_flex;

                proposed_current_pixel_change -= current_pixel_change;
            }

            workspace
                .update(cx, |this, cx| this.schedule_serialize(cx))
                .log_err();
            cx.stop_propagation();
            cx.refresh();
        }

        fn push_handle(
            flexes: Arc<Mutex<Vec<f32>>>,
            dragged_handle: Rc<RefCell<Option<usize>>>,
            axis: Axis,
            ix: usize,
            pane_bounds: Bounds<Pixels>,
            axis_bounds: Bounds<Pixels>,
            workspace: WeakView<Workspace>,
            cx: &mut ElementContext,
        ) {
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

            cx.with_z_index(3, |cx| {
                if handle_bounds.contains(&cx.mouse_position()) {
                    let stacking_order = cx.stacking_order().clone();
                    let cursor_style = match axis {
                        Axis::Vertical => CursorStyle::ResizeUpDown,
                        Axis::Horizontal => CursorStyle::ResizeLeftRight,
                    };
                    cx.set_cursor_style(cursor_style, stacking_order);
                }

                cx.add_opaque_layer(handle_bounds);
                cx.paint_quad(gpui::fill(divider_bounds, cx.theme().colors().border));

                cx.on_mouse_event({
                    let dragged_handle = dragged_handle.clone();
                    let flexes = flexes.clone();
                    let workspace = workspace.clone();
                    move |e: &MouseDownEvent, phase, cx| {
                        if phase.bubble() && handle_bounds.contains(&e.position) {
                            dragged_handle.replace(Some(ix));
                            if e.click_count >= 2 {
                                let mut borrow = flexes.lock();
                                *borrow = vec![1.; borrow.len()];
                                workspace
                                    .update(cx, |this, cx| this.schedule_serialize(cx))
                                    .log_err();

                                cx.refresh();
                            }
                            cx.stop_propagation();
                        }
                    }
                });
                cx.on_mouse_event({
                    let workspace = workspace.clone();
                    move |e: &MouseMoveEvent, phase, cx| {
                        let dragged_handle = dragged_handle.borrow();

                        if phase.bubble() && *dragged_handle == Some(ix) {
                            Self::compute_resize(
                                &flexes,
                                e,
                                ix,
                                axis,
                                pane_bounds.origin,
                                axis_bounds.size,
                                workspace.clone(),
                                cx,
                            )
                        }
                    }
                });
            });
        }
    }

    impl IntoElement for PaneAxisElement {
        type Element = Self;

        fn element_id(&self) -> Option<ui::prelude::ElementId> {
            Some(self.basis.into())
        }

        fn into_element(self) -> Self::Element {
            self
        }
    }

    impl Element for PaneAxisElement {
        type State = Rc<RefCell<Option<usize>>>;

        fn request_layout(
            &mut self,
            state: Option<Self::State>,
            cx: &mut ui::prelude::ElementContext,
        ) -> (gpui::LayoutId, Self::State) {
            let mut style = Style::default();
            style.flex_grow = 1.;
            style.flex_shrink = 1.;
            style.flex_basis = relative(0.).into();
            style.size.width = relative(1.).into();
            style.size.height = relative(1.).into();
            let layout_id = cx.request_layout(&style, None);
            let dragged_pane = state.unwrap_or_else(|| Rc::new(RefCell::new(None)));
            (layout_id, dragged_pane)
        }

        fn paint(
            &mut self,
            bounds: gpui::Bounds<ui::prelude::Pixels>,
            state: &mut Self::State,
            cx: &mut ui::prelude::ElementContext,
        ) {
            let flexes = self.flexes.lock().clone();
            let len = self.children.len();
            debug_assert!(flexes.len() == len);
            debug_assert!(flex_values_in_bounds(flexes.as_slice()));

            let magnification_value = WorkspaceSettings::get(None, cx).active_pane_magnification;
            let active_pane_magnification = if magnification_value == 1. {
                None
            } else {
                Some(magnification_value)
            };

            let total_flex = if let Some(flex) = active_pane_magnification {
                self.children.len() as f32 - 1. + flex
            } else {
                len as f32
            };

            let mut origin = bounds.origin;
            let space_per_flex = bounds.size.along(self.axis) / total_flex;

            let mut bounding_boxes = self.bounding_boxes.lock();
            bounding_boxes.clear();

            for (ix, child) in self.children.iter_mut().enumerate() {
                let child_flex = active_pane_magnification
                    .map(|magnification| {
                        if self.active_pane_ix == Some(ix) {
                            magnification
                        } else {
                            1.
                        }
                    })
                    .unwrap_or_else(|| flexes[ix]);

                let child_size = bounds
                    .size
                    .apply_along(self.axis, |_| space_per_flex * child_flex)
                    .map(|d| d.round());

                let child_bounds = Bounds {
                    origin,
                    size: child_size,
                };
                bounding_boxes.push(Some(child_bounds));
                cx.with_z_index(0, |cx| {
                    child.draw(origin, child_size.into(), cx);
                });

                if active_pane_magnification.is_none() {
                    cx.with_z_index(1, |cx| {
                        if ix < len - 1 {
                            Self::push_handle(
                                self.flexes.clone(),
                                state.clone(),
                                self.axis,
                                ix,
                                child_bounds,
                                bounds,
                                self.workspace.clone(),
                                cx,
                            );
                        }
                    });
                }

                origin = origin.apply_along(self.axis, |val| val + child_size.along(self.axis));
            }

            cx.with_z_index(1, |cx| {
                cx.on_mouse_event({
                    let state = state.clone();
                    move |_: &MouseUpEvent, phase, _cx| {
                        if phase.bubble() {
                            state.replace(None);
                        }
                    }
                });
            })
        }
    }

    impl ParentElement for PaneAxisElement {
        fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
            self.children.extend(elements)
        }
    }

    fn flex_values_in_bounds(flexes: &[f32]) -> bool {
        (flexes.iter().copied().sum::<f32>() - flexes.len() as f32).abs() < 0.001
    }
}
