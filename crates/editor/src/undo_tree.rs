use super::*;
use gpui::{InteractiveElement as _, StatefulInteractiveElement as _};
use std::time::SystemTime;
use text::{UndoNodeId, UndoTreeSnapshot};

#[derive(Clone, Debug)]
pub(crate) struct UndoTreeVisualizerState {
    pub(crate) available: bool,
    pub(crate) current: Option<UndoNodeId>,
    pub(crate) selected: Option<UndoNodeId>,
    pub(crate) can_switch_branch: bool,
    pub(crate) nodes: Vec<UndoTreeVisualizerNode>,
}

#[derive(Clone, Debug)]
pub(crate) struct UndoTreeVisualizerNode {
    pub(crate) id: UndoNodeId,
    pub(crate) depth: usize,
    pub(crate) ordinal: usize,
    pub(crate) label: SharedString,
    pub(crate) timestamp: Option<SharedString>,
    pub(crate) current: bool,
    pub(crate) selected: bool,
    pub(crate) active_branch: bool,
    pub(crate) branch_head: bool,
    pub(crate) saved: bool,
    pub(crate) latest_saved: bool,
    pub(crate) branch_point: bool,
}

impl UndoTreeVisualizerNode {
    fn badges(&self) -> Vec<(&'static str, Color)> {
        let mut badges = Vec::new();
        if self.current {
            badges.push(("current", Color::Accent));
        }
        if self.latest_saved {
            badges.push(("latest", Color::Success));
        } else if self.saved {
            badges.push(("saved", Color::Success));
        }
        if self.branch_point {
            badges.push(("branch", Color::Info));
        } else if self.branch_head {
            badges.push(("head", Color::Muted));
        }
        if self.active_branch {
            badges.push(("active", Color::Modified));
        }
        badges
    }
}

impl Editor {
    pub fn undo_tree_visible(&self) -> bool {
        self.show_undo_tree
    }

    pub fn selected_undo_node(&self) -> Option<UndoNodeId> {
        self.selected_undo_node
    }

    pub fn undo_tree_snapshot(&self, cx: &App) -> Option<UndoTreeSnapshot> {
        let buffer = self.buffer.read(cx).as_singleton()?;
        Some(buffer.read(cx).undo_tree_snapshot())
    }

    pub fn show_undo_tree(&mut self, _: &ShowUndoTree, _: &mut Window, cx: &mut Context<Self>) {
        self.show_undo_tree = true;
        self.selected_undo_node = self.undo_tree_snapshot(cx).map(|snapshot| snapshot.current);
        cx.emit(EditorEvent::UndoHistoryChanged);
        cx.notify();
    }

    pub fn hide_undo_tree(&mut self, _: &HideUndoTree, _: &mut Window, cx: &mut Context<Self>) {
        if self.show_undo_tree {
            self.show_undo_tree = false;
            cx.emit(EditorEvent::UndoHistoryChanged);
            cx.notify();
        }
    }

    pub fn toggle_undo_tree(
        &mut self,
        _: &ToggleUndoTree,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.show_undo_tree {
            self.hide_undo_tree(&HideUndoTree, window, cx);
        } else {
            self.show_undo_tree(&ShowUndoTree, window, cx);
        }
    }

    pub fn undo_tree_select_previous(
        &mut self,
        _: &UndoTreeSelectPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_undo_tree_node_relative(-1, cx);
    }

    pub fn undo_tree_select_next(
        &mut self,
        _: &UndoTreeSelectNext,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_undo_tree_node_relative(1, cx);
    }

    pub fn undo_tree_switch_branch_previous(
        &mut self,
        _: &UndoTreeSwitchBranchPrevious,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.switch_selected_undo_branch(-1, cx);
    }

    pub fn undo_tree_switch_branch_next(
        &mut self,
        _: &UndoTreeSwitchBranchNext,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.switch_selected_undo_branch(1, cx);
    }

