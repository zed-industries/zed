use std::{path::Path, sync::Arc};

use gpui::{AppContext as _, EventEmitter, FocusHandle, Focusable, PromptLevel, Task};
use ui::{
    App, Button, ButtonCommon, ButtonStyle, Clickable, Context, FluentBuilder, InteractiveElement,
    KeyBinding, Label, LabelCommon, LabelSize, ParentElement, Render, SharedString, Styled as _,
    Window, h_flex, v_flex,
};
use zed_actions::workspace::OpenWithSystem;

use crate::{Item, item::ItemEvent, persistence::model::SerializedItem};

/// A view to display when a certain buffer/image/other item fails to open.
#[derive(Debug)]
pub struct InvalidItemView {
    /// Which path was attempted to open.
    pub abs_path: Arc<Path>,
    /// An error message, happened when opening the item.
    pub error: SharedString,
    is_local: bool,
    focus_handle: FocusHandle,
}

impl InvalidItemView {
    pub fn new(
        abs_path: &Path,
        is_local: bool,
        e: &anyhow::Error,
        _: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self {
            is_local,
            abs_path: Arc::from(abs_path),
            error: format!("{}", e.root_cause()).into(),
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Item for InvalidItemView {
    type Event = ();

    fn tab_content_text(&self, mut detail: usize, _: &App) -> SharedString {
        // Ensure we always render at least the filename.
        detail += 1;

        let path = self.abs_path.as_ref();

        let mut prefix = path;
        while detail > 0 {
            if let Some(parent) = prefix.parent() {
                prefix = parent;
                detail -= 1;
            } else {
                break;
            }
        }

        let path = if detail > 0 {
            path
        } else {
            path.strip_prefix(prefix).unwrap_or(path)
        };

        SharedString::new(path.to_string_lossy())
    }
}

impl EventEmitter<()> for InvalidItemView {}

impl Focusable for InvalidItemView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InvalidItemView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let abs_path = self.abs_path.clone();
        v_flex()
            .size_full()
            .track_focus(&self.focus_handle(cx))
            .flex_none()
            .justify_center()
            .overflow_hidden()
            .key_context("InvalidItem")
            .child(
                h_flex().size_full().justify_center().child(
                    v_flex()
                        .justify_center()
                        .gap_2()
                        .child(h_flex().justify_center().child("Could not open file"))
                        .child(
                            h_flex()
                                .justify_center()
                                .child(Label::new(self.error.clone()).size(LabelSize::Small)),
                        )
                        .when(self.is_local, |contents| {
                            contents.child(
                                h_flex().justify_center().child(
                                    Button::new("open-with-system", "Open in Default App")
                                        .on_click(move |_, _, cx| {
                                            cx.open_with_system(&abs_path);
                                        })
                                        .style(ButtonStyle::Outlined)
                                        .key_binding(KeyBinding::for_action(&OpenWithSystem, cx)),
                                ),
                            )
                        }),
                ),
            )
    }
}

#[derive(Debug)]
pub struct RestoringItem {
    pub serialized_item: SerializedItem,
    pub(crate) error: Option<SharedString>,
    focus_handle: FocusHandle,
}

impl RestoringItem {
    pub fn new(serialized_item: SerializedItem, cx: &mut App) -> Self {
        Self {
            serialized_item,
            error: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn set_error(&mut self, error: SharedString, cx: &mut Context<Self>) {
        self.error = Some(error);
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    pub(crate) fn confirm_close(window: &mut Window, cx: &mut App) -> Task<bool> {
        let answer = window.prompt(
            PromptLevel::Warning,
            "Some items have not finished restoring. Close without saving their changes?",
            None,
            &["Close Without Saving", "Cancel"],
            cx,
        );
        cx.background_spawn(async move { answer.await.ok() == Some(0) })
    }
}

impl Item for RestoringItem {
    type Event = ItemEvent;

    fn tab_content_text(&self, _: usize, _: &App) -> SharedString {
        if self.error.is_some() {
            SharedString::from(format!("{} (restore failed)", self.serialized_item.kind))
        } else {
            SharedString::from(format!("Restoring {}…", self.serialized_item.kind))
        }
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event);
    }
}

impl EventEmitter<ItemEvent> for RestoringItem {}

impl Focusable for RestoringItem {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for RestoringItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex()
            .size_full()
            .track_focus(&self.focus_handle(cx))
            .justify_center()
            .items_center()
            .gap_2()
            .child(Label::new(if self.error.is_some() {
                "Could not restore item"
            } else {
                "Restoring item…"
            }))
            .when_some(self.error.clone(), |contents, error| {
                contents.child(Label::new(error).size(LabelSize::Small))
            })
    }
}
