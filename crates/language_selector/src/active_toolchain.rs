use std::sync::Arc;

use editor::Editor;
use gpui::{
    div, AsyncWindowContext, IntoElement, ParentElement, Render, Subscription, Task, View,
    ViewContext, WeakView,
};
use language::{language_settings::all_language_settings, File, Toolchain, ToolchainLister};
use ui::{Button, ButtonCommon, Clickable, FluentBuilder, LabelSize, Tooltip};
use workspace::{item::ItemHandle, StatusItemView, Workspace};

use crate::LanguageSelector;

pub struct ActiveToolchain {
    lister: Option<Arc<dyn ToolchainLister>>,
    active_toolchain: Option<Toolchain>,
    workspace: WeakView<Workspace>,
    active_file: Option<Arc<dyn File>>,
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
            active_file: None,
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
                    this.lister.clone().zip(this.active_file.clone())
                })
                .ok()
                .flatten()?;
            let toolchain = Self::active_toolchain(lister, active_file, cx.clone()).await?;
            let _ = this.update(&mut cx, |this, cx| {
                dbg!(&toolchain.label);
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
            self.active_file = buffer.read(cx).file().cloned();
        }

        cx.notify();
    }
    fn active_toolchain(
        toolchain: Arc<dyn ToolchainLister>,
        file: Arc<dyn File>,
        mut cx: AsyncWindowContext,
    ) -> Task<Option<Toolchain>> {
        let language = toolchain.language_name();
        let settings_for = cx
            .update(|cx| {
                all_language_settings(Some(&file), cx)
                    .language(Some(&language))
                    .toolchain
                    .clone()
            })
            .ok()
            .flatten();
        if let Some(toolchain) = settings_for {
            return Task::ready(Some(Toolchain {
                label: toolchain.name,
            }));
        }
        cx.spawn(move |_| async move {
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
                                LanguageSelector::toggle(workspace, cx)
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
            self._observe_active_editor = Some(cx.observe(&editor, Self::update_lister));
            self.update_lister(editor, cx);
        } else {
            self.lister = None;
            self._observe_active_editor = None;
        }

        cx.notify();
    }
}
