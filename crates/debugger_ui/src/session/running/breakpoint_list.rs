use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use dap::{Capabilities, ExceptionBreakpointsFilter};
use editor::Editor;
use gpui::{
    Action, AppContext, ClickEvent, Entity, FocusHandle, Focusable, MouseButton, ScrollStrategy,
    Stateful, Task, UniformListScrollHandle, WeakEntity, actions, uniform_list,
};
use language::Point;
use project::{
    Project,
    debugger::{
        breakpoint_store::{BreakpointEditAction, BreakpointStore, SourceBreakpoint},
        session::Session,
    },
    worktree_store::WorktreeStore,
};
use ui::{
    ActiveTheme, AnyElement, App, ButtonCommon, Clickable, Color, Context, Disableable, Div,
    Divider, FluentBuilder as _, Icon, IconButton, IconName, IconSize, Indicator,
    InteractiveElement, IntoElement, Label, LabelCommon, LabelSize, ListItem, ParentElement,
    Render, RenderOnce, Scrollbar, ScrollbarState, SharedString, StatefulInteractiveElement,
    Styled, Toggleable, Tooltip, Window, div, h_flex, px, v_flex,
};
use util::ResultExt;
use workspace::Workspace;
use zed_actions::{ToggleEnableBreakpoint, UnsetBreakpoint};

actions!(
    debugger,
    [PreviousBreakpointProperty, NextBreakpointProperty]
);
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum SelectedBreakpointKind {
    Source,
    Exception,
}
pub(crate) struct BreakpointList {
    workspace: WeakEntity<Workspace>,
    breakpoint_store: Entity<BreakpointStore>,
    worktree_store: Entity<WorktreeStore>,
    scrollbar_state: ScrollbarState,
    breakpoints: Vec<BreakpointEntry>,
    session: Option<Entity<Session>>,
    hide_scrollbar_task: Option<Task<()>>,
    show_scrollbar: bool,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    selected_ix: Option<usize>,
    input: Entity<Editor>,
    strip_mode: Option<ActiveBreakpointStripMode>,
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
        let focus_handle = cx.focus_handle();
        let scroll_handle = UniformListScrollHandle::new();
        let scrollbar_state = ScrollbarState::new(scroll_handle.clone());

