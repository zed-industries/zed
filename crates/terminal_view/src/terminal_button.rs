use context_menu::{ContextMenu, ContextMenuItem};
use gpui::{
    actions, elements::*, geometry::vector::vec2f, impl_internal_actions, CursorStyle, Element,
    ElementBox, Entity, MouseButton, MutableAppContext, RenderContext, View, ViewContext,
    ViewHandle, WeakModelHandle, WeakViewHandle,
};
use settings::Settings;
use terminal::Terminal;
use workspace::{dock::FocusDock, item::ItemHandle, NewTerminal, StatusItemView, Workspace};

use crate::TerminalView;

#[derive(Clone, PartialEq)]
pub struct FocusTerminal {
    terminal_handle: WeakModelHandle<Terminal>,
}

actions!(terminal, [DeployTerminalMenu]);
impl_internal_actions!(terminal, [FocusTerminal]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(TerminalButton::deploy_terminal_menu);
    cx.add_action(TerminalButton::focus_terminal);
}

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

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let workspace = self.workspace.upgrade(cx);
        let project = match workspace {
            Some(workspace) => workspace.read(cx).project().read(cx),
            None => return Empty::new().boxed(),
        };
        let has_terminals = !project.local_terminal_handles().is_empty();
        let terminal_count = project.local_terminal_handles().iter().count();
        let theme = cx.global::<Settings>().theme.clone();

        Stack::new()
            .with_child(
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
                    // TODO: Do we need this stuff?
                    // let dock_pane = workspace.read(cx.app).dock_pane();
                    // let drop_index = dock_pane.read(cx.app).items_len() + 1;
                    // handle_dropped_item(event, &dock_pane.downgrade(), drop_index, false, None, cx);
                })
                .on_click(MouseButton::Left, move |_, cx| {
                    if has_terminals {
                        cx.dispatch_action(DeployTerminalMenu);
                        println!("Yes, has_terminals {}", terminal_count);
                    } else {
                        cx.dispatch_action(FocusDock);
                    };
                })
                .with_tooltip::<Self, _>(
                    0,
                    "Show Terminal".into(),
                    Some(Box::new(FocusDock)),
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed(),
            )
            .with_child(ChildView::new(&self.popup_menu, cx).boxed())
            .boxed()
    }
}

// TODO: Rename this to `DeployTerminalButton`
impl TerminalButton {
    pub fn new(workspace: ViewHandle<Workspace>, cx: &mut ViewContext<Self>) -> Self {
        // When terminal moves, redraw so that the icon and toggle status matches.
        cx.subscribe(&workspace, |_, _, _, cx| cx.notify()).detach();
        Self {
            workspace: workspace.downgrade(),
            popup_menu: cx.add_view(|cx| {
                let mut menu = ContextMenu::new(cx);
                menu.set_position_mode(OverlayPositionMode::Local);
                menu
            }),
        }
    }

    pub fn deploy_terminal_menu(
        &mut self,
        _action: &DeployTerminalMenu,
        cx: &mut ViewContext<Self>,
    ) {
        let mut menu_options = vec![ContextMenuItem::item("New Terminal", NewTerminal)];

        if let Some(workspace) = self.workspace.upgrade(cx) {
            let project = workspace.read(cx).project().read(cx);
            let local_terminal_handles = project.local_terminal_handles();

            if !local_terminal_handles.is_empty() {
                menu_options.push(ContextMenuItem::Separator)
            }

            for local_terminal_handle in local_terminal_handles {
                if let Some(terminal) = local_terminal_handle.upgrade(cx) {
                    menu_options.push(ContextMenuItem::item(
                        terminal.read(cx).title(),
                        FocusTerminal {
                            terminal_handle: local_terminal_handle.clone(),
                        },
                    ))
                }
            }
        }

        self.popup_menu.update(cx, |menu, cx| {
            menu.show(vec2f(0., 0.), AnchorCorner::TopRight, menu_options, cx);
        });
    }

    pub fn focus_terminal(&mut self, action: &FocusTerminal, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade(cx) {
            workspace.update(cx, |workspace, cx| {
                let terminal = workspace
                    .items_of_type::<TerminalView>(cx)
                    .find(|terminal| {
                        terminal.read(cx).model().downgrade() == action.terminal_handle
                    });
                if let Some(terminal) = terminal {
                    workspace.activate_item(&terminal, cx);
                }
            });
        }
    }
}

impl StatusItemView for TerminalButton {
    fn set_active_pane_item(&mut self, _: Option<&dyn ItemHandle>, _: &mut ViewContext<Self>) {}
}
