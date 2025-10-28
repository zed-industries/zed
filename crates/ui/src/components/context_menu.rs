use crate::{
    Icon, IconButtonShape, IconName, IconSize, KeyBinding, Label, List, ListItem, ListSeparator,
    ListSubHeader, h_flex, prelude::*, utils::WithRemSize, v_flex,
};
use gpui::{
    Action, AnyElement, App, AppContext as _, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, Subscription, px,
};
use menu::{SelectFirst, SelectLast, SelectNext, SelectPrevious};
use settings::Settings;
use std::{rc::Rc, time::Duration};
use theme::ThemeSettings;

use super::Tooltip;

pub enum ContextMenuItem {
    Separator,
    Header(SharedString),
    /// title, link_label, link_url
    HeaderWithLink(SharedString, SharedString, SharedString), // This could be folded into header
    Label(SharedString),
    Entry(ContextMenuEntry),
    CustomEntry {
        entry_render: Box<dyn Fn(&mut Window, &mut App) -> AnyElement>,
        handler: Rc<dyn Fn(Option<&FocusHandle>, &mut Window, &mut App)>,
        selectable: bool,
        documentation_aside: Option<DocumentationAside>,
    },
}

impl ContextMenuItem {
    pub fn custom_entry(
        entry_render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
        handler: impl Fn(&mut Window, &mut App) + 'static,
        documentation_aside: Option<DocumentationAside>,
    ) -> Self {
        Self::CustomEntry {
            entry_render: Box::new(entry_render),
            handler: Rc::new(move |_, window, cx| handler(window, cx)),
            selectable: true,
            documentation_aside,
        }
    }
}

pub struct ContextMenuEntry {
    toggle: Option<(IconPosition, bool)>,
    label: SharedString,
    icon: Option<IconName>,
    custom_icon_path: Option<SharedString>,
    icon_position: IconPosition,
    icon_size: IconSize,
    icon_color: Option<Color>,
    handler: Rc<dyn Fn(Option<&FocusHandle>, &mut Window, &mut App)>,
    action: Option<Box<dyn Action>>,
    disabled: bool,
    documentation_aside: Option<DocumentationAside>,
    end_slot_icon: Option<IconName>,
    end_slot_title: Option<SharedString>,
    end_slot_handler: Option<Rc<dyn Fn(Option<&FocusHandle>, &mut Window, &mut App)>>,
    show_end_slot_on_hover: bool,
}

impl ContextMenuEntry {
    pub fn new(label: impl Into<SharedString>) -> Self {
        ContextMenuEntry {
            toggle: None,
            label: label.into(),
            icon: None,
            custom_icon_path: None,
            icon_position: IconPosition::Start,
            icon_size: IconSize::Small,
            icon_color: None,
            handler: Rc::new(|_, _, _| {}),
            action: None,
            disabled: false,
            documentation_aside: None,
            end_slot_icon: None,
            end_slot_title: None,
            end_slot_handler: None,
            show_end_slot_on_hover: false,
        }
    }

