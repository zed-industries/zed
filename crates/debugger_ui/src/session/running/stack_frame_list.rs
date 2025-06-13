use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow};
use dap::StackFrameId;
use gpui::{
    AnyElement, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, ListState, MouseButton,
    Stateful, Subscription, Task, WeakEntity, list,
};
use util::debug_panic;

use crate::StackTraceView;
use language::PointUtf16;
use project::debugger::breakpoint_store::ActiveStackFrame;
use project::debugger::session::{Session, SessionEvent, StackFrame};
use project::{ProjectItem, ProjectPath};
use ui::{Scrollbar, ScrollbarState, Tooltip, prelude::*};
use workspace::{ItemHandle, Workspace};

use super::RunningState;

#[derive(Debug)]
pub enum StackFrameListEvent {
    SelectedStackFrameChanged(StackFrameId),
    BuiltEntries,
}

pub struct StackFrameList {
    focus_handle: FocusHandle,
    _subscription: Subscription,
    session: Entity<Session>,
    state: WeakEntity<RunningState>,
    entries: Vec<StackFrameEntry>,
    workspace: WeakEntity<Workspace>,
    selected_ix: Option<usize>,
    opened_stack_frame_id: Option<StackFrameId>,
    scrollbar_state: ScrollbarState,
    list_state: ListState,
    error: Option<SharedString>,
    _refresh_task: Task<()>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackFrameEntry {
    Normal(dap::StackFrame),
    /// Used to indicate that the frame is artificial and is a visual label or separator
    Label(dap::StackFrame),
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
        let focus_handle = cx.focus_handle();

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

        let list_state = ListState::new(0, gpui::ListAlignment::Top, px(1000.), {
            let this = cx.weak_entity();
            move |ix, _window, cx| {
                this.update(cx, |this, cx| this.render_entry(ix, cx))
                    .unwrap_or(div().into_any())
            }
        });
        let scrollbar_state = ScrollbarState::new(list_state.clone());

        let mut this = Self {
            session,
            workspace,
            focus_handle,
            state,
            _subscription,
            entries: Default::default(),
            error: None,
            selected_ix: None,
            opened_stack_frame_id: None,
            list_state,
            scrollbar_state,
            _refresh_task: Task::ready(()),
        };
        this.schedule_refresh(true, window, cx);
        this
    }

    #[cfg(test)]
    pub(crate) fn entries(&self) -> &Vec<StackFrameEntry> {
        &self.entries
    }

    pub(crate) fn flatten_entries(
        &self,
        show_collapsed: bool,
        show_labels: bool,
    ) -> Vec<dap::StackFrame> {
        self.entries
            .iter()
            .flat_map(|frame| match frame {
                StackFrameEntry::Normal(frame) => vec![frame.clone()],
                StackFrameEntry::Label(frame) if show_labels => vec![frame.clone()],
                StackFrameEntry::Collapsed(frames) if show_collapsed => frames.clone(),
                _ => vec![],
            })
            .collect::<Vec<_>>()
    }

    fn stack_frames(&self, cx: &mut App) -> Result<Vec<StackFrame>> {
        if let Ok(Some(thread_id)) = self.state.read_with(cx, |state, _| state.thread_id) {
            self.session
                .update(cx, |this, cx| this.stack_frames(thread_id, cx))
        } else {
            Ok(Vec::default())
        }
    }

    #[cfg(test)]
    pub(crate) fn dap_stack_frames(&self, cx: &mut App) -> Vec<dap::StackFrame> {
        self.stack_frames(cx)
            .unwrap_or_default()
            .into_iter()
            .map(|stack_frame| stack_frame.dap.clone())
            .collect()
    }