    pub fn undo_tree_jump_to_selected(
        &mut self,
        _: &UndoTreeJumpToSelected,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(target) = self.ensure_selected_undo_node(cx) {
            self.jump_to_undo_node(target, window, cx);
        }
    }

    pub fn undo_tree_jump_to_latest(
        &mut self,
        _: &UndoTreeJumpToLatest,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(snapshot) = self.undo_tree_snapshot(cx) {
            let target = Self::latest_undo_tree_node(&snapshot);
            self.selected_undo_node = Some(target);
            self.jump_to_undo_node(target, window, cx);
        }
    }

    pub fn undo_tree_jump_to_latest_saved(
        &mut self,
        _: &UndoTreeJumpToLatestSaved,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(target) = self
            .undo_tree_snapshot(cx)
            .and_then(|snapshot| snapshot.latest_saved)
        {
            self.selected_undo_node = Some(target);
            self.jump_to_undo_node(target, window, cx);
        }
    }

    pub fn jump_to_undo_node(
        &mut self,
        target: UndoNodeId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.read_only(cx) {
            return false;
        }

        let Some(snapshot) = self.undo_tree_snapshot(cx) else {
            return false;
        };
        let target_transaction_id = snapshot
            .nodes
            .iter()
            .find(|node| node.id == target)
            .and_then(|node| node.transaction_id);

        let jumped = self.buffer.update(cx, |multi_buffer, cx| {
            let Some(buffer) = multi_buffer.as_singleton() else {
                return false;
            };
            buffer.update(cx, |buffer, cx| buffer.jump_to_undo_node(target, cx))
        });

        if jumped {
            self.selected_undo_node = Some(target);
            self.restore_selection_for_undo_tree_target(target_transaction_id, window, cx);
            self.request_autoscroll(Autoscroll::fit(), cx);
            self.unmark_text(window, cx);
            self.refresh_edit_prediction(true, false, window, cx);
            if let Some(transaction_id) = target_transaction_id {
                cx.emit(EditorEvent::Edited { transaction_id });
            }
            cx.emit(EditorEvent::UndoHistoryChanged);
            cx.notify();
        }

        jumped
    }

    pub(crate) fn refresh_undo_tree_visualizer(
        &mut self,
        follow_current: bool,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.show_undo_tree {
            return false;
        }

        if let Some(snapshot) = self.undo_tree_snapshot(cx) {
            let selected_is_valid = self
                .selected_undo_node
                .is_some_and(|selected| snapshot.nodes.iter().any(|node| node.id == selected));
            if follow_current || !selected_is_valid {
                self.selected_undo_node = Some(snapshot.current);
            }
        } else {
            self.selected_undo_node = None;
        }

        cx.emit(EditorEvent::UndoHistoryChanged);
        cx.notify();
        true
    }

    fn select_undo_tree_node_relative(&mut self, direction: isize, cx: &mut Context<Self>) {
        let Some(snapshot) = self.undo_tree_snapshot(cx) else {
            return;
        };
        let ordered_nodes = Self::undo_tree_node_order(&snapshot);
        if ordered_nodes.is_empty() {
            return;
        }

        let selected = self
            .selected_undo_node
            .filter(|node_id| ordered_nodes.contains(node_id))
            .unwrap_or(snapshot.current);
        let selected_index = ordered_nodes
            .iter()
            .position(|node_id| *node_id == selected)
            .unwrap_or(0);
        let new_index = if direction.is_negative() {
            selected_index.saturating_sub(1)
        } else {
            (selected_index + 1).min(ordered_nodes.len().saturating_sub(1))
        };

        self.selected_undo_node = ordered_nodes.get(new_index).copied();
        cx.emit(EditorEvent::UndoHistoryChanged);
        cx.notify();
    }

