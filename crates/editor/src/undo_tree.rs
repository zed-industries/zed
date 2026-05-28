use super::*;
use gpui::{
    DragMoveEvent, Empty, InteractiveElement as _, StatefulInteractiveElement as _, canvas, fill,
};
use std::rc::Rc;
use std::time::SystemTime;
use text::{UndoNodeId, UndoTreeSnapshot};

/// Default width of the visualizer popover before the user resizes it.
const UNDO_TREE_DEFAULT_WIDTH: Pixels = px(420.);
/// Height cap applied to the (content-driven) popover until the user resizes it.
const UNDO_TREE_DEFAULT_MAX_HEIGHT: Pixels = px(420.);
const UNDO_TREE_MIN_WIDTH: Pixels = px(240.);
const UNDO_TREE_MIN_HEIGHT: Pixels = px(160.);
/// Thickness of the draggable resize affordances along the popover edges.
const UNDO_TREE_RESIZE_HANDLE: Pixels = px(6.);

/// Which edge/corner of the popover a resize drag is acting on. The popover is
/// anchored to its top-right corner, so only the left edge, bottom edge, and
/// bottom-left corner are draggable.
#[derive(Clone, Copy, Debug)]
enum UndoTreeResizeEdge {
    Left,
    Bottom,
    BottomLeft,
}

#[derive(Clone)]
struct UndoTreeResizeDrag {
    edge: UndoTreeResizeEdge,
}

