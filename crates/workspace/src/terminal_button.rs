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
        // When terminal moves, redraw so that the icon and toggle status matches.
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

        let focused_view = cx.focused_view_id(cx.window_id());

        // FIXME: Don't hardcode "Terminal" in here
        let active = focused_view
            .map(|view| cx.view_ui_name(cx.window_id(), view) == Some("Terminal"))
            .unwrap_or(false);

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
                    .style_for(state, active);

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
        .on_click(MouseButton::Left, move |_, cx| {
            if !active {
                cx.dispatch_action(FocusDock);
            }
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
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}
