use std::{path::PathBuf, sync::Arc};

use gpui::{EventEmitter, FocusHandle, Focusable};
use ui::{
    App, Button, ButtonCommon, ButtonStyle, Clickable, Context, FluentBuilder, InteractiveElement,
    KeyBinding, ParentElement, Render, SharedString, Styled as _, Window, h_flex, v_flex,
};
use zed_actions::workspace::OpenWithSystem;

use crate::Item;

/// A view to display when a certain buffer fails to open.
pub struct InvalidBufferView {
    /// Which path was attempted to open.
    pub abs_path: Arc<PathBuf>,
    /// An error message, happened when opening the buffer.
    pub error: SharedString,
    is_local: bool,
    focus_handle: FocusHandle,
}

impl InvalidBufferView {
    pub fn new(
        abs_path: PathBuf,
        is_local: bool,
        e: &anyhow::Error,
        _: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self {
            is_local,
            abs_path: Arc::new(abs_path),
            error: format!("{e}").into(),
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Item for InvalidBufferView {
    type Event = ();

    fn tab_content_text(&self, mut detail: usize, _: &App) -> SharedString {
        // Ensure we always render at least the filename.
        detail += 1;

        let path = self.abs_path.as_path();

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

impl EventEmitter<()> for InvalidBufferView {}

impl Focusable for InvalidBufferView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for InvalidBufferView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let abs_path = self.abs_path.clone();
        v_flex()
            .size_full()
            .track_focus(&self.focus_handle(cx))
            .flex_none()
            .justify_center()
            .overflow_hidden()
            .key_context("InvalidBuffer")
            .child(
                h_flex().size_full().justify_center().child(
                    v_flex()
                        .justify_center()
                        .gap_2()
                        .child(h_flex().justify_center().child("Unsupported file type"))
                        .when(self.is_local, |contents| {
                            contents.child(
                                h_flex().justify_center().child(
                                    Button::new("open-with-system", "Open in Default App")
                                        .on_click(move |_, _, cx| {
                                            cx.open_with_system(&abs_path);
                                        })
                                        .style(ButtonStyle::Outlined)
                                        .key_binding(KeyBinding::for_action(
                                            &OpenWithSystem,
                                            window,
                                            cx,
                                        )),
                                ),
                            )
                        }),
                ),
            )
    }
}
