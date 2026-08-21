use std::{path::Path, sync::Arc};

use gpui::{EventEmitter, FocusHandle, Focusable, Task, WeakEntity};
use ui::{
    App, Button, ButtonCommon, ButtonStyle, Clickable, Context, FluentBuilder, InteractiveElement,
    KeyBinding, Label, LabelCommon, LabelSize, ParentElement, Render, SharedString, Styled as _,
    Window, h_flex, v_flex,
};
use util::ResultExt as _;
use zed_actions::workspace::OpenWithSystem;

use crate::{Item, SaveIntent, Workspace};

/// A view to display when a certain buffer/image/other item fails to open.
#[derive(Debug)]
pub struct InvalidItemView {
    /// Which path was attempted to open.
    pub abs_path: Arc<Path>,
    /// An error message, happened when opening the item.
    pub error: SharedString,
    is_local: bool,
    pub is_binary: bool,
    workspace: Option<WeakEntity<Workspace>>,
    open_as_text_task: Option<Task<()>>,
    focus_handle: FocusHandle,
}

impl InvalidItemView {
    pub fn new(
        abs_path: &Path,
        is_local: bool,
        error: &anyhow::Error,
        _: &mut Window,
        cx: &mut App,
    ) -> Self {
        Self {
            is_local,
            is_binary: language::is_binary_file_error(error),
            abs_path: Arc::from(abs_path),
            error: format!("{}", error.root_cause()).into(),
            workspace: None,
            open_as_text_task: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn open_as_text(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.clone() else {
            log::error!(
                "cannot open {:?} as text: not in a workspace",
                self.abs_path
            );
            return;
        };
        let abs_path = self.abs_path.clone();
        let item_id = cx.entity_id();
        self.open_as_text_task = Some(cx.spawn_in(window, async move |_, cx| {
            let result = async {
                let (_worktree, project_path) = workspace
                    .update(cx, |workspace, cx| {
                        Workspace::project_path_for_path(
                            workspace.project().clone(),
                            &abs_path,
                            false,
                            cx,
                        )
                    })?
                    .await?;
                // Force the buffer open first: the regular open below then
                // reuses it (a path maps to at most one buffer at a time)
                // while building the item through the standard open flow. The
                // handle must stay alive until then, or the buffer would be
                // released before the regular open gets to it.
                let buffer = workspace
                    .update(cx, |workspace, cx| {
                        workspace.project().update(cx, |project, cx| {
                            project.open_buffer_forcing_text(project_path.clone(), cx)
                        })
                    })?
                    .await?;
                let pane = workspace.update(cx, |workspace, _| {
                    workspace
                        .pane_for_item_id(item_id)
                        .unwrap_or_else(|| workspace.active_pane().clone())
                })?;
                workspace
                    .update_in(cx, |workspace, window, cx| {
                        workspace.open_path(project_path, Some(pane.downgrade()), true, window, cx)
                    })?
                    .await?;
                drop(buffer);
                // A no-op when this view is no longer in the pane.
                pane.update_in(cx, |pane, window, cx| {
                    pane.close_item_by_id(item_id, SaveIntent::Skip, window, cx)
                })?
                .await?;
                anyhow::Ok(())
            }
            .await;
            if let Err(error) = result {
                workspace
                    .update(cx, |workspace, cx| workspace.show_error(error, cx))
                    .log_err();
            }
        }));
    }
}

impl Item for InvalidItemView {
    type Event = ();

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        self.workspace = Some(workspace.weak_handle());
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
                        .when(self.is_binary, |contents| {
                            contents.child(
                                h_flex().justify_center().child(
                                    Button::new("open-as-text", "Open as Text")
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.open_as_text(window, cx);
                                        }))
                                        .style(ButtonStyle::Outlined),
                                ),
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
