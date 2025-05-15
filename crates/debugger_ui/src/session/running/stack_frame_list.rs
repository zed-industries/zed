use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Result, anyhow};
use dap::StackFrameId;
use gpui::{
    AnyElement, Entity, EventEmitter, FocusHandle, Focusable, ListState, MouseButton, Stateful,
    Subscription, Task, WeakEntity, list,
};

use language::PointUtf16;
use project::debugger::breakpoint_store::ActiveStackFrame;
use project::debugger::session::{Session, SessionEvent, StackFrame};
use project::{ProjectItem, ProjectPath};
use ui::{Scrollbar, ScrollbarState, Tooltip, prelude::*};
use util::ResultExt;
use workspace::{ItemHandle, Workspace};

use crate::StackTraceView;

use super::RunningState;

#[derive(Debug)]
pub enum StackFrameListEvent {
    SelectedStackFrameChanged(StackFrameId),
    BuiltEntries,
}

pub struct StackFrameList {
    list: ListState,
    focus_handle: FocusHandle,
    _subscription: Subscription,
    session: Entity<Session>,
    state: WeakEntity<RunningState>,
    entries: Vec<StackFrameEntry>,
    workspace: WeakEntity<Workspace>,
    selected_stack_frame_id: Option<StackFrameId>,
    scrollbar_state: ScrollbarState,
    _refresh_task: Task<()>,
}

