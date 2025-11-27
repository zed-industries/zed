mod call_hierarchy_panel_settings;

use std::collections::HashMap;
use std::ops::Range;
use std::sync::atomic::{AtomicU64, Ordering};

use call_hierarchy::{Call, CallHierarchyMode, fetch_calls, render_item};

use anyhow::Context as _;
use db::kvp::KEY_VALUE_STORE;
use editor::{
    Bias, Editor, SelectionEffects, ShowCallHierarchy, items::entry_label_color, scroll::Autoscroll,
};
use file_icons::FileIcons;

use fuzzy::{StringMatch, StringMatchCandidate, match_strings};
use gpui::{
    Action, AnyElement, App, AsyncWindowContext, Bounds, ClickEvent, Context, ElementId, Entity,
    EventEmitter, FocusHandle, Focusable, InteractiveElement, IntoElement, KeyContext,
    ParentElement, Pixels, Render, SharedString, Styled, Subscription, Task,
    UniformListScrollHandle, WeakEntity, Window, actions, div, point, px, size, uniform_list,
};
use language::{Anchor, PointUtf16, ToPointUtf16, Unclipped};
use lsp::Uri;
use menu::{Confirm, SelectNext, SelectPrevious};
use project::{CallHierarchyItem, Project};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};

use ui::{
    CommonAnimationExt, Icon, IconButton, IconButtonShape, IconName, IconSize, IndentGuideColors,
    IndentGuideLayout, Label, LabelSize, ListItem, RenderedIndentGuide, ScrollAxes, Scrollbars,
    Tab, Tooltip, WithScrollbar, indent_guides, prelude::*,
};
use util::ResultExt;
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

use call_hierarchy_panel_settings::{CallHierarchyPanelSettings, DockSide, ShowIndentGuides};

actions!(
    call_hierarchy_panel,
    [
        ToggleFocus,
        ToggleMode,
        ToggleDetails,
        Refresh,
        ExpandSelectedEntry,
        CollapseSelectedEntry,
        ToggleSelectedEntry,
        CollapseAll,
    ]
);

const CALL_HIERARCHY_PANEL_KEY: &str = "CallHierarchyPanel";

static NEXT_ENTRY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct EntryId(u64);

impl EntryId {
    fn next() -> Self {
        Self(NEXT_ENTRY_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct CacheKey(Uri, Range<Unclipped<PointUtf16>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum CachedEntryState {
    #[default]
    Unknown,
    Loading,
    Leaf,
    Collapsed,
    Expanded,
}

#[derive(Debug, Clone)]
struct CachedEntry {
    id: EntryId,
    call: Call,
    state: CachedEntryState,
    depth: usize,
    string_match: Option<StringMatch>,
}

#[derive(Debug, Clone)]
struct CallHierarchyEntry {
    entry: CachedEntry,
    children: Option<Vec<CallHierarchyEntry>>,
}

impl From<Call> for CallHierarchyEntry {
    fn from(call: Call) -> Self {
        Self {
            entry: CachedEntry {
                id: EntryId::next(),
                call,
                state: CachedEntryState::Unknown,
                depth: 0,
                string_match: None,
            },
            children: None,
        }
    }
}

impl CallHierarchyEntry {
    fn iter_visible(&self) -> VisibleEntryIter<'_> {
        VisibleEntryIter(vec![(0, self)])
    }

    fn find_by_id_mut(&mut self, target_id: EntryId) -> Option<&mut CallHierarchyEntry> {
        if self.entry.id == target_id {
            return Some(self);
        }

        if let Some(children) = &mut self.children {
            for child in children {
                if let Some(found) = child.find_by_id_mut(target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
}

struct VisibleEntryIter<'a>(Vec<(usize, &'a CallHierarchyEntry)>);

impl<'a> Iterator for VisibleEntryIter<'a> {
    type Item = (usize, &'a CallHierarchyEntry);

    fn next(&mut self) -> Option<Self::Item> {
        let (depth, entry) = self.0.pop()?;

        if entry.entry.state == CachedEntryState::Expanded {
            if let Some(children) = &entry.children {
                for child in children.iter().rev() {
                    self.0.push((depth + 1, child));
                }
            }
        }

        Some((depth, entry))
    }
}

pub struct CallHierarchyPanel {
    width: Option<Pixels>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    active: bool,
    mode: CallHierarchyMode,
    show_details: bool,
    focus_handle: FocusHandle,
    scroll_handle: UniformListScrollHandle,
    pending_serialization: Task<Option<()>>,
    _subscriptions: Vec<Subscription>,

    root_entry: Option<CallHierarchyEntry>,
    root_buffer: Option<Entity<language::Buffer>>,
    root_anchor: Option<Anchor>,
    fetch_task: Task<()>,
    expanding_tasks: HashMap<EntryId, Task<()>>,
    loading: bool,
    selected_index: Option<usize>,
    filter_editor: Entity<Editor>,
    cached_entries: Vec<CachedEntry>,
    cached_entries_update_task: Task<()>,
    children_cache: HashMap<CacheKey, Vec<Call>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializedCallHierarchyPanel {
    width: Option<f32>,
    active: Option<bool>,
}

pub fn init(cx: &mut App) {
    CallHierarchyPanelSettings::register(cx);
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<CallHierarchyPanel>(window, cx);
        });
        workspace.register_action(|workspace, _: &ToggleDetails, _, cx| {
            if let Some(panel) = workspace.panel::<CallHierarchyPanel>(cx) {
                panel.update(cx, |panel, cx| {
                    panel.toggle_details(cx);
                });
            }
        });
    })
    .detach();
}

impl CallHierarchyPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        let serialization_key = Self::serialization_key();
        let serialized_panel = cx
            .background_spawn(async move { KEY_VALUE_STORE.read_kvp(&serialization_key) })
            .await
            .context("loading call hierarchy panel")
            .log_err()
            .flatten()
            .and_then(|panel| {
                serde_json::from_str::<SerializedCallHierarchyPanel>(&panel).log_err()
            });

        workspace.update_in(&mut cx, |workspace, window, cx| {
            let panel = CallHierarchyPanel::new(workspace, window, cx);
            if let Some(serialized_panel) = serialized_panel {
                panel.update(cx, |panel, cx| {
                    panel.width = serialized_panel.width.map(gpui::px);
                    panel.active = serialized_panel.active.unwrap_or(false);
                    cx.notify();
                });
            }
            panel
        })
    }

    fn new(
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let project = workspace.project().clone();
        let workspace_weak = workspace.weak_handle();

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Incoming calls...", window, cx);
            editor
        });

