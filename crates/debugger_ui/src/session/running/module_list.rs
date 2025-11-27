use anyhow::anyhow;
use dap::Module;
use gpui::{
    AnyElement, Entity, FocusHandle, Focusable, ScrollStrategy, Subscription, Task,
    UniformListScrollHandle, WeakEntity, uniform_list,
};
use project::{
    ProjectItem as _, ProjectPath,
    debugger::session::{Session, SessionEvent},
};
use std::{ops::Range, path::Path, sync::Arc};
use ui::{WithScrollbar, prelude::*};
use workspace::Workspace;

pub struct ModuleList {
    scroll_handle: UniformListScrollHandle,
    selected_ix: Option<usize>,
    session: Entity<Session>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    entries: Vec<Module>,
    _rebuild_task: Option<Task<()>>,
    _subscription: Subscription,
}

impl ModuleList {
    pub fn new(
        session: Entity<Session>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let _subscription = cx.subscribe(&session, |this, _, event, cx| match event {
            SessionEvent::Stopped(_) | SessionEvent::Modules => {
                if this._rebuild_task.is_some() {
                    this.schedule_rebuild(cx);
                }
            }
            _ => {}
        });

        let scroll_handle = UniformListScrollHandle::new();

        Self {
            scroll_handle,
            session,
            workspace,
            focus_handle,
            entries: Vec::new(),
            selected_ix: None,
            _subscription,
            _rebuild_task: None,
        }
    }

    fn schedule_rebuild(&mut self, cx: &mut Context<Self>) {
        self._rebuild_task = Some(cx.spawn(async move |this, cx| {
            this.update(cx, |this, cx| {
                let modules = this
                    .session
                    .update(cx, |session, cx| session.modules(cx).to_owned());
                this.entries = modules;
                cx.notify();
            })
            .ok();
        }));
    }

    fn open_module(&mut self, path: Arc<Path>, window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn_in(window, async move |this, cx| {
            let (worktree, relative_path) = this
                .update(cx, |this, cx| {
                    this.workspace.update(cx, |workspace, cx| {
                        workspace.project().update(cx, |this, cx| {
                            this.find_or_create_worktree(&path, false, cx)
                        })
                    })
                })??
                .await?;

            let buffer = this
                .update(cx, |this, cx| {
                    this.workspace.update(cx, |this, cx| {
                        this.project().update(cx, |this, cx| {
                            let worktree_id = worktree.read(cx).id();
                            this.open_buffer(
                                ProjectPath {
                                    worktree_id,
                                    path: relative_path,
                                },
                                cx,
                            )
                        })
                    })
                })??
                .await?;

            this.update_in(cx, |this, window, cx| {
                this.workspace.update(cx, |workspace, cx| {
                    let project_path = buffer.read(cx).project_path(cx).ok_or_else(|| {
                        anyhow!("Could not select a stack frame for unnamed buffer")
                    })?;
                    anyhow::Ok(workspace.open_path_preview(
                        project_path,
                        None,
                        false,
                        true,
                        true,
                        window,
                        cx,
                    ))
                })
            })???
            .await?;

            anyhow::Ok(())
        })
        .detach();
    }

    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let module = self.entries[ix].clone();

        v_flex()
            .rounded_md()
            .w_full()
            .group("")
            .id(("module-list", ix))
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .when(module.path.is_some(), |this| {
                this.on_click({
                    let path = module
                        .path
                        .as_deref()
                        .map(|path| Arc::<Path>::from(Path::new(path)));
                    cx.listener(move |this, _, window, cx| {
                        this.selected_ix = Some(ix);
                        if let Some(path) = path.as_ref() {
                            this.open_module(path.clone(), window, cx);
                        }
                        cx.notify();
                    })
                })
            })
            .p_1()
            .hover(|s| s.bg(cx.theme().colors().element_hover))
            .when(Some(ix) == self.selected_ix, |s| {
                s.bg(cx.theme().colors().element_hover)
            })
            .child(h_flex().gap_0p5().text_ui_sm(cx).child(module.name.clone()))
            .child(
                h_flex()
                    .text_ui_xs(cx)
                    .text_color(cx.theme().colors().text_muted)
                    .when_some(module.path, |this, path| this.child(path)),
            )
            .into_any()
    }

    #[cfg(test)]
    pub(crate) fn modules(&self, cx: &mut Context<Self>) -> Vec<dap::Module> {
        self.session
            .update(cx, |session, cx| session.modules(cx).to_vec())
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ix) = self.selected_ix else { return };
        let Some(entry) = self.entries.get(ix) else {
            return;
        };
        let Some(path) = entry.path.as_deref() else {
            return;
        };
        let path = Arc::from(Path::new(path));
        self.open_module(path, window, cx);
    }

    fn select_ix(&mut self, ix: Option<usize>, cx: &mut Context<Self>) {
        self.selected_ix = ix;
        if let Some(ix) = ix {
            self.scroll_handle
                .scroll_to_item(ix, ScrollStrategy::Center);
        }
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let ix = match self.selected_ix {
            _ if self.entries.is_empty() => None,
            None => Some(0),
            Some(ix) => {
                if ix == self.entries.len() - 1 {
                    Some(0)
                } else {
                    Some(ix + 1)
                }
            }
        };
        self.select_ix(ix, cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ix = match self.selected_ix {
            _ if self.entries.is_empty() => None,
            None => Some(self.entries.len() - 1),
            Some(ix) => {
                if ix == 0 {
                    Some(self.entries.len() - 1)
                } else {
                    Some(ix - 1)
                }
            }
        };
        self.select_ix(ix, cx);
    }

    fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ix = if !self.entries.is_empty() {
            Some(0)
        } else {
            None
        };
        self.select_ix(ix, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let ix = if !self.entries.is_empty() {
            Some(self.entries.len() - 1)
        } else {
            None
        };
        self.select_ix(ix, cx);
    }

    fn render_list(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        uniform_list(
            "module-list",
            self.entries.len(),
            cx.processor(|this, range: Range<usize>, _window, cx| {
                range.map(|ix| this.render_entry(ix, cx)).collect()
            }),
        )
        .track_scroll(&self.scroll_handle)
        .size_full()
    }
}

impl Focusable for ModuleList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ModuleList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self._rebuild_task.is_none() {
            self.schedule_rebuild(cx);
        }
        div()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::confirm))
            .size_full()
            .p_1()
            .child(self.render_list(window, cx))
            .vertical_scrollbar_for(&self.scroll_handle, window, cx)
    }
}
