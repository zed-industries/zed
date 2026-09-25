use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use call_hierarchy::{Call, CallHierarchyMode, fetch_calls, make_call, render_item};
use editor::actions::ShowCallHierarchy;
use editor::{Editor, SelectionEffects, items::entry_label_color, scroll::Autoscroll};
use file_icons::FileIcons;
use fuzzy::StringMatch;
use gpui::{Context, Div, ElementId, Entity, SharedString, Stateful, Task, Window};
use language::{Buffer, ToPoint};
use project::CallHierarchyItem;
use text::Anchor as TextAnchor;
use ui::{Icon, IconName, prelude::*};
use workspace::Workspace;

use super::{
    ExitCallHierarchy, ItemsDisplayMode, OutlinePanel, PanelEntry, SelectedEntry,
    ToggleCallHierarchyDirection, empty_icon, workspace_active_editor,
};

static NEXT_CALL_NODE_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CallNodeId(u64);

impl CallNodeId {
    fn next() -> Self {
        Self(NEXT_CALL_NODE_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CallNodeState {
    /// Children have not been fetched yet.
    Unknown,
    /// Children are currently being fetched.
    Loading,
    /// The node has no callers/callees.
    Leaf,
    Collapsed,
    Expanded,
}

/// A node in the lazily-fetched call hierarchy tree.
#[derive(Debug, Clone)]
struct CallHierarchyNode {
    id: CallNodeId,
    parent_id: Option<CallNodeId>,
    call: Call,
    state: CallNodeState,
    children: Option<Vec<CallHierarchyNode>>,
}

impl CallHierarchyNode {
    fn find(&self, id: CallNodeId) -> Option<&Self> {
        if self.id == id {
            return Some(self);
        }
        self.children
            .as_ref()?
            .iter()
            .find_map(|child| child.find(id))
    }

    fn find_mut(&mut self, id: CallNodeId) -> Option<&mut Self> {
        if self.id == id {
            return Some(self);
        }
        self.children
            .as_mut()?
            .iter_mut()
            .find_map(|child| child.find_mut(id))
    }

    fn flatten<'a>(&'a self, depth: usize, output: &mut Vec<(usize, &'a Self)>) {
        output.push((depth, self));
        if self.state == CallNodeState::Expanded
            && let Some(children) = &self.children
        {
            for child in children {
                child.flatten(depth + 1, output);
            }
        }
    }

    fn row(&self) -> CallHierarchyRow {
        CallHierarchyRow {
            id: self.id,
            call: self.call.clone(),
            state: self.state,
        }
    }
}

pub(super) struct CallHierarchyState {
    direction: CallHierarchyMode,
    root: Option<CallHierarchyNode>,
    root_item: Option<CallHierarchyItem>,
    origin: Option<(Entity<Buffer>, TextAnchor)>,
    loading: bool,
    prepare_task: Task<()>,
    fetch_task: Task<()>,
    expanding: HashMap<CallNodeId, Task<()>>,
}

impl CallHierarchyState {
    fn new(
        direction: CallHierarchyMode,
        origin_buffer: Entity<Buffer>,
        origin_anchor: TextAnchor,
    ) -> Self {
        Self {
            direction,
            root: None,
            root_item: None,
            origin: Some((origin_buffer, origin_anchor)),
            loading: true,
            prepare_task: Task::ready(()),
            fetch_task: Task::ready(()),
            expanding: HashMap::default(),
        }
    }

    pub(super) fn direction(&self) -> CallHierarchyMode {
        self.direction
    }

    pub(super) fn rows(&self) -> Vec<(usize, CallHierarchyRow)> {
        let Some(root) = &self.root else {
            return Vec::new();
        };
        let mut flattened = Vec::new();
        root.flatten(0, &mut flattened);
        flattened
            .into_iter()
            .map(|(depth, node)| (depth, node.row()))
            .collect()
    }

    fn parent_row(&self, row: &CallHierarchyRow) -> Option<CallHierarchyRow> {
        let root = self.root.as_ref()?;
        let parent_id = root.find(row.id)?.parent_id?;
        root.find(parent_id).map(CallHierarchyNode::row)
    }
}

impl std::fmt::Debug for CallHierarchyState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CallHierarchyState")
            .field("direction", &self.direction)
            .field("root", &self.root)
            .field("loading", &self.loading)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug)]
pub(super) struct CallHierarchyRow {
    id: CallNodeId,
    call: Call,
    state: CallNodeState,
}

impl CallHierarchyRow {
    pub(super) fn is_expanded(&self) -> bool {
        self.state == CallNodeState::Expanded
    }

