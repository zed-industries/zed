#![allow(missing_docs)]
use crate::{
    h_flex, prelude::*, utils::WithRemSize, v_flex, Icon, IconName, KeyBinding, Label, List,
    ListItem, ListSeparator, ListSubHeader,
};
use gpui::{
    px, Action, AnyElement, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView,
    IntoElement, Modifiers, ModifiersChangedEvent, Render, Subscription, View, VisualContext,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};
use settings::Settings;
use std::{rc::Rc, time::Duration};
use theme::ThemeSettings;

enum ContextMenuItem {
    Separator,
    Header(SharedString),
    Label(SharedString),
    Entry {
        toggle: Option<(IconPosition, bool)>,
        label: SharedString,
        icon: Option<IconName>,
        handler: Rc<dyn Fn(Option<&FocusHandle>, &mut WindowContext)>,
        action: Option<Box<dyn Action>>,
        disabled: bool,
    },
    AdvancedEntry {
        toggle: Option<(IconPosition, bool)>,
        label: SharedString,
        icon: Option<IconName>,
        handler: Rc<dyn Fn(Option<&FocusHandle>, &mut WindowContext)>,
        action: Option<Box<dyn Action>>,
        disabled: bool,
        /// Allows the advanced entry to specify a entry to hide when it is visible, enabling a "swapping" effect between the two.
        hide_other_entry_when_visible: Option<SharedString>,
    },
    CustomEntry {
        entry_render: Box<dyn Fn(&mut WindowContext) -> AnyElement>,
        handler: Rc<dyn Fn(Option<&FocusHandle>, &mut WindowContext)>,
        selectable: bool,
    },
}

pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    item_visibility: Vec<bool>,
    focus_handle: FocusHandle,
    action_context: Option<FocusHandle>,
    selected_index: Option<usize>,
    show_advanced: bool,
    delayed: bool,
    clicked: bool,
    init_modifiers: Option<Modifiers>,
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
        f: impl FnOnce(Self, &mut ViewContext<Self>) -> Self,
    ) -> View<Self> {
        cx.new_view(|cx| {
            let focus_handle = cx.focus_handle();
            let _on_blur_subscription = cx.on_blur(&focus_handle, |this: &mut ContextMenu, cx| {
                this.cancel(&menu::Cancel, cx)
            });
            cx.refresh();
            let mut menu = Self {
                items: Default::default(),
                item_visibility: Default::default(),
                focus_handle,
                action_context: None,
                selected_index: None,
                delayed: false,
                clicked: false,
                show_advanced: false,
                init_modifiers: None,
                _on_blur_subscription,
            };
            menu = f(menu, cx);
            menu.item_visibility = vec![true; menu.items.len()];
            menu.handle_update_items(cx);
            menu
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
            toggle: None,
            label: label.into(),
            handler: Rc::new(move |_, cx| handler(cx)),
            icon: None,
            action,
            disabled: false,
        });
        self
    }

    /// Adds an entry that is only visible when `show_advanced` is enabled.
    pub fn advanced_entry(
        mut self,
        label: impl Into<SharedString>,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut WindowContext) + 'static,
        hidden_entry_when_visible: impl Into<Option<SharedString>>,
    ) -> Self {
        self.items.push(ContextMenuItem::AdvancedEntry {
            toggle: None,
            label: label.into(),
            handler: Rc::new(move |_, cx| handler(cx)),
            icon: None,
            action,
            disabled: false,
            hide_other_entry_when_visible: hidden_entry_when_visible.into(),
        });
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
        self.items.push(ContextMenuItem::Entry {
            toggle: Some((position, toggled)),
            label: label.into(),
            handler: Rc::new(move |_, cx| handler(cx)),
            icon: None,
            action,
            disabled: false,
        });
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
        self.items.push(ContextMenuItem::Entry {
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
            disabled: false,
        });
        self
    }

    pub fn advanced_action(
        mut self,
        label: impl Into<SharedString>,
        action: Box<dyn Action>,
        hidden_entry_when_visible: impl Into<Option<SharedString>>,
    ) -> Self {
        self.items.push(ContextMenuItem::AdvancedEntry {
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
            disabled: false,
            hide_other_entry_when_visible: hidden_entry_when_visible.into(),
        });
        self
    }

    pub fn disabled_action(
        mut self,
        label: impl Into<SharedString>,
        action: Box<dyn Action>,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry {
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
            disabled: true,
        });
        self
    }

    pub fn link(mut self, label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        self.items.push(ContextMenuItem::Entry {
            toggle: None,
            label: label.into(),

            action: Some(action.boxed_clone()),
            handler: Rc::new(move |_, cx| cx.dispatch_action(action.boxed_clone())),
            icon: Some(IconName::ArrowUpRight),
            disabled: false,
        });
        self
    }

    pub fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        let context = self.action_context.as_ref();
        if let Some(
            ContextMenuItem::Entry {
                handler,
                disabled: false,
                ..
            }
            | ContextMenuItem::CustomEntry { handler, .. },
        ) = self.selected_index.and_then(|ix| self.items.get(ix))
        {
            (handler)(context, cx)
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

    pub fn on_action_dispatch(&mut self, dispatched: &dyn Action, cx: &mut ViewContext<Self>) {
        if self.clicked {
            cx.propagate();
            return;
        }

        if let Some(ix) = self.items.iter().position(|item| {
            if let ContextMenuItem::Entry {
                action: Some(action),
                disabled: false,
                ..
            } = item
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

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(init_modifiers) = self.init_modifiers else {
            self.init_modifiers = Some(event.modifiers);
            return;
        };

        if event.modifiers.alt != init_modifiers.alt {
            self.show_advanced(event.modifiers.alt, cx);
        }

        self.init_modifiers = Some(event.modifiers);
    }

    fn show_advanced(&mut self, show_advanced: bool, cx: &mut ViewContext<Self>) {
        self.show_advanced = show_advanced;
        self.handle_update_items(cx);
    }

    fn handle_update_items(&mut self, cx: &mut ViewContext<Self>) {
        self.item_visibility.clear();
        self.item_visibility.reserve(self.items.len());

        for item in &self.items {
            match item {
                ContextMenuItem::AdvancedEntry {
                    hide_other_entry_when_visible,
                    ..
                } => {
                    let visible = self.show_advanced;
                    self.item_visibility.push(visible);
                    if visible {
                        if let Some(hidden_label) = hide_other_entry_when_visible {
                            // Hide the corresponding entry
                            if let Some(pos) = self.items.iter().position(|i| {
                                matches!(i, ContextMenuItem::Entry { label, .. } if label == hidden_label)
                            }) {
                                self.item_visibility[pos] = false;
                            }
                        }
                    }
                }
                ContextMenuItem::Entry { label, .. } => {
                    let visible = !self.show_advanced || !self.items.iter().any(|i| {
                        matches!(i, ContextMenuItem::AdvancedEntry { hide_other_entry_when_visible: Some(hidden), .. } if hidden == label)
                    });
                    self.item_visibility.push(visible);
                }
                _ => self.item_visibility.push(true),
            }
        }

        cx.notify();
    }
}

impl ContextMenuItem {
    fn is_selectable(&self) -> bool {
        match self {
            ContextMenuItem::Header(_)
            | ContextMenuItem::Separator
            | ContextMenuItem::Label { .. } => false,
            ContextMenuItem::Entry { disabled, .. } => !disabled,
            ContextMenuItem::AdvancedEntry { disabled, .. } => !disabled,
            ContextMenuItem::CustomEntry { selectable, .. } => *selectable,
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size;

        println!("show_advanced: {}", self.show_advanced);
        println!("hidden_items: {:#?}", self.item_visibility);

        div().occlude().elevation_2(cx).flex().flex_row().child(
            WithRemSize::new(ui_font_size).flex().child(
                v_flex()
                    .id("context-menu")
                    .min_w(px(200.))
                    .max_h(vh(0.75, cx))
                    .overflow_y_scroll()
                    .track_focus(&self.focus_handle)
                    .on_mouse_down_out(cx.listener(|this, _, cx| this.cancel(&menu::Cancel, cx)))
                    .key_context("menu")
                    .on_action(cx.listener(ContextMenu::select_first))
                    .on_action(cx.listener(ContextMenu::handle_select_last))
                    .on_action(cx.listener(ContextMenu::select_next))
                    .on_action(cx.listener(ContextMenu::select_prev))
                    .on_action(cx.listener(ContextMenu::confirm))
                    .on_action(cx.listener(ContextMenu::cancel))
                    .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
                    .when(!self.delayed, |mut el| {
                        for item in self.items.iter() {
                            if let ContextMenuItem::Entry {
                                action: Some(action),
                                disabled: false,
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
                    .child(
                        List::new().children(self.items.iter_mut().enumerate().filter_map(
                            |(ix, item)| {
                                if !self.item_visibility[ix] {
                                    return None;
                                }
                                Some(match item {
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
                                    ContextMenuItem::Entry {
                                        toggle,
                                        label,
                                        handler,
                                        icon,
                                        action,
                                        disabled,
                                    } => {
                                        let handler = handler.clone();
                                        let menu = cx.view().downgrade();
                                        let color = if *disabled {
                                            Color::Muted
                                        } else {
                                            Color::Default
                                        };
                                        let label_element = if let Some(icon) = icon {
                                            h_flex()
                                                .gap_1()
                                                .child(Label::new(label.clone()).color(color))
                                                .child(
                                                    Icon::new(*icon)
                                                        .size(IconSize::Small)
                                                        .color(color),
                                                )
                                                .into_any_element()
                                        } else {
                                            Label::new(label.clone())
                                                .color(color)
                                                .into_any_element()
                                        };

                                        ListItem::new(ix)
                                            .inset(true)
                                            .disabled(*disabled)
                                            .selected(Some(ix) == self.selected_index)
                                            .when_some(*toggle, |list_item, (position, toggled)| {
                                                let contents = if toggled {
                                                    v_flex().flex_none().child(
                                                        Icon::new(IconName::Check)
                                                            .color(Color::Accent),
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
                                                    IconPosition::End => {
                                                        list_item.end_slot(contents)
                                                    }
                                                }
                                            })
                                            .child(
                                                h_flex()
                                                    .w_full()
                                                    .justify_between()
                                                    .child(label_element)
                                                    .debug_selector(|| {
                                                        format!("MENU_ITEM-{}", label)
                                                    })
                                                    .children(action.as_ref().and_then(|action| {
                                                        self.action_context
                                                            .as_ref()
                                                            .map(|focus| {
                                                                KeyBinding::for_action_in(
                                                                    &**action, focus, cx,
                                                                )
                                                            })
                                                            .unwrap_or_else(|| {
                                                                KeyBinding::for_action(
                                                                    &**action, cx,
                                                                )
                                                            })
                                                            .map(|binding| {
                                                                div().ml_4().child(binding)
                                                            })
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
                                    ContextMenuItem::AdvancedEntry {
                                        toggle,
                                        label,
                                        handler,
                                        icon,
                                        action,
                                        disabled,
                                        ..
                                    } => {
                                        let handler = handler.clone();
                                        let menu = cx.view().downgrade();
                                        let color = if *disabled {
                                            Color::Muted
                                        } else {
                                            Color::Default
                                        };
                                        let label_element = if let Some(icon) = icon {
                                            h_flex()
                                                .gap_1()
                                                .child(Label::new(label.clone()).color(color))
                                                .child(
                                                    Icon::new(*icon)
                                                        .size(IconSize::Small)
                                                        .color(color),
                                                )
                                                .into_any_element()
                                        } else {
                                            Label::new(label.clone())
                                                .color(color)
                                                .into_any_element()
                                        };

                                        ListItem::new(ix)
                                            .inset(true)
                                            .disabled(*disabled)
                                            .selected(Some(ix) == self.selected_index)
                                            .when_some(*toggle, |list_item, (position, toggled)| {
                                                let contents = if toggled {
                                                    v_flex().flex_none().child(
                                                        Icon::new(IconName::Check)
                                                            .color(Color::Accent),
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
                                                    IconPosition::End => {
                                                        list_item.end_slot(contents)
                                                    }
                                                }
                                            })
                                            .child(
                                                h_flex()
                                                    .w_full()
                                                    .justify_between()
                                                    .child(label_element)
                                                    .debug_selector(|| {
                                                        format!("MENU_ITEM-{}", label)
                                                    })
                                                    .children(action.as_ref().and_then(|action| {
                                                        self.action_context
                                                            .as_ref()
                                                            .map(|focus| {
                                                                KeyBinding::for_action_in(
                                                                    &**action, focus, cx,
                                                                )
                                                            })
                                                            .unwrap_or_else(|| {
                                                                KeyBinding::for_action(
                                                                    &**action, cx,
                                                                )
                                                            })
                                                            .map(|binding| {
                                                                div().ml_4().child(binding)
                                                            })
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
                                            .selected(if selectable {
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
                                })
                            },
                        )),
                    ),
            ),
        )
    }
}
