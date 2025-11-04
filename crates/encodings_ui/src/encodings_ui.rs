//! A crate for handling file encodings in the text editor.

use editor::Editor;
use gpui::{Entity, Subscription, WeakEntity};
use language::{Buffer, BufferEvent};
use ui::{
    App, Button, ButtonCommon, Context, IntoElement, LabelSize, Render, Tooltip, Window, div,
};
use ui::{Clickable, ParentElement};
use workspace::notifications::NotifyTaskExt;
use workspace::{ItemHandle, StatusItemView, Workspace};
use zed_actions::encodings_ui::OpenWithEncoding;
// use zed_actions::encodings_ui::Toggle;

/// A status bar item that shows the current file encoding and allows changing it.
pub struct EncodingIndicator {
    pub buffer: Option<WeakEntity<Buffer>>,
    pub workspace: WeakEntity<Workspace>,
    observe_buffer: Option<Subscription>,
}

pub mod selectors;

impl Render for EncodingIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let Some(buffer) = self.buffer() else {
            return gpui::Empty.into_any_element();
        };

        div()
            .child(
                Button::new("encoding", buffer.read(cx).encoding().name())
                    .label_size(LabelSize::Small)
                    .tooltip(Tooltip::text("Select Encoding"))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        let Some(buffer) = this.buffer() else {
                            return;
                        };
                        this.workspace
                            .update(cx, move |workspace, cx| {
                                if buffer.read(cx).file().is_some() {
                                    selectors::save_or_reopen(buffer, workspace, window, cx)
                                } else {
                                    // todo!()
                                }
                            })
                            .ok();
                    })),
            )
            .into_any_element()
    }
}

impl EncodingIndicator {
    pub fn new(workspace: WeakEntity<Workspace>) -> EncodingIndicator {
        EncodingIndicator {
            workspace,
            buffer: None,
            observe_buffer: None,
        }
    }

    fn buffer(&self) -> Option<Entity<Buffer>> {
        self.buffer.as_ref().and_then(|b| b.upgrade())
    }

    /// Update the encoding when the `encoding` field of the `Buffer` struct changes.
    pub fn on_buffer_event(
        &mut self,
        _: Entity<Buffer>,
        e: &BufferEvent,
        cx: &mut Context<EncodingIndicator>,
    ) {
        if matches!(e, BufferEvent::EncodingChanged) {
            cx.notify();
        }
    }
}

impl StatusItemView for EncodingIndicator {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx))
            && let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton()
        {
            self.buffer = Some(buffer.downgrade());
            self.observe_buffer = Some(cx.subscribe(&buffer, Self::on_buffer_event));
        } else {
            self.buffer = None;
            self.observe_buffer = None;
        }
        cx.notify();
    }
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, action: &OpenWithEncoding, window, cx| {
            selectors::open_with_encoding(action.0.clone(), workspace, window, cx)
                .detach_and_notify_err(window, cx);
        });
    })
    .detach();
}