        cx.new(|cx| {
            let settings_subscription = cx.observe_global::<SettingsStore>(|_, cx| {
                cx.notify();
            });
            let filter_editor_subscription = cx.subscribe_in(
                &filter_editor,
                window,
                |this: &mut CallHierarchyPanel, _, event: &editor::EditorEvent, window, cx| {
                    if let editor::EditorEvent::BufferEdited { .. } = event {
                        this.update_cached_entries(window, cx);
                    }
                },
            );

            let focus_handle = cx.focus_handle();

            Self {
                width: None,
                workspace: workspace_weak,
                project,
                active: false,
                mode: CallHierarchyMode::default(),
                show_details: true,
                focus_handle,
                scroll_handle: UniformListScrollHandle::new(),
                pending_serialization: Task::ready(None),
                _subscriptions: vec![settings_subscription, filter_editor_subscription],

                root_entry: None,
                root_buffer: None,
                root_anchor: None,
                fetch_task: Task::ready(()),
                expanding_tasks: HashMap::default(),
                loading: false,
                selected_index: None,
                filter_editor,
                cached_entries: Vec::new(),
                cached_entries_update_task: Task::ready(()),
                children_cache: HashMap::default(),
            }
        })
    }

    pub fn show_call_hierarchy(
        workspace: &mut Workspace,
        _: &ShowCallHierarchy,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        let Some(panel) = workspace.panel::<CallHierarchyPanel>(cx) else {
            return;
        };

        workspace.focus_panel::<CallHierarchyPanel>(window, cx);

        panel.update(cx, |panel, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });
    }

    fn fetch_call_hierarchy(
        &mut self,
        editor: Entity<Editor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.children_cache.clear();

        let buffer = editor.read(cx).buffer().read(cx).as_singleton();
        let Some(buffer) = buffer else {
            self.clear_entries(cx);
            return;
        };

        let (position, anchor) = editor.update(cx, |editor, cx| {
            let snapshot = editor.display_snapshot(cx);
            let position = editor
                .selections
                .newest::<language::Point>(&snapshot)
                .head();
            let anchor = buffer.read(cx).anchor_after(position);
            (position, anchor)
        });
        let position_utf16 = position.to_point_utf16(&buffer.read(cx).snapshot());

        let prepare_task = self.project.update(cx, |project, cx| {
            project.prepare_call_hierarchy(&buffer, position_utf16, cx)
        });

        let project = self.project.clone();
        let mode = self.mode;

        self.loading = true;
        cx.notify();

        self.fetch_task = cx.spawn_in(window, async move |panel, mut cx| {
            let items = prepare_task.await;
            let Ok(Some(items)) = items else {
                panel
                    .update_in(cx, |panel, _window, cx| {
                        panel.loading = false;
                        panel.clear_entries(cx);
                    })
                    .log_err();
                return;
            };
            let Some(root_item) = items.into_iter().next() else {
                panel
                    .update_in(cx, |panel, _window, cx| {
                        panel.loading = false;
                        panel.clear_entries(cx);
                    })
                    .log_err();
                return;
            };

            let (state, children) =
                expand(&root_item, &project, &buffer, mode, &panel, &mut cx).await;

            let root_entry = CallHierarchyEntry {
                entry: CachedEntry {
                    id: EntryId::next(),
                    call: Call {
                        target: root_item.selection_range.start,
                        item: root_item,
                    },
                    state,
                    depth: 0,
                    string_match: None,
                },
                children,
            };

            panel
                .update_in(cx, |panel, window, cx| {
                    panel.loading = false;
                    panel.root_entry = Some(root_entry);
                    panel.root_buffer = Some(buffer);
                    panel.root_anchor = Some(anchor);
                    panel.update_cached_entries(window, cx);
                    cx.notify();
                })
                .log_err();
        });
    }

    fn clear_entries(&mut self, cx: &mut Context<Self>) {
        self.root_entry = None;
        self.root_buffer = None;
        self.root_anchor = None;
        cx.notify();
    }

    fn serialization_key() -> String {
        CALL_HIERARCHY_PANEL_KEY.to_string()
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        let active = Some(self.active);
        let serialization_key = Self::serialization_key();

        self.pending_serialization = cx.background_spawn(async move {
            let serial = SerializedCallHierarchyPanel {
                width: width.map(|w| w.into()),
                active,
            };
            KEY_VALUE_STORE
                .write_kvp(serialization_key, serde_json::to_string(&serial).ok()?)
                .await
                .log_err();
            Some(())
        });
    }

    fn toggle_mode(&mut self, _: &ToggleMode, window: &mut Window, cx: &mut Context<Self>) {
        self.clear_filter(window, cx);
        self.children_cache.clear();

        self.mode = match self.mode {
            CallHierarchyMode::Incoming => CallHierarchyMode::Outgoing,
            CallHierarchyMode::Outgoing => CallHierarchyMode::Incoming,
        };

        let placeholder = match self.mode {
            CallHierarchyMode::Incoming => "Incoming calls...",
            CallHierarchyMode::Outgoing => "Outgoing calls...",
        };
        self.filter_editor.update(cx, |editor, cx| {
            editor.set_placeholder_text(placeholder, window, cx);
        });

        let Some(buffer) = &self.root_buffer else {
            return;
        };
        let Some(anchor) = &self.root_anchor else {
            return;
        };

        let position = anchor.to_point_utf16(&buffer.read(cx).snapshot());

        let prepare_task = self.project.update(cx, |project, cx| {
            project.prepare_call_hierarchy(buffer, position, cx)
        });

        let project = self.project.clone();
        let buffer = buffer.clone();
        let mode = self.mode;

        self.loading = true;
        cx.notify();

        self.fetch_task = cx.spawn_in(window, async move |panel, mut cx| {
            let items = prepare_task.await;
            let Ok(Some(items)) = items else {
                panel
                    .update_in(cx, |panel, _window, cx| {
                        panel.loading = false;
                        cx.notify();
                    })
                    .log_err();
                return;
            };
            let Some(root_item) = items.into_iter().next() else {
                panel
                    .update_in(cx, |panel, _window, cx| {
                        panel.loading = false;
                        cx.notify();
                    })
                    .log_err();
                return;
            };

            let (state, children) =
                expand(&root_item, &project, &buffer, mode, &panel, &mut cx).await;

            let root_entry = CallHierarchyEntry {
                entry: CachedEntry {
                    id: EntryId::next(),
                    call: Call {
                        target: root_item.selection_range.start,
                        item: root_item,
                    },
                    state,
                    depth: 0,
                    string_match: None,
                },
                children,
            };

            panel
                .update_in(cx, |panel, window, cx| {
                    panel.loading = false;
                    panel.root_entry = Some(root_entry);
                    panel.update_cached_entries(window, cx);
                })
                .log_err();
        });
    }

    fn toggle_details(&mut self, cx: &mut Context<Self>) {
        self.show_details = !self.show_details;
        cx.notify();
    }

    fn refresh(&mut self, _: &Refresh, window: &mut Window, cx: &mut Context<Self>) {
        self.clear_filter(window, cx);
        self.children_cache.clear();

        let Some(root_entry) = &self.root_entry else {
            return;
        };
        let Some(buffer) = &self.root_buffer else {
            return;
        };
        let Some(anchor) = &self.root_anchor else {
            return;
        };

        let expanded_paths = Self::collect_expanded_paths(root_entry);

        let position = anchor.to_point_utf16(&buffer.read(cx).snapshot());

        let prepare_task = self.project.update(cx, |project, cx| {
            project.prepare_call_hierarchy(buffer, position, cx)
        });

        let project = self.project.clone();
        let buffer = buffer.clone();
        let mode = self.mode;

        self.loading = true;
        cx.notify();

        self.fetch_task = cx.spawn_in(window, async move |panel, mut cx| {
            let items = prepare_task.await;
            let Ok(Some(items)) = items else {
                panel
                    .update_in(cx, |panel, _window, cx| {
                        panel.loading = false;
                        cx.notify();
                    })
                    .log_err();
                return;
            };
            let Some(root_item) = items.into_iter().next() else {
                panel
                    .update_in(cx, |panel, _window, cx| {
                        panel.loading = false;
                        cx.notify();
                    })
                    .log_err();
                return;
            };

            let mut root_entry = CallHierarchyEntry {
                entry: CachedEntry {
                    id: EntryId::next(),
                    call: Call {
                        target: root_item.selection_range.start,
                        item: root_item,
                    },
                    state: CachedEntryState::Unknown,
                    depth: 0,
                    string_match: None,
                },
                children: None,
            };

            expand_by_paths(
                &mut root_entry,
                &project,
                &buffer,
                mode,
                &expanded_paths,
                &panel,
                &mut cx,
            )
            .await;

            panel
                .update_in(cx, |panel, window, cx| {
                    panel.loading = false;
                    panel.root_entry = Some(root_entry);
                    panel.update_cached_entries(window, cx);
                })
                .log_err();
        });
    }

    fn render_entry_icon(
        &self,
        cached_entry: &CachedEntry,
        is_active: bool,
        cx: &App,
    ) -> AnyElement {
        match cached_entry.state {
            CachedEntryState::Loading => Icon::new(IconName::ArrowCircle)
                .size(IconSize::Small)
                .color(entry_label_color(is_active))
                .with_rotate_animation(1)
                .into_any_element(),
            CachedEntryState::Expanded
            | CachedEntryState::Collapsed
            | CachedEntryState::Unknown => {
                let is_expanded = cached_entry.state == CachedEntryState::Expanded;
                FileIcons::get_chevron_icon(is_expanded, cx)
                    .map(|icon_path| {
                        Icon::from_path(icon_path)
                            .color(entry_label_color(is_active))
                            .into_any_element()
                    })
                    .unwrap_or_else(empty_icon)
            }
            CachedEntryState::Leaf => empty_icon(),
        }
    }

    fn collect_expanded_paths(entry: &CallHierarchyEntry) -> Vec<Vec<(String, usize)>> {
        fn collect_recursive(
            entry: &CallHierarchyEntry,
            current_path: &mut Vec<(String, usize)>,
            result: &mut Vec<Vec<(String, usize)>>,
        ) {
            if entry.entry.state == CachedEntryState::Expanded {
                if let Some(children) = &entry.children {
                    for (sibling_index, child) in children.iter().enumerate() {
                        current_path.push((child.entry.call.item.name.clone(), sibling_index));
                        if child.entry.state == CachedEntryState::Expanded {
                            result.push(current_path.clone());
                            collect_recursive(child, current_path, result);
                        }
                        current_path.pop();
                    }
                }
            }
        }

        let mut result = Vec::new();
        let mut current_path = Vec::new();
        collect_recursive(entry, &mut current_path, &mut result);
        result
    }

    fn collect_visible_entries(&self) -> Vec<(usize, EntryId, Call, CachedEntryState)> {
        self.root_entry
            .as_ref()
            .map(|root| {
                root.iter_visible()
                    .map(|(depth, entry)| {
                        (
                            depth,
                            entry.entry.id,
                            entry.entry.call.clone(),
                            entry.entry.state,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn find_entry_by_id_mut(&mut self, target_id: EntryId) -> Option<&mut CallHierarchyEntry> {
        self.root_entry.as_mut()?.find_by_id_mut(target_id)
    }

    fn toggle_expanded(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let query = self.filter_editor.read(cx).text(cx);

        if !query.is_empty() {
            return;
        }

        let Some(cached_entry) = self.cached_entries.get(index) else {
            return;
        };

        match cached_entry.state {
            CachedEntryState::Expanded => self.collapse_entry(index, window, cx),
            _ => self.expand_entry(index, window, cx),
        }
    }

    fn collapse_entry(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry_id) = self.cached_entries.get(index).map(|e| e.id) else {
            return;
        };

        if let Some(entry) = self.find_entry_by_id_mut(entry_id) {
            if entry.entry.state == CachedEntryState::Expanded {
                entry.entry.state = CachedEntryState::Collapsed;
            }
        }
        self.update_cached_entries(window, cx);
    }

    fn expand_entry(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(cached_entry) = self.cached_entries.get(index).cloned() else {
            return;
        };

        let entry_id = cached_entry.id;

        match cached_entry.state {
            CachedEntryState::Leaf => {
                self.open_entry(index, window, cx);
                return;
            }
            CachedEntryState::Expanded => {
                cx.notify();
                return;
            }
            CachedEntryState::Collapsed => {
                if let Some(entry) = self.find_entry_by_id_mut(entry_id) {
                    entry.entry.state = CachedEntryState::Expanded;
                }
                self.update_cached_entries(window, cx);
                return;
            }
            CachedEntryState::Unknown | CachedEntryState::Loading => {}
        }

        let item = cached_entry.call.item;

        let cache_key = CacheKey(item.uri.clone(), item.selection_range.clone());
        if let Some(cached_calls) = self.children_cache.get(&cache_key).cloned() {
            let children: Vec<CallHierarchyEntry> =
                cached_calls.into_iter().map(Into::into).collect();

            if let Some(entry) = self.find_entry_by_id_mut(entry_id) {
                if children.is_empty() {
                    entry.entry.state = CachedEntryState::Leaf;
                    entry.children = Some(Vec::new());
                    self.open_entry(index, window, cx);
                } else {
                    entry.entry.state = CachedEntryState::Expanded;
                    entry.children = Some(children);
                }
            }
            self.update_cached_entries(window, cx);
            cx.notify();
            return;
        }

        if let Some(entry) = self.find_entry_by_id_mut(entry_id) {
            entry.entry.state = CachedEntryState::Loading;
        }
        self.update_cached_entries(window, cx);

        let mode = self.mode;
        let project = self.project.clone();

        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(active_item) = workspace.read(cx).active_item(cx) else {
            return;
        };
        let Some(editor) = active_item.act_as::<Editor>(cx) else {
            return;
        };
        let buffer = editor.read(cx).buffer().read(cx).as_singleton();
        let Some(buffer) = buffer else {
            return;
        };

        let task = cx.spawn_in(window, async move |panel, mut cx| {
            let (state, children) = expand(&item, &project, &buffer, mode, &panel, &mut cx).await;

            panel
                .update_in(cx, |panel, window, cx| {
                    panel.expanding_tasks.remove(&entry_id);
                    if let Some(entry) = panel.find_entry_by_id_mut(entry_id) {
                        entry.entry.state = state;
                        entry.children = children;
                        if entry.entry.state == CachedEntryState::Leaf {
                            panel.open_entry(index, window, cx);
                        }
                    }
                    panel.update_cached_entries(window, cx);
                    cx.notify();
                })
                .log_err();
        });
        self.expanding_tasks.insert(entry_id, task);
    }

    fn open_entry(&self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(cached_entry) = self.cached_entries.get(index) else {
            return;
        };

        let start = cached_entry.call.target;
        let uri = cached_entry.call.item.uri.clone();

        let abs_path = match uri.to_file_path() {
            Ok(path) => path,
            Err(_) => return,
        };

        let buffer_task = self
            .project
            .update(cx, |project, cx| project.open_local_buffer(&abs_path, cx));
        let workspace = self.workspace.clone();

        cx.spawn_in(window, async move |_, cx| {
            let buffer = buffer_task.await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let position = buffer.read(cx).clip_point_utf16(start, Bias::Left);
                let pane = workspace.active_pane().clone();

                let editor =
                    workspace.open_project_item::<Editor>(pane, buffer, true, true, window, cx);

                editor.update(cx, |editor, cx| {
                    editor.change_selections(
                        SelectionEffects::scroll(Autoscroll::center()),
                        window,
                        cx,
                        |s| s.select_ranges([position..position]),
                    );
                });
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn select_next(&mut self, _: &SelectNext, window: &mut Window, cx: &mut Context<Self>) {
        let entry_count = self.cached_entries.len();

        if entry_count == 0 {
            return;
        }

        if self.filter_editor.focus_handle(cx).is_focused(window) {
            self.focus_handle.focus(window);
        }

        let new_index = match self.selected_index {
            Some(index) => (index + 1).min(entry_count - 1),
            None => 0,
        };

        self.selected_index = Some(new_index);
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        let entry_count = self.cached_entries.len();

        if entry_count == 0 {
            return;
        }

        if self.filter_editor.focus_handle(cx).is_focused(window) {
            self.focus_handle.focus(window);
        }

        let new_index = match self.selected_index {
            Some(index) => index.saturating_sub(1),
            None => entry_count - 1,
        };

        self.selected_index = Some(new_index);
        cx.notify();
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_index {
            self.open_entry(index, window, cx);
        }
    }

    fn toggle_selected_entry(
        &mut self,
        _: &ToggleSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(index) = self.selected_index {
            self.toggle_expanded(index, window, cx);
        }
    }

    fn collapse_all(&mut self, _: &CollapseAll, window: &mut Window, cx: &mut Context<Self>) {
        fn collapse_entry_recursive(entry: &mut CallHierarchyEntry) {
            if entry.entry.state == CachedEntryState::Expanded {
                if let Some(children) = &mut entry.children {
                    for child in children.iter_mut() {
                        collapse_entry_recursive(child);
                    }
                }
                entry.entry.state = CachedEntryState::Collapsed;
            }
        }

        if let Some(root) = &mut self.root_entry {
            if let Some(children) = &mut root.children {
                for child in children {
                    collapse_entry_recursive(child);
                }
            }
        }
        self.update_cached_entries(window, cx);
    }

    fn expand_selected_entry(
        &mut self,
        _: &ExpandSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.selected_index else {
            return;
        };

        let Some(cached_entry) = self.cached_entries.get(index) else {
            return;
        };

        let has_visible_children = cached_entry.state == CachedEntryState::Expanded
            && self
                .cached_entries
                .get(index + 1)
                .is_some_and(|next| next.depth > cached_entry.depth);

        if has_visible_children {
            self.selected_index = Some(index + 1);
            cx.notify();
        } else {
            self.expand_entry(index, window, cx);
        }
    }

    fn collapse_selected_entry(
        &mut self,
        _: &CollapseSelectedEntry,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self.selected_index else {
            return;
        };

        let Some(cached_entry) = self.cached_entries.get(index) else {
            return;
        };
        let depth = cached_entry.depth;

        if cached_entry.state == CachedEntryState::Expanded {
            self.collapse_entry(index, window, cx);
        } else if depth > 0 {
            // Go up from the bottom of list and find first entry with depth less than current
            // It's a parent of current entry
            for (i, entry) in self.cached_entries.iter().enumerate().rev() {
                if i < index && entry.depth < depth {
                    self.selected_index = Some(i);
                    cx.notify();
                    break;
                }
            }
        }
    }

    fn select_entry(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected_index = Some(index);
        cx.notify();
    }

    fn clear_filter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.filter_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
    }

    fn update_cached_entries(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let query = self.filter_editor.read(cx).text(cx);
        let visible_entries = self.collect_visible_entries();

        self.cached_entries_update_task = cx.spawn_in(window, async move |panel, cx| {
            let new_cached_entries =
                Self::generate_cached_entries(query, visible_entries, &cx).await;

            panel
                .update_in(cx, |panel, _window, cx| {
                    panel.cached_entries = new_cached_entries;

                    if let Some(index) = panel.selected_index {
                        if index >= panel.cached_entries.len() {
                            panel.selected_index = if panel.cached_entries.is_empty() {
                                None
                            } else {
                                Some(0)
                            };
                        }
                    }

                    cx.notify();
                })
                .log_err();
        });
    }

    async fn generate_cached_entries(
        query: String,
        entries: Vec<(usize, EntryId, Call, CachedEntryState)>,
        cx: &AsyncWindowContext,
    ) -> Vec<CachedEntry> {
        use std::sync::atomic::AtomicBool;

        if query.is_empty() {
            entries
                .into_iter()
                .map(|(depth, id, call, state)| CachedEntry {
                    id,
                    call,
                    state,
                    depth,
                    string_match: None,
                })
                .collect()
        } else {
            let candidates: Vec<StringMatchCandidate> = entries
                .iter()
                .enumerate()
                .map(|(id, (_, _, call, _))| StringMatchCandidate::new(id, call.item.name.as_str()))
                .collect();

            let matches = match_strings(
                &candidates,
                &query,
                false,
                true,
                100,
                &AtomicBool::default(),
                cx.background_executor().clone(),
            )
            .await;

            matches
                .into_iter()
                .filter_map(|string_match| {
                    let (depth, id, call, state) = entries.get(string_match.candidate_id)?;
                    Some(CachedEntry {
                        id: *id,
                        call: call.clone(),
                        state: *state,
                        depth: *depth,
                        string_match: Some(string_match),
                    })
                })
                .collect()
        }
    }

    fn dispatch_context(&self) -> KeyContext {
        let mut dispatch_context = KeyContext::new_with_defaults();
        dispatch_context.add("CallHierarchyPanel");
        dispatch_context.add("menu");
        dispatch_context
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let has_content = self.root_entry.is_some();

        let switch_mode_icon = match self.mode {
            CallHierarchyMode::Incoming => IconName::ArrowUpRight,
            CallHierarchyMode::Outgoing => IconName::ArrowDownLeft,
        };

        let switch_mode_tooltip = match self.mode {
            CallHierarchyMode::Incoming => "Show Outgoing Calls",
            CallHierarchyMode::Outgoing => "Show Incoming Calls",
        };

        let mode_label = match self.mode {
            CallHierarchyMode::Incoming => "Incoming calls",
            CallHierarchyMode::Outgoing => "Outgoing calls",
        };

        h_flex()
            .h(Tab::container_height(cx))
            .px_2()
            .gap_1()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(if has_content {
                h_flex()
                    .flex_1()
                    .gap_1p5()
                    .child(
                        Icon::new(IconName::MagnifyingGlass)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.filter_editor.clone())
                    .into_any_element()
            } else {
                Label::new(mode_label)
                    .size(LabelSize::Small)
                    .into_any_element()
            })
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("toggle-mode", switch_mode_icon)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text(switch_mode_tooltip))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.toggle_mode(&ToggleMode, window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("collapse-all", IconName::SquareMinus)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Collapse all"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.collapse_all(&CollapseAll, window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("refresh", IconName::RotateCw)
                            .shape(IconButtonShape::Square)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Refresh call hierarchy"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.refresh(&Refresh, window, cx);
                            })),
                    ),
            )
    }

    fn query(&self, cx: &App) -> Option<String> {
        let query = self.filter_editor.read(cx).text(cx);
        if query.trim().is_empty() {
            None
        } else {
            Some(query)
        }
    }

    fn render_main_contents(
        &self,
        query: Option<String>,
        show_indent_guides: bool,
        indent_size: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let contents = if self.loading {
            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Label::new("Loading...")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element()
        } else if self.cached_entries.is_empty() {
            let message = if query.is_some() {
                "No matches"
            } else {
                "Use 'Show Call Hierarchy' on a symbol"
            };

            div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Label::new(message)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element()
        } else {
            let list_contents = {
                let entry_count = self.cached_entries.len();

                uniform_list(
                    "call-hierarchy-entries",
                    entry_count,
                    cx.processor(move |this, range: std::ops::Range<usize>, window, cx| {
                        let selected_index = this.selected_index;
                        let mut items = Vec::with_capacity(range.len());

                        for index in range {
                            if let Some(cached_entry) = this.cached_entries.get(index) {
                                items.push(this.render_entry(
                                    index,
                                    cached_entry,
                                    selected_index,
                                    window,
                                    cx,
                                ));
                            }
                        }
                        items
                    }),
                )
                .size_full()
                .track_scroll(&self.scroll_handle)
                .when(show_indent_guides, |list| {
                    list.with_decoration(
                        indent_guides(indent_size, IndentGuideColors::panel(cx))
                            .with_compute_indents_fn(cx.entity(), |panel, range, _, _cx| {
                                range
                                    .filter_map(|i| {
                                        panel.cached_entries.get(i).map(|entry| entry.depth)
                                    })
                                    .collect()
                            })
                            .with_render_fn(cx.entity(), move |panel, params, _, _| {
                                const LEFT_OFFSET: Pixels = px(14.);

                                let indent_size = params.indent_size;
                                let item_height = params.item_height;
                                let active_indent_guide_ix =
                                    find_active_indent_guide_ix(panel, &params.indent_guides);

                                params
                                    .indent_guides
                                    .into_iter()
                                    .enumerate()
                                    .map(|(ix, layout)| {
                                        let bounds = Bounds::new(
                                            point(
                                                layout.offset.x * indent_size + LEFT_OFFSET,
                                                layout.offset.y * item_height,
                                            ),
                                            size(px(1.), layout.length * item_height),
                                        );
                                        RenderedIndentGuide {
                                            bounds,
                                            layout,
                                            is_active: active_indent_guide_ix == Some(ix),
                                            hitbox: None,
                                        }
                                    })
                                    .collect()
                            }),
                    )
                })
            };

            v_flex()
                .size_full()
                .child(list_contents)
                .custom_scrollbars(
                    Scrollbars::for_settings::<CallHierarchyPanelSettings>()
                        .tracked_scroll_handle(&self.scroll_handle)
                        .with_track_along(
                            ScrollAxes::Horizontal,
                            cx.theme().colors().panel_background,
                        )
                        .tracked_entity(cx.entity_id()),
                    window,
                    cx,
                )
                .into_any_element()
        };

        v_flex().w_full().flex_1().overflow_hidden().child(contents)
    }

    fn render_entry(
        &self,
        index: usize,
        cached_entry: &CachedEntry,
        selected_index: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let settings = CallHierarchyPanelSettings::get_global(cx);
        let is_active = selected_index == Some(index);
        let name = &cached_entry.call.item.name;
        let depth = cached_entry.depth;

        let icon = self.render_entry_icon(cached_entry, is_active, cx);

        let item_id = ElementId::from(SharedString::from(format!("call-hierarchy-{}", index)));
        let details_id = ElementId::from(SharedString::from(format!(
            "call-hierarchy-details-{}",
            index
        )));

        let match_ranges: Vec<Range<usize>> = cached_entry
            .string_match
            .as_ref()
            .map(|m| {
                m.positions
                    .iter()
                    .map(|&pos| {
                        pos..pos
                            + name
                                .get(pos..)
                                .and_then(|s| s.chars().next())
                                .map_or(1, |c| c.len_utf8())
                    })
                    .collect()
            })
            .unwrap_or_default();

        let (name_styled, detail_styled, path_styled) =
            render_item(&cached_entry.call, match_ranges, cx);

        div()
            .text_ui(cx)
            .id(item_id.clone())
            .cursor_pointer()
            .child(
                ListItem::new(item_id)
                    .indent_level(depth)
                    .indent_step_size(settings.indent_size)
                    .toggle_state(is_active)
                    .on_click(cx.listener(move |this, event: &ClickEvent, window, cx| {
                        if event.is_right_click() || event.first_focus() {
                            return;
                        }
                        this.select_entry(index, cx);
                        this.toggle_expanded(index, window, cx);
                        this.open_entry(index, window, cx);
                    }))
                    .child(
                        h_flex()
                            .child(h_flex().w(px(16.)).justify_center().child(icon))
                            .child(
                                h_flex()
                                    .id(details_id)
                                    .h_6()
                                    .gap_1()
                                    .child(name_styled)
                                    .when(self.show_details, |this| {
                                        this.when_some(detail_styled, |this, detail| {
                                            this.child(detail)
                                        })
                                    })
                                    .when_some(path_styled, |this, path| {
                                        this.tooltip(Tooltip::element(move |_window, _cx| {
                                            div().max_w_72().child(path.clone()).into_any_element()
                                        }))
                                    })
                                    .ml_1(),
                            ),
                    ),
            )
            .border_1()
            .border_r_2()
            .rounded_none()
            .hover(|style| {
                if is_active {
                    style
                } else {
                    let hover_color = cx.theme().colors().ghost_element_hover;
                    style.bg(hover_color).border_color(hover_color)
                }
            })
            .when(
                is_active && self.focus_handle.contains_focused(window, cx),
                |div| div.border_color(Color::Selected.color(cx)),
            )
            .into_any_element()
    }
}

async fn expand(
    item: &CallHierarchyItem,
    project: &Entity<Project>,
    buffer: &Entity<language::Buffer>,
    mode: CallHierarchyMode,
    panel: &WeakEntity<CallHierarchyPanel>,
    cx: &mut AsyncWindowContext,
) -> (CachedEntryState, Option<Vec<CallHierarchyEntry>>) {
    let children = fetch_calls(item, project, buffer, mode, cx).await;

    panel
        .update(cx, |panel, _cx| {
            panel.children_cache.insert(
                CacheKey(item.uri.clone(), item.selection_range.clone()),
                children.clone(),
            );
        })
        .log_err();

    if children.is_empty() {
        (CachedEntryState::Leaf, Some(Vec::new()))
    } else {
        (
            CachedEntryState::Expanded,
            Some(children.into_iter().map(Into::into).collect()),
        )
    }
}

async fn expand_by_paths(
    entry: &mut CallHierarchyEntry,
    project: &Entity<Project>,
    buffer: &Entity<language::Buffer>,
    mode: CallHierarchyMode,
    paths: &[Vec<(String, usize)>],
    panel: &WeakEntity<CallHierarchyPanel>,
    cx: &mut AsyncWindowContext,
) {
    let (state, children) = expand(&entry.entry.call.item, project, buffer, mode, panel, cx).await;
    entry.entry.state = state;
    entry.children = children;

    if paths.is_empty() {
        return;
    }

    let Some(children) = &mut entry.children else {
        return;
    };

    // Group paths by their first segment (name, sibling_index)
    // For each matching child, collect the remaining path segments
    for (sibling_index, child) in children.iter_mut().enumerate() {
        let child_paths = filter_paths_for_child(paths, &child.entry.call.item.name, sibling_index);

        if !child_paths.is_empty() {
            Box::pin(expand_by_paths(
                child,
                project,
                buffer,
                mode,
                &child_paths,
                panel,
                cx,
            ))
            .await;
        }
    }
}

fn filter_paths_for_child(
    paths: &[Vec<(String, usize)>],
    child_name: &str,
    sibling_index: usize,
) -> Vec<Vec<(String, usize)>> {
    paths
        .iter()
        .filter(|path| {
            path.first()
                .is_some_and(|(name, idx)| name == child_name && *idx == sibling_index)
        })
        .map(|path| path[1..].to_vec())
        .collect()
}

fn empty_icon() -> AnyElement {
    h_flex()
        .size(IconSize::default().rems())
        .invisible()
        .flex_none()
        .into_any_element()
}

fn find_active_indent_guide_ix(
    panel: &CallHierarchyPanel,
    candidates: &[IndentGuideLayout],
) -> Option<usize> {
    let selected_index = panel.selected_index?;
    let target_depth = panel.cached_entries.get(selected_index)?.depth;

    candidates
        .iter()
        .enumerate()
        .filter(|(_, layout)| {
            let start = layout.offset.y;
            let end = start + layout.length;
            selected_index >= start && selected_index < end
        })
        .max_by_key(|(_, layout)| layout.offset.x)
        .filter(|(_, layout)| layout.offset.x < target_depth)
        .map(|(ix, _)| ix)
}

impl Focusable for CallHierarchyPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.root_entry.is_some() {
            self.filter_editor.focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl EventEmitter<PanelEvent> for CallHierarchyPanel {}

impl Render for CallHierarchyPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let query = self.query(cx);
        let settings = CallHierarchyPanelSettings::get_global(cx);
        let indent_size = settings.indent_size;
        let show_indent_guides = settings.indent_guides.show == ShowIndentGuides::Always;

        v_flex()
            .id("call-hierarchy-panel")
            .size_full()
            .overflow_hidden()
            .track_focus(&self.focus_handle)
            .key_context(self.dispatch_context())
            .on_action(cx.listener(Self::toggle_mode))
            .on_action(cx.listener(Self::refresh))
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::toggle_selected_entry))
            .on_action(cx.listener(Self::expand_selected_entry))
            .on_action(cx.listener(Self::collapse_selected_entry))
            .on_action(cx.listener(Self::collapse_all))
            .child(self.render_header(cx))
            .child(self.render_main_contents(query, show_indent_guides, indent_size, window, cx))
    }
}

