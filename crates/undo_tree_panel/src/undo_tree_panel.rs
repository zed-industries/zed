use anyhow::Result;
use collections::{HashMap, HashSet};
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use gpui::{
    actions, div, px, Action, AnyElement, App, AsyncWindowContext, Context, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement, Pixels,
    Render, SharedString, StatefulInteractiveElement as _, Styled, Subscription, WeakEntity,
    Window,
};

use serde::{Deserialize, Serialize};
use settings::Settings;
use std::time::Instant;
use text::{Operation, TransactionId, UndoTree};
use theme::ThemeSettings;
use ui::{prelude::*, IconName, Label, LabelSize};
use util::ResultExt;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

const UNDO_TREE_PANEL_KEY: &str = "UndoTreePanel";
const FOLD_THRESHOLD: usize = 4;

actions!(undo_tree_panel, [ToggleFocus]);

#[derive(Serialize, Deserialize)]
struct SerializedUndoTreePanel {
    width: Option<f32>,
}

struct DisplayRow {
    prefix: SharedString,
    label: SharedString,
    node_index: Option<usize>,
    transaction_id: Option<TransactionId>,
    is_current: bool,
    is_on_active_path: bool,
    is_fold_marker: bool,
}

pub struct UndoTreePanel {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    active_editor: Option<WeakEntity<Editor>>,
    display_rows: Vec<DisplayRow>,
    selected_index: usize,
    _subscriptions: Vec<Subscription>,
}