    pub(super) fn name(&self) -> &str {
        &self.call.display.name
    }
}

impl PartialEq for CallHierarchyRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CallHierarchyRow {}

fn call_into_node(call: Call, parent_id: Option<CallNodeId>) -> CallHierarchyNode {
    CallHierarchyNode {
        id: CallNodeId::next(),
        parent_id,
        call,
        state: CallNodeState::Unknown,
        children: None,
    }
}

impl OutlinePanel {
    /// Workspace action handler: enters call hierarchy mode in the outline panel,
    /// seeded from the symbol under the cursor in the active editor.
    pub(super) fn show_call_hierarchy(
        workspace: &mut Workspace,
        _: &ShowCallHierarchy,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some((_, editor)) = workspace_active_editor(workspace, cx) else {
            return;
        };
        let Some(panel) = workspace.panel::<OutlinePanel>(cx) else {
            return;
        };
        let started = panel.update(cx, |panel, cx| {
            panel.start_call_hierarchy(editor, CallHierarchyMode::Incoming, window, cx)
        });
        if started {
            workspace.focus_panel::<OutlinePanel>(window, cx);
            panel.update(cx, |panel, cx| panel.focus_handle.focus(window, cx));
        }
    }

    fn start_call_hierarchy(
        &mut self,
        editor: Entity<Editor>,
        direction: CallHierarchyMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let multi_buffer = editor.read(cx).buffer().clone();
        let selection = editor.read(cx).selections.newest_anchor().head();
        let Some((buffer, anchor)) = multi_buffer
            .read(cx)
            .text_anchor_for_position(selection, cx)
        else {
            return false;
        };

        self.mode = ItemsDisplayMode::CallHierarchy(CallHierarchyState::new(
            direction,
            buffer.clone(),
            anchor,
        ));
        self.selected_entry = SelectedEntry::None;
        self.reveal_selection_task = Task::ready(Ok(()));
        self.cached_entries_update_pending = None;
        self.cached_entries_update_task = Task::ready(());
        self.fetch_call_hierarchy_root(buffer, anchor, direction, window, cx);
        true
    }

    fn fetch_call_hierarchy_root(
        &mut self,
        buffer: Entity<Buffer>,
        position: TextAnchor,
        direction: CallHierarchyMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let prepare_task = self.project.update(cx, |project, cx| {
            project.prepare_call_hierarchy(&buffer, position, cx)
        });
        let task = cx.spawn_in(window, async move |panel, cx| {
            let root_item = match prepare_task.await {
                Ok(Some(items)) => items.into_iter().next(),
                _ => None,
            };
            let Some(root_item) = root_item else {
                panel
                    .update_in(cx, |panel, window, cx| {
                        if let ItemsDisplayMode::CallHierarchy(state) = &mut panel.mode
                            && state.direction == direction
                        {
                            state.loading = false;
                            state.root = None;
                            state.root_item = None;
                        }
                        panel.update_cached_entries(None, window, cx);
                    })
                    .ok();
                return;
            };
            panel
                .update_in(cx, |panel, window, cx| {
                    if let ItemsDisplayMode::CallHierarchy(state) = &mut panel.mode
                        && state.direction == direction
                    {
                        state.root_item = Some(root_item.clone());
                        panel.fetch_call_hierarchy_root_item(root_item, direction, window, cx);
                    }
                })
                .ok();
        });
        if let ItemsDisplayMode::CallHierarchy(state) = &mut self.mode {
            state.prepare_task = task;
        }
        self.update_cached_entries(None, window, cx);
    }

    fn fetch_call_hierarchy_root_item(
        &mut self,
        root_item: CallHierarchyItem,
        direction: CallHierarchyMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let project = self.project.clone();
        let task = cx.spawn_in(window, async move |panel, cx| {
            let children = fetch_calls(&root_item, &project, direction, cx).await;
            let root_call = make_call(root_item, &project, cx).await;
            panel
                .update_in(cx, |panel, window, cx| {
                    if let ItemsDisplayMode::CallHierarchy(state) = &mut panel.mode
                        && state.direction == direction
                    {
                        let mut child_nodes: Vec<CallHierarchyNode> = children
                            .into_iter()
                            .map(|call| call_into_node(call, None))
                            .collect();
                        let root_state = if child_nodes.is_empty() {
                            CallNodeState::Leaf
                        } else {
                            CallNodeState::Expanded
                        };
                        let root_id = CallNodeId::next();
                        for child in &mut child_nodes {
                            child.parent_id = Some(root_id);
                        }
                        state.root_item = Some(root_call.item.clone());
                        state.root = Some(CallHierarchyNode {
                            id: root_id,
                            parent_id: None,
                            call: root_call,
                            state: root_state,
                            children: Some(child_nodes),
                        });
                        state.loading = false;
                    }
                    panel.update_cached_entries(None, window, cx);
                })
                .ok();
        });
        if let ItemsDisplayMode::CallHierarchy(state) = &mut self.mode {
            state.fetch_task = task;
        }
        self.update_cached_entries(None, window, cx);
    }

