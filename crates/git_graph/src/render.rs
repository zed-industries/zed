//! GPUI rendering for git graph visualization

use crate::graph::{GitGraph, RefType};
use crate::layout::GraphLayout;
use gpui::{
    actions, div, prelude::*, App, Context, EventEmitter, FocusHandle, Focusable,
    KeyContext, Render, SharedString, Window,
};
use menu::{Confirm, SelectNext, SelectPrevious};
use ui::prelude::*;

actions!(git_graph_view, [CopyCommitSha, ShowCommitDetails]);

/// Branch colors for graph lines (based on VS Code Git Graph)
pub const BRANCH_COLORS: [u32; 8] = [
    0x00FF7F, // Spring green (main)
    0x1E90FF, // Dodger blue
    0xFFD700, // Gold
    0xFF69B4, // Hot pink
    0x00CED1, // Dark turquoise
    0xFF6347, // Tomato
    0x9370DB, // Medium purple
    0x32CD32, // Lime green
];

/// Get color for a branch lane
pub fn lane_color(lane: usize) -> gpui::Hsla {
    let rgb = BRANCH_COLORS[lane % BRANCH_COLORS.len()];
    let r = ((rgb >> 16) & 0xFF) as f32 / 255.0;
    let g = ((rgb >> 8) & 0xFF) as f32 / 255.0;
    let b = (rgb & 0xFF) as f32 / 255.0;

    // Convert RGB to HSL
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    let (h, s) = if (max - min).abs() < f32::EPSILON {
        (0.0, 0.0)
    } else {
        let d = max - min;
        let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
        let h = if (max - r).abs() < f32::EPSILON {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if (max - g).abs() < f32::EPSILON {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };
        (h / 6.0, s)
    };

    gpui::hsla(h, s, l, 1.0)
}

/// Events emitted by GitGraphView
pub enum GitGraphEvent {
    CommitSelected(SharedString),
    CommitActivated(SharedString),
}

/// Git Graph View panel
pub struct GitGraphView {
    graph: GitGraph,
    layout: GraphLayout,
    selected_commit: Option<SharedString>,
    selected_index: Option<usize>,
    focus_handle: FocusHandle,
}

impl GitGraphView {
    pub fn new(graph: GitGraph, cx: &mut Context<Self>) -> Self {
        let layout = GraphLayout::from_graph(&graph);
        Self {
            graph,
            layout,
            selected_commit: None,
            selected_index: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn update_graph(&mut self, graph: GitGraph, cx: &mut Context<Self>) {
        self.layout = GraphLayout::from_graph(&graph);
        self.graph = graph;
        self.selected_commit = None;
        self.selected_index = None;
        cx.notify();
    }

    pub fn select_commit(&mut self, sha: SharedString, cx: &mut Context<Self>) {
        // Find index for this commit
        self.selected_index = self.graph.ordered_commits.iter().position(|s| s == &sha);
        self.selected_commit = Some(sha.clone());
        cx.emit(GitGraphEvent::CommitSelected(sha));
        cx.notify();
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let count = self.graph.ordered_commits.len();
        if count == 0 {
            return;
        }
        let new_index = match self.selected_index {
            Some(ix) => (ix + 1).min(count - 1),
            None => 0,
        };
        self.selected_index = Some(new_index);
        if let Some(sha) = self.graph.ordered_commits.get(new_index) {
            self.selected_commit = Some(sha.clone());
            cx.emit(GitGraphEvent::CommitSelected(sha.clone()));
        }
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, _window: &mut Window, cx: &mut Context<Self>) {
        let count = self.graph.ordered_commits.len();
        if count == 0 {
            return;
        }
        let new_index = match self.selected_index {
            Some(ix) => ix.saturating_sub(1),
            None => count - 1,
        };
        self.selected_index = Some(new_index);
        if let Some(sha) = self.graph.ordered_commits.get(new_index) {
            self.selected_commit = Some(sha.clone());
            cx.emit(GitGraphEvent::CommitSelected(sha.clone()));
        }
        cx.notify();
    }

    fn confirm(&mut self, _: &Confirm, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(sha) = &self.selected_commit {
            cx.emit(GitGraphEvent::CommitActivated(sha.clone()));
        }
    }

    fn copy_commit_sha(&mut self, _: &CopyCommitSha, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(sha) = &self.selected_commit {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(sha.to_string()));
        }
    }

    fn dispatch_context(&self, _window: &Window, _cx: &Context<Self>) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();
        context.add("GitGraphView");
        context.add("menu");
        context
    }

    fn render_commit_row(&self, row: usize, sha: &SharedString, cx: &Context<Self>) -> impl IntoElement {
        let commit = self.graph.commits.get(sha);
        let is_selected = self.selected_commit.as_ref() == Some(sha);

        let commit_info = commit.map(|c| {
            (
                c.short_sha.clone(),
                c.subject.clone(),
                c.author_name.clone(),
                c.lane,
                c.refs.clone(),
                c.timestamp,
            )
        });

        let sha_for_click = sha.clone();
        let sha_for_double_click = sha.clone();

        div()
            .id(SharedString::from(format!("commit-{}", row)))
            .h(px(self.layout.row_height))
            .w_full()
            .flex()
            .items_center()
            .cursor_pointer()
            .when(is_selected, |el| {
                el.bg(cx.theme().colors().element_selected)
                    .border_l_2()
                    .border_color(cx.theme().colors().border_focused)
            })
            .hover(|el| el.bg(cx.theme().colors().element_hover))
            .on_click(cx.listener(move |this, event: &gpui::ClickEvent, _, cx| {
                this.select_commit(sha_for_click.clone(), cx);
                // Double-click activates the commit
                if event.click_count() >= 2 {
                    cx.emit(GitGraphEvent::CommitActivated(sha_for_double_click.clone()));
                }
            }))
            .child(
                // Graph lines area
                div()
                    .w(px(self.layout.graph_width() + 8.0))
                    .h_full()
                    .flex()
                    .items_center()
                    .child(self.render_graph_node(row, commit_info.as_ref().map(|(_, _, _, lane, _, _)| *lane).unwrap_or(0), cx))
            )
            .child(
                // Commit info
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .gap_2()
                    .overflow_hidden()
                    .children(commit_info.map(|(short_sha, subject, author, _lane, refs, _timestamp)| {
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .overflow_hidden()
                            .child(
                                // Short SHA with monospace font
                                div()
                                    .px_1()
                                    .py_0p5()
                                    .rounded_sm()
                                    .bg(cx.theme().colors().surface_background)
                                    .child(
                                        Label::new(short_sha)
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted)
                                    )
                            )
                            // Refs (branches/tags)
                            .children(refs.iter().take(3).map(|r| {
                                let (color, bg_color) = match r.ref_type {
                                    RefType::Head => (Color::Accent, cx.theme().colors().element_selected),
                                    RefType::LocalBranch => (Color::Success, cx.theme().status().success_background),
                                    RefType::RemoteBranch => (Color::Warning, cx.theme().status().warning_background),
                                    RefType::Tag => (Color::Info, cx.theme().status().info_background),
                                };
                                div()
                                    .px_1()
                                    .py_0p5()
                                    .rounded_sm()
                                    .bg(bg_color)
                                    .child(
                                        Label::new(r.short_name().to_string())
                                            .size(LabelSize::XSmall)
                                            .color(color)
                                    )
                            }))
                            .child(
                                // Subject (truncated)
                                div()
                                    .flex_1()
                                    .overflow_hidden()
                                    .child(
                                        Label::new(subject.to_string())
                                            .size(LabelSize::Small)
                                            .color(Color::Default)
                                    )
                            )
                            .child(
                                // Author
                                Label::new(author.to_string())
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted)
                            )
                    }))
            )
    }

    fn render_graph_node(&self, _row: usize, lane: usize, _cx: &Context<Self>) -> impl IntoElement {
        let x = self.layout.lane_x(lane);
        let color = lane_color(lane);

        div()
            .w(px(self.layout.graph_width()))
            .h_full()
            .flex()
            .items_center()
            .child(
                div()
                    .absolute()
                    .left(px(x - 4.0))
                    .w_2()
                    .h_2()
                    .rounded_full()
                    .bg(color)
            )
    }
}

impl Focusable for GitGraphView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<GitGraphEvent> for GitGraphView {}

impl Render for GitGraphView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("git-graph-view")
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .track_focus(&self.focus_handle)
            .key_context(self.dispatch_context(window, cx))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::copy_commit_sha))
            .child(
                // Header
                div()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(Label::new("Git Graph").size(LabelSize::Default).weight(gpui::FontWeight::MEDIUM))
                    .child(
                        div()
                            .flex()
                            .gap_1()
                            .child(
                                Label::new(format!("{} commits", self.graph.ordered_commits.len()))
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                    )
            )
            .child(
                // Graph content
                div()
                    .id("git-graph-content")
                    .flex_1()
                    .overflow_y_scroll()
                    .children(
                        self.graph.ordered_commits.iter().enumerate().map(|(row, sha)| {
                            self.render_commit_row(row, sha, cx).into_any_element()
                        })
                    )
            )
    }
}
