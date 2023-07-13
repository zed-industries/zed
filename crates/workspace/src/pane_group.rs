use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{AppState, FollowerStatesByLeader, Pane, Workspace};
use anyhow::{anyhow, Result};
use call::{ActiveCall, ParticipantLocation};
use gpui::{
    elements::*,
    geometry::{rect::RectF, vector::Vector2F},
    platform::{CursorStyle, MouseButton},
    AnyViewHandle, Axis, Border, ModelHandle, ViewContext, ViewHandle,
};
use project::Project;
use serde::Deserialize;
use theme::Theme;

use self::adjustable_group::{AdjustableGroupElement, AdjustableGroupItem};

#[derive(Clone, Debug, PartialEq)]
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
        zoomed: Option<&AnyViewHandle>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> AnyElement<Workspace> {
        self.root.render(
            project,
            0,
            theme,
            follower_states,
            active_call,
            active_pane,
            zoomed,
            app_state,
            cx,
        )
    }

    pub(crate) fn panes(&self) -> Vec<&ViewHandle<Pane>> {
        let mut panes = Vec::new();
        self.root.collect_panes(&mut panes);
        panes
    }
}

#[derive(Clone, Debug, PartialEq)]
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

        Member::Axis(PaneAxis {
            axis,
            members,
            ratios: Default::default(),
        })
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
        basis: usize,
        theme: &Theme,
        follower_states: &FollowerStatesByLeader,
        active_call: Option<&ModelHandle<ActiveCall>>,
        active_pane: &ViewHandle<Pane>,
        zoomed: Option<&AnyViewHandle>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> AnyElement<Workspace> {
        enum FollowIntoExternalProject {}

        match self {
            Member::Pane(pane) => {
                let pane_element = if Some(&**pane) == zoomed {
                    Empty::new().into_any()
                } else {
                    ChildView::new(pane, cx).into_any()
                };

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

                let leader_status_box = if let Some((_, leader)) = leader {
                    match leader.location {
                        ParticipantLocation::SharedProject {
                            project_id: leader_project_id,
                        } => {
                            if Some(leader_project_id) == project.read(cx).remote_id() {
                                None
                            } else {
                                let leader_user = leader.user.clone();
                                let leader_user_id = leader.user.id;
                                let app_state = Arc::downgrade(app_state);
                                Some(
                                    MouseEventHandler::<FollowIntoExternalProject, _>::new(
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
                                        },
                                    )
                                    .with_cursor_style(CursorStyle::PointingHand)
                                    .on_click(MouseButton::Left, move |_, _, cx| {
                                        if let Some(app_state) = app_state.upgrade() {
                                            crate::join_remote_project(
                                                leader_project_id,
                                                leader_user_id,
                                                app_state,
                                                cx,
                                            )
                                            .detach_and_log_err(cx);
                                        }
                                    })
                                    .aligned()
                                    .bottom()
                                    .right()
                                    .into_any(),
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
                            .into_any(),
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
                            .into_any(),
                        ),
                    }
                } else {
                    None
                };

                Stack::new()
                    .with_child(pane_element.contained().with_border(border))
                    .with_children(leader_status_box)
                    .into_any()
            }
            Member::Axis(axis) => axis.render(
                project,
                basis + 1,
                theme,
                follower_states,
                active_call,
                active_pane,
                zoomed,
                app_state,
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PaneAxis {
    pub axis: Axis,
    pub members: Vec<Member>,
    ratios: Rc<RefCell<Vec<f32>>>,
}

impl PaneAxis {
    pub fn new(axis: Axis, members: Vec<Member>) -> Self {
        let ratios = Rc::new(RefCell::new(vec![1.; members.len()]));
        Self {
            axis,
            members,
            ratios,
        }
    }

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
        basis: usize,
        theme: &Theme,
        follower_state: &FollowerStatesByLeader,
        active_call: Option<&ModelHandle<ActiveCall>>,
        active_pane: &ViewHandle<Pane>,
        zoomed: Option<&AnyViewHandle>,
        app_state: &Arc<AppState>,
        cx: &mut ViewContext<Workspace>,
    ) -> AnyElement<Workspace> {
        let ratios = self.ratios.clone();
        let mut flex_container =
            AdjustableGroupElement::new(self.axis, 2., basis, move |new_flexes, _, cx| {
                let mut borrow = ratios.borrow_mut();
                for (ix, flex) in new_flexes {
                    if let Some(el) = borrow.get_mut(ix) {
                        *el = flex;
                    }
                }

                cx.notify();
            });

        let ratios_borrow = self.ratios.borrow();
        let next_basis = basis + self.members.len();
        let mut members = self.members.iter().zip(ratios_borrow.iter()).peekable();
        while let Some((member, flex)) = members.next() {
            let last = members.peek().is_none();

            // TODO: Restore this
            // if member.contains(active_pane) {
            // flex = settings::get::<WorkspaceSettings>(cx).active_pane_magnification;
            // }

            let mut member = member.render(
                project,
                next_basis,
                theme,
                follower_state,
                active_call,
                active_pane,
                zoomed,
                app_state,
                cx,
            );
            if !last {
                let mut border = theme.workspace.pane_divider;
                border.left = false;
                border.right = false;
                border.top = false;
                border.bottom = false;

                match self.axis {
                    Axis::Vertical => border.bottom = true,
                    Axis::Horizontal => border.right = true,
                }

                member = member.contained().with_border(border).into_any();
            }

            flex_container =
                flex_container.with_child(AdjustableGroupItem::new(member, *flex).into_any());
        }

        flex_container.into_any()
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

mod adjustable_group {

    use std::{any::Any, ops::Range, rc::Rc};

    use gpui::{
        color::Color,
        geometry::{
            rect::RectF,
            vector::{vec2f, Vector2F},
        },
        json::{self, ToJson},
        platform::{CursorStyle, MouseButton},
        AnyElement, Axis, CursorRegion, Element, EventContext, LayoutContext, MouseRegion, Quad,
        RectFExt, SceneBuilder, SizeConstraint, Vector2FExt, View, ViewContext,
    };
    use serde_json::Value;
    use smallvec::SmallVec;

    struct AdjustableFlexData {
        flex: f32,
    }

    pub struct AdjustableGroupElement<V: View> {
        axis: Axis,
        handle_size: f32,
        basis: usize,
        callback: Rc<dyn Fn(SmallVec<[(usize, f32); 2]>, &mut V, &mut EventContext<V>)>,
        children: Vec<AnyElement<V>>,
    }

    impl<V: View> AdjustableGroupElement<V> {
        pub fn new(
            axis: Axis,
            handle_size: f32,
            basis: usize,
            callback: impl Fn(SmallVec<[(usize, f32); 2]>, &mut V, &mut EventContext<V>) + 'static,
        ) -> Self {
            Self {
                axis,
                handle_size,
                basis,
                callback: Rc::new(callback),
                children: Default::default(),
            }
        }

        fn layout_flex_children(
            &mut self,
            constraint: SizeConstraint,
            remaining_space: &mut f32,
            remaining_flex: &mut f32,
            cross_axis_max: &mut f32,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) {
            let cross_axis = self.axis.invert();
            let last_ix = self.children.len() - 1;
            for (ix, child) in self.children.iter_mut().enumerate() {
                let flex = child.metadata::<AdjustableFlexData>().unwrap().flex;

                let handle_size = if ix == last_ix { 0. } else { self.handle_size };

                let child_size = if *remaining_flex == 0.0 {
                    *remaining_space
                } else {
                    let space_per_flex = *remaining_space / *remaining_flex;
                    space_per_flex * flex
                } - handle_size;

                let child_constraint = match self.axis {
                    Axis::Horizontal => SizeConstraint::new(
                        vec2f(child_size, constraint.min.y()),
                        vec2f(child_size, constraint.max.y()),
                    ),
                    Axis::Vertical => SizeConstraint::new(
                        vec2f(constraint.min.x(), child_size),
                        vec2f(constraint.max.x(), child_size),
                    ),
                };
                let child_size = child.layout(child_constraint, view, cx);
                *remaining_space -= child_size.along(self.axis) + handle_size;
                *remaining_flex -= flex;
                *cross_axis_max = cross_axis_max.max(child_size.along(cross_axis));
            }
        }
    }

    impl<V: View> Extend<AnyElement<V>> for AdjustableGroupElement<V> {
        fn extend<T: IntoIterator<Item = AnyElement<V>>>(&mut self, children: T) {
            self.children.extend(children);
        }
    }

    impl<V: View> Element<V> for AdjustableGroupElement<V> {
        type LayoutState = f32;
        type PaintState = ();

        fn layout(
            &mut self,
            constraint: SizeConstraint,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) -> (Vector2F, Self::LayoutState) {
            let mut remaining_flex = 0.;

            let mut cross_axis_max: f32 = 0.0;
            for child in &mut self.children {
                let metadata = child.metadata::<AdjustableFlexData>();
                let flex = metadata
                    .map(|metadata| metadata.flex)
                    .expect("All children of an adjustable flex must be AdjustableFlexItems");
                remaining_flex += flex;
            }

            let mut remaining_space = constraint.max_along(self.axis);

            if remaining_space.is_infinite() {
                panic!("flex contains flexible children but has an infinite constraint along the flex axis");
            }

            self.layout_flex_children(
                constraint,
                &mut remaining_space,
                &mut remaining_flex,
                &mut cross_axis_max,
                view,
                cx,
            );

            let mut size = match self.axis {
                Axis::Horizontal => vec2f(constraint.max.x() - remaining_space, cross_axis_max),
                Axis::Vertical => vec2f(cross_axis_max, constraint.max.y() - remaining_space),
            };

            if constraint.min.x().is_finite() {
                size.set_x(size.x().max(constraint.min.x()));
            }
            if constraint.min.y().is_finite() {
                size.set_y(size.y().max(constraint.min.y()));
            }

            if size.x() > constraint.max.x() {
                size.set_x(constraint.max.x());
            }
            if size.y() > constraint.max.y() {
                size.set_y(constraint.max.y());
            }

            (size, remaining_space)
        }

        fn paint(
            &mut self,
            scene: &mut SceneBuilder,
            bounds: RectF,
            visible_bounds: RectF,
            remaining_space: &mut Self::LayoutState,
            view: &mut V,
            cx: &mut ViewContext<V>,
        ) -> Self::PaintState {
            let visible_bounds = bounds.intersection(visible_bounds).unwrap_or_default();

            let overflowing = *remaining_space < 0.;
            if overflowing {
                scene.push_layer(Some(visible_bounds));
            }

            let mut child_origin = bounds.origin();

            let mut children_iter = self.children.iter_mut().enumerate().peekable();
            while let Some((ix, child)) = children_iter.next() {
                let child_start = child_origin.clone();
                child.paint(scene, child_origin, visible_bounds, view, cx);

                match self.axis {
                    Axis::Horizontal => child_origin += vec2f(child.size().x(), 0.0),
                    Axis::Vertical => child_origin += vec2f(0.0, child.size().y()),
                }

                if let Some((next_ix, next_child)) = children_iter.peek() {
                    let bounds = match self.axis {
                        Axis::Horizontal => RectF::new(
                            child_origin,
                            vec2f(self.handle_size, visible_bounds.height()),
                        ),
                        Axis::Vertical => RectF::new(
                            child_origin,
                            vec2f(visible_bounds.width(), self.handle_size),
                        ),
                    };

                    scene.push_quad(Quad {
                        bounds,
                        background: Some(Color::red()),
                        ..Default::default()
                    });

                    let style = match self.axis {
                        Axis::Horizontal => CursorStyle::ResizeLeftRight,
                        Axis::Vertical => CursorStyle::ResizeUpDown,
                    };

                    scene.push_cursor_region(CursorRegion { bounds, style });

                    let callback = self.callback.clone();
                    let axis = self.axis;
                    let child_size = child.size();
                    let next_child_size = next_child.size();
                    let mut drag_bounds = visible_bounds.clone();
                    // Unsure why this should be needed....
                    drag_bounds.set_origin_y(0.);
                    let current_flex = child.metadata::<AdjustableFlexData>().unwrap().flex;
                    let next_flex = next_child.metadata::<AdjustableFlexData>().unwrap().flex;
                    let next_ix = *next_ix;
                    const HORIZONTAL_MIN_SIZE: f32 = 80.;
                    const VERTICAL_MIN_SIZE: f32 = 100.;
                    enum ResizeHandle {}
                    let mut mouse_region =
                        MouseRegion::new::<ResizeHandle>(cx.view_id(), self.basis + ix, bounds);
                    mouse_region =
                        mouse_region.on_drag(MouseButton::Left, move |drag, v: &mut V, cx| {
                            let min_size = match axis {
                                Axis::Horizontal => HORIZONTAL_MIN_SIZE,
                                Axis::Vertical => VERTICAL_MIN_SIZE,
                            };
                            // Don't allow resizing to less than the minimum size, if elements are already too small
                            if min_size - 1. > child_size.along(axis)
                                || min_size - 1. > next_child_size.along(axis)
                            {
                                return;
                            }

                            let flex_position = drag.position - drag_bounds.origin();
                            let mut current_target_size = (flex_position - child_start).along(axis);
                            let proposed_current_pixel_change =
                                current_target_size - child_size.along(axis);

                            if proposed_current_pixel_change < 0. {
                                current_target_size = current_target_size.max(min_size);
                            } else if proposed_current_pixel_change > 0. {
                                // TODO: cascade this size change down, collect into a vec
                                let next_target_size = (next_child_size.along(axis)
                                    - proposed_current_pixel_change)
                                    .max(min_size);
                                current_target_size = current_target_size.min(
                                    child_size.along(axis) + next_child_size.along(axis)
                                        - next_target_size,
                                );
                            }

                            let current_pixel_change = current_target_size - child_size.along(axis);
                            let flex_change = current_pixel_change / drag_bounds.length_along(axis);

                            let current_target_flex = current_flex + flex_change;
                            let next_target_flex = next_flex - flex_change;

                            callback(
                                smallvec::smallvec![
                                    (ix, current_target_flex),
                                    (next_ix, next_target_flex),
                                ],
                                v,
                                cx,
                            )
                        });
                    scene.push_mouse_region(mouse_region);

                    match self.axis {
                        Axis::Horizontal => child_origin += vec2f(self.handle_size, 0.0),
                        Axis::Vertical => child_origin += vec2f(0.0, self.handle_size),
                    }
                }
            }

            if overflowing {
                scene.pop_layer();
            }
        }

        fn rect_for_text_range(
            &self,
            range_utf16: Range<usize>,
            _: RectF,
            _: RectF,
            _: &Self::LayoutState,
            _: &Self::PaintState,
            view: &V,
            cx: &ViewContext<V>,
        ) -> Option<RectF> {
            self.children
                .iter()
                .find_map(|child| child.rect_for_text_range(range_utf16.clone(), view, cx))
        }

        fn debug(
            &self,
            bounds: RectF,
            _: &Self::LayoutState,
            _: &Self::PaintState,
            view: &V,
            cx: &ViewContext<V>,
        ) -> json::Value {
            serde_json::json!({
                "type": "Flex",
                "bounds": bounds.to_json(),
                "axis": self.axis.to_json(),
                "children": self.children.iter().map(|child| child.debug(view, cx)).collect::<Vec<json::Value>>()
            })
        }
    }

    pub struct AdjustableGroupItem<V: View> {
        metadata: AdjustableFlexData,
        child: AnyElement<V>,
    }

    impl<V: View> AdjustableGroupItem<V> {
        pub fn new(child: impl Element<V>, flex: f32) -> Self {
            Self {
                metadata: AdjustableFlexData { flex },
                child: child.into_any(),
            }
        }
    }

    impl<V: View> Element<V> for AdjustableGroupItem<V> {
        type LayoutState = ();
        type PaintState = ();

        fn layout(
            &mut self,
            constraint: SizeConstraint,
            view: &mut V,
            cx: &mut LayoutContext<V>,
        ) -> (Vector2F, Self::LayoutState) {
            let size = self.child.layout(constraint, view, cx);
            (size, ())
        }

        fn paint(
            &mut self,
            scene: &mut SceneBuilder,
            bounds: RectF,
            visible_bounds: RectF,
            _: &mut Self::LayoutState,
            view: &mut V,
            cx: &mut ViewContext<V>,
        ) -> Self::PaintState {
            self.child
                .paint(scene, bounds.origin(), visible_bounds, view, cx)
        }

        fn rect_for_text_range(
            &self,
            range_utf16: Range<usize>,
            _: RectF,
            _: RectF,
            _: &Self::LayoutState,
            _: &Self::PaintState,
            view: &V,
            cx: &ViewContext<V>,
        ) -> Option<RectF> {
            self.child.rect_for_text_range(range_utf16, view, cx)
        }

        fn metadata(&self) -> Option<&dyn Any> {
            Some(&self.metadata)
        }

        fn debug(
            &self,
            _: RectF,
            _: &Self::LayoutState,
            _: &Self::PaintState,
            view: &V,
            cx: &ViewContext<V>,
        ) -> Value {
            serde_json::json!({
                "type": "Flexible",
                "flex": self.metadata.flex,
                "child": self.child.debug(view, cx)
            })
        }
    }
}
