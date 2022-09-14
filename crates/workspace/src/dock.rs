use collections::HashMap;
use gpui::{
    actions,
    elements::{ChildView, Container, Empty, Margin, MouseEventHandler, Side, Svg},
    impl_internal_actions, CursorStyle, Element, ElementBox, Entity, MouseButton,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle, WeakViewHandle,
};
use serde::Deserialize;
use settings::{DockAnchor, Settings};
use theme::Theme;

use crate::{sidebar::SidebarSide, ItemHandle, Pane, StatusItemView, Workspace};

#[derive(PartialEq, Clone, Deserialize)]
pub struct MoveDock(pub DockAnchor);

#[derive(PartialEq, Clone)]
pub struct AddDefaultItemToDock;

actions!(workspace, [ToggleDock, ActivateOrHideDock]);
impl_internal_actions!(workspace, [MoveDock, AddDefaultItemToDock]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Dock::toggle);
    cx.add_action(Dock::activate_or_hide_dock);
    cx.add_action(Dock::move_dock);
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
    fn is_visible(&self) -> bool {
        match self {
            DockPosition::Shown(_) => true,
            DockPosition::Hidden(_) => false,
        }
    }

    fn anchor(&self) -> DockAnchor {
        match self {
            DockPosition::Shown(anchor) | DockPosition::Hidden(anchor) => *anchor,
        }
    }

    fn toggle(self) -> Self {
        match self {
            DockPosition::Shown(anchor) => DockPosition::Hidden(anchor),
            DockPosition::Hidden(anchor) => DockPosition::Shown(anchor),
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

pub type DefaultItemFactory =
    fn(&mut Workspace, &mut ViewContext<Workspace>) -> Box<dyn ItemHandle>;

pub struct Dock {
    position: DockPosition,
    panel_sizes: HashMap<DockAnchor, f32>,
    pane: ViewHandle<Pane>,
    default_item_factory: DefaultItemFactory,
}

impl Dock {
    pub fn new(cx: &mut ViewContext<Workspace>, default_item_factory: DefaultItemFactory) -> Self {
        let anchor = cx.global::<Settings>().default_dock_anchor;
        let pane = cx.add_view(|cx| Pane::new(Some(anchor), cx));
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
            position: DockPosition::Hidden(anchor),
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

    fn set_dock_position(
        workspace: &mut Workspace,
        new_position: DockPosition,
        cx: &mut ViewContext<Workspace>,
    ) {
        if workspace.dock.position == new_position {
            return;
        }

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
                let item_to_add = (workspace.dock.default_item_factory)(workspace, cx);
                Pane::add_item(workspace, &pane, item_to_add, true, true, None, cx);
            }
        } else if let Some(last_active_center_pane) = workspace.last_active_center_pane.clone() {
            cx.focus(last_active_center_pane);
        }
        cx.emit(crate::Event::DockAnchorChanged);
        cx.notify();
    }

    pub fn hide(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.hide(), cx);
    }

    pub fn show(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.show(), cx);
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

    fn toggle(workspace: &mut Workspace, _: &ToggleDock, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.toggle(), cx);
    }

    fn activate_or_hide_dock(
        workspace: &mut Workspace,
        _: &ActivateOrHideDock,
        cx: &mut ViewContext<Workspace>,
    ) {
        let dock_pane = workspace.dock_pane().clone();
        if dock_pane.read(cx).is_active() {
            Self::hide(workspace, cx);
        } else {
            Self::show(workspace, cx);
            cx.focus(dock_pane);
        }
    }

    fn move_dock(
        workspace: &mut Workspace,
        &MoveDock(new_anchor): &MoveDock,
        cx: &mut ViewContext<Workspace>,
    ) {
        Self::set_dock_position(workspace, DockPosition::Shown(new_anchor), cx);
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
                        panel_style.margin = Margin {
                            top: panel_style.margin.top,
                            ..Default::default()
                        };

                        (Side::Top, style.initial_size_bottom)
                    } else {
                        panel_style.margin = Margin {
                            left: panel_style.margin.left,
                            ..Default::default()
                        };
                        (Side::Left, style.initial_size_right)
                    };

                    enum DockResizeHandle {}

                    let resizable = Container::new(ChildView::new(self.pane.clone()).boxed())
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

                    resizable.flex(style.flex, false).boxed()
                }
                DockAnchor::Expanded => {
                    enum ExpandedDockWash {}
                    enum ExpandedDockPane {}
                    Container::new(
                        MouseEventHandler::<ExpandedDockWash>::new(0, cx, |_state, cx| {
                            MouseEventHandler::<ExpandedDockPane>::new(0, cx, |_state, _cx| {
                                ChildView::new(self.pane.clone()).boxed()
                            })
                            .capture_all()
                            .contained()
                            .with_style(style.maximized)
                            .boxed()
                        })
                        .capture_all()
                        .on_down(MouseButton::Left, |_, cx| {
                            cx.dispatch_action(ToggleDock);
                        })
                        .with_cursor_style(CursorStyle::Arrow)
                        .boxed(),
                    )
                    .with_background_color(style.wash_color)
                    .boxed()
                }
            })
    }
}

