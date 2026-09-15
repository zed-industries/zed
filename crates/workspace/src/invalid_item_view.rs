use std::{path::Path, sync::Arc};

use anyhow::{Context as _, Result};
use gpui::{EventEmitter, FocusHandle, Focusable, Task, WeakEntity};
use ui::{
    App, Button, ButtonCommon, ButtonStyle, Clickable, Context, Disableable, FluentBuilder,
    InteractiveElement, KeyBinding, Label, LabelCommon, LabelSize, ParentElement, Render,
    SharedString, Styled as _, Window, h_flex, v_flex,
};
use util::ResultExt;
use zed_actions::workspace::OpenWithSystem;

use crate::{
    Item, ItemId, SerializableItemRegistry, Workspace, WorkspaceId, item::SaveDisposition,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SerializedItemReference {
    pub workspace_id: WorkspaceId,
    pub kind: Arc<str>,
    pub item_id: ItemId,
}

/// A view to display when a certain buffer/image/other item fails to open.
pub struct InvalidItemView {
    /// Which path was attempted to open.
    pub abs_path: Arc<Path>,
    /// An error message, happened when opening the item.
    pub error: SharedString,
    is_local: bool,
    focus_handle: FocusHandle,
    serialized_reference: Option<SerializedItemReference>,
    workspace: Option<WeakEntity<Workspace>>,
    retry_task: Option<Task<()>>,
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
            serialized_reference: None,
            workspace: None,
            retry_task: None,
        }
    }

    pub(crate) fn for_serialized_item(
        reference: SerializedItemReference,
        workspace: WeakEntity<Workspace>,
        error: &anyhow::Error,
        window: &mut Window,
        cx: &mut App,
    ) -> Self {
        let mut item = Self::new(
            Path::new(&format!("{} {}", reference.kind, reference.item_id)),
            false,
            error,
            window,
            cx,
        );
        item.serialized_reference = Some(reference);
        item.workspace = Some(workspace);
        item
    }

    pub(crate) fn serialized_reference(&self) -> Option<&SerializedItemReference> {
        self.serialized_reference.as_ref()
    }

    pub(crate) fn retry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.retry_task.is_some() {
            return;
        }
        let Some(reference) = self.serialized_reference.clone() else {
            return;
        };
        let Some(workspace) = self.workspace.clone() else {
            return;
        };
        let item_id = cx.entity_id();
        self.retry_task = Some(cx.spawn_in(window, async move |this, cx| {
            let result: Result<()> = async {
                this.update(cx, |_, _| ())?;
                let (pane, project) = workspace.read_with(cx, |workspace, cx| {
                    let pane = workspace
                        .panes
                        .iter()
                        .find(|pane| pane.read(cx).index_for_item_id(item_id).is_some())
                        .cloned();
                    (pane, workspace.project().clone())
                })?;
                let pane = pane.context("failed tab was closed")?;
                let restored = pane
                    .update_in(cx, |pane, window, cx| {
                        anyhow::ensure!(
                            pane.index_for_item_id(item_id).is_some(),
                            "failed tab was closed"
                        );
                        Ok::<_, anyhow::Error>(SerializableItemRegistry::deserialize(
                            &reference.kind,
                            project,
                            workspace.clone(),
                            reference.workspace_id,
                            reference.item_id,
                            window,
                            cx,
                        ))
                    })??
                    .await?;
                workspace.update_in(cx, |workspace, window, cx| {
                    let pane = workspace
                        .panes
                        .iter()
                        .find(|pane| pane.read(cx).index_for_item_id(item_id).is_some())
                        .cloned()
                        .context("failed tab was closed")?;
                    let serializable = restored
                        .to_serializable_item_handle(cx)
                        .context("restored item is not serializable")?;
                    anyhow::ensure!(
                        serializable.serialized_item_kind() == reference.kind.as_ref(),
                        "restored item kind changed"
                    );
                    workspace.register_serialized_item_id(
                        serializable.serialized_item_kind(),
                        restored.item_id(),
                        reference.item_id,
                        cx,
                    )?;
                    let index = pane
                        .read(cx)
                        .index_for_item_id(item_id)
                        .context("failed tab was closed")?;
                    pane.update(cx, |pane, cx| {
                        let pinned_count = pane.pinned_count();
                        let active = pane
                            .active_item()
                            .is_some_and(|item| item.item_id() == item_id);
                        let focus = active && pane.has_focus(window, cx);
                        let preview = pane.preview_item_id() == Some(item_id);
                        pane.remove_item(item_id, false, false, window, cx);
                        pane.add_restored_item(
                            restored.clone(),
                            active,
                            focus,
                            Some(index),
                            window,
                            cx,
                        );
                        pane.set_pinned_count(pinned_count);
                        if preview {
                            pane.set_preview_item_id(Some(restored.item_id()), cx);
                        }
                    });
                    workspace.serialize_workspace(window, cx);
                    Ok(())
                })?
            }
            .await;
            this.update(cx, |this, cx| {
                if let Err(error) = result {
                    this.error = SharedString::from(format!("{error:#}"));
                }
                if let Some(task) = this.retry_task.take() {
                    task.detach();
                }
                cx.notify();
            })
            .log_err();
        }));
        cx.notify();
    }
}

impl Item for InvalidItemView {
    type Event = ();

    fn can_move_to(
        &self,
        workspace: &WeakEntity<Workspace>,
        in_center_group: bool,
        _cx: &App,
    ) -> bool {
        self.serialized_reference.is_none()
            || (in_center_group && self.workspace.as_ref() == Some(workspace))
    }

    fn is_dirty(&self, _: &App) -> bool {
        self.serialized_reference.is_some()
    }

    fn save_disposition(&self, _: &App) -> SaveDisposition {
        if self.serialized_reference.is_some() {
            SaveDisposition::DiscardOnly
        } else {
            SaveDisposition::Normal
        }
    }

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
                        .when(self.serialized_reference.is_some(), |contents| {
                            contents
                                .child(
                                    h_flex().justify_center().child(
                                        Button::new("retry-restoration", "Retry")
                                            .disabled(self.retry_task.is_some())
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.retry(window, cx)
                                            })),
                                    ),
                                )
                                .child(
                                    Label::new("Close this tab to discard the saved reference")
                                        .size(LabelSize::Small),
                                )
                        })
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
