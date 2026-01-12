use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow};
use dap::StackFrameId;
use dap::adapters::DebugAdapterName;
use db::kvp::KEY_VALUE_STORE;
use gpui::{
    Action, AnyElement, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, ListState,
    Subscription, Task, WeakEntity, list,
};
use util::{
    debug_panic,
    paths::{PathStyle, is_absolute},
};

use crate::{StackTraceView, ToggleUserFrames};
use language::PointUtf16;
use project::debugger::breakpoint_store::ActiveStackFrame;
use project::debugger::session::{Session, SessionEvent, StackFrame, ThreadStatus};
use project::{ProjectItem, ProjectPath};
use ui::{Tooltip, WithScrollbar, prelude::*};
use workspace::{ItemHandle, Workspace, WorkspaceId};

use super::RunningState;

#[derive(Debug)]
pub enum StackFrameListEvent {
    SelectedStackFrameChanged(StackFrameId),
    BuiltEntries,
}

/// Represents the filter applied to the stack frame list
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub(crate) enum StackFrameFilter {
    /// Show all frames
    All,
    /// Show only frames from the user's code
    OnlyUserFrames,
}

impl StackFrameFilter {
    fn from_str_or_default(s: impl AsRef<str>) -> Self {
        match s.as_ref() {
            "user" => StackFrameFilter::OnlyUserFrames,
            "all" => StackFrameFilter::All,
            _ => StackFrameFilter::All,
        }
    }
}

impl From<StackFrameFilter> for String {
    fn from(filter: StackFrameFilter) -> Self {
        match filter {
            StackFrameFilter::All => "all".to_string(),
            StackFrameFilter::OnlyUserFrames => "user".to_string(),
        }
    }
}