    pub fn toggleable(mut self, toggle_position: IconPosition, toggled: bool) -> Self {
        self.toggle = Some((toggle_position, toggled));
        self
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn custom_icon_path(mut self, path: impl Into<SharedString>) -> Self {
        self.custom_icon_path = Some(path.into());
        self.icon = None; // Clear IconName if custom path is set
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

    pub fn icon_color(mut self, icon_color: Color) -> Self {
        self.icon_color = Some(icon_color);
        self
    }

    pub fn toggle(mut self, toggle_position: IconPosition, toggled: bool) -> Self {
        self.toggle = Some((toggle_position, toggled));
        self
    }

    pub fn action(mut self, action: Box<dyn Action>) -> Self {
        self.action = Some(action);
        self
    }

    pub fn handler(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.handler = Rc::new(move |_, window, cx| handler(window, cx));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn documentation_aside(
        mut self,
        side: DocumentationSide,
        edge: DocumentationEdge,
        render: impl Fn(&mut App) -> AnyElement + 'static,
    ) -> Self {
        self.documentation_aside = Some(DocumentationAside {
            side,
            edge,
            render: Rc::new(render),
        });

        self
    }
}

impl FluentBuilder for ContextMenuEntry {}

impl From<ContextMenuEntry> for ContextMenuItem {
    fn from(entry: ContextMenuEntry) -> Self {
        ContextMenuItem::Entry(entry)
    }
}

pub struct ContextMenu {
    builder: Option<Rc<dyn Fn(Self, &mut Window, &mut Context<Self>) -> Self>>,
    items: Vec<ContextMenuItem>,
    focus_handle: FocusHandle,
    action_context: Option<FocusHandle>,
    selected_index: Option<usize>,
    delayed: bool,
    clicked: bool,
    end_slot_action: Option<Box<dyn Action>>,
    key_context: SharedString,
    _on_blur_subscription: Subscription,
    keep_open_on_confirm: bool,
    documentation_aside: Option<(usize, DocumentationAside)>,
    fixed_width: Option<DefiniteLength>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DocumentationSide {
    Left,
    Right,
}

#[derive(Copy, Default, Clone, PartialEq, Eq)]
pub enum DocumentationEdge {
    #[default]
    Top,
    Bottom,
}

#[derive(Clone)]
pub struct DocumentationAside {
    pub side: DocumentationSide,
    pub edge: DocumentationEdge,
    pub render: Rc<dyn Fn(&mut App) -> AnyElement>,
}

impl DocumentationAside {
    pub fn new(
        side: DocumentationSide,
        edge: DocumentationEdge,
        render: Rc<dyn Fn(&mut App) -> AnyElement>,
    ) -> Self {
        Self { side, edge, render }
    }
}

impl Focusable for ContextMenu {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ContextMenu {}

impl FluentBuilder for ContextMenu {}

impl ContextMenu {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        f: impl FnOnce(Self, &mut Window, &mut Context<Self>) -> Self,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let _on_blur_subscription = cx.on_blur(
            &focus_handle,
            window,
            |this: &mut ContextMenu, window, cx| this.cancel(&menu::Cancel, window, cx),
        );
        window.refresh();

        f(
            Self {
                builder: None,
                items: Default::default(),
                focus_handle,
                action_context: None,
                selected_index: None,
                delayed: false,
                clicked: false,
                key_context: "menu".into(),
                _on_blur_subscription,
                keep_open_on_confirm: false,
                documentation_aside: None,
                fixed_width: None,
                end_slot_action: None,
            },
            window,
            cx,
        )
    }

    pub fn build(
        window: &mut Window,
        cx: &mut App,
        f: impl FnOnce(Self, &mut Window, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, f))
    }

    /// Builds a [`ContextMenu`] that will stay open when making changes instead of closing after each confirmation.
    ///
    /// The main difference from [`ContextMenu::build`] is the type of the `builder`, as we need to be able to hold onto
    /// it to call it again.
    pub fn build_persistent(
        window: &mut Window,
        cx: &mut App,
        builder: impl Fn(Self, &mut Window, &mut Context<Self>) -> Self + 'static,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let builder = Rc::new(builder);

            let focus_handle = cx.focus_handle();
            let _on_blur_subscription = cx.on_blur(
                &focus_handle,
                window,
                |this: &mut ContextMenu, window, cx| this.cancel(&menu::Cancel, window, cx),
            );
            window.refresh();

            (builder.clone())(
                Self {
                    builder: Some(builder),
                    items: Default::default(),
                    focus_handle,
                    action_context: None,
                    selected_index: None,
                    delayed: false,
                    clicked: false,
                    key_context: "menu".into(),
                    _on_blur_subscription,
                    keep_open_on_confirm: true,
                    documentation_aside: None,
                    fixed_width: None,
                    end_slot_action: None,
                },
                window,
                cx,
            )
        })
    }

    /// Rebuilds the menu.
    ///
    /// This is used to refresh the menu entries when entries are toggled when the menu is configured with
    /// `keep_open_on_confirm = true`.
    ///
    /// This only works if the [`ContextMenu`] was constructed using [`ContextMenu::build_persistent`]. Otherwise it is
    /// a no-op.
    pub fn rebuild(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(builder) = self.builder.clone() else {
            return;
        };

        // The way we rebuild the menu is a bit of a hack.
        let focus_handle = cx.focus_handle();
        let new_menu = (builder.clone())(
            Self {
                builder: Some(builder),
                items: Default::default(),
                focus_handle: focus_handle.clone(),
                action_context: None,
                selected_index: None,
                delayed: false,
                clicked: false,
                key_context: "menu".into(),
                _on_blur_subscription: cx.on_blur(
                    &focus_handle,
                    window,
                    |this: &mut ContextMenu, window, cx| this.cancel(&menu::Cancel, window, cx),
                ),
                keep_open_on_confirm: false,
                documentation_aside: None,
                fixed_width: None,
                end_slot_action: None,
            },
            window,
            cx,
        );

        self.items = new_menu.items;

        cx.notify();
    }

    pub fn context(mut self, focus: FocusHandle) -> Self {
        self.action_context = Some(focus);
        self
    }

    pub fn header(mut self, title: impl Into<SharedString>) -> Self {
        self.items.push(ContextMenuItem::Header(title.into()));
        self
    }

    pub fn header_with_link(
        mut self,
        title: impl Into<SharedString>,
        link_label: impl Into<SharedString>,
        link_url: impl Into<SharedString>,
    ) -> Self {
        self.items.push(ContextMenuItem::HeaderWithLink(
            title.into(),
            link_label.into(),
            link_url.into(),
        ));
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

    pub fn push_item(&mut self, item: impl Into<ContextMenuItem>) {
        self.items.push(item.into());
    }

    pub fn entry(
        mut self,
        label: impl Into<SharedString>,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            handler: Rc::new(move |_, window, cx| handler(window, cx)),
            icon: None,
            custom_icon_path: None,
            icon_position: IconPosition::End,
            icon_size: IconSize::Small,
            icon_color: None,
            action,
            disabled: false,
            documentation_aside: None,
            end_slot_icon: None,
            end_slot_title: None,
            end_slot_handler: None,
            show_end_slot_on_hover: false,
        }));
        self
    }

    pub fn entry_with_end_slot(
        mut self,
        label: impl Into<SharedString>,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
        end_slot_icon: IconName,
        end_slot_title: SharedString,
        end_slot_handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            handler: Rc::new(move |_, window, cx| handler(window, cx)),
            icon: None,
            custom_icon_path: None,
            icon_position: IconPosition::End,
            icon_size: IconSize::Small,
            icon_color: None,
            action,
            disabled: false,
            documentation_aside: None,
            end_slot_icon: Some(end_slot_icon),
            end_slot_title: Some(end_slot_title),
            end_slot_handler: Some(Rc::new(move |_, window, cx| end_slot_handler(window, cx))),
            show_end_slot_on_hover: false,
        }));
        self
    }

    pub fn entry_with_end_slot_on_hover(
        mut self,
        label: impl Into<SharedString>,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
        end_slot_icon: IconName,
        end_slot_title: SharedString,
        end_slot_handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            handler: Rc::new(move |_, window, cx| handler(window, cx)),
            icon: None,
            custom_icon_path: None,
            icon_position: IconPosition::End,
            icon_size: IconSize::Small,
            icon_color: None,
            action,
            disabled: false,
            documentation_aside: None,
            end_slot_icon: Some(end_slot_icon),
            end_slot_title: Some(end_slot_title),
            end_slot_handler: Some(Rc::new(move |_, window, cx| end_slot_handler(window, cx))),
            show_end_slot_on_hover: true,
        }));
        self
    }

    pub fn toggleable_entry(
        mut self,
        label: impl Into<SharedString>,
        toggled: bool,
        position: IconPosition,
        action: Option<Box<dyn Action>>,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: Some((position, toggled)),
            label: label.into(),
            handler: Rc::new(move |_, window, cx| handler(window, cx)),
            icon: None,
            custom_icon_path: None,
            icon_position: position,
            icon_size: IconSize::Small,
            icon_color: None,
            action,
            disabled: false,
            documentation_aside: None,
            end_slot_icon: None,
            end_slot_title: None,
            end_slot_handler: None,
            show_end_slot_on_hover: false,
        }));
        self
    }

    pub fn custom_row(
        mut self,
        entry_render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::CustomEntry {
            entry_render: Box::new(entry_render),
            handler: Rc::new(|_, _, _| {}),
            selectable: false,
            documentation_aside: None,
        });
        self
    }

    pub fn custom_entry(
        mut self,
        entry_render: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
        handler: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(ContextMenuItem::CustomEntry {
            entry_render: Box::new(entry_render),
            handler: Rc::new(move |_, window, cx| handler(window, cx)),
            selectable: true,
            documentation_aside: None,
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
            handler: Rc::new(move |context, window, cx| {
                if let Some(context) = &context {
                    window.focus(context);
                }
                window.dispatch_action(action.boxed_clone(), cx);
            }),
            icon: None,
            custom_icon_path: None,
            icon_position: IconPosition::End,
            icon_size: IconSize::Small,
            icon_color: None,
            disabled: false,
            documentation_aside: None,
            end_slot_icon: None,
            end_slot_title: None,
            end_slot_handler: None,
            show_end_slot_on_hover: false,
        }));
        self
    }

    pub fn action_disabled_when(
        mut self,
        disabled: bool,
        label: impl Into<SharedString>,
        action: Box<dyn Action>,
    ) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            action: Some(action.boxed_clone()),
            handler: Rc::new(move |context, window, cx| {
                if let Some(context) = &context {
                    window.focus(context);
                }
                window.dispatch_action(action.boxed_clone(), cx);
            }),
            icon: None,
            custom_icon_path: None,
            icon_size: IconSize::Small,
            icon_position: IconPosition::End,
            icon_color: None,
            disabled,
            documentation_aside: None,
            end_slot_icon: None,
            end_slot_title: None,
            end_slot_handler: None,
            show_end_slot_on_hover: false,
        }));
        self
    }

    pub fn link(mut self, label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        self.items.push(ContextMenuItem::Entry(ContextMenuEntry {
            toggle: None,
            label: label.into(),
            action: Some(action.boxed_clone()),
            handler: Rc::new(move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx)),
            icon: Some(IconName::ArrowUpRight),
            custom_icon_path: None,
            icon_size: IconSize::XSmall,
            icon_position: IconPosition::End,
            icon_color: None,
            disabled: false,
            documentation_aside: None,
            end_slot_icon: None,
            end_slot_title: None,
            end_slot_handler: None,
            show_end_slot_on_hover: false,
        }));
        self
    }

    pub fn keep_open_on_confirm(mut self, keep_open: bool) -> Self {
        self.keep_open_on_confirm = keep_open;
        self
    }

    pub fn trigger_end_slot_handler(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.selected_index.and_then(|ix| self.items.get(ix)) else {
            return;
        };
        let ContextMenuItem::Entry(entry) = entry else {
            return;
        };
        let Some(handler) = entry.end_slot_handler.as_ref() else {
            return;
        };
        handler(None, window, cx);
    }

    pub fn fixed_width(mut self, width: DefiniteLength) -> Self {
        self.fixed_width = Some(width);
        self
    }

    pub fn end_slot_action(mut self, action: Box<dyn Action>) -> Self {
        self.end_slot_action = Some(action);
        self
    }

    pub fn key_context(mut self, context: impl Into<SharedString>) -> Self {
        self.key_context = context.into();
        self
    }

    pub fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
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
            (handler)(context, window, cx)
        }

        if self.keep_open_on_confirm {
            self.rebuild(window, cx);
        } else {
            cx.emit(DismissEvent);
        }
    }

    pub fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
        cx.emit(DismissEvent);
    }

    pub fn end_slot(&mut self, _: &dyn Action, window: &mut Window, cx: &mut Context<Self>) {
        let Some(item) = self.selected_index.and_then(|ix| self.items.get(ix)) else {
            return;
        };
        let ContextMenuItem::Entry(entry) = item else {
            return;
        };
        let Some(handler) = entry.end_slot_handler.as_ref() else {
            return;
        };
        handler(None, window, cx);
        self.rebuild(window, cx);
        cx.notify();
    }

    pub fn clear_selected(&mut self) {
        self.selected_index = None;
    }

    pub fn select_first(&mut self, _: &SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.items.iter().position(|item| item.is_selectable()) {
            self.select_index(ix, window, cx);
        }
        cx.notify();
    }

    pub fn select_last(&mut self, window: &mut Window, cx: &mut Context<Self>) -> Option<usize> {
        for (ix, item) in self.items.iter().enumerate().rev() {
            if item.is_selectable() {
                return self.select_index(ix, window, cx);
            }
        }
        None
    }

    fn handle_select_last(&mut self, _: &SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        if self.select_last(window, cx).is_some() {
            cx.notify();
        }
    }

    pub fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(ix) = self.selected_index {
            let next_index = ix + 1;
            if self.items.len() <= next_index {
                self.select_first(&SelectFirst, window, cx);
                return;
            } else {
                for (ix, item) in self.items.iter().enumerate().skip(next_index) {
                    if item.is_selectable() {
                        self.select_index(ix, window, cx);
                        cx.notify();
                        return;
                    }
                }
            }
        }
        self.select_first(&SelectFirst, window, cx);
    }

    pub fn select_previous(
        &mut self,
        _: &SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(ix) = self.selected_index {
            for (ix, item) in self.items.iter().enumerate().take(ix).rev() {
                if item.is_selectable() {
                    self.select_index(ix, window, cx);
                    cx.notify();
                    return;
                }
            }
        }
        self.handle_select_last(&SelectLast, window, cx);
    }

    fn select_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        self.documentation_aside = None;
        let item = self.items.get(ix)?;
        if item.is_selectable() {
            self.selected_index = Some(ix);
            match item {
                ContextMenuItem::Entry(entry) => {
                    if let Some(callback) = &entry.documentation_aside {
                        self.documentation_aside = Some((ix, callback.clone()));
                    }
                }
                ContextMenuItem::CustomEntry {
                    documentation_aside: Some(callback),
                    ..
                } => {
                    self.documentation_aside = Some((ix, callback.clone()));
                }
                _ => (),
            }
        }
        Some(ix)
    }

    pub fn on_action_dispatch(
        &mut self,
        dispatched: &dyn Action,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
            self.select_index(ix, window, cx);
            self.delayed = true;
            cx.notify();
            let action = dispatched.boxed_clone();
            cx.spawn_in(window, async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(50))
                    .await;
                cx.update(|window, cx| {
                    this.update(cx, |this, cx| {
                        this.cancel(&menu::Cancel, window, cx);
                        window.dispatch_action(action, cx);
                    })
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

    fn render_menu_item(
        &self,
        ix: usize,
        item: &ContextMenuItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        match item {
            ContextMenuItem::Separator => ListSeparator.into_any_element(),
            ContextMenuItem::Header(header) => ListSubHeader::new(header.clone())
                .inset(true)
                .into_any_element(),
            ContextMenuItem::HeaderWithLink(header, label, url) => {
                let url = url.clone();
                let link_id = ElementId::Name(format!("link-{}", url).into());
                ListSubHeader::new(header.clone())
                    .inset(true)
                    .end_slot(
                        Button::new(link_id, label.clone())
                            .color(Color::Muted)
                            .label_size(LabelSize::Small)
                            .size(ButtonSize::None)
                            .style(ButtonStyle::Transparent)
                            .on_click(move |_, _, cx| {
                                let url = url.clone();
                                cx.open_url(&url);
                            })
                            .into_any_element(),
                    )
                    .into_any_element()
            }
            ContextMenuItem::Label(label) => ListItem::new(ix)
                .inset(true)
                .disabled(true)
                .child(Label::new(label.clone()))
                .into_any_element(),
            ContextMenuItem::Entry(entry) => {
                self.render_menu_entry(ix, entry, cx).into_any_element()
            }
            ContextMenuItem::CustomEntry {
                entry_render,
                handler,
                selectable,
                ..
            } => {
                let handler = handler.clone();
                let menu = cx.entity().downgrade();
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
                            let keep_open_on_confirm = self.keep_open_on_confirm;
                            move |_, window, cx| {
                                handler(context.as_ref(), window, cx);
                                menu.update(cx, |menu, cx| {
                                    menu.clicked = true;

                                    if keep_open_on_confirm {
                                        menu.rebuild(window, cx);
                                    } else {
                                        cx.emit(DismissEvent);
                                    }
                                })
                                .ok();
                            }
                        })
                    })
                    .child(entry_render(window, cx))
                    .into_any_element()
            }
        }
    }

    fn render_menu_entry(
        &self,
        ix: usize,
        entry: &ContextMenuEntry,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let ContextMenuEntry {
            toggle,
            label,
            handler,
            icon,
            custom_icon_path,
            icon_position,
            icon_size,
            icon_color,
            action,
            disabled,
            documentation_aside,
            end_slot_icon,
            end_slot_title,
            end_slot_handler,
            show_end_slot_on_hover,
        } = entry;
        let this = cx.weak_entity();

        let handler = handler.clone();
        let menu = cx.entity().downgrade();

        let icon_color = if *disabled {
            Color::Muted
        } else if toggle.is_some() {
            icon_color.unwrap_or(Color::Accent)
        } else {
            icon_color.unwrap_or(Color::Default)
        };

        let label_color = if *disabled {
            Color::Disabled
        } else {
            Color::Default
        };

        let label_element = if let Some(custom_path) = custom_icon_path {
            h_flex()
                .gap_1p5()
                .when(
                    *icon_position == IconPosition::Start && toggle.is_none(),
                    |flex| {
                        flex.child(
                            Icon::from_path(custom_path.clone())
                                .size(*icon_size)
                                .color(icon_color),
                        )
                    },
                )
                .child(Label::new(label.clone()).color(label_color).truncate())
                .when(*icon_position == IconPosition::End, |flex| {
                    flex.child(
                        Icon::from_path(custom_path.clone())
                            .size(*icon_size)
                            .color(icon_color),
                    )
                })
                .into_any_element()
        } else if let Some(icon_name) = icon {
            h_flex()
                .gap_1p5()
                .when(
                    *icon_position == IconPosition::Start && toggle.is_none(),
                    |flex| flex.child(Icon::new(*icon_name).size(*icon_size).color(icon_color)),
                )
                .child(Label::new(label.clone()).color(label_color).truncate())
                .when(*icon_position == IconPosition::End, |flex| {
                    flex.child(Icon::new(*icon_name).size(*icon_size).color(icon_color))
                })
                .into_any_element()
        } else {
            Label::new(label.clone())
                .color(label_color)
                .truncate()
                .into_any_element()
        };

        div()
            .id(("context-menu-child", ix))
            .when_some(documentation_aside.clone(), |this, documentation_aside| {
                this.occlude()
                    .on_hover(cx.listener(move |menu, hovered, _, cx| {
                        if *hovered {
                            menu.documentation_aside = Some((ix, documentation_aside.clone()));
                        } else if matches!(menu.documentation_aside, Some((id, _)) if id == ix) {
                            menu.documentation_aside = None;
                        }
                        cx.notify();
                    }))
            })
            .child(
                ListItem::new(ix)
                    .group_name("label_container")
                    .inset(true)
                    .disabled(*disabled)
                    .toggle_state(Some(ix) == self.selected_index)
                    .when_some(*toggle, |list_item, (position, toggled)| {
                        let contents = div()
                            .flex_none()
                            .child(
                                Icon::new(icon.unwrap_or(IconName::Check))
                                    .color(icon_color)
                                    .size(*icon_size),
                            )
                            .when(!toggled, |contents| contents.invisible());

                        match position {
                            IconPosition::Start => list_item.start_slot(contents),
                            IconPosition::End => list_item.end_slot(contents),
                        }
                    })
                    .child(
                        h_flex()
                            .w_full()
                            .justify_between()
                            .child(label_element)
                            .debug_selector(|| format!("MENU_ITEM-{}", label))
                            .children(action.as_ref().map(|action| {
                                let binding = self
                                    .action_context
                                    .as_ref()
                                    .map(|focus| KeyBinding::for_action_in(&**action, focus, cx))
                                    .unwrap_or_else(|| KeyBinding::for_action(&**action, cx));

                                div()
                                    .ml_4()
                                    .child(binding.disabled(*disabled))
                                    .when(*disabled && documentation_aside.is_some(), |parent| {
                                        parent.invisible()
                                    })
                            }))
                            .when(*disabled && documentation_aside.is_some(), |parent| {
                                parent.child(
                                    Icon::new(IconName::Info)
                                        .size(IconSize::XSmall)
                                        .color(Color::Muted),
                                )
                            }),
                    )
                    .when_some(
                        end_slot_icon
                            .as_ref()
                            .zip(self.end_slot_action.as_ref())
                            .zip(end_slot_title.as_ref())
                            .zip(end_slot_handler.as_ref()),
                        |el, (((icon, action), title), handler)| {
                            el.end_slot({
                                let icon_button = IconButton::new("end-slot-icon", *icon)
                                    .shape(IconButtonShape::Square)
                                    .tooltip({
                                        let action_context = self.action_context.clone();
                                        let title = title.clone();
                                        let action = action.boxed_clone();
                                        move |_window, cx| {
                                            action_context
                                                .as_ref()
                                                .map(|focus| {
                                                    Tooltip::for_action_in(
                                                        title.clone(),
                                                        &*action,
                                                        focus,
                                                        cx,
                                                    )
                                                })
                                                .unwrap_or_else(|| {
                                                    Tooltip::for_action(title.clone(), &*action, cx)
                                                })
                                        }
                                    })
                                    .on_click({
                                        let handler = handler.clone();
                                        move |_, window, cx| {
                                            handler(None, window, cx);
                                            this.update(cx, |this, cx| {
                                                this.rebuild(window, cx);
                                                cx.notify();
                                            })
                                            .ok();
                                        }
                                    });

                                if *show_end_slot_on_hover {
                                    div()
                                        .visible_on_hover("label_container")
                                        .child(icon_button)
                                        .into_any_element()
                                } else {
                                    icon_button.into_any_element()
                                }
                            })
                        },
                    )
                    .on_click({
                        let context = self.action_context.clone();
                        let keep_open_on_confirm = self.keep_open_on_confirm;
                        move |_, window, cx| {
                            handler(context.as_ref(), window, cx);
                            menu.update(cx, |menu, cx| {
                                menu.clicked = true;
                                if keep_open_on_confirm {
                                    menu.rebuild(window, cx);
                                } else {
                                    cx.emit(DismissEvent);
                                }
                            })
                            .ok();
                        }
                    }),
            )
            .into_any_element()
    }
}

