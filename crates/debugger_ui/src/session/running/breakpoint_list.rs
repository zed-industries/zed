use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use dap::{Capabilities, ExceptionBreakpointsFilter, adapters::DebugAdapterName};
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use gpui::{
    Action, AppContext, ClickEvent, Entity, FocusHandle, Focusable, MouseButton, ScrollStrategy,
    Task, UniformListScrollHandle, WeakEntity, actions, uniform_list,
};
use itertools::Itertools;
use language::Point;
use project::{
    Project,
    debugger::{
        breakpoint_store::{BreakpointEditAction, BreakpointStore, SourceBreakpoint},
        dap_store::{DapStore, PersistedAdapterOptions},
        session::Session,
    },
    worktree_store::WorktreeStore,
};
use ui::{
    Divider, DividerColor, FluentBuilder as _, Indicator, IntoElement, ListItem, Render,
    ScrollAxes, StatefulInteractiveElement, Tooltip, WithScrollbar, prelude::*,
};
use util::rel_path::RelPath;
use workspace::Workspace;
use zed_actions::{ToggleEnableBreakpoint, UnsetBreakpoint};

actions!(
    debugger,
    [
        /// Navigates to the previous breakpoint property in the list.
        PreviousBreakpointProperty,
        /// Navigates to the next breakpoint property in the list.
        NextBreakpointProperty
    ]
);
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum SelectedBreakpointKind {
    Source,
    Exception,
    Data,
}
pub(crate) struct BreakpointList {
    workspace: WeakEntity<Workspace>,
    breakpoint_store: Entity<BreakpointStore>,
    dap_store: Entity<DapStore>,
    worktree_store: Entity<WorktreeStore>,
    breakpoints: Vec<BreakpointEntry>,
    session: Option<Entity<Session>>,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    selected_ix: Option<usize>,
    max_width_index: Option<usize>,
    input: Entity<Editor>,
    strip_mode: Option<ActiveBreakpointStripMode>,
    serialize_exception_breakpoints_task: Option<Task<anyhow::Result<()>>>,
}

impl Focusable for BreakpointList {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ActiveBreakpointStripMode {
    Log,
    Condition,
    HitCondition,
}

impl BreakpointList {
    pub(crate) fn new(
        session: Option<Entity<Session>>,
        workspace: WeakEntity<Workspace>,
        project: &Entity<Project>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        let project = project.read(cx);
        let breakpoint_store = project.breakpoint_store();
        let worktree_store = project.worktree_store();
        let dap_store = project.dap_store();
        let focus_handle = cx.focus_handle();
        let scroll_handle = UniformListScrollHandle::new();

        let adapter_name = session.as_ref().map(|session| session.read(cx).adapter());
        cx.new(|cx| {
            let this = Self {
                breakpoint_store,
                dap_store,
                worktree_store,
                breakpoints: Default::default(),
                max_width_index: None,
                workspace,
                session,
                focus_handle,
                scroll_handle,
                selected_ix: None,
                input: cx.new(|cx| Editor::single_line(window, cx)),
                strip_mode: None,
                serialize_exception_breakpoints_task: None,
            };
            if let Some(name) = adapter_name {
                _ = this.deserialize_exception_breakpoints(name, cx);
            }
            this
        })
    }

    fn edit_line_breakpoint(
        &self,
        path: Arc<Path>,
        row: u32,
        action: BreakpointEditAction,
        cx: &mut App,
    ) {
        Self::edit_line_breakpoint_inner(&self.breakpoint_store, path, row, action, cx);
    }
    fn edit_line_breakpoint_inner(
        breakpoint_store: &Entity<BreakpointStore>,
        path: Arc<Path>,
        row: u32,
        action: BreakpointEditAction,
        cx: &mut App,
    ) {
        breakpoint_store.update(cx, |breakpoint_store, cx| {
            if let Some((buffer, breakpoint)) = breakpoint_store.breakpoint_at_row(&path, row, cx) {
                breakpoint_store.toggle_breakpoint(buffer, breakpoint, action, cx);
            } else {
                log::error!("Couldn't find breakpoint at row event though it exists: row {row}")
            }
        })
    }

    fn go_to_line_breakpoint(
        &mut self,
        path: Arc<Path>,
        row: u32,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let task = self
            .worktree_store
            .update(cx, |this, cx| this.find_or_create_worktree(path, false, cx));
        cx.spawn_in(window, async move |this, cx| {
            let (worktree, relative_path) = task.await?;
            let worktree_id = worktree.read_with(cx, |this, _| this.id());
            let item = this
                .update_in(cx, |this, window, cx| {
                    this.workspace.update(cx, |this, cx| {
                        this.open_path((worktree_id, relative_path), None, true, window, cx)
                    })
                })??
                .await?;
            if let Some(editor) = item.downcast::<Editor>() {
                editor
                    .update_in(cx, |this, window, cx| {
                        this.go_to_singleton_buffer_point(Point { row, column: 0 }, window, cx);
                    })
                    .ok();
            }
            anyhow::Ok(())
        })
        .detach();
    }

    pub(crate) fn selection_kind(&self) -> Option<(SelectedBreakpointKind, bool)> {
        self.selected_ix.and_then(|ix| {
            self.breakpoints.get(ix).map(|bp| match &bp.kind {
                BreakpointEntryKind::LineBreakpoint(bp) => (
                    SelectedBreakpointKind::Source,
                    bp.breakpoint.state
                        == project::debugger::breakpoint_store::BreakpointState::Enabled,
                ),
                BreakpointEntryKind::ExceptionBreakpoint(bp) => {
                    (SelectedBreakpointKind::Exception, bp.is_enabled)
                }
                BreakpointEntryKind::DataBreakpoint(bp) => {
                    (SelectedBreakpointKind::Data, bp.0.is_enabled)
                }
            })
        })
    }

