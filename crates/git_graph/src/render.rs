//! GPUI rendering for git graph visualization

use crate::graph::{GitGraph, RefType};
use crate::layout::GraphLayout;
use gpui::{
    div, prelude::*, App, Context, EventEmitter, FocusHandle, Focusable,
    Render, SharedString, Window,
};
use ui::prelude::*;

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
    focus_handle: FocusHandle,
}

impl GitGraphView {
    pub fn new(graph: GitGraph, cx: &mut Context<Self>) -> Self {
        let layout = GraphLayout::from_graph(&graph);
        Self {
            graph,
            layout,
            selected_commit: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn update_graph(&mut self, graph: GitGraph, cx: &mut Context<Self>) {
        self.layout = GraphLayout::from_graph(&graph);
        self.graph = graph;
        self.selected_commit = None;
        cx.notify();
    }

    pub fn select_commit(&mut self, sha: SharedString, cx: &mut Context<Self>) {
        self.selected_commit = Some(sha.clone());
        cx.emit(GitGraphEvent::CommitSelected(sha));
        cx.notify();
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
            )
        });

        let sha_for_click = sha.clone();

        div()
            .id(SharedString::from(format!("commit-{}", row)))
            .h(px(self.layout.row_height))
            .w_full()
            .flex()
            .items_center()
            .cursor_pointer()
            .when(is_selected, |el| {
                el.bg(cx.theme().colors().element_selected)
            })
            .hover(|el| el.bg(cx.theme().colors().element_hover))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.select_commit(sha_for_click.clone(), cx);
            }))
            .child(
                // Graph lines area
                div()
                    .w(px(self.layout.graph_width() + 8.0))
                    .h_full()
                    .flex()
                    .items_center()
                    .child(self.render_graph_node(row, commit_info.as_ref().map(|(_, _, _, lane, _)| *lane).unwrap_or(0), cx))
            )
            .child(
                // Commit info
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .gap_2()
                    .children(commit_info.map(|(short_sha, subject, author, _lane, refs)| {
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                // Short SHA
                                Label::new(short_sha)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
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
                                // Subject
                                Label::new(subject.to_string())
                                    .size(LabelSize::Small)
                                    .color(Color::Default)
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("git-graph-view")
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .track_focus(&self.focus_handle)
            .child(
                // Header
                div()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(Label::new("Git Graph").size(LabelSize::Default).weight(gpui::FontWeight::MEDIUM))
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