pub struct ToggleDockButton {
    workspace: WeakViewHandle<Workspace>,
}

impl ToggleDockButton {
    pub fn new(workspace: ViewHandle<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        // When dock moves, redraw so that the icon and toggle status matches.
        cx.subscribe(&workspace, |_, _, _, cx| cx.notify()).detach();

        Self {
            workspace: workspace.downgrade(),
        }
    }
}

impl Entity for ToggleDockButton {
    type Event = ();
}

impl View for ToggleDockButton {
    fn ui_name() -> &'static str {
        "Dock Toggle"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
        let workspace = self.workspace.upgrade(cx);

        if workspace.is_none() {
            return Empty::new().boxed();
        }

        let dock_position = workspace.unwrap().read(cx).dock.position;

        let theme = cx.global::<Settings>().theme.clone();
        MouseEventHandler::<Self>::new(0, cx, {
            let theme = theme.clone();
            move |state, _| {
                let style = theme
                    .workspace
                    .status_bar
                    .sidebar_buttons
                    .item
                    .style_for(state, dock_position.is_visible());

                Svg::new(icon_for_dock_anchor(dock_position.anchor()))
                    .with_color(style.icon_color)
                    .constrained()
                    .with_width(style.icon_size)
                    .with_height(style.icon_size)
                    .contained()
                    .with_style(style.container)
                    .boxed()
            }
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, cx| {
            cx.dispatch_action(ToggleDock);
        })
        .with_tooltip::<Self, _>(
            0,
            "Toggle Dock".to_string(),
            Some(Box::new(ToggleDock)),
            theme.tooltip.clone(),
            cx,
        )
        .boxed()
    }
}

impl StatusItemView for ToggleDockButton {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn crate::ItemHandle>,
        _cx: &mut ViewContext<Self>,
    ) {
        //Not applicable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, ViewContext};
    use project::{FakeFs, Project};
    use settings::Settings;

    use crate::{tests::TestItem, ItemHandle, Workspace};

    pub fn default_item_factory(
        _workspace: &mut Workspace,
        cx: &mut ViewContext<Workspace>,
    ) -> Box<dyn ItemHandle> {
        Box::new(cx.add_view(|_| TestItem::new()))
    }

    #[gpui::test]
    async fn test_dock_hides_when_pane_empty(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();

        Settings::test_async(cx);
        let fs = FakeFs::new(cx.background());

        let project = Project::test(fs, [], cx).await;
        let (_, workspace) = cx.add_window(|cx| Workspace::new(project, default_item_factory, cx));

        // Open dock
        workspace.update(cx, |workspace, cx| {
            Dock::show(workspace, cx);
        });

        // Ensure dock has an item in it
        let dock_item_handle = workspace.read_with(cx, |workspace, cx| {
            let dock = workspace.dock_pane().read(cx);
            dock.items()
                .next()
                .expect("Dock should have an item in it")
                .clone()
        });

        // Close item
        let close_task = workspace.update(cx, |workspace, cx| {
            Pane::close_item(
                workspace,
                workspace.dock_pane().clone(),
                dock_item_handle.id(),
                cx,
            )
        });
        close_task.await.expect("Dock item closed successfully");

        // Ensure dock closes
        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.dock.visible_pane().is_some())
        });

        // Open again
        workspace.update(cx, |workspace, cx| {
            Dock::show(workspace, cx);
        });

        // Ensure dock has item in it
        workspace.read_with(cx, |workspace, cx| {
            let dock = workspace.dock_pane().read(cx);
            dock.items().next().expect("Dock should have an item in it");
        });
    }

    #[gpui::test]
    async fn test_dock_panel_collisions(cx: &mut TestAppContext) {
        // Open dock expanded
        // Open left panel
        // Ensure dock closes
        // Open dock to the right
        // Open left panel
        // Ensure dock is left open
        // Open right panel
        // Ensure dock closes
        // Open dock bottom
        // Open left panel
        // Open right panel
        // Ensure dock still open
    }

    #[gpui::test]
    async fn test_focusing_panes_shows_and_hides_dock(cx: &mut TestAppContext) {
        // Open item in center pane
        // Open dock expanded
        // Focus new item
        // Ensure the dock gets hidden
        // Open dock to the right
        // Focus new item
        // Ensure dock stays shown but inactive
        // Add item to dock and hide it
        // Focus the added item
        // Ensure the dock is open
    }
}