    fn set_active_breakpoint_property(
        &mut self,
        prop: ActiveBreakpointStripMode,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.strip_mode = Some(prop);
        let placeholder = match prop {
            ActiveBreakpointStripMode::Log => "Set Log Message",
            ActiveBreakpointStripMode::Condition => "Set Condition",
            ActiveBreakpointStripMode::HitCondition => "Set Hit Condition",
        };
        let mut is_exception_breakpoint = true;
        let active_value = self.selected_ix.and_then(|ix| {
            self.breakpoints.get(ix).and_then(|bp| {
                if let BreakpointEntryKind::LineBreakpoint(bp) = &bp.kind {
                    is_exception_breakpoint = false;
                    match prop {
                        ActiveBreakpointStripMode::Log => bp.breakpoint.message.clone(),
                        ActiveBreakpointStripMode::Condition => bp.breakpoint.condition.clone(),
                        ActiveBreakpointStripMode::HitCondition => {
                            bp.breakpoint.hit_condition.clone()
                        }
                    }
                } else {
                    None
                }
            })
        });

        self.input.update(cx, |this, cx| {
            this.set_placeholder_text(placeholder, window, cx);
            this.set_read_only(is_exception_breakpoint);
            this.set_text(active_value.as_deref().unwrap_or(""), window, cx);
        });
    }

    fn select_ix(&mut self, ix: Option<usize>, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_ix = ix;
        if let Some(ix) = ix {
            self.scroll_handle
                .scroll_to_item(ix, ScrollStrategy::Center);
        }
        if let Some(mode) = self.strip_mode {
            self.set_active_breakpoint_property(mode, window, cx);
        }

        cx.notify();
    }

    fn select_next(&mut self, _: &menu::SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        if self.strip_mode.is_some() && self.input.focus_handle(cx).contains_focused(window, cx) {
            cx.propagate();
            return;
        }
        let ix = match self.selected_ix {
            _ if self.breakpoints.is_empty() => None,
            None => Some(0),
            Some(ix) => {
                if ix == self.breakpoints.len() - 1 {
                    Some(0)
                } else {
                    Some(ix + 1)
                }
            }
        };
        self.select_ix(ix, window, cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.strip_mode.is_some() && self.input.focus_handle(cx).contains_focused(window, cx) {
            cx.propagate();
            return;
        }
        let ix = match self.selected_ix {
            _ if self.breakpoints.is_empty() => None,
            None => Some(self.breakpoints.len() - 1),
            Some(ix) => {
                if ix == 0 {
                    Some(self.breakpoints.len() - 1)
                } else {
                    Some(ix - 1)
                }
            }
        };
        self.select_ix(ix, window, cx);
    }

    fn select_first(&mut self, _: &menu::SelectFirst, window: &mut Window, cx: &mut Context<Self>) {
        if self.strip_mode.is_some() && self.input.focus_handle(cx).contains_focused(window, cx) {
            cx.propagate();
            return;
        }
        let ix = if !self.breakpoints.is_empty() {
            Some(0)
        } else {
            None
        };
        self.select_ix(ix, window, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        if self.strip_mode.is_some() && self.input.focus_handle(cx).contains_focused(window, cx) {
            cx.propagate();
            return;
        }
        let ix = if !self.breakpoints.is_empty() {
            Some(self.breakpoints.len() - 1)
        } else {
            None
        };
        self.select_ix(ix, window, cx);
    }

    fn dismiss(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.input.focus_handle(cx).contains_focused(window, cx) {
            self.focus_handle.focus(window, cx);
        } else if self.strip_mode.is_some() {
            self.strip_mode.take();
            cx.notify();
        } else {
            cx.propagate();
        }
    }
    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.selected_ix.and_then(|ix| self.breakpoints.get_mut(ix)) else {
            return;
        };

        if let Some(mode) = self.strip_mode {
            let handle = self.input.focus_handle(cx);
            if handle.is_focused(window) {
                // Go back to the main strip. Save the result as well.
                let text = self.input.read(cx).text(cx);

                match mode {
                    ActiveBreakpointStripMode::Log => {
                        if let BreakpointEntryKind::LineBreakpoint(line_breakpoint) = &entry.kind {
                            Self::edit_line_breakpoint_inner(
                                &self.breakpoint_store,
                                line_breakpoint.breakpoint.path.clone(),
                                line_breakpoint.breakpoint.row,
                                BreakpointEditAction::EditLogMessage(Arc::from(text)),
                                cx,
                            );
                        }
                    }
                    ActiveBreakpointStripMode::Condition => {
                        if let BreakpointEntryKind::LineBreakpoint(line_breakpoint) = &entry.kind {
                            Self::edit_line_breakpoint_inner(
                                &self.breakpoint_store,
                                line_breakpoint.breakpoint.path.clone(),
                                line_breakpoint.breakpoint.row,
                                BreakpointEditAction::EditCondition(Arc::from(text)),
                                cx,
                            );
                        }
                    }
                    ActiveBreakpointStripMode::HitCondition => {
                        if let BreakpointEntryKind::LineBreakpoint(line_breakpoint) = &entry.kind {
                            Self::edit_line_breakpoint_inner(
                                &self.breakpoint_store,
                                line_breakpoint.breakpoint.path.clone(),
                                line_breakpoint.breakpoint.row,
                                BreakpointEditAction::EditHitCondition(Arc::from(text)),
                                cx,
                            );
                        }
                    }
                }
                self.focus_handle.focus(window, cx);
            } else {
                handle.focus(window, cx);
            }

            return;
        }
        match &mut entry.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                let path = line_breakpoint.breakpoint.path.clone();
                let row = line_breakpoint.breakpoint.row;
                self.go_to_line_breakpoint(path, row, window, cx);
            }
            BreakpointEntryKind::DataBreakpoint(_)
            | BreakpointEntryKind::ExceptionBreakpoint(_) => {}
        }
    }

