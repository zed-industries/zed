use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use dap::ExceptionBreakpointsFilter;
use editor::Editor;
use gpui::{
    AppContext, Entity, FocusHandle, Focusable, ListState, MouseButton, ScrollHandle,
    ScrollStrategy, Stateful, Task, UniformListScrollHandle, WeakEntity, list, uniform_list,
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
    ActiveTheme as _, App, Clickable, Color, Context, Div, FluentBuilder as _, Icon, IconButton,
    IconName, Indicator, InteractiveElement, IntoElement, Label, LabelCommon, LabelSize, ListItem,
    ParentElement, Render, RenderOnce, Scrollbar, ScrollbarState, SharedString,
    StatefulInteractiveElement, Styled, Toggleable, Tooltip, Window, div, h_flex, px, v_flex,
};
use util::{ResultExt, maybe};
use workspace::Workspace;

pub(crate) struct BreakpointList {
    workspace: WeakEntity<Workspace>,
    breakpoint_store: Entity<BreakpointStore>,
    worktree_store: Entity<WorktreeStore>,
    // list_state: ListState,
    scrollbar_state: ScrollbarState,
    breakpoints: Vec<BreakpointEntry>,
    session: Entity<Session>,
    hide_scrollbar_task: Option<Task<()>>,
    show_scrollbar: bool,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    selected_ix: Option<usize>,
}

impl Focusable for BreakpointList {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl BreakpointList {
    pub(super) fn new(
        session: Entity<Session>,
        workspace: WeakEntity<Workspace>,
        project: &Entity<Project>,
        cx: &mut App,
    ) -> Entity<Self> {
        let project = project.read(cx);
        let breakpoint_store = project.breakpoint_store();
        let worktree_store = project.worktree_store();
        let focus_handle = cx.focus_handle();
        let scroll_handle = UniformListScrollHandle::new();
        let scrollbar_state = ScrollbarState::new(scroll_handle.clone());

        cx.new(|cx| {
            // let weak: gpui::WeakEntity<Self> = cx.weak_entity();
            // let list_state = ListState::new(
            //     0,
            //     gpui::ListAlignment::Top,
            //     px(1000.),
            //     move |ix, window, cx| {
            //         let Ok(Some(breakpoint)) =
            //             weak.update(cx, |this, _| this.breakpoints.get(ix).cloned())
            //         else {
            //             return div().into_any_element();
            //         };

            //         breakpoint.render(window, cx).into_any_element()
            //     },
            // );
            Self {
                breakpoint_store,
                worktree_store,
                scrollbar_state,
                // list_state,
                breakpoints: Default::default(),
                hide_scrollbar_task: None,
                show_scrollbar: false,
                workspace,
                session,
                focus_handle,
                scroll_handle,
                selected_ix: None,
            }
        })
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
        self.select_ix(ix, cx);
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
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
        self.select_ix(ix, cx);
    }

    fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ix = if self.breakpoints.len() > 0 {
            Some(0)
        } else {
            None
        };
        self.select_ix(ix, cx);
    }