    pub(super) fn toggle_call_hierarchy_row(
        &mut self,
        row: &CallHierarchyRow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_call_node(row.id, window, cx);
    }

    fn toggle_call_node(&mut self, id: CallNodeId, window: &mut Window, cx: &mut Context<Self>) {
        let ItemsDisplayMode::CallHierarchy(state) = &mut self.mode else {
            return;
        };
        let direction = state.direction;
        let Some(node) = state.root.as_mut().and_then(|root| root.find_mut(id)) else {
            return;
        };
        match node.state {
            CallNodeState::Expanded => node.state = CallNodeState::Collapsed,
            CallNodeState::Collapsed => node.state = CallNodeState::Expanded,
            CallNodeState::Leaf | CallNodeState::Loading => return,
            CallNodeState::Unknown => {
                let item = node.call.item.clone();
                node.state = CallNodeState::Loading;
                let project = self.project.clone();
                let task = cx.spawn_in(window, async move |panel, cx| {
                    let children = fetch_calls(&item, &project, direction, cx).await;
                    panel
                        .update_in(cx, |panel, window, cx| {
                            if let ItemsDisplayMode::CallHierarchy(state) = &mut panel.mode
                                && state.direction == direction
                            {
                                if let Some(node) =
                                    state.root.as_mut().and_then(|root| root.find_mut(id))
                                {
                                    let child_nodes: Vec<CallHierarchyNode> = children
                                        .into_iter()
                                        .map(|call| call_into_node(call, Some(id)))
                                        .collect();
                                    node.state = if child_nodes.is_empty() {
                                        CallNodeState::Leaf
                                    } else {
                                        CallNodeState::Expanded
                                    };
                                    node.children = Some(child_nodes);
                                }
                                state.expanding.remove(&id);
                            }
                            panel.update_cached_entries(None, window, cx);
                        })
                        .ok();
                });
                if let ItemsDisplayMode::CallHierarchy(state) = &mut self.mode {
                    state.expanding.insert(id, task);
                }
            }
        }
        self.update_cached_entries(None, window, cx);
    }

    pub(super) fn select_call_hierarchy_parent(
        &mut self,
        row: &CallHierarchyRow,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let ItemsDisplayMode::CallHierarchy(state) = &self.mode else {
            return false;
        };
        let Some(parent) = state.parent_row(row) else {
            return false;
        };
        let entry = PanelEntry::CallHierarchy(parent);
        if !self
            .cached_entries
            .iter()
            .any(|cached_entry| cached_entry.entry == entry)
        {
            return false;
        }
        self.select_entry(entry, true, window, cx);
        true
    }

    pub(super) fn toggle_call_hierarchy_direction(
        &mut self,
        _: &ToggleCallHierarchyDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let ItemsDisplayMode::CallHierarchy(state) = &mut self.mode else {
            return;
        };
        let new_direction = match state.direction {
            CallHierarchyMode::Incoming => CallHierarchyMode::Outgoing,
            CallHierarchyMode::Outgoing => CallHierarchyMode::Incoming,
        };
        state.direction = new_direction;
        state.root = None;
        state.loading = true;
        state.fetch_task = Task::ready(());
        state.expanding.clear();
        let root_item = state.root_item.clone();
        let origin = state.origin.clone();
        self.selected_entry = SelectedEntry::None;
        self.cached_entries_update_pending = None;
        self.cached_entries_update_task = Task::ready(());
        if let Some(root_item) = root_item {
            self.fetch_call_hierarchy_root_item(root_item, new_direction, window, cx);
        } else if let Some((buffer, anchor)) = origin {
            self.fetch_call_hierarchy_root(buffer, anchor, new_direction, window, cx);
        } else {
            self.update_cached_entries(None, window, cx);
        }
    }