    fn toggle_enable_breakpoint(
        &mut self,
        _: &ToggleEnableBreakpoint,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.selected_ix.and_then(|ix| self.breakpoints.get_mut(ix)) else {
            return;
        };
        if self.strip_mode.is_some() && self.input.focus_handle(cx).contains_focused(window, cx) {
            cx.propagate();
            return;
        }

        match &mut entry.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                let path = line_breakpoint.breakpoint.path.clone();
                let row = line_breakpoint.breakpoint.row;
                self.edit_line_breakpoint(path, row, BreakpointEditAction::InvertState, cx);
            }
            BreakpointEntryKind::ExceptionBreakpoint(exception_breakpoint) => {
                let id = exception_breakpoint.id.clone();
                self.toggle_exception_breakpoint(&id, cx);
            }
            BreakpointEntryKind::DataBreakpoint(data_breakpoint) => {
                let id = data_breakpoint.0.dap.data_id.clone();
                self.toggle_data_breakpoint(&id, cx);
            }
        }
        cx.notify();
    }

    fn unset_breakpoint(
        &mut self,
        _: &UnsetBreakpoint,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(entry) = self.selected_ix.and_then(|ix| self.breakpoints.get_mut(ix)) else {
            return;
        };

        if let BreakpointEntryKind::LineBreakpoint(line_breakpoint) = &mut entry.kind {
            let path = line_breakpoint.breakpoint.path.clone();
            let row = line_breakpoint.breakpoint.row;
            self.edit_line_breakpoint(path, row, BreakpointEditAction::Toggle, cx);
        }
        cx.notify();
    }

    fn previous_breakpoint_property(
        &mut self,
        _: &PreviousBreakpointProperty,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let next_mode = match self.strip_mode {
            Some(ActiveBreakpointStripMode::Log) => None,
            Some(ActiveBreakpointStripMode::Condition) => Some(ActiveBreakpointStripMode::Log),
            Some(ActiveBreakpointStripMode::HitCondition) => {
                Some(ActiveBreakpointStripMode::Condition)
            }
            None => Some(ActiveBreakpointStripMode::HitCondition),
        };
        if let Some(mode) = next_mode {
            self.set_active_breakpoint_property(mode, window, cx);
        } else {
            self.strip_mode.take();
        }

        cx.notify();
    }
    fn next_breakpoint_property(
        &mut self,
        _: &NextBreakpointProperty,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let next_mode = match self.strip_mode {
            Some(ActiveBreakpointStripMode::Log) => Some(ActiveBreakpointStripMode::Condition),
            Some(ActiveBreakpointStripMode::Condition) => {
                Some(ActiveBreakpointStripMode::HitCondition)
            }
            Some(ActiveBreakpointStripMode::HitCondition) => None,
            None => Some(ActiveBreakpointStripMode::Log),
        };
        if let Some(mode) = next_mode {
            self.set_active_breakpoint_property(mode, window, cx);
        } else {
            self.strip_mode.take();
        }
        cx.notify();
    }

    fn toggle_data_breakpoint(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Some(session) = &self.session {
            session.update(cx, |this, cx| {
                this.toggle_data_breakpoint(id, cx);
            });
        }
    }

    fn toggle_exception_breakpoint(&mut self, id: &str, cx: &mut Context<Self>) {
        if let Some(session) = &self.session {
            session.update(cx, |this, cx| {
                this.toggle_exception_breakpoint(id, cx);
            });
            cx.notify();
            const EXCEPTION_SERIALIZATION_INTERVAL: Duration = Duration::from_secs(1);
            self.serialize_exception_breakpoints_task = Some(cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(EXCEPTION_SERIALIZATION_INTERVAL)
                    .await;
                this.update(cx, |this, cx| this.serialize_exception_breakpoints(cx))?
                    .await?;
                Ok(())
            }));
        }
    }

    fn kvp_key(adapter_name: &str) -> String {
        format!("debug_adapter_`{adapter_name}`_persistence")
    }
    fn serialize_exception_breakpoints(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        if let Some(session) = self.session.as_ref() {
            let key = {
                let session = session.read(cx);
                let name = session.adapter().0;
                Self::kvp_key(&name)
            };
            let settings = self.dap_store.update(cx, |this, cx| {
                this.sync_adapter_options(session, cx);
            });
            let value = serde_json::to_string(&settings);

            cx.background_executor()
                .spawn(async move { KEY_VALUE_STORE.write_kvp(key, value?).await })
        } else {
            Task::ready(Result::Ok(()))
        }
    }

    fn deserialize_exception_breakpoints(
        &self,
        adapter_name: DebugAdapterName,
        cx: &mut Context<Self>,
    ) -> anyhow::Result<()> {
        let Some(val) = KEY_VALUE_STORE.read_kvp(&Self::kvp_key(&adapter_name))? else {
            return Ok(());
        };
        let value: PersistedAdapterOptions = serde_json::from_str(&val)?;
        self.dap_store
            .update(cx, |this, _| this.set_adapter_options(adapter_name, value));

        Ok(())
    }

    fn render_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_ix = self.selected_ix;
        let focus_handle = self.focus_handle.clone();
        let supported_breakpoint_properties = self
            .session
            .as_ref()
            .map(|session| SupportedBreakpointProperties::from(session.read(cx).capabilities()))
            .unwrap_or_else(SupportedBreakpointProperties::all);
        let strip_mode = self.strip_mode;

        uniform_list(
            "breakpoint-list",
            self.breakpoints.len(),
            cx.processor(move |this, range: Range<usize>, _, _| {
                range
                    .clone()
                    .zip(&mut this.breakpoints[range])
                    .map(|(ix, breakpoint)| {
                        breakpoint
                            .render(
                                strip_mode,
                                supported_breakpoint_properties,
                                ix,
                                Some(ix) == selected_ix,
                                focus_handle.clone(),
                            )
                            .into_any_element()
                    })
                    .collect()
            }),
        )
        .with_horizontal_sizing_behavior(gpui::ListHorizontalSizingBehavior::Unconstrained)
        .with_width_from_item(self.max_width_index)
        .track_scroll(&self.scroll_handle)
        .flex_1()
    }

    pub(crate) fn render_control_strip(&self) -> AnyElement {
        let selection_kind = self.selection_kind();
        let focus_handle = self.focus_handle.clone();

        let remove_breakpoint_tooltip = selection_kind.map(|(kind, _)| match kind {
            SelectedBreakpointKind::Source => "Remove breakpoint from a breakpoint list",
            SelectedBreakpointKind::Exception => {
                "Exception Breakpoints cannot be removed from the breakpoint list"
            }
            SelectedBreakpointKind::Data => "Remove data breakpoint from a breakpoint list",
        });

        let toggle_label = selection_kind.map(|(_, is_enabled)| {
            if is_enabled {
                (
                    "Disable Breakpoint",
                    "Disable a breakpoint without removing it from the list",
                )
            } else {
                ("Enable Breakpoint", "Re-enable a breakpoint")
            }
        });

        h_flex()
            .child(
                IconButton::new(
                    "disable-breakpoint-breakpoint-list",
                    IconName::DebugDisabledBreakpoint,
                )
                .icon_size(IconSize::Small)
                .when_some(toggle_label, |this, (label, meta)| {
                    this.tooltip({
                        let focus_handle = focus_handle.clone();
                        move |_window, cx| {
                            Tooltip::with_meta_in(
                                label,
                                Some(&ToggleEnableBreakpoint),
                                meta,
                                &focus_handle,
                                cx,
                            )
                        }
                    })
                })
                .disabled(selection_kind.is_none())
                .on_click({
                    let focus_handle = focus_handle.clone();
                    move |_, window, cx| {
                        focus_handle.focus(window, cx);
                        window.dispatch_action(ToggleEnableBreakpoint.boxed_clone(), cx)
                    }
                }),
            )
            .child(
                IconButton::new("remove-breakpoint-breakpoint-list", IconName::Trash)
                    .icon_size(IconSize::Small)
                    .when_some(remove_breakpoint_tooltip, |this, tooltip| {
                        this.tooltip({
                            let focus_handle = focus_handle.clone();
                            move |_window, cx| {
                                Tooltip::with_meta_in(
                                    "Remove Breakpoint",
                                    Some(&UnsetBreakpoint),
                                    tooltip,
                                    &focus_handle,
                                    cx,
                                )
                            }
                        })
                    })
                    .disabled(
                        selection_kind.map(|kind| kind.0) != Some(SelectedBreakpointKind::Source),
                    )
                    .on_click({
                        move |_, window, cx| {
                            focus_handle.focus(window, cx);
                            window.dispatch_action(UnsetBreakpoint.boxed_clone(), cx)
                        }
                    }),
            )
            .into_any_element()
    }
}

