use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use dap::StackFrameId;
use gpui::{
    list, AnyElement, Entity, EventEmitter, FocusHandle, Focusable, ListState, Subscription, Task,
    WeakEntity,
};

use language::Point;
use project::debugger::session::{Session, SessionEvent, StackFrame};
use project::ProjectItem;
use ui::{prelude::*, Tooltip};
use util::ResultExt;
use workspace::Workspace;

use super::RunningState;

#[derive(Debug)]
pub enum StackFrameListEvent {
    SelectedStackFrameChanged(StackFrameId),
}

pub struct StackFrameList {
    list: ListState,
    focus_handle: FocusHandle,
    _subscription: Subscription,
    session: Entity<Session>,
    state: WeakEntity<RunningState>,
    entries: Vec<StackFrameEntry>,
    workspace: WeakEntity<Workspace>,
    current_stack_frame_id: Option<StackFrameId>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackFrameEntry {
    Normal(dap::StackFrame),
    Collapsed(Vec<dap::StackFrame>),
}

impl StackFrameList {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        session: Entity<Session>,
        state: WeakEntity<RunningState>,
        window: &mut Window,
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
                    .map(|stack_frame_list| {
                        stack_frame_list.update(cx, |this, cx| this.render_entry(ix, cx))
                    })
                    .unwrap_or(div().into_any())
            },
        );

        let _subscription =
            cx.subscribe_in(&session, window, |this, _, event, window, cx| match event {
                SessionEvent::Stopped => {
                    this.build_entries(true, window, cx);
                }
                SessionEvent::StackTrace => {
                    this.build_entries(this.entries.is_empty(), window, cx);
                }
                _ => {}
            });

        Self {
            list,
            session,
            workspace,
            focus_handle,
            state,
            _subscription,
            entries: Default::default(),
            current_stack_frame_id: None,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn entries(&self) -> &Vec<StackFrameEntry> {
        &self.entries
    }

    fn stack_frames(&self, cx: &mut App) -> Vec<StackFrame> {
        self.state
            .read_with(cx, |state, _| state.thread.as_ref().map(|(id, _)| *id))
            .log_err()
            .flatten()
            .map(|thread_id| {
                self.session
                    .update(cx, |this, cx| this.stack_frames(thread_id, cx))
            })
            .unwrap_or_default()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn dap_stack_frames(&self, cx: &mut App) -> Vec<dap::StackFrame> {
        self.stack_frames(cx)
            .into_iter()
            .map(|stack_frame| stack_frame.dap.clone())
            .collect()
    }

    pub fn _get_main_stack_frame_id(&self, cx: &mut Context<Self>) -> u64 {
        self.stack_frames(cx)
            .first()
            .map(|stack_frame| stack_frame.dap.id)
            .unwrap_or(0)
    }

    pub fn current_stack_frame_id(&self) -> Option<u64> {
        self.current_stack_frame_id
    }

    pub(super) fn refresh(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        cx.defer_in(window, |this, window, cx| {
            this.build_entries(this.entries.is_empty(), window, cx);
        });
    }

    pub fn build_entries(
        &mut self,
        select_first_stack_frame: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut entries = Vec::new();
        let mut collapsed_entries = Vec::new();
        let mut current_stack_frame = None;

        let stack_frames = self.stack_frames(cx);
        for stack_frame in &stack_frames {
            match stack_frame.dap.presentation_hint {
                Some(dap::StackFramePresentationHint::Deemphasize) => {
                    collapsed_entries.push(stack_frame.dap.clone());
                }
                _ => {
                    let collapsed_entries = std::mem::take(&mut collapsed_entries);
                    if !collapsed_entries.is_empty() {
                        entries.push(StackFrameEntry::Collapsed(collapsed_entries.clone()));
                    }

                    current_stack_frame.get_or_insert(&stack_frame.dap);
                    entries.push(StackFrameEntry::Normal(stack_frame.dap.clone()));
                }
            }
        }

        let collapsed_entries = std::mem::take(&mut collapsed_entries);
        if !collapsed_entries.is_empty() {
            entries.push(StackFrameEntry::Collapsed(collapsed_entries.clone()));
        }

        std::mem::swap(&mut self.entries, &mut entries);
        self.list.reset(self.entries.len());

        if let Some(current_stack_frame) = current_stack_frame.filter(|_| select_first_stack_frame)
        {
            self.select_stack_frame(current_stack_frame, true, window, cx)
                .detach_and_log_err(cx);
        }

        cx.notify();
    }

    pub fn select_stack_frame(
        &mut self,
        stack_frame: &dap::StackFrame,
        go_to_stack_frame: bool,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.current_stack_frame_id = Some(stack_frame.id);

        cx.emit(StackFrameListEvent::SelectedStackFrameChanged(
            stack_frame.id,
        ));
        cx.notify();

        if !go_to_stack_frame {
            return Task::ready(Ok(()));
        };

        let row = (stack_frame.line.saturating_sub(1)) as u32;

        let Some(abs_path) = self.abs_path_from_stack_frame(&stack_frame) else {
            return Task::ready(Err(anyhow!("Project path not found")));
        };

        cx.spawn_in(window, move |this, mut cx| async move {
            let buffer = this
                .update(&mut cx, |this, cx| {
                    this.workspace.update(cx, |workspace, cx| {
                        // todo(debugger): This will cause an error if we hit a breakpoint that is outside the project
                        // open local buffer can't find a worktree_id because there is none
                        workspace
                            .project()
                            .update(cx, |this, cx| this.open_local_buffer(abs_path.clone(), cx))
                    })
                })??
                .await?;
            let position = buffer.update(&mut cx, |this, _| {
                this.snapshot().anchor_after(Point::new(row, 0))
            })?;
            this.update_in(&mut cx, |this, window, cx| {
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

            this.update(&mut cx, |this, cx| {
                this.workspace.update(cx, |workspace, cx| {
                    let breakpoint_store = workspace.project().read(cx).breakpoint_store();

                    breakpoint_store.update(cx, |store, cx| {
                        store.set_active_position(Some((abs_path, position)), cx);
                    })
                })
            })?
        })
    }

    fn abs_path_from_stack_frame(&self, stack_frame: &dap::StackFrame) -> Option<Arc<Path>> {
        stack_frame.source.as_ref().and_then(|s| {
            s.path
                .as_deref()
                .map(|path| Arc::<Path>::from(Path::new(path)))
        })
    }

    pub fn restart_stack_frame(&mut self, stack_frame_id: u64, cx: &mut Context<Self>) {
        self.session.update(cx, |state, cx| {
            state.restart_stack_frame(stack_frame_id, cx)
        });
    }

    fn render_normal_entry(
        &self,
        stack_frame: &dap::StackFrame,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let source = stack_frame.source.clone();
        let is_selected_frame = Some(stack_frame.id) == self.current_stack_frame_id;

        let formatted_path = format!(
            "{}:{}",
            source.clone().and_then(|s| s.name).unwrap_or_default(),
            stack_frame.line,
        );

        let supports_frame_restart = self
            .session
            .read(cx)
            .capabilities()
            .supports_restart_frame
            .unwrap_or_default();

        let origin = stack_frame
            .source
            .to_owned()
            .and_then(|source| source.origin);

        h_flex()
            .rounded_md()
            .justify_between()
            .w_full()
            .group("")
            .id(("stack-frame", stack_frame.id))
            .tooltip({
                let formatted_path = formatted_path.clone();
                move |_window, app| {
                    app.new(|_| {
                        let mut tooltip = Tooltip::new(formatted_path.clone());

                        if let Some(origin) = &origin {
                            tooltip = tooltip.meta(origin);
                        }

                        tooltip
                    })
                    .into()
                }
            })
            .p_1()
            .when(is_selected_frame, |this| {
                this.bg(cx.theme().colors().element_hover)
            })
            .on_click(cx.listener({
                let stack_frame = stack_frame.clone();
                move |this, _, window, cx| {
                    this.select_stack_frame(&stack_frame, true, window, cx)
                        .detach_and_log_err(cx);
                }
            }))
            .hover(|style| style.bg(cx.theme().colors().element_hover).cursor_pointer())
            .child(
                v_flex()
                    .child(
                        h_flex()
                            .gap_0p5()
                            .text_ui_sm(cx)
                            .truncate()
                            .child(stack_frame.name.clone())
                            .child(formatted_path),
                    )
                    .child(
                        h_flex()
                            .text_ui_xs(cx)
                            .truncate()
                            .text_color(cx.theme().colors().text_muted)
                            .when_some(source.and_then(|s| s.path), |this, path| this.child(path)),
                    ),
            )
            .when(
                supports_frame_restart && stack_frame.can_restart.unwrap_or(true),
                |this| {
                    this.child(
                        h_flex()
                            .id(("restart-stack-frame", stack_frame.id))
                            .visible_on_hover("")
                            .absolute()
                            .right_2()
                            .overflow_hidden()
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().colors().element_selected)
                            .bg(cx.theme().colors().element_background)
                            .hover(|style| {
                                style
                                    .bg(cx.theme().colors().ghost_element_hover)
                                    .cursor_pointer()
                            })
                            .child(
                                IconButton::new(
                                    ("restart-stack-frame", stack_frame.id),
                                    IconName::DebugRestart,
                                )
                                .icon_size(IconSize::Small)
                                .on_click(cx.listener({
                                    let stack_frame_id = stack_frame.id;
                                    move |this, _, _window, cx| {
                                        this.restart_stack_frame(stack_frame_id, cx);
                                    }
                                }))
                                .tooltip(move |window, cx| {
                                    Tooltip::text("Restart Stack Frame")(window, cx)
                                }),
                            ),
                    )
                },
            )
            .into_any()
    }

    pub fn expand_collapsed_entry(
        &mut self,
        ix: usize,
        stack_frames: &Vec<dap::StackFrame>,
        cx: &mut Context<Self>,
    ) {
        self.entries.splice(
            ix..ix + 1,
            stack_frames
                .iter()
                .map(|frame| StackFrameEntry::Normal(frame.clone())),
        );
        self.list.reset(self.entries.len());
        cx.notify();
    }

    fn render_collapsed_entry(
        &self,
        ix: usize,
        stack_frames: &Vec<dap::StackFrame>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let first_stack_frame = &stack_frames[0];

        h_flex()
            .rounded_md()
            .justify_between()
            .w_full()
            .group("")
            .id(("stack-frame", first_stack_frame.id))
            .p_1()
            .on_click(cx.listener({
                let stack_frames = stack_frames.clone();
                move |this, _, _window, cx| {
                    this.expand_collapsed_entry(ix, &stack_frames, cx);
                }
            }))
            .hover(|style| style.bg(cx.theme().colors().element_hover).cursor_pointer())
            .child(
                v_flex()
                    .text_ui_sm(cx)
                    .truncate()
                    .text_color(cx.theme().colors().text_muted)
                    .child(format!(
                        "Show {} more{}",
                        stack_frames.len(),
                        first_stack_frame
                            .source
                            .as_ref()
                            .and_then(|source| source.origin.as_ref())
                            .map_or(String::new(), |origin| format!(": {}", origin))
                    )),
            )
            .into_any()
    }

    fn render_entry(&self, ix: usize, cx: &mut Context<Self>) -> AnyElement {
        match &self.entries[ix] {
            StackFrameEntry::Normal(stack_frame) => self.render_normal_entry(stack_frame, cx),
            StackFrameEntry::Collapsed(stack_frames) => {
                self.render_collapsed_entry(ix, stack_frames, cx)
            }
        }
    }
}

impl Render for StackFrameList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
    }
}

impl Focusable for StackFrameList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<StackFrameListEvent> for StackFrameList {}