        cx.new(|cx| Self {
            breakpoint_store,
            worktree_store,
            scrollbar_state,
            breakpoints: Default::default(),
            hide_scrollbar_task: None,
            show_scrollbar: false,
            workspace,
            session,
            focus_handle,
            scroll_handle,
            selected_ix: None,
            input: cx.new(|cx| Editor::single_line(window, cx)),
            strip_mode: None,
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
            let worktree_id = worktree.read_with(cx, |this, _| this.id())?;
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
            this.set_placeholder_text(placeholder, cx);
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
        if self.strip_mode.is_some() {
            if self.input.focus_handle(cx).contains_focused(window, cx) {
                cx.propagate();
                return;
            }
        }
        let ix = match self.selected_ix {
            _ if self.breakpoints.len() == 0 => None,
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
        if self.strip_mode.is_some() {
            if self.input.focus_handle(cx).contains_focused(window, cx) {
                cx.propagate();
                return;
            }
        }
        let ix = match self.selected_ix {
            _ if self.breakpoints.len() == 0 => None,
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
        if self.strip_mode.is_some() {
            if self.input.focus_handle(cx).contains_focused(window, cx) {
                cx.propagate();
                return;
            }
        }
        let ix = if self.breakpoints.len() > 0 {
            Some(0)
        } else {
            None
        };
        self.select_ix(ix, window, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, window: &mut Window, cx: &mut Context<Self>) {
        if self.strip_mode.is_some() {
            if self.input.focus_handle(cx).contains_focused(window, cx) {
                cx.propagate();
                return;
            }
        }
        let ix = if self.breakpoints.len() > 0 {
            Some(self.breakpoints.len() - 1)
        } else {
            None
        };
        self.select_ix(ix, window, cx);
    }

    fn dismiss(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.input.focus_handle(cx).contains_focused(window, cx) {
            self.focus_handle.focus(window);
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
                    ActiveBreakpointStripMode::Log => match &entry.kind {
                        BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                            Self::edit_line_breakpoint_inner(
                                &self.breakpoint_store,
                                line_breakpoint.breakpoint.path.clone(),
                                line_breakpoint.breakpoint.row,
                                BreakpointEditAction::EditLogMessage(Arc::from(text)),
                                cx,
                            );
                        }
                        _ => {}
                    },
                    ActiveBreakpointStripMode::Condition => match &entry.kind {
                        BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                            Self::edit_line_breakpoint_inner(
                                &self.breakpoint_store,
                                line_breakpoint.breakpoint.path.clone(),
                                line_breakpoint.breakpoint.row,
                                BreakpointEditAction::EditCondition(Arc::from(text)),
                                cx,
                            );
                        }
                        _ => {}
                    },
                    ActiveBreakpointStripMode::HitCondition => match &entry.kind {
                        BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                            Self::edit_line_breakpoint_inner(
                                &self.breakpoint_store,
                                line_breakpoint.breakpoint.path.clone(),
                                line_breakpoint.breakpoint.row,
                                BreakpointEditAction::EditHitCondition(Arc::from(text)),
                                cx,
                            );
                        }
                        _ => {}
                    },
                }
                self.focus_handle.focus(window);
            } else {
                handle.focus(window);
            }

            return;
        }
        match &mut entry.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                let path = line_breakpoint.breakpoint.path.clone();
                let row = line_breakpoint.breakpoint.row;
                self.go_to_line_breakpoint(path, row, window, cx);
            }
            BreakpointEntryKind::ExceptionBreakpoint(_) => {}
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
        if self.strip_mode.is_some() {
            if self.input.focus_handle(cx).contains_focused(window, cx) {
                cx.propagate();
                return;
            }
        }

        match &mut entry.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                let path = line_breakpoint.breakpoint.path.clone();
                let row = line_breakpoint.breakpoint.row;
                self.edit_line_breakpoint(path, row, BreakpointEditAction::InvertState, cx);
            }
            BreakpointEntryKind::ExceptionBreakpoint(exception_breakpoint) => {
                if let Some(session) = &self.session {
                    let id = exception_breakpoint.id.clone();
                    session.update(cx, |session, cx| {
                        session.toggle_exception_breakpoint(&id, cx);
                    });
                }
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

        match &mut entry.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                let path = line_breakpoint.breakpoint.path.clone();
                let row = line_breakpoint.breakpoint.row;
                self.edit_line_breakpoint(path, row, BreakpointEditAction::Toggle, cx);
            }
            BreakpointEntryKind::ExceptionBreakpoint(_) => {}
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

    fn hide_scrollbar(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        const SCROLLBAR_SHOW_INTERVAL: Duration = Duration::from_secs(1);
        self.hide_scrollbar_task = Some(cx.spawn_in(window, async move |panel, cx| {
            cx.background_executor()
                .timer(SCROLLBAR_SHOW_INTERVAL)
                .await;
            panel
                .update(cx, |panel, cx| {
                    panel.show_scrollbar = false;
                    cx.notify();
                })
                .log_err();
        }))
    }

    fn render_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_ix = self.selected_ix;
        let focus_handle = self.focus_handle.clone();
        let supported_breakpoint_properties = self
            .session
            .as_ref()
            .map(|session| SupportedBreakpointProperties::from(session.read(cx).capabilities()))
            .unwrap_or_else(SupportedBreakpointProperties::empty);
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
        .track_scroll(self.scroll_handle.clone())
        .flex_grow()
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Option<Stateful<Div>> {
        if !(self.show_scrollbar || self.scrollbar_state.is_dragging()) {
            return None;
        }
        Some(
            div()
                .occlude()
                .id("breakpoint-list-vertical-scrollbar")
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
                .children(Scrollbar::vertical(self.scrollbar_state.clone())),
        )
    }
    pub(crate) fn render_control_strip(&self) -> AnyElement {
        let selection_kind = self.selection_kind();
        let focus_handle = self.focus_handle.clone();
        let remove_breakpoint_tooltip = selection_kind.map(|(kind, _)| match kind {
            SelectedBreakpointKind::Source => "Remove breakpoint from a breakpoint list",
            SelectedBreakpointKind::Exception => {
                "Exception Breakpoints cannot be removed from the breakpoint list"
            }
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
            .gap_2()
            .child(
                IconButton::new(
                    "disable-breakpoint-breakpoint-list",
                    IconName::DebugDisabledBreakpoint,
                )
                .icon_size(IconSize::XSmall)
                .when_some(toggle_label, |this, (label, meta)| {
                    this.tooltip({
                        let focus_handle = focus_handle.clone();
                        move |window, cx| {
                            Tooltip::with_meta_in(
                                label,
                                Some(&ToggleEnableBreakpoint),
                                meta,
                                &focus_handle,
                                window,
                                cx,
                            )
                        }
                    })
                })
                .disabled(selection_kind.is_none())
                .on_click({
                    let focus_handle = focus_handle.clone();
                    move |_, window, cx| {
                        focus_handle.focus(window);
                        window.dispatch_action(ToggleEnableBreakpoint.boxed_clone(), cx)
                    }
                }),
            )
            .child(
                IconButton::new("remove-breakpoint-breakpoint-list", IconName::X)
                    .icon_size(IconSize::XSmall)
                    .icon_color(ui::Color::Error)
                    .when_some(remove_breakpoint_tooltip, |this, tooltip| {
                        this.tooltip({
                            let focus_handle = focus_handle.clone();
                            move |window, cx| {
                                Tooltip::with_meta_in(
                                    "Remove Breakpoint",
                                    Some(&UnsetBreakpoint),
                                    tooltip,
                                    &focus_handle,
                                    window,
                                    cx,
                                )
                            }
                        })
                    })
                    .disabled(
                        selection_kind.map(|kind| kind.0) != Some(SelectedBreakpointKind::Source),
                    )
                    .on_click({
                        let focus_handle = focus_handle.clone();
                        move |_, window, cx| {
                            focus_handle.focus(window);
                            window.dispatch_action(UnsetBreakpoint.boxed_clone(), cx)
                        }
                    }),
            )
            .mr_2()
            .into_any_element()
    }
}

impl Render for BreakpointList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let breakpoints = self.breakpoint_store.read(cx).all_source_breakpoints(cx);
        self.breakpoints.clear();
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
                        .then(|| Path::new(worktree.read(cx).root_name()).join(relative_path))
                });
            breakpoints.sort_by_key(|breakpoint| breakpoint.row);
            let weak = weak.clone();
            breakpoints.into_iter().filter_map(move |breakpoint| {
                debug_assert_eq!(&path, &breakpoint.path);
                let file_name = breakpoint.path.file_name()?;

                let dir = relative_worktree_path
                    .clone()
                    .unwrap_or_else(|| PathBuf::from(&*breakpoint.path))
                    .parent()
                    .and_then(|parent| {
                        parent
                            .to_str()
                            .map(ToOwned::to_owned)
                            .map(SharedString::from)
                    });
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
        self.breakpoints
            .extend(breakpoints.chain(exception_breakpoints));
        v_flex()
            .id("breakpoint-list")
            .key_context("BreakpointList")
            .track_focus(&self.focus_handle)
            .on_hover(cx.listener(|this, hovered, window, cx| {
                if *hovered {
                    this.show_scrollbar = true;
                    this.hide_scrollbar_task.take();
                    cx.notify();
                } else if !this.focus_handle.contains_focused(window, cx) {
                    this.hide_scrollbar(window, cx);
                }
            }))
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
            .m_0p5()
            .child(
                v_flex()
                    .size_full()
                    .child(self.render_list(cx))
                    .children(self.render_vertical_scrollbar(cx)),
            )
            .when_some(self.strip_mode, |this, _| {
                this.child(Divider::horizontal()).child(
                    h_flex()
                        // .w_full()
                        .m_0p5()
                        .p_0p5()
                        .border_1()
                        .rounded_sm()
                        .when(
                            self.input.focus_handle(cx).contains_focused(window, cx),
                            |this| {
                                let colors = cx.theme().colors();
                                let border = if self.input.read(cx).read_only(cx) {
                                    colors.border_disabled
                                } else {
                                    colors.border_focused
                                };
                                this.border_color(border)
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
            .cursor_pointer()
            .tooltip({
                let focus_handle = focus_handle.clone();
                move |window, cx| {
                    Tooltip::for_action_in(
                        if is_enabled {
                            "Disable Breakpoint"
                        } else {
                            "Enable Breakpoint"
                        },
                        &ToggleEnableBreakpoint,
                        &focus_handle,
                        window,
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
            .child(Indicator::icon(Icon::new(icon_name)).color(Color::Debugger))
            .on_mouse_down(MouseButton::Left, move |_, _, _| {});

        ListItem::new(SharedString::from(format!(
            "breakpoint-ui-item-{:?}/{}:{}",
            self.dir, self.name, self.line
        )))
        .on_click({
            let weak = weak.clone();
            move |_, window, cx| {
                weak.update(cx, |breakpoint_list, cx| {
                    breakpoint_list.select_ix(Some(ix), window, cx);
                })
                .ok();
            }
        })
        .start_slot(indicator)
        .rounded()
        .on_secondary_mouse_down(|_, _, cx| {
            cx.stop_propagation();
        })
        .child(
            h_flex()
                .w_full()
                .mr_4()
                .py_0p5()
                .gap_1()
                .min_h(px(26.))
                .justify_between()
                .id(SharedString::from(format!(
                    "breakpoint-ui-on-click-go-to-line-{:?}/{}:{}",
                    self.dir, self.name, self.line
                )))
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
                .cursor_pointer()
                .child(
                    Label::new(format!("{}:{}", self.name, self.line))
                        .size(LabelSize::Small)
                        .line_height_style(ui::LineHeightStyle::UiLabel),
                )
                .when_some(self.dir.as_ref(), |this, parent_dir| {
                    this.tooltip(Tooltip::text(format!("Worktree parent path: {parent_dir}")))
                })
                .child(BreakpointOptionsStrip {
                    props,
                    breakpoint: BreakpointEntry {
                        kind: BreakpointEntryKind::LineBreakpoint(self.clone()),
                        weak: weak,
                    },
                    is_selected,
                    focus_handle,
                    strip_mode,
                    index: ix,
                }),
        )
        .toggle_state(is_selected)
    }
}
#[derive(Clone, Debug)]
struct ExceptionBreakpoint {
    id: String,
    data: ExceptionBreakpointsFilter,
    is_enabled: bool,
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
        .on_click({
            let list = list.clone();
            move |_, window, cx| {
                list.update(cx, |list, cx| list.select_ix(Some(ix), window, cx))
                    .ok();
            }
        })
        .rounded()
        .on_secondary_mouse_down(|_, _, cx| {
            cx.stop_propagation();
        })
        .start_slot(
            div()
                .id(SharedString::from(format!(
                    "exception-breakpoint-ui-item-{}-click-handler",
                    self.id
                )))
                .tooltip({
                    let focus_handle = focus_handle.clone();
                    move |window, cx| {
                        Tooltip::for_action_in(
                            if is_enabled {
                                "Disable Exception Breakpoint"
                            } else {
                                "Enable Exception Breakpoint"
                            },
                            &ToggleEnableBreakpoint,
                            &focus_handle,
                            window,
                            cx,
                        )
                    }
                })
                .on_click({
                    let list = list.clone();
                    move |_, _, cx| {
                        list.update(cx, |this, cx| {
                            if let Some(session) = &this.session {
                                session.update(cx, |this, cx| {
                                    this.toggle_exception_breakpoint(&id, cx);
                                });
                                cx.notify();
                            }
                        })
                        .ok();
                    }
                })
                .cursor_pointer()
                .child(Indicator::icon(Icon::new(IconName::Flame)).color(color)),
        )
        .child(
            h_flex()
                .w_full()
                .mr_4()
                .py_0p5()
                .justify_between()
                .child(
                    v_flex()
                        .py_1()
                        .gap_1()
                        .min_h(px(26.))
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
                        weak: weak,
                    },
                    is_selected,
                    focus_handle,
                    strip_mode,
                    index: ix,
                }),
        )
        .toggle_state(is_selected)
    }
}
#[derive(Clone, Debug)]
enum BreakpointEntryKind {
    LineBreakpoint(LineBreakpoint),
    ExceptionBreakpoint(ExceptionBreakpoint),
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
            // We don't support conditions on exception breakpoints
            BreakpointEntryKind::ExceptionBreakpoint(_) => false,
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
    fn add_border(
        &self,
        kind: ActiveBreakpointStripMode,
        available: bool,
        window: &Window,
        cx: &App,
    ) -> impl Fn(Div) -> Div {
        move |this: Div| {
            // Avoid layout shifts in case there's no colored border
            let this = this.border_2().rounded_sm();
            if self.is_selected && self.strip_mode == Some(kind) {
                let theme = cx.theme().colors();
                if self.focus_handle.is_focused(window) {
                    this.border_color(theme.border_selected)
                } else {
                    this.border_color(theme.border_disabled)
                }
            } else if !available {
                this.border_color(cx.theme().colors().border_disabled)
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
                ui::Color::Default
            } else {
                ui::Color::Muted
            }
        };

        h_flex()
            .gap_2()
            .child(
                div() .map(self.add_border(ActiveBreakpointStripMode::Log, supports_logs, window, cx))
                    .child(
                        IconButton::new(
                            SharedString::from(format!("{id}-log-toggle")),
                            IconName::ScrollText,
                        )
                        .style(style_for_toggle(ActiveBreakpointStripMode::Log, has_logs))
                        .icon_color(color_for_toggle(has_logs))
                        .disabled(!supports_logs)
                        .toggle_state(self.is_toggled(ActiveBreakpointStripMode::Log))
                        .on_click(self.on_click_callback(ActiveBreakpointStripMode::Log)).tooltip(|window, cx| Tooltip::with_meta("Set Log Message", None, "Set log message to display (instead of stopping) when a breakpoint is hit", window, cx))
                    )
                    .when(!has_logs && !self.is_selected, |this| this.invisible()),
            )
            .child(
                div().map(self.add_border(
                    ActiveBreakpointStripMode::Condition,
                    supports_condition,
                    window, cx
                ))
                    .child(
                        IconButton::new(
                            SharedString::from(format!("{id}-condition-toggle")),
                            IconName::SplitAlt,
                        )
                        .style(style_for_toggle(
                            ActiveBreakpointStripMode::Condition,
                            has_condition
                        ))
                        .icon_color(color_for_toggle(has_condition))
                        .disabled(!supports_condition)
                        .toggle_state(self.is_toggled(ActiveBreakpointStripMode::Condition))
                        .on_click(self.on_click_callback(ActiveBreakpointStripMode::Condition))
                        .tooltip(|window, cx| Tooltip::with_meta("Set Condition", None, "Set condition to evaluate when a breakpoint is hit. Program execution will stop only when the condition is met", window, cx))
                    )
                    .when(!has_condition && !self.is_selected, |this| this.invisible()),
            )
            .child(
                div()                  .map(self.add_border(
                    ActiveBreakpointStripMode::HitCondition,
                    supports_hit_condition,window, cx
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
                        .icon_color(color_for_toggle(has_hit_condition))
                        .disabled(!supports_hit_condition)
                        .toggle_state(self.is_toggled(ActiveBreakpointStripMode::HitCondition))
                        .on_click(self.on_click_callback(ActiveBreakpointStripMode::HitCondition)).tooltip(|window, cx| Tooltip::with_meta("Set Hit Condition", None, "Set expression that controls how many hits of the breakpoint are ignored.", window, cx))
                    )
                    .when(!has_hit_condition && !self.is_selected, |this| {
                        this.invisible()
                    }),
            )
    }
}