impl Render for BreakpointList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let breakpoints = self.breakpoint_store.read(cx).all_source_breakpoints(cx);
        self.breakpoints.clear();
        let path_style = self.worktree_store.read(cx).path_style();
        let weak = cx.weak_entity();
        let breakpoints = breakpoints.into_iter().flat_map(|(path, mut breakpoints)| {
            let relative_worktree_path = self
                .worktree_store
                .read(cx)
                .find_worktree(&path, cx)
                .and_then(|(worktree, relative_path)| {
                    worktree
                        .read(cx)
                        .is_visible()
                        .then(|| worktree.read(cx).root_name().join(&relative_path))
                });
            breakpoints.sort_by_key(|breakpoint| breakpoint.row);
            let weak = weak.clone();
            breakpoints.into_iter().filter_map(move |breakpoint| {
                debug_assert_eq!(&path, &breakpoint.path);
                let file_name = breakpoint.path.file_name()?;
                let breakpoint_path = RelPath::new(&breakpoint.path, path_style).ok();

                let dir = relative_worktree_path
                    .as_deref()
                    .or(breakpoint_path.as_deref())?
                    .parent()
                    .map(|parent| SharedString::from(parent.display(path_style).to_string()));
                let name = file_name
                    .to_str()
                    .map(ToOwned::to_owned)
                    .map(SharedString::from)?;
                let weak = weak.clone();
                let line = breakpoint.row + 1;
                Some(BreakpointEntry {
                    kind: BreakpointEntryKind::LineBreakpoint(LineBreakpoint {
                        name,
                        dir,
                        line,
                        breakpoint,
                    }),
                    weak,
                })
            })
        });
        let exception_breakpoints = self.session.as_ref().into_iter().flat_map(|session| {
            session
                .read(cx)
                .exception_breakpoints()
                .map(|(data, is_enabled)| BreakpointEntry {
                    kind: BreakpointEntryKind::ExceptionBreakpoint(ExceptionBreakpoint {
                        id: data.filter.clone(),
                        data: data.clone(),
                        is_enabled: *is_enabled,
                    }),
                    weak: weak.clone(),
                })
        });
        let data_breakpoints = self.session.as_ref().into_iter().flat_map(|session| {
            session
                .read(cx)
                .data_breakpoints()
                .map(|state| BreakpointEntry {
                    kind: BreakpointEntryKind::DataBreakpoint(DataBreakpoint(state.clone())),
                    weak: weak.clone(),
                })
        });
        self.breakpoints.extend(
            breakpoints
                .chain(data_breakpoints)
                .chain(exception_breakpoints),
        );

        let text_pixels = ui::TextSize::Default.pixels(cx).to_f64() as f32;

        self.max_width_index = self
            .breakpoints
            .iter()
            .map(|entry| match &entry.kind {
                BreakpointEntryKind::LineBreakpoint(line_bp) => {
                    let name_and_line = format!("{}:{}", line_bp.name, line_bp.line);
                    let dir_len = line_bp.dir.as_ref().map(|d| d.len()).unwrap_or(0);
                    (name_and_line.len() + dir_len) as f32 * text_pixels
                }
                BreakpointEntryKind::ExceptionBreakpoint(exc_bp) => {
                    exc_bp.data.label.len() as f32 * text_pixels
                }
                BreakpointEntryKind::DataBreakpoint(data_bp) => {
                    data_bp.0.context.human_readable_label().len() as f32 * text_pixels
                }
            })
            .position_max_by(|left, right| left.total_cmp(right));

        v_flex()
            .id("breakpoint-list")
            .key_context("BreakpointList")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::dismiss))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::toggle_enable_breakpoint))
            .on_action(cx.listener(Self::unset_breakpoint))
            .on_action(cx.listener(Self::next_breakpoint_property))
            .on_action(cx.listener(Self::previous_breakpoint_property))
            .size_full()
            .pt_1()
            .child(self.render_list(cx))
            .custom_scrollbars(
                ui::Scrollbars::new(ScrollAxes::Both)
                    .tracked_scroll_handle(&self.scroll_handle)
                    .with_track_along(ScrollAxes::Both, cx.theme().colors().panel_background)
                    .tracked_entity(cx.entity_id()),
                window,
                cx,
            )
            .when_some(self.strip_mode, |this, _| {
                this.child(Divider::horizontal().color(DividerColor::Border))
                    .child(
                        h_flex()
                            .p_1()
                            .rounded_sm()
                            .bg(cx.theme().colors().editor_background)
                            .border_1()
                            .when(
                                self.input.focus_handle(cx).contains_focused(window, cx),
                                |this| {
                                    let colors = cx.theme().colors();

                                    let border_color = if self.input.read(cx).read_only(cx) {
                                        colors.border_disabled
                                    } else {
                                        colors.border_transparent
                                    };

                                    this.border_color(border_color)
                                },
                            )
                            .child(self.input.clone()),
                    )
            })
    }
}

