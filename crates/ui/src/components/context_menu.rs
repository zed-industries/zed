#![allow(missing_docs)]
use crate::{
    h_flex, prelude::*, utils::WithRemSize, v_flex, Icon, IconName, IconSize, KeyBinding, Label,
    List, ListItem, ListSeparator, ListSubHeader,
};
use gpui::{
    px, Action, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    IntoElement, Render, Subscription, View, VisualContext,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};
use settings::Settings;
use std::{rc::Rc, time::Duration};
use theme::ThemeSettings;

pub enum ContextMenuItem {
    Separator,
    Header(SharedString),
    Label(SharedString),
    Entry(ContextMenuEntry),
    CustomEntry {
        entry_render: Box<dyn Fn(&mut WindowContext) -> AnyElement>,
        handler: Rc<dyn Fn(Option<&FocusHandle>, &mut WindowContext)>,
        selectable: bool,
    },
}

impl ContextMenuItem {
    pub fn custom_entry(
        entry_render: impl Fn(&mut WindowContext) -> AnyElement + 'static,
        handler: impl Fn(&mut WindowContext) + 'static,
    ) -> Self {
        Self::CustomEntry {
            entry_render: Box::new(entry_render),
            handler: Rc::new(move |_, cx| handler(cx)),
            selectable: true,
        }
    }
}

pub struct ContextMenuEntry {
    toggle: Option<(IconPosition, bool)>,
    label: SharedString,
    icon: Option<IconName>,
    icon_size: IconSize,
    icon_position: IconPosition,
    handler: Rc<dyn Fn(Option<&FocusHandle>, &mut WindowContext)>,
    action: Option<Box<dyn Action>>,
    disabled: bool,
}

impl ContextMenuEntry {
    pub fn new(label: impl Into<SharedString>) -> Self {
        ContextMenuEntry {
            toggle: None,
            label: label.into(),
            icon: None,
            icon_size: IconSize::Small,
            icon_position: IconPosition::Start,
            handler: Rc::new(|_, _| {}),
            action: None,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn icon_position(mut self, position: IconPosition) -> Self {
        self.icon_position = position;
        self
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = icon_size;
        self
    }

    pub fn toggle(mut self, toggle_position: IconPosition, toggled: bool) -> Self {
        self.toggle = Some((toggle_position, toggled));
        self
    }

    pub fn action(mut self, action: Option<Box<dyn Action>>) -> Self {
        self.action = action;
        self
    }

    pub fn handler(mut self, handler: impl Fn(&mut WindowContext) + 'static) -> Self {
        self.handler = Rc::new(move |_, cx| handler(cx));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl From<ContextMenuEntry> for ContextMenuItem {
    fn from(entry: ContextMenuEntry) -> Self {
        ContextMenuItem::Entry(entry)
    }
}

pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    focus_handle: FocusHandle,
    action_context: Option<FocusHandle>,
    selected_index: Option<usize>,
    delayed: bool,
    clicked: bool,
    _on_blur_subscription: Subscription,
    keep_open_on_confirm: bool,
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
        f: impl FnOnce(Self, &mut ViewContext<Self>) -> Self,
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
                    keep_open_on_confirm: false,
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

    pub fn extend<I: Into<ContextMenuItem>>(mut self, items: impl IntoIterator<Item = I>) -> Self {
        self.items.extend(items.into_iter().map(Into::into));
        self
    }

    pub fn item(mut self, item: impl Into<ContextMenuItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn entry(
        mut self,
        label: impl Into<SharedString>,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut WindowContext) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            handler: Rc::new(move |_, cx| handler(cx)),
            icon: None,
            icon_size: IconSize::Small,
            icon_position: IconPosition::End,
            action,
            disabled: false,
        }));
        self
    }