    pub fn opened_stack_frame_id(&self) -> Option<StackFrameId> {
        self.opened_stack_frame_id
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
                    new_stack_frames.unwrap_or_default().is_empty() && !this.entries.is_empty()
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
        open_first_stack_frame: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let old_selected_frame_id = self
            .selected_ix
            .and_then(|ix| self.entries.get(ix))
            .and_then(|entry| match entry {
                StackFrameEntry::Normal(stack_frame) => Some(stack_frame.id),
                StackFrameEntry::Collapsed(_) | StackFrameEntry::Label(_) => None,
            });
        let mut entries = Vec::new();
        let mut collapsed_entries = Vec::new();
        let mut first_stack_frame = None;
        let mut first_stack_frame_with_path = None;

        let stack_frames = match self.stack_frames(cx) {
            Ok(stack_frames) => stack_frames,
            Err(e) => {
                self.error = Some(format!("{}", e).into());
                self.entries.clear();
                self.selected_ix = None;
                self.list_state.reset(0);
                cx.emit(StackFrameListEvent::BuiltEntries);
                cx.notify();
                return;
            }
        };
        for stack_frame in &stack_frames {
            match stack_frame.dap.presentation_hint {
                Some(dap::StackFramePresentationHint::Deemphasize)
                | Some(dap::StackFramePresentationHint::Subtle) => {
                    collapsed_entries.push(stack_frame.dap.clone());
                }
                Some(dap::StackFramePresentationHint::Label) => {
                    entries.push(StackFrameEntry::Label(stack_frame.dap.clone()));
                }
                _ => {
                    let collapsed_entries = std::mem::take(&mut collapsed_entries);
                    if !collapsed_entries.is_empty() {
                        entries.push(StackFrameEntry::Collapsed(collapsed_entries.clone()));
                    }

                    first_stack_frame.get_or_insert(entries.len());

                    if stack_frame
                        .dap
                        .source
                        .as_ref()
                        .is_some_and(|source| source.path.is_some())
                    {
                        first_stack_frame_with_path.get_or_insert(entries.len());
                    }
                    entries.push(StackFrameEntry::Normal(stack_frame.dap.clone()));
                }
            }
        }

        let collapsed_entries = std::mem::take(&mut collapsed_entries);
        if !collapsed_entries.is_empty() {
            entries.push(StackFrameEntry::Collapsed(collapsed_entries.clone()));
        }
        self.entries = entries;

        if let Some(ix) = first_stack_frame_with_path
            .or(first_stack_frame)
            .filter(|_| open_first_stack_frame)
        {
            self.select_ix(Some(ix), cx);
            self.activate_selected_entry(window, cx);
        } else if let Some(old_selected_frame_id) = old_selected_frame_id {
            let ix = self.entries.iter().position(|entry| match entry {
                StackFrameEntry::Normal(frame) => frame.id == old_selected_frame_id,
                StackFrameEntry::Collapsed(_) | StackFrameEntry::Label(_) => false,
            });
            self.selected_ix = ix;
        }

        self.list_state.reset(self.entries.len());
        cx.emit(StackFrameListEvent::BuiltEntries);
        cx.notify();
    }

    pub fn go_to_stack_frame(
        &mut self,
        stack_frame_id: StackFrameId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let Some(stack_frame) = self
            .entries
            .iter()
            .flat_map(|entry| match entry {
                StackFrameEntry::Label(stack_frame) => std::slice::from_ref(stack_frame),
                StackFrameEntry::Normal(stack_frame) => std::slice::from_ref(stack_frame),
                StackFrameEntry::Collapsed(stack_frames) => stack_frames.as_slice(),
            })
            .find(|stack_frame| stack_frame.id == stack_frame_id)
            .cloned()
        else {
            return Task::ready(Err(anyhow!("No stack frame for ID")));
        };
        self.go_to_stack_frame_inner(stack_frame, window, cx)
    }