#[derive(Clone, Debug)]
struct LineBreakpoint {
    name: SharedString,
    dir: Option<SharedString>,
    line: u32,
    breakpoint: SourceBreakpoint,
}

impl LineBreakpoint {
    fn render(
        &mut self,
        props: SupportedBreakpointProperties,
        strip_mode: Option<ActiveBreakpointStripMode>,
        ix: usize,
        is_selected: bool,
        focus_handle: FocusHandle,
        weak: WeakEntity<BreakpointList>,
    ) -> ListItem {
        let icon_name = if self.breakpoint.state.is_enabled() {
            IconName::DebugBreakpoint
        } else {
            IconName::DebugDisabledBreakpoint
        };
        let path = self.breakpoint.path.clone();
        let row = self.breakpoint.row;
        let is_enabled = self.breakpoint.state.is_enabled();

        let indicator = div()
            .id(SharedString::from(format!(
                "breakpoint-ui-toggle-{:?}/{}:{}",
                self.dir, self.name, self.line
            )))
            .child(
                Icon::new(icon_name)
                    .color(Color::Debugger)
                    .size(IconSize::XSmall),
            )
            .tooltip({
                let focus_handle = focus_handle.clone();
                move |_window, cx| {
                    Tooltip::for_action_in(
                        if is_enabled {
                            "Disable Breakpoint"
                        } else {
                            "Enable Breakpoint"
                        },
                        &ToggleEnableBreakpoint,
                        &focus_handle,
                        cx,
                    )
                }
            })
            .on_click({
                let weak = weak.clone();
                let path = path.clone();
                move |_, _, cx| {
                    weak.update(cx, |breakpoint_list, cx| {
                        breakpoint_list.edit_line_breakpoint(
                            path.clone(),
                            row,
                            BreakpointEditAction::InvertState,
                            cx,
                        );
                    })
                    .ok();
                }
            })
            .on_mouse_down(MouseButton::Left, move |_, _, _| {});

        ListItem::new(SharedString::from(format!(
            "breakpoint-ui-item-{:?}/{}:{}",
            self.dir, self.name, self.line
        )))
        .toggle_state(is_selected)
        .inset(true)
        .on_click({
            let weak = weak.clone();
            move |_, window, cx| {
                weak.update(cx, |breakpoint_list, cx| {
                    breakpoint_list.select_ix(Some(ix), window, cx);
                })
                .ok();
            }
        })
        .on_secondary_mouse_down(|_, _, cx| {
            cx.stop_propagation();
        })
        .start_slot(indicator)
        .child(
            h_flex()
                .id(SharedString::from(format!(
                    "breakpoint-ui-on-click-go-to-line-{:?}/{}:{}",
                    self.dir, self.name, self.line
                )))
                .w_full()
                .gap_1()
                .min_h(rems_from_px(26.))
                .justify_between()
                .on_click({
                    let weak = weak.clone();
                    move |_, window, cx| {
                        weak.update(cx, |breakpoint_list, cx| {
                            breakpoint_list.select_ix(Some(ix), window, cx);
                            breakpoint_list.go_to_line_breakpoint(path.clone(), row, window, cx);
                        })
                        .ok();
                    }
                })
                .child(
                    h_flex()
                        .id("label-container")
                        .gap_0p5()
                        .child(
                            Label::new(format!("{}:{}", self.name, self.line))
                                .size(LabelSize::Small)
                                .line_height_style(ui::LineHeightStyle::UiLabel),
                        )
                        .children(self.dir.as_ref().and_then(|dir| {
                            let path_without_root = Path::new(dir.as_ref())
                                .components()
                                .skip(1)
                                .collect::<PathBuf>();
                            path_without_root.components().next()?;
                            Some(
                                Label::new(path_without_root.to_string_lossy().into_owned())
                                    .color(Color::Muted)
                                    .size(LabelSize::Small)
                                    .line_height_style(ui::LineHeightStyle::UiLabel)
                                    .truncate(),
                            )
                        }))
                        .when_some(self.dir.as_ref(), |this, parent_dir| {
                            this.tooltip(Tooltip::text(format!(
                                "Worktree parent path: {parent_dir}"
                            )))
                        }),
                )
                .child(BreakpointOptionsStrip {
                    props,
                    breakpoint: BreakpointEntry {
                        kind: BreakpointEntryKind::LineBreakpoint(self.clone()),
                        weak,
                    },
                    is_selected,
                    focus_handle,
                    strip_mode,
                    index: ix,
                }),
        )
    }
}