    pub(super) fn exit_call_hierarchy(
        &mut self,
        _: &ExitCallHierarchy,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !matches!(self.mode, ItemsDisplayMode::CallHierarchy(_)) {
            return;
        }
        self.mode = ItemsDisplayMode::Outline;
        self.selected_entry = SelectedEntry::None;
        self.cached_entries_update_pending = None;
        self.cached_entries_update_task = Task::ready(());
        if let Some((active_item, active_editor)) = self
            .workspace
            .upgrade()
            .and_then(|workspace| workspace_active_editor(workspace.read(cx), cx))
            && self.should_replace_active_item(active_item.as_ref())
        {
            self.replace_active_editor(active_item, active_editor, window, cx);
        } else {
            self.update_contents(None, window, cx);
        }
    }

    pub(super) fn open_call_hierarchy_row(
        &mut self,
        row: &CallHierarchyRow,
        focus_item: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_call_target(&row.call, focus_item, window, cx);
    }

    fn open_call_target(
        &mut self,
        call: &Call,
        focus_item: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let buffer = call.target.buffer.clone();
        let target = call.target.range.start;
        self.workspace
            .update(cx, |workspace, cx| {
                let position = target.to_point(&buffer.read(cx).snapshot());
                let pane = workspace.active_pane().clone();
                let editor = workspace.open_project_item::<Editor>(
                    Some(pane),
                    buffer,
                    true,
                    focus_item,
                    true,
                    true,
                    window,
                    cx,
                );
                editor.update(cx, |editor, cx| {
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()),
                        window,
                        cx,
                        |selections| selections.select_ranges([position..position]),
                    );
                });
            })
            .ok();
    }

    pub(super) fn render_call_hierarchy_row(
        &self,
        row: &CallHierarchyRow,
        depth: usize,
        string_match: Option<&StringMatch>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Stateful<Div> {
        let item_id = ElementId::from(SharedString::from(format!("call-hierarchy-{}", row.id.0)));
        let (name_element, detail_element) = render_item(
            &row.call,
            string_match
                .map(|string_match| string_match.ranges().collect::<Vec<_>>())
                .unwrap_or_default(),
            cx,
        );
        let is_active = matches!(
            self.selected_entry(),
            Some(PanelEntry::CallHierarchy(selected)) if selected == row
        );
        let icon = match row.state {
            CallNodeState::Loading => Icon::new(IconName::ArrowCircle)
                .color(Color::Muted)
                .into_any_element(),
            CallNodeState::Leaf => empty_icon(),
            CallNodeState::Unknown | CallNodeState::Collapsed | CallNodeState::Expanded => {
                let is_expanded = row.state == CallNodeState::Expanded;
                FileIcons::get_chevron_icon(is_expanded, cx)
                    .map(|icon_path| {
                        Icon::from_path(icon_path)
                            .color(entry_label_color(is_active))
                            .into_any_element()
                    })
                    .unwrap_or_else(empty_icon)
            }
        };
        let label = h_flex()
            .gap_1()
            .child(name_element)
            .when_some(detail_element, |this, detail| this.child(detail))
            .into_any_element();
        self.entry_element(
            PanelEntry::CallHierarchy(row.clone()),
            item_id,
            depth,
            icon,
            is_active,
            label,
            window,
            cx,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    };

    use editor::{Editor, actions::ShowCallHierarchy};
    use futures::StreamExt as _;
    use gpui::{AppContext as _, Entity, TestAppContext, VisualTestContext};
    use language::{Buffer, FakeLspAdapter, rust_lang};
    use project::{FakeFs, Project};
    use search::BufferSearchBar;
    use serde_json::json;
    use workspace::{Panel, ToolbarItemView, Workspace};

    use super::{CallHierarchyNode, CallNodeState};
    use crate::{
        CollapseSelectedEntry, ExitCallHierarchy, ExpandSelectedEntry, ItemsDisplayMode,
        OpenSelectedEntry, OutlinePanel, PanelEntry, SelectParent, ToggleActiveEditorPin,
        ToggleCallHierarchyDirection,
        tests::{
            add_multi_buffer_editor, add_outline_panel, init_test, outline_panel, select_in_buffer,
            wait_for_outline_tasks,
        },
    };
    use menu::SelectNext;
    use util::{path, rel_path::rel_path};

    async fn setup_call_hierarchy_test(
        source_text: &str,
        cx: &mut TestAppContext,
    ) -> (
        Entity<Project>,
        Entity<Workspace>,
        lsp::FakeLanguageServer,
        lsp::Uri,
        Entity<Editor>,
        Entity<Buffer>,
        Entity<OutlinePanel>,
        VisualTestContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": source_text,
                    "other.rs": "fn other() {}\n",
                }
            }),
        )
        .await;
        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
        project.read_with(cx, |project, _| project.languages().add(rust_lang()));
        let mut fake_language_servers = project.read_with(cx, |project, _| {
            project.languages().register_fake_lsp(
                "Rust",
                FakeLspAdapter {
                    capabilities: lsp::ServerCapabilities {
                        call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(
                            true,
                        )),
                        ..lsp::ServerCapabilities::default()
                    },
                    ..FakeLspAdapter::default()
                },
            )
        });
        let (window, workspace) = add_outline_panel(&project, cx).await;
        let mut visual_cx = VisualTestContext::from_window(window.into(), cx);
        let cx = &mut visual_cx;
        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });
        let editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("src/main.rs")),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        let fake_language_server = fake_language_servers.next().await.unwrap();
        let panel = outline_panel(&workspace, cx);
        panel.update_in(cx, |panel, window, cx| panel.set_active(true, window, cx));
        wait_for_outline_tasks(&panel, cx).await;
        let buffer = editor.read_with(cx, |editor, cx| {
            editor.buffer().read(cx).as_singleton().unwrap()
        });
        let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
        select_in_buffer(&editor, buffer_id, cx);
        let uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();
        (
            project,
            workspace,
            fake_language_server,
            uri,
            editor,
            buffer,
            panel,
            visual_cx,
        )
    }

    fn make_lsp_call_hierarchy_item(
        name: &str,
        uri: lsp::Uri,
        line: u32,
    ) -> lsp::CallHierarchyItem {
        lsp::CallHierarchyItem {
            name: name.to_string(),
            kind: lsp::SymbolKind::FUNCTION,
            tags: None,
            detail: Some(format!("fn {name}()")),
            uri,
            range: lsp::Range {
                start: lsp::Position { line, character: 0 },
                end: lsp::Position {
                    line,
                    character: 10,
                },
            },
            selection_range: lsp::Range {
                start: lsp::Position { line, character: 3 },
                end: lsp::Position {
                    line,
                    character: 3 + name.len() as u32,
                },
            },
            data: None,
        }
    }

    async fn wait_for_call_hierarchy(panel: &Entity<OutlinePanel>, cx: &mut VisualTestContext) {
        for _ in 0..3 {
            cx.executor().run_until_parked();
            wait_for_outline_tasks(panel, cx).await;
        }
    }

    fn row_names(panel: &Entity<OutlinePanel>, cx: &mut VisualTestContext) -> Vec<String> {
        panel.read_with(cx, |panel, _| {
            panel
                .cached_entries
                .iter()
                .filter_map(|cached_entry| match &cached_entry.entry {
                    PanelEntry::CallHierarchy(row) => Some(row.name().to_string()),
                    _ => None,
                })
                .collect()
        })
    }

    fn node_state(
        panel: &Entity<OutlinePanel>,
        name: &str,
        cx: &mut VisualTestContext,
    ) -> Option<CallNodeState> {
        fn find_node(node: &CallHierarchyNode, name: &str) -> Option<CallNodeState> {
            if node.call.display.name == name {
                return Some(node.state);
            }
            node.children
                .as_ref()?
                .iter()
                .find_map(|child| find_node(child, name))
        }

        panel.read_with(cx, |panel, _| {
            let ItemsDisplayMode::CallHierarchy(state) = &panel.mode else {
                return None;
            };
            state.root.as_ref().and_then(|root| find_node(root, name))
        })
    }

    fn selected_row_is_expanded(panel: &Entity<OutlinePanel>, cx: &mut VisualTestContext) -> bool {
        panel.read_with(cx, |panel, _| {
            matches!(
                panel.selected_entry(),
                Some(PanelEntry::CallHierarchy(row)) if row.is_expanded()
            )
        })
    }

    fn selected_row_name(
        panel: &Entity<OutlinePanel>,
        cx: &mut VisualTestContext,
    ) -> Option<String> {
        panel.read_with(cx, |panel, _| match panel.selected_entry() {
            Some(PanelEntry::CallHierarchy(row)) => Some(row.name().to_string()),
            _ => None,
        })
    }

    #[gpui::test]
    async fn test_call_hierarchy_direction_width_and_focus(cx: &mut TestAppContext) {
        let (
        _project,
        _workspace,
        fake_server,
        uri,
        _editor,
        buffer,
        panel,
        mut visual_cx,
    ) = setup_call_hierarchy_test(
        "fn root() {}\nfn first_caller() {}\nfn a_very_long_caller_name() {}\nfn last_caller() {}\nfn callee() {}\n",
        cx,
    )
    .await;
        let cx = &mut visual_cx;
        let prepare_count = Arc::new(AtomicUsize::new(0));
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = uri.clone();
            let prepare_count = prepare_count.clone();
            move |_, _| {
                let uri = uri.clone();
                let prepare_count = prepare_count.clone();
                async move {
                    prepare_count.fetch_add(1, Ordering::SeqCst);
                    Ok(Some(vec![make_lsp_call_hierarchy_item("root", uri, 0)]))
                }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(
                        ["first_caller", "a_very_long_caller_name", "last_caller"]
                            .into_iter()
                            .enumerate()
                            .map(|(index, name)| lsp::CallHierarchyIncomingCall {
                                from: make_lsp_call_hierarchy_item(
                                    name,
                                    uri.clone(),
                                    index as u32 + 1,
                                ),
                                from_ranges: Vec::new(),
                            })
                            .collect(),
                    ))
                }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                        to: make_lsp_call_hierarchy_item("callee", uri, 4),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });

        cx.dispatch_action(ShowCallHierarchy);
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(
            row_names(&panel, cx),
            [
                "root",
                "first_caller",
                "a_very_long_caller_name",
                "last_caller"
            ]
        );
        assert_eq!(selected_row_name(&panel, cx).as_deref(), Some("root"));
        let max_width_index = panel.read_with(cx, |panel, _| panel.max_width_item_index);
        assert_eq!(max_width_index, Some(2));
        let focused =
            cx.update(|window, cx| panel.read(cx).focus_handle.contains_focused(window, cx));
        assert!(focused);

        panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&SelectNext, window, cx);
            panel.select_next(&SelectNext, window, cx);
        });
        assert_eq!(
            selected_row_name(&panel, cx).as_deref(),
            Some("a_very_long_caller_name")
        );
        let focused =
            cx.update(|window, cx| panel.read(cx).focus_handle.contains_focused(window, cx));
        assert!(focused);

        panel.update_in(cx, |panel, window, cx| {
            panel.open_selected_entry(&OpenSelectedEntry, window, cx);
        });
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(
            selected_row_name(&panel, cx).as_deref(),
            Some("a_very_long_caller_name")
        );

        buffer.update(cx, |buffer, cx| {
            let source = buffer.text();
            buffer.edit(
                [(0..buffer.len(), format!("fn inserted() {{}}\n{source}"))],
                None,
                cx,
            );
        });
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_call_hierarchy_direction(&ToggleCallHierarchyDirection, window, cx);
        });
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(prepare_count.load(Ordering::SeqCst), 1);
        assert_eq!(row_names(&panel, cx), ["root", "callee"]);
        assert_eq!(selected_row_name(&panel, cx).as_deref(), Some("root"));
    }

    #[gpui::test]
    async fn test_call_hierarchy_width_index_after_filter(cx: &mut TestAppContext) {
        let (_project, _workspace, fake_server, uri, _editor, _buffer, panel, mut visual_cx) =
            setup_call_hierarchy_test("fn f() {}\nfn a_very_long_caller_name() {}\n", cx).await;
        let cx = &mut visual_cx;
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("f", uri, 0)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("a_very_long_caller_name", uri, 1),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });
        cx.dispatch_action(ShowCallHierarchy);
        wait_for_call_hierarchy(&panel, cx).await;
        let filter_editor = panel.read_with(cx, |panel, _| panel.filter_editor.clone());
        filter_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("very_long", window, cx);
        });
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(row_names(&panel, cx), ["a_very_long_caller_name"]);
        assert_eq!(
            panel.read_with(cx, |panel, _| panel.max_width_item_index),
            Some(0)
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_refreshes_when_reactivated(cx: &mut TestAppContext) {
        let (_project, _workspace, fake_server, uri, _editor, _buffer, panel, mut visual_cx) =
            setup_call_hierarchy_test("fn root() {}\n", cx).await;
        let cx = &mut visual_cx;
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("root", uri, 0)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>(
            move |_, _| async { Ok(Some(Vec::new())) },
        );
        cx.dispatch_action(ShowCallHierarchy);
        panel.update_in(cx, |panel, window, cx| panel.set_active(false, window, cx));
        wait_for_call_hierarchy(&panel, cx).await;
        panel.update(cx, |panel, _| panel.cached_entries.clear());
        panel.update_in(cx, |panel, window, cx| panel.set_active(true, window, cx));
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(row_names(&panel, cx), ["root"]);
    }

    #[gpui::test]
    async fn test_call_hierarchy_parent_and_child_navigation(cx: &mut TestAppContext) {
        let (_project, _workspace, fake_server, uri, _editor, _buffer, panel, mut visual_cx) =
            setup_call_hierarchy_test("fn a() {}\nfn b() {}\nfn c() {}\n", cx).await;
        let cx = &mut visual_cx;
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("a", uri, 0)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            let uri = uri.clone();
            move |params, _| {
                let uri = uri.clone();
                async move {
                    let call = match params.item.name.as_str() {
                        "a" => Some(lsp::CallHierarchyIncomingCall {
                            from: make_lsp_call_hierarchy_item("b", uri.clone(), 1),
                            from_ranges: Vec::new(),
                        }),
                        "b" => Some(lsp::CallHierarchyIncomingCall {
                            from: make_lsp_call_hierarchy_item("c", uri, 2),
                            from_ranges: Vec::new(),
                        }),
                        _ => None,
                    };
                    Ok(call.map(|call| vec![call]))
                }
            }
        });

        cx.dispatch_action(ShowCallHierarchy);
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(row_names(&panel, cx), ["a", "b"]);

        panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        assert_eq!(selected_row_name(&panel, cx).as_deref(), Some("b"));
        panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(row_names(&panel, cx), ["a", "b", "c"]);

        panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        assert_eq!(selected_row_name(&panel, cx).as_deref(), Some("c"));
        panel.update_in(cx, |panel, window, cx| {
            panel.select_parent(&SelectParent, window, cx);
        });
        assert_eq!(selected_row_name(&panel, cx).as_deref(), Some("b"));
        panel.update_in(cx, |panel, window, cx| {
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        assert_eq!(selected_row_name(&panel, cx).as_deref(), Some("c"));
        panel.update_in(cx, |panel, window, cx| {
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        assert_eq!(selected_row_name(&panel, cx).as_deref(), Some("b"));
        assert!(selected_row_is_expanded(&panel, cx));
        assert_eq!(node_state(&panel, "b", cx), Some(CallNodeState::Expanded));
        panel.update_in(cx, |panel, window, cx| {
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(node_state(&panel, "b", cx), Some(CallNodeState::Collapsed));
        assert_eq!(row_names(&panel, cx), ["a", "b"]);
        panel.update_in(cx, |panel, window, cx| {
            panel.collapse_selected_entry(&CollapseSelectedEntry, window, cx);
        });
        assert_eq!(selected_row_name(&panel, cx).as_deref(), Some("a"));
    }

    #[gpui::test]
    async fn test_call_hierarchy_exit_preserves_pin(cx: &mut TestAppContext) {
        let (_project, _workspace, fake_server, uri, _editor, _buffer, panel, mut visual_cx) =
            setup_call_hierarchy_test("fn root() {}\n", cx).await;
        let cx = &mut visual_cx;
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("root", uri, 0)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>(
            move |_, _| async { Ok(Some(Vec::new())) },
        );
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_active_editor_pin(&ToggleActiveEditorPin, window, cx);
        });
        let original_editor = panel.read_with(cx, |panel, _| {
            panel.active_editor().map(|editor| editor.entity_id())
        });
        cx.dispatch_action(ShowCallHierarchy);
        wait_for_call_hierarchy(&panel, cx).await;
        panel.update_in(cx, |panel, window, cx| {
            panel.exit_call_hierarchy(&ExitCallHierarchy, window, cx);
        });
        wait_for_call_hierarchy(&panel, cx).await;
        panel.read_with(cx, |panel, _| {
            assert!(matches!(panel.mode, ItemsDisplayMode::Outline));
            assert!(panel.pinned);
            assert_eq!(
                panel.active_editor().map(|editor| editor.entity_id()),
                original_editor
            );
            assert!(
                panel
                    .cached_entries
                    .iter()
                    .all(|entry| !matches!(entry.entry, PanelEntry::CallHierarchy(_)))
            );
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_keeps_mode_until_exit_after_editor_switch(
        cx: &mut TestAppContext,
    ) {
        let (_project, workspace, fake_server, uri, _editor, _buffer, panel, mut visual_cx) =
            setup_call_hierarchy_test("fn root() {}\n", cx).await;
        let cx = &mut visual_cx;
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("root", uri, 0)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>(
            move |_, _| async { Ok(Some(Vec::new())) },
        );
        cx.dispatch_action(ShowCallHierarchy);
        wait_for_call_hierarchy(&panel, cx).await;
        let original_editor = panel.read_with(cx, |panel, _| {
            panel.active_editor().map(|editor| editor.entity_id())
        });

        let worktree_id = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        });
        let other_editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("src/other.rs")),
                    None,
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        wait_for_call_hierarchy(&panel, cx).await;
        assert!(panel.read_with(cx, |panel, _| matches!(
            panel.mode,
            ItemsDisplayMode::CallHierarchy(_)
        )));
        assert_eq!(
            panel.read_with(cx, |panel, _| {
                panel.active_editor().map(|editor| editor.entity_id())
            }),
            original_editor
        );

        panel.update_in(cx, |panel, window, cx| {
            panel.exit_call_hierarchy(&ExitCallHierarchy, window, cx);
        });
        wait_for_call_hierarchy(&panel, cx).await;
        assert!(panel.read_with(cx, |panel, _| matches!(
            panel.mode,
            ItemsDisplayMode::Outline
        )));
        assert_eq!(
            panel.read_with(cx, |panel, _| {
                panel.active_editor().map(|editor| editor.entity_id())
            }),
            Some(other_editor.entity_id())
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_starts_from_excerpted_multibuffer(cx: &mut TestAppContext) {
        let (project, workspace, fake_server, _uri, _editor, main_buffer, panel, mut visual_cx) =
            setup_call_hierarchy_test("fn main() {}\n", cx).await;
        let cx = &mut visual_cx;
        let other_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/test/src/other.rs"), cx)
            })
            .await
            .unwrap();
        let editor = add_multi_buffer_editor(
            &workspace,
            &project,
            &[(&main_buffer, Vec::new()), (&other_buffer, Vec::new())],
            cx,
        );
        wait_for_outline_tasks(&panel, cx).await;
        let other_buffer_id = other_buffer.read_with(cx, |buffer, _| buffer.remote_id());
        select_in_buffer(&editor, other_buffer_id, cx);
        let prepared_uri = Arc::new(Mutex::new(None));
        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let prepared_uri = prepared_uri.clone();
            move |params, _| {
                let prepared_uri = prepared_uri.clone();
                async move {
                    let uri = params
                        .text_document_position_params
                        .text_document
                        .uri
                        .clone();
                    *prepared_uri.lock().unwrap() = Some(uri.clone());
                    Ok(Some(vec![make_lsp_call_hierarchy_item("other", uri, 0)]))
                }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>(
            move |_, _| async { Ok(Some(Vec::new())) },
        );

        cx.dispatch_action(ShowCallHierarchy);
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(row_names(&panel, cx), ["other"]);
        assert_eq!(
            prepared_uri.lock().unwrap().as_ref(),
            Some(&lsp::Uri::from_file_path(path!("/test/src/other.rs")).unwrap())
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_survives_search_invalidation(cx: &mut TestAppContext) {
        let (_project, workspace, fake_server, uri, editor, buffer, panel, mut visual_cx) =
            setup_call_hierarchy_test(
                "fn root() { needle(); }\nfn caller() {}\nfn needle() {}\n",
                cx,
            )
            .await;
        let cx = &mut visual_cx;
        let search_bar = workspace.update_in(cx, |_, window, cx| {
            cx.new(|cx| {
                let mut search_bar = BufferSearchBar::new(None, window, cx);
                search_bar.set_active_pane_item(Some(&editor), window, cx);
                search_bar.show(window, cx);
                search_bar
            })
        });
        search_bar
            .update_in(cx, |search_bar, window, cx| {
                search_bar.search("needle", None, true, window, cx)
            })
            .await
            .unwrap();
        wait_for_call_hierarchy(&panel, cx).await;
        panel.read_with(cx, |panel, _| {
            assert!(matches!(panel.mode, ItemsDisplayMode::Search(_)));
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("root", uri, 0)])) }
            }
        });
        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            let uri = uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("caller", uri, 1),
                        from_ranges: Vec::new(),
                    }]))
                }
            }
        });
        cx.dispatch_action(ShowCallHierarchy);
        wait_for_call_hierarchy(&panel, cx).await;
        assert_eq!(row_names(&panel, cx), ["root", "caller"]);

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, " ".to_string())], None, cx);
        });
        wait_for_call_hierarchy(&panel, cx).await;
        panel.read_with(cx, |panel, _| {
            assert!(matches!(panel.mode, ItemsDisplayMode::CallHierarchy(_)));
            assert_eq!(
                panel
                    .cached_entries
                    .iter()
                    .filter(|entry| matches!(entry.entry, PanelEntry::CallHierarchy(_)))
                    .count(),
                2
            );
        });
    }
}