impl UndoTreePanel {
    pub fn init(cx: &mut App) {
        cx.observe_new(|workspace: &mut Workspace, _, _| {
            workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
                workspace.toggle_panel_focus::<UndoTreePanel>(window, cx);
            });
        })
        .detach();
    }

    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        let serialized = workspace
            .read_with(&cx, |workspace, _| Self::serialization_key(workspace))
            .ok()
            .and_then(|key| KEY_VALUE_STORE.read_kvp(&key).log_err().flatten())
            .and_then(|value| serde_json::from_str::<SerializedUndoTreePanel>(&value).log_err());

        workspace.update_in(&mut cx, |workspace, window, cx| {
            cx.new(|cx| Self::new(workspace, serialized.as_ref(), window, cx))
        })
    }

    fn serialization_key(workspace: &Workspace) -> String {
        let id = workspace
            .database_id()
            .map(|id| i64::from(id).to_string())
            .or(workspace.session_id())
            .unwrap_or_else(|| "default".to_string());
        format!("{}:{}", UNDO_TREE_PANEL_KEY, id)
    }

    fn new(
        workspace: &mut Workspace,
        serialized: Option<&SerializedUndoTreePanel>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let weak_workspace = workspace.weak_handle();

        let workspace_subscription = cx.subscribe_in(
            &weak_workspace
                .upgrade()
                .expect("workspace must exist during panel creation"),
            window,
            |this: &mut Self, workspace, event, window, cx| {
                if let workspace::Event::ActiveItemChanged = event {
                    if let Some(editor) = workspace_active_editor(workspace.read(cx), cx) {
                        this.replace_active_editor(editor, window, cx);
                    } else {
                        this.clear_active_editor(cx);
                    }
                }
            },
        );

        let width = serialized.and_then(|s| s.width.map(px));

        let mut panel = Self {
            workspace: weak_workspace,
            focus_handle,
            width,
            active_editor: None,
            display_rows: Vec::new(),
            selected_index: 0,
            _subscriptions: vec![workspace_subscription],
        };

        if let Some(editor) = workspace_active_editor(workspace, cx) {
            panel.replace_active_editor(editor, window, cx);
        }

        panel
    }

    fn replace_active_editor(
        &mut self,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.active_editor = Some(editor.downgrade());

        let editor_subscription =
            cx.subscribe_in(&editor, window, |this, _editor, event, _window, cx| {
                if matches!(
                    event,
                    editor::EditorEvent::Edited { .. }
                        | editor::EditorEvent::TransactionUndone { .. }
                        | editor::EditorEvent::TransactionBegun { .. }
                ) {
                    this.rebuild_display(cx);
                }
            });

        // Keep the workspace subscription (index 0), replace any previous editor subscription.
        self._subscriptions.truncate(1);
        self._subscriptions.push(editor_subscription);

        self.rebuild_display(cx);
    }

    fn clear_active_editor(&mut self, cx: &mut Context<Self>) {
        self.active_editor = None;
        self.display_rows.clear();
        self.selected_index = 0;
        self._subscriptions.truncate(1);
        cx.notify();
    }

    fn rebuild_display(&mut self, cx: &mut Context<Self>) {
        let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) else {
            self.display_rows.clear();
            self.selected_index = 0;
            cx.notify();
            return;
        };
        if editor.read(cx).buffer().read(cx).is_singleton() {
            self.rebuild_display_singleton(editor, cx);
        } else {
            self.rebuild_display_multibuffer(editor, cx);
        }
    }

    fn rebuild_display_singleton(&mut self, editor: Entity<Editor>, cx: &mut Context<Self>) {
        let (tree, edit_summaries) = {
            let buffer = editor.read(cx).buffer().read(cx).as_singleton().unwrap();
            let buffer_ref = buffer.read(cx);
            let tree = buffer_ref.undo_tree_snapshot();
            if tree.is_empty() {
                (None, HashMap::default())
            } else {
                let operations = buffer_ref.operations();
                let mut summaries: HashMap<TransactionId, String> = HashMap::default();
                for node in tree.all_nodes() {
                    let tid = node.entry.transaction_id();
                    let mut new_texts: Vec<&str> = Vec::new();
                    for edit_id in &node.entry.transaction().edit_ids {
                        if let Some(Operation::Edit(edit)) = operations.get(edit_id) {
                            for text in &edit.new_text {
                                if !text.is_empty() {
                                    new_texts.push(text);
                                }
                            }
                        }
                    }
                    let summary = if new_texts.is_empty() {
                        "delete".to_string()
                    } else {
                        truncate_label(&new_texts.join(""), 24)
                    };
                    summaries.insert(tid, summary);
                }
                (Some(tree), summaries)
            }
        };

        let Some(tree) = tree else {
            self.display_rows.clear();
            self.selected_index = 0;
            cx.notify();
            return;
        };

        let active_path: HashSet<usize> = tree.active_path().into_iter().collect();
        let current = tree.current();
        let (chrono_pos, chrono_total) = tree.chrono_position();

        self.display_rows.clear();

        let root_children: Vec<usize> = tree
            .root_children()
            .iter()
            .copied()
            .filter(|&idx| tree.is_live(idx))
            .collect();

        // Root node — jj-style with glyph.
        let is_initial_current = current.is_none();
        let initial_glyph = if is_initial_current { '@' } else { '○' };
        let first_transaction_id = root_children
            .first()
            .and_then(|&idx| tree.node(idx))
            .map(|node| node.entry.transaction_id());
        self.display_rows.push(DisplayRow {
            prefix: SharedString::from(format!("{}  ", initial_glyph)),
            label: SharedString::from(format!("initial ({}/{})", chrono_pos, chrono_total)),
            node_index: None,
            transaction_id: first_transaction_id,
            is_current: is_initial_current,
            is_on_active_path: true,
            is_fold_marker: false,
        });

        if !root_children.is_empty() {
            let mut active_columns: Vec<bool> = vec![true];

            let now = Instant::now();

            // Alt root children (fork to right columns).
            for &alt_child in root_children.iter().skip(1) {
                let new_col = active_columns
                    .iter()
                    .position(|active| !active)
                    .unwrap_or(active_columns.len());
                while active_columns.len() <= new_col {
                    active_columns.push(false);
                }
                active_columns[new_col] = true;

                let fork = build_fork_connector(&active_columns, 0, new_col);
                self.display_rows.push(DisplayRow {
                    prefix: SharedString::from(fork),
                    label: SharedString::from(""),
                    node_index: None,
                    transaction_id: None,
                    is_current: false,
                    is_on_active_path: false,
                    is_fold_marker: false,
                });

                flatten_node(
                    &tree,
                    alt_child,
                    new_col,
                    &mut active_columns,
                    &mut self.display_rows,
                    &active_path,
                    current,
                    true,
                    now,
                    &edit_summaries,
                );
            }

            // Continuation before primary root child.
            self.display_rows.push(DisplayRow {
                prefix: SharedString::from(build_edge_prefix(&active_columns)),
                label: SharedString::from(""),
                node_index: None,
                transaction_id: None,
                is_current: false,
                is_on_active_path: false,
                is_fold_marker: false,
            });

            // Primary root child continues at col 0.
            flatten_node(
                &tree,
                root_children[0],
                0,
                &mut active_columns,
                &mut self.display_rows,
                &active_path,
                current,
                true,
                now,
                &edit_summaries,
            );
        }

        // Move selection to the current node.
        if let Some(current_idx) = current {
            if let Some(pos) = self
                .display_rows
                .iter()
                .position(|r| r.node_index == Some(current_idx))
            {
                self.selected_index = pos;
            }
        } else {
            self.selected_index = 0;
        }

        cx.notify();
    }

    fn rebuild_display_multibuffer(&mut self, editor: Entity<Editor>, cx: &mut Context<Self>) {
        let multi_buffer = editor.read(cx).buffer().clone();
        let snapshot = multi_buffer.read(cx).undo_history_snapshot();

        self.display_rows.clear();

        if snapshot.entries.is_empty() {
            self.selected_index = 0;
            cx.notify();
            return;
        }

        let now = Instant::now();
        let total = snapshot.entries.len() + 1;

        let is_initial_current = snapshot.undo_depth == 0;
        let initial_glyph = if is_initial_current { '@' } else { '○' };
        self.display_rows.push(DisplayRow {
            prefix: SharedString::from(format!("{}  ", initial_glyph)),
            label: SharedString::from(format!(
                "initial ({}/{})",
                if is_initial_current { 1 } else { snapshot.undo_depth + 1 },
                total,
            )),
            node_index: None,
            transaction_id: None,
            is_current: is_initial_current,
            is_on_active_path: true,
            is_fold_marker: false,
        });

        // Gather per-entry display data while holding read borrows, then
        // release them before calling cx.notify().
        {
            let multi_buffer_ref = multi_buffer.read(cx);
            for (entry_index, entry) in snapshot.entries.iter().enumerate() {
                let is_entry_current = entry_index + 1 == snapshot.undo_depth;
                let is_on_undo_stack = entry_index < snapshot.undo_depth;

                // Edge connector row.
                self.display_rows.push(DisplayRow {
                    prefix: SharedString::from("│ "),
                    label: SharedString::from(""),
                    node_index: None,
                    transaction_id: None,
                    is_current: false,
                    is_on_active_path: false,
                    is_fold_marker: false,
                });

                // Build summary from per-buffer edit content.
                let mut file_names: Vec<String> = Vec::new();
                let mut buffer_summaries: Vec<String> = Vec::new();
                for (buffer_id, text_txn_id) in &entry.buffer_transactions {
                    if let Some(buffer_entity) = multi_buffer_ref.buffer(*buffer_id) {
                        let buffer_ref = buffer_entity.read(cx);
                        if let Some(file) = buffer_ref.file() {
                            file_names.push(file.file_name(cx).to_string());
                        }
                        let tree = buffer_ref.undo_tree_snapshot();
                        for node in tree.all_nodes() {
                            if node.entry.transaction_id() == *text_txn_id {
                                let operations = buffer_ref.operations();
                                let mut new_texts: Vec<&str> = Vec::new();
                                for edit_id in &node.entry.transaction().edit_ids {
                                    if let Some(Operation::Edit(edit)) =
                                        operations.get(edit_id)
                                    {
                                        for text in &edit.new_text {
                                            if !text.is_empty() {
                                                new_texts.push(text);
                                            }
                                        }
                                    }
                                }
                                if new_texts.is_empty() {
                                    buffer_summaries.push("delete".to_string());
                                } else {
                                    buffer_summaries
                                        .push(truncate_label(&new_texts.join(""), 24));
                                }
                                break;
                            }
                        }
                    }
                }

                let file_label = match file_names.len() {
                    0 => String::new(),
                    1 => file_names.into_iter().next().unwrap_or_default(),
                    n => format!("{} files", n),
                };
                let text_summary = if buffer_summaries.is_empty() {
                    String::new()
                } else {
                    truncate_label(&buffer_summaries.join(", "), 24)
                };
                let summary = match (text_summary.is_empty(), file_label.is_empty()) {
                    (true, true) => String::new(),
                    (true, false) => file_label,
                    (false, true) => text_summary,
                    (false, false) => format!("{} · {}", text_summary, file_label),
                };
                let age = format_elapsed(entry.first_edit_at, now);
                let label = if summary.is_empty() {
                    age
                } else {
                    format!("{} · {}", summary, age)
                };

                let glyph = if is_entry_current { '@' } else { '○' };
                self.display_rows.push(DisplayRow {
                    prefix: SharedString::from(format!("{}  ", glyph)),
                    label: SharedString::from(label),
                    node_index: Some(entry_index),
                    transaction_id: Some(entry.id),
                    is_current: is_entry_current,
                    is_on_active_path: is_on_undo_stack || is_entry_current,
                    is_fold_marker: false,
                });
            }
        }

        if let Some(pos) = self.display_rows.iter().position(|row| row.is_current) {
            self.selected_index = pos;
        } else {
            self.selected_index = 0;
        }

        cx.notify();
    }

    fn goto_node(&mut self, row_index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(row) = self.display_rows.get(row_index) else {
            return;
        };
        let Some(editor) = self.active_editor.as_ref().and_then(|e| e.upgrade()) else {
            return;
        };

        let is_singleton = editor.read(cx).buffer().read(cx).is_singleton();

        if is_singleton {
            let Some(transaction_id) = row.transaction_id else {
                return;
            };
            editor.update(cx, |editor, cx| {
                editor.buffer().update(cx, |buffer, cx| {
                    if let Some(buffer) = buffer.as_singleton() {
                        buffer.update(cx, |buffer, cx| {
                            buffer.goto_transaction(transaction_id, cx);
                            if row.node_index.is_none() {
                                buffer.undo(cx);
                            }
                        });
                    }
                });
            });
        } else {
            let target_depth = match row.node_index {
                Some(entry_index) => entry_index + 1,
                None => 0, // "initial" row — undo everything
            };
            let multi_buffer = editor.read(cx).buffer().clone();
            let current_depth = multi_buffer.read(cx).undo_history_snapshot().undo_depth;

            multi_buffer.update(cx, |buffer, cx| {
                if target_depth < current_depth {
                    for _ in 0..(current_depth - target_depth) {
                        buffer.undo(cx);
                    }
                } else if target_depth > current_depth {
                    for _ in 0..(target_depth - current_depth) {
                        buffer.redo(cx);
                    }
                }
            });
        }

        self.selected_index = row_index;
        self.rebuild_display(cx);
    }

    fn serialize(&self, cx: &Context<Self>) {
        let width = self.width.map(|w| w.as_f32());
        let workspace = self.workspace.clone();
        cx.spawn(async move |_, cx| {
            let key = workspace.read_with(cx, |workspace, _| {
                Self::serialization_key(workspace)
            }).ok();
            if let Some(key) = key {
                let serialized = serde_json::to_string(&SerializedUndoTreePanel { width })
                    .log_err()
                    .unwrap_or_default();
                KEY_VALUE_STORE.write_kvp(key, serialized).await.log_err();
            }
        })
        .detach();
    }
}