#[derive(Clone, Debug)]
struct ExceptionBreakpoint {
    id: String,
    data: ExceptionBreakpointsFilter,
    is_enabled: bool,
}

#[derive(Clone, Debug)]
struct DataBreakpoint(project::debugger::session::DataBreakpointState);

impl DataBreakpoint {
    fn render(
        &self,
        props: SupportedBreakpointProperties,
        strip_mode: Option<ActiveBreakpointStripMode>,
        ix: usize,
        is_selected: bool,
        focus_handle: FocusHandle,
        list: WeakEntity<BreakpointList>,
    ) -> ListItem {
        let color = if self.0.is_enabled {
            Color::Debugger
        } else {
            Color::Muted
        };
        let is_enabled = self.0.is_enabled;
        let id = self.0.dap.data_id.clone();

        ListItem::new(SharedString::from(format!(
            "data-breakpoint-ui-item-{}",
            self.0.dap.data_id
        )))
        .toggle_state(is_selected)
        .inset(true)
        .start_slot(
            div()
                .id(SharedString::from(format!(
                    "data-breakpoint-ui-item-{}-click-handler",
                    self.0.dap.data_id
                )))
                .child(
                    Icon::new(IconName::Binary)
                        .color(color)
                        .size(IconSize::Small),
                )
                .tooltip({
                    let focus_handle = focus_handle.clone();
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            if is_enabled {
                                "Disable Data Breakpoint"
                            } else {
                                "Enable Data Breakpoint"
                            },
                            &ToggleEnableBreakpoint,
                            &focus_handle,
                            cx,
                        )
                    }
                })
                .on_click({
                    let list = list.clone();
                    move |_, _, cx| {
                        list.update(cx, |this, cx| {
                            this.toggle_data_breakpoint(&id, cx);
                        })
                        .ok();
                    }
                }),
        )
        .child(
            h_flex()
                .w_full()
                .gap_1()
                .min_h(rems_from_px(26.))
                .justify_between()
                .child(
                    v_flex()
                        .py_1()
                        .gap_1()
                        .justify_center()
                        .id(("data-breakpoint-label", ix))
                        .child(
                            Label::new(self.0.context.human_readable_label())
                                .size(LabelSize::Small)
                                .line_height_style(ui::LineHeightStyle::UiLabel),
                        ),
                )
                .child(BreakpointOptionsStrip {
                    props,
                    breakpoint: BreakpointEntry {
                        kind: BreakpointEntryKind::DataBreakpoint(self.clone()),
                        weak: list,
                    },
                    is_selected,
                    focus_handle,
                    strip_mode,
                    index: ix,
                }),
        )
    }
}

impl ExceptionBreakpoint {
    fn render(
        &mut self,
        props: SupportedBreakpointProperties,
        strip_mode: Option<ActiveBreakpointStripMode>,
        ix: usize,
        is_selected: bool,
        focus_handle: FocusHandle,
        list: WeakEntity<BreakpointList>,
    ) -> ListItem {
        let color = if self.is_enabled {
            Color::Debugger
        } else {
            Color::Muted
        };
        let id = SharedString::from(&self.id);
        let is_enabled = self.is_enabled;
        let weak = list.clone();

        ListItem::new(SharedString::from(format!(
            "exception-breakpoint-ui-item-{}",
            self.id
        )))
        .toggle_state(is_selected)
        .inset(true)
        .on_click({
            let list = list.clone();
            move |_, window, cx| {
                list.update(cx, |list, cx| list.select_ix(Some(ix), window, cx))
                    .ok();
            }
        })
        .on_secondary_mouse_down(|_, _, cx| {
            cx.stop_propagation();
        })
        .start_slot(
            div()
                .id(SharedString::from(format!(
                    "exception-breakpoint-ui-item-{}-click-handler",
                    self.id
                )))
                .child(
                    Icon::new(IconName::Flame)
                        .color(color)
                        .size(IconSize::Small),
                )
                .tooltip({
                    let focus_handle = focus_handle.clone();
                    move |_window, cx| {
                        Tooltip::for_action_in(
                            if is_enabled {
                                "Disable Exception Breakpoint"
                            } else {
                                "Enable Exception Breakpoint"
                            },
                            &ToggleEnableBreakpoint,
                            &focus_handle,
                            cx,
                        )
                    }
                })
                .on_click({
                    move |_, _, cx| {
                        list.update(cx, |this, cx| {
                            this.toggle_exception_breakpoint(&id, cx);
                        })
                        .ok();
                    }
                }),
        )
        .child(
            h_flex()
                .w_full()
                .gap_1()
                .min_h(rems_from_px(26.))
                .justify_between()
                .child(
                    v_flex()
                        .py_1()
                        .gap_1()
                        .justify_center()
                        .id(("exception-breakpoint-label", ix))
                        .child(
                            Label::new(self.data.label.clone())
                                .size(LabelSize::Small)
                                .line_height_style(ui::LineHeightStyle::UiLabel),
                        )
                        .when_some(self.data.description.clone(), |el, description| {
                            el.tooltip(Tooltip::text(description))
                        }),
                )
                .child(BreakpointOptionsStrip {
                    props,
                    breakpoint: BreakpointEntry {
                        kind: BreakpointEntryKind::ExceptionBreakpoint(self.clone()),
                        weak,
                    },
                    is_selected,
                    focus_handle,
                    strip_mode,
                    index: ix,
                }),
        )
    }
}
#[derive(Clone, Debug)]
enum BreakpointEntryKind {
    LineBreakpoint(LineBreakpoint),
    ExceptionBreakpoint(ExceptionBreakpoint),
    DataBreakpoint(DataBreakpoint),
}

