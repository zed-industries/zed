use std::sync::Arc;

use editor::Editor;
use gpui::{
    div, AsyncWindowContext, Entity, IntoElement, ParentElement, Render, Subscription, Task, View,
    ViewContext, WeakModel, WeakView,
};
use language::{
    language_settings::all_language_settings, Buffer, File, LanguageName, Toolchain,
    ToolchainLister,
};
use settings::SettingsStore;
use ui::{Button, ButtonCommon, Clickable, FluentBuilder, LabelSize, SharedString, Tooltip};
use workspace::{item::ItemHandle, StatusItemView, Workspace, WorkspaceId};

use crate::ToolchainSelector;

pub struct ActiveToolchain {
    lister: Option<Arc<dyn ToolchainLister>>,
    active_toolchain: Option<Toolchain>,
    workspace: WeakView<Workspace>,
    active_buffer: Option<WeakModel<Buffer>>,
    _observe_active_editor: Option<Subscription>,
    _observe_language_changes: Subscription,
    _update_toolchain_task: Task<Option<()>>,
}

impl ActiveToolchain {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let view = cx.view().clone();
        Self {
            lister: None,
            active_toolchain: None,
            active_buffer: None,
            workspace: workspace.weak_handle(),
            _observe_active_editor: None,
            _observe_language_changes: cx.observe(&view, |this, _, cx| {
                this._update_toolchain_task = Self::spawn_tracker_task(cx);
            }),
            _update_toolchain_task: Self::spawn_tracker_task(cx),
        }
    }
    fn spawn_tracker_task(cx: &mut ViewContext<Self>) -> Task<Option<()>> {
        cx.spawn(|this, mut cx| async move {
            let (lister, active_file) = this
                .update(&mut cx, |this, _| {
                    this.lister.clone().zip(this.active_buffer.clone())
                })
                .ok()
                .flatten()?;
            let workspace_id = this
                .update(&mut cx, |this, cx| {
                    this.workspace.update(cx, |this, _| this.database_id())
                })
                .ok()
                .map(Result::ok)
                .flatten()
                .flatten()?;
            let language_name = active_file
                .update(&mut cx, |this, _| Some(this.language()?.name()))
                .ok()
                .flatten()?;
            let toolchain = Self::active_toolchain(
                workspace_id,
                language_name,
                lister,
                active_file
                    .read_with(&mut cx, |this, _| this.file().cloned())
                    .ok()
                    .flatten()?,
                cx.clone(),
            )
            .await?;
            let _ = this.update(&mut cx, |this, cx| {
                this.active_toolchain = Some(toolchain);

                cx.notify();
            });
            Some(())
        })
    }

    fn update_lister(&mut self, editor: View<Editor>, cx: &mut ViewContext<Self>) {
        self.lister = None;

        let editor = editor.read(cx);
        if let Some((_, buffer, _)) = editor.active_excerpt(cx) {
            self.lister = buffer
                .read(cx)
                .language()
                .and_then(|language| language.toolchain_lister());
            self.active_buffer = Some(buffer.downgrade());
        }

        cx.notify();
    }

    fn active_toolchain(
        workspace_id: WorkspaceId,
        language_name: LanguageName,
        toolchain: Arc<dyn ToolchainLister>,
        file: Arc<dyn File>,
        mut cx: AsyncWindowContext,
    ) -> Task<Option<Toolchain>> {
        cx.spawn(move |_| async move {
            let toolchain_for = workspace::WORKSPACE_DB
                .toolchain(workspace_id, language_name)
                .await;

            let toolchains = toolchain.list().await;
            toolchains.default_toolchain()
        })
    }
}

impl Render for ActiveToolchain {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().when_some(self.active_toolchain.as_ref(), |el, active_toolchain| {
            el.child(
                Button::new("change-toolchain", active_toolchain.label.clone())
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _, cx| {
                        if let Some(workspace) = this.workspace.upgrade() {
                            workspace.update(cx, |workspace, cx| {
                                ToolchainSelector::toggle(workspace, cx)
                            });
                        }
                    }))
                    .tooltip(|cx| Tooltip::text("Select Toolchain", cx)),
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
        if let Some(editor) = active_pane_item.and_then(|item| item.act_as::<Editor>(cx)) {
            self.active_toolchain.take();
            self._observe_active_editor = Some(cx.observe(&editor, Self::update_lister));
            self.update_lister(editor, cx);
        } else {
            self.lister = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
