use settings::Settings;

use gpui::{
    Context, Hsla, InteractiveElement, ParentElement, ScrollHandle, Styled, Subscription,
    SystemWindowTab, SystemWindowTabController, Window, WindowId, actions, canvas, div,
};

use ui::{
    Color, ContextMenu, DynamicSpacing, IconButton, IconButtonShape, IconName, IconSize, Label,
    LabelSize, Tab, h_flex, prelude::*, right_click_menu,
};
use workspace::{
    CloseWindow, ItemSettings, Workspace,
    item::{ClosePosition, ShowCloseButton},
};

actions!(
    window,
    [
        ShowNextWindowTab,
        ShowPreviousWindowTab,
        MergeAllWindows,
        MoveTabToNewWindow
    ]
);

#[derive(Clone)]
pub struct DraggedWindowTab {
    pub id: WindowId,
    pub title: String,
    pub width: Pixels,
    pub is_active: bool,
    pub active_background_color: Hsla,
    pub inactive_background_color: Hsla,
}

pub struct SystemWindowTabs {
    tabs: Vec<SystemWindowTab>,
    tab_bar_scroll_handle: ScrollHandle,
    measured_tab_width: Pixels,
    _subscriptions: Vec<Subscription>,
}

impl SystemWindowTabs {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let window_id = window.window_handle().window_id();
        let mut subscriptions = Vec::new();

        subscriptions.push(
            cx.observe_global::<SystemWindowTabController>(move |this, cx| {
                let controller = cx.global::<SystemWindowTabController>();
                let tab_group = controller.tabs().iter().find_map(|(group, windows)| {
                    windows
                        .iter()
                        .find(|tab| tab.id == window_id)
                        .map(|_| *group)
                });

                if let Some(tab_group) = tab_group {
                    if let Some(windows) = controller.windows(tab_group) {
                        this.tabs = windows.clone();
                    }
                }
            }),
        );