#[derive(Clone, Debug)]
struct BreakpointEntry {
    kind: BreakpointEntryKind,
    weak: WeakEntity<BreakpointList>,
}

impl BreakpointEntry {
    fn render(
        &mut self,
        strip_mode: Option<ActiveBreakpointStripMode>,
        props: SupportedBreakpointProperties,
        ix: usize,
        is_selected: bool,
        focus_handle: FocusHandle,
    ) -> ListItem {
        match &mut self.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => line_breakpoint.render(
                props,
                strip_mode,
                ix,
                is_selected,
                focus_handle,
                self.weak.clone(),
            ),
            BreakpointEntryKind::ExceptionBreakpoint(exception_breakpoint) => exception_breakpoint
                .render(
                    props.for_exception_breakpoints(),
                    strip_mode,
                    ix,
                    is_selected,
                    focus_handle,
                    self.weak.clone(),
                ),
            BreakpointEntryKind::DataBreakpoint(data_breakpoint) => data_breakpoint.render(
                props.for_data_breakpoints(),
                strip_mode,
                ix,
                is_selected,
                focus_handle,
                self.weak.clone(),
            ),
        }
    }

    fn id(&self) -> SharedString {
        match &self.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => format!(
                "source-breakpoint-control-strip-{:?}:{}",
                line_breakpoint.breakpoint.path, line_breakpoint.breakpoint.row
            )
            .into(),
            BreakpointEntryKind::ExceptionBreakpoint(exception_breakpoint) => format!(
                "exception-breakpoint-control-strip--{}",
                exception_breakpoint.id
            )
            .into(),
            BreakpointEntryKind::DataBreakpoint(data_breakpoint) => format!(
                "data-breakpoint-control-strip--{}",
                data_breakpoint.0.dap.data_id
            )
            .into(),
        }
    }

    fn has_log(&self) -> bool {
        match &self.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                line_breakpoint.breakpoint.message.is_some()
            }
            _ => false,
        }
    }

    fn has_condition(&self) -> bool {
        match &self.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                line_breakpoint.breakpoint.condition.is_some()
            }
            // We don't support conditions on exception/data breakpoints
            _ => false,
        }
    }

    fn has_hit_condition(&self) -> bool {
        match &self.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                line_breakpoint.breakpoint.hit_condition.is_some()
            }
            _ => false,
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    pub struct SupportedBreakpointProperties: u32 {
        const LOG = 1 << 0;
        const CONDITION = 1 << 1;
        const HIT_CONDITION = 1 << 2;
        // Conditions for exceptions can be set only when exception filters are supported.
        const EXCEPTION_FILTER_OPTIONS = 1 << 3;
    }
}

impl From<&Capabilities> for SupportedBreakpointProperties {
    fn from(caps: &Capabilities) -> Self {
        let mut this = Self::empty();
        for (prop, offset) in [
            (caps.supports_log_points, Self::LOG),
            (caps.supports_conditional_breakpoints, Self::CONDITION),
            (
                caps.supports_hit_conditional_breakpoints,
                Self::HIT_CONDITION,
            ),
            (
                caps.supports_exception_options,
                Self::EXCEPTION_FILTER_OPTIONS,
            ),
        ] {
            if prop.unwrap_or_default() {
                this.insert(offset);
            }
        }
        this
    }
}

impl SupportedBreakpointProperties {
    fn for_exception_breakpoints(self) -> Self {
        // TODO: we don't yet support conditions for exception breakpoints at the data layer, hence all props are disabled here.
        Self::empty()
    }
    fn for_data_breakpoints(self) -> Self {
        // TODO: we don't yet support conditions for data breakpoints at the data layer, hence all props are disabled here.
        Self::empty()
    }
}
#[derive(IntoElement)]
struct BreakpointOptionsStrip {
    props: SupportedBreakpointProperties,
    breakpoint: BreakpointEntry,
    is_selected: bool,
    focus_handle: FocusHandle,
    strip_mode: Option<ActiveBreakpointStripMode>,
    index: usize,
}

impl BreakpointOptionsStrip {
    fn is_toggled(&self, expected_mode: ActiveBreakpointStripMode) -> bool {
        self.is_selected && self.strip_mode == Some(expected_mode)
    }