impl Render for UndoTreeResizeDrag {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

/// Horizontal distance between adjacent node columns in the graph.
const UNDO_TREE_COLUMN_WIDTH: Pixels = px(22.);
/// Vertical distance between depth levels in the graph.
const UNDO_TREE_ROW_HEIGHT: Pixels = px(28.);
/// Side length of the (square) clickable box drawn for each node glyph.
const UNDO_TREE_NODE_SIZE: Pixels = px(16.);
/// Diameter of the visible circular node within the clickable box.
const UNDO_TREE_NODE_GLYPH_SIZE: Pixels = px(10.);
/// Padding around the graph so edge nodes aren't clipped against the viewport.
const UNDO_TREE_GRAPH_PADDING: Pixels = px(10.);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UndoTreeNodeKind {
    Normal,
    Saved,
    Current,
}

impl UndoTreeNodeKind {
    fn color(self) -> Color {
        match self {
            UndoTreeNodeKind::Current => Color::Accent,
            UndoTreeNodeKind::Saved => Color::Success,
            UndoTreeNodeKind::Normal => Color::Muted,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UndoTreeVisualizerState {
    pub(crate) available: bool,
    pub(crate) nodes: Vec<UndoTreeVisualizerNode>,
    pub(crate) edges: Vec<UndoTreeVisualizerEdge>,
    pub(crate) row_timestamps: Vec<Option<SharedString>>,
    pub(crate) columns: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct UndoTreeVisualizerNode {
    pub(crate) id: UndoNodeId,
    /// Fractional column so that a parent can sit centered between its children.
    pub(crate) column: f32,
    pub(crate) row: usize,
    pub(crate) kind: UndoTreeNodeKind,
    pub(crate) selected: bool,
}

/// A parent → child link, with the column/row of each endpoint and whether it
/// lies on the path from the root to the selected node (used for line coloring).
#[derive(Clone, Debug)]
pub(crate) struct UndoTreeVisualizerEdge {
    pub(crate) from_column: f32,
    pub(crate) from_row: usize,
    pub(crate) to_column: f32,
    pub(crate) to_row: usize,
    pub(crate) active: bool,
}

/// Cached visualizer state plus the inputs it was built from. Reused across
/// frames while the undo tree and selection are unchanged, so a large history
/// isn't re-snapshotted and re-laid-out on every repaint.
pub(crate) struct UndoTreeVisualizerCache {
    version: Option<u64>,
    selected: Option<UndoNodeId>,
    state: Rc<UndoTreeVisualizerState>,
}

/// Vec indexed by `UndoNodeId::as_usize()` — cheaper than the previous
/// `HashMap<UndoNodeId, _>` we rebuilt every time the visualizer (or its
/// keyboard navigation) needed to chase a parent/child link. Ids stay dense in
/// the live tree because `History` reuses slots via its free-list, so the
/// table size tracks the live node count and not arbitrary lifetime growth.
struct UndoNodeLookup<'a> {
    table: Vec<Option<&'a text::UndoTreeNodeSnapshot>>,
}

impl<'a> UndoNodeLookup<'a> {
    fn new(snapshot: &'a UndoTreeSnapshot) -> Self {
        let max_index = snapshot
            .nodes
            .iter()
            .map(|node| node.id.as_usize())
            .max()
            .unwrap_or(0);
        let mut table = vec![None; max_index + 1];
        for node in &snapshot.nodes {
            table[node.id.as_usize()] = Some(node);
        }
        Self { table }
    }

    fn get(&self, id: UndoNodeId) -> Option<&'a text::UndoTreeNodeSnapshot> {
        *self.table.get(id.as_usize())?
    }
}

impl Editor {
    pub fn undo_tree_visible(&self) -> bool {
        self.show_undo_tree
    }

    pub fn selected_undo_node(&self) -> Option<UndoNodeId> {
        self.selected_undo_node
    }

    fn undo_tree_buffer(&self, cx: &App) -> Option<Entity<Buffer>> {
        let buffer = self.buffer.read(cx).as_singleton()?;
        let is_local = buffer.read(cx).replica_id() == ReplicaId::LOCAL;
        is_local.then_some(buffer)
    }

    pub fn undo_tree_snapshot(&self, cx: &App) -> Option<UndoTreeSnapshot> {
        Some(self.undo_tree_buffer(cx)?.read(cx).undo_tree_snapshot())
    }

    pub fn show_undo_tree(&mut self, _: &ShowUndoTree, _: &mut Window, cx: &mut Context<Self>) {
        self.show_undo_tree = true;
        self.selected_undo_node = self.undo_tree_snapshot(cx).map(|snapshot| snapshot.current);
        // View-only change (panel visibility / selection): re-render, but don't
        // emit `UndoHistoryChanged`, which `should_serialize` treats as a reason
        // to persist. Nothing persistable changed here.
        cx.notify();
    }

    pub fn hide_undo_tree(&mut self, _: &HideUndoTree, _: &mut Window, cx: &mut Context<Self>) {
        if self.show_undo_tree {
            self.show_undo_tree = false;
            // View-only change; re-render without triggering serialization.
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

        let Some(buffer) = self.undo_tree_buffer(cx) else {
            return false;
        };
        let target_transaction_id = buffer.read(cx).transaction_id_for_undo_node(target);

        let jumped = buffer.update(cx, |buffer, cx| buffer.jump_to_undo_node(target, cx));

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

        if let Some(buffer) = self.undo_tree_buffer(cx) {
            let buffer = buffer.read(cx);
            let selected_is_valid = self
                .selected_undo_node
                .is_some_and(|selected| buffer.contains_undo_node(selected));
            if follow_current || !selected_is_valid {
                self.selected_undo_node = Some(buffer.current_undo_node());
            }
        } else {
            self.selected_undo_node = None;
        }

        cx.emit(EditorEvent::UndoHistoryChanged);
        cx.notify();
        true
    }

    #[doc(hidden)]
    pub fn benchmark_undo_tree_visualizer_state(&self, cx: &App) -> (bool, usize, usize, usize) {
        let state = self.undo_tree_visualizer_state(cx);
        (
            state.available,
            state.nodes.len(),
            state.edges.len(),
            state.columns,
        )
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
        // Selection-only change; re-render without triggering serialization.
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
        let lookup = UndoNodeLookup::new(&snapshot);

        let Some(parent_id) = lookup.get(selected).and_then(|selected_node| {
            if selected_node.children.is_empty() {
                selected_node.parent
            } else {
                Some(selected_node.id)
            }
        }) else {
            return;
        };
        let Some(parent) = lookup.get(parent_id) else {
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

        let Some(buffer) = self.undo_tree_buffer(cx) else {
            return;
        };
        let switched = buffer.update(cx, |buffer, cx| {
            buffer.switch_undo_branch(parent_id, new_child, cx)
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

    /// Returns the nodes in pre-order (parent before children, children
    /// left-to-right), which is the order keyboard selection steps through.
    ///
    /// Iterative on purpose: a normal session is a near-linear chain, so depth
    /// ≈ node count and a recursive walk would overflow the stack on large
    /// histories. The `visited` set keeps it O(n) and tolerates malformed
    /// snapshots without looping.
    fn undo_tree_node_order(snapshot: &UndoTreeSnapshot) -> Vec<UndoNodeId> {
        let lookup = UndoNodeLookup::new(snapshot);
        let mut ordered_nodes = Vec::new();
        let mut visited = HashSet::default();
        let mut stack = vec![snapshot.root];
        while let Some(node_id) = stack.pop() {
            if !visited.insert(node_id) {
                continue;
            }
            ordered_nodes.push(node_id);
            if let Some(node) = lookup.get(node_id) {
                // Reverse so children pop left-to-right.
                stack.extend(node.children.iter().rev().copied());
            }
        }
        ordered_nodes
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
                nodes: Vec::new(),
                edges: Vec::new(),
                row_timestamps: Vec::new(),
                columns: 0,
            };
        };

        let selected = self
            .selected_undo_node
            .filter(|node_id| snapshot.nodes.iter().any(|node| node.id == *node_id))
            .unwrap_or(snapshot.current);
        undo_tree_visualizer_state(&snapshot, selected)
    }

    /// Visualizer state for rendering, reusing the cached result while the
    /// buffer's undo-history version and the selected node are unchanged. This
    /// keeps the per-frame cost off the O(nodes) snapshot/layout path for large
    /// histories; the cache key changes whenever the tree or selection does.
    fn cached_undo_tree_visualizer_state(&mut self, cx: &App) -> Rc<UndoTreeVisualizerState> {
        let version = self
            .undo_tree_buffer(cx)
            .map(|buffer| buffer.read(cx).undo_history_version());
        let selected = self.selected_undo_node;
        if let Some(cache) = &self.undo_tree_visualizer_cache
            && cache.version == version
            && cache.selected == selected
        {
            return cache.state.clone();
        }

        let state = Rc::new(self.undo_tree_visualizer_state(cx));
        self.undo_tree_visualizer_cache = Some(UndoTreeVisualizerCache {
            version,
            selected,
            state: state.clone(),
        });
        state
    }

    pub(crate) fn render_undo_tree_visualizer(
        &mut self,
        max_size: Size<Pixels>,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        if !self.show_undo_tree {
            return None;
        }

        let state = self.cached_undo_tree_visualizer_state(cx);
        let read_only = self.read_only(cx);
        let can_jump = state.available && !read_only;

        let width = self
            .undo_tree_width
            .unwrap_or(UNDO_TREE_DEFAULT_WIDTH)
            .clamp(UNDO_TREE_MIN_WIDTH, max_size.width.max(UNDO_TREE_MIN_WIDTH));
        // Until the user drags the bottom edge, the height tracks the content
        // (capped), so a small tree stays compact.
        let height = self.undo_tree_height.map(|height| {
            height.clamp(
                UNDO_TREE_MIN_HEIGHT,
                max_size.height.max(UNDO_TREE_MIN_HEIGHT),
            )
        });
        let max_height =
            UNDO_TREE_DEFAULT_MAX_HEIGHT.min(max_size.height.max(UNDO_TREE_MIN_HEIGHT));

        Some(
            WithRemSize::new(ThemeSettings::get_global(cx).ui_font_size(cx))
                .child(
                    div()
                        .id("undo-tree-visualizer")
                        .w(width)
                        .when_some(height, |this, height| this.h(height))
                        .when(height.is_none(), |this| this.max_h(max_height))
                        .flex()
                        .flex_col()
                        .rounded_md()
                        .border_1()
                        .border_color(cx.theme().colors().border)
                        .elevation_2(cx)
                        .overflow_hidden()
                        .on_drag_move(cx.listener(
                            |editor, event: &DragMoveEvent<UndoTreeResizeDrag>, _, cx| {
                                let edge = event.drag(cx).edge;
                                let bounds = event.bounds;
                                let position = event.event.position;
                                if matches!(
                                    edge,
                                    UndoTreeResizeEdge::Left | UndoTreeResizeEdge::BottomLeft
                                ) {
                                    editor.undo_tree_width = Some(
                                        (bounds.right() - position.x).max(UNDO_TREE_MIN_WIDTH),
                                    );
                                }
                                if matches!(
                                    edge,
                                    UndoTreeResizeEdge::Bottom | UndoTreeResizeEdge::BottomLeft
                                ) {
                                    editor.undo_tree_height =
                                        Some((position.y - bounds.top()).max(UNDO_TREE_MIN_HEIGHT));
                                }
                                cx.notify();
                            },
                        ))
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
                                            IconButton::new(
                                                "undo-tree-title-icon",
                                                IconName::GitBranch,
                                            )
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
                                                "undo-tree-latest",
                                                IconName::FastForward,
                                            )
                                            .shape(IconButtonShape::Square)
                                            .size(ButtonSize::Compact)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Muted)
                                            .disabled(!can_jump)
                                            .tooltip(|_, cx| {
                                                cx.new(|_| Tooltip::new("Jump to Latest")).into()
                                            })
                                            .on_click(
                                                cx.listener(|editor, _, window, cx| {
                                                    editor.undo_tree_jump_to_latest(
                                                        &UndoTreeJumpToLatest,
                                                        window,
                                                        cx,
                                                    );
                                                }),
                                            ),
                                        )
                                        .child(
                                            IconButton::new("undo-tree-close", IconName::Close)
                                                .shape(IconButtonShape::Square)
                                                .size(ButtonSize::Compact)
                                                .icon_size(IconSize::Small)
                                                .icon_color(Color::Muted)
                                                .tooltip(|_, cx| {
                                                    cx.new(|_| Tooltip::new("Hide Undo Tree"))
                                                        .into()
                                                })
                                                .on_click(cx.listener(|editor, _, window, cx| {
                                                    editor.hide_undo_tree(
                                                        &HideUndoTree,
                                                        window,
                                                        cx,
                                                    );
                                                })),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .id("undo-tree-body")
                                .flex_1()
                                .min_h_0()
                                .overflow_y_scroll()
                                .when(!state.available, |this| {
                                    this.p_3().child(
                                        Label::new("Undo tree is unavailable for this buffer")
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
                                .when(state.available && !state.nodes.is_empty(), |this| {
                                    this.child(self.render_undo_tree_graph(&state, cx))
                                }),
                        )
                        // Resize handles paint last so they sit above the body and
                        // receive the drag instead of the graph beneath them.
                        .child(Self::render_undo_tree_resize_handle(
                            "undo-tree-resize-left",
                            UndoTreeResizeEdge::Left,
                        ))
                        .child(Self::render_undo_tree_resize_handle(
                            "undo-tree-resize-bottom",
                            UndoTreeResizeEdge::Bottom,
                        ))
                        .child(Self::render_undo_tree_resize_handle(
                            "undo-tree-resize-corner",
                            UndoTreeResizeEdge::BottomLeft,
                        )),
                )
                .into_any_element(),
        )
    }

    fn render_undo_tree_resize_handle(
        id: &'static str,
        edge: UndoTreeResizeEdge,
    ) -> impl IntoElement {
        let handle = div()
            .id(id)
            .absolute()
            .occlude()
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_drag(UndoTreeResizeDrag { edge }, |drag, _, _, cx| {
                cx.new(|_| drag.clone())
            });
        match edge {
            UndoTreeResizeEdge::Left => handle
                .left_0()
                .top_0()
                .h_full()
                .w(UNDO_TREE_RESIZE_HANDLE)
                .cursor_ew_resize(),
            UndoTreeResizeEdge::Bottom => handle
                .bottom_0()
                .left_0()
                .w_full()
                .h(UNDO_TREE_RESIZE_HANDLE)
                .cursor_ns_resize(),
            UndoTreeResizeEdge::BottomLeft => handle
                .bottom_0()
                .left_0()
                .size(px(12.))
                .cursor_nesw_resize(),
        }
    }

    fn render_undo_tree_graph(
        &self,
        state: &UndoTreeVisualizerState,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let rows = state.row_timestamps.len();
        let graph_width =
            UNDO_TREE_GRAPH_PADDING * 2. + UNDO_TREE_COLUMN_WIDTH * state.columns as f32;
        let graph_height = UNDO_TREE_GRAPH_PADDING * 2. + UNDO_TREE_ROW_HEIGHT * rows as f32;

        let column_center =
            |column: f32| UNDO_TREE_GRAPH_PADDING + UNDO_TREE_COLUMN_WIDTH * (column + 0.5);
        let row_center =
            |row: usize| UNDO_TREE_GRAPH_PADDING + UNDO_TREE_ROW_HEIGHT * (row as f32 + 0.5);

        let active_edge_color = cx.theme().colors().text;
        let inactive_edge_color = cx.theme().colors().border_variant.opacity(0.65);
        let edges = state
            .edges
            .iter()
            .map(|edge| {
                let from = point(column_center(edge.from_column), row_center(edge.from_row));
                let to = point(column_center(edge.to_column), row_center(edge.to_row));
                (from, to, edge.active)
            })
            .collect::<Vec<_>>();

        let connectors = canvas(
            |_, _, _| {},
            move |bounds, _, window, _cx| {
                let node_gap = UNDO_TREE_NODE_GLYPH_SIZE / 2. + px(1.);
                let origin = bounds.origin;
                let mut line = |left: Pixels, top: Pixels, right: Pixels, bottom: Pixels, color| {
                    window.paint_quad(fill(
                        Bounds::from_corners(
                            point(origin.x + left, origin.y + top),
                            point(origin.x + right, origin.y + bottom),
                        ),
                        color,
                    ));
                };
                for active_pass in [false, true] {
                    for (from, to, active) in &edges {
                        if *active != active_pass {
                            continue;
                        }
                        let (color, half) = if *active {
                            (active_edge_color, px(1.5) / 2.)
                        } else {
                            (inactive_edge_color, px(1.) / 2.)
                        };
                        let mid_y = from.y + (to.y - from.y) / 2.;
                        let from_exit_y = (from.y + node_gap).min(mid_y);
                        let to_entry_y = (to.y - node_gap).max(mid_y);
                        let (left_x, right_x) = if from.x <= to.x {
                            (from.x, to.x)
                        } else {
                            (to.x, from.x)
                        };
                        // Vertical out of the parent, horizontal across, vertical into the child.
                        line(from.x - half, from_exit_y, from.x + half, mid_y, color);
                        line(left_x, mid_y - half, right_x, mid_y + half, color);
                        line(to.x - half, mid_y, to.x + half, to_entry_y, color);
                    }
                }
            },
        )
        .absolute()
        .size_full();

        let nodes = state
            .nodes
            .iter()
            .enumerate()
            .map(|(index, node)| {
                let node_id = node.id;
                let center = point(column_center(node.column), row_center(node.row));
                let kind = node.kind;
                let node_color = kind.color().color(cx);
                let node_background = cx.theme().colors().elevated_surface_background;
                div()
                    .id(("undo-tree-node", index))
                    .absolute()
                    .left(center.x - UNDO_TREE_NODE_SIZE / 2.)
                    .top(center.y - UNDO_TREE_NODE_SIZE / 2.)
                    .size(UNDO_TREE_NODE_SIZE)
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .cursor_pointer()
                    .when(node.selected, |this| {
                        this.bg(cx.theme().colors().element_selected)
                    })
                    .hover(|style| style.bg(cx.theme().colors().element_hover))
                    .child(
                        div()
                            .size(UNDO_TREE_NODE_GLYPH_SIZE)
                            .rounded_full()
                            .border_1()
                            .border_color(node_color)
                            .bg(node_background)
                            .when(kind == UndoTreeNodeKind::Current, |this| {
                                this.border_2().bg(node_color.opacity(0.18))
                            })
                            .when(kind == UndoTreeNodeKind::Saved, |this| {
                                this.bg(node_color.opacity(0.14))
                            }),
                    )
                    .on_click(cx.listener(move |editor, _, window, cx| {
                        editor.selected_undo_node = Some(node_id);
                        editor.jump_to_undo_node(node_id, window, cx);
                    }))
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        let timestamp_gutter = v_flex()
            .flex_none()
            .w(px(56.))
            .h(graph_height)
            .pt(UNDO_TREE_GRAPH_PADDING)
            .pr_2()
            .border_l_1()
            .border_color(cx.theme().colors().border_variant)
            .children(state.row_timestamps.iter().cloned().map(|timestamp| {
                h_flex()
                    .h(UNDO_TREE_ROW_HEIGHT)
                    .w_full()
                    .items_center()
                    .justify_end()
                    .when_some(timestamp, |this, timestamp| {
                        this.child(
                            Label::new(timestamp)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .single_line(),
                        )
                    })
            }));

        // The outer body scrolls vertically, carrying both the graph and the
        // timestamp gutter together; only the graph itself scrolls horizontally,
        // so the gutter stays pinned to the right edge.
        h_flex()
            .w_full()
            .items_start()
            .child(
                div()
                    .id("undo-tree-graph")
                    .flex_1()
                    .overflow_x_scroll()
                    .child(
                        div()
                            .relative()
                            .w(graph_width)
                            .h(graph_height)
                            .child(connectors)
                            .children(nodes),
                    ),
            )
            .child(timestamp_gutter)
            .into_any_element()
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

/// Builds the renderable visualizer state from a tree snapshot.
///
/// Pure (no `cx`/buffer access) so it can be exercised directly on large
/// synthetic trees in tests; the `Editor` method just supplies the snapshot.
pub(crate) fn undo_tree_visualizer_state(
    snapshot: &UndoTreeSnapshot,
    selected: UndoNodeId,
) -> UndoTreeVisualizerState {
    let lookup = UndoNodeLookup::new(snapshot);
    let selected_path = undo_tree_path_to_root(&lookup, snapshot.root, selected);

    let mut layout = UndoTreeLayout {
        current: snapshot.current,
        selected,
        selected_path,
        lookup: &lookup,
        visited: HashSet::default(),
        nodes: Vec::new(),
        edges: Vec::new(),
        row_edit_times: Vec::new(),
        next_leaf_column: 0.,
    };
    layout.build(snapshot.root);

    let row_timestamps = layout
        .row_edit_times
        .iter()
        .map(|time| time.map(Editor::format_undo_tree_timestamp))
        .collect::<Vec<_>>();
    let columns = layout
        .nodes
        .iter()
        .map(|node| node.column.ceil() as usize + 1)
        .max()
        .unwrap_or(1);

    UndoTreeVisualizerState {
        available: true,
        nodes: layout.nodes,
        edges: layout.edges,
        row_timestamps,
        columns,
    }
}

fn undo_tree_path_to_root(
    lookup: &UndoNodeLookup,
    root: UndoNodeId,
    selected: UndoNodeId,
) -> HashSet<UndoNodeId> {
    let mut path = HashSet::default();
    let mut node_id = selected;
    loop {
        if !path.insert(node_id) {
            return HashSet::default();
        }
        if node_id == root {
            return path;
        }
        let Some(parent_id) = lookup.get(node_id).and_then(|node| node.parent) else {
            return HashSet::default();
        };
        node_id = parent_id;
    }
}

/// Assigns each undo-tree node a (column, row) position for the visualizer.
///
/// Rows correspond to tree depth. Columns are assigned bottom-up: leaves take
/// successive columns, and every parent is centered between its first and last
/// child, so sibling subtrees fan out horizontally without overlapping. This
/// mirrors the layered layout emacs' `undo-tree` draws, but produces fractional
/// columns so a parent can sit exactly between an even number of children.
struct UndoTreeLayout<'a> {
    current: UndoNodeId,
    selected: UndoNodeId,
    selected_path: HashSet<UndoNodeId>,
    lookup: &'a UndoNodeLookup<'a>,
    visited: HashSet<UndoNodeId>,
    nodes: Vec<UndoTreeVisualizerNode>,
    edges: Vec<UndoTreeVisualizerEdge>,
    row_edit_times: Vec<Option<SystemTime>>,
    next_leaf_column: f32,
}

/// One suspended node in the iterative post-order traversal: it has been entered
/// (and marked visited) but is waiting for its children to finish so their
/// columns can center it.
struct UndoTreeLayoutFrame<'a> {
    node: Option<&'a text::UndoTreeNodeSnapshot>,
    node_id: UndoNodeId,
    row: usize,
    on_selected_path: bool,
    /// Index of the next child to descend into.
    next_child: usize,
    /// `(column, active)` of each already-finalized child, in order.
    child_columns: Vec<(f32, bool)>,
}

impl<'a> UndoTreeLayout<'a> {
    /// Lays out the whole tree starting at `root`.
    ///
    /// This is an explicit-stack rewrite of the natural post-order recursion: a
    /// child must be placed before its parent (the parent centers itself between
    /// its first and last child), so a frame stays on the stack until all its
    /// children report back. It is iterative on purpose — a typical undo history
    /// is a near-linear chain, so recursion would be ~depth-deep and overflow the
    /// stack on large histories. Output is identical to the recursive form:
    /// leaves take successive columns left-to-right, parents sit at the midpoint.
    fn build(&mut self, root: UndoNodeId) {
        if !self.visited.insert(root) {
            return;
        }
        let mut stack = vec![self.new_frame(root, 0, self.selected_path.contains(&root))];
        while let Some(frame) = stack.last_mut() {
            // Try to descend into the next unvisited child of the top frame.
            let descend = match frame.node {
                Some(node) if frame.next_child < node.children.len() => {
                    let index = frame.next_child;
                    frame.next_child += 1;
                    let child_active = frame.on_selected_path
                        && self.selected_path.contains(&node.children[index]);
                    Some((node.children[index], frame.row + 1, child_active))
                }
                _ => None,
            };

            if let Some((child_id, row, child_active)) = descend {
                if self.visited.insert(child_id) {
                    let child_frame = self.new_frame(child_id, row, child_active);
                    stack.push(child_frame);
                } else if let Some(frame) = stack.last_mut() {
                    // Already placed elsewhere (only possible for malformed,
                    // non-tree snapshots); mirror the recursion's `return 0.`.
                    frame.child_columns.push((0., child_active));
                }
                continue;
            }

            // All children done: finalize this frame and report up to its parent.
            let frame = stack.pop().expect("loop guard guarantees a top frame");
            let reported = self.finalize(frame);
            if let Some(parent) = stack.last_mut() {
                parent.child_columns.push(reported);
            }
        }
    }

    fn new_frame(
        &self,
        node_id: UndoNodeId,
        row: usize,
        on_selected_path: bool,
    ) -> UndoTreeLayoutFrame<'a> {
        UndoTreeLayoutFrame {
            node: self.lookup.get(node_id),
            node_id,
            row,
            on_selected_path,
            next_child: 0,
            child_columns: Vec::new(),
        }
    }

    /// Emits the node, its child edges, and its row timestamp, then returns the
    /// node's `(column, active)` for its parent to record.
    fn finalize(&mut self, frame: UndoTreeLayoutFrame<'a>) -> (f32, bool) {
        let Some(node) = frame.node else {
            return (0., frame.on_selected_path);
        };

        let column = match (frame.child_columns.first(), frame.child_columns.last()) {
            (Some(first), Some(last)) => (first.0 + last.0) / 2.,
            _ => {
                let column = self.next_leaf_column;
                self.next_leaf_column += 1.;
                column
            }
        };

        for (child_column, child_active) in &frame.child_columns {
            self.edges.push(UndoTreeVisualizerEdge {
                from_column: column,
                from_row: frame.row,
                to_column: *child_column,
                to_row: frame.row + 1,
                active: *child_active,
            });
        }

        let kind = if frame.node_id == self.current {
            UndoTreeNodeKind::Current
        } else if node.saved || node.latest_saved {
            UndoTreeNodeKind::Saved
        } else {
            UndoTreeNodeKind::Normal
        };
        self.nodes.push(UndoTreeVisualizerNode {
            id: frame.node_id,
            column,
            row: frame.row,
            kind,
            selected: frame.node_id == self.selected,
        });

        if self.row_edit_times.len() <= frame.row {
            self.row_edit_times.resize(frame.row + 1, None);
        }
        if let Some(edit_time) = node.last_edit_at.or(node.first_edit_at) {
            let slot = &mut self.row_edit_times[frame.row];
            *slot = Some(slot.map_or(edit_time, |current| current.max(edit_time)));
        }

        (column, frame.on_selected_path)
    }
}
