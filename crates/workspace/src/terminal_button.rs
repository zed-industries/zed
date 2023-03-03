use gpui::{
    elements::{Empty, MouseEventHandler, Svg},
    CursorStyle, Element, ElementBox, Entity, MouseButton, RenderContext, View, ViewContext,
    ViewHandle, WeakViewHandle,
};
use settings::Settings;

use crate::{dock::FocusDock, item::ItemHandle, StatusItemView, Workspace};

pub struct TerminalButton {
    workspace: WeakViewHandle<Workspace>,
}

impl TerminalButton {
    pub fn new(workspace: ViewHandle<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        // When dock moves, redraw so that the icon and toggle status matches.
        cx.subscribe(&workspace, |_, _, _, cx| cx.notify()).detach();

        Self {
            workspace: workspace.downgrade(),
        }
    }
}

impl Entity for TerminalButton {
    type Event = ();
}

impl View for TerminalButton {
    fn ui_name() -> &'static str {
        "TerminalButton"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let workspace = self.workspace.upgrade(cx);

        if workspace.is_none() {
            return Empty::new().boxed();
        }

        // let workspace = workspace.unwrap();
        let theme = cx.global::<Settings>().theme.clone();

        MouseEventHandler::<Self>::new(0, cx, {
            let theme = theme.clone();
            move |state, _| {
                let style = theme
                    .workspace
                    .status_bar
                    .sidebar_buttons
                    .item
                    .style_for(state, true);

                Svg::new("icons/terminal_12.svg")
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
        .on_up(MouseButton::Left, move |_, _| {
            // let dock_pane = workspace.read(cx.app).dock_pane();
            // let drop_index = dock_pane.read(cx.app).items_len() + 1;
            // handle_dropped_item(event, &dock_pane.downgrade(), drop_index, false, None, cx);
        })
        .on_click(MouseButton::Left, |_, cx| {
            cx.dispatch_action(FocusDock);
        })
        .with_tooltip::<Self, _>(
            0,
            "Show Terminal".into(),
            Some(Box::new(FocusDock)),
            theme.tooltip.clone(),
            cx,
        )
        .boxed()
    }
}

impl StatusItemView for TerminalButton {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
}