// ── Workspace helper ──────────────────────────────────────────────────────

fn workspace_active_editor(workspace: &Workspace, cx: &App) -> Option<Entity<Editor>> {
    let active_item = workspace.active_item(cx)?;
    active_item
        .act_as::<Editor>(cx)
        .filter(|editor| editor.read(cx).mode().is_full())
}

// ── Tree flattening ───────────────────────────────────────────────────────

fn flatten_node(
    tree: &UndoTree,
    node_index: usize,
    col: usize,
    active_columns: &mut Vec<bool>,
    out: &mut Vec<DisplayRow>,
    active_path: &HashSet<usize>,
    current: Option<usize>,
    folding_enabled: bool,
    now: Instant,
    edit_summaries: &HashMap<TransactionId, String>,
) {
    let Some(node) = tree.node(node_index) else {
        return;
    };

    while active_columns.len() <= col {
        active_columns.push(false);
    }
    active_columns[col] = true;

    let is_current = current == Some(node_index);
    let prefix = build_node_prefix(active_columns, col, is_current);
    let age = format_elapsed(node.entry.first_edit_at(), now);
    let summary = edit_summaries
        .get(&node.entry.transaction_id())
        .map(|s| s.as_str())
        .unwrap_or("edit");
    let label = format!("{} · {}", summary, age);

    out.push(DisplayRow {
        prefix: SharedString::from(prefix),
        label: SharedString::from(label),
        node_index: Some(node_index),
        transaction_id: Some(node.entry.transaction_id()),
        is_current,
        is_on_active_path: active_path.contains(&node_index),
        is_fold_marker: false,
    });

    let live_children: Vec<usize> = node
        .children
        .iter()
        .copied()
        .filter(|&idx| tree.is_live(idx))
        .collect();

    if live_children.is_empty() {
        active_columns[col] = false;
        return;
    }

    // Alternate branches (fork to right columns with ├─╮ connector).
    for &alt_child in live_children.iter().skip(1) {
        let new_col = active_columns
            .iter()
            .position(|active| !active)
            .unwrap_or(active_columns.len());
        while active_columns.len() <= new_col {
            active_columns.push(false);
        }
        active_columns[new_col] = true;

        let fork = build_fork_connector(active_columns, col, new_col);
        out.push(DisplayRow {
            prefix: SharedString::from(fork),
            label: SharedString::from(""),
            node_index: None,
            transaction_id: None,
            is_current: false,
            is_on_active_path: false,
            is_fold_marker: false,
        });

        flatten_node(
            tree,
            alt_child,
            new_col,
            active_columns,
            out,
            active_path,
            current,
            folding_enabled,
            now,
            edit_summaries,
        );
    }

    // Continuation before primary child.
    let continuation = build_edge_prefix(active_columns);
    out.push(DisplayRow {
        prefix: SharedString::from(continuation),
        label: SharedString::from(""),
        node_index: None,
        transaction_id: None,
        is_current: false,
        is_on_active_path: false,
        is_fold_marker: false,
    });

    // Fold long single-child chains (≥ FOLD_THRESHOLD consecutive nodes).
    if live_children.len() == 1 && folding_enabled {
        let child = live_children[0];
        let chain_len = chain_length_from(tree, child);
        let chain_end = chain_end_from(tree, child);
        let hides_current =
            current.is_some_and(|c| chain_contains_node(tree, child, c) && chain_end != c);

        if chain_len >= FOLD_THRESHOLD && !hides_current {
            let hidden = chain_len - 1;
            let fold_prefix = build_fold_prefix(active_columns, col);
            out.push(DisplayRow {
                prefix: SharedString::from(fold_prefix),
                label: SharedString::from(format!("({} hidden)", hidden)),
                node_index: None,
                transaction_id: None,
                is_current: false,
                is_on_active_path: false,
                is_fold_marker: true,
            });
            flatten_node(
                tree,
                chain_end,
                col,
                active_columns,
                out,
                active_path,
                current,
                folding_enabled,
                now,
                edit_summaries,
            );
            return;
        }
    }

    // Primary child (newest, continues straight down).
    flatten_node(
        tree,
        live_children[0],
        col,
        active_columns,
        out,
        active_path,
        current,
        folding_enabled,
        now,
        edit_summaries,
    );
}

