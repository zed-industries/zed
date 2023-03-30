use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    elements::*, impl_internal_actions, CursorStyle, Element, ElementBox, Entity, MouseButton,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;
use theme::Editor;
use workspace::{item::ItemHandle, NewTerminal, StatusItemView};

use crate::{Copilot, Status};

const COPILOT_SETTINGS_URL: &str = "https://github.com/settings/copilot";

#[derive(Clone, PartialEq)]
pub struct DeployCopilotMenu;

// TODO: Make the other code path use `get_or_insert` logic for this modal
#[derive(Clone, PartialEq)]
pub struct DeployCopilotModal;

impl_internal_actions!(copilot, [DeployCopilotMenu, DeployCopilotModal]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(CopilotButton::deploy_copilot_menu);
}

pub struct CopilotButton {
    popup_menu: ViewHandle<ContextMenu>,
    editor: Option<WeakViewHandle<Editor>>,
}

impl Entity for CopilotButton {
    type Event = ();
}

impl View for CopilotButton {
    fn ui_name() -> &'static str {
        "CopilotButton"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let settings = cx.global::<Settings>();

        if !settings.enable_copilot_integration {
            return Empty::new().boxed();
        }

        let theme = settings.theme.clone();
        let active = self.popup_menu.read(cx).visible() /* || modal.is_shown */;
        let authorized = Copilot::global(cx).unwrap().read(cx).status() == Status::Authorized;
        let enabled = true;

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
                            .style_for(state, active);

                        Flex::row()
                            .with_child(
                                Svg::new({
                                    if authorized {
                                        if enabled {
                                            "icons/copilot_16.svg"
                                        } else {
                                            "icons/copilot_disabled_16.svg"
                                        }
                                    } else {
                                        "icons/copilot_init_16.svg"
                                    }
                                })
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
                .on_click(MouseButton::Left, move |_, cx| {
                    if authorized {
                        cx.dispatch_action(DeployCopilotMenu);
                    } else {
                        cx.dispatch_action(DeployCopilotModal);
                    }
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
            editor: None,
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
        if let Some(editor) = item.map(|item| item.act_as::<editor::Editor>(cx)) {}
        cx.notify();
    }
}