    pub fn toggleable_entry(
        mut self,
        label: impl Into<SharedString>,
        toggled: bool,
        position: IconPosition,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut WindowContext) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: Some((position, toggled)),
            label: label.into(),
            handler: Rc::new(move |_, cx| handler(cx)),
            icon: None,
            icon_size: IconSize::Small,
            icon_position: position,
            action,
            disabled: false,
        }));
        self
    }

    pub fn custom_row(
        mut self,
        entry_render: impl Fn(&mut WindowContext) -> AnyElement + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::CustomEntry {
            entry_render: Box::new(entry_render),
            handler: Rc::new(|_, _| {}),
            selectable: false,
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
            handler: Rc::new(move |_, cx| handler(cx)),
            selectable: true,
        });
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.items.push(ContextMenuItem::Label(label.into()));
        self
    }

    pub fn action(mut self, label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            action: Some(action.boxed_clone()),

            handler: Rc::new(move |context, cx| {
                if let Some(context) = &context {
                    cx.focus(context);
                }
                cx.dispatch_action(action.boxed_clone());
            }),
            icon: None,
            icon_position: IconPosition::End,
            icon_size: IconSize::Small,
            disabled: false,
        }));
        self
    }

    pub fn disabled_action(
        mut self,
        label: impl Into<SharedString>,
        action: Box<dyn Action>,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            action: Some(action.boxed_clone()),

            handler: Rc::new(move |context, cx| {
                if let Some(context) = &context {
                    cx.focus(context);
                }
                cx.dispatch_action(action.boxed_clone());
            }),
            icon: None,
            icon_size: IconSize::Small,
            icon_position: IconPosition::End,
            disabled: true,
        }));
        self
    }

    pub fn link(mut self, label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),

            action: Some(action.boxed_clone()),
            handler: Rc::new(move |_, cx| cx.dispatch_action(action.boxed_clone())),
            icon: Some(IconName::ArrowUpRight),
            icon_size: IconSize::XSmall,
            icon_position: IconPosition::End,
            disabled: false,
        }));
        self
    }

    pub fn keep_open_on_confirm(mut self) -> Self {
        self.keep_open_on_confirm = true;
        self
    }

    pub fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let context = self.action_context.as_ref();
        if let Some(
            ContextMenuItem::Entry(ContextMenuEntry {
                handler,
                disabled: false,
                ..
            })
            | ContextMenuItem::CustomEntry { handler, .. },
        ) = self.selected_index.and_then(|ix| self.items.get(ix))
        {
            (handler)(context, cx)
        }

        if !self.keep_open_on_confirm {
            cx.emit(DismissEvent);
        }
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
            let next_index = ix + 1;
            if self.items.len() <= next_index {
                self.select_first(&SelectFirst, cx);
            } else {
                for (ix, item) in self.items.iter().enumerate().skip(next_index) {
                    if item.is_selectable() {
                        self.selected_index = Some(ix);
                        cx.notify();
                        break;
                    }
                }
            }
        } else {
            self.select_first(&SelectFirst, cx);
        }
    }

    pub fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selected_index {
            if ix == 0 {
                self.handle_select_last(&SelectLast, cx);
            } else {
                for (ix, item) in self.items.iter().enumerate().take(ix).rev() {
                    if item.is_selectable() {
                        self.selected_index = Some(ix);
                        cx.notify();
                        break;
                    }
                }
            }
        } else {
            self.handle_select_last(&SelectLast, cx);
        }
    }

    pub fn on_action_dispatch(&mut self, dispatched: &dyn Action, cx: &mut ViewContext<Self>) {
        if self.clicked {
            cx.propagate();
            return;
        }

        if let Some(ix) = self.items.iter().position(|item| {
            if let ContextMenuItem::Entry(ContextMenuEntry {
                action: Some(action),
                disabled: false,
                ..
            }) = item
            {
                action.partial_eq(dispatched)
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

    pub fn on_blur_subscription(mut self, new_subscription: Subscription) -> Self {
        self._on_blur_subscription = new_subscription;
        self
    }
}

impl ContextMenuItem {
    fn is_selectable(&self) -> bool {
        match self {
            ContextMenuItem::Header(_)
            | ContextMenuItem::Separator
            | ContextMenuItem::Label { .. } => false,
            ContextMenuItem::Entry(ContextMenuEntry { disabled, .. }) => !disabled,
            ContextMenuItem::CustomEntry { selectable, .. } => *selectable,
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;

        WithRemSize::new(ui_font_size)
            .occlude()
            .elevation_2(cx)
            .flex()
            .flex_row()
            .child(
                v_flex()
                    .id("context-menu")
                    .min_w(px(200.))
                    .max_h(vh(0.75, cx))
                    .flex_1()
                    .overflow_y_scroll()
                    .track_focus(&self.focus_handle(cx))
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
                            if let ContextMenuItem::Entry(ContextMenuEntry {
                                action: Some(action),
                                disabled: false,
                                ..
                            }) = item
                            {
                                el = el.on_boxed_action(
                                    &**action,
                                    cx.listener(ContextMenu::on_action_dispatch),
                                );
                            }
                        }
                        el
                    })
                    .child(List::new().children(self.items.iter_mut().enumerate().map(
                        |(ix, item)| {
                            match item {
                                ContextMenuItem::Separator => ListSeparator.into_any_element(),
                                ContextMenuItem::Header(header) => {
                                    ListSubHeader::new(header.clone())
                                        .inset(true)
                                        .into_any_element()
                                }
                                ContextMenuItem::Label(label) => ListItem::new(ix)
                                    .inset(true)
                                    .disabled(true)
                                    .child(Label::new(label.clone()))
                                    .into_any_element(),
                                ContextMenuItem::Entry(ContextMenuEntry {
                                    toggle,
                                    label,
                                    handler,
                                    icon,
                                    icon_size,
                                    icon_position,
                                    action,
                                    disabled,
                                }) => {
                                    let handler = handler.clone();
                                    let menu = cx.view().downgrade();
                                    let color = if *disabled {
                                        Color::Muted
                                    } else {
                                        Color::Default
                                    };
                                    let label_element = if let Some(icon_name) = icon {
                                        h_flex()
                                            .gap_1()
                                            .when(*icon_position == IconPosition::Start, |flex| {
                                                flex.child(
                                                    Icon::new(*icon_name)
                                                        .size(*icon_size)
                                                        .color(color),
                                                )
                                            })
                                            .child(Label::new(label.clone()).color(color))
                                            .when(*icon_position == IconPosition::End, |flex| {
                                                flex.child(
                                                    Icon::new(*icon_name)
                                                        .size(*icon_size)
                                                        .color(color),
                                                )
                                            })
                                            .into_any_element()
                                    } else {
                                        Label::new(label.clone()).color(color).into_any_element()
                                    };

                                    ListItem::new(ix)
                                        .inset(true)
                                        .disabled(*disabled)
                                        .toggle_state(Some(ix) == self.selected_index)
                                        .when_some(*toggle, |list_item, (position, toggled)| {
                                            let contents = if toggled {
                                                v_flex().flex_none().child(
                                                    Icon::new(IconName::Check).color(Color::Accent),
                                                )
                                            } else {
                                                v_flex()
                                                    .flex_none()
                                                    .size(IconSize::default().rems())
                                            };
                                            match position {
                                                IconPosition::Start => {
                                                    list_item.start_slot(contents)
                                                }
                                                IconPosition::End => list_item.end_slot(contents),
                                            }
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
                                        .on_click({
                                            let context = self.action_context.clone();
                                            move |_, cx| {
                                                handler(context.as_ref(), cx);
                                                menu.update(cx, |menu, cx| {
                                                    menu.clicked = true;
                                                    cx.emit(DismissEvent);
                                                })
                                                .ok();
                                            }
                                        })
                                        .into_any_element()
                                }
                                ContextMenuItem::CustomEntry {
                                    entry_render,
                                    handler,
                                    selectable,
                                } => {
                                    let handler = handler.clone();
                                    let menu = cx.view().downgrade();
                                    let selectable = *selectable;
                                    ListItem::new(ix)
                                        .inset(true)
                                        .toggle_state(if selectable {
                                            Some(ix) == self.selected_index
                                        } else {
                                            false
                                        })
                                        .selectable(selectable)
                                        .when(selectable, |item| {
                                            item.on_click({
                                                let context = self.action_context.clone();
                                                move |_, cx| {
                                                    handler(context.as_ref(), cx);
                                                    menu.update(cx, |menu, cx| {
                                                        menu.clicked = true;
                                                        cx.emit(DismissEvent);
                                                    })
                                                    .ok();
                                                }
                                            })
                                        })
                                        .child(entry_render(cx))
                                        .into_any_element()
                                }
                            }
                        },
                    ))),
            )
    }
}
