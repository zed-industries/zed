use crate::{
    h_flex, prelude::*, v_flex, Icon, IconName, KeyBinding, Label, List, ListItem, ListSeparator,
    ListSubHeader, WithRemSize,
};
use gpui::{
    px, Action, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    IntoElement, Render, Subscription, View, VisualContext,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};
use settings::Settings;
use std::{rc::Rc, time::Duration};
use theme::ThemeSettings;

enum ContextMenuItem {
    Separator,
    Header(SharedString),
    Entry {
        toggled: Option<bool>,
        label: SharedString,
        icon: Option<IconName>,
        handler: Rc<dyn Fn(&mut WindowContext)>,
        action: Option<Box<dyn Action>>,
    },
    CustomEntry {
        entry_render: Box<dyn Fn(&mut WindowContext) -> AnyElement>,
        handler: Rc<dyn Fn(&mut WindowContext)>,
    },
}

pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    focus_handle: FocusHandle,
    action_context: Option<FocusHandle>,
    selected_index: Option<usize>,
    delayed: bool,
    clicked: bool,
    _on_blur_subscription: Subscription,
}

impl FocusableView for ContextMenu {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ContextMenu {}

impl FluentBuilder for ContextMenu {}

impl ContextMenu {
    pub fn build(
        cx: &mut WindowContext,
        f: impl FnOnce(Self, &mut WindowContext) -> Self,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let focus_handle = cx.focus_handle();
            let _on_blur_subscription = cx.on_blur(&focus_handle, |this: &mut ContextMenu, cx| {
                this.cancel(&menu::Cancel, cx)
            });
            cx.refresh();
            f(
                Self {
                    items: Default::default(),
                    focus_handle,
                    action_context: None,
                    selected_index: None,
                    delayed: false,
                    clicked: false,
                    _on_blur_subscription,
                },
                cx,
            )
        })
    }

    pub fn context(mut self, focus: FocusHandle) -> Self {
        self.action_context = Some(focus);
        self
    }

    pub fn header(mut self, title: impl Into<SharedString>) -> Self {
        self.items.push(ContextMenuItem::Header(title.into()));
        self
    }

    pub fn separator(mut self) -> Self {
        self.items.push(ContextMenuItem::Separator);
        self
    }

    pub fn entry(
        mut self,
        label: impl Into<SharedString>,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut WindowContext) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry {
            toggled: None,
            label: label.into(),
            handler: Rc::new(handler),
            icon: None,
            action,
        });
        self
    }

    pub fn toggleable_entry(
        mut self,
        label: impl Into<SharedString>,
        toggled: bool,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut WindowContext) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry {
            toggled: Some(toggled),
            label: label.into(),
            handler: Rc::new(handler),
            icon: None,
            action,
        });
        self
    }

    pub fn custom_entry(
        mut self,
        entry_render: impl Fn(&mut WindowContext) -> AnyElement + 'static,
        handler: impl Fn(&mut WindowContext) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::CustomEntry {
            entry_render: Box::new(entry_render),
            handler: Rc::new(handler),
        });
        self
    }

    pub fn action(mut self, label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        self.items.push(ContextMenuItem::Entry {
            toggled: None,
            label: label.into(),
            action: Some(action.boxed_clone()),
            handler: Rc::new(move |cx| cx.dispatch_action(action.boxed_clone())),
            icon: None,
        });
        self
    }

    pub fn link(mut self, label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        self.items.push(ContextMenuItem::Entry {
            toggled: None,
            label: label.into(),
            action: Some(action.boxed_clone()),
            handler: Rc::new(move |cx| cx.dispatch_action(action.boxed_clone())),
            icon: Some(IconName::Link),
        });
        self
    }

    pub fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        match self.selected_index.and_then(|ix| self.items.get(ix)) {
            Some(
                ContextMenuItem::Entry { handler, .. }
                | ContextMenuItem::CustomEntry { handler, .. },
            ) => (handler)(cx),
            _ => {}
        }

        cx.emit(DismissEvent);
    }

    pub fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
        cx.emit(DismissEvent);
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        self.selected_index = self.items.iter().position(|item| item.is_selectable());
        cx.notify();
    }

    pub fn select_last(&mut self) -> Option<usize> {
        for (ix, item) in self.items.iter().enumerate().rev() {
            if item.is_selectable() {
                self.selected_index = Some(ix);
                return Some(ix);
            }
        }
        None
    }

    fn handle_select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        if self.select_last().is_some() {
            cx.notify();
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            for (ix, item) in self.items.iter().enumerate().skip(ix + 1) {
                if item.is_selectable() {
                    self.selected_index = Some(ix);
                    cx.notify();
                    break;
                }
            }
        } else {
            self.select_first(&Default::default(), cx);
        }
    }

    pub fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            for (ix, item) in self.items.iter().enumerate().take(ix).rev() {
                if item.is_selectable() {
                    self.selected_index = Some(ix);
                    cx.notify();
                    break;
                }
            }
        } else {
            self.handle_select_last(&Default::default(), cx);
        }
    }

    pub fn on_action_dispatch(&mut self, dispatched: &Box<dyn Action>, cx: &mut ViewContext<Self>) {
        if self.clicked {
            cx.propagate();
            return;
        }

        if let Some(ix) = self.items.iter().position(|item| {
            if let ContextMenuItem::Entry {
                action: Some(action),
                ..
            } = item
            {
                action.partial_eq(&**dispatched)
            } else {
                false
            }
        }) {
            self.selected_index = Some(ix);
            self.delayed = true;
            cx.notify();
            let action = dispatched.boxed_clone();
            cx.spawn(|this, mut cx| async move {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                this.update(&mut cx, |this, cx| {
                    this.cancel(&menu::Cancel, cx);
                    cx.dispatch_action(action);
                })
            })
            .detach_and_log_err(cx);
        } else {
            cx.propagate()
        }
    }
}