fn chain_length_from(tree: &UndoTree, start: usize) -> usize {
    let mut length = 0;
    let mut current = start;
    while let Some(node) = tree.node(current) {
        let live_children: Vec<usize> = node
            .children
            .iter()
            .copied()
            .filter(|&idx| tree.is_live(idx))
            .collect();
        length += 1;
        if live_children.len() == 1 {
            current = live_children[0];
        } else {
            break;
        }
    }
    length
}

fn chain_end_from(tree: &UndoTree, start: usize) -> usize {
    let mut current = start;
    while let Some(node) = tree.node(current) {
        let live_children: Vec<usize> = node
            .children
            .iter()
            .copied()
            .filter(|&idx| tree.is_live(idx))
            .collect();
        if live_children.len() == 1 {
            current = live_children[0];
        } else {
            break;
        }
    }
    current
}

fn chain_contains_node(tree: &UndoTree, start: usize, target: usize) -> bool {
    let mut current = start;
    while let Some(node) = tree.node(current) {
        if current == target {
            return true;
        }
        let live_children: Vec<usize> = node
            .children
            .iter()
            .copied()
            .filter(|&idx| tree.is_live(idx))
            .collect();
        if live_children.len() == 1 {
            current = live_children[0];
        } else {
            break;
        }
    }
    current == target
}

