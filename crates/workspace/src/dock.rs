use gpui::{
    actions,
    elements::{ChildView, Container, FlexItem, Margin, MouseEventHandler, Svg},
    impl_internal_actions, CursorStyle, Element, ElementBox, Entity, MouseButton,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle, WeakViewHandle,
};
use serde::Deserialize;
use settings::{DockAnchor, Settings};
use theme::Theme;

use crate::{ItemHandle, Pane, StatusItemView, Workspace};

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
        let anchor = cx.global::<Settings>().default_dock_anchor;
        let pane = cx.add_view(|cx| Pane::new(Some(anchor), cx));
        let pane_id = pane.id();
        cx.subscribe(&pane, move |workspace, _, event, cx| {
            workspace.handle_pane_event(pane_id, event, cx);
        })
        .detach();

        Self {
            pane,
            position: DockPosition::Hidden(anchor),
            default_item_factory,
        }
    }

    pub fn pane(&self) -> &ViewHandle<Pane> {
        &self.pane
    }

    pub fn visible_pane(&self) -> Option<&ViewHandle<Pane>> {
        self.position.visible().map(|_| self.pane())
    }

    fn set_dock_position(
        workspace: &mut Workspace,
        new_position: DockPosition,
        cx: &mut ViewContext<Workspace>,
    ) {
        workspace.dock.position = new_position;
        // Tell the pane about the new anchor position
        workspace.dock.pane.update(cx, |pane, cx| {
            pane.set_docked(Some(new_position.anchor()), cx)
        });

        let now_visible = workspace.dock.visible_pane().is_some();
        if now_visible {
            // Ensure that the pane has at least one item or construct a default item to put in it
            let pane = workspace.dock.pane.clone();
            if pane.read(cx).items().next().is_none() {
                let item_to_add = (workspace.dock.default_item_factory)(workspace, cx);
                Pane::add_item(workspace, &pane, item_to_add, true, true, None, cx);
            }
            cx.focus(pane);
        } else {
            if let Some(last_active_center_pane) = workspace.last_active_center_pane.clone() {
                cx.focus(last_active_center_pane);
            }
        }
        cx.notify();
    }

    pub fn hide(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.hide(), cx);
    }

    fn toggle(workspace: &mut Workspace, _: &ToggleDock, cx: &mut ViewContext<Workspace>) {
        Self::set_dock_position(workspace, workspace.dock.position.toggle(), cx);
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
            .visible()
            .filter(|current_anchor| *current_anchor == anchor)
            .map(|anchor| match anchor {
                DockAnchor::Bottom | DockAnchor::Right => {
                    let mut panel_style = style.panel.clone();
                    if anchor == DockAnchor::Bottom {
                        panel_style.margin = Margin {
                            top: panel_style.margin.top,
                            ..Default::default()
                        };
                    } else {
                        panel_style.margin = Margin {
                            left: panel_style.margin.left,
                            ..Default::default()
                        };
                    }
                    FlexItem::new(
                        Container::new(ChildView::new(self.pane.clone()).boxed())
                            .with_style(style.panel)
                            .boxed(),
                    )
                    .flex(style.flex, true)
                    .boxed()
                }
                DockAnchor::Expanded => Container::new(
                    MouseEventHandler::<Dock>::new(0, cx, |_state, _cx| {
                        Container::new(ChildView::new(self.pane.clone()).boxed())
                            .with_style(style.maximized)
                            .boxed()
                    })
                    .capture_all()
                    .with_cursor_style(CursorStyle::Arrow)
                    .boxed(),
                )
                .with_background_color(style.wash_color)
                .boxed(),
            })
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

        MouseEventHandler::<Self>::new(0, cx, |state, cx| {
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
