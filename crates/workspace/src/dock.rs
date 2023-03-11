mod toggle_dock_button;

use serde::Deserialize;

use collections::HashMap;
use gpui::{
    actions,
    elements::{ChildView, Container, Empty, MouseEventHandler, ParentElement, Side, Stack},
    geometry::vector::Vector2F,
    impl_internal_actions, Border, CursorStyle, Element, ElementBox, MouseButton,
    MutableAppContext, RenderContext, SizeConstraint, ViewContext, ViewHandle,
};
use settings::{DockAnchor, Settings};
use theme::Theme;

use crate::{sidebar::SidebarSide, BackgroundActions, ItemHandle, Pane, Workspace};
pub use toggle_dock_button::ToggleDockButton;

#[derive(PartialEq, Clone, Deserialize)]
pub struct MoveDock(pub DockAnchor);

#[derive(PartialEq, Clone)]
pub struct AddDefaultItemToDock;

actions!(
    dock,
    [
        FocusDock,
        HideDock,
        AnchorDockRight,
        AnchorDockBottom,
        ExpandDock,
        AddTabToDock,
        RemoveTabFromDock,
    ]
);
impl_internal_actions!(dock, [MoveDock, AddDefaultItemToDock]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Dock::focus_dock);
    cx.add_action(Dock::hide_dock);
    cx.add_action(
        |workspace: &mut Workspace, &MoveDock(dock_anchor), cx: &mut ViewContext<Workspace>| {
            Dock::move_dock(workspace, dock_anchor, true, cx);
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &AnchorDockRight, cx: &mut ViewContext<Workspace>| {
            Dock::move_dock(workspace, DockAnchor::Right, true, cx);
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &AnchorDockBottom, cx: &mut ViewContext<Workspace>| {
            Dock::move_dock(workspace, DockAnchor::Bottom, true, cx)
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &ExpandDock, cx: &mut ViewContext<Workspace>| {
            Dock::move_dock(workspace, DockAnchor::Expanded, true, cx)
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &AddTabToDock, cx: &mut ViewContext<Workspace>| {
            if let Some(active_item) = workspace.active_item(cx) {
                let item_id = active_item.id();

                let from = workspace.active_pane();
                let to = workspace.dock_pane();
                if from.id() == to.id() {
                    return;
                }

                let destination_index = to.read(cx).items_len() + 1;

                Pane::move_item(
                    workspace,
                    from.clone(),
                    to.clone(),
                    item_id,
                    destination_index,
                    cx,
                );
            }
        },
    );
    cx.add_action(
        |workspace: &mut Workspace, _: &RemoveTabFromDock, cx: &mut ViewContext<Workspace>| {
            if let Some(active_item) = workspace.active_item(cx) {
                let item_id = active_item.id();

                let from = workspace.dock_pane();
                let to = workspace
                    .last_active_center_pane
                    .as_ref()
                    .and_then(|pane| pane.upgrade(cx))
                    .unwrap_or_else(|| {
                        workspace
                            .panes
                            .first()
                            .expect("There must be a pane")
                            .clone()
                    });

                if from.id() == to.id() {
                    return;
                }

                let destination_index = to.read(cx).items_len() + 1;

                Pane::move_item(
                    workspace,
                    from.clone(),
                    to.clone(),
                    item_id,
                    destination_index,
                    cx,
                );
            }
        },
    );
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum DockPosition {
    Shown(DockAnchor),
    Hidden(DockAnchor),
}

impl Default for DockPosition {
    fn default() -> Self {
        DockPosition::Hidden(Default::default())
    }
}

pub fn icon_for_dock_anchor(anchor: DockAnchor) -> &'static str {
    match anchor {
        DockAnchor::Right => "icons/dock_right_12.svg",
        DockAnchor::Bottom => "icons/dock_bottom_12.svg",
        DockAnchor::Expanded => "icons/dock_modal_12.svg",
    }
}

impl DockPosition {
    pub fn is_visible(&self) -> bool {
        match self {
            DockPosition::Shown(_) => true,
            DockPosition::Hidden(_) => false,
        }
    }

    pub fn anchor(&self) -> DockAnchor {
        match self {
            DockPosition::Shown(anchor) | DockPosition::Hidden(anchor) => *anchor,
        }
    }

    fn hide(self) -> Self {
        match self {
            DockPosition::Shown(anchor) => DockPosition::Hidden(anchor),
            DockPosition::Hidden(_) => self,
        }
    }

    fn show(self) -> Self {
        match self {
            DockPosition::Hidden(anchor) => DockPosition::Shown(anchor),
            DockPosition::Shown(_) => self,
        }
    }
}

pub type DockDefaultItemFactory =
    fn(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> Option<Box<dyn ItemHandle>>;

pub struct Dock {
    position: DockPosition,
    panel_sizes: HashMap<DockAnchor, f32>,
    pane: ViewHandle<Pane>,
    default_item_factory: DockDefaultItemFactory,
}

impl Dock {
    pub fn new(
        workspace_id: usize,
        default_item_factory: DockDefaultItemFactory,
        background_actions: BackgroundActions,
        cx: &mut ViewContext<Workspace>,
    ) -> Self {
        let position = DockPosition::Hidden(cx.global::<Settings>().default_dock_anchor);

        let pane = cx.add_view(|cx| {
            Pane::new(
                workspace_id,
                Some(position.anchor()),
                background_actions,
                cx,
            )
        });
        pane.update(cx, |pane, cx| {
            pane.set_active(false, cx);
        });
        let pane_id = pane.id();
        cx.subscribe(&pane, move |workspace, _, event, cx| {
            workspace.handle_pane_event(pane_id, event, cx);
        })
        .detach();

        Self {
            pane,
            panel_sizes: Default::default(),
            position,
            default_item_factory,
        }
    }

    pub fn pane(&self) -> &ViewHandle<Pane> {
        &self.pane
    }

    pub fn visible_pane(&self) -> Option<&ViewHandle<Pane>> {
        self.position.is_visible().then(|| self.pane())
    }

    pub fn is_anchored_at(&self, anchor: DockAnchor) -> bool {
        self.position.is_visible() && self.position.anchor() == anchor
    }

    pub(crate) fn set_dock_position(
        workspace: &mut Workspace,
        new_position: DockPosition,
        focus: bool,
        cx: &mut ViewContext<Workspace>,
    ) {
        workspace.dock.position = new_position;
        // Tell the pane about the new anchor position
        workspace.dock.pane.update(cx, |pane, cx| {
            pane.set_docked(Some(new_position.anchor()), cx)
        });

        if workspace.dock.position.is_visible() {
            // Close the right sidebar if the dock is on the right side and the right sidebar is open
            if workspace.dock.position.anchor() == DockAnchor::Right {
                if workspace.right_sidebar().read(cx).is_open() {
                    workspace.toggle_sidebar(SidebarSide::Right, cx);
                }
            }

            // Ensure that the pane has at least one item or construct a default item to put in it
            let pane = workspace.dock.pane.clone();
            if pane.read(cx).items().next().is_none() {
                if let Some(item_to_add) = (workspace.dock.default_item_factory)(workspace, cx) {
                    Pane::add_item(workspace, &pane, item_to_add, focus, focus, None, cx);
                } else {
                    workspace.dock.position = workspace.dock.position.hide();
                }
            } else {
                if focus {
                    cx.focus(pane);
                }
            }
        } else if let Some(last_active_center_pane) = workspace
            .last_active_center_pane
            .as_ref()
            .and_then(|pane| pane.upgrade(cx))
        {
            if focus {
                cx.focus(last_active_center_pane);
            }
        }
        cx.emit(crate::Event::DockAnchorChanged);
        workspace.serialize_workspace(cx);
        cx.notify();
    }

    pub fn hide(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.hide(), true, cx);
    }

    pub fn show(workspace: &mut Workspace, focus: bool, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.show(), focus, cx);
    }

    pub fn hide_on_sidebar_shown(
        workspace: &mut Workspace,
        sidebar_side: SidebarSide,
        cx: &mut ViewContext<Workspace>,
    ) {
        if (sidebar_side == SidebarSide::Right && workspace.dock.is_anchored_at(DockAnchor::Right))
            || workspace.dock.is_anchored_at(DockAnchor::Expanded)
        {
            Self::hide(workspace, cx);
        }
    }

    fn focus_dock(workspace: &mut Workspace, _: &FocusDock, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.show(), true, cx);
    }

    fn hide_dock(workspace: &mut Workspace, _: &HideDock, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.hide(), true, cx);
    }

    pub fn move_dock(
        workspace: &mut Workspace,
        new_anchor: DockAnchor,
        focus: bool,
        cx: &mut ViewContext<Workspace>,
    ) {
        Self::set_dock_position(workspace, DockPosition::Shown(new_anchor), focus, cx);
    }

    pub fn render(
        &self,
        theme: &Theme,
        anchor: DockAnchor,
        cx: &mut RenderContext<Workspace>,
    ) -> Option<ElementBox> {
        let style = &theme.workspace.dock;

        self.position
            .is_visible()
            .then(|| self.position.anchor())
            .filter(|current_anchor| *current_anchor == anchor)
            .map(|anchor| match anchor {
                DockAnchor::Bottom | DockAnchor::Right => {
                    let mut panel_style = style.panel.clone();
                    let (resize_side, initial_size) = if anchor == DockAnchor::Bottom {
                        panel_style.border = Border {
                            top: true,
                            bottom: false,
                            left: false,
                            right: false,
                            ..panel_style.border
                        };

                        (Side::Top, style.initial_size_bottom)
                    } else {
                        panel_style.border = Border {
                            top: false,
                            bottom: false,
                            left: true,
                            right: false,
                            ..panel_style.border
                        };
                        (Side::Left, style.initial_size_right)
                    };

                    enum DockResizeHandle {}

                    let resizable = Container::new(ChildView::new(self.pane.clone(), cx).boxed())
                        .with_style(panel_style)
                        .with_resize_handle::<DockResizeHandle, _>(
                            resize_side as usize,
                            resize_side,
                            4.,
                            self.panel_sizes
                                .get(&anchor)
                                .copied()
                                .unwrap_or(initial_size),
                            cx,
                        );

                    let size = resizable.current_size();
                    let workspace = cx.handle();
                    cx.defer(move |cx| {
                        if let Some(workspace) = workspace.upgrade(cx) {
                            workspace.update(cx, |workspace, _| {
                                workspace.dock.panel_sizes.insert(anchor, size);
                            })
                        }
                    });

                    if anchor == DockAnchor::Right {
                        resizable
                            .constrained()
                            .dynamically(|constraint, cx| {
                                SizeConstraint::new(
                                    Vector2F::new(20., constraint.min.y()),
                                    Vector2F::new(cx.window_size.x() * 0.8, constraint.max.y()),
                                )
                            })
                            .boxed()
                    } else {
                        resizable
                            .constrained()
                            .dynamically(|constraint, cx| {
                                SizeConstraint::new(
                                    Vector2F::new(constraint.min.x(), 50.),
                                    Vector2F::new(constraint.max.x(), cx.window_size.y() * 0.8),
                                )
                            })
                            .boxed()
                    }
                }
                DockAnchor::Expanded => {
                    enum ExpandedDockWash {}
                    enum ExpandedDockPane {}
                    Stack::new()
                        .with_child(
                            // Render wash under the dock which when clicked hides it
                            MouseEventHandler::<ExpandedDockWash>::new(0, cx, |_, _| {
                                Empty::new()
                                    .contained()
                                    .with_background_color(style.wash_color)
                                    .boxed()
                            })
                            .capture_all()
                            .on_down(MouseButton::Left, |_, cx| {
                                cx.dispatch_action(HideDock);
                            })
                            .with_cursor_style(CursorStyle::Arrow)
                            .boxed(),
                        )
                        .with_child(
                            MouseEventHandler::<ExpandedDockPane>::new(0, cx, |_state, cx| {
                                ChildView::new(&self.pane, cx).boxed()
                            })
                            // Make sure all events directly under the dock pane
                            // are captured
                            .capture_all()
                            .contained()
                            .with_style(style.maximized)
                            .boxed(),
                        )
                        .boxed()
                }
            })
    }

    pub fn position(&self) -> DockPosition {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use std::{
        ops::{Deref, DerefMut},
        path::PathBuf,
    };

    use gpui::{AppContext, TestAppContext, UpdateView, View, ViewContext};
    use project::{FakeFs, Project};
    use settings::Settings;

    use super::*;
    use crate::{
        dock,
        item::{self, test::TestItem},
        persistence::model::{
            SerializedItem, SerializedPane, SerializedPaneGroup, SerializedWorkspace,
        },
        register_deserializable_item,
        sidebar::Sidebar,
        ItemHandle, Workspace,
    };

    pub fn default_item_factory(
        _workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Box<dyn ItemHandle>> {
        Some(Box::new(cx.add_view(|_| TestItem::new())))
    }

    #[gpui::test]
    async fn test_dock_workspace_infinite_loop(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();
        Settings::test_async(cx);

        cx.update(|cx| {
            register_deserializable_item::<item::test::TestItem>(cx);
        });

        let serialized_workspace = SerializedWorkspace {
            id: 0,
            location: Vec::<PathBuf>::new().into(),
            dock_position: dock::DockPosition::Shown(DockAnchor::Expanded),
            center_group: SerializedPaneGroup::Pane(SerializedPane {
                active: false,
                children: vec![],
            }),
            dock_pane: SerializedPane {
                active: true,
                children: vec![SerializedItem {
                    active: true,
                    item_id: 0,
                    kind: "TestItem".into(),
                }],
            },
            left_sidebar_open: false,
            bounds: Default::default(),
            display: Default::default(),
        };

        let fs = FakeFs::new(cx.background());
        let project = Project::test(fs, [], cx).await;

        let (_, _workspace) = cx.add_window(|cx| {
            Workspace::new(
                Some(serialized_workspace),
                0,
                project.clone(),
                default_item_factory,
                || &[],
                cx,
            )
        });

        cx.foreground().run_until_parked();
        //Should terminate
    }

    #[gpui::test]
    async fn test_dock_hides_when_pane_empty(cx: &mut TestAppContext) {
        let mut cx = DockTestContext::new(cx).await;

        // Closing the last item in the dock hides the dock
        cx.move_dock(DockAnchor::Right);
        let old_items = cx.dock_items();
        assert!(!old_items.is_empty());
        cx.close_dock_items().await;
        cx.assert_dock_position(DockPosition::Hidden(DockAnchor::Right));

        // Reopening the dock adds a new item
        cx.move_dock(DockAnchor::Right);
        let new_items = cx.dock_items();
        assert!(!new_items.is_empty());
        assert!(new_items
            .into_iter()
            .all(|new_item| !old_items.contains(&new_item)));
    }

    #[gpui::test]
    async fn test_dock_panel_collisions(cx: &mut TestAppContext) {
        let mut cx = DockTestContext::new(cx).await;

        // Dock closes when expanded for either panel
        cx.move_dock(DockAnchor::Expanded);
        cx.open_sidebar(SidebarSide::Left);
        cx.assert_dock_position(DockPosition::Hidden(DockAnchor::Expanded));
        cx.close_sidebar(SidebarSide::Left);
        cx.move_dock(DockAnchor::Expanded);
        cx.open_sidebar(SidebarSide::Right);
        cx.assert_dock_position(DockPosition::Hidden(DockAnchor::Expanded));

        // Dock closes in the right position if the right sidebar is opened
        cx.move_dock(DockAnchor::Right);
        cx.open_sidebar(SidebarSide::Left);
        cx.assert_dock_position(DockPosition::Shown(DockAnchor::Right));
        cx.open_sidebar(SidebarSide::Right);
        cx.assert_dock_position(DockPosition::Hidden(DockAnchor::Right));
        cx.close_sidebar(SidebarSide::Right);

        // Dock in bottom position ignores sidebars
        cx.move_dock(DockAnchor::Bottom);
        cx.open_sidebar(SidebarSide::Left);
        cx.open_sidebar(SidebarSide::Right);
        cx.assert_dock_position(DockPosition::Shown(DockAnchor::Bottom));

        // Opening the dock in the right position closes the right sidebar
        cx.move_dock(DockAnchor::Right);
        cx.assert_sidebar_closed(SidebarSide::Right);
    }

    #[gpui::test]
    async fn test_focusing_panes_shows_and_hides_dock(cx: &mut TestAppContext) {
        let mut cx = DockTestContext::new(cx).await;

        // Focusing an item not in the dock when expanded hides the dock
        let center_item = cx.add_item_to_center_pane();
        cx.move_dock(DockAnchor::Expanded);
        let dock_item = cx
            .dock_items()
            .get(0)
            .cloned()
            .expect("Dock should have an item at this point");
        center_item.update(&mut cx, |_, cx| cx.focus_self());
        cx.assert_dock_position(DockPosition::Hidden(DockAnchor::Expanded));

        // Focusing an item not in the dock when not expanded, leaves the dock open but inactive
        cx.move_dock(DockAnchor::Right);
        center_item.update(&mut cx, |_, cx| cx.focus_self());
        cx.assert_dock_position(DockPosition::Shown(DockAnchor::Right));
        cx.assert_dock_pane_inactive();
        cx.assert_workspace_pane_active();

        // Focusing an item in the dock activates it's pane
        dock_item.update(&mut cx, |_, cx| cx.focus_self());
        cx.assert_dock_position(DockPosition::Shown(DockAnchor::Right));
        cx.assert_dock_pane_active();
        cx.assert_workspace_pane_inactive();
    }

    #[gpui::test]
    async fn test_toggle_dock_focus(cx: &mut TestAppContext) {
        let cx = DockTestContext::new(cx).await;

        cx.move_dock(DockAnchor::Right);
        cx.assert_dock_pane_active();
        cx.hide_dock();
        cx.move_dock(DockAnchor::Right);
        cx.assert_dock_pane_active();
    }

    #[gpui::test]
    async fn test_activate_next_and_prev_pane(cx: &mut TestAppContext) {
        let mut cx = DockTestContext::new(cx).await;

        cx.move_dock(DockAnchor::Right);
        cx.assert_dock_pane_active();

        cx.update_workspace(|workspace, cx| workspace.activate_next_pane(cx));
        cx.assert_dock_pane_active();

        cx.update_workspace(|workspace, cx| workspace.activate_previous_pane(cx));
        cx.assert_dock_pane_active();
    }

    struct DockTestContext<'a> {
        pub cx: &'a mut TestAppContext,
        pub window_id: usize,
        pub workspace: ViewHandle<Workspace>,
    }

    impl<'a> DockTestContext<'a> {
        pub async fn new(cx: &'a mut TestAppContext) -> DockTestContext<'a> {
            Settings::test_async(cx);
            let fs = FakeFs::new(cx.background());

            cx.update(|cx| init(cx));
            let project = Project::test(fs, [], cx).await;
            let (window_id, workspace) = cx.add_window(|cx| {
                Workspace::new(
                    Default::default(),
                    0,
                    project,
                    default_item_factory,
                    || &[],
                    cx,
                )
            });

            workspace.update(cx, |workspace, cx| {
                let left_panel = cx.add_view(|_| TestItem::new());
                workspace.left_sidebar().update(cx, |sidebar, cx| {
                    sidebar.add_item(
                        "icons/folder_tree_16.svg",
                        "Left Test Panel".to_string(),
                        left_panel.clone(),
                        cx,
                    );
                });

                let right_panel = cx.add_view(|_| TestItem::new());
                workspace.right_sidebar().update(cx, |sidebar, cx| {
                    sidebar.add_item(
                        "icons/folder_tree_16.svg",
                        "Right Test Panel".to_string(),
                        right_panel.clone(),
                        cx,
                    );
                });
            });

            Self {
                cx,
                window_id,
                workspace,
            }
        }

        pub fn workspace<F, T>(&self, read: F) -> T
        where
            F: FnOnce(&Workspace, &AppContext) -> T,
        {
            self.workspace.read_with(self.cx, read)
        }

        pub fn update_workspace<F, T>(&mut self, update: F) -> T
        where
            F: FnOnce(&mut Workspace, &mut ViewContext<Workspace>) -> T,
        {
            self.workspace.update(self.cx, update)
        }

        pub fn sidebar<F, T>(&self, sidebar_side: SidebarSide, read: F) -> T
        where
            F: FnOnce(&Sidebar, &AppContext) -> T,
        {
            self.workspace(|workspace, cx| {
                let sidebar = match sidebar_side {
                    SidebarSide::Left => workspace.left_sidebar(),
                    SidebarSide::Right => workspace.right_sidebar(),
                }
                .read(cx);

                read(sidebar, cx)
            })
        }

        pub fn center_pane_handle(&self) -> ViewHandle<Pane> {
            self.workspace(|workspace, cx| {
                workspace
                    .last_active_center_pane
                    .clone()
                    .and_then(|pane| pane.upgrade(cx))
                    .unwrap_or_else(|| workspace.center.panes()[0].clone())
            })
        }

        pub fn add_item_to_center_pane(&mut self) -> ViewHandle<TestItem> {
            self.update_workspace(|workspace, cx| {
                let item = cx.add_view(|_| TestItem::new());
                let pane = workspace
                    .last_active_center_pane
                    .clone()
                    .and_then(|pane| pane.upgrade(cx))
                    .unwrap_or_else(|| workspace.center.panes()[0].clone());
                Pane::add_item(
                    workspace,
                    &pane,
                    Box::new(item.clone()),
                    true,
                    true,
                    None,
                    cx,
                );
                item
            })
        }

        pub fn dock_pane<F, T>(&self, read: F) -> T
        where
            F: FnOnce(&Pane, &AppContext) -> T,
        {
            self.workspace(|workspace, cx| {
                let dock_pane = workspace.dock_pane().read(cx);
                read(dock_pane, cx)
            })
        }

        pub fn move_dock(&self, anchor: DockAnchor) {
            self.cx.dispatch_action(self.window_id, MoveDock(anchor));
        }

        pub fn hide_dock(&self) {
            self.cx.dispatch_action(self.window_id, HideDock);
        }

        pub fn open_sidebar(&mut self, sidebar_side: SidebarSide) {
            if !self.sidebar(sidebar_side, |sidebar, _| sidebar.is_open()) {
                self.update_workspace(|workspace, cx| workspace.toggle_sidebar(sidebar_side, cx));
            }
        }

        pub fn close_sidebar(&mut self, sidebar_side: SidebarSide) {
            if self.sidebar(sidebar_side, |sidebar, _| sidebar.is_open()) {
                self.update_workspace(|workspace, cx| workspace.toggle_sidebar(sidebar_side, cx));
            }
        }

        pub fn dock_items(&self) -> Vec<ViewHandle<TestItem>> {
            self.dock_pane(|pane, cx| {
                pane.items()
                    .map(|item| {
                        item.act_as::<TestItem>(cx)
                            .expect("Dock Test Context uses TestItems in the dock")
                    })
                    .collect()
            })
        }

        pub async fn close_dock_items(&mut self) {
            self.update_workspace(|workspace, cx| {
                Pane::close_items(workspace, workspace.dock_pane().clone(), cx, |_| true)
            })
            .await
            .expect("Could not close dock items")
        }

        pub fn assert_dock_position(&self, expected_position: DockPosition) {
            self.workspace(|workspace, _| assert_eq!(workspace.dock.position, expected_position));
        }

        pub fn assert_sidebar_closed(&self, sidebar_side: SidebarSide) {
            assert!(!self.sidebar(sidebar_side, |sidebar, _| sidebar.is_open()));
        }

        pub fn assert_workspace_pane_active(&self) {
            assert!(self
                .center_pane_handle()
                .read_with(self.cx, |pane, _| pane.is_active()));
        }

        pub fn assert_workspace_pane_inactive(&self) {
            assert!(!self
                .center_pane_handle()
                .read_with(self.cx, |pane, _| pane.is_active()));
        }

        pub fn assert_dock_pane_active(&self) {
            assert!(self.dock_pane(|pane, _| pane.is_active()))
        }

        pub fn assert_dock_pane_inactive(&self) {
            assert!(!self.dock_pane(|pane, _| pane.is_active()))
        }
    }

    impl<'a> Deref for DockTestContext<'a> {
        type Target = gpui::TestAppContext;

        fn deref(&self) -> &Self::Target {
            self.cx
        }
    }

    impl<'a> DerefMut for DockTestContext<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.cx
        }
    }

    impl<'a> UpdateView for DockTestContext<'a> {
        fn update_view<T, S>(
            &mut self,
            handle: &ViewHandle<T>,
            update: &mut dyn FnMut(&mut T, &mut ViewContext<T>) -> S,
        ) -> S
        where
            T: View,
        {
            handle.update(self.cx, update)
        }
    }
}