    fn switch_selected_undo_branch(&mut self, direction: isize, cx: &mut Context<Self>) {
        if self.read_only(cx) {
            return;
        }

        let Some(snapshot) = self.undo_tree_snapshot(cx) else {
            return;
        };
        let selected = self.selected_undo_node.unwrap_or(snapshot.current);
        let nodes_by_id = snapshot
            .nodes
            .iter()
            .map(|node| (node.id, node))
            .collect::<HashMap<_, _>>();

        let Some(parent_id) = nodes_by_id.get(&selected).and_then(|selected_node| {
            if selected_node.children.is_empty() {
                selected_node.parent
            } else {
                Some(selected_node.id)
            }
        }) else {
            return;
        };
        let Some(parent) = nodes_by_id.get(&parent_id) else {
            return;
        };
        if parent.children.is_empty() {
            return;
        }

        let active_child = parent
            .active_child
            .filter(|index| *index < parent.children.len())
            .unwrap_or(0);
        let new_child = if direction.is_negative() {
            active_child
                .checked_sub(1)
                .unwrap_or_else(|| parent.children.len().saturating_sub(1))
        } else {
            (active_child + 1) % parent.children.len()
        };
        if new_child == active_child {
            return;
        }

        let switched = self.buffer.update(cx, |multi_buffer, cx| {
            let Some(buffer) = multi_buffer.as_singleton() else {
                return false;
            };
            buffer.update(cx, |buffer, cx| {
                buffer.switch_undo_branch(parent_id, new_child, cx)
            })
        });
        if switched {
            self.selected_undo_node = parent.children.get(new_child).copied();
            cx.emit(EditorEvent::UndoHistoryChanged);
            cx.notify();
        }
    }

    fn ensure_selected_undo_node(&mut self, cx: &App) -> Option<UndoNodeId> {
        let snapshot = self.undo_tree_snapshot(cx)?;
        let selected = self
            .selected_undo_node
            .filter(|node_id| snapshot.nodes.iter().any(|node| node.id == *node_id))
            .unwrap_or(snapshot.current);
        self.selected_undo_node = Some(selected);
        Some(selected)
    }

    fn latest_undo_tree_node(snapshot: &UndoTreeSnapshot) -> UndoNodeId {
        snapshot
            .nodes
            .iter()
            .rev()
            .find(|node| node.transaction_id.is_some())
            .map(|node| node.id)
            .unwrap_or(snapshot.root)
    }

    fn undo_tree_node_order(snapshot: &UndoTreeSnapshot) -> Vec<UndoNodeId> {
        let nodes_by_id = snapshot
            .nodes
            .iter()
            .map(|node| (node.id, node))
            .collect::<HashMap<_, _>>();
        let mut ordered_nodes = Vec::new();
        Self::push_undo_tree_node(snapshot.root, &nodes_by_id, &mut ordered_nodes);
        ordered_nodes
    }

    fn push_undo_tree_node(
        node_id: UndoNodeId,
        nodes_by_id: &HashMap<UndoNodeId, &text::UndoTreeNodeSnapshot>,
        ordered_nodes: &mut Vec<UndoNodeId>,
    ) {
        if ordered_nodes.contains(&node_id) {
            return;
        }
        ordered_nodes.push(node_id);
        if let Some(node) = nodes_by_id.get(&node_id) {
            for child in &node.children {
                Self::push_undo_tree_node(*child, nodes_by_id, ordered_nodes);
            }
        }
    }

    fn restore_selection_for_undo_tree_target(
        &mut self,
        target_transaction_id: Option<TransactionId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(transaction_id) = target_transaction_id {
            if let Some((_, Some(selections))) =
                self.selection_history.transaction(transaction_id).cloned()
            {
                self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_anchors(selections.to_vec());
                });
                return;
            }