    fn go_to_stack_frame_inner(
        &mut self,
        stack_frame: dap::StackFrame,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let stack_frame_id = stack_frame.id;
        self.opened_stack_frame_id = Some(stack_frame_id);
        let Some(abs_path) = Self::abs_path_from_stack_frame(&stack_frame) else {
            return Task::ready(Err(anyhow!("Project path not found")));
        };
        let row = stack_frame.line.saturating_sub(1) as u32;
        cx.emit(StackFrameListEvent::SelectedStackFrameChanged(
            stack_frame_id,
        ));
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
            let position = buffer.read_with(cx, |this, _| {
                this.snapshot().anchor_after(PointUtf16::new(row, 0))
            })?;
            this.update_in(cx, |this, window, cx| {
                this.workspace.update(cx, |workspace, cx| {
                    let project_path = buffer
                        .read(cx)
                        .project_path(cx)
                        .context("Could not select a stack frame for unnamed buffer")?;

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
                let thread_id = this.state.read_with(cx, |state, _| {
                    state.thread_id.context("No selected thread ID found")
                })??;

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
                .filter(|path| path.is_absolute())
        })
    }

    pub fn restart_stack_frame(&mut self, stack_frame_id: u64, cx: &mut Context<Self>) {
        self.session.update(cx, |state, cx| {
            state.restart_stack_frame(stack_frame_id, cx)
        });
    }

    fn render_label_entry(
        &self,
        stack_frame: &dap::StackFrame,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        h_flex()
            .rounded_md()
            .justify_between()
            .w_full()
            .group("")
            .id(("label-stack-frame", stack_frame.id))
            .p_1()
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .child(
                v_flex().justify_center().gap_0p5().child(
                    Label::new(stack_frame.name.clone())
                        .size(LabelSize::Small)
                        .weight(FontWeight::BOLD)
                        .truncate()
                        .color(Color::Info),
                ),
            )
            .into_any()
    }

    fn render_normal_entry(
        &self,
        ix: usize,
        stack_frame: &dap::StackFrame,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let source = stack_frame.source.clone();
        let is_selected_frame = Some(ix) == self.selected_ix;

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
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selected_ix = Some(ix);
                this.activate_selected_entry(window, cx);
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

    pub(crate) fn expand_collapsed_entry(&mut self, ix: usize, cx: &mut Context<Self>) {
        let Some(StackFrameEntry::Collapsed(stack_frames)) = self.entries.get_mut(ix) else {
            return;
        };
        let entries = std::mem::take(stack_frames)
            .into_iter()
            .map(StackFrameEntry::Normal);
        self.entries.splice(ix..ix + 1, entries);
        self.selected_ix = Some(ix);
        self.list_state.reset(self.entries.len());
        cx.emit(StackFrameListEvent::BuiltEntries);
        cx.notify();
    }

    fn render_collapsed_entry(
        &self,
        ix: usize,
        stack_frames: &Vec<dap::StackFrame>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let first_stack_frame = &stack_frames[0];
        let is_selected = Some(ix) == self.selected_ix;

        h_flex()
            .rounded_md()
            .justify_between()
            .w_full()
            .group("")
            .id(("stack-frame", first_stack_frame.id))
            .p_1()
            .when(is_selected, |this| {
                this.bg(cx.theme().colors().element_hover)
            })
            .on_any_mouse_down(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.selected_ix = Some(ix);
                this.activate_selected_entry(window, cx);
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
            StackFrameEntry::Label(stack_frame) => self.render_label_entry(stack_frame, cx),
            StackFrameEntry::Normal(stack_frame) => self.render_normal_entry(ix, stack_frame, cx),
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

    fn select_ix(&mut self, ix: Option<usize>, cx: &mut Context<Self>) {
        self.selected_ix = ix;
        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let ix = match self.selected_ix {
            _ if self.entries.len() == 0 => None,
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
            _ if self.entries.len() == 0 => None,
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
        let ix = if self.entries.len() > 0 {
            Some(0)
        } else {
            None
        };
        self.select_ix(ix, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let ix = if self.entries.len() > 0 {
            Some(self.entries.len() - 1)
        } else {
            None
        };
        self.select_ix(ix, cx);
    }

    fn activate_selected_entry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ix) = self.selected_ix else {
            return;
        };
        let Some(entry) = self.entries.get_mut(ix) else {
            return;
        };
        match entry {
            StackFrameEntry::Normal(stack_frame) => {
                let stack_frame = stack_frame.clone();
                self.go_to_stack_frame_inner(stack_frame, window, cx)
                    .detach_and_log_err(cx)
            }
            StackFrameEntry::Label(_) => {
                debug_panic!("You should not be able to select a label stack frame")
            }
            StackFrameEntry::Collapsed(_) => self.expand_collapsed_entry(ix, cx),
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.activate_selected_entry(window, cx);
    }

    fn render_list(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_1()
            .size_full()
            .child(list(self.list_state.clone()).size_full())
    }
}

impl Render for StackFrameList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .track_focus(&self.focus_handle)
            .size_full()
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .when_some(self.error.clone(), |el, error| {
                el.child(
                    h_flex()
                        .bg(cx.theme().status().warning_background)
                        .border_b_1()
                        .border_color(cx.theme().status().warning_border)
                        .pl_1()
                        .child(Icon::new(IconName::Warning).color(Color::Warning))
                        .gap_2()
                        .child(
                            Label::new(error)
                                .size(LabelSize::Small)
                                .color(Color::Warning),
                        ),
                )
            })
            .child(self.render_list(window, cx))
            .child(self.render_vertical_scrollbar(cx))
    }
}

impl Focusable for StackFrameList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<StackFrameListEvent> for StackFrameList {}
