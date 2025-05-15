use anyhow::anyhow;
use gpui::{
    AnyElement, Empty, Entity, FocusHandle, Focusable, ListState, MouseButton, Stateful,
    Subscription, WeakEntity, list,
};
use project::{
    ProjectItem as _, ProjectPath,
    debugger::session::{Session, SessionEvent},
};
use std::{path::Path, sync::Arc};
use ui::{Scrollbar, ScrollbarState, prelude::*};
use util::maybe;
use workspace::Workspace;

pub struct ModuleList {
    list: ListState,
    invalidate: bool,
    session: Entity<Session>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    scrollbar_state: ScrollbarState,
    _subscription: Subscription,
}

impl ModuleList {
    pub fn new(
        session: Entity<Session>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let weak_entity = cx.weak_entity();
        let focus_handle = cx.focus_handle();

        let list = ListState::new(
            0,
            gpui::ListAlignment::Top,
            px(1000.),
            move |ix, _window, cx| {
                weak_entity
                    .upgrade()
                    .map(|module_list| module_list.update(cx, |this, cx| this.render_entry(ix, cx)))
                    .unwrap_or(div().into_any())
            },
        );

        let _subscription = cx.subscribe(&session, |this, _, event, cx| match event {
            SessionEvent::Stopped(_) | SessionEvent::Modules => {
                this.invalidate = true;
                cx.notify();
            }
            _ => {}
        });

        Self {
            scrollbar_state: ScrollbarState::new(list.clone()),
            list,
            session,
            workspace,
            focus_handle,
            _subscription,
            invalidate: true,
        }
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
                                    path: relative_path.into(),
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
        .detach_and_log_err(cx);
    }

    fn render_entry(&mut self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        let Some(module) = maybe!({
            self.session
                .update(cx, |state, cx| state.modules(cx).get(ix).cloned())
        }) else {
            return Empty.into_any();
        };

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
                    let path = module.path.as_deref().map(|path| Arc::<Path>::from(Path::new(path)));
                    cx.listener(move |this, _, window, cx| {
                        if let Some(path) = path.as_ref() {
                            this.open_module(path.clone(), window, cx);
                        } else {
                            log::error!("Wasn't able to find module path, but was still able to click on module list entry");
                        }
                    })
                })
            })
            .p_1()
            .hover(|s| s.bg(cx.theme().colors().element_hover))
            .child(h_flex().gap_0p5().text_ui_sm(cx).child(module.name.clone()))
            .child(
                h_flex()
                    .text_ui_xs(cx)
                    .text_color(cx.theme().colors().text_muted)
                    .when_some(module.path.clone(), |this, path| this.child(path)),
            )
            .into_any()
    }

    #[cfg(test)]
    pub(crate) fn modules(&self, cx: &mut Context<Self>) -> Vec<dap::Module> {
        self.session
            .update(cx, |session, cx| session.modules(cx).to_vec())
    }
    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .occlude()
            .id("module-list-vertical-scrollbar")
            .on_mouse_move(cx.listener(|_, _, _, cx| {
                cx.notify();
                cx.stop_propagation()
            }))
            .on_hover(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|_, _, _, cx| {
                    cx.stop_propagation();
                }),
            )
            .on_scroll_wheel(cx.listener(|_, _, _, cx| {
                cx.notify();
            }))
            .h_full()
            .absolute()
            .right_1()
            .top_1()
            .bottom_0()
            .w(px(12.))
            .cursor_default()
            .children(Scrollbar::vertical(self.scrollbar_state.clone()))
    }
}

impl Focusable for ModuleList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ModuleList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.invalidate {
            let len = self
                .session
                .update(cx, |session, cx| session.modules(cx).len());
            self.list.reset(len);
            self.invalidate = false;
            cx.notify();
        }

        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
            .child(self.render_vertical_scrollbar(cx))
    }
}
