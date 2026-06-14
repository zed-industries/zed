//! The blocking `openDiff` tool.
//!
//! Shows Claude's proposed change as a side-by-side diff tab in Zed plus a
//! Keep/Reject notification, then blocks until the user decides. On Keep we
//! return `FILE_SAVED` together with the final buffer contents and let the
//! Claude CLI perform the actual write (matching the official IDE protocol);
//! on Reject we return `DIFF_REJECTED` and discard the change. The IDE
//! deliberately does *not* write the file itself, which would race the CLI's
//! own save.

use crate::server::{ProtocolError, error_codes};
use buffer_diff::BufferDiff;
use editor::{DiffViewStyle, MultiBuffer, SelectionEffects, SplittableEditor, scroll::Autoscroll};
use futures::channel::oneshot;
use gpui::{AnyWindowHandle, AppContext as _, AsyncApp, DismissEvent, WeakEntity};
use language::Buffer;
use serde_json::{Value, json};
use ui::{Color, IconName};
use std::{cell::RefCell, rc::Rc};
use workspace::{
    SaveIntent, SplitDirection, Workspace,
    notifications::{NotificationId, simple_message_notification::MessageNotification},
};

/// Distinguishes our notification from others in the notification registry.
struct ClaudeDiffNotification;

pub async fn open_diff(
    workspace: WeakEntity<Workspace>,
    window: Option<AnyWindowHandle>,
    arguments: Value,
    cx: &mut AsyncApp,
) -> Result<Value, ProtocolError> {
    let string_arg = |key: &str| arguments.get(key).and_then(Value::as_str).map(str::to_owned);

    let old_file_path = string_arg("old_file_path")
        .ok_or_else(|| ProtocolError::new(error_codes::INVALID_REQUEST, "missing old_file_path"))?;
    let new_file_contents = string_arg("new_file_contents").ok_or_else(|| {
        ProtocolError::new(error_codes::INVALID_REQUEST, "missing new_file_contents")
    })?;
    let tab_name = string_arg("tab_name").unwrap_or_else(|| "Proposed changes".to_owned());

    let window =
        window.ok_or_else(|| ProtocolError::internal("no window available to show a diff"))?;

    // The current on-disk contents are the diff base (the "old" side).
    let old_contents = {
        let path = old_file_path.clone();
        smol::unblock(move || std::fs::read_to_string(&path).unwrap_or_default()).await
    };

    let (decision_tx, decision_rx) = oneshot::channel::<bool>();
    let decision_tx = Rc::new(RefCell::new(Some(decision_tx)));

    let message = format!("Claude proposes changes to {old_file_path}");

    // Build the proposed buffer and start computing the diff against the old
    // contents (the base). `set_base_text` is asynchronous, so we await it
    // before showing the editor to avoid flashing a "whole file added" diff.
    let (buffer, diff, base_ready) = window
        .update(cx, |_root, _window, cx| {
            let buffer = cx.new(|cx| Buffer::local(new_file_contents.clone(), cx));
            let snapshot = buffer.read(cx).text_snapshot();
            let diff = cx.new(|cx| BufferDiff::new(&snapshot, cx));
            let base_ready = diff.update(cx, |diff, cx| {
                diff.set_base_text(Some(old_contents.into()), None, snapshot, cx)
            });
            (buffer, diff, base_ready)
        })
        .map_err(|error| ProtocolError::internal(error.to_string()))?;

    base_ready.await.ok();

    let editor_id = window
        .update(cx, |_root, window, cx| {
            let multibuffer = cx.new(|cx| {
                let mut multibuffer = MultiBuffer::singleton(buffer.clone(), cx);
                multibuffer.add_diff(diff, cx);
                multibuffer
            });

            let Some(workspace_entity) = workspace.upgrade() else {
                return None;
            };
            let project = workspace_entity.read(cx).project().clone();

            // `DiffViewStyle::Split` renders side-by-side (old on the left, new on
            // the right) like the JetBrains diff viewer; `SplittableEditor` is
            // itself a workspace item, so it can be opened directly as a tab.
            let diff_editor = cx.new(|cx| {
                SplittableEditor::new(
                    DiffViewStyle::Split,
                    multibuffer,
                    project,
                    workspace_entity.clone(),
                    window,
                    cx,
                )
            });
            let editor_id = diff_editor.entity_id();

            workspace_entity.update(cx, |workspace, cx| {
                // The active pane holds Claude's integrated terminal: the CLI has
                // focus there when it sends `openDiff`. Show the diff in any other
                // pane so it doesn't cover the terminal; if the terminal is the
                // only pane, split a new one off to its left.
                let active_pane_id = workspace.active_pane().entity_id();
                let other_pane = workspace
                    .panes()
                    .iter()
                    .find(|pane| pane.entity_id() != active_pane_id)
                    .cloned();
                if let Some(target_pane) = other_pane {
                    workspace.add_item(
                        target_pane,
                        Box::new(diff_editor.clone()),
                        None,
                        true,
                        true,
                        window,
                        cx,
                    );
                } else {
                    workspace.split_item(
                        SplitDirection::Left,
                        Box::new(diff_editor.clone()),
                        window,
                        cx,
                    );
                }

                let tx_keep = decision_tx.clone();
                let tx_reject = decision_tx.clone();
                workspace.show_notification(
                    NotificationId::composite::<ClaudeDiffNotification>(tab_name),
                    cx,
                    move |cx| {
                        cx.new(|cx| {
                            MessageNotification::new(message, cx)
                                .primary_message("Keep")
                                .primary_icon(IconName::Check)
                                .primary_icon_color(Color::Success)
                                .primary_on_click(move |_window, cx| {
                                    if let Some(tx) = tx_keep.borrow_mut().take() {
                                        let _ = tx.send(true);
                                    }
                                    cx.emit(DismissEvent);
                                })
                                .secondary_message("Reject")
                                .secondary_icon(IconName::Close)
                                .secondary_icon_color(Color::Error)
                                .secondary_on_click(move |_window, cx| {
                                    if let Some(tx) = tx_reject.borrow_mut().take() {
                                        let _ = tx.send(false);
                                    }
                                    cx.emit(DismissEvent);
                                })
                        })
                    },
                );
            });

            // Center the view on the first change so the user lands on the diff
            // rather than at the top of an otherwise-unchanged file.
            diff_editor.update(cx, |diff_editor, cx| {
                let editor = diff_editor.rhs_editor().clone();
                editor.update(cx, |editor, cx| {
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    if let Some(first_hunk) = snapshot.diff_hunks().next() {
                        let start = first_hunk.multi_buffer_range.start;
                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::center()),
                            window,
                            cx,
                            |selections| selections.select_anchor_ranges([start..start]),
                        );
                    }
                });
            });
            Some(editor_id)
        })
        .map_err(|error| ProtocolError::internal(error.to_string()))?;

    // Block until the user clicks Keep/Reject. A dropped sender (e.g. window
    // closed) resolves to `false`, i.e. rejected.
    let accepted = decision_rx.await.unwrap_or(false);

    // Read the final buffer contents (the user may have edited the proposed
    // side) and close the diff tab now that the decision is made.
    let final_contents = window
        .update(cx, |_root, window, cx| {
            let final_contents = buffer.read(cx).text();
            if let Some(editor_id) = editor_id
                && let Some(workspace_entity) = workspace.upgrade()
            {
                workspace_entity.update(cx, |workspace, cx| {
                    for pane in workspace.panes().to_vec() {
                        pane.update(cx, |pane, cx| {
                            pane.close_item_by_id(editor_id, SaveIntent::Skip, window, cx)
                        })
                        .detach();
                    }
                });
            }
            final_contents
        })
        .map_err(|error| ProtocolError::internal(error.to_string()))?;

    // The official IDE protocol has the IDE return the accepted contents and the
    // CLI perform the write, so we must not write the file here ourselves.
    if accepted {
        Ok(json!({ "content": [
            { "type": "text", "text": "FILE_SAVED" },
            { "type": "text", "text": final_contents },
        ] }))
    } else {
        Ok(json!({ "content": [{ "type": "text", "text": "DIFF_REJECTED" }] }))
    }
}
