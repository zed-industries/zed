//! Single workspace view over shared worktrees.
//!
//! This module contains the `Workspace` struct which represents a single workspace's
//! view over the shared `WorktreeStore`. Multiple `Workspace` instances can exist
//! within a `MultiWorkspace`, each with their own subset of worktrees.

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, atomic::AtomicUsize},
};

use collections::{HashMap, HashSet};
use gpui::{
    AnyView, AnyWeakView, App, Entity, EntityId, IntoElement, Pixels, Point, Subscription,
    WeakEntity, Window,
};
use project::WorktreeId;

use client::proto;

use crate::{
    dock::{Dock, DockPosition},
    pane::Pane,
    pane_group::{PaneGroup, PaneLeaderDecorator},
    status_bar::StatusBar,
    DispatchingKeystrokes,
};

pub struct Workspace {
    worktree_ids: HashSet<WorktreeId>,
    center: PaneGroup,
    panes: Vec<Entity<Pane>>,
    active_pane: Entity<Pane>,
    panes_by_item: HashMap<EntityId, WeakEntity<Pane>>,
    last_active_center_pane: Option<WeakEntity<Pane>>,
    pane_history_timestamp: Arc<AtomicUsize>,
    // Docks
    left_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    right_dock: Entity<Dock>,
    last_open_dock_positions: Vec<DockPosition>,
    previous_dock_drag_coordinates: Option<Point<Pixels>>,
    // Zoom state
    zoomed: Option<AnyWeakView>,
    zoomed_position: Option<DockPosition>,
    // UI state
    status_bar: Entity<StatusBar>,
    titlebar_item: Option<AnyView>,
    pub centered_layout: bool,
    // Worktree state
    active_worktree_override: Option<WorktreeId>,
    // Item tracking
    last_active_view_id: Option<proto::ViewId>,
    dirty_items: HashMap<EntityId, Subscription>,
    // Input handling
    dispatching_keystrokes: Rc<RefCell<DispatchingKeystrokes>>,
}

impl Workspace {
    pub fn new(
        center_pane: Entity<Pane>,
        pane_history_timestamp: Arc<AtomicUsize>,
        left_dock: Entity<Dock>,
        bottom_dock: Entity<Dock>,
        right_dock: Entity<Dock>,
        status_bar: Entity<StatusBar>,
    ) -> Self {
        let mut center = PaneGroup::new(center_pane.clone());
        center.set_is_center(true);

        Self {
            worktree_ids: HashSet::default(),
            center,
            panes: vec![center_pane.clone()],
            active_pane: center_pane.clone(),
            panes_by_item: HashMap::default(),
            last_active_center_pane: Some(center_pane.downgrade()),
            pane_history_timestamp,
            left_dock,
            bottom_dock,
            right_dock,
            last_open_dock_positions: Vec::new(),
            previous_dock_drag_coordinates: None,
            zoomed: None,
            zoomed_position: None,
            status_bar,
            titlebar_item: None,
            centered_layout: false,
            active_worktree_override: None,
            last_active_view_id: None,
            dirty_items: HashMap::default(),
            dispatching_keystrokes: Default::default(),
        }
    }

    pub fn worktree_ids(&self) -> &HashSet<WorktreeId> {
        &self.worktree_ids
    }

    pub fn add_worktree(&mut self, worktree_id: WorktreeId) {
        self.worktree_ids.insert(worktree_id);
    }

    pub fn remove_worktree(&mut self, worktree_id: WorktreeId) {
        self.worktree_ids.remove(&worktree_id);
    }

    pub fn contains_worktree(&self, worktree_id: WorktreeId) -> bool {
        self.worktree_ids.contains(&worktree_id)
    }

    pub fn center(&self) -> &PaneGroup {
        &self.center
    }

    pub fn center_mut(&mut self) -> &mut PaneGroup {
        &mut self.center
    }

    pub fn panes(&self) -> &[Entity<Pane>] {
        &self.panes
    }

    pub fn panes_mut(&mut self) -> &mut Vec<Entity<Pane>> {
        &mut self.panes
    }

    pub fn active_pane(&self) -> &Entity<Pane> {
        &self.active_pane
    }

    pub fn set_active_pane(&mut self, pane: Entity<Pane>) {
        self.active_pane = pane;
    }

