use crate::{
    h_stack, prelude::*, v_stack, Icon, IconElement, KeyBinding, Label, List, ListItem,
    ListSeparator, ListSubHeader,
};
use gpui::{
    px, Action, AppContext, DismissEvent, Div, EventEmitter, FocusHandle, FocusableView,
    IntoElement, Render, Subscription, View, VisualContext,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrev};
use std::{rc::Rc, time::Duration};

pub enum ContextMenuItem {
    Separator,
    Header(SharedString),
    Entry {
        label: SharedString,
        icon: Option<Icon>,
        handler: Rc<dyn Fn(&mut WindowContext)>,
        action: Option<Box<dyn Action>>,
    },
}

pub struct ContextMenu {
    items: Vec<ContextMenuItem>,
    focus_handle: FocusHandle,
    selected_index: Option<usize>,
    delayed: bool,
    _on_blur_subscription: Subscription,
}

impl FocusableView for ContextMenu {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ContextMenu {}

impl ContextMenu {
    pub fn build(
        cx: &mut WindowContext,
        f: impl FnOnce(Self, &mut WindowContext) -> Self,
    ) -> View<Self> {
        cx.build_view(|cx| {
            let focus_handle = cx.focus_handle();
            let _on_blur_subscription = cx.on_blur(&focus_handle, |this: &mut ContextMenu, cx| {
                this.cancel(&menu::Cancel, cx)
            });
            f(
                Self {
                    items: Default::default(),
                    focus_handle,
                    selected_index: None,
                    delayed: false,
                    _on_blur_subscription,
                },
                cx,
            )
        })
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
        on_click: impl Fn(&mut WindowContext) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry {
            label: label.into(),
            handler: Rc::new(on_click),
            icon: None,
            action: None,
        });
        self
    }

    pub fn action(mut self, label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        self.items.push(ContextMenuItem::Entry {
            label: label.into(),
            action: Some(action.boxed_clone()),
            handler: Rc::new(move |cx| cx.dispatch_action(action.boxed_clone())),
            icon: None,
        });
        self
    }

    pub fn link(mut self, label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        self.items.push(ContextMenuItem::Entry {
            label: label.into(),
            action: Some(action.boxed_clone()),
            handler: Rc::new(move |cx| cx.dispatch_action(action.boxed_clone())),
            icon: Some(Icon::Link),
        });
        self
    }

    pub fn confirm(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if let Some(ContextMenuItem::Entry { handler, .. }) =
            self.selected_index.and_then(|ix| self.items.get(ix))
        {
            (handler)(cx)
        }
        cx.emit(DismissEvent);
    }

    pub fn cancel(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }

    fn select_first(&mut self, _: &SelectFirst, cx: &mut ViewContext<Self>) {
        self.selected_index = self.items.iter().position(|item| item.is_selectable());
        cx.notify();
    }

    fn select_last(&mut self, _: &SelectLast, cx: &mut ViewContext<Self>) {
        for (ix, item) in self.items.iter().enumerate().rev() {
            if item.is_selectable() {
                self.selected_index = Some(ix);
                cx.notify();
                break;
            }
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
            self.select_last(&Default::default(), cx);
        }
    }

    pub fn on_action_dispatch(&mut self, dispatched: &Box<dyn Action>, cx: &mut ViewContext<Self>) {
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
                    cx.dispatch_action(action);
                    this.cancel(&Default::default(), cx)
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
        matches!(self, Self::Entry { .. })
    }
}

impl Render for ContextMenu {
    type Element = Div;

    fn render(&mut self, cx: &mut ViewContext<Self>) -> Self::Element {
        div().elevation_2(cx).flex().flex_row().child(
            v_stack()
                .min_w(px(200.))
                .track_focus(&self.focus_handle)
                .on_mouse_down_out(cx.listener(|this, _, cx| this.cancel(&Default::default(), cx)))
                .key_context("menu")
                .on_action(cx.listener(ContextMenu::select_first))
                .on_action(cx.listener(ContextMenu::select_last))
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
                                action,
                                cx.listener(ContextMenu::on_action_dispatch),
                            );
                        }
                    }
                    el
                })
                .flex_none()
                .child(
                    List::new().children(self.items.iter().enumerate().map(
                        |(ix, item)| match item {
                            ContextMenuItem::Separator => ListSeparator.into_any_element(),
                            ContextMenuItem::Header(header) => {
                                ListSubHeader::new(header.clone()).into_any_element()
                            }
                            ContextMenuItem::Entry {
                                label,
                                handler,
                                icon,
                                action,
                            } => {
                                let handler = handler.clone();

                                let label_element = if let Some(icon) = icon {
                                    h_stack()
                                        .gap_1()
                                        .child(Label::new(label.clone()))
                                        .child(IconElement::new(*icon))
                                        .into_any_element()
                                } else {
                                    Label::new(label.clone()).into_any_element()
                                };

                                ListItem::new(label.clone())
                                    .child(
                                        h_stack()
                                            .w_full()
                                            .justify_between()
                                            .child(label_element)
                                            .children(action.as_ref().and_then(|action| {
                                                KeyBinding::for_action(&**action, cx)
                                                    .map(|binding| div().ml_1().child(binding))
                                            })),
                                    )
                                    .selected(Some(ix) == self.selected_index)
                                    .on_click(move |_, cx| handler(cx))
                                    .into_any_element()
                            }
                        },
                    )),
                ),
        )
    }
}
