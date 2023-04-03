use gpui::{
    elements::{Empty, MouseEventHandler, Svg},
    CursorStyle, Element, ElementBox, Entity, MouseButton, View, ViewContext, ViewHandle,
    WeakViewHandle,
};
use settings::Settings;

use crate::{handle_dropped_item, StatusItemView, Workspace};

use super::{icon_for_dock_anchor, FocusDock, HideDock};

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

        let workspace = workspace.unwrap();
        let dock_position = workspace.read(cx).dock.position;
        let dock_pane = workspace.read(cx.app).dock_pane().clone();

        let theme = cx.global::<Settings>().theme.clone();

        let button = MouseEventHandler::<Self>::new(0, cx, {
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
        .on_up(MouseButton::Left, move |event, cx| {
            let drop_index = dock_pane.read(cx.app).items_len() + 1;
            handle_dropped_item(event, &dock_pane.downgrade(), drop_index, false, None, cx);
        });

        if dock_position.is_visible() {
            button
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(HideDock);
                })
                .with_tooltip::<Self, _>(
                    0,
                    "Hide Dock".into(),
                    Some(Box::new(HideDock)),
                    theme.tooltip.clone(),
                    cx,
                )
        } else {
            button
                .on_click(MouseButton::Left, |_, cx| {
                    cx.dispatch_action(FocusDock);
                })
                .with_tooltip::<Self, _>(
                    0,
                    "Focus Dock".into(),
                    Some(Box::new(FocusDock)),
                    theme.tooltip.clone(),
                    cx,
                )
        }
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
