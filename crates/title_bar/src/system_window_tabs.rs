use settings::{Settings, SettingsStore};

use gpui::{
    AnyWindowHandle, Context, Hsla, InteractiveElement, MouseButton, ParentElement, ScrollHandle,
    Styled, SystemWindowTab, SystemWindowTabController, Window, WindowId, actions, canvas, div,
};

use theme::ThemeSettings;
use ui::{
    Color, ContextMenu, DynamicSpacing, IconButton, IconButtonShape, IconName, IconSize, Label,
    LabelSize, Tab, h_flex, prelude::*, right_click_menu,
};
use workspace::{
    CloseWindow, ItemSettings, Workspace, WorkspaceSettings,
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
    pub ix: usize,
    pub handle: AnyWindowHandle,
    pub title: String,
    pub width: Pixels,
    pub is_active: bool,
    pub active_background_color: Hsla,
    pub inactive_background_color: Hsla,
}

pub struct SystemWindowTabs {
    tab_bar_scroll_handle: ScrollHandle,
    measured_tab_width: Pixels,
    last_dragged_tab: Option<DraggedWindowTab>,
}

impl SystemWindowTabs {
    pub fn new() -> Self {
        Self {
            tab_bar_scroll_handle: ScrollHandle::new(),
            measured_tab_width: px(0.),
            last_dragged_tab: None,
        }
    }

    pub fn init(cx: &mut App) {
        let mut was_use_system_window_tabs =
            WorkspaceSettings::get_global(cx).use_system_window_tabs;

        cx.observe_global::<SettingsStore>(move |cx| {
            let use_system_window_tabs = WorkspaceSettings::get_global(cx).use_system_window_tabs;
            if use_system_window_tabs == was_use_system_window_tabs {
                return;
            }
            was_use_system_window_tabs = use_system_window_tabs;

            let tabbing_identifier = if use_system_window_tabs {
                Some(String::from("zed"))
            } else {
                None
            };

            if use_system_window_tabs {
                SystemWindowTabController::init(cx);
            }

            cx.windows().iter().for_each(|handle| {
                let _ = handle.update(cx, |_, window, cx| {
                    window.set_tabbing_identifier(tabbing_identifier.clone());
                    if use_system_window_tabs {
                        let tabs = if let Some(tabs) = window.tabbed_windows() {
                            tabs
                        } else {
                            vec![SystemWindowTab::new(
                                SharedString::from(window.window_title()),
                                window.window_handle(),
                            )]
                        };

                        SystemWindowTabController::add_tab(cx, handle.window_id(), tabs);
                    }
                });
            });
        })
        .detach();

        cx.observe_new(|workspace: &mut Workspace, _, _| {
            workspace.register_action_renderer(|div, _, window, cx| {
                let window_id = window.window_handle().window_id();
                let controller = cx.global::<SystemWindowTabController>();

                let tab_groups = controller.tab_groups();
                let tabs = controller.tabs(window_id);
                let Some(tabs) = tabs else {
                    return div;
                };

                div.when(tabs.len() > 1, |div| {
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
                    .on_action(move |_: &MoveTabToNewWindow, window, cx| {
                        SystemWindowTabController::move_tab_to_new_window(
                            cx,
                            window.window_handle().window_id(),
                        );
                        window.move_tab_to_new_window();
                    })
                })
                .when(tab_groups.len() > 1, |div| {
                    div.on_action(move |_: &MergeAllWindows, window, cx| {
                        SystemWindowTabController::merge_all_windows(
                            cx,
                            window.window_handle().window_id(),
                        );
                        window.merge_all_windows();
                    })
                })
            });
        })
        .detach();
    }

    fn render_tab(
        &self,
        ix: usize,
        item: SystemWindowTab,
        tabs: Vec<SystemWindowTab>,
        active_background_color: Hsla,
        inactive_background_color: Hsla,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let entity = cx.entity();
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
                    ix,
                    handle: item.handle,
                    title: item.title.to_string(),
                    width,
                    is_active,
                    active_background_color,
                    inactive_background_color,
                },
                move |tab, _, _, cx| {
                    entity.update(cx, |this, _cx| {
                        this.last_dragged_tab = Some(tab.clone());
                    });
                    cx.new(|_| tab.clone())
                },
            )
            .drag_over::<DraggedWindowTab>({
                let tab_ix = ix;
                move |element, dragged_tab: &DraggedWindowTab, _, cx| {
                    let mut styled_tab = element
                        .bg(cx.theme().colors().drop_target_background)
                        .border_color(cx.theme().colors().drop_target_border)
                        .border_0();

                    if tab_ix < dragged_tab.ix {
                        styled_tab = styled_tab.border_l_2();
                    } else if tab_ix > dragged_tab.ix {
                        styled_tab = styled_tab.border_r_2();
                    }

                    styled_tab
                }
            })
            .on_drop({
                let tab_ix = ix;
                cx.listener(move |this, dragged_tab: &DraggedWindowTab, _window, cx| {
                    this.last_dragged_tab = None;
                    Self::handle_tab_drop(dragged_tab, tab_ix, cx);
                })
            })
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
                                    move |_, window, cx| {
                                        if item.handle.window_id()
                                            == window.window_handle().window_id()
                                        {
                                            window.dispatch_action(Box::new(CloseWindow), cx);
                                        } else {
                                            let _ = item.handle.update(cx, |_, window, cx| {
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

                    menu.context(focus_handle)
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
        SystemWindowTabController::update_tab_position(cx, dragged_tab.id, ix);
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
        let use_system_window_tabs = WorkspaceSettings::get_global(cx).use_system_window_tabs;
        let active_background_color = cx.theme().colors().title_bar_background;
        let inactive_background_color = cx.theme().colors().tab_bar_background;
        let entity = cx.entity();

        let controller = cx.global::<SystemWindowTabController>();
        let visible = controller.is_visible();
        let current_window_tab = vec![SystemWindowTab::new(
            SharedString::from(window.window_title()),
            window.window_handle(),
        )];
        let tabs = controller
            .tabs(window.window_handle().window_id())
            .unwrap_or(&current_window_tab)
            .clone();

        let tab_items = tabs
            .iter()
            .enumerate()
            .map(|(ix, item)| {
                self.render_tab(
                    ix,
                    item.clone(),
                    tabs.clone(),
                    active_background_color,
                    inactive_background_color,
                    window,
                    cx,
                )
            })
            .collect::<Vec<_>>();

        let number_of_tabs = tab_items.len().max(1);
        if (!window.tab_bar_visible() && !visible)
            || (!use_system_window_tabs && number_of_tabs == 1)
        {
            return h_flex().into_any_element();
        }

        h_flex()
            .w_full()
            .h(Tab::container_height(cx))
            .bg(inactive_background_color)
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _event, window, cx| {
                    if let Some(tab) = this.last_dragged_tab.take() {
                        SystemWindowTabController::move_tab_to_new_window(cx, tab.id);
                        if tab.id == window.window_handle().window_id() {
                            window.move_tab_to_new_window();
                        } else {
                            let _ = tab.handle.update(cx, |_, window, _cx| {
                                window.move_tab_to_new_window();
                            });
                        }
                    }
                }),
            )
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
        let ui_font = ThemeSettings::get_global(cx).ui_font.clone();
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
            .font(ui_font)
            .child(label)
    }
}
