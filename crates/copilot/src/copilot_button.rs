use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    elements::*, impl_internal_actions, CursorStyle, Element, ElementBox, Entity, MouseButton,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
};
use settings::Settings;
use workspace::{item::ItemHandle, NewTerminal, StatusItemView};

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";

#[derive(Clone, PartialEq)]
pub struct DeployCopilotMenu;

impl_internal_actions!(copilot, [DeployCopilotMenu]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(CopilotButton::deploy_copilot_menu);
}

pub struct CopilotButton {
    popup_menu: ViewHandle<ContextMenu>,
}

impl Entity for CopilotButton {
    type Event = ();
}

impl View for CopilotButton {
    fn ui_name() -> &'static str {
        "CopilotButton"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();

        let visible = self.popup_menu.read(cx).visible();

        Stack::new()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, {
                    let theme = theme.clone();
                    move |state, _cx| {
                        let style = theme
                            .workspace
                            .status_bar
                            .sidebar_buttons
                            .item
                            .style_for(state, visible);

                        Flex::row()
                            .with_child(
                                Svg::new("icons/maybe_copilot.svg")
                                    .with_color(style.icon_color)
                                    .constrained()
                                    .with_width(style.icon_size)
                                    .aligned()
                                    .named("copilot-icon"),
                            )
                            .constrained()
                            .with_height(style.icon_size)
                            .contained()
                            .with_style(style.container)
                            .boxed()
                    }
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, _cx| {
                    // TODO: Behavior of this
                    // if has_terminals {
                    //     cx.dispatch_action(DeployCopilotMenu);
                    // } else {
                    //     if !active {
                    //         cx.dispatch_action(FocusDock);
                    //     }
                    // };
                })
                .with_tooltip::<Self, _>(
                    0,
                    "GitHub Copilot".into(),
                    None,
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed(),
            )
            .with_child(
                ChildView::new(&self.popup_menu, cx)
                    .aligned()
                    .top()
                    .right()
                    .boxed(),
            )
            .boxed()
    }
}

impl CopilotButton {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        Self {
            popup_menu: cx.add_view(|cx| {
                let mut menu = ContextMenu::new(cx);
                menu.set_position_mode(OverlayPositionMode::Local);
                menu
            }),
        }
    }

    pub fn deploy_copilot_menu(&mut self, _: &DeployCopilotMenu, cx: &mut ViewContext<Self>) {
        let mut menu_options = vec![ContextMenuItem::item("New Terminal", NewTerminal)];

        self.popup_menu.update(cx, |menu, cx| {
            menu.show(
                Default::default(),
                AnchorCorner::BottomRight,
                menu_options,
                cx,
            );
        });
    }
}

impl StatusItemView for CopilotButton {
    fn set_active_pane_item(&mut self, item: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}