impl Panel for CallHierarchyPanel {
    fn persistent_name() -> &'static str {
        "Call Hierarchy Panel"
    }

    fn panel_key() -> &'static str {
        CALL_HIERARCHY_PANEL_KEY
    }

    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        match CallHierarchyPanelSettings::get_global(cx).dock {
            DockSide::Left => DockPosition::Left,
            DockSide::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, _position: DockPosition, _: &mut Window, _cx: &mut Context<Self>) {}

    fn size(&self, _: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| CallHierarchyPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        cx.notify();
        cx.defer_in(window, |this, _, cx| {
            this.serialize(cx);
        });
    }

    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        CallHierarchyPanelSettings::get_global(cx)
            .button
            .then_some(IconName::ArrowDownLeftUpRight)
    }

    fn icon_tooltip(&self, _window: &Window, _: &App) -> Option<&'static str> {
        Some("Call Hierarchy Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _window: &Window, _: &App) -> bool {
        self.active
    }

    fn set_active(&mut self, active: bool, _window: &mut Window, cx: &mut Context<Self>) {
        let old_active = self.active;
        self.active = active;
        if old_active != active {
            self.serialize(cx);
        }
    }

    fn activation_priority(&self) -> u32 {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt as _;
    use gpui::{TestAppContext, VisualTestContext, WindowHandle};
    use indoc::indoc;
    use language::{FakeLspAdapter, Language, LanguageConfig, LanguageMatcher};
    use pretty_assertions::assert_eq;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use std::path::PathBuf;
    use std::sync::Arc;
    use util::path;
    use workspace::{OpenOptions, OpenVisible};

    const SELECTED_MARKER: &str = "  <==== selected";

    fn make_item(name: &str) -> CallHierarchyItem {
        CallHierarchyItem {
            name: name.to_string(),
            kind: lsp::SymbolKind::FUNCTION,
            detail: None,
            uri: lsp::Uri::from_file_path(path!("/test.rs")).unwrap(),
            range: Unclipped(PointUtf16::zero())..Unclipped(PointUtf16::zero()),
            selection_range: Unclipped(PointUtf16::zero())..Unclipped(PointUtf16::zero()),
            data: None,
        }
    }

    fn make_entry(
        name: &str,
        state: CachedEntryState,
        children: Option<Vec<CallHierarchyEntry>>,
    ) -> CallHierarchyEntry {
        let item = make_item(name);
        CallHierarchyEntry {
            entry: CachedEntry {
                id: EntryId::next(),
                call: Call {
                    target: item.selection_range.start,
                    item,
                },
                state,
                depth: 0,
                string_match: None,
            },
            children,
        }
    }

    fn make_leaf(name: &str) -> CallHierarchyEntry {
        make_entry(name, CachedEntryState::Leaf, Some(Vec::new()))
    }

    fn make_collapsed(name: &str, children: Vec<CallHierarchyEntry>) -> CallHierarchyEntry {
        make_entry(name, CachedEntryState::Collapsed, Some(children))
    }

    fn make_expanded(name: &str, children: Vec<CallHierarchyEntry>) -> CallHierarchyEntry {
        make_entry(name, CachedEntryState::Expanded, Some(children))
    }

    #[test]
    fn test_collect_expanded_paths_empty_tree() {
        let root = make_leaf("root");
        let paths = CallHierarchyPanel::collect_expanded_paths(&root);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_collect_expanded_paths_collapsed_children() {
        let root = make_expanded("root", vec![make_collapsed("foo", vec![make_leaf("bar")])]);
        let paths = CallHierarchyPanel::collect_expanded_paths(&root);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_collect_expanded_paths_single_expanded_child() {
        // root (expanded)
        //   └─ foo (expanded)
        //        └─ bar (leaf)
        let root = make_expanded("root", vec![make_expanded("foo", vec![make_leaf("bar")])]);
        let paths = CallHierarchyPanel::collect_expanded_paths(&root);
        assert_eq!(paths, vec![vec![("foo".to_string(), 0)]]);
    }

    #[test]
    fn test_collect_expanded_paths_deeply_nested() {
        // root (expanded)
        //   └─ foo (expanded)
        //        └─ bar (expanded)
        //             └─ baz (leaf)
        let root = make_expanded(
            "root",
            vec![make_expanded(
                "foo",
                vec![make_expanded("bar", vec![make_leaf("baz")])],
            )],
        );
        let paths = CallHierarchyPanel::collect_expanded_paths(&root);
        assert_eq!(
            paths,
            vec![
                vec![("foo".to_string(), 0)],
                vec![("foo".to_string(), 0), ("bar".to_string(), 0)],
            ]
        );
    }

    #[test]
    fn test_collect_expanded_paths_multiple_siblings() {
        // root (expanded)
        //   ├─ foo [0] (expanded)
        //   │    └─ child (leaf)
        //   └─ foo [1] (expanded)
        //        └─ child (leaf)
        let root = make_expanded(
            "root",
            vec![
                make_expanded("foo", vec![make_leaf("child")]),
                make_expanded("foo", vec![make_leaf("child")]),
            ],
        );
        let paths = CallHierarchyPanel::collect_expanded_paths(&root);
        assert_eq!(
            paths,
            vec![vec![("foo".to_string(), 0)], vec![("foo".to_string(), 1)],]
        );
    }

    #[test]
    fn test_collect_expanded_paths_mixed_expansion() {
        // root (expanded)
        //   ├─ foo [0] (collapsed)
        //   ├─ bar [1] (expanded)
        //   │    └─ baz (leaf)
        //   └─ foo [2] (expanded)
        //        └─ qux (leaf)
        let root = make_expanded(
            "root",
            vec![
                make_collapsed("foo", vec![make_leaf("child")]),
                make_expanded("bar", vec![make_leaf("baz")]),
                make_expanded("foo", vec![make_leaf("qux")]),
            ],
        );
        let paths = CallHierarchyPanel::collect_expanded_paths(&root);
        assert_eq!(
            paths,
            vec![vec![("bar".to_string(), 1)], vec![("foo".to_string(), 2)],]
        );
    }

    #[test]
    fn test_filter_paths_for_child_empty_paths() {
        let paths: Vec<Vec<(String, usize)>> = vec![];
        let result = filter_paths_for_child(&paths, "foo", 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_paths_for_child_no_match() {
        let paths = vec![
            vec![("bar".to_string(), 0)],
            vec![("foo".to_string(), 1)], // wrong index
        ];
        let result = filter_paths_for_child(&paths, "foo", 0);
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_paths_for_child_single_match() {
        let paths = vec![
            vec![("foo".to_string(), 0), ("bar".to_string(), 0)],
            vec![("baz".to_string(), 1)],
        ];
        let result = filter_paths_for_child(&paths, "foo", 0);
        assert_eq!(result, vec![vec![("bar".to_string(), 0)]]);
    }

    #[test]
    fn test_filter_paths_for_child_multiple_matches() {
        // Multiple paths that start with the same child
        let paths = vec![
            vec![("foo".to_string(), 0), ("bar".to_string(), 0)],
            vec![("foo".to_string(), 0), ("baz".to_string(), 1)],
            vec![("foo".to_string(), 1)], // different sibling index
        ];
        let result = filter_paths_for_child(&paths, "foo", 0);
        assert_eq!(
            result,
            vec![vec![("bar".to_string(), 0)], vec![("baz".to_string(), 1)],]
        );
    }

    #[test]
    fn test_filter_paths_for_child_strips_first_segment() {
        let paths = vec![vec![
            ("foo".to_string(), 0),
            ("bar".to_string(), 1),
            ("baz".to_string(), 2),
        ]];
        let result = filter_paths_for_child(&paths, "foo", 0);
        assert_eq!(
            result,
            vec![vec![("bar".to_string(), 1), ("baz".to_string(), 2)]]
        );
    }

    #[test]
    fn test_filter_paths_for_child_single_segment_becomes_empty() {
        // Path with only one segment becomes empty after filtering
        let paths = vec![vec![("foo".to_string(), 0)]];
        let result = filter_paths_for_child(&paths, "foo", 0);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_filter_paths_distinguishes_same_name_different_index() {
        // Two children with same name but different sibling indices
        let paths = vec![
            vec![("foo".to_string(), 0), ("child_a".to_string(), 0)],
            vec![("foo".to_string(), 1), ("child_b".to_string(), 0)],
        ];

        let result_0 = filter_paths_for_child(&paths, "foo", 0);
        assert_eq!(result_0, vec![vec![("child_a".to_string(), 0)]]);

        let result_1 = filter_paths_for_child(&paths, "foo", 1);
        assert_eq!(result_1, vec![vec![("child_b".to_string(), 0)]]);
    }

    // ============================================================================
    // Integration Test Infrastructure
    // ============================================================================

    async fn add_call_hierarchy_panel(
        project: &Entity<Project>,
        cx: &mut TestAppContext,
    ) -> WindowHandle<Workspace> {
        let window = cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));

        let panel = window
            .update(cx, |_, window, cx| {
                cx.spawn_in(window, async |this, cx| {
                    CallHierarchyPanel::load(this, cx.clone()).await
                })
            })
            .unwrap()
            .await
            .expect("Failed to load call hierarchy panel");

        window
            .update(cx, |workspace, window, cx| {
                workspace.add_panel(panel, window, cx);
            })
            .unwrap();
        window
    }

    fn call_hierarchy_panel(
        workspace: &WindowHandle<Workspace>,
        cx: &mut TestAppContext,
    ) -> Entity<CallHierarchyPanel> {
        workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .panel::<CallHierarchyPanel>(cx)
                    .expect("no call hierarchy panel")
            })
            .unwrap()
    }

    fn display_entries(panel: &CallHierarchyPanel, selected_index: Option<usize>) -> String {
        let mut display_string = String::new();

        for (index, cached_entry) in panel.cached_entries.iter().enumerate() {
            if !display_string.is_empty() {
                display_string += "\n";
            }
            for _ in 0..cached_entry.depth {
                display_string += "  ";
            }

            let state_indicator = match cached_entry.state {
                CachedEntryState::Unknown => " [?]",
                CachedEntryState::Loading => " [...]",
                CachedEntryState::Leaf => "",
                CachedEntryState::Expanded => " [-]",
                CachedEntryState::Collapsed => " [+]",
            };

            display_string += &cached_entry.call.item.name;
            display_string += state_indicator;

            if Some(index) == selected_index {
                display_string += SELECTED_MARKER;
            }
        }
        display_string
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            theme::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            super::init(cx);
        });
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )
    }

    fn make_lsp_call_hierarchy_item(name: &str, uri: lsp::Uri, row: u32) -> lsp::CallHierarchyItem {
        lsp::CallHierarchyItem {
            name: name.to_string(),
            kind: lsp::SymbolKind::FUNCTION,
            tags: None,
            detail: None,
            uri,
            range: lsp::Range {
                start: lsp::Position {
                    line: row,
                    character: 0,
                },
                end: lsp::Position {
                    line: row,
                    character: 10,
                },
            },
            selection_range: lsp::Range {
                start: lsp::Position {
                    line: row,
                    character: 0,
                },
                end: lsp::Position {
                    line: row,
                    character: 10,
                },
            },
            data: None,
        }
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[gpui::test]
    async fn test_call_hierarchy_panel_basic_display(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": indoc! {"
                        fn main() {
                            helper();
                        }

                        fn helper() {
                            util();
                        }

                        fn util() {
                            println!(\"util\");
                        }
                    "},
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        // Open the file to trigger LSP
        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        // Wait for LSP to start
        let fake_server = fake_servers.next().await.unwrap();

        // Set up LSP handlers
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |params, _| {
                let uri = test_uri.clone();
                async move {
                    let name = &params.item.name;
                    match name.as_str() {
                        "main" => Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("helper", uri, 4),
                            from_ranges: vec![lsp::Range {
                                start: lsp::Position {
                                    line: 1,
                                    character: 4,
                                },
                                end: lsp::Position {
                                    line: 1,
                                    character: 10,
                                },
                            }],
                        }])),
                        "helper" => Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("util", uri, 8),
                            from_ranges: vec![lsp::Range {
                                start: lsp::Position {
                                    line: 5,
                                    character: 4,
                                },
                                end: lsp::Position {
                                    line: 5,
                                    character: 8,
                                },
                            }],
                        }])),
                        _ => Ok(Some(vec![])),
                    }
                }
            }
        });

        // Get the editor and trigger call hierarchy
        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        // Set mode to outgoing and trigger fetch
        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Outgoing;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        // Wait for async operations
        cx.executor().run_until_parked();

        // Verify basic display
        panel.update(cx, |panel, _cx| {
            let entries = display_entries(panel, panel.selected_index);
            assert!(
                entries.contains("main"),
                "Should display root entry 'main', got: {}",
                entries
            );
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_navigation(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn main() {}\nfn foo() {}\nfn bar() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |params, _| {
                let uri = test_uri.clone();
                async move {
                    if params.item.name == "main" {
                        Ok(Some(vec![
                            lsp::CallHierarchyOutgoingCall {
                                to: make_lsp_call_hierarchy_item("foo", uri.clone(), 1),
                                from_ranges: vec![],
                            },
                            lsp::CallHierarchyOutgoingCall {
                                to: make_lsp_call_hierarchy_item("bar", uri, 2),
                                from_ranges: vec![],
                            },
                        ]))
                    } else {
                        Ok(Some(vec![]))
                    }
                }
            }
        });

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Outgoing;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        cx.executor().run_until_parked();

        // Verify initial selection is None (no selection until user navigates)
        panel.update(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_index, None,
                "Initial selection should be None"
            );
        });

        // Navigate - first SelectNext should select index 0
        panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&SelectNext, window, cx);
        });
        cx.executor().run_until_parked();

        panel.update(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_index,
                Some(0),
                "First SelectNext should select index 0"
            );
        });

        // Navigate down again
        panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&SelectNext, window, cx);
        });
        cx.executor().run_until_parked();

        panel.update(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_index,
                Some(1),
                "Selection should move to 1 after second SelectNext"
            );
        });

        // Navigate down again
        panel.update_in(cx, |panel, window, cx| {
            panel.select_next(&SelectNext, window, cx);
        });
        cx.executor().run_until_parked();

        panel.update(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_index,
                Some(2),
                "Selection should move to 2 after third SelectNext"
            );
        });

        // Navigate back up
        panel.update_in(cx, |panel, window, cx| {
            panel.select_previous(&SelectPrevious, window, cx);
        });
        cx.executor().run_until_parked();

        panel.update(cx, |panel, _cx| {
            assert_eq!(
                panel.selected_index,
                Some(1),
                "Selection should move back to 1 after SelectPrevious"
            );
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_collapse_all(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn main() {}\nfn helper() {}\nfn util() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |params, _| {
                let uri = test_uri.clone();
                async move {
                    match params.item.name.as_str() {
                        "main" => Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("helper", uri, 1),
                            from_ranges: vec![],
                        }])),
                        "helper" => Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("util", uri, 2),
                            from_ranges: vec![],
                        }])),
                        _ => Ok(Some(vec![])),
                    }
                }
            }
        });

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Outgoing;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        cx.executor().run_until_parked();

        // Initially we have: main (root, expanded) -> helper (unknown)
        // Count visible entries - should be 2 (main + helper)
        let count_initial = panel.read_with(cx, |panel, _| panel.collect_visible_entries().len());
        assert_eq!(
            count_initial, 2,
            "Initially should have 2 entries (main + helper), got {}",
            count_initial
        );

        // Expand helper by selecting it and triggering expand
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_index = Some(1); // Select helper
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        cx.executor().run_until_parked();

        // Now we should have: main -> helper (expanded) -> util
        // Count should be 3 (main + helper + util)
        let count_after_expand =
            panel.read_with(cx, |panel, _| panel.collect_visible_entries().len());
        assert_eq!(
            count_after_expand, 3,
            "After expanding helper, should have 3 entries (main + helper + util), got {}",
            count_after_expand
        );

        // Verify helper is expanded
        let helper_is_expanded = panel.read_with(cx, |panel, _| {
            panel.root_entry.as_ref().map_or(false, |root| {
                root.children.as_ref().map_or(false, |children| {
                    children
                        .iter()
                        .any(|child| child.entry.state == CachedEntryState::Expanded)
                })
            })
        });
        assert!(
            helper_is_expanded,
            "Helper should be expanded before collapse_all"
        );

        // Collapse all - this collapses children of the root recursively
        // The root remains expanded but helper becomes collapsed
        panel.update_in(cx, |panel, window, cx| {
            panel.collapse_all(&CollapseAll, window, cx);
        });
        cx.executor().run_until_parked();

        // After collapse_all:
        // - main (root) is still expanded (showing helper)
        // - helper is now collapsed (util is hidden)
        // So we expect 2 visible entries: main + helper
        let count_after_collapse =
            panel.read_with(cx, |panel, _| panel.collect_visible_entries().len());
        assert_eq!(
            count_after_collapse, 2,
            "After collapse_all, should have 2 entries (main + helper), got {}",
            count_after_collapse
        );

        // Verify helper is now collapsed
        let helper_is_collapsed = panel.read_with(cx, |panel, _| {
            panel.root_entry.as_ref().map_or(false, |root| {
                root.children.as_ref().map_or(false, |children| {
                    children.iter().all(|child| {
                        child.entry.state == CachedEntryState::Collapsed
                            || child.entry.state == CachedEntryState::Leaf
                    })
                })
            })
        });
        assert!(
            helper_is_collapsed,
            "Helper should be collapsed after collapse_all"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_mode_toggle(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn main() { foo(); }\nfn foo() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("foo", uri, 1)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("main", uri, 0),
                        from_ranges: vec![lsp::Range {
                            start: lsp::Position {
                                line: 0,
                                character: 12,
                            },
                            end: lsp::Position {
                                line: 0,
                                character: 15,
                            },
                        }],
                    }]))
                }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>(
            |_, _| async move { Ok(Some(vec![])) },
        );

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        // Start with incoming mode (default)
        panel.update(cx, |panel, _cx| {
            assert_eq!(
                panel.mode,
                CallHierarchyMode::Incoming,
                "Default mode should be Incoming"
            );
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor.clone(), window, cx);
        });
        cx.executor().run_until_parked();

        // Toggle to outgoing mode
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_mode(&ToggleMode, window, cx);
        });
        cx.executor().run_until_parked();

        panel.update(cx, |panel, _cx| {
            assert_eq!(
                panel.mode,
                CallHierarchyMode::Outgoing,
                "Mode should be Outgoing after toggle"
            );
        });

        // Toggle back to incoming mode
        panel.update_in(cx, |panel, window, cx| {
            panel.toggle_mode(&ToggleMode, window, cx);
        });
        cx.executor().run_until_parked();

        panel.update(cx, |panel, _cx| {
            assert_eq!(
                panel.mode,
                CallHierarchyMode::Incoming,
                "Mode should be Incoming after second toggle"
            );
        });
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_filter(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn main() {}\nfn foo() {}\nfn bar() {}\nfn foobar() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |params, _| {
                let uri = test_uri.clone();
                async move {
                    if params.item.name == "main" {
                        Ok(Some(vec![
                            lsp::CallHierarchyOutgoingCall {
                                to: make_lsp_call_hierarchy_item("foo", uri.clone(), 1),
                                from_ranges: vec![],
                            },
                            lsp::CallHierarchyOutgoingCall {
                                to: make_lsp_call_hierarchy_item("bar", uri.clone(), 2),
                                from_ranges: vec![],
                            },
                            lsp::CallHierarchyOutgoingCall {
                                to: make_lsp_call_hierarchy_item("foobar", uri, 3),
                                from_ranges: vec![],
                            },
                        ]))
                    } else {
                        Ok(Some(vec![]))
                    }
                }
            }
        });

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Outgoing;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        cx.executor().run_until_parked();

        // Count entries before filter
        let count_before = panel.read_with(cx, |panel, _| panel.cached_entries.len());
        assert_eq!(
            count_before, 4,
            "Should have 4 entries (main, foo, bar, foobar)"
        );

        // Apply filter "foo"
        panel.update_in(cx, |panel, window, cx| {
            panel.filter_editor.update(cx, |editor, cx| {
                editor.set_text("foo", window, cx);
            });
        });
        cx.executor().run_until_parked();

        // Count entries after filter - should match "foo" and "foobar"
        let count_after = panel.read_with(cx, |panel, _| panel.cached_entries.len());
        assert_eq!(
            count_after, 2,
            "Filter 'foo' should match 2 entries (foo, foobar)"
        );

        // Clear filter
        panel.update_in(cx, |panel, window, cx| {
            panel.filter_editor.update(cx, |editor, cx| {
                editor.set_text("", window, cx);
            });
        });
        cx.executor().run_until_parked();

        // Should be back to all entries
        let count_cleared = panel.read_with(cx, |panel, _| panel.cached_entries.len());
        assert_eq!(
            count_cleared, 4,
            "After clearing filter, should have all 4 entries"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_open_entry(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn main() { helper(); }\nfn helper() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                        to: make_lsp_call_hierarchy_item("helper", uri, 1),
                        from_ranges: vec![],
                    }]))
                }
            }
        });

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Outgoing;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        cx.executor().run_until_parked();

        // Select the helper entry (index 1)
        panel.update_in(cx, |panel, _window, _cx| {
            panel.selected_index = Some(1);
        });

        // Open the selected entry using confirm (which calls open_entry)
        panel.update_in(cx, |panel, window, cx| {
            panel.confirm(&menu::Confirm, window, cx);
        });
        cx.executor().run_until_parked();

        // Verify an editor is focused on the helper function location
        let active_editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap();

        assert!(
            active_editor.is_some(),
            "Should have an active editor after opening entry"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_refresh(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn main() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        let call_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let call_count_clone = call_count.clone();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            move |_, _| {
                call_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let uri = test_uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>(
            |_, _| async { Ok(Some(vec![])) },
        );

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Outgoing;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        cx.executor().run_until_parked();

        // Initial fetch should have been called once
        assert_eq!(
            call_count.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "Initial fetch should call prepare once"
        );

        // Trigger refresh
        panel.update_in(cx, |panel, window, cx| {
            panel.refresh(&Refresh, window, cx);
        });
        cx.executor().run_until_parked();

        // After refresh, prepare should have been called again
        assert_eq!(
            call_count.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "Refresh should call prepare again"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_lazy_expand(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn main() { a(); }\nfn a() { b(); }\nfn b() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("main", uri, 0)])) }
            }
        });

        let outgoing_call_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let outgoing_call_count_clone = outgoing_call_count.clone();

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |params, _| {
                outgoing_call_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let uri = test_uri.clone();
                async move {
                    match params.item.name.as_str() {
                        "main" => Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("a", uri, 1),
                            from_ranges: vec![],
                        }])),
                        "a" => Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("b", uri, 2),
                            from_ranges: vec![],
                        }])),
                        _ => Ok(Some(vec![])),
                    }
                }
            }
        });

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Outgoing;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        cx.executor().run_until_parked();

        // Initial fetch: only root's children are fetched (main -> a)
        // So outgoing calls should be called once for "main"
        assert_eq!(
            outgoing_call_count.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "Initial fetch should only fetch root's children"
        );

        // Verify 'a' is in Unknown state (children not yet fetched)
        let a_state = panel.read_with(cx, |panel, _| {
            panel.root_entry.as_ref().and_then(|root| {
                root.children.as_ref().and_then(|children| {
                    children
                        .first()
                        .map(|child| child.entry.state == CachedEntryState::Unknown)
                })
            })
        });
        assert_eq!(
            a_state,
            Some(true),
            "'a' should be in Unknown state before expansion"
        );

        // Expand 'a' by selecting it and triggering expand
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_index = Some(1); // Select 'a'
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        cx.executor().run_until_parked();

        // Now outgoing calls should have been called for 'a' as well
        assert_eq!(
            outgoing_call_count.load(std::sync::atomic::Ordering::SeqCst),
            2,
            "Expanding 'a' should fetch its children"
        );

        // Now we should have 3 visible entries: main, a, b
        let count = panel.read_with(cx, |panel, _| panel.collect_visible_entries().len());
        assert_eq!(count, 3, "Should have 3 entries after expanding 'a'");
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_recursive_calls(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn factorial(n: u32) -> u32 { if n <= 1 { 1 } else { n * factorial(n-1) } }\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move {
                    Ok(Some(vec![make_lsp_call_hierarchy_item(
                        "factorial",
                        uri,
                        0,
                    )]))
                }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyOutgoingCalls, _, _>({
            move |params, _| {
                let uri = test_uri.clone();
                async move {
                    if params.item.name == "factorial" {
                        Ok(Some(vec![lsp::CallHierarchyOutgoingCall {
                            to: make_lsp_call_hierarchy_item("factorial", uri, 0),
                            from_ranges: vec![],
                        }]))
                    } else {
                        Ok(Some(vec![]))
                    }
                }
            }
        });

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Outgoing;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        cx.executor().run_until_parked();

        // Should have factorial (root) -> factorial (child) initially
        let count = panel.read_with(cx, |panel, _| panel.collect_visible_entries().len());
        assert_eq!(
            count, 2,
            "Should have 2 entries: factorial (root) and factorial (child)"
        );

        // Expand the child factorial
        panel.update_in(cx, |panel, window, cx| {
            panel.selected_index = Some(1);
            panel.expand_selected_entry(&ExpandSelectedEntry, window, cx);
        });
        cx.executor().run_until_parked();

        // Should now have 3 entries: factorial -> factorial -> factorial
        let count_after = panel.read_with(cx, |panel, _| panel.collect_visible_entries().len());
        assert_eq!(
            count_after, 3,
            "Should have 3 entries after expanding child"
        );

        // Verify the panel handles recursive calls gracefully
        let root_name = panel.read_with(cx, |panel, _| {
            panel
                .root_entry
                .as_ref()
                .map(|e| e.entry.call.item.name.clone())
        });
        assert_eq!(
            root_name,
            Some("factorial".to_string()),
            "Root should be factorial"
        );
    }

    #[gpui::test]
    async fn test_call_hierarchy_panel_multiple_call_sites(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/test",
            json!({
                "src": {
                    "main.rs": "fn main() { helper(); helper(); }\nfn helper() {}\n",
                }
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(rust_lang()));

        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    call_hierarchy_provider: Some(lsp::CallHierarchyServerCapability::Simple(true)),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        let workspace = add_call_hierarchy_panel(&project, cx).await;
        let cx = &mut VisualTestContext::from_window(*workspace, cx);
        let panel = call_hierarchy_panel(&workspace, cx);

        panel.update_in(cx, |panel, window, cx| {
            panel.set_active(true, window, cx);
        });

        let _editor = workspace
            .update(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/test/src/main.rs")),
                    OpenOptions {
                        visible: Some(OpenVisible::All),
                        ..Default::default()
                    },
                    window,
                    cx,
                )
            })
            .unwrap()
            .await
            .unwrap();

        let fake_server = fake_servers.next().await.unwrap();
        let test_uri = lsp::Uri::from_file_path(path!("/test/src/main.rs")).unwrap();

        fake_server.set_request_handler::<lsp::request::CallHierarchyPrepare, _, _>({
            let uri = test_uri.clone();
            move |_, _| {
                let uri = uri.clone();
                async move { Ok(Some(vec![make_lsp_call_hierarchy_item("helper", uri, 1)])) }
            }
        });

        fake_server.set_request_handler::<lsp::request::CallHierarchyIncomingCalls, _, _>({
            move |_, _| {
                let uri = test_uri.clone();
                async move {
                    Ok(Some(vec![lsp::CallHierarchyIncomingCall {
                        from: make_lsp_call_hierarchy_item("main", uri, 0),
                        from_ranges: vec![
                            lsp::Range {
                                start: lsp::Position {
                                    line: 0,
                                    character: 12,
                                },
                                end: lsp::Position {
                                    line: 0,
                                    character: 18,
                                },
                            },
                            lsp::Range {
                                start: lsp::Position {
                                    line: 0,
                                    character: 22,
                                },
                                end: lsp::Position {
                                    line: 0,
                                    character: 28,
                                },
                            },
                        ],
                    }]))
                }
            }
        });

        let editor = workspace
            .update(cx, |workspace, _, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.act_as::<Editor>(cx))
            })
            .unwrap()
            .expect("should have active editor");

        panel.update_in(cx, |panel, _window, cx| {
            panel.mode = CallHierarchyMode::Incoming;
            cx.notify();
        });

        panel.update_in(cx, |panel, window, cx| {
            panel.fetch_call_hierarchy(editor, window, cx);
        });

        cx.executor().run_until_parked();

        // Should have helper (root) -> main (caller with 2 call sites)
        let count = panel.read_with(cx, |panel, _| panel.collect_visible_entries().len());
        assert_eq!(
            count, 2,
            "Should have 2 entries: helper (root) and main (caller)"
        );

        // Verify that main is shown as a caller
        let has_main_caller = panel.read_with(cx, |panel, _| {
            panel.root_entry.as_ref().map_or(false, |root| {
                root.children.as_ref().map_or(false, |children| {
                    children
                        .iter()
                        .any(|child| child.entry.call.item.name == "main")
                })
            })
        });
        assert!(
            has_main_caller,
            "main should be shown as a caller of helper"
        );
    }
}