    pub fn panes_by_item(&self) -> &HashMap<EntityId, WeakEntity<Pane>> {
        &self.panes_by_item
    }

    pub fn panes_by_item_mut(&mut self) -> &mut HashMap<EntityId, WeakEntity<Pane>> {
        &mut self.panes_by_item
    }

    pub fn last_active_center_pane(&self) -> Option<&WeakEntity<Pane>> {
        self.last_active_center_pane.as_ref()
    }

    pub fn set_last_active_center_pane(&mut self, pane: Option<WeakEntity<Pane>>) {
        self.last_active_center_pane = pane;
    }

    pub fn pane_history_timestamp(&self) -> &Arc<AtomicUsize> {
        &self.pane_history_timestamp
    }

    // Dock accessors
    pub fn left_dock(&self) -> &Entity<Dock> {
        &self.left_dock
    }

    pub fn bottom_dock(&self) -> &Entity<Dock> {
        &self.bottom_dock
    }

    pub fn right_dock(&self) -> &Entity<Dock> {
        &self.right_dock
    }

    pub fn docks(&self) -> [&Entity<Dock>; 3] {
        [&self.left_dock, &self.bottom_dock, &self.right_dock]
    }

    pub fn dock_at_position(&self, position: DockPosition) -> &Entity<Dock> {
        match position {
            DockPosition::Left => &self.left_dock,
            DockPosition::Bottom => &self.bottom_dock,
            DockPosition::Right => &self.right_dock,
        }
    }

    // Zoom accessors
    pub fn zoomed(&self) -> Option<&AnyWeakView> {
        self.zoomed.as_ref()
    }

    pub fn set_zoomed(&mut self, zoomed: Option<AnyWeakView>) {
        self.zoomed = zoomed;
    }

    pub fn zoomed_position(&self) -> Option<DockPosition> {
        self.zoomed_position
    }

    pub fn set_zoomed_position(&mut self, position: Option<DockPosition>) {
        self.zoomed_position = position;
    }

    pub fn render_center(
        &self,
        zoomed: Option<&AnyWeakView>,
        render_cx: &dyn PaneLeaderDecorator,
        window: &mut Window,
        cx: &mut App,
    ) -> impl IntoElement {
        self.center.render(zoomed, render_cx, window, cx)
    }

    pub fn center_pane_count(&self) -> usize {
        self.center.panes().len()
    }

    // Dock position tracking
    pub fn last_open_dock_positions(&self) -> &[DockPosition] {
        &self.last_open_dock_positions
    }

    pub fn last_open_dock_positions_mut(&mut self) -> &mut Vec<DockPosition> {
        &mut self.last_open_dock_positions
    }

    pub fn previous_dock_drag_coordinates(&self) -> Option<Point<Pixels>> {
        self.previous_dock_drag_coordinates
    }

    pub fn set_previous_dock_drag_coordinates(&mut self, coords: Option<Point<Pixels>>) {
        self.previous_dock_drag_coordinates = coords;
    }

    // Status bar
    pub fn status_bar(&self) -> &Entity<StatusBar> {
        &self.status_bar
    }

    // Titlebar
    pub fn titlebar_item(&self) -> Option<&AnyView> {
        self.titlebar_item.as_ref()
    }

    pub fn set_titlebar_item(&mut self, item: Option<AnyView>) {
        self.titlebar_item = item;
    }

    // Worktree override
    pub fn active_worktree_override(&self) -> Option<WorktreeId> {
        self.active_worktree_override
    }

    pub fn set_active_worktree_override(&mut self, worktree_id: Option<WorktreeId>) {
        self.active_worktree_override = worktree_id;
    }

    // View tracking
    pub fn last_active_view_id(&self) -> Option<proto::ViewId> {
        self.last_active_view_id.clone()
    }

    pub fn set_last_active_view_id(&mut self, id: Option<proto::ViewId>) {
        self.last_active_view_id = id;
    }

    // Dirty items
    pub fn dirty_items(&self) -> &HashMap<EntityId, Subscription> {
        &self.dirty_items
    }

    pub fn dirty_items_mut(&mut self) -> &mut HashMap<EntityId, Subscription> {
        &mut self.dirty_items
    }

    // Input handling
    pub(crate) fn dispatching_keystrokes(&self) -> &Rc<RefCell<DispatchingKeystrokes>> {
        &self.dispatching_keystrokes
    }
}