fn build_node_prefix(active_columns: &[bool], node_col: usize, is_current: bool) -> String {
    let glyph = if is_current { '@' } else { '○' };
    let max_col = active_columns
        .iter()
        .rposition(|&active| active)
        .map(|pos| pos + 1)
        .unwrap_or(0)
        .max(node_col + 1);
    let mut prefix = String::new();
    for col in 0..max_col {
        if col == node_col {
            prefix.push(glyph);
        } else if active_columns.get(col).copied().unwrap_or(false) {
            prefix.push('│');
        } else {
            prefix.push(' ');
        }
        prefix.push(' ');
    }
    prefix.push(' ');
    prefix
}

fn build_edge_prefix(active_columns: &[bool]) -> String {
    let max_col = active_columns
        .iter()
        .rposition(|&active| active)
        .map(|pos| pos + 1)
        .unwrap_or(0);
    let mut prefix = String::new();
    for col in 0..max_col {
        if active_columns[col] {
            prefix.push('│');
        } else {
            prefix.push(' ');
        }
        prefix.push(' ');
    }
    prefix
}

fn build_fork_connector(active_columns: &[bool], trunk_col: usize, branch_col: usize) -> String {
    let max_col = active_columns
        .iter()
        .rposition(|&active| active)
        .map(|pos| pos + 1)
        .unwrap_or(0)
        .max(branch_col + 1);
    let mut prefix = String::new();
    for col in 0..max_col {
        if col == trunk_col {
            prefix.push('├');
        } else if col == branch_col {
            prefix.push('╮');
        } else if col > trunk_col && col < branch_col {
            if active_columns.get(col).copied().unwrap_or(false) {
                prefix.push('┼');
            } else {
                prefix.push('─');
            }
        } else if active_columns.get(col).copied().unwrap_or(false) {
            prefix.push('│');
        } else {
            prefix.push(' ');
        }
        if col >= trunk_col && col < branch_col {
            prefix.push('─');
        } else {
            prefix.push(' ');
        }
    }
    prefix
}