impl ContextMenuItem {
    fn is_selectable(&self) -> bool {
        match self {
            ContextMenuItem::Header(_)
            | ContextMenuItem::HeaderWithLink(_, _, _)
            | ContextMenuItem::Separator
            | ContextMenuItem::Label { .. } => false,
            ContextMenuItem::Entry(ContextMenuEntry { disabled, .. }) => !disabled,
            ContextMenuItem::CustomEntry { selectable, .. } => *selectable,
        }
    }
}

impl Render for ContextMenu {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let window_size = window.viewport_size();
        let rem_size = window.rem_size();
        let is_wide_window = window_size.width / rem_size > rems_from_px(800.).0;

        let aside = self.documentation_aside.clone();
        let render_aside = |aside: DocumentationAside, cx: &mut Context<Self>| {
            WithRemSize::new(ui_font_size)
                .occlude()
                .elevation_2(cx)
                .w_full()
                .p_2()
                .overflow_hidden()
                .when(is_wide_window, |this| this.max_w_96())
                .when(!is_wide_window, |this| this.max_w_48())
                .child((aside.render)(cx))
        };

        let render_menu =
            |cx: &mut Context<Self>, window: &mut Window| {
                WithRemSize::new(ui_font_size)
                    .occlude()
                    .elevation_2(cx)
                    .flex()
                    .flex_row()
                    .flex_shrink_0()
                    .child(
                        v_flex()
                            .id("context-menu")
                            .max_h(vh(0.75, window))
                            .flex_shrink_0()
                            .when_some(self.fixed_width, |this, width| {
                                this.w(width).overflow_x_hidden()
                            })
                            .when(self.fixed_width.is_none(), |this| {
                                this.min_w(px(200.)).flex_1()
                            })
                            .overflow_y_scroll()
                            .track_focus(&self.focus_handle(cx))
                            .on_mouse_down_out(cx.listener(|this, _, window, cx| {
                                this.cancel(&menu::Cancel, window, cx)
                            }))
                            .key_context(self.key_context.as_ref())
                            .on_action(cx.listener(ContextMenu::select_first))
                            .on_action(cx.listener(ContextMenu::handle_select_last))
                            .on_action(cx.listener(ContextMenu::select_next))
                            .on_action(cx.listener(ContextMenu::select_previous))
                            .on_action(cx.listener(ContextMenu::confirm))
                            .on_action(cx.listener(ContextMenu::cancel))
                            .when_some(self.end_slot_action.as_ref(), |el, action| {
                                el.on_boxed_action(&**action, cx.listener(ContextMenu::end_slot))
                            })
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
                            .child(
                                List::new().children(
                                    self.items.iter().enumerate().map(|(ix, item)| {
                                        self.render_menu_item(ix, item, window, cx)
                                    }),
                                ),
                            ),
                    )
            };

