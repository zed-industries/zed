use std::rc::Rc;

use gpui::{Action, FocusHandle, Focusable};
use ui::{ContextMenu, Divider, FluentBuilder, KeyBinding, PopoverMenu, Tooltip, prelude::*};

use crate::Picker;
use crate::PickerDelegate;
use crate::SetPreviewBelow;
use crate::SetPreviewRight;
use crate::ToggleActionsMenu;
use crate::TogglePreview;
use crate::preview;

/// Line in the Actions menu on the default footer.
pub enum PickerAction {
    Header(SharedString),
    Separator,
    Entry {
        label: SharedString,
        action: Box<dyn Action>,
        toggled: Option<bool>,
    },
}

impl PickerAction {
    pub fn button(label: impl Into<SharedString>, action: Box<dyn Action>) -> Self {
        Self::Entry {
            label: label.into(),
            action,
            toggled: None,
        }
    }

    /// A non-clickable section title.
    pub fn header(label: impl Into<SharedString>) -> Self {
        Self::Header(label.into())
    }

    /// A divider between groups of entries.
    pub fn separator() -> Self {
        Self::Separator
    }

    /// Make it possible to turn the previous item on and off
    pub fn toggled(mut self, toggled: bool) -> Self {
        if let Self::Entry { toggled: t, .. } = &mut self {
            *t = Some(toggled);
        }
        self
    }

    pub(crate) fn add_to_menu(&self, menu: ContextMenu, focus_handle: &FocusHandle) -> ContextMenu {
        match self {
            Self::Header(label) => menu.header(label),
            Self::Separator => menu.separator(),
            Self::Entry {
                label,
                action,
                toggled: Some(toggled),
            } => {
                let dispatched = action.boxed_clone();
                let handler_focus = focus_handle.clone();
                menu.toggleable_entry(
                    label,
                    *toggled,
                    ui::IconPosition::End,
                    Some(action.boxed_clone()),
                    move |window, cx| {
                        window.focus(&handler_focus, cx);
                        window.dispatch_action(dispatched.boxed_clone(), cx);
                    },
                )
            }
            Self::Entry {
                label,
                action,
                toggled: None,
            } => menu.action(label, action.boxed_clone()),
        }
    }
}

impl<D: PickerDelegate> Picker<D> {
    pub(crate) fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if let Some(footer) = self.delegate.render_footer(window, cx) {
            return Some(footer);
        }
        self.render_default_footer(window, cx)
    }

    fn render_default_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let actions = self.delegate.actions_menu(window, cx);

        if self.preview.is_none() && actions.is_empty() {
            return None;
        }

        let focus_handle = self.focus_handle(cx);

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .justify_between()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .when(self.preview.is_some(), |this| {
                    this.child(self.render_preview_controls(window, cx))
                })
                .when(!actions.is_empty(), |this| {
                    this.child(self.render_actions_button(actions.into(), focus_handle, window, cx))
                })
                .into_any_element(),
        )
    }

    fn render_preview_controls(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx);
        let right_focus_handle = focus_handle.clone();
        let below_focus_handle = focus_handle.clone();
        let current = self.preview_layout().unwrap_or(preview::Layout::Hidden);
        let preview_visible = current != preview::Layout::Hidden;

        let diff_split = if self.is_auto_vertical(window) {
            IconName::DiffSplitAuto
        } else {
            IconName::DiffSplit
        };

        h_flex()
            .child(
                Button::new("picker-preview-toggle", "Preview")
                    .when(preview_visible, |this| this.color(Color::Accent))
                    .key_binding(
                        KeyBinding::for_action_in(&TogglePreview, &focus_handle, cx)
                            .size(rems_from_px(12.)),
                    )
                    .on_click(
                        cx.listener(|this, _, window, cx| this.toggle_preview_visible(window, cx)),
                    ),
            )
            .when(preview_visible, |this| {
                this.child(Divider::vertical().mx_1())
                    .child(
                        IconButton::new("picker-preview-below", IconName::DiffUnified)
                            .icon_size(IconSize::Small)
                            .toggle_state(current == preview::Layout::Below)
                            .tooltip(move |_window, cx| {
                                Tooltip::for_action_in(
                                    "Preview Below",
                                    &SetPreviewBelow,
                                    &below_focus_handle,
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_preview_layout(preview::Layout::Below, window, cx)
                            })),
                    )
                    .child(
                        IconButton::new("picker-preview-right", diff_split)
                            .icon_size(IconSize::Small)
                            .toggle_state(current == preview::Layout::Right)
                            .tooltip(move |_window, cx| {
                                Tooltip::for_action_in(
                                    "Preview to the Right",
                                    &SetPreviewRight,
                                    &right_focus_handle,
                                    cx,
                                )
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_preview_layout(preview::Layout::Right, window, cx)
                            })),
                    )
            })
    }

    fn render_actions_button(
        &self,
        actions: Rc<[crate::footer::PickerAction]>,
        focus_handle: gpui::FocusHandle,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        PopoverMenu::new("picker-actions-menu")
            .with_handle(self.actions_menu_handle.clone())
            .trigger(
                Button::new("picker-actions-trigger", "Actions…")
                    .key_binding(
                        KeyBinding::for_action_in(&ToggleActionsMenu, &focus_handle, cx)
                            .size(rems_from_px(12.)),
                    )
                    .selected_style(ui::ButtonStyle::Tinted(ui::TintColor::Accent)),
            )
            .menu(move |window, cx| {
                let actions = Rc::clone(&actions);
                let focus_handle = focus_handle.clone();
                Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                    menu = menu.context(focus_handle.clone());
                    for item in actions.iter() {
                        menu = item.add_to_menu(menu, &focus_handle);
                    }
                    menu
                }))
            })
            .attach(gpui::Anchor::TopRight)
            .anchor(gpui::Anchor::BottomRight)
            .offset(gpui::Point {
                x: px(0.0),
                y: px(-2.0),
            })
    }
}
