use crate::TerminalView;
use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    elements::*,
    platform::{CursorStyle, MouseButton},
    AnyElement, Element, Entity, View, ViewContext, ViewHandle, WeakViewHandle,
};
use std::any::TypeId;
use workspace::{
    dock::{Dock, FocusDock},
    item::ItemHandle,
    NewTerminal, StatusItemView, Workspace,
};

pub struct TerminalButton {
    workspace: WeakViewHandle<Workspace>,
    popup_menu: ViewHandle<ContextMenu>,
}

impl Entity for TerminalButton {
    type Event = ();
}

impl View for TerminalButton {
    fn ui_name() -> &'static str {
        "TerminalButton"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let workspace = self.workspace.upgrade(cx);
        let project = match workspace {
            Some(workspace) => workspace.read(cx).project().read(cx),
            None => return Empty::new().into_any(),
        };

        let focused_view = cx.focused_view_id();
        let active = focused_view
            .map(|view_id| {
                cx.view_type_id(cx.window_id(), view_id) == Some(TypeId::of::<TerminalView>())
            })
            .unwrap_or(false);

        let has_terminals = !project.local_terminal_handles().is_empty();
        let terminal_count = project.local_terminal_handles().len() as i32;
        let theme = theme::current(cx).clone();

        Stack::new()
            .with_child(
                MouseEventHandler::<Self, _>::new(0, cx, {
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
                                Svg::new("icons/terminal_12.svg")
                                    .with_color(style.icon_color)
                                    .constrained()
                                    .with_width(style.icon_size)
                                    .aligned()
                                    .into_any_named("terminals-icon"),
                            )
                            .with_children(has_terminals.then(|| {
                                Label::new(terminal_count.to_string(), style.label.text.clone())
                                    .contained()
                                    .with_style(style.label.container)
                                    .aligned()
                            }))
                            .constrained()
                            .with_height(style.icon_size)
                            .contained()
                            .with_style(style.container)
                    }
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, this, cx| {
                    if has_terminals {
                        this.deploy_terminal_menu(cx);
                    } else {
                        if !active {
                            if let Some(workspace) = this.workspace.upgrade(cx) {
                                workspace.update(cx, |workspace, cx| {
                                    Dock::focus_dock(workspace, &Default::default(), cx)
                                })
                            }
                        }
                    };
                })
                .with_tooltip::<Self>(
                    0,
                    "Show Terminal".into(),
                    Some(Box::new(FocusDock)),
                    theme.tooltip.clone(),
                    cx,
                ),
            )
            .with_child(ChildView::new(&self.popup_menu, cx).aligned().top().right())
            .into_any_named("terminal button")
    }
}

impl TerminalButton {
    pub fn new(workspace: ViewHandle<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        let button_view_id = cx.view_id();
        cx.observe(&workspace, |_, _, cx| cx.notify()).detach();
        Self {
            workspace: workspace.downgrade(),
            popup_menu: cx.add_view(|cx| {
                let mut menu = ContextMenu::new(button_view_id, cx);
                menu.set_position_mode(OverlayPositionMode::Local);
                menu
            }),
        }
    }

    pub fn deploy_terminal_menu(&mut self, cx: &mut ViewContext<Self>) {
        let mut menu_options = vec![ContextMenuItem::action("New Terminal", NewTerminal)];

        if let Some(workspace) = self.workspace.upgrade(cx) {
            let project = workspace.read(cx).project().read(cx);
            let local_terminal_handles = project.local_terminal_handles();

            if !local_terminal_handles.is_empty() {
                menu_options.push(ContextMenuItem::Separator)
            }

            for local_terminal_handle in local_terminal_handles {
                if let Some(terminal) = local_terminal_handle.upgrade(cx) {
                    let workspace = self.workspace.clone();
                    let local_terminal_handle = local_terminal_handle.clone();
                    menu_options.push(ContextMenuItem::handler(
                        terminal.read(cx).title(),
                        move |cx| {
                            if let Some(workspace) = workspace.upgrade(cx) {
                                workspace.update(cx, |workspace, cx| {
                                    let terminal = workspace
                                        .items_of_type::<TerminalView>(cx)
                                        .find(|terminal| {
                                            terminal.read(cx).model().downgrade()
                                                == local_terminal_handle
                                        });
                                    if let Some(terminal) = terminal {
                                        workspace.activate_item(&terminal, cx);
                                    }
                                });
                            }
                        },
                    ))
                }
            }
        }

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

impl StatusItemView for TerminalButton {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}