        Self {
            tabs: Vec::new(),
            tab_bar_scroll_handle: ScrollHandle::new(),
            measured_tab_width: px(0.),
            _subscriptions: subscriptions,
        }
    }

    pub fn init(cx: &mut App) {
        cx.observe_new(|workspace: &mut Workspace, _, _| {
            workspace.register_action_renderer(|div, _, window, cx| {
                let window_id = window.window_handle().window_id();
                let controller = cx.global::<SystemWindowTabController>();
                let tab_group = controller.tabs().iter().find_map(|(group, windows)| {
                    windows
                        .iter()
                        .find(|tab| tab.id == window_id)
                        .map(|_| *group)
                });

                if let Some(tab_group) = tab_group {
                    let all_tab_groups = controller.tabs();
                    let tabs = controller.windows(tab_group);
                    let show_merge_all_windows = all_tab_groups.len() > 1;
                    let show_other_tab_actions = if let Some(tabs) = tabs {
                        tabs.len() > 1
                    } else {
                        false
                    };

                    return div
                        .when(show_other_tab_actions, |div| {
                            div.on_action(move |_: &ShowNextWindowTab, window, cx| {
                                SystemWindowTabController::select_next_tab(
                                    cx,
                                    window.window_handle().window_id(),
                                );
                            })
                            .on_action(move |_: &ShowPreviousWindowTab, window, cx| {
                                SystemWindowTabController::select_previous_tab(
                                    cx,
                                    window.window_handle().window_id(),
                                );
                            })
                            .on_action(
                                move |_: &MoveTabToNewWindow, window, cx| {
                                    SystemWindowTabController::move_tab_to_new_window(
                                        cx,
                                        window.window_handle().window_id(),
                                    );
                                    window.move_tab_to_new_window();
                                },
                            )
                        })
                        .when(show_merge_all_windows, |div| {
                            div.on_action(move |_: &MergeAllWindows, window, cx| {
                                SystemWindowTabController::merge_all_windows(
                                    cx,
                                    window.window_handle().window_id(),
                                );
                                window.merge_all_windows();
                            })
                        });
                }

                div
            });
        })
        .detach();
    }

    fn render_tab(
        &self,
        ix: usize,
        item: SystemWindowTab,
        active_background_color: Hsla,
        inactive_background_color: Hsla,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let settings = ItemSettings::get_global(cx);
        let close_side = &settings.close_position;
        let show_close_button = &settings.show_close_button;

        let rem_size = window.rem_size();
        let width = self.measured_tab_width.max(rem_size * 10);
        let is_active = window.window_handle().window_id() == item.id;
        let title = item.title.to_string();

        let label = Label::new(&title)
            .size(LabelSize::Small)
            .truncate()
            .color(if is_active {
                Color::Default
            } else {
                Color::Muted
            });

        let tab = h_flex()
            .id(ix)
            .group("tab")
            .w_full()
            .overflow_hidden()
            .h(Tab::content_height(cx))
            .relative()
            .px(DynamicSpacing::Base16.px(cx))
            .justify_center()
            .border_l_1()
            .border_color(cx.theme().colors().border)
            .cursor_pointer()
            .on_drag(
                DraggedWindowTab {
                    id: item.id,
                    title: item.title.to_string(),
                    width,
                    is_active,
                    active_background_color,
                    inactive_background_color,
                },
                |tab, _, _, cx| cx.new(|_| tab.clone()),
            )
            .drag_over::<DraggedWindowTab>(|element, _, _, cx| {
                element.bg(cx.theme().colors().drop_target_background)
            })
            .on_drop(
                cx.listener(move |_this, dragged_tab: &DraggedWindowTab, _window, cx| {
                    Self::handle_tab_drop(dragged_tab, ix, cx);
                }),
            )
            .on_click(move |_, _, cx| {
                let _ = item.handle.update(cx, |_, window, _| {
                    window.activate_window();
                });
            })
            .child(label)
            .map(|this| match show_close_button {
                ShowCloseButton::Hidden => this,
                _ => this.child(
                    div()
                        .absolute()
                        .top_2()
                        .w_4()
                        .h_4()
                        .map(|this| match close_side {
                            ClosePosition::Left => this.left_1(),
                            ClosePosition::Right => this.right_1(),
                        })
                        .child(
                            IconButton::new("close", IconName::Close)
                                .shape(IconButtonShape::Square)
                                .icon_color(Color::Muted)
                                .icon_size(IconSize::XSmall)
                                .on_click({
                                    let handle = item.handle.clone();
                                    move |_, window, cx| {
                                        if handle.window_id() == window.window_handle().window_id()
                                        {
                                            window.dispatch_action(Box::new(CloseWindow), cx);
                                        } else {
                                            let _ = handle.update(cx, |_, window, cx| {
                                                window.dispatch_action(Box::new(CloseWindow), cx);
                                            });
                                        }
                                    }
                                })
                                .map(|this| match show_close_button {
                                    ShowCloseButton::Hover => this.visible_on_hover("tab"),
                                    _ => this,
                                }),
                        ),
                ),
            })
            .into_any();

        let tabs = self.tabs.clone();
        let menu = right_click_menu(ix)
            .trigger(|_, _, _| tab)
            .menu(move |window, cx| {
                let focus_handle = cx.focus_handle();
                let tabs = tabs.clone();
                let other_tabs = tabs.clone();
                let move_tabs = tabs.clone();
                let merge_tabs = tabs.clone();

                ContextMenu::build(window, cx, move |mut menu, _window_, _cx| {
                    menu = menu.entry("Close Tab", None, move |window, cx| {
                        Self::handle_right_click_action(
                            cx,
                            window,
                            &tabs,
                            |tab| tab.id == item.id,
                            |window, cx| {
                                window.dispatch_action(Box::new(CloseWindow), cx);
                            },
                        );
                    });

                    menu = menu.entry("Close Other Tabs", None, move |window, cx| {
                        Self::handle_right_click_action(
                            cx,
                            window,
                            &other_tabs,
                            |tab| tab.id != item.id,
                            |window, cx| {
                                window.dispatch_action(Box::new(CloseWindow), cx);
                            },
                        );
                    });

                    menu = menu.entry("Move Tab to New Window", None, move |window, cx| {
                        Self::handle_right_click_action(
                            cx,
                            window,
                            &move_tabs,
                            |tab| tab.id == item.id,
                            |window, cx| {
                                SystemWindowTabController::move_tab_to_new_window(
                                    cx,
                                    window.window_handle().window_id(),
                                );
                                window.move_tab_to_new_window();
                            },
                        );
                    });

                    menu = menu.entry("Show All Tabs", None, move |window, cx| {
                        Self::handle_right_click_action(
                            cx,
                            window,
                            &merge_tabs,
                            |tab| tab.id == item.id,
                            |window, _cx| {
                                window.toggle_window_tab_overview();
                            },
                        );
                    });

                    menu.context(focus_handle.clone())
                })
            });

        div()
            .flex_1()
            .min_w(rem_size * 10)
            .when(is_active, |this| this.bg(active_background_color))
            .border_t_1()
            .border_color(if is_active {
                active_background_color
            } else {
                cx.theme().colors().border
            })
            .child(menu)
    }

    fn handle_tab_drop(dragged_tab: &DraggedWindowTab, ix: usize, cx: &mut Context<Self>) {
        SystemWindowTabController::update_window_position(cx, dragged_tab.id, ix);
    }

    fn handle_right_click_action<F, P>(
        cx: &mut App,
        window: &mut Window,
        tabs: &Vec<SystemWindowTab>,
        predicate: P,
        mut action: F,
    ) where
        P: Fn(&SystemWindowTab) -> bool,
        F: FnMut(&mut Window, &mut App),
    {
        for tab in tabs {
            if predicate(tab) {
                if tab.id == window.window_handle().window_id() {
                    action(window, cx);
                } else {
                    let _ = tab.handle.update(cx, |_view, window, cx| {
                        action(window, cx);
                    });
                }
            }
        }
    }
}