    fn select_last(&mut self, _: &menu::SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let ix = if self.breakpoints.len() > 0 {
            Some(self.breakpoints.len() - 1)
        } else {
            None
        };
        self.select_ix(ix, cx);
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

    fn render_list(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_ix = self.selected_ix;
        uniform_list(
            cx.entity(),
            "breakpoint-list",
            self.breakpoints.len(),
            move |this, range, window, cx| {
                range
                    .clone()
                    .zip(&mut this.breakpoints[range])
                    .map(|(ix, breakpoint)| {
                        breakpoint
                            .render(window, cx)
                            .toggle_state(Some(ix) == selected_ix)
                            .into_any_element()
                    })
                    .collect()
            },
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
}
impl Render for BreakpointList {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        // let old_len = self.breakpoints.len();
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
        let exception_breakpoints =
            self.session
                .read(cx)
                .exception_breakpoints()
                .map(|(data, is_enabled)| BreakpointEntry {
                    kind: BreakpointEntryKind::ExceptionBreakpoint(ExceptionBreakpoint {
                        id: data.filter.clone(),
                        data: data.clone(),
                        is_enabled: *is_enabled,
                    }),
                    weak: weak.clone(),
                });
        self.breakpoints
            .extend(exception_breakpoints.chain(breakpoints));
        // if self.breakpoints.len() != old_len {
        //     self.list_state.reset(self.breakpoints.len());
        // }
        v_flex()
            .id("breakpoint-list")
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
            .size_full()
            .m_0p5()
            .child(self.render_list(window, cx))
            .children(self.render_vertical_scrollbar(cx))
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
    fn render(&mut self, weak: WeakEntity<BreakpointList>) -> ListItem {
        let icon_name = if self.breakpoint.state.is_enabled() {
            IconName::DebugBreakpoint
        } else {
            IconName::DebugDisabledBreakpoint
        };
        let path = self.breakpoint.path.clone();
        let row = self.breakpoint.row;
        let indicator = div()
            .id(SharedString::from(format!(
                "breakpoint-ui-toggle-{:?}/{}:{}",
                self.dir, self.name, self.line
            )))
            .cursor_pointer()
            .tooltip(Tooltip::text(if self.breakpoint.state.is_enabled() {
                "Disable Breakpoint"
            } else {
                "Enable Breakpoint"
            }))
            .on_click({
                let weak = weak.clone();
                let path = path.clone();
                move |_, _, cx| {
                    weak.update(cx, |this, cx| {
                        this.breakpoint_store.update(cx, |this, cx| {
                            if let Some((buffer, breakpoint)) =
                                this.breakpoint_at_row(&path, row, cx)
                            {
                                this.toggle_breakpoint(
                                    buffer,
                                    breakpoint,
                                    BreakpointEditAction::InvertState,
                                    cx,
                                );
                            } else {
                                log::error!("Couldn't find breakpoint at row event though it exists: row {row}")
                            }
                        })
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
        .start_slot(indicator)
        .rounded()
        .on_secondary_mouse_down(|_, _, cx| {
            cx.stop_propagation();
        })
        .end_hover_slot(
            IconButton::new(
                SharedString::from(format!(
                    "breakpoint-ui-on-click-go-to-line-remove-{:?}/{}:{}",
                    self.dir, self.name, self.line
                )),
                IconName::Close,
            )
            .on_click({
                let weak = weak.clone();
                let path = path.clone();
                move |_, _, cx| {
                    weak.update(cx, |this, cx| {
                        this.breakpoint_store.update(cx, |this, cx| {
                            if let Some((buffer, breakpoint)) =
                                this.breakpoint_at_row(&path, row, cx)
                            {
                                this.toggle_breakpoint(
                                    buffer,
                                    breakpoint,
                                    BreakpointEditAction::Toggle,
                                    cx,
                                );
                            } else {
                                log::error!("Couldn't find breakpoint at row event though it exists: row {row}")
                            }
                        })
                    })
                    .ok();
                }
            })
            .icon_size(ui::IconSize::XSmall),
        )
        .child(
            v_flex()
                .id(SharedString::from(format!(
                    "breakpoint-ui-on-click-go-to-line-{:?}/{}:{}",
                    self.dir, self.name, self.line
                )))
                .on_click(move |_, window, cx| {
                    let path = path.clone();
                    let weak = weak.clone();
                    maybe!({
                        let task = weak
                            .update(cx, |this, cx| {
                                this.worktree_store.update(cx, |this, cx| {
                                    this.find_or_create_worktree(path, false, cx)
                                })
                            })
                            .ok()?;
                        window
                            .spawn(cx, async move |cx| {
                                let (worktree, relative_path) = task.await?;
                                let worktree_id = worktree.update(cx, |this, _| this.id())?;
                                let item = weak
                                    .update_in(cx, |this, window, cx| {
                                        this.workspace.update(cx, |this, cx| {
                                            this.open_path(
                                                (worktree_id, relative_path),
                                                None,
                                                true,
                                                window,
                                                cx,
                                            )
                                        })
                                    })??
                                    .await?;
                                if let Some(editor) = item.downcast::<Editor>() {
                                    editor
                                        .update_in(cx, |this, window, cx| {
                                            this.go_to_singleton_buffer_point(
                                                Point { row, column: 0 },
                                                window,
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                                anyhow::Ok(())
                            })
                            .detach();

                        Some(())
                    });
                })
                .cursor_pointer()
                .py_1()
                .items_center()
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Label::new(format!("{}:{}", self.name, self.line))
                                .size(LabelSize::Small)
                                .line_height_style(ui::LineHeightStyle::UiLabel),
                        )
                        .children(self.dir.clone().map(|dir| {
                            Label::new(dir)
                                .color(Color::Muted)
                                .size(LabelSize::Small)
                                .line_height_style(ui::LineHeightStyle::UiLabel)
                        })),
                )
                // .child(
                //     Label::new(line)
                //         .size(LabelSize::XSmall)
                //         .color(Color::Muted)
                //         .line_height_style(ui::LineHeightStyle::UiLabel),
                // ),
        )
    }
}
#[derive(Clone, Debug)]
struct ExceptionBreakpoint {
    id: String,
    data: ExceptionBreakpointsFilter,
    is_enabled: bool,
}

impl ExceptionBreakpoint {
    fn render(&mut self, list: WeakEntity<BreakpointList>) -> ListItem {
        let color = if self.is_enabled {
            Color::Debugger
        } else {
            Color::Muted
        };
        let id = SharedString::from(&self.id);
        ListItem::new(SharedString::from(format!(
            "exception-breakpoint-ui-item-{}",
            self.id
        )))
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
                .tooltip(Tooltip::text(if self.is_enabled {
                    "Disable Exception Breakpoint"
                } else {
                    "Enable Exception Breakpoint"
                }))
                .on_click(move |_, _, cx| {
                    list.update(cx, |this, cx| {
                        this.session.update(cx, |this, cx| {
                            this.toggle_exception_breakpoint(&id, cx);
                        });
                        cx.notify();
                    })
                    .ok();
                })
                .cursor_pointer()
                .child(Indicator::icon(Icon::new(IconName::Flame)).color(color)),
        )
        .child(
            div()
                .py_1()
                .gap_1()
                .child(
                    Label::new(self.data.label.clone())
                        .size(LabelSize::Small)
                        .line_height_style(ui::LineHeightStyle::UiLabel),
                )
                .children(self.data.description.clone().map(|description| {
                    Label::new(description)
                        .size(LabelSize::XSmall)
                        .line_height_style(ui::LineHeightStyle::UiLabel)
                        .color(Color::Muted)
                })),
        )
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
    fn render(&mut self, _: &mut Window, _: &mut App) -> ListItem {
        match &mut self.kind {
            BreakpointEntryKind::LineBreakpoint(line_breakpoint) => {
                line_breakpoint.render(self.weak.clone())
            }
            BreakpointEntryKind::ExceptionBreakpoint(exception_breakpoint) => {
                exception_breakpoint.render(self.weak.clone())
            }
        }
    }
}
