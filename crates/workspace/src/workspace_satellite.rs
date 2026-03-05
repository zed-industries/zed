use std::collections::HashMap;

use gpui::{
    AnyWeakView, App, Context, Entity, EntityId, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, Render, Styled, WeakEntity, Window,
};
use theme::ActiveTheme;
use ui::prelude::*;

use crate::{
    Workspace, client_side_decorations,
    pane::{self, Pane},
    pane_group::{ActivePaneDecorator, PaneGroup},
};

pub enum ItemOrigin {
    CenterPane,
    DockPanel { panel_id: EntityId },
}

pub enum Event {
    Closing,
    ReattachRequested {
        item_id: EntityId,
        origin: Option<ItemOrigin>,
    },
}

impl EventEmitter<Event> for WorkspaceSatellite {}

pub struct WorkspaceSatellite {
    pub(crate) center: PaneGroup,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    item_origins: HashMap<EntityId, ItemOrigin>,
}

impl WorkspaceSatellite {
    pub fn new(
        pane: Entity<Pane>,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        cx.subscribe_in(&pane, window, Self::handle_pane_event)
            .detach();

        pane.update(cx, |pane, _| {
            pane.in_satellite = true;
        });

        let mut center = PaneGroup::new(pane);
        center.set_is_center(false);
        center.mark_positions(cx);

        Self {
            center,
            workspace,
            focus_handle,
            item_origins: HashMap::default(),
        }
    }

    pub fn root(&self) -> Entity<Pane> {
        self.center.first_pane()
    }

    pub fn record_item_origin(&mut self, item_id: EntityId, origin: ItemOrigin) {
        self.item_origins.insert(item_id, origin);
    }

    pub fn take_item_origin(&mut self, item_id: EntityId) -> Option<ItemOrigin> {
        self.item_origins.remove(&item_id)
    }

    fn handle_pane_event(
        &mut self,
        pane: &Entity<Pane>,
        event: &pane::Event,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            pane::Event::Remove { .. } => {
                cx.emit(Event::Closing);
                window.remove_window();
            }
            pane::Event::AddItem { item } => {
                if let Some(workspace) = self.workspace.upgrade() {
                    let item = item.boxed_clone();
                    let pane = pane.clone();
                    workspace.update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, pane, window, cx);
                    });
                }
            }
            pane::Event::RemovedItem { item } => {
                if let Some(workspace) = self.workspace.upgrade() {
                    let item_id = item.item_id();
                    let pane_entity_id = pane.entity_id();
                    workspace.update(cx, |workspace, _cx| {
                        if let std::collections::hash_map::Entry::Occupied(entry) =
                            workspace.panes_by_item.entry(item_id)
                        {
                            if entry.get().entity_id() == pane_entity_id {
                                entry.remove();
                            }
                        }
                    });
                }
            }
            pane::Event::ActivateItem { .. } => {}
            _ => {}
        }
    }
}

impl Focusable for WorkspaceSatellite {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl WorkspaceSatellite {
    fn reattach_active_item(
        &mut self,
        _: &crate::ReattachActiveItem,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let root = self.root();
        let Some(active_item) = root.read(cx).active_item() else {
            return;
        };
        let item_id = active_item.item_id();
        let origin = self.take_item_origin(item_id);

        cx.emit(Event::ReattachRequested { item_id, origin });
    }
}

impl Render for WorkspaceSatellite {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title_bar_bg = cx.theme().colors().title_bar_background;
        let title_bar_height = ui::utils::platform_title_bar_height(window);

        let reattach_listener = cx.listener(Self::reattach_active_item);
        let root_pane = self.root();
        let render_cx = ActivePaneDecorator::new(&root_pane, &self.workspace);
        let zoomed: Option<&AnyWeakView> = None;

        let content = self.center.render(zoomed, &render_cx, window, cx);

        client_side_decorations(
            div()
                .id("workspace-satellite")
                .track_focus(&self.focus_handle)
                .on_action(reattach_listener)
                .flex()
                .flex_col()
                .size_full()
                .bg(title_bar_bg)
                .child(div().h(title_bar_height).w_full())
                .child(content),
            window,
            cx,
            gpui::Tiling::default(),
        )
    }
}
