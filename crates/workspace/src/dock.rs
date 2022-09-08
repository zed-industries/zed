use gpui::{
    actions,
    elements::{ChildView, MouseEventHandler, Svg},
    impl_internal_actions, CursorStyle, Element, ElementBox, Entity, MouseButton,
    MutableAppContext, View, ViewContext, ViewHandle, WeakViewHandle,
};
use serde::Deserialize;
use settings::Settings;
use theme::Theme;

use crate::{pane, ItemHandle, Pane, StatusItemView, Workspace};

#[derive(PartialEq, Clone, Deserialize)]
pub struct MoveDock(pub DockAnchor);

#[derive(PartialEq, Clone)]
pub struct AddDefaultItemToDock;

actions!(workspace, [ToggleDock]);
impl_internal_actions!(workspace, [MoveDock, AddDefaultItemToDock]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Dock::toggle);
    cx.add_action(Dock::move_dock);
}

#[derive(PartialEq, Eq, Default, Copy, Clone, Deserialize)]
pub enum DockAnchor {
    #[default]
    Bottom,
    Right,
    Expanded,
}

#[derive(Copy, Clone)]
pub enum DockPosition {
    Shown(DockAnchor),
    Hidden(DockAnchor),
}

impl Default for DockPosition {
    fn default() -> Self {
        DockPosition::Hidden(Default::default())
    }
}

impl DockPosition {
    fn toggle(self) -> Self {
        match self {
            DockPosition::Shown(anchor) => DockPosition::Hidden(anchor),
            DockPosition::Hidden(anchor) => DockPosition::Shown(anchor),
        }
    }

    fn visible(&self) -> Option<DockAnchor> {
        match self {
            DockPosition::Shown(anchor) => Some(*anchor),
            DockPosition::Hidden(_) => None,
        }
    }

    fn hide(self) -> Self {
        match self {
            DockPosition::Shown(anchor) => DockPosition::Hidden(anchor),
            DockPosition::Hidden(_) => self,
        }
    }
}

pub type DefaultItemFactory =
    fn(&mut Workspace, &mut ViewContext<Workspace>) -> Box<dyn ItemHandle>;

pub struct Dock {
    position: DockPosition,
    pane: ViewHandle<Pane>,
    default_item_factory: DefaultItemFactory,
}

impl Dock {
    pub fn new(cx: &mut ViewContext<Workspace>, default_item_factory: DefaultItemFactory) -> Self {
        let pane = cx.add_view(|cx| Pane::new(true, cx));

        cx.subscribe(&pane.clone(), |workspace, _, event, cx| {
            if let pane::Event::Remove = event {
                workspace.dock.hide();
                cx.notify();
            }
        })
        .detach();

        Self {
            pane,
            position: Default::default(),
            default_item_factory,
        }
    }

    pub fn pane(&self) -> ViewHandle<Pane> {
        self.pane.clone()
    }

    fn hide(&mut self) {
        self.position = self.position.hide();
    }

    fn ensure_not_empty(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let pane = workspace.dock.pane.clone();
        if pane.read(cx).items().next().is_none() {
            let item_to_add = (workspace.dock.default_item_factory)(workspace, cx);
            Pane::add_item(workspace, &pane, item_to_add, true, true, None, cx);
        }
    }

    fn toggle(workspace: &mut Workspace, _: &ToggleDock, cx: &mut ViewContext<Workspace>) {
        // Shift-escape ON
        // Get or insert the dock's last focused terminal
        // Open the dock in fullscreen
        // Focus that terminal

        // Shift-escape OFF
        // Close the dock
        // Return focus to center

        // Behaviors:
        // If the dock is shown, hide it
        // If the dock is hidden, show it
        // If the dock was full screen, open it in last position (bottom or right)
        // If the dock was bottom or right, re-open it in that context (and with the previous % width)

        workspace.dock.position = workspace.dock.position.toggle();
        if workspace.dock.position.visible().is_some() {
            Self::ensure_not_empty(workspace, cx);
            cx.focus(workspace.dock.pane.clone());
        } else {
            cx.focus_self();
        }
        cx.notify();
        workspace.status_bar().update(cx, |_, cx| cx.notify());
    }

    fn move_dock(
        workspace: &mut Workspace,
        &MoveDock(new_anchor): &MoveDock,
        cx: &mut ViewContext<Workspace>,
    ) {
        // Clear the previous position if the dock is not visible.
        workspace.dock.position = DockPosition::Shown(new_anchor);
        Self::ensure_not_empty(workspace, cx);
        cx.notify();
    }

    pub fn render(&self, _theme: &Theme, anchor: DockAnchor) -> Option<ElementBox> {
        self.position
            .visible()
            .filter(|current_anchor| *current_anchor == anchor)
            .map(|_| ChildView::new(self.pane.clone()).boxed())
    }
}

pub struct ToggleDockButton {
    workspace: WeakViewHandle<Workspace>,
}

impl ToggleDockButton {
    pub fn new(workspace: WeakViewHandle<Workspace>, _cx: &mut ViewContext<Self>) -> Self {
        Self { workspace }
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
        let dock_is_open = self
            .workspace
            .upgrade(cx)
            .map(|workspace| workspace.read(cx).dock.position.visible().is_some())
            .unwrap_or(false);

        MouseEventHandler::new::<Self, _, _>(0, cx, |state, cx| {
            let theme = &cx
                .global::<Settings>()
                .theme
                .workspace
                .status_bar
                .sidebar_buttons;
            let style = theme.item.style_for(state, dock_is_open);

            Svg::new("icons/terminal_16.svg")
                .with_color(style.icon_color)
                .constrained()
                .with_width(style.icon_size)
                .with_height(style.icon_size)
                .contained()
                .with_style(style.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, cx| {
            cx.dispatch_action(ToggleDock);
        })
        // TODO: Add tooltip
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