#[allow(clippy::large_enum_variant)]
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
                SessionEvent::Threads => {
                    this.schedule_refresh(false, window, cx);
                }
                SessionEvent::Stopped(..) | SessionEvent::StackTrace => {
                    this.schedule_refresh(true, window, cx);
                }
                _ => {}
            });

        let mut this = Self {
            scrollbar_state: ScrollbarState::new(list.clone()),
            list,
            session,
            workspace,
            focus_handle,
            state,
            _subscription,
            entries: Default::default(),
            selected_stack_frame_id: None,
            _refresh_task: Task::ready(()),
        };
        this.schedule_refresh(true, window, cx);
        this
    }

    #[cfg(test)]
    pub(crate) fn entries(&self) -> &Vec<StackFrameEntry> {
        &self.entries
    }

    pub(crate) fn flatten_entries(&self, show_collapsed: bool) -> Vec<dap::StackFrame> {
        self.entries
            .iter()
            .flat_map(|frame| match frame {
                StackFrameEntry::Normal(frame) => vec![frame.clone()],
                StackFrameEntry::Collapsed(frames) => {
                    if show_collapsed {
                        frames.clone()
                    } else {
                        vec![]
                    }
                }
            })
            .collect::<Vec<_>>()
    }

    fn stack_frames(&self, cx: &mut App) -> Vec<StackFrame> {
        self.state
            .read_with(cx, |state, _| state.thread_id)
            .log_err()
            .flatten()
            .map(|thread_id| {
                self.session
                    .update(cx, |this, cx| this.stack_frames(thread_id, cx))
            })
            .unwrap_or_default()
    }

    #[cfg(test)]
    pub(crate) fn dap_stack_frames(&self, cx: &mut App) -> Vec<dap::StackFrame> {
        self.stack_frames(cx)
            .into_iter()
            .map(|stack_frame| stack_frame.dap.clone())
            .collect()
    }

    pub fn selected_stack_frame_id(&self) -> Option<StackFrameId> {
        self.selected_stack_frame_id
    }

    pub(crate) fn select_stack_frame_id(
        &mut self,
        id: StackFrameId,
        window: &Window,
        cx: &mut Context<Self>,
    ) {
        if !self.entries.iter().any(|entry| match entry {
            StackFrameEntry::Normal(entry) => entry.id == id,
            StackFrameEntry::Collapsed(stack_frames) => {
                stack_frames.iter().any(|frame| frame.id == id)
            }
        }) {
            return;
        }

        self.selected_stack_frame_id = Some(id);
        self.go_to_selected_stack_frame(window, cx);
    }

    pub(super) fn schedule_refresh(
        &mut self,
        select_first: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        const REFRESH_DEBOUNCE: Duration = Duration::from_millis(20);

        self._refresh_task = cx.spawn_in(window, async move |this, cx| {
            let debounce = this
                .update(cx, |this, cx| {
                    let new_stack_frames = this.stack_frames(cx);
                    new_stack_frames.is_empty() && !this.entries.is_empty()
                })
                .ok()
                .unwrap_or_default();

            if debounce {
                cx.background_executor().timer(REFRESH_DEBOUNCE).await;
            }
            this.update_in(cx, |this, window, cx| {
                this.build_entries(select_first, window, cx);
                cx.notify();
            })
            .ok();
        })
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

        cx.emit(StackFrameListEvent::BuiltEntries);
        cx.notify();
    }

    pub fn go_to_selected_stack_frame(&mut self, window: &Window, cx: &mut Context<Self>) {
        if let Some(selected_stack_frame_id) = self.selected_stack_frame_id {
            let frame = self
                .entries
                .iter()
                .find_map(|entry| match entry {
                    StackFrameEntry::Normal(dap) => {
                        if dap.id == selected_stack_frame_id {
                            Some(dap)
                        } else {
                            None
                        }
                    }
                    StackFrameEntry::Collapsed(daps) => {
                        daps.iter().find(|dap| dap.id == selected_stack_frame_id)
                    }
                })
                .cloned();

            if let Some(frame) = frame.as_ref() {
                self.select_stack_frame(frame, true, window, cx)
                    .detach_and_log_err(cx);
            }
        }
    }

    pub fn select_stack_frame(
        &mut self,
        stack_frame: &dap::StackFrame,
        go_to_stack_frame: bool,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.selected_stack_frame_id = Some(stack_frame.id);

        cx.emit(StackFrameListEvent::SelectedStackFrameChanged(
            stack_frame.id,
        ));
        cx.notify();

        if !go_to_stack_frame {
            return Task::ready(Ok(()));
        };

        let row = (stack_frame.line.saturating_sub(1)) as u32;

        let Some(abs_path) = Self::abs_path_from_stack_frame(&stack_frame) else {
            return Task::ready(Err(anyhow!("Project path not found")));
        };

        let stack_frame_id = stack_frame.id;
        cx.spawn_in(window, async move |this, cx| {
            let (worktree, relative_path) = this
                .update(cx, |this, cx| {
                    this.workspace.update(cx, |workspace, cx| {
                        workspace.project().update(cx, |this, cx| {
                            this.find_or_create_worktree(&abs_path, false, cx)
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
            let position = buffer.update(cx, |this, _| {
                this.snapshot().anchor_after(PointUtf16::new(row, 0))
            })?;
            this.update_in(cx, |this, window, cx| {
                this.workspace.update(cx, |workspace, cx| {
                    let project_path = buffer.read(cx).project_path(cx).ok_or_else(|| {
                        anyhow!("Could not select a stack frame for unnamed buffer")
                    })?;

                    let open_preview = !workspace
                        .item_of_type::<StackTraceView>(cx)
                        .map(|viewer| {
                            workspace
                                .active_item(cx)
                                .is_some_and(|item| item.item_id() == viewer.item_id())
                        })
                        .unwrap_or_default();

                    anyhow::Ok(workspace.open_path_preview(
                        project_path,
                        None,
                        true,
                        true,
                        open_preview,
                        window,
                        cx,
                    ))
                })
            })???
            .await?;

            this.update(cx, |this, cx| {
                let Some(thread_id) = this.state.read_with(cx, |state, _| state.thread_id)? else {
                    return Err(anyhow!("No selected thread ID found"));
                };

                this.workspace.update(cx, |workspace, cx| {
                    let breakpoint_store = workspace.project().read(cx).breakpoint_store();

                    breakpoint_store.update(cx, |store, cx| {
                        store.set_active_position(
                            ActiveStackFrame {
                                session_id: this.session.read(cx).session_id(),
                                thread_id,
                                stack_frame_id,
                                path: abs_path,
                                position,
                            },
                            cx,
                        );
                    })
                })
            })?
        })
    }

    pub(crate) fn abs_path_from_stack_frame(stack_frame: &dap::StackFrame) -> Option<Arc<Path>> {
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
        let is_selected_frame = Some(stack_frame.id) == self.selected_stack_frame_id;

        let path = source.clone().and_then(|s| s.path.or(s.name));
        let formatted_path = path.map(|path| format!("{}:{}", path, stack_frame.line,));
        let formatted_path = formatted_path.map(|path| {
            Label::new(path)
                .size(LabelSize::XSmall)
                .line_height_style(LineHeightStyle::UiLabel)
                .truncate()
                .color(Color::Muted)
        });

        let supports_frame_restart = self
            .session
            .read(cx)
            .capabilities()
            .supports_restart_frame
            .unwrap_or_default();

        let should_deemphasize = matches!(
            stack_frame.presentation_hint,
            Some(
                dap::StackFramePresentationHint::Subtle
                    | dap::StackFramePresentationHint::Deemphasize
            )
        );
        h_flex()
            .rounded_md()
            .justify_between()
            .w_full()
            .group("")
            .id(("stack-frame", stack_frame.id))
            .p_1()
            .when(is_selected_frame, |this| {
                this.bg(cx.theme().colors().element_hover)
            })
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
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
                    .gap_0p5()
                    .child(
                        Label::new(stack_frame.name.clone())
                            .size(LabelSize::Small)
                            .truncate()
                            .when(should_deemphasize, |this| this.color(Color::Muted)),
                    )
                    .children(formatted_path),
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
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
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

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .occlude()
            .id("stack-frame-list-vertical-scrollbar")
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

impl Render for StackFrameList {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .p_1()
            .child(list(self.list.clone()).size_full())
            .child(self.render_vertical_scrollbar(cx))
    }
}

impl Focusable for StackFrameList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<StackFrameListEvent> for StackFrameList {}