fn build_fold_prefix(active_columns: &[bool], fold_col: usize) -> String {
    let max_col = active_columns
        .iter()
        .rposition(|&active| active)
        .map(|pos| pos + 1)
        .unwrap_or(0)
        .max(fold_col + 1);
    let mut prefix = String::new();
    for col in 0..max_col {
        if col == fold_col {
            prefix.push('~');
        } else if active_columns.get(col).copied().unwrap_or(false) {
            prefix.push('│');
        } else {
            prefix.push(' ');
        }
        prefix.push(' ');
    }
    prefix.push(' ');
    prefix
}

fn truncate_label(text: &str, max_chars: usize) -> String {
    let normalized: String = text
        .chars()
        .map(|c| if c.is_whitespace() { ' ' } else { c })
        .collect();
    let trimmed = normalized.trim();
    if trimmed.chars().count() <= max_chars {
        trimmed.to_string()
    } else {
        let truncated: String = trimmed.chars().take(max_chars).collect();
        format!("{}…", truncated)
    }
}

fn format_elapsed(instant: Instant, now: Instant) -> String {
    let elapsed = now.checked_duration_since(instant).unwrap_or_default();
    let secs = elapsed.as_secs();
    if secs < 5 {
        "just now".to_string()
    } else if secs < 60 {
        format!("{}s ago", secs)
    } else if secs < 3600 {
        format!("{}m ago", secs / 60)
    } else if secs < 86400 {
        format!("{}h ago", secs / 3600)
    } else {
        format!("{}d ago", secs / 86400)
    }
}