impl ContextMenuItem {
    fn is_selectable(&self) -> bool {
        matches!(self, Self::Entry { .. } | Self::CustomEntry { .. })
    }
}

impl Render for ContextMenu {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;

        div().occlude().elevation_2(cx).flex().flex_row().child(
            WithRemSize::new(ui_font_size).flex().child(
                v_flex()
                    .min_w(px(200.))
                    .track_focus(&self.focus_handle)
                    .on_mouse_down_out(cx.listener(|this, _, cx| this.cancel(&menu::Cancel, cx)))
                    .key_context("menu")
                    .on_action(cx.listener(ContextMenu::select_first))
                    .on_action(cx.listener(ContextMenu::handle_select_last))
                    .on_action(cx.listener(ContextMenu::select_next))
                    .on_action(cx.listener(ContextMenu::select_prev))
                    .on_action(cx.listener(ContextMenu::confirm))
                    .on_action(cx.listener(ContextMenu::cancel))
                    .when(!self.delayed, |mut el| {
                        for item in self.items.iter() {
                            if let ContextMenuItem::Entry {
                                action: Some(action),
                                ..
                            } = item
                            {
                                el = el.on_boxed_action(
                                    &**action,
                                    cx.listener(ContextMenu::on_action_dispatch),
                                );
                            }
                        }
                        el
                    })
                    .flex_none()
                    .child(List::new().children(self.items.iter_mut().enumerate().map(
                        |(ix, item)| {
                            match item {
                                ContextMenuItem::Separator => ListSeparator.into_any_element(),
                                ContextMenuItem::Header(header) => {
                                    ListSubHeader::new(header.clone())
                                        .inset(true)
                                        .into_any_element()
                                }
                                ContextMenuItem::Entry {
                                    toggled,
                                    label,
                                    handler,
                                    icon,
                                    action,
                                } => {
                                    let handler = handler.clone();
                                    let menu = cx.view().downgrade();

                                    let label_element = if let Some(icon) = icon {
                                        h_flex()
                                            .gap_1()
                                            .child(Label::new(label.clone()))
                                            .child(Icon::new(*icon))
                                            .into_any_element()
                                    } else {
                                        Label::new(label.clone()).into_any_element()
                                    };

                                    ListItem::new(ix)
                                        .inset(true)
                                        .selected(Some(ix) == self.selected_index)
                                        .when_some(*toggled, |list_item, toggled| {
                                            list_item.start_slot(if toggled {
                                                v_flex().flex_none().child(
                                                    Icon::new(IconName::Check).color(Color::Accent),
                                                )
                                            } else {
                                                v_flex()
                                                    .flex_none()
                                                    .size(IconSize::default().rems())
                                            })
                                        })
                                        .child(
                                            h_flex()
                                                .w_full()
                                                .justify_between()
                                                .child(label_element)
                                                .debug_selector(|| format!("MENU_ITEM-{}", label))
                                                .children(action.as_ref().and_then(|action| {
                                                    self.action_context
                                                        .as_ref()
                                                        .map(|focus| {
                                                            KeyBinding::for_action_in(
                                                                &**action, focus, cx,
                                                            )
                                                        })
                                                        .unwrap_or_else(|| {
                                                            KeyBinding::for_action(&**action, cx)
                                                        })
                                                        .map(|binding| div().ml_4().child(binding))
                                                })),
                                        )
                                        .on_click(move |_, cx| {
                                            handler(cx);
                                            menu.update(cx, |menu, cx| {
                                                menu.clicked = true;
                                                cx.emit(DismissEvent);
                                            })
                                            .ok();
                                        })
                                        .into_any_element()
                                }
                                ContextMenuItem::CustomEntry {
                                    entry_render,
                                    handler,
                                } => {
                                    let handler = handler.clone();
                                    let menu = cx.view().downgrade();
                                    ListItem::new(ix)
                                        .inset(true)
                                        .selected(Some(ix) == self.selected_index)
                                        .on_click(move |_, cx| {
                                            handler(cx);
                                            menu.update(cx, |menu, cx| {
                                                menu.clicked = true;
                                                cx.emit(DismissEvent);
                                            })
                                            .ok();
                                        })
                                        .child(entry_render(cx))
                                        .into_any_element()
                                }
                            }
                        },
                    ))),
            ),
        )
    }
}
