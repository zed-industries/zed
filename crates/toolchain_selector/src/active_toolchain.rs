use editor::Editor;
use gpui::{
    div, AsyncWindowContext, IntoElement, ParentElement, Render, Subscription, Task, View,
    ViewContext, WeakModel, WeakView,
};
use language::{Buffer, BufferEvent, LanguageName, Toolchain};
use project::{Project, WorktreeId};
use ui::{Button, ButtonCommon, Clickable, FluentBuilder, LabelSize, SharedString, Tooltip};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

use crate::ToolchainSelector;

pub struct ActiveToolchain {
    active_toolchain: Option<Toolchain>,
    term: SharedString,
    workspace: WeakView<Workspace>,
    active_buffer: Option<(WorktreeId, WeakModel<Buffer>, Subscription)>,
    _update_toolchain_task: Task<Option<()>>,
}

impl ActiveToolchain {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        Self {
            active_toolchain: None,
            active_buffer: None,
            term: SharedString::new_static("Toolchain"),
            workspace: workspace.weak_handle(),

            _update_toolchain_task: Self::spawn_tracker_task(cx),
        }
    }
    fn spawn_tracker_task(cx: &mut ViewContext<Self>) -> Task<Option<()>> {
        cx.spawn(|this, mut cx| async move {
            let active_file = this
                .update(&mut cx, |this, _| {
                    this.active_buffer
                        .as_ref()
                        .map(|(_, buffer, _)| buffer.clone())
                })
                .ok()
                .flatten()?;
            let workspace = this
                .update(&mut cx, |this, _| this.workspace.clone())
                .ok()?;
            let language_name = active_file
                .update(&mut cx, |this, _| Some(this.language()?.name()))
                .ok()
                .flatten()?;
            let term = workspace
                .update(&mut cx, |workspace, cx| {
                    let languages = workspace.project().read(cx).languages();
                    Project::toolchain_term(languages.clone(), language_name.clone())
                })
                .ok()?
                .await?;
            let _ = this.update(&mut cx, |this, cx| {
                this.term = term;
                cx.notify();
            });
            let worktree_id = active_file
                .update(&mut cx, |this, cx| Some(this.file()?.worktree_id(cx)))
                .ok()
                .flatten()?;
            let toolchain =
                Self::active_toolchain(workspace, worktree_id, language_name, cx.clone()).await?;
            let _ = this.update(&mut cx, |this, cx| {
                this.active_toolchain = Some(toolchain);

                cx.notify();
            });
            Some(())
        })
    }

    fn update_lister(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            if let Some(worktree_id) = buffer.read(cx).file().map(|file| file.worktree_id(cx)) {
                let subscription = cx.subscribe(&buffer, |this, _, event: &BufferEvent, cx| {
                    if matches!(event, BufferEvent::LanguageChanged) {
                        this._update_toolchain_task = Self::spawn_tracker_task(cx);
                    }
                });
                self.active_buffer = Some((worktree_id, buffer.downgrade(), subscription));
                self._update_toolchain_task = Self::spawn_tracker_task(cx);
            }
        }

        cx.notify();
    }

    fn active_toolchain(
        workspace: WeakView<Workspace>,
        worktree_id: WorktreeId,
        language_name: LanguageName,
        cx: AsyncWindowContext,
    ) -> Task<Option<Toolchain>> {
        cx.spawn(move |mut cx| async move {
            let workspace_id = workspace
                .update(&mut cx, |this, _| this.database_id())
                .ok()
                .flatten()?;
            let selected_toolchain = workspace
                .update(&mut cx, |this, cx| {
                    this.project()
                        .read(cx)
                        .active_toolchain(worktree_id, language_name.clone(), cx)
                })
                .ok()?
                .await;
            if let Some(toolchain) = selected_toolchain {
                Some(toolchain)
            } else {
                let project = workspace
                    .update(&mut cx, |this, _| this.project().clone())
                    .ok()?;
                let toolchains = cx
                    .update(|cx| {
                        project
                            .read(cx)
                            .available_toolchains(worktree_id, language_name, cx)
                    })
                    .ok()?
                    .await?;
                if let Some(toolchain) = toolchains.toolchains.first() {
                    // Since we don't have a selected toolchain, pick one for user here.
                    workspace::WORKSPACE_DB
                        .set_toolchain(workspace_id, worktree_id, toolchain.clone())
                        .await
                        .ok()?;
                    project
                        .update(&mut cx, |this, cx| {
                            this.activate_toolchain(worktree_id, toolchain.clone(), cx)
                        })
                        .ok()?
                        .await;
                }

                toolchains.toolchains.first().cloned()
            }
        })
    }
}

impl Render for ActiveToolchain {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().when_some(self.active_toolchain.as_ref(), |el, active_toolchain| {
            let term = self.term.clone();
            el.child(
                Button::new("change-toolchain", active_toolchain.name.clone())
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                ToolchainSelector::toggle(workspace, cx)
                            });
                        }
                    }))
                    .tooltip(move |cx| Tooltip::text(format!("Select {}", &term), cx)),
            )
        })
    }
}

impl StatusItemView for ActiveToolchain {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
            self.active_toolchain.take();
            self.update_lister(editor, cx);
        }
        cx.notify();
    }
}