    fn on_click_callback(
        &self,
        mode: ActiveBreakpointStripMode,
    ) -> impl for<'a> Fn(&ClickEvent, &mut Window, &'a mut App) + use<> {
        let list = self.breakpoint.weak.clone();
        let ix = self.index;
        move |_, window, cx| {
            list.update(cx, |this, cx| {
                if this.strip_mode != Some(mode) {
                    this.set_active_breakpoint_property(mode, window, cx);
                } else if this.selected_ix == Some(ix) {
                    this.strip_mode.take();
                } else {
                    cx.propagate();
                }
            })
            .ok();
        }
    }

    fn add_focus_styles(
        &self,
        kind: ActiveBreakpointStripMode,
        available: bool,
        window: &Window,
        cx: &App,
    ) -> impl Fn(Div) -> Div {
        move |this: Div| {
            // Avoid layout shifts in case there's no colored border
            let this = this.border_1().rounded_sm();
            let color = cx.theme().colors();

            if self.is_selected && self.strip_mode == Some(kind) {
                if self.focus_handle.is_focused(window) {
                    this.bg(color.editor_background)
                        .border_color(color.border_focused)
                } else {
                    this.border_color(color.border)
                }
            } else if !available {
                this.border_color(color.border_transparent)
            } else {
                this
            }
        }
    }
}

impl RenderOnce for BreakpointOptionsStrip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = self.breakpoint.id();
        let supports_logs = self.props.contains(SupportedBreakpointProperties::LOG);
        let supports_condition = self
            .props
            .contains(SupportedBreakpointProperties::CONDITION);
        let supports_hit_condition = self
            .props
            .contains(SupportedBreakpointProperties::HIT_CONDITION);
        let has_logs = self.breakpoint.has_log();
        let has_condition = self.breakpoint.has_condition();
        let has_hit_condition = self.breakpoint.has_hit_condition();
        let style_for_toggle = |mode, is_enabled| {
            if is_enabled && self.strip_mode == Some(mode) && self.is_selected {
                ui::ButtonStyle::Filled
            } else {
                ui::ButtonStyle::Subtle
            }
        };
        let color_for_toggle = |is_enabled| {
            if is_enabled {
                Color::Default
            } else {
                Color::Muted
            }
        };

        h_flex()
            .gap_px()
            .justify_end()
            .when(has_logs || self.is_selected, |this| {
                this.child(
                    div()
                    .map(self.add_focus_styles(
                        ActiveBreakpointStripMode::Log,
                        supports_logs,
                        window,
                        cx,
                    ))
                    .child(
                        IconButton::new(
                            SharedString::from(format!("{id}-log-toggle")),
                            IconName::Notepad,
                        )
                        .shape(ui::IconButtonShape::Square)
                        .style(style_for_toggle(ActiveBreakpointStripMode::Log, has_logs))
                        .icon_size(IconSize::Small)
                        .icon_color(color_for_toggle(has_logs))
                        .when(has_logs, |this| this.indicator(Indicator::dot().color(Color::Info)))
                        .disabled(!supports_logs)
                        .toggle_state(self.is_toggled(ActiveBreakpointStripMode::Log))
                        .on_click(self.on_click_callback(ActiveBreakpointStripMode::Log))
                        .tooltip(|_window, cx|  {
                            Tooltip::with_meta(
                                "Set Log Message",
                                None,
                                "Set log message to display (instead of stopping) when a breakpoint is hit.",
                                cx,
                            )
                        }),
                    )
                )
            })
            .when(has_condition || self.is_selected, |this| {
                this.child(
                    div()
                        .map(self.add_focus_styles(
                            ActiveBreakpointStripMode::Condition,
                            supports_condition,
                            window,
                            cx,
                        ))
                        .child(
                            IconButton::new(
                                SharedString::from(format!("{id}-condition-toggle")),
                                IconName::SplitAlt,
                            )
                            .shape(ui::IconButtonShape::Square)
                            .style(style_for_toggle(
                                ActiveBreakpointStripMode::Condition,
                                has_condition,
                            ))
                            .icon_size(IconSize::Small)
                            .icon_color(color_for_toggle(has_condition))
                            .when(has_condition, |this| this.indicator(Indicator::dot().color(Color::Info)))
                            .disabled(!supports_condition)
                            .toggle_state(self.is_toggled(ActiveBreakpointStripMode::Condition))
                            .on_click(self.on_click_callback(ActiveBreakpointStripMode::Condition))
                            .tooltip(|_window, cx|  {
                                Tooltip::with_meta(
                                    "Set Condition",
                                    None,
                                    "Set condition to evaluate when a breakpoint is hit. Program execution will stop only when the condition is met.",
                                    cx,
                                )
                            }),
                        )
                )
            })
            .when(has_hit_condition || self.is_selected, |this| {
                this.child(div()
                    .map(self.add_focus_styles(
                        ActiveBreakpointStripMode::HitCondition,
                        supports_hit_condition,
                        window,
                        cx,
                    ))
                    .child(
                        IconButton::new(
                            SharedString::from(format!("{id}-hit-condition-toggle")),
                            IconName::ArrowDown10,
                        )
                        .style(style_for_toggle(
                            ActiveBreakpointStripMode::HitCondition,
                            has_hit_condition,
                        ))
                        .shape(ui::IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .icon_color(color_for_toggle(has_hit_condition))
                        .when(has_hit_condition, |this| this.indicator(Indicator::dot().color(Color::Info)))
                        .disabled(!supports_hit_condition)
                        .toggle_state(self.is_toggled(ActiveBreakpointStripMode::HitCondition))
                        .on_click(self.on_click_callback(ActiveBreakpointStripMode::HitCondition))
                        .tooltip(|_window, cx|  {
                            Tooltip::with_meta(
                                "Set Hit Condition",
                                None,
                                "Set expression that controls how many hits of the breakpoint are ignored.",
                                cx,
                            )
                        }),
                    ))

            })
    }
}