impl Render for SystemWindowTabs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let active_background_color = cx.theme().colors().title_bar_background;
        let inactive_background_color = cx.theme().colors().tab_bar_background;
        let entity = cx.entity();

        let tab_items = self
            .tabs
            .iter()
            .enumerate()
            .map(|(ix, item)| {
                self.render_tab(
                    ix,
                    item.clone(),
                    active_background_color,
                    inactive_background_color,
                    window,
                    cx,
                )
            })
            .collect::<Vec<_>>();

        let number_of_tabs = tab_items.len().max(1);
        if number_of_tabs <= 1 {
            return h_flex().into_any_element();
        }

        h_flex()
            .w_full()
            .h(Tab::container_height(cx))
            .bg(inactive_background_color)
            .child(
                h_flex()
                    .id("window tabs")
                    .w_full()
                    .h(Tab::container_height(cx))
                    .bg(inactive_background_color)
                    .overflow_x_scroll()
                    .track_scroll(&self.tab_bar_scroll_handle)
                    .children(tab_items)
                    .child(
                        canvas(
                            |_, _, _| (),
                            move |bounds, _, _, cx| {
                                let entity = entity.clone();
                                entity.update(cx, |this, cx| {
                                    let width = bounds.size.width / number_of_tabs as f32;
                                    if width != this.measured_tab_width {
                                        this.measured_tab_width = width;
                                        cx.notify();
                                    }
                                });
                            },
                        )
                        .absolute()
                        .size_full(),
                    ),
            )
            .child(
                h_flex()
                    .h_full()
                    .px(DynamicSpacing::Base06.rems(cx))
                    .border_t_1()
                    .border_l_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        IconButton::new("plus", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .on_click(|_event, window, cx| {
                                window.dispatch_action(
                                    Box::new(zed_actions::OpenRecent {
                                        create_new_window: true,
                                    }),
                                    cx,
                                );
                            }),
                    ),
            )
            .into_any_element()
    }
}

impl Render for DraggedWindowTab {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let label = Label::new(self.title.clone())
            .size(LabelSize::Small)
            .truncate()
            .color(if self.is_active {
                Color::Default
            } else {
                Color::Muted
            });

        h_flex()
            .h(Tab::container_height(cx))
            .w(self.width)
            .px(DynamicSpacing::Base16.px(cx))
            .justify_center()
            .bg(if self.is_active {
                self.active_background_color
            } else {
                self.inactive_background_color
            })
            .border_1()
            .border_color(cx.theme().colors().border)
            .child(label)
    }
}