        if is_wide_window {
            div()
                .relative()
                .child(render_menu(cx, window))
                .children(aside.map(|(_item_index, aside)| {
                    h_flex()
                        .absolute()
                        .when(aside.side == DocumentationSide::Left, |this| {
                            this.right_full().mr_1()
                        })
                        .when(aside.side == DocumentationSide::Right, |this| {
                            this.left_full().ml_1()
                        })
                        .when(aside.edge == DocumentationEdge::Top, |this| this.top_0())
                        .when(aside.edge == DocumentationEdge::Bottom, |this| {
                            this.bottom_0()
                        })
                        .child(render_aside(aside, cx))
                }))
        } else {
            v_flex()
                .w_full()
                .gap_1()
                .justify_end()
                .children(aside.map(|(_, aside)| render_aside(aside, cx)))
                .child(render_menu(cx, window))
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;

    use super::*;

    #[gpui::test]
    fn can_navigate_back_over_headers(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        let context_menu = cx.update(|window, cx| {
            ContextMenu::build(window, cx, |menu, _, _| {
                menu.header("First header")
                    .separator()
                    .entry("First entry", None, |_, _| {})
                    .separator()
                    .separator()
                    .entry("Last entry", None, |_, _| {})
                    .header("Last header")
            })
        });

        context_menu.update_in(cx, |context_menu, window, cx| {
            assert_eq!(
                None, context_menu.selected_index,
                "No selection is in the menu initially"
            );

            context_menu.select_first(&SelectFirst, window, cx);
            assert_eq!(
                Some(2),
                context_menu.selected_index,
                "Should select first selectable entry, skipping the header and the separator"
            );

            context_menu.select_next(&SelectNext, window, cx);
            assert_eq!(
                Some(5),
                context_menu.selected_index,
                "Should select next selectable entry, skipping 2 separators along the way"
            );

            context_menu.select_next(&SelectNext, window, cx);
            assert_eq!(
                Some(2),
                context_menu.selected_index,
                "Should wrap around to first selectable entry"
            );
        });

        context_menu.update_in(cx, |context_menu, window, cx| {
            assert_eq!(
                Some(2),
                context_menu.selected_index,
                "Should start from the first selectable entry"
            );

            context_menu.select_previous(&SelectPrevious, window, cx);
            assert_eq!(
                Some(5),
                context_menu.selected_index,
                "Should wrap around to previous selectable entry (last)"
            );

            context_menu.select_previous(&SelectPrevious, window, cx);
            assert_eq!(
                Some(2),
                context_menu.selected_index,
                "Should go back to previous selectable entry (first)"
            );
        });

        context_menu.update_in(cx, |context_menu, window, cx| {
            context_menu.select_first(&SelectFirst, window, cx);
            assert_eq!(
                Some(2),
                context_menu.selected_index,
                "Should start from the first selectable entry"
            );

            context_menu.select_previous(&SelectPrevious, window, cx);
            assert_eq!(
                Some(5),
                context_menu.selected_index,
                "Should wrap around to last selectable entry"
            );
            context_menu.select_next(&SelectNext, window, cx);
            assert_eq!(
                Some(2),
                context_menu.selected_index,
                "Should wrap around to first selectable entry"
            );
        });
    }
}