pub(crate) fn stack_frame_filter_key(
    adapter_name: &DebugAdapterName,
    workspace_id: WorkspaceId,
) -> String {
    let database_id: i64 = workspace_id.into();
    format!("stack-frame-list-filter-{}-{}", adapter_name.0, database_id)
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
    list_state: ListState,
    list_filter: StackFrameFilter,
    filter_entries_indices: Vec<usize>,
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
                SessionEvent::Stopped(..)
                | SessionEvent::StackTrace
                | SessionEvent::HistoricSnapshotSelected => {
                    this.schedule_refresh(true, window, cx);
                }
                _ => {}
            });

        let list_state = ListState::new(0, gpui::ListAlignment::Top, px(1000.));

        let list_filter = workspace
            .read_with(cx, |workspace, _| workspace.database_id())
            .ok()
            .flatten()
            .and_then(|database_id| {
                let key = stack_frame_filter_key(&session.read(cx).adapter(), database_id);
                KEY_VALUE_STORE
                    .read_kvp(&key)
                    .ok()
                    .flatten()
                    .map(StackFrameFilter::from_str_or_default)
            })
            .unwrap_or(StackFrameFilter::All);

        let mut this = Self {
            session,
            workspace,
            focus_handle,
            state,
            _subscription,
            entries: Default::default(),
            filter_entries_indices: Vec::default(),
            error: None,
            selected_ix: None,
            opened_stack_frame_id: None,
            list_filter,
            list_state,
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
            .enumerate()
            .filter(|(ix, _)| {
                self.list_filter == StackFrameFilter::All
                    || self
                        .filter_entries_indices
                        .binary_search_by_key(&ix, |ix| ix)
                        .is_ok()
            })
            .flat_map(|(_, frame)| match frame {
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
        match self.list_filter {
            StackFrameFilter::All => self
                .stack_frames(cx)
                .unwrap_or_default()
                .into_iter()
                .map(|stack_frame| stack_frame.dap)
                .collect(),
            StackFrameFilter::OnlyUserFrames => self
                .filter_entries_indices
                .iter()
                .map(|ix| match &self.entries[*ix] {
                    StackFrameEntry::Label(label) => label,
                    StackFrameEntry::Collapsed(_) => panic!("Collapsed tabs should not be visible"),
                    StackFrameEntry::Normal(frame) => frame,
                })
                .cloned()
                .collect(),
        }
    }

    #[cfg(test)]
    pub(crate) fn list_filter(&self) -> StackFrameFilter {
        self.list_filter
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
                self.filter_entries_indices.clear();
                cx.emit(StackFrameListEvent::BuiltEntries);
                cx.notify();
                return;
            }
        };

        let worktree_prefixes: Vec<_> = self
            .workspace
            .read_with(cx, |workspace, cx| {
                workspace
                    .visible_worktrees(cx)
                    .map(|tree| tree.read(cx).abs_path())
                    .collect()
            })
            .unwrap_or_default();

        let mut filter_entries_indices = Vec::default();
        for stack_frame in stack_frames.iter() {
            let frame_in_visible_worktree = stack_frame.dap.source.as_ref().is_some_and(|source| {
                source.path.as_ref().is_some_and(|path| {
                    worktree_prefixes
                        .iter()
                        .filter_map(|tree| tree.to_str())
                        .any(|tree| path.starts_with(tree))
                })
            });

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
                    if frame_in_visible_worktree {
                        filter_entries_indices.push(entries.len() - 1);
                    }
                }
            }
        }

        let collapsed_entries = std::mem::take(&mut collapsed_entries);
        if !collapsed_entries.is_empty() {
            entries.push(StackFrameEntry::Collapsed(collapsed_entries));
        }
        self.entries = entries;
        self.filter_entries_indices = filter_entries_indices;

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

        match self.list_filter {
            StackFrameFilter::All => {
                self.list_state.reset(self.entries.len());
            }
            StackFrameFilter::OnlyUserFrames => {
                self.list_state.reset(self.filter_entries_indices.len());
            }
        }
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
                                    path: relative_path,
                                },
                                cx,
                            )
                        })
                    })
                })??
                .await?;
            let position = buffer.read_with(cx, |this, _| {
                this.snapshot().anchor_after(PointUtf16::new(row, 0))
            });
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
                .filter(|path| {
                    // Since we do not know if we are debugging on the host or (a remote/WSL) target,
                    // we need to check if either the path is absolute as Posix or Windows.
                    is_absolute(path, PathStyle::Posix) || is_absolute(path, PathStyle::Windows)
                })
                .map(|path| Arc::<Path>::from(Path::new(path)))
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

        let path = source.and_then(|s| s.path.or(s.name));
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
            .overflow_x_scroll()
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
                                    IconName::RotateCcw,
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
        // HERE
        let entries_len = entries.len();
        self.entries.splice(ix..ix + 1, entries);
        let (Ok(filtered_indices_start) | Err(filtered_indices_start)) =
            self.filter_entries_indices.binary_search(&ix);

        for idx in &mut self.filter_entries_indices[filtered_indices_start..] {
            *idx += entries_len - 1;
        }

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
        let ix = match self.list_filter {
            StackFrameFilter::All => ix,
            StackFrameFilter::OnlyUserFrames => self.filter_entries_indices[ix],
        };

        match &self.entries[ix] {
            StackFrameEntry::Label(stack_frame) => self.render_label_entry(stack_frame, cx),
            StackFrameEntry::Normal(stack_frame) => self.render_normal_entry(ix, stack_frame, cx),
            StackFrameEntry::Collapsed(stack_frames) => {
                self.render_collapsed_entry(ix, stack_frames, cx)
            }
        }
    }

    fn select_ix(&mut self, ix: Option<usize>, cx: &mut Context<Self>) {
        self.selected_ix = ix;
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

    pub(crate) fn toggle_frame_filter(
        &mut self,
        thread_status: Option<ThreadStatus>,
        cx: &mut Context<Self>,
    ) {
        self.list_filter = match self.list_filter {
            StackFrameFilter::All => StackFrameFilter::OnlyUserFrames,
            StackFrameFilter::OnlyUserFrames => StackFrameFilter::All,
        };

        if let Some(database_id) = self
            .workspace
            .read_with(cx, |workspace, _| workspace.database_id())
            .ok()
            .flatten()
        {
            let key = stack_frame_filter_key(&self.session.read(cx).adapter(), database_id);
            let save_task = KEY_VALUE_STORE.write_kvp(key, self.list_filter.into());
            cx.background_spawn(save_task).detach();
        }

        if let Some(ThreadStatus::Stopped) = thread_status {
            match self.list_filter {
                StackFrameFilter::All => {
                    self.list_state.reset(self.entries.len());
                }
                StackFrameFilter::OnlyUserFrames => {
                    self.list_state.reset(self.filter_entries_indices.len());
                    if !self
                        .selected_ix
                        .map(|ix| self.filter_entries_indices.contains(&ix))
                        .unwrap_or_default()
                    {
                        self.selected_ix = None;
                    }
                }
            }

            if let Some(ix) = self.selected_ix {
                let scroll_to = match self.list_filter {
                    StackFrameFilter::All => ix,
                    StackFrameFilter::OnlyUserFrames => self
                        .filter_entries_indices
                        .binary_search_by_key(&ix, |ix| *ix)
                        .expect("This index will always exist"),
                };
                self.list_state.scroll_to_reveal_item(scroll_to);
            }

            cx.emit(StackFrameListEvent::BuiltEntries);
            cx.notify();
        }
    }

    fn render_list(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().p_1().size_full().child(
            list(
                self.list_state.clone(),
                cx.processor(|this, ix, _window, cx| this.render_entry(ix, cx)),
            )
            .size_full(),
        )
    }

    pub(crate) fn render_control_strip(&self) -> AnyElement {
        let tooltip_title = match self.list_filter {
            StackFrameFilter::All => "Show stack frames from your project",
            StackFrameFilter::OnlyUserFrames => "Show all stack frames",
        };

        h_flex()
            .child(
                IconButton::new(
                    "filter-by-visible-worktree-stack-frame-list",
                    IconName::ListFilter,
                )
                .tooltip(move |_window, cx| {
                    Tooltip::for_action(tooltip_title, &ToggleUserFrames, cx)
                })
                .toggle_state(self.list_filter == StackFrameFilter::OnlyUserFrames)
                .icon_size(IconSize::Small)
                .on_click(|_, window, cx| {
                    window.dispatch_action(ToggleUserFrames.boxed_clone(), cx)
                }),
            )
            .into_any_element()
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
            .vertical_scrollbar_for(&self.list_state, window, cx)
    }
}

impl Focusable for StackFrameList {
    fn focus_handle(&self, _: &gpui::App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<StackFrameListEvent> for StackFrameList {}
