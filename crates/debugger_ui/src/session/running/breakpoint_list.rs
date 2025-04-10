use std::path::{Path, PathBuf};

use gpui::{AppContext, Entity, ListState, MouseButton, Stateful, list};
use project::{
    Project, debugger::breakpoint_store::BreakpointStore, worktree_store::WorktreeStore,
};
use ui::{
    App, Color, Context, Div, Indicator, InteractiveElement, IntoElement, Label, LabelCommon,
    LabelSize, ListItem, ParentElement, Render, RenderOnce, Scrollbar, ScrollbarState,
    SharedString, StatefulInteractiveElement, Styled, div, h_flex, px, v_flex,
};

pub(super) struct BreakpointList {
    breakpoint_store: Entity<BreakpointStore>,
    worktree_store: Entity<WorktreeStore>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    breakpoints: Vec<BreakpointEntry>,
}

impl BreakpointList {
    pub(super) fn new(project: &Entity<Project>, cx: &mut App) -> Entity<Self> {
        let project = project.read(cx);
        let breakpoint_store = project.breakpoint_store();
        let worktree_store = project.worktree_store();

        cx.new(|cx| {
            let weak: gpui::WeakEntity<Self> = cx.weak_entity();
            let list_state = ListState::new(
                0,
                gpui::ListAlignment::Top,
                px(1000.),
                move |ix, window, cx| {
                    let Ok(Some(breakpoint)) =
                        weak.update(cx, |this, _| this.breakpoints.get(ix).cloned())
                    else {
                        return div().into_any_element();
                    };

                    breakpoint.render(window, cx).into_any_element()
                },
            );
            Self {
                breakpoint_store,
                worktree_store,
                scrollbar_state: ScrollbarState::new(list_state.clone()),
                list_state,
                breakpoints: Default::default(),
            }
        })
    }

    fn render_vertical_scrollbar(&self, cx: &mut Context<Self>) -> Stateful<Div> {
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
            .children(Scrollbar::vertical(self.scrollbar_state.clone()))
    }
}
impl Render for BreakpointList {
    fn render(
        &mut self,
        _window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let breakpoints = self.breakpoint_store.read(cx).all_breakpoints(cx);
        let breakpoints = breakpoints
            .into_iter()
            .flat_map(|(path, mut breakpoints)| {
                let relative_worktree_path =
                    self.worktree_store
                        .read(cx)
                        .find_worktree(&path, cx)
                        .and_then(|(worktree, relative_path)| {
                            worktree.read(cx).is_visible().then(|| {
                                Path::new(worktree.read(cx).root_name()).join(relative_path)
                            })
                        });
                breakpoints.sort_by_key(|breakpoint| breakpoint.row);
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
                    let line = format!("Line {}", breakpoint.row + 1).into();
                    Some(BreakpointEntry::LineBreakpoint { name, dir, line })
                })
            })
            .collect::<Vec<_>>();
        if self.breakpoints.len() != breakpoints.len() {
            self.breakpoints = breakpoints;
            self.list_state.reset(self.breakpoints.len());
        }
        v_flex()
            .size_full()
            .child(list(self.list_state.clone()).flex_grow())
            .child(self.render_vertical_scrollbar(cx))
    }
}

#[derive(Clone, Debug)]
enum BreakpointEntry {
    LineBreakpoint {
        name: SharedString,
        dir: Option<SharedString>,
        line: SharedString,
    },
}

impl RenderOnce for BreakpointEntry {
    fn render(self, _: &mut ui::Window, _: &mut App) -> impl ui::IntoElement {
        let Self::LineBreakpoint { name, dir, line } = self;
        ListItem::new(SharedString::from(format!(
            "breakpoint-ui-item-{:?}/{}:{}",
            dir, name, line
        )))
        .start_slot(Indicator::dot().color(Color::Debugger))
        .rounded()
        .child(
            v_flex()
                .py_0p5()
                .items_center()
                .child(
                    h_flex()
                        .gap_1()
                        .child(Label::new(name).size(LabelSize::Small))
                        .children(
                            dir.map(|dir| {
                                Label::new(dir).color(Color::Muted).size(LabelSize::Small)
                            }),
                        ),
                )
                .child(
                    Label::new(line)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .line_height_style(ui::LineHeightStyle::UiLabel),
                ),
        )
    }
}