// ── Trait impls ───────────────────────────────────────────────────────────

impl EventEmitter<PanelEvent> for UndoTreePanel {}

impl Focusable for UndoTreePanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for UndoTreePanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let row_count = self.display_rows.len();
        let colors = cx.theme().colors();

        v_flex()
            .id("undo_tree_panel")
            .track_focus(&self.focus_handle)
            .size_full()
            .overflow_hidden()
            .bg(colors.panel_background)
            .child(
                div()
                    .px_2()
                    .py_1()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(colors.border)
                    .child(
                        Label::new("Undo Tree")
                            .size(LabelSize::Small)
                            .color(ui::Color::Muted),
                    ),
            )
            .child({
                let buffer_font = ThemeSettings::get_global(cx).buffer_font.clone();
                div()
                    .id("undo-tree-scroll-area")
                    .font(buffer_font)
                    .text_sm()
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .py_1()
                    .children(if row_count > 0 {
                        self.display_rows
                            .iter()
                            .enumerate()
                            .map(|(ix, row)| self.render_row_div(ix, row, cx))
                            .collect::<Vec<_>>()
                    } else {
                        vec![div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .py_8()
                            .child(
                                Label::new("No undo history")
                                    .size(LabelSize::Small)
                                    .color(ui::Color::Muted),
                            )
                            .into_any_element()]
                    })
            })
    }
}

impl UndoTreePanel {
    fn render_row_div(
        &self,
        ix: usize,
        row: &DisplayRow,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let colors = cx.theme().colors();
        let is_selected = ix == self.selected_index;
        let is_clickable = !row.is_fold_marker && !row.label.is_empty();

        let text_color = if row.is_current {
            colors.text_accent
        } else if row.is_on_active_path {
            colors.text
        } else if row.is_fold_marker {
            colors.text_disabled
        } else {
            colors.text_muted
        };

        let text: SharedString = format!("{}{}", row.prefix, row.label).into();

        div()
            .id(("undo-tree-row", ix))
            .px_2()
            .py_0p5()
            .w_full()
            .rounded_md()
            .text_color(text_color)
            .when(is_selected, |el| el.bg(colors.ghost_element_selected))
            .when(is_clickable, |el| {
                el.hover(|el| el.bg(colors.ghost_element_hover))
                    .cursor_pointer()
            })
            .on_click(cx.listener(move |this, _event, window, cx| {
                this.goto_node(ix, window, cx);
            }))
            .child(text)
            .into_any_element()
    }
}

impl Panel for UndoTreePanel {
    fn persistent_name() -> &'static str {
        "UndoTreePanel"
    }

    fn panel_key() -> &'static str {
        UNDO_TREE_PANEL_KEY
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(
        &mut self,
        _position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.serialize(cx);
    }

    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        self.width.unwrap_or(px(240.0))
    }

    fn set_size(
        &mut self,
        size: Option<Pixels>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::ListTree)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Undo Tree Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        4
    }
}