            if let Some(offset) = self.first_edited_offset_for_transaction(transaction_id, cx) {
                self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.select_ranges([offset..offset]);
                });
                return;
            }
        }

        self.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(0)..MultiBufferOffset(0)]);
        });
    }

    fn first_edited_offset_for_transaction(
        &self,
        transaction_id: TransactionId,
        cx: &App,
    ) -> Option<MultiBufferOffset> {
        let buffer = self.buffer.read(cx).as_singleton()?;
        let text_anchor = buffer.read_with(cx, |buffer, _| {
            let snapshot = buffer.snapshot();
            let range = buffer
                .edited_ranges_for_transaction_id::<usize>(transaction_id)
                .next()?;
            Some(snapshot.anchor_at(range.start, Bias::Left))
        })?;
        let snapshot = self.buffer.read(cx).read(cx);
        let anchor = snapshot.anchor_in_excerpt(text_anchor)?;
        Some(anchor.to_offset(&snapshot))
    }

    pub(crate) fn undo_tree_visualizer_state(&self, cx: &App) -> UndoTreeVisualizerState {
        let Some(snapshot) = self.undo_tree_snapshot(cx) else {
            return UndoTreeVisualizerState {
                available: false,
                current: None,
                selected: None,
                can_switch_branch: false,
                nodes: Vec::new(),
            };
        };

        let selected = self
            .selected_undo_node
            .filter(|node_id| snapshot.nodes.iter().any(|node| node.id == *node_id))
            .unwrap_or(snapshot.current);
        let nodes_by_id = snapshot
            .nodes
            .iter()
            .map(|node| (node.id, node))
            .collect::<HashMap<_, _>>();
        let mut nodes = Vec::new();
        let mut visited = HashSet::default();
        Self::push_undo_tree_visualizer_node(
            snapshot.root,
            0,
            snapshot.current,
            selected,
            &nodes_by_id,
            &mut visited,
            &mut nodes,
        );

        UndoTreeVisualizerState {
            available: true,
            current: Some(snapshot.current),
            selected: Some(selected),
            can_switch_branch: snapshot.nodes.iter().any(|node| node.children.len() > 1),
            nodes,
        }
    }

    pub(crate) fn render_undo_tree_visualizer(
        &mut self,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.show_undo_tree {
            return None;
        }

        let state = self.undo_tree_visualizer_state(cx);
        let read_only = self.read_only(cx);
        let can_jump = state.available && !read_only;
        let can_switch_branch = state.can_switch_branch && !read_only;
        let latest_saved_enabled = state.nodes.iter().any(|node| node.latest_saved) && !read_only;

        Some(
            WithRemSize::new(ThemeSettings::get_global(cx).ui_font_size(cx))
                .w_full()
                .max_w(px(320.))
                .max_h(px(420.))
                .flex()
                .flex_col()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border)
                .elevation_2(cx)
                .overflow_hidden()
                .child(
                    h_flex()
                        .h_8()
                        .justify_between()
                        .gap_2()
                        .border_b_1()
                        .border_color(cx.theme().colors().border_variant)
                        .px_2()
                        .child(
                            h_flex()
                                .min_w_0()
                                .gap_1()
                                .child(
                                    IconButton::new("undo-tree-title-icon", IconName::GitBranch)
                                        .disabled(true)
                                        .icon_color(Color::Muted)
                                        .icon_size(IconSize::Small)
                                        .shape(IconButtonShape::Square),
                                )
                                .child(
                                    Label::new("Undo Tree")
                                        .size(LabelSize::Small)
                                        .weight(FontWeight::BOLD)
                                        .single_line(),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_0p5()
                                .child(
                                    IconButton::new(
                                        "undo-tree-branch-previous",
                                        IconName::ChevronLeft,
                                    )
                                    .shape(IconButtonShape::Square)
                                    .size(ButtonSize::Compact)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .disabled(!can_switch_branch)
                                    .tooltip(|_, cx| {
                                        cx.new(|_| Tooltip::new("Previous Branch")).into()
                                    })
                                    .on_click(cx.listener(
                                        |editor, _, window, cx| {
                                            editor.undo_tree_switch_branch_previous(
                                                &UndoTreeSwitchBranchPrevious,
                                                window,
                                                cx,
                                            );
                                        },
                                    )),
                                )
                                .child(
                                    IconButton::new(
                                        "undo-tree-branch-next",
                                        IconName::ChevronRight,
                                    )
                                    .shape(IconButtonShape::Square)
                                    .size(ButtonSize::Compact)
                                    .icon_size(IconSize::Small)
                                    .icon_color(Color::Muted)
                                    .disabled(!can_switch_branch)
                                    .tooltip(|_, cx| cx.new(|_| Tooltip::new("Next Branch")).into())
                                    .on_click(cx.listener(
                                        |editor, _, window, cx| {
                                            editor.undo_tree_switch_branch_next(
                                                &UndoTreeSwitchBranchNext,
                                                window,
                                                cx,
                                            );
                                        },
                                    )),
                                )
                                .child(
                                    IconButton::new("undo-tree-latest-saved", IconName::Check)
                                        .shape(IconButtonShape::Square)
                                        .size(ButtonSize::Compact)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .disabled(!latest_saved_enabled)
                                        .tooltip(|_, cx| {
                                            cx.new(|_| Tooltip::new("Jump to Latest Saved")).into()
                                        })
                                        .on_click(cx.listener(|editor, _, window, cx| {
                                            editor.undo_tree_jump_to_latest_saved(
                                                &UndoTreeJumpToLatestSaved,
                                                window,
                                                cx,
                                            );
                                        })),
                                )
                                .child(
                                    IconButton::new("undo-tree-latest", IconName::FastForward)
                                        .shape(IconButtonShape::Square)
                                        .size(ButtonSize::Compact)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .disabled(!can_jump)
                                        .tooltip(|_, cx| {
                                            cx.new(|_| Tooltip::new("Jump to Latest")).into()
                                        })
                                        .on_click(cx.listener(|editor, _, window, cx| {
                                            editor.undo_tree_jump_to_latest(
                                                &UndoTreeJumpToLatest,
                                                window,
                                                cx,
                                            );
                                        })),
                                )
                                .child(
                                    IconButton::new("undo-tree-close", IconName::Close)
                                        .shape(IconButtonShape::Square)
                                        .size(ButtonSize::Compact)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Muted)
                                        .tooltip(|_, cx| {
                                            cx.new(|_| Tooltip::new("Hide Undo Tree")).into()
                                        })
                                        .on_click(cx.listener(|editor, _, window, cx| {
                                            editor.hide_undo_tree(&HideUndoTree, window, cx);
                                        })),
                                ),
                        ),
                )
                .child(
                    div()
                        .id("undo-tree-body")
                        .max_h(px(388.))
                        .overflow_y_scroll()
                        .when(!state.available, |this| {
                            this.p_3().child(
                                Label::new("Undo tree is unavailable for multibuffers")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        })
                        .when(state.available && state.nodes.is_empty(), |this| {
                            this.p_3().child(
                                Label::new("No undo history")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        })
                        .when(state.available, |this| {
                            this.children(
                                state
                                    .nodes
                                    .into_iter()
                                    .map(|node| {
                                        Self::render_undo_tree_visualizer_node(node, read_only, cx)
                                    })
                                    .collect::<Vec<_>>(),
                            )
                        }),
                )
                .into_any_element(),
        )
    }

    fn render_undo_tree_visualizer_node(
        node: UndoTreeVisualizerNode,
        read_only: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let node_id = node.id;
        let indent = px(8. + node.depth as f32 * 14.);
        let marker = if node.current {
            ">"
        } else if node.selected {
            "*"
        } else if node.branch_head {
            "o"
        } else {
            "-"
        };

        h_flex()
            .id(("undo-tree-node", node.ordinal))
            .h_7()
            .min_w_0()
            .gap_1()
            .px_2()
            .cursor_pointer()
            .overflow_hidden()
            .when(node.selected, |this| {
                this.bg(cx.theme().colors().element_selected)
            })
            .when(!node.selected && node.current, |this| {
                this.bg(cx.theme().colors().element_hover)
            })
            .hover(|style| style.bg(cx.theme().colors().element_hover))
            .on_click(cx.listener(move |editor, _, window, cx| {
                editor.selected_undo_node = Some(node_id);
                editor.jump_to_undo_node(node_id, window, cx);
            }))
            .child(div().w(indent).flex_none())
            .child(
                Label::new(marker)
                    .size(LabelSize::XSmall)
                    .color(if node.current {
                        Color::Accent
                    } else {
                        Color::Muted
                    })
                    .line_height_style(LineHeightStyle::UiLabel)
                    .flex_none(),
            )
            .child(
                Label::new(node.label.clone())
                    .size(LabelSize::Small)
                    .color(if read_only {
                        Color::Disabled
                    } else {
                        Color::Default
                    })
                    .single_line()
                    .truncate()
                    .flex_1(),
            )
            .children(
                node.badges()
                    .into_iter()
                    .map(|(label, color)| Self::render_undo_tree_badge(label, color, cx))
                    .collect::<Vec<_>>(),
            )
            .when_some(node.timestamp, |this, timestamp| {
                this.child(
                    Label::new(timestamp)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                        .single_line()
                        .flex_none(),
                )
            })
            .into_any_element()
    }

    fn render_undo_tree_badge(label: &'static str, color: Color, cx: &App) -> AnyElement {
        div()
            .px_1()
            .py_0p5()
            .rounded_sm()
            .bg(color.color(cx).opacity(0.12))
            .child(
                Label::new(label)
                    .size(LabelSize::XSmall)
                    .line_height_style(LineHeightStyle::UiLabel)
                    .color(color),
            )
            .into_any_element()
    }

    fn push_undo_tree_visualizer_node(
        node_id: UndoNodeId,
        depth: usize,
        current: UndoNodeId,
        selected: UndoNodeId,
        nodes_by_id: &HashMap<UndoNodeId, &text::UndoTreeNodeSnapshot>,
        visited: &mut HashSet<UndoNodeId>,
        visualizer_nodes: &mut Vec<UndoTreeVisualizerNode>,
    ) {
        if !visited.insert(node_id) {
            return;
        }

        let Some(node) = nodes_by_id.get(&node_id) else {
            return;
        };
        let active_branch = node
            .parent
            .and_then(|parent_id| nodes_by_id.get(&parent_id))
            .and_then(|parent| {
                parent
                    .active_child
                    .and_then(|active_child| parent.children.get(active_child))
                    .copied()
            })
            .is_some_and(|active_child| active_child == node_id);
        let label = node
            .transaction_id
            .map(|transaction_id| SharedString::from(format!("#{}", transaction_id.value)))
            .unwrap_or_else(|| SharedString::from("Root"));
        let timestamp = node
            .last_edit_at
            .or(node.first_edit_at)
            .map(Self::format_undo_tree_timestamp);

        visualizer_nodes.push(UndoTreeVisualizerNode {
            id: node_id,
            depth,
            ordinal: visualizer_nodes.len(),
            label,
            timestamp,
            current: node_id == current,
            selected: node_id == selected,
            active_branch,
            branch_head: node.transaction_id.is_some() && node.children.is_empty(),
            saved: node.saved,
            latest_saved: node.latest_saved,
            branch_point: node.children.len() > 1,
        });

        for child in &node.children {
            Self::push_undo_tree_visualizer_node(
                *child,
                depth + 1,
                current,
                selected,
                nodes_by_id,
                visited,
                visualizer_nodes,
            );
        }
    }

    fn format_undo_tree_timestamp(timestamp: SystemTime) -> SharedString {
        let elapsed = SystemTime::now()
            .duration_since(timestamp)
            .unwrap_or(Duration::ZERO);
        if elapsed < Duration::from_secs(5) {
            "now".into()
        } else if elapsed < Duration::from_secs(60) {
            format!("{}s", elapsed.as_secs()).into()
        } else if elapsed < Duration::from_secs(60 * 60) {
            format!("{}m", elapsed.as_secs() / 60).into()
        } else if elapsed < Duration::from_secs(60 * 60 * 24) {
            format!("{}h", elapsed.as_secs() / 60 / 60).into()
        } else {
            format!("{}d", elapsed.as_secs() / 60 / 60 / 24).into()
        }
    }
}